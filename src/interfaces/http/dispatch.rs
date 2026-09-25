//! Shared entry paths that several dialects answer (ADR-0005 decision 4).
//!
//! `/` and `/api` are where other tools serve their images, so a host swap
//! lands here. Each dispatcher asks the dialects that claim the query in
//! order and falls back to the page the path had before: the studio at `/`,
//! the JSON index at `/api`. Adding a dialect is one more arm.

use std::collections::HashMap;

use axum::extract::{Query, RawQuery, State};
use axum::http::{HeaderMap, Uri};
use axum::response::{IntoResponse, Response};

use super::response::{if_none_match, parse_bool, svg_response_conditional};
use super::{catalog, studio};
use crate::bootstrap::AppState;
use crate::capabilities::mark::interfaces::dialects::{capsule, typing};

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

/// `GET /api`: capsule-render (`?type=`, `?text=`, …), else the JSON index.
///
/// github-readme-stats also serves `/api?username=`; its dialect claims
/// before capsule-render when it lands (both accept `theme`).
pub(crate) async fn api(
    State(st): State<AppState>,
    Query(pairs): Query<HashMap<String, String>>,
    RawQuery(raw): RawQuery,
    headers: HeaderMap,
) -> Response {
    let query = raw.as_deref().unwrap_or("");
    if capsule::claims(&pairs) {
        let credit = parse_bool(pairs.get("credit").map(String::as_str), st.default_credit);
        let svg = capsule::render(&pairs, query, credit);
        return svg_response_conditional(&svg, if_none_match(&headers));
    }
    catalog::api_index(State(st)).await.into_response()
}
