//! GitHub readers: each turns one or two upstream calls into a data shape.
//!
//! Anonymous REST is the default path (`api.github.com`, 60 core calls per
//! hour per IP, 10 search calls per minute); a server token switches the
//! profile readers to one GraphQL call each. The contributions calendar is
//! public HTML and spends no API quota.

use serde_json::{json, Value};

use super::upstream::{read_body, read_json, Call, Resource, Upstream, UpstreamError};
use crate::capabilities::live::domain::calendar::{parse_calendar_html, Calendar};
use crate::capabilities::live::domain::model::{RepoInfo, WorkflowRun};

const API: &str = "https://api.github.com";
/// Owned-repository pages read anonymously (100 per page, most recently pushed).
const REPO_PAGES: usize = 2;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Profile {
    pub login: String,
    pub name: Option<String>,
    pub followers: u64,
    pub public_repos: u64,
    pub created_at: Option<String>,
}

/// The slice of an owned repository the stats and languages cards need.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RepoLite {
    pub name: String,
    pub language: Option<String>,
    pub size: u64,
    pub stars: u64,
    pub fork: bool,
}

/// One GraphQL stats read (server token).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GqlStats {
    pub login: String,
    pub name: Option<String>,
    pub followers: u64,
    pub commits: u64,
    pub prs: u64,
    pub issues: u64,
    pub reviews: u64,
    pub contributed_to: u64,
    pub repo_count: u64,
    pub created_at: Option<String>,
    /// (repository name, stars) for owned repositories.
    pub repos: Vec<(String, u64)>,
}

/// (repository, language, color, bytes) from GraphQL.
pub(crate) type GqlLang = (String, String, Option<String>, u64);

fn text(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::to_string)
}

fn num(v: &Value, key: &str) -> u64 {
    v.get(key).and_then(Value::as_u64).unwrap_or(0)
}

fn count(v: &Value, path: &[&str]) -> u64 {
    let mut cur = v;
    for key in path {
        cur = match cur.get(key) {
            Some(n) => n,
            None => return 0,
        };
    }
    cur.get("totalCount").and_then(Value::as_u64).unwrap_or(0)
}

async fn read(
    up: &dyn Upstream,
    resource: Resource,
    url: String,
) -> Result<Option<Value>, UpstreamError> {
    read_json(up, Call::read(resource, url)).await
}

async fn graphql(
    up: &dyn Upstream,
    query: &str,
    login: &str,
) -> Result<Option<Value>, UpstreamError> {
    let body = json!({ "query": query, "variables": { "login": login } }).to_string();
    let call = Call {
        resource: Resource::Graphql,
        url: format!("{API}/graphql"),
        body: Some(body),
        accept: None,
    };
    let Some(v) = read_json(up, call).await? else {
        return Ok(None);
    };
    match v.pointer("/data/user") {
        Some(Value::Null) | None => {
            let not_found = v.get("errors").and_then(Value::as_array).is_some_and(|es| {
                es.iter()
                    .any(|e| e.get("type").and_then(Value::as_str) == Some("NOT_FOUND"))
            });
            if not_found {
                Ok(None)
            } else {
                Err(UpstreamError::Malformed(
                    "GraphQL answered without data".into(),
                ))
            }
        }
        Some(user) => Ok(Some(user.clone())),
    }
}

pub(crate) async fn profile(
    up: &dyn Upstream,
    login: &str,
) -> Result<Option<Profile>, UpstreamError> {
    Ok(read(up, Resource::Core, format!("{API}/users/{login}"))
        .await?
        .map(|v| Profile {
            login: text(&v, "login").unwrap_or_else(|| login.to_string()),
            name: text(&v, "name").filter(|n| !n.trim().is_empty()),
            followers: num(&v, "followers"),
            public_repos: num(&v, "public_repos"),
            created_at: text(&v, "created_at"),
        }))
}

fn repo_lite(v: &Value) -> Option<RepoLite> {
    Some(RepoLite {
        name: text(v, "name")?,
        language: text(v, "language"),
        size: num(v, "size"),
        stars: num(v, "stargazers_count"),
        fork: v.get("fork").and_then(Value::as_bool).unwrap_or(false),
    })
}

/// Owned repositories, most recently pushed first (at most `REPO_PAGES` pages).
pub(crate) async fn owned_repos(
    up: &dyn Upstream,
    login: &str,
) -> Result<Option<Vec<RepoLite>>, UpstreamError> {
    let mut out = Vec::new();
    for page in 1..=REPO_PAGES {
        let url =
            format!("{API}/users/{login}/repos?per_page=100&type=owner&sort=pushed&page={page}");
        let Some(v) = read(up, Resource::Core, url).await? else {
            return Ok(if page == 1 { None } else { Some(out) });
        };
        let items = v.as_array().cloned().unwrap_or_default();
        let full = items.len() == 100;
        out.extend(items.iter().filter_map(repo_lite));
        if !full {
            break;
        }
    }
    Ok(Some(out))
}

/// `total_count` of an issue search (`author:x type:pr`, `author:x type:issue`).
pub(crate) async fn search_count(
    up: &dyn Upstream,
    query: &str,
) -> Result<Option<u64>, UpstreamError> {
    let q = urlencoding::encode(query);
    let url = format!("{API}/search/issues?q={q}&per_page=1");
    Ok(read(up, Resource::Search, url)
        .await?
        .map(|v| num(&v, "total_count")))
}

pub(crate) async fn calendar(
    up: &dyn Upstream,
    login: &str,
) -> Result<Option<Calendar>, UpstreamError> {
    let url = format!("https://github.com/users/{login}/contributions");
    let Some(html) = read_body(up, Call::read(Resource::Web, url)).await? else {
        return Ok(None);
    };
    parse_calendar_html(&html)
        .map(Some)
        .ok_or_else(|| UpstreamError::Malformed("no calendar days".into()))
}

pub(crate) fn repo_info(v: &Value) -> Option<RepoInfo> {
    Some(RepoInfo {
        owner: v
            .pointer("/owner/login")
            .and_then(Value::as_str)?
            .to_string(),
        name: text(v, "name")?,
        description: text(v, "description"),
        language: text(v, "language"),
        language_color: None,
        stars: num(v, "stargazers_count"),
        forks: num(v, "forks_count"),
        size: num(v, "size"),
        fork: v.get("fork").and_then(Value::as_bool).unwrap_or(false),
        archived: v.get("archived").and_then(Value::as_bool).unwrap_or(false),
        template: v
            .get("is_template")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        license: v
            .pointer("/license/spdx_id")
            .and_then(Value::as_str)
            .map(str::to_string),
        pushed_at: text(v, "pushed_at"),
    })
}

pub(crate) async fn repo(
    up: &dyn Upstream,
    owner: &str,
    name: &str,
) -> Result<Option<RepoInfo>, UpstreamError> {
    match read(up, Resource::Core, format!("{API}/repos/{owner}/{name}")).await? {
        Some(v) => repo_info(&v)
            .map(Some)
            .ok_or_else(|| UpstreamError::Malformed("repository without owner/name".into())),
        None => Ok(None),
    }
}

/// Latest release tag and whether it is a pre-release; the inner `None`
/// means no releases (or no repository: shields reads both the same way).
pub(crate) async fn release(
    up: &dyn Upstream,
    owner: &str,
    name: &str,
    include_pre: bool,
) -> Result<Option<Option<(String, bool)>>, UpstreamError> {
    let pick = |v: &Value| {
        let pre = v
            .get("prerelease")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        text(v, "tag_name").map(|t| (t, pre))
    };
    let tag = if include_pre {
        let url = format!("{API}/repos/{owner}/{name}/releases?per_page=1");
        read(up, Resource::Core, url)
            .await?
            .and_then(|v| v.get(0).and_then(pick))
    } else {
        let url = format!("{API}/repos/{owner}/{name}/releases/latest");
        read(up, Resource::Core, url).await?.and_then(|v| pick(&v))
    };
    Ok(Some(tag))
}

/// Committer date of the newest commit (default branch, or `branch`); the
/// inner `None` means an empty or missing repository.
pub(crate) async fn last_commit(
    up: &dyn Upstream,
    owner: &str,
    name: &str,
    branch: Option<&str>,
) -> Result<Option<Option<String>>, UpstreamError> {
    let sha = branch
        .map(|b| format!("&sha={}", urlencoding::encode(b)))
        .unwrap_or_default();
    let url = format!("{API}/repos/{owner}/{name}/commits?per_page=1{sha}");
    let v = match read(up, Resource::Core, url).await {
        // An empty repository answers 409.
        Err(UpstreamError::Status(409)) => None,
        other => other?,
    };
    let date = v
        .as_ref()
        .and_then(|v| v.pointer("/0/commit/committer/date"))
        .and_then(Value::as_str)
        .map(str::to_string);
    Ok(Some(date))
}

/// The newest completed run of one workflow file (optionally on a branch or for an
/// event); the inner `None` means the workflow has never run.
pub(crate) async fn workflow_run(
    up: &dyn Upstream,
    owner: &str,
    name: &str,
    file: &str,
    branch: Option<&str>,
    event: Option<&str>,
) -> Result<Option<Option<WorkflowRun>>, UpstreamError> {
    let mut url = format!(
        "{API}/repos/{owner}/{name}/actions/workflows/{}/runs?per_page=1&status=completed&exclude_pull_requests=true",
        urlencoding::encode(file)
    );
    for (key, value) in [("branch", branch), ("event", event)] {
        if let Some(v) = value {
            url.push_str(&format!("&{key}={}", urlencoding::encode(v)));
        }
    }
    let Some(v) = read(up, Resource::Core, url).await? else {
        return Ok(None);
    };
    let run = v.pointer("/workflow_runs/0").map(|r| WorkflowRun {
        name: text(r, "name").unwrap_or_else(|| file.trim_end_matches(".yml").to_string()),
        status: text(r, "status").unwrap_or_default(),
        conclusion: text(r, "conclusion"),
    });
    Ok(Some(run))
}

/// When each stargazer on one page (100 per page, oldest first) starred the
/// repository. Empty when the page is empty or GitHub does not share it.
pub(crate) async fn stargazer_page(
    up: &dyn Upstream,
    owner: &str,
    name: &str,
    page: u64,
) -> Result<Vec<String>, UpstreamError> {
    let url = format!("{API}/repos/{owner}/{name}/stargazers?per_page=100&page={page}");
    let call = Call::read(Resource::Core, url).accepting("application/vnd.github.star+json");
    // GitHub lists stargazers only to readers it allows (a signed-in reader
    // with access to the repository); anyone else is refused.
    let v = match read_json(up, call).await {
        Err(UpstreamError::Status(401 | 403)) => return Ok(Vec::new()),
        other => other?,
    };
    Ok(v.as_ref()
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(|i| text(i, "starred_at")).collect())
        .unwrap_or_default())
}

const STATS_QUERY: &str = "query($login:String!){user(login:$login){name login createdAt \
followers{totalCount} \
contributionsCollection{totalCommitContributions totalPullRequestReviewContributions} \
repositoriesContributedTo(first:1,contributionTypes:[COMMIT,ISSUE,PULL_REQUEST,REPOSITORY]){totalCount} \
pullRequests(first:1){totalCount} openIssues:issues(states:OPEN){totalCount} closedIssues:issues(states:CLOSED){totalCount} \
repositories(first:100,ownerAffiliations:OWNER,isFork:false,orderBy:{direction:DESC,field:STARGAZERS}){totalCount nodes{name stargazers{totalCount}}}}}";

pub(crate) async fn gql_stats(
    up: &dyn Upstream,
    login: &str,
) -> Result<Option<GqlStats>, UpstreamError> {
    let Some(u) = graphql(up, STATS_QUERY, login).await? else {
        return Ok(None);
    };
    let repos = u
        .pointer("/repositories/nodes")
        .and_then(Value::as_array)
        .map(|ns| {
            ns.iter()
                .filter_map(|n| Some((text(n, "name")?, count(n, &["stargazers"]))))
                .collect()
        })
        .unwrap_or_default();
    let cc = u
        .get("contributionsCollection")
        .cloned()
        .unwrap_or(Value::Null);
    Ok(Some(GqlStats {
        login: text(&u, "login").unwrap_or_else(|| login.to_string()),
        name: text(&u, "name").filter(|n| !n.trim().is_empty()),
        followers: count(&u, &["followers"]),
        commits: num(&cc, "totalCommitContributions"),
        prs: count(&u, &["pullRequests"]),
        issues: count(&u, &["openIssues"]) + count(&u, &["closedIssues"]),
        reviews: num(&cc, "totalPullRequestReviewContributions"),
        contributed_to: count(&u, &["repositoriesContributedTo"]),
        repo_count: count(&u, &["repositories"]),
        created_at: text(&u, "createdAt"),
        repos,
    }))
}

const LANGS_QUERY: &str = "query($login:String!){user(login:$login){repositories(ownerAffiliations:OWNER,isFork:false,first:100){nodes{name \
languages(first:10,orderBy:{field:SIZE,direction:DESC}){edges{size node{color name}}}}}}}";

pub(crate) async fn gql_langs(
    up: &dyn Upstream,
    login: &str,
) -> Result<Option<Vec<GqlLang>>, UpstreamError> {
    let Some(u) = graphql(up, LANGS_QUERY, login).await? else {
        return Ok(None);
    };
    let mut out = Vec::new();
    for repo in u
        .pointer("/repositories/nodes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(name) = text(repo, "name") else {
            continue;
        };
        for edge in repo
            .pointer("/languages/edges")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(lang) = edge.pointer("/node/name").and_then(Value::as_str) {
                let color = edge
                    .pointer("/node/color")
                    .and_then(Value::as_str)
                    .map(str::to_string);
                out.push((name.clone(), lang.to_string(), color, num(edge, "size")));
            }
        }
    }
    Ok(Some(out))
}
