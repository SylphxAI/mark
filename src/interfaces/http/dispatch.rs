//! Shared entry paths that several dialects answer (ADR-0005 decision 4).
//!
//! `/` is where readme-typing-svg serves its image, so a host swap lands
//! here. The dispatcher asks the dialects that claim the query in order and
//! falls back to the page the path had before (the studio). Adding a dialect
//! is one more arm.

use std::collections::HashMap;

use axum::extract::{Query, RawQuery, State};
use axum::http::{HeaderMap, Uri};
use axum::response::Response;

use super::response::{if_none_match, svg_response_conditional};
use super::studio;
use crate::bootstrap::AppState;
use crate::capabilities::mark::interfaces::dialects::typing;

/// `GET /`: readme-typing-svg (`?lines=`), else the studio.
pub(crate) async fn root(
    state: State<AppState>,
    Query(pairs): Query<HashMap<String, String>>,
    RawQuery(raw): RawQuery,
    uri: Uri,
    headers: HeaderMap,
) -> Response {
    let query = raw.as_deref().unwrap_or("");
    if typing::claims(&pairs) {
        return svg_response_conditional(&typing::svg(&pairs, query), if_none_match(&headers));
    }
    studio::index_page(state, uri).await
}
