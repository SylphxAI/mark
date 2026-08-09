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
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use crate::bootstrap::AppState;
use crate::capabilities::mark::interfaces as mark_http;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/api", get(catalog::api_index))
        .route("/api/v1", get(catalog::api_index))
        .route("/api/v1/catalog", get(catalog::catalog))
        // One surface (ADR-0003): /api/v1/mark/{form} is the whole grammar,
        // plus the shields-style /badge/{label}-{message}-{color} pill path.
        // Every legacy capability route is deleted (banner, badge, icons,
        // brand, deploy, stats, org, repo).
        .route("/api/v1/mark", get(mark_http::mark_default_handler))
        .route("/api/v1/mark/{form}", get(mark_http::mark_handler))
        .route("/badge/{*tail}", get(mark_http::badge_path))
        .route("/", get(studio::index_page))
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .with_state(state)
}
