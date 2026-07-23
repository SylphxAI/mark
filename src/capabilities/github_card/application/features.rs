//! GitHub card use cases: fetch via port, then pure render.

use super::ports::GitHubSource;
use super::render::{render_org_card, render_repo_card, render_user_card};
use crate::capabilities::github_card::domain::{aggregate, CardOpts};

pub async fn user_stats<G: GitHubSource + ?Sized>(
    github: &G,
    username: &str,
    opts: &CardOpts,
) -> Result<String, String> {
    let user = github.get_user(username).await?;
    let repos = github.get_user_repos(username).await.unwrap_or_default();
    let agg = aggregate(&repos);
    Ok(render_user_card(&user, &agg, opts))
}

pub async fn org_stats<G: GitHubSource + ?Sized>(
    github: &G,
    org: &str,
    opts: &CardOpts,
) -> Result<String, String> {
    let repos = github.get_org_repos(org).await?;
    let agg = aggregate(&repos);
    Ok(render_org_card(org, &agg, opts))
}

pub async fn repo_card<G: GitHubSource + ?Sized>(
    github: &G,
    owner: &str,
    repo: &str,
    opts: &CardOpts,
) -> Result<String, String> {
    let r = github.get_repo(owner, repo).await?;
    Ok(render_repo_card(&r, opts))
}
