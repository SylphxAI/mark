//! Mark HTTP surface — one grammar, one endpoint.

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap};
use axum::response::Response;
use serde::Deserialize;

use crate::bootstrap::AppState;
use crate::capabilities::mark::application::tiles::{self, parse_per_line, TileTheme};
use crate::capabilities::mark::domain::{
    cap_text, split_badge_path, MarkForm, MarkSpec, MAX_SERVICE_CHARS,
};
use crate::capabilities::mark::render;
use crate::interfaces::http::response::{decode_text, parse_bool, svg_response_conditional};

#[derive(Debug, Deserialize)]
pub(crate) struct MarkQuery {
    pub color: Option<String>,
    pub theme: Option<String>,
    #[serde(rename = "type")]
    pub art: Option<String>,
    pub credit: Option<String>,
    pub animation: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// Content typography: sans (default) or mono.
    pub font: Option<String>,
    /// Content: title (hero) / name (profile).
    pub text: Option<String>,
    /// Content: description (hero) / tagline (profile).
    pub desc: Option<String>,
    // hero
    pub layout: Option<String>,
    // pill / deploy
    pub label: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "labelColor")]
    pub label_color: Option<String>,
    pub style: Option<String>,
    // strip
    pub icons: Option<String>,
    pub perline: Option<u32>,
    // deploy
    pub service: Option<String>,
}

pub(crate) async fn mark_handler(
    State(st): State<AppState>,
    Path(form): Path<String>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let spec = q.to_spec(MarkForm::parse(Some(&form)), st.default_credit);
    svg_response_conditional(&render(&spec), if_none_match(&headers))
}

pub(crate) async fn mark_default_handler(
    State(st): State<AppState>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let spec = q.to_spec(MarkForm::Hero, st.default_credit);
    svg_response_conditional(&render(&spec), if_none_match(&headers))
}

/// Shields-style pill shorthand: `/badge/{label}-{message}-{color}`.
///
/// Path tokens stay the shields embed. Grammar query (`style`, `theme`,
/// `animation`, `labelColor`, `font`, `credit`) composes the same way as
/// `/api/v1/mark/pill` — a `?style=for-the-badge` URL is a valid mark.
pub(crate) async fn badge_path(
    State(st): State<AppState>,
    Path(tail): Path<String>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let (label, message, color) = split_badge_path(&tail);
    let mut spec = q.to_spec(MarkForm::Pill, st.default_credit);
    spec.pill.label = Some(label);
    spec.pill.message = Some(message);
    // Path tokens stay the shields embed.
    // Query `color` only fills a missing path token.
    spec.color = color.or(spec.color);
    svg_response_conditional(&render(&spec), if_none_match(&headers))
}

/// skill-icons dialect query: `i`/`icons`, `theme`/`t`, `perline`.
#[derive(Debug, Deserialize)]
pub(crate) struct IconsQuery {
    pub i: Option<String>,
    pub icons: Option<String>,
    pub t: Option<String>,
    pub theme: Option<String>,
    pub perline: Option<String>,
}

/// skillicons.dev drop-in: `/icons?i=js,ts,rust&theme=light&perline=8`.
pub(crate) async fn icons_handler(Query(q): Query<IconsQuery>, headers: HeaderMap) -> Response {
    let ids = q.i.or(q.icons).unwrap_or_default();
    let theme = TileTheme::parse(q.theme.or(q.t).as_deref());
    let svg = tiles::render(&ids, theme, parse_per_line(q.perline.as_deref()));
    svg_response_conditional(&svg, if_none_match(&headers))
}

impl MarkQuery {
    pub(crate) fn to_spec(&self, form: MarkForm, default_credit: bool) -> MarkSpec {
        MarkSpec {
            form,
            color: self.color.clone(),
            theme: self.theme.clone(),
            art: self.art.clone(),
            credit: parse_bool(self.credit.as_deref(), default_credit),
            animation: self.animation.clone(),
            width: self.width,
            height: self.height,
            text: self.text.clone().map(decode_text),
            desc: self.desc.clone().map(decode_text),
            font: self.font.clone(),
            hero: crate::capabilities::mark::domain::HeroSpec {
                layout: self.layout.clone(),
            },
            pill: crate::capabilities::mark::domain::PillSpec {
                label: self.label.clone(),
                message: self.message.clone(),
                style: self.style.clone(),
                label_color: self.label_color.clone(),
            },
            strip: crate::capabilities::mark::domain::StripSpec {
                icons: self.icons.clone(),
                per_line: self.perline,
            },
            deploy: crate::capabilities::mark::domain::DeploySpec {
                service: self
                    .service
                    .clone()
                    .map(|s| cap_text(&s, MAX_SERVICE_CHARS)),
            },
        }
    }
}

fn if_none_match(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
}
