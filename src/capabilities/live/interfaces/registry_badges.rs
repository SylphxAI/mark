//! Registry and CI badges on shields' paths: pub.dev, Packagist,
//! Bundlephobia, the Chrome Web Store, and GitHub Actions workflow status.

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::response::Response;
use serde::Deserialize;

use super::badges::{from_lookup, respond};
use super::cards::{valid_login, valid_repo};
use crate::bootstrap::AppState;
use crate::capabilities::live::application::npm::valid_package;
use crate::capabilities::live::application::registries::{
    token, valid_chrome_id, valid_packagist, valid_pub,
};
use crate::capabilities::live::domain::badges::{self, Face};
use crate::capabilities::mark::interfaces::MarkQuery;
use crate::interfaces::http::response::CachePolicy;

fn missing(label: &str, what: &str) -> (Face, CachePolicy) {
    (badges::not_found(label, what), CachePolicy::Fallback)
}

/// `/pub/{v|likes|points|dm}/{package}`.
pub(crate) async fn pub_badge(
    State(st): State<AppState>,
    Path((kind, name)): Path<(String, String)>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let label = match kind.as_str() {
        "v" => "pub",
        "likes" => "likes",
        "points" => "pub points",
        _ => "downloads",
    };
    let (face, policy) = if !valid_pub(&name) {
        missing(label, "package")
    } else if kind == "v" {
        from_lookup(st.live.pub_version(&name).await, label, "package", |v| {
            badges::version("pub", &v)
        })
    } else {
        from_lookup(
            st.live.pub_score(&name).await,
            label,
            "package",
            |s| match kind.as_str() {
                "likes" => badges::likes(s.likes),
                "points" => badges::pub_points(s.points, s.max_points),
                _ => badges::downloads(s.downloads_30d, Some("month")),
            },
        )
    };
    respond(face, policy, &st, &q, &headers)
}

/// `/packagist/{v|dm|dd|dt}/{vendor}/{package}`.
pub(crate) async fn packagist_badge(
    State(st): State<AppState>,
    Path((kind, vendor, package)): Path<(String, String, String)>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let label = if kind == "v" {
        "packagist"
    } else {
        "downloads"
    };
    let (face, policy) = if !valid_packagist(&vendor) || !valid_packagist(&package) {
        missing(label, "package")
    } else if kind == "v" {
        from_lookup(
            st.live.packagist_version(&vendor, &package).await,
            label,
            "package",
            |v| badges::version("packagist", &v),
        )
    } else {
        from_lookup(
            st.live.packagist_downloads(&vendor, &package).await,
            label,
            "package",
            |d| match kind.as_str() {
                "dm" => badges::downloads(d.monthly, Some("month")),
                "dd" => badges::downloads(d.daily, Some("day")),
                _ => badges::downloads(d.total, None),
            },
        )
    };
    respond(face, policy, &st, &q, &headers)
}

/// `/bundlephobia/{min|minzip}/{package}` (scoped names and `@version` too).
pub(crate) async fn bundlephobia_badge(
    State(st): State<AppState>,
    Path((kind, tail)): Path<(String, String)>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let label = if kind == "min" {
        "minified size"
    } else {
        "minzipped size"
    };
    let tail = tail.trim_matches('/');
    let name = match tail.rfind('@') {
        Some(at) if at > 0 => &tail[..at],
        _ => tail,
    };
    let version_ok = tail[name.len()..]
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '@' | '.' | '-' | '_'));
    let (face, policy) = if !valid_package(name) || !version_ok {
        missing(label, "package")
    } else {
        from_lookup(st.live.bundle_size(tail).await, label, "package", |s| {
            badges::size(label, if kind == "min" { s.min } else { s.gzip })
        })
    };
    respond(face, policy, &st, &q, &headers)
}

/// `/chrome-web-store/{v|users|rating|stars|rating-count}/{id}`.
pub(crate) async fn chrome_badge(
    State(st): State<AppState>,
    Path((kind, id)): Path<(String, String)>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let label = match kind.as_str() {
        "v" => "chrome web store",
        "users" | "d" => "users",
        "rating-count" => "rating count",
        _ => "rating",
    };
    let (face, policy) = if !valid_chrome_id(&id) {
        missing(label, "item")
    } else {
        from_lookup(
            st.live.chrome_item(&id).await,
            label,
            "item",
            |item| match kind.as_str() {
                "v" => match item.version.as_deref() {
                    Some(v) => badges::version(label, v),
                    None => badges::unavailable(label),
                },
                "users" | "d" => badges::users(item.users),
                "rating-count" => badges::rating_count(item.rating_count),
                "stars" => badges::rating(item.rating, true),
                _ => badges::rating(item.rating, false),
            },
        )
    };
    respond(face, policy, &st, &q, &headers)
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct WorkflowQuery {
    pub branch: Option<String>,
    pub event: Option<String>,
}

fn valid_ref(v: &str) -> bool {
    token(v, 100, |c| {
        c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/')
    })
}

/// `/github/actions/workflow/status/{owner}/{repo}/{workflow file}`, with
/// shields' `branch` and `event` query.
pub(crate) async fn workflow_badge(
    State(st): State<AppState>,
    Path((owner, repo, file)): Path<(String, String, String)>,
    Query(q): Query<MarkQuery>,
    Query(w): Query<WorkflowQuery>,
    headers: HeaderMap,
) -> Response {
    let label = file
        .trim_end_matches(".yml")
        .trim_end_matches(".yaml")
        .to_string();
    let branch = w.branch.as_deref().filter(|b| valid_ref(b));
    let event = w.event.as_deref().filter(|e| valid_ref(e));
    let (face, policy) = if !valid_login(&owner) || !valid_repo(&repo) || !valid_ref(&file) {
        missing(&label, "workflow")
    } else {
        from_lookup(
            st.live
                .workflow_run(&owner, &repo, &file, branch, event)
                .await,
            &label,
            "workflow",
            |run| match run {
                Some(r) => badges::workflow(&r.name, r.conclusion.as_deref()),
                None => badges::workflow(&label, None),
            },
        )
    };
    respond(face, policy, &st, &q, &headers)
}
