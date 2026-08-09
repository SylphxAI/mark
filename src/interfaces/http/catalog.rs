//! Catalog surface — composition of capability catalogs for the studio/API index.

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::bootstrap::AppState;
use crate::capabilities::mark::domain::catalog::vocabulary;

pub async fn api_index(State(st): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "name": "Sylphx Mark",
        "tagline": "Any URL. One image. Your brand.",
        "base": st.public_base,
        "endpoints": [
            "/api/v1/mark",
            "/api/v1/mark/{form}",
            "/badge/{label}-{message}-{color}",
            "/api/v1/catalog",
            "/health"
        ]
    }))
}

pub async fn catalog() -> impl IntoResponse {
    let mut v = vocabulary();
    if let Some(obj) = v.as_object_mut() {
        obj.insert(
            "limits".into(),
            json!({
                "hero_text": 500,
                "hero_desc": 240,
                "hero_lines": 8,
                "pill_label": 80,
                "pill_message": 120,
                "strip_icons": 60,
                "identity_brand": 40,
                "identity_tagline": 120,
                "deploy_service": 40
            }),
        );
        obj.insert(
            "notes".into(),
            json!({
                "grammar": "mark = form x art x paint(theme/color) x geometry(width/height) x text x motion",
                "determinism": "same URL, same mark, forever — no clock, no upstream, no state",
                "live_data": "not offered — use specialist hosts; Mark renders only what the URL says",
                "animation_type": "true per-character typewriter with cursor (SMIL)"
            }),
        );
    }
    Json(v)
}
