//! HTTP composition contracts — the single mark surface.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use mark::{app, AppState};
use tower::ServiceExt;

fn state() -> AppState {
    AppState {
        default_credit: false,
        public_base: "http://test.local".into(),
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
async fn health_is_json_liveness_with_revision() {
    let (status, ctype, body) = get("/health").await;
    assert_eq!(status, StatusCode::OK);
    assert!(ctype.contains("json"), "ctype={ctype}");
    assert!(body.contains("\"ok\":true") || body.contains("\"ok\": true"));
    let v: serde_json::Value = serde_json::from_str(&body).expect("json");
    let rev = v.get("revision").and_then(|x| x.as_str()).unwrap_or("");
    assert!(!rev.is_empty(), "revision must be present: {body}");
}

#[tokio::test]
async fn mark_surface_serves_every_form() {
    for (path, needle) in [
        ("/api/v1/mark?type=aurora&text=Hi&animation=none", "Hi"),
        ("/api/v1/mark/hero?type=soft&text=Hi&animation=none", "Hi"),
        ("/api/v1/mark/pill?label=build&message=passing", "passing"),
        ("/api/v1/mark/strip?icons=rust,ts", "rust"),
        ("/api/v1/mark/profile?text=Kyle%20Tse", "Kyle Tse"),
        ("/api/v1/mark/deploy?service=mark", "Sylphx"),
        ("/badge/build-passing-brightgreen", "passing"),
    ] {
        let (status, ctype, body) = get(path).await;
        assert_eq!(status, StatusCode::OK, "mark path must serve: {path}");
        assert!(ctype.contains("svg"), "ctype={ctype} for {path}");
        assert!(body.contains(needle), "needle {needle} missing in {path}");
    }
}

#[tokio::test]
async fn badge_shorthand_accepts_grammar_query() {
    let (status, _, styled) = get("/badge/build-passing-brightgreen?style=for-the-badge").await;
    assert_eq!(status, StatusCode::OK);
    assert!(styled.contains("height=\"28\""), "for-the-badge must apply");
    assert!(styled.contains("BUILD"), "for-the-badge paints uppercase");

    let (_, _, flat) = get("/badge/build-passing-brightgreen").await;
    assert!(flat.contains("height=\"20\""), "bare shorthand stays flat");
    assert!(flat.contains("passing"));
}

#[tokio::test]
async fn legacy_surfaces_are_removed() {
    for path in [
        "/api/v1/banner",
        "/api/v1/badge",
        "/api/v1/icons",
        "/api/v1/brand/sylphx",
        "/api/v1/deploy",
        "/api/v1/stats/shtse8",
        "/api/v1/org/SylphxAI",
        "/api/v1/repo/SylphxAI/mark",
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
        assert_eq!(status, StatusCode::NOT_FOUND, "legacy surface must 404: {path}");
    }
}

#[tokio::test]
async fn svg_responses_have_csp_and_nosniff() {
    let app = app(state());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/mark/pill?label=x&message=y")
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
    assert!(csp.contains("script-src 'none'"), "CSP must block scripts: {csp}");
    assert_eq!(
        res.headers()
            .get("x-content-type-options")
            .and_then(|v| v.to_str().ok()),
        Some("nosniff")
    );
}

#[tokio::test]
async fn catalog_exposes_the_one_vocabulary() {
    let (status, _, body) = get("/api/v1/catalog").await;
    assert_eq!(status, StatusCode::OK);
    for key in [
        "forms",
        "art_types",
        "featured_art_types",
        "layouts",
        "themes",
        "icons",
        "badge_styles",
        "animations",
        "fonts",
        "limits",
        "notes",
    ] {
        assert!(body.contains(key), "missing catalog key {key}");
    }
}

#[tokio::test]
async fn injection_is_inert_over_http() {
    let (status, _, body) = get(
        "/api/v1/mark/hero?type=soft&text=probe&animation=none&fontColor=%22%20onload=%22alert(7)",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(!body.contains("onload="), "injection must be inert");
}

#[tokio::test]
async fn determinism_over_http() {
    let path = "/api/v1/mark/hero?type=aurora&text=Same&animation=none";
    let (_, _, a) = get(path).await;
    let (_, _, b) = get(path).await;
    assert_eq!(a, b, "same URL, same mark, forever");
}
