//! HTTP composition contracts — surfaces translate to capabilities without owning domain.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use mark::capabilities::github_card::HttpGitHubSource;
use mark::{app, AppState};
use tower::ServiceExt;

fn state() -> AppState {
    AppState {
        default_credit: false,
        public_base: "http://test.local".into(),
        github: HttpGitHubSource,
    }
}

async fn get(path: &str) -> (StatusCode, String, String) {
    let app = app(state());
    let res = app
        .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let ctype = res
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let body = res.into_body().collect().await.unwrap().to_bytes();
    (status, ctype, String::from_utf8_lossy(&body).into_owned())
}

#[tokio::test]
async fn health_is_json_liveness_not_capability_proof() {
    let (status, ctype, body) = get("/health").await;
    assert_eq!(status, StatusCode::OK);
    assert!(ctype.contains("json"), "ctype={ctype}");
    assert!(body.contains("\"ok\":true") || body.contains("\"ok\": true"));
    assert!(body.contains("mark"));
    assert!(body.contains("revision"), "health must expose revision: {body}");
}

#[tokio::test]
async fn catalog_exposes_capability_catalog_keys() {
    let (status, _, body) = get("/api/v1/catalog").await;
    assert_eq!(status, StatusCode::OK);
    for key in [
        "banner_types",
        "featured_banner_types",
        "layouts",
        "themes",
        "icons",
        "badge_styles",
        "animations",
    ] {
        assert!(body.contains(key), "missing catalog key {key} in {body}");
    }
}

#[tokio::test]
async fn banner_returns_svg_with_content() {
    let (status, ctype, body) =
        get("/api/v1/banner?type=aurora&text=Hello&animation=ambient&credit=0").await;
    assert_eq!(status, StatusCode::OK);
    assert!(ctype.contains("svg"), "ctype={ctype}");
    assert!(body.contains("<svg"));
    assert!(body.contains("Hello"));
}

#[tokio::test]
async fn badge_path_shields_shape() {
    let (status, ctype, body) = get("/badge/build-passing-brightgreen").await;
    assert_eq!(status, StatusCode::OK);
    assert!(ctype.contains("svg"));
    assert!(body.contains("passing") || body.contains("build"));
}

#[tokio::test]
async fn deploy_mark_route() {
    let (status, _, body) = get("/api/v1/deploy?service=mark&style=flat").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Sylphx") || body.contains("deployed"));
}

#[tokio::test]
async fn health_revision_is_non_empty_string() {
    let (status, _, body) = get("/health").await;
    assert_eq!(status, StatusCode::OK);
    let v: serde_json::Value = serde_json::from_str(&body).expect("json");
    let rev = v.get("revision").and_then(|x| x.as_str()).unwrap_or("");
    assert!(!rev.is_empty(), "revision must be present: {body}");
}

#[tokio::test]
async fn legacy_bare_aliases_are_removed() {
    for path in [
        "/banner",
        "/stats/shtse8",
        "/org/SylphxAI",
        "/repo/SylphxAI/mark",
        "/icons",
        "/brand/sylphx",
        "/deploy",
        "/api/v1/nope",
    ] {
        let (status, _, _) = get(path).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "legacy alias must 404: {path}");
    }
}

#[tokio::test]
async fn canonical_surface_still_serves() {
    for (path, needle) in [
        ("/api/v1/banner?type=soft&text=Hi&animation=none", "Hi"),
        ("/api/v1/badge?label=build&message=passing", "passing"),
        ("/badge/build-passing-brightgreen", "passing"),
        ("/api/v1/icons?i=rust,ts", "rust"),
        ("/api/v1/brand/sylphx", "Sylphx"),
        ("/api/v1/deploy?service=mark", "Sylphx"),
    ] {
        let (status, ctype, body) = get(path).await;
        assert_eq!(status, StatusCode::OK, "canonical path must serve: {path}");
        assert!(ctype.contains("svg"), "ctype={ctype} for {path}");
        assert!(body.contains(needle), "needle {needle} missing in {path}");
    }
}

#[tokio::test]
async fn svg_responses_have_csp_and_nosniff() {
    let app = app(state());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/badge?label=x&message=y")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let csp = res
        .headers()
        .get("content-security-policy")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        csp.contains("script-src 'none'"),
        "CSP must block scripts: {csp}"
    );
    assert_eq!(
        res.headers()
            .get("x-content-type-options")
            .and_then(|v| v.to_str().ok()),
        Some("nosniff")
    );
}

#[tokio::test]
async fn catalog_exposes_limits() {
    let (status, _, body) = get("/api/v1/catalog").await;
    assert_eq!(status, StatusCode::OK);
    for key in ["limits", "banner_text", "badge_message", "icons"] {
        assert!(body.contains(key), "missing catalog key {key}");
    }
}
