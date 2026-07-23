//! GitHub card HTTP surface.

use axum::extract::{Path, Query, State};
use axum::response::Response;
use serde::Deserialize;

use crate::bootstrap::AppState;
use crate::capabilities::github_card::{self, CardOpts};
use crate::interfaces::http::response::{err_svg, parse_bool, svg_response};
use crate::shared::svg::SVG_CACHE_SHORT;

#[derive(Debug, Deserialize)]
pub struct CardQuery {
    pub theme: Option<String>,
    pub color: Option<String>,
    pub credit: Option<String>,
    pub width: Option<u32>,
}

fn card_opts(st: &AppState, q: &CardQuery) -> CardOpts {
    CardOpts {
        theme: q.theme.clone().or_else(|| Some("dark".into())),
        color: q.color.clone(),
        credit: parse_bool(q.credit.as_deref(), st.default_credit),
        width: q.width.unwrap_or(420),
    }
}

pub async fn user_stats_handler(
    State(st): State<AppState>,
    Path(user): Path<String>,
    Query(q): Query<CardQuery>,
) -> Response {
    let opts = card_opts(&st, &q);
    match github_card::user_stats(&st.github, &user, &opts).await {
        Ok(svg) => svg_response(&svg, SVG_CACHE_SHORT),
        Err(e) => err_svg(&e),
    }
}

pub async fn org_stats_handler(
    State(st): State<AppState>,
    Path(org): Path<String>,
    Query(q): Query<CardQuery>,
) -> Response {
    let opts = card_opts(&st, &q);
    match github_card::org_stats(&st.github, &org, &opts).await {
        Ok(svg) => svg_response(&svg, SVG_CACHE_SHORT),
        Err(e) => err_svg(&e),
    }
}

pub async fn repo_card_handler(
    State(st): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(q): Query<CardQuery>,
) -> Response {
    let opts = card_opts(&st, &q);
    match github_card::repo_card(&st.github, &owner, &repo, &opts).await {
        Ok(svg) => svg_response(&svg, SVG_CACHE_SHORT),
        Err(e) => err_svg(&e),
    }
}
