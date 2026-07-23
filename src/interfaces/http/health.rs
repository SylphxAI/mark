//! Liveness surface (not product capability proof).

use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn health() -> impl IntoResponse {
    Json(json!({
        "ok": true,
        "service": "mark",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
