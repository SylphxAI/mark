//! Brand-kit HTTP surface.

use axum::extract::{Path, Query, State};
use axum::response::Response;
use serde::Deserialize;

use crate::bootstrap::AppState;
use crate::capabilities::brand_kit;
use crate::interfaces::http::response::{parse_bool, svg_response};
use crate::shared::svg::SVG_CACHE;

#[derive(Debug, Deserialize)]
pub struct BrandQuery {
    pub tagline: Option<String>,
    pub credit: Option<String>,
}

pub async fn brand_handler(
    State(st): State<AppState>,
    Path(name): Path<String>,
    Query(q): Query<BrandQuery>,
) -> Response {
    let credit = parse_bool(q.credit.as_deref(), st.default_credit);
    let svg = brand_kit::render(&name, q.tagline.as_deref(), credit);
    svg_response(&svg, SVG_CACHE)
}
