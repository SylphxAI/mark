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
