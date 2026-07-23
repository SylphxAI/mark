//! Outbound ports for GitHub card use cases.

use crate::capabilities::github_card::domain::{GhRepo, GhUser};
use std::future::Future;
use std::pin::Pin;

/// Effect boundary for GitHub upstream reads.
pub trait GitHubSource: Send + Sync {
    fn get_user<'a>(
        &'a self,
        username: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<GhUser, String>> + Send + 'a>>;

    fn get_repo<'a>(
        &'a self,
        owner: &'a str,
        repo: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<GhRepo, String>> + Send + 'a>>;

    fn get_user_repos<'a>(
        &'a self,
        username: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<GhRepo>, String>> + Send + 'a>>;

    fn get_org_repos<'a>(
        &'a self,
        org: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<GhRepo>, String>> + Send + 'a>>;
}
