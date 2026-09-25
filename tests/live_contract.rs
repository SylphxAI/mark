//! MARK-LIVE network contract over HTTP (offline fixtures, ADR-0005).
//!
//! Live routes answer `200 image/svg+xml` whatever upstream does, cache for
//! hours (not forever), revalidate with ETag/`304`, and keep the SVG security
//! headers. A missing subject or a failed upstream renders a short-cached
//! stand-in card instead of a broken image.

use axum::body::Body;
use axum::http::{HeaderMap, Request, StatusCode};
use http_body_util::BodyExt;
use mark::{app, AppState};
use tower::ServiceExt;

async fn get_with(path: &str, inm: Option<&str>) -> (StatusCode, HeaderMap, String) {
    let mut req = Request::builder().uri(path);
    if let Some(tag) = inm {
        req = req.header("if-none-match", tag);
    }
    let res = app(AppState::for_tests())
        .oneshot(req.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (status, headers) = (res.status(), res.headers().clone());
    let body = res.into_body().collect().await.unwrap().to_bytes();
    (status, headers, String::from_utf8_lossy(&body).into_owned())
}

async fn get(path: &str) -> (StatusCode, HeaderMap, String) {
    get_with(path, None).await
}

fn hdr<'a>(h: &'a HeaderMap, k: &str) -> &'a str {
    h.get(k).and_then(|v| v.to_str().ok()).unwrap_or("")
}

const LIVE_PATHS: &[&str] = &[
    "/api?username=ada-dev",
    "/api/top-langs?username=ada-dev&layout=donut",
    "/api/pin?username=SylphxAI&repo=mark",
    "/streak?user=ada-dev",
    "/?user=ada-dev",
    "/api/v1/card/stats?username=ada-dev",
    "/api/v1/card/langs.svg?username=ada-dev",
    "/github/stars/SylphxAI/mark",
    "/github/license/SylphxAI/mark.svg",
    "/npm/v/@sylphx/mark-demo",
    "/npm/dm/mark-demo",
];

#[tokio::test]
async fn live_routes_cache_for_hours_with_etag_and_safety_headers() {
    for path in LIVE_PATHS {
        let (status, h, body) = get(path).await;
        assert_eq!(status, StatusCode::OK, "{path}");
        assert!(
            hdr(&h, "content-type").starts_with("image/svg+xml"),
            "{path}"
        );
        assert!(body.starts_with("<?xml") && body.contains("<svg"), "{path}");
        let cc = hdr(&h, "cache-control");
        assert!(cc.contains("s-maxage=14400"), "{path}: {cc}");
        assert!(cc.contains("stale-while-revalidate=86400"), "{path}: {cc}");
        assert!(cc.contains("stale-if-error=604800"), "{path}: {cc}");
        assert!(
            !cc.contains("immutable"),
            "live data is not immutable: {path}"
        );
        assert!(
            hdr(&h, "cdn-cache-control").contains("s-maxage=14400"),
            "{path}"
        );
        assert!(
            hdr(&h, "content-security-policy").contains("script-src 'none'"),
            "{path}"
        );
        assert_eq!(hdr(&h, "x-content-type-options"), "nosniff", "{path}");
        let etag = hdr(&h, "etag").to_string();
        assert!(etag.starts_with('"'), "{path}: {etag}");
        let (again, h304, empty) = get_with(path, Some(&etag)).await;
        assert_eq!(again, StatusCode::NOT_MODIFIED, "{path}");
        assert!(empty.is_empty(), "{path}");
        assert_eq!(hdr(&h304, "cache-control"), cc, "{path}");
    }
}

#[tokio::test]
async fn failures_render_short_cached_cards_never_errors() {
    for path in [
        "/api?username=ghost-404",
        "/api?username=offline-user",
        "/api?username=bad%20name%3Cscript%3E",
        "/api/top-langs",
        "/api/pin?username=SylphxAI",
        "/api/pin?repo=ghost-404/nothing",
        "/streak?user=offline-now",
        "/github/stars/ghost-404/repo",
        "/github/forks/offline/repo",
        "/github/stars/..%2F..%2Fetc/passwd",
        "/npm/v/ghost-404",
        "/npm/v/Not_Valid!",
        "/api?username=ada-dev&card_width=abc&border_radius=NaN&langs_count=-3",
    ] {
        let (status, h, body) = get(path).await;
        assert_eq!(status, StatusCode::OK, "{path}");
        assert!(
            hdr(&h, "content-type").starts_with("image/svg+xml"),
            "{path}"
        );
        assert!(!body.contains("<script"), "{path}");
        let cc = hdr(&h, "cache-control");
        let fallback = cc == "public, max-age=300, s-maxage=300";
        let live = cc.contains("s-maxage=14400");
        assert!(fallback || live, "{path}: {cc}");
        if path.contains("ghost-404") || path.contains("offline") {
            assert!(
                fallback,
                "missing or failed subject is short-cached: {path}: {cc}"
            );
        }
    }
}

#[tokio::test]
async fn dispatchers_keep_their_previous_answers() {
    let (status, h, body) = get("/api").await;
    assert_eq!(status, StatusCode::OK);
    assert!(hdr(&h, "content-type").starts_with("application/json"));
    assert!(body.contains("/api/v1/card/"));
    let (status, h, _) = get("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(hdr(&h, "content-type").starts_with("text/html"));
    let (status, _, _) = get("/api/v1/card/nope?username=ada-dev").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _, _) = get("/github/nope/SylphxAI/mark").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn host_swap_urls_read_like_the_originals() {
    let (_, _, stats) = get("/api?username=ada-dev&theme=radical&hide=prs&show_icons=true").await;
    assert!(stats.contains("Ada Lovelace&apos;s GitHub Stats"));
    assert!(stats.contains("#fe428e"), "radical title color");
    assert!(!stats.contains("Total PRs"), "hide=prs");
    let (_, _, streak) = get("/?user=ada-dev").await;
    assert!(streak.contains("Current Streak") && streak.contains("Longest Streak"));
    let (_, _, badge) = get("/npm/dm/mark-demo?label=installs&color=purple").await;
    assert!(badge.contains("installs") && badge.contains("1.2M/month"));
    let (_, _, v) = get("/npm/v/@sylphx/mark-demo").await;
    assert!(v.contains("v0.4.0-beta.2"));
}
