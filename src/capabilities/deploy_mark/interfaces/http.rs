//! Deploy-mark HTTP surface.

use axum::extract::Query;
use axum::response::Response;
use serde::Deserialize;

use crate::capabilities::deploy_mark;
use crate::interfaces::http::response::svg_response;
use crate::shared::svg::SVG_CACHE;

#[derive(Debug, Deserialize)]
pub struct DeployQuery {
    pub service: Option<String>,
    pub theme: Option<String>,
    pub style: Option<String>,
}

pub async fn deploy_handler(Query(q): Query<DeployQuery>) -> Response {
    let svg = deploy_mark::render(
        q.service.as_deref().unwrap_or(""),
        q.theme.as_deref(),
        q.style.as_deref().unwrap_or("flat"),
    );
    svg_response(&svg, SVG_CACHE)
}
