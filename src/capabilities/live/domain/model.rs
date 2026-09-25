//! Live data shapes: what the upstream readers produce and the cards consume.

use super::languages::LangShare;

/// How the stats card's commit number was counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommitSource {
    /// Commit contributions in the last year (GraphQL, server token).
    CommitsLastYear,
    /// All contributions in the last year (public calendar, no token).
    ContributionsLastYear,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UserStats {
    pub login: String,
    pub name: Option<String>,
    pub stars: Option<u64>,
    pub followers: u64,
    pub commits: Option<(u64, CommitSource)>,
    pub prs: Option<u64>,
    pub issues: Option<u64>,
    pub reviews: Option<u64>,
    /// Repositories contributed to last year (GraphQL only).
    pub contributed_to: Option<u64>,
}

/// How top-language weights were measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LangSource {
    /// Bytes per language across owned repositories (GraphQL, server token).
    Bytes,
    /// Each owned repository's primary language weighted by its size (REST).
    PrimaryBySize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TopLangs {
    pub login: String,
    pub langs: Vec<LangShare>,
    pub source: LangSource,
}

/// One owned repository, as the REST listing or GraphQL returns it.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RepoInfo {
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub language_color: Option<String>,
    pub stars: u64,
    pub forks: u64,
    /// Repository size in KB (the REST `size` field).
    pub size: u64,
    pub fork: bool,
    pub archived: bool,
    pub template: bool,
    pub license: Option<String>,
    pub pushed_at: Option<String>,
}

/// npm package facts for badges.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NpmPackage {
    pub version: String,
    pub license: Option<String>,
}
