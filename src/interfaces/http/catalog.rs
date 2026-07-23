//! Catalog surface — composition of capability catalogs for the studio/API index.

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::bootstrap::AppState;
use crate::capabilities::banner::{ANIMATIONS, BANNER_TYPES, FEATURED_TYPES, LAYOUTS};
use crate::capabilities::icon_row;
use crate::shared::theme;

pub async fn api_index(State(st): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "name": "Sylphx Mark",
        "tagline": "Any URL. One image. Your brand.",
        "base": st.public_base,
        "endpoints": [
            "/api/v1/banner",
            "/api/v1/badge",
            "/badge/{label}-{message}-{color}",
            "/api/v1/stats/{user}",
            "/api/v1/org/{org}",
            "/api/v1/repo/{owner}/{repo}",
            "/api/v1/icons?i=rust,ts,k8s",
            "/api/v1/brand/{name}",
            "/api/v1/deploy",
            "/api/v1/catalog",
            "/health"
        ]
    }))
}

pub async fn catalog() -> impl IntoResponse {
    Json(json!({
        "banner_types": BANNER_TYPES,
        "featured_banner_types": FEATURED_TYPES,
        "layouts": LAYOUTS,
        "themes": theme::list_names(),
        "icons": icon_row::available(),
        "badge_styles": ["flat", "plastic", "for-the-badge", "social", "pill"],
        "animations": ANIMATIONS,
        "notes": {
            "layout": "plate = left monogram product cover; signal = centered hero; terminal = left mono systems look",
            "animation_type": "true per-character typewriter with cursor (SMIL)",
            "star_history": "not offered — use star-history.com; time-series store is out of art-kernel scope"
        }
    }))
}
