//! Live service: cached readers composed into card and badge data.
//!
//! Upstream call budget per cold request (anonymous path):
//! - stats: profile (1 core) → repos (1–2 core), calendar (1 web),
//!   PR and issue counts (2 search), in parallel after the profile answers;
//! - top languages: repos (1–2 core, shared with stats);
//! - streak: calendar (1 web, shared with stats);
//! - repo card, GitHub badges: 1 core each (the repo read is shared);
//! - npm badges: 1 registry or downloads call.
//!
//! With a server token, stats and languages are one GraphQL call each, and
//! fall back to the anonymous path when GraphQL is unavailable.

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use super::cache::{Lookup, Ttl, TtlCache};
use super::fixtures::{fixture_now, FixtureUpstream};
use super::github::{self, GqlLang, GqlStats, Profile, RepoLite};
use super::npm;
use super::upstream::{unix_now, HttpUpstream, Upstream, UpstreamError};

/// How long a stats card waits for an optional part (stars, commits, PRs,
/// issues) before rendering without it.
const OPTIONAL_WAIT: Duration = Duration::from_millis(3000);
use crate::capabilities::live::domain::calendar::{summarize, Calendar, StreakSummary};
use crate::capabilities::live::domain::languages::aggregate;
use crate::capabilities::live::domain::model::{
    CommitSource, LangSource, NpmPackage, RepoInfo, TopLangs, UserStats,
};

/// Process-wide live data: the upstream port, a clock, and bounded caches.
pub struct LiveService {
    up: Arc<dyn Upstream>,
    clock: fn() -> i64,
    profiles: TtlCache<Profile>,
    repos: TtlCache<Vec<RepoLite>>,
    counts: TtlCache<u64>,
    calendars: TtlCache<Calendar>,
    gql_stats: TtlCache<GqlStats>,
    gql_langs: TtlCache<Vec<GqlLang>>,
    repo: TtlCache<RepoInfo>,
    releases: TtlCache<Option<(String, bool)>>,
    commits: TtlCache<Option<String>>,
    packages: TtlCache<NpmPackage>,
    downloads: TtlCache<u64>,
}

impl LiveService {
    fn build(up: Arc<dyn Upstream>, clock: fn() -> i64) -> Self {
        Self {
            up,
            clock,
            profiles: TtlCache::new("profile", 2000, Ttl::PROFILE),
            repos: TtlCache::new("repos", 1000, Ttl::PROFILE),
            counts: TtlCache::new("counts", 4000, Ttl::PROFILE),
            calendars: TtlCache::new("calendar", 2000, Ttl::BADGE),
            gql_stats: TtlCache::new("gql-stats", 2000, Ttl::PROFILE),
            gql_langs: TtlCache::new("gql-langs", 1000, Ttl::PROFILE),
            repo: TtlCache::new("repo", 2000, Ttl::BADGE),
            releases: TtlCache::new("release", 1000, Ttl::BADGE),
            commits: TtlCache::new("commit", 1000, Ttl::BADGE),
            packages: TtlCache::new("npm", 1000, Ttl::BADGE),
            downloads: TtlCache::new("npm-dl", 2000, Ttl::BADGE),
        }
    }

    /// Production wiring: real HTTP, optional server tokens from the env.
    pub fn from_env() -> Self {
        let tokens = HttpUpstream::tokens_from_env();
        tracing::info!(tokens = tokens.len(), "live upstream configured");
        Self::build(Arc::new(HttpUpstream::new(tokens)), unix_now)
    }

    /// Offline fixtures and a fixed clock (anonymous path).
    pub fn for_tests() -> Self {
        Self::build(Arc::new(FixtureUpstream { token: false }), fixture_now)
    }

    /// Offline fixtures on the server-token (GraphQL) path.
    pub fn for_tests_with_token() -> Self {
        Self::build(Arc::new(FixtureUpstream { token: true }), fixture_now)
    }

    pub(crate) fn now_unix(&self) -> i64 {
        (self.clock)()
    }

    fn up(&self) -> &dyn Upstream {
        self.up.as_ref()
    }

    // duplicate-exception: cache-read wrappers bind one TtlCache to one upstream reader under a normalized key; the shared shape is the design (one line of body each).
    async fn calendar(&self, login: &str) -> Lookup<Calendar> {
        let key = login.to_ascii_lowercase();
        self.calendars
            .fetch(key, || github::calendar(self.up(), login))
            .await
    }

    async fn owned_repos(&self, login: &str) -> Lookup<Vec<RepoLite>> {
        let key = login.to_ascii_lowercase();
        self.repos
            .fetch(key, || github::owned_repos(self.up(), login))
            .await
    }

    /// A stats-card part that may hide: waits at most [`OPTIONAL_WAIT`], and
    /// a slower upstream still fills the cache for the next request.
    async fn optional<V, F, Fut>(&self, cache: &TtlCache<V>, key: String, read: F) -> Option<V>
    where
        V: Clone + Send + Sync + 'static,
        F: FnOnce(Arc<dyn Upstream>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<Option<V>, UpstreamError>> + Send + 'static,
    {
        let up = self.up.clone();
        cache
            .fetch_detached(key, OPTIONAL_WAIT, move || read(up))
            .await
            .found()
    }

    pub(crate) async fn stats(&self, login: &str, exclude: &[String]) -> Lookup<UserStats> {
        let up = self.up.as_ref();
        let key = login.to_ascii_lowercase();
        let excluded = |name: &str| exclude.iter().any(|e| e.eq_ignore_ascii_case(name));
        if up.has_token() {
            match self
                .gql_stats
                .fetch(key.clone(), || github::gql_stats(up, login))
                .await
            {
                Lookup::Found(g) => {
                    return Lookup::Found(UserStats {
                        stars: Some(
                            g.repos
                                .iter()
                                .filter(|(n, _)| !excluded(n))
                                .map(|(_, s)| s)
                                .sum(),
                        ),
                        login: g.login,
                        name: g.name,
                        followers: g.followers,
                        repos: Some(g.repo_count),
                        created_at: g.created_at,
                        commits: Some((g.commits, CommitSource::CommitsLastYear)),
                        prs: Some(g.prs),
                        issues: Some(g.issues),
                        reviews: Some(g.reviews),
                        contributed_to: Some(g.contributed_to),
                    })
                }
                Lookup::Missing => return Lookup::Missing,
                Lookup::Unavailable => {}
            }
        }
        let profile = match self
            .profiles
            .fetch(key, || github::profile(up, login))
            .await
        {
            Lookup::Found(p) => p,
            Lookup::Missing => return Lookup::Missing,
            Lookup::Unavailable => return Lookup::Unavailable,
        };
        let key = login.to_ascii_lowercase();
        let (l1, l2, l3, l4) = (
            login.to_string(),
            login.to_string(),
            login.to_string(),
            login.to_string(),
        );
        let (repos, calendar, prs, issues) = tokio::join!(
            self.optional(&self.repos, key.clone(), move |up| async move {
                github::owned_repos(up.as_ref(), &l1).await
            }),
            self.optional(&self.calendars, key.clone(), move |up| async move {
                github::calendar(up.as_ref(), &l2).await
            }),
            self.optional(&self.counts, format!("pr:{key}"), move |up| async move {
                github::search_count(up.as_ref(), &format!("author:{l3} type:pr")).await
            }),
            self.optional(&self.counts, format!("issue:{key}"), move |up| async move {
                github::search_count(up.as_ref(), &format!("author:{l4} type:issue")).await
            }),
        );
        Lookup::Found(UserStats {
            stars: repos.map(|rs| {
                rs.iter()
                    .filter(|r| !excluded(&r.name))
                    .map(|r| r.stars)
                    .sum()
            }),
            commits: calendar.map(|c| (c.total, CommitSource::ContributionsLastYear)),
            login: profile.login,
            name: profile.name,
            followers: profile.followers,
            repos: Some(profile.public_repos),
            created_at: profile.created_at,
            prs,
            issues,
            reviews: None,
            contributed_to: None,
        })
    }

    /// Top languages. Anonymous path: each owned non-fork repository's primary
    /// language weighted by its size (an approximation of bytes per language).
    pub(crate) async fn top_langs(
        &self,
        login: &str,
        exclude: &[String],
        hide: &[String],
    ) -> Lookup<TopLangs> {
        let up = self.up.as_ref();
        let excluded = |name: &str| exclude.iter().any(|e| e.eq_ignore_ascii_case(name));
        if up.has_token() {
            let key = login.to_ascii_lowercase();
            match self
                .gql_langs
                .fetch(key, || github::gql_langs(up, login))
                .await
            {
                Lookup::Found(rows) => {
                    let samples = rows
                        .iter()
                        .filter(|r| !excluded(&r.0))
                        .map(|r| (r.1.as_str(), r.2.as_deref(), r.3));
                    return Lookup::Found(TopLangs {
                        login: login.to_string(),
                        langs: aggregate(samples, hide),
                        source: LangSource::Bytes,
                    });
                }
                Lookup::Missing => return Lookup::Missing,
                Lookup::Unavailable => {}
            }
        }
        match self.owned_repos(login).await {
            Lookup::Found(repos) => {
                let samples = repos
                    .iter()
                    .filter(|r| !r.fork && !excluded(&r.name))
                    .filter_map(|r| r.language.as_deref().map(|l| (l, None, r.size.max(1))));
                Lookup::Found(TopLangs {
                    login: login.to_string(),
                    langs: aggregate(samples, hide),
                    source: LangSource::PrimaryBySize,
                })
            }
            Lookup::Missing => Lookup::Missing,
            Lookup::Unavailable => Lookup::Unavailable,
        }
    }

    pub(crate) async fn streak(&self, login: &str) -> Lookup<StreakSummary> {
        match self.calendar(login).await {
            Lookup::Found(c) => Lookup::Found(summarize(&c)),
            Lookup::Missing => Lookup::Missing,
            Lookup::Unavailable => Lookup::Unavailable,
        }
    }

    pub(crate) async fn repo(&self, owner: &str, name: &str) -> Lookup<RepoInfo> {
        let key = format!("{owner}/{name}").to_ascii_lowercase();
        self.repo
            .fetch(key, || github::repo(self.up(), owner, name))
            .await
    }

    /// Latest release; `Found(None)` when the repository has none.
    // duplicate-exception: cache-read wrappers bind one TtlCache to one upstream reader under a normalized key; the shared shape is the design (one line of body each).
    pub(crate) async fn release(
        &self,
        owner: &str,
        name: &str,
        pre: bool,
    ) -> Lookup<Option<(String, bool)>> {
        let key = format!("{owner}/{name}:{pre}").to_ascii_lowercase();
        let load = || github::release(self.up(), owner, name, pre);
        self.releases.fetch(key, load).await
    }

    /// Newest commit date; `Found(None)` for an empty repository.
    pub(crate) async fn last_commit(
        &self,
        owner: &str,
        name: &str,
        branch: Option<&str>,
    ) -> Lookup<Option<String>> {
        let key = format!("{owner}/{name}@{}", branch.unwrap_or("")).to_ascii_lowercase();
        let load = || github::last_commit(self.up(), owner, name, branch);
        self.commits.fetch(key, load).await
    }

    // duplicate-exception: cache-read wrappers bind one TtlCache to one upstream reader under a normalized key; the shared shape is the design (one line of body each).
    pub(crate) async fn npm_package(&self, name: &str, tag: &str) -> Lookup<NpmPackage> {
        let load = || npm::package(self.up(), name, tag);
        self.packages.fetch(format!("{name}@{tag}"), load).await
    }

    pub(crate) async fn npm_downloads(&self, name: &str, period: &str) -> Lookup<u64> {
        let load = || npm::downloads(self.up(), name, period);
        self.downloads.fetch(format!("{name}:{period}"), load).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn anonymous_stats_compose_rest_search_and_calendar() {
        let live = LiveService::for_tests();
        let Lookup::Found(s) = live.stats("ada-dev", &[]).await else {
            panic!("fixture user resolves");
        };
        assert_eq!(s.name.as_deref(), Some("Ada Lovelace"));
        assert_eq!(s.stars, Some(1840 + 642 + 215 + 97 + 12 + 3));
        assert_eq!(s.prs, Some(142));
        assert!(matches!(
            s.commits,
            Some((_, CommitSource::ContributionsLastYear))
        ));
        assert_eq!(s.contributed_to, None);
        assert_eq!(live.stats("ghost-404", &[]).await, Lookup::Missing);
        assert_eq!(live.stats("offline-user", &[]).await, Lookup::Unavailable);
    }

    #[tokio::test]
    async fn token_path_uses_graphql() {
        let live = LiveService::for_tests_with_token();
        let Lookup::Found(s) = live.stats("ada-dev", &["notes-on-bernoulli".into()]).await else {
            panic!("fixture user resolves");
        };
        assert_eq!(s.stars, Some(1840));
        assert_eq!(s.contributed_to, Some(41));
        let Lookup::Found(l) = live.top_langs("ada-dev", &[], &[]).await else {
            panic!("langs resolve");
        };
        assert_eq!(l.source, LangSource::Bytes);
        assert_eq!(l.langs[0].name, "Rust");
    }

    #[tokio::test]
    async fn anonymous_langs_skip_forks_and_weight_by_size() {
        let live = LiveService::for_tests();
        let Lookup::Found(l) = live.top_langs("ada-dev", &[], &["go".into()]).await else {
            panic!("langs resolve");
        };
        assert_eq!(l.source, LangSource::PrimaryBySize);
        let names: Vec<&str> = l.langs.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["Rust", "TypeScript", "Python", "Shell"]);
    }
}
