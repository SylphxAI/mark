//! GitHub HTTP adapter with short-TTL positive/negative caches.
//!
//! Effects (network, env token, process-global client/cache) stay here.
//! JSON wire DTOs are adapter-local; domain models stay serde-free.

use crate::capabilities::github_card::application::GitHubSource;
use crate::capabilities::github_card::domain::{GhLicense, GhRepo, GhUser};
use moka::future::Cache;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

static CACHE: Lazy<Cache<String, String>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(512)
        .time_to_live(Duration::from_secs(300))
        .build()
});

/// Negative cache for rate-limit / auth failures (brief).
static NEG_CACHE: Lazy<Cache<String, String>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(128)
        .time_to_live(Duration::from_secs(45))
        .build()
});

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent("Sylphx-Mark/0.1 (+https://github.com/SylphxAI/mark)")
        .timeout(Duration::from_secs(12))
        .pool_max_idle_per_host(4)
        .build()
        .expect("reqwest client")
});

/// Production GitHub API source.
#[derive(Debug, Default, Clone, Copy)]
pub struct HttpGitHubSource;

#[derive(Debug, Deserialize)]
struct GhUserDto {
    login: String,
    name: Option<String>,
    public_repos: u32,
    followers: u32,
    following: u32,
    avatar_url: String,
    bio: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GhRepoDto {
    name: String,
    full_name: String,
    description: Option<String>,
    stargazers_count: u32,
    forks_count: u32,
    language: Option<String>,
    html_url: String,
    open_issues_count: u32,
    license: Option<GhLicenseDto>,
}

#[derive(Debug, Deserialize)]
struct GhLicenseDto {
    spdx_id: Option<String>,
}

impl From<GhUserDto> for GhUser {
    fn from(d: GhUserDto) -> Self {
        Self {
            login: d.login,
            name: d.name,
            public_repos: d.public_repos,
            followers: d.followers,
            following: d.following,
            avatar_url: d.avatar_url,
            bio: d.bio,
        }
    }
}

impl From<GhLicenseDto> for GhLicense {
    fn from(d: GhLicenseDto) -> Self {
        Self {
            spdx_id: d.spdx_id,
        }
    }
}

impl From<GhRepoDto> for GhRepo {
    fn from(d: GhRepoDto) -> Self {
        Self {
            name: d.name,
            full_name: d.full_name,
            description: d.description,
            stargazers_count: d.stargazers_count,
            forks_count: d.forks_count,
            language: d.language,
            html_url: d.html_url,
            open_issues_count: d.open_issues_count,
            license: d.license.map(Into::into),
        }
    }
}

fn github_token() -> Option<String> {
    for key in ["GITHUB_TOKEN", "GH_TOKEN", "SYLPHX_GITHUB_TOKEN"] {
        if let Ok(t) = std::env::var(key) {
            let t = t.trim().to_string();
            if !t.is_empty() {
                return Some(t);
            }
        }
    }
    None
}

fn humanize_gh_error(status: reqwest::StatusCode, body: &str) -> String {
    let lower = body.to_ascii_lowercase();
    if status.as_u16() == 403 && lower.contains("rate limit") {
        if github_token().is_none() {
            return "GitHub rate limit exceeded. Set GITHUB_TOKEN on this Mark service for authenticated limits (5000/hr).".into();
        }
        return "GitHub rate limit exceeded for this token. Retry later.".into();
    }
    if status.as_u16() == 404 {
        return "GitHub user/repo not found".into();
    }
    let snippet: String = body.chars().take(160).collect();
    format!("GitHub {status}: {snippet}")
}

async fn gh_get(path: &str) -> Result<String, String> {
    if let Some(hit) = CACHE.get(path).await {
        return Ok(hit);
    }
    if let Some(err) = NEG_CACHE.get(path).await {
        return Err(err);
    }
    let mut req = CLIENT
        .get(format!("https://api.github.com{path}"))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28");
    if let Some(token) = github_token() {
        req = req.bearer_auth(token);
    }
    let res = req
        .send()
        .await
        .map_err(|e| format!("GitHub request failed: {e}"))?;
    let status = res.status();
    let body = res
        .text()
        .await
        .map_err(|e| format!("GitHub response failed: {e}"))?;
    if !status.is_success() {
        let msg = humanize_gh_error(status, &body);
        if status.as_u16() == 403 || status.as_u16() == 429 {
            NEG_CACHE.insert(path.to_string(), msg.clone()).await;
        }
        return Err(msg);
    }
    CACHE.insert(path.to_string(), body.clone()).await;
    Ok(body)
}

impl GitHubSource for HttpGitHubSource {
    fn get_user<'a>(
        &'a self,
        username: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<GhUser, String>> + Send + 'a>> {
        Box::pin(async move {
            let body = gh_get(&format!("/users/{}", urlencoding::encode(username))).await?;
            let dto: GhUserDto = serde_json::from_str(&body).map_err(|e| e.to_string())?;
            Ok(dto.into())
        })
    }

    fn get_repo<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<GhRepo, String>> + Send + 'a>> {
        Box::pin(async move {
            let body = gh_get(&format!(
                "/repos/{}/{}",
                urlencoding::encode(owner),
                urlencoding::encode(repo)
            ))
            .await?;
            let dto: GhRepoDto = serde_json::from_str(&body).map_err(|e| e.to_string())?;
            Ok(dto.into())
        })
    }

    fn get_user_repos<'a>(
        &'a self,
        username: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<GhRepo>, String>> + Send + 'a>> {
        Box::pin(async move {
            let body = gh_get(&format!(
                "/users/{}/repos?per_page=100&sort=updated",
                urlencoding::encode(username)
            ))
            .await?;
            let dtos: Vec<GhRepoDto> = serde_json::from_str(&body).map_err(|e| e.to_string())?;
            Ok(dtos.into_iter().map(Into::into).collect())
        })
    }

    fn get_org_repos<'a>(
        &'a self,
        org: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<GhRepo>, String>> + Send + 'a>> {
        Box::pin(async move {
            let body = gh_get(&format!(
                "/orgs/{}/repos?per_page=100&sort=updated&type=public",
                urlencoding::encode(org)
            ))
            .await?;
            let dtos: Vec<GhRepoDto> = serde_json::from_str(&body).map_err(|e| e.to_string())?;
            Ok(dtos.into_iter().map(Into::into).collect())
        })
    }
}
