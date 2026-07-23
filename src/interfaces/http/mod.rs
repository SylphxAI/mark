//! HTTP composition root — wires capability interfaces into one router.
//!
//! Domain meaning is not owned here; handlers translate HTTP to capability use cases.

mod catalog;
mod health;
pub mod response;
mod studio;

use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::bootstrap::AppState;
use crate::capabilities::badge::interfaces as badge_http;
use crate::capabilities::banner::interfaces as banner_http;
use crate::capabilities::brand_kit::interfaces as brand_http;
use crate::capabilities::deploy_mark::interfaces as deploy_http;
use crate::capabilities::github_card::interfaces as github_http;
use crate::capabilities::icon_row::interfaces as icons_http;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/api", get(catalog::api_index))
        .route("/api/v1", get(catalog::api_index))
        .route("/api/v1/catalog", get(catalog::catalog))
        .route("/api/v1/banner", get(banner_http::banner_handler))
        .route("/banner", get(banner_http::banner_handler))
        .route("/api/v1/badge", get(badge_http::badge_query))
        .route("/badge", get(badge_http::badge_query))
        .route("/badge/{*tail}", get(badge_http::badge_path))
        .route("/api/v1/stats/{user}", get(github_http::user_stats_handler))
        .route("/stats/{user}", get(github_http::user_stats_handler))
        .route("/api/v1/org/{org}", get(github_http::org_stats_handler))
        .route("/org/{org}", get(github_http::org_stats_handler))
        .route(
            "/api/v1/repo/{owner}/{repo}",
            get(github_http::repo_card_handler),
        )
        .route("/repo/{owner}/{repo}", get(github_http::repo_card_handler))
        .route("/api/v1/icons", get(icons_http::icons_handler))
        .route("/icons", get(icons_http::icons_handler))
        .route("/api/v1/brand/{name}", get(brand_http::brand_handler))
        .route("/brand/{name}", get(brand_http::brand_handler))
        .route("/api/v1/deploy", get(deploy_http::deploy_handler))
        .route("/deploy", get(deploy_http::deploy_handler))
        .route("/", get(studio::index_page))
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
