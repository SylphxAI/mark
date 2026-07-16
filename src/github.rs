//! GitHub API client with in-memory TTL cache.

use moka::future::Cache;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::time::Duration;

static CACHE: Lazy<Cache<String, String>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(512)
        .time_to_live(Duration::from_secs(600))
        .build()
});

/// Short-lived negative cache so rate-limit storms don't burn remaining quota.
static NEG_CACHE: Lazy<Cache<String, String>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(256)
        .time_to_live(Duration::from_secs(60))
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

#[derive(Debug, Clone, Deserialize)]
pub struct GhUser {
    pub login: String,
    pub name: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub avatar_url: String,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GhRepo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub html_url: String,
    pub open_issues_count: u32,
    pub license: Option<GhLicense>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GhLicense {
    pub spdx_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Aggregate {
    pub stars: u32,
    pub forks: u32,
    pub repos: u32,
    pub top_langs: Vec<(String, u32, u32)>, // name, count, pct
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
    let res = req.send().await.map_err(|e| format!("GitHub request failed: {e}"))?;
    let status = res.status();
    let body = res.text().await.map_err(|e| format!("GitHub response failed: {e}"))?;
    if !status.is_success() {
        let msg = humanize_gh_error(status, &body);
        // Cache rate-limit / auth failures briefly to avoid stampede.
        if status.as_u16() == 403 || status.as_u16() == 429 {
            NEG_CACHE.insert(path.to_string(), msg.clone()).await;
        }
        return Err(msg);
    }
    CACHE.insert(path.to_string(), body.clone()).await;
    Ok(body)
}

pub async fn get_user(username: &str) -> Result<GhUser, String> {
    let body = gh_get(&format!("/users/{}", urlencoding::encode(username))).await?;
    serde_json::from_str(&body).map_err(|e| e.to_string())
}

pub async fn get_repo(owner: &str, repo: &str) -> Result<GhRepo, String> {
    let body = gh_get(&format!(
        "/repos/{}/{}",
        urlencoding::encode(owner),
        urlencoding::encode(repo)
    ))
    .await?;
    serde_json::from_str(&body).map_err(|e| e.to_string())
}

pub async fn get_user_repos(username: &str) -> Result<Vec<GhRepo>, String> {
    let body = gh_get(&format!(
        "/users/{}/repos?per_page=100&sort=updated",
        urlencoding::encode(username)
    ))
    .await?;
    serde_json::from_str(&body).map_err(|e| e.to_string())
}

pub async fn get_org_repos(org: &str) -> Result<Vec<GhRepo>, String> {
    let body = gh_get(&format!(
        "/orgs/{}/repos?per_page=100&sort=updated&type=public",
        urlencoding::encode(org)
    ))
    .await?;
    serde_json::from_str(&body).map_err(|e| e.to_string())
}

pub fn aggregate(repos: &[GhRepo]) -> Aggregate {
    let mut stars = 0u32;
    let mut forks = 0u32;
    let mut lang_count: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for r in repos {
        stars += r.stargazers_count;
        forks += r.forks_count;
        if let Some(ref lang) = r.language {
            *lang_count.entry(lang.clone()).or_default() += 1;
        }
    }
    let total: u32 = lang_count.values().sum::<u32>().max(1);
    let mut top: Vec<_> = lang_count.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1));
    top.truncate(6);
    let top_langs = top
        .into_iter()
        .map(|(name, count)| (name, count, (count * 100) / total))
        .collect();
    Aggregate {
        stars,
        forks,
        repos: repos.len() as u32,
        top_langs,
    }
}
