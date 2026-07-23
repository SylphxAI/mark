//! Icon-row HTTP surface.

use axum::extract::Query;
use axum::response::Response;
use serde::Deserialize;

use crate::capabilities::icon_row;
use crate::interfaces::http::response::svg_response;
use crate::shared::svg::SVG_CACHE;

#[derive(Debug, Deserialize)]
pub struct IconsQuery {
    pub i: Option<String>,
    pub icons: Option<String>,
    pub theme: Option<String>,
    #[serde(rename = "perline")]
    pub per_line: Option<u32>,
}

pub async fn icons_handler(Query(q): Query<IconsQuery>) -> Response {
    let list = q
        .i
        .or(q.icons)
        .unwrap_or_else(|| "rust,ts,docker".into());
    let svg = icon_row::render_row(&list, q.theme.as_deref(), q.per_line.unwrap_or(12));
    svg_response(&svg, SVG_CACHE)
}
