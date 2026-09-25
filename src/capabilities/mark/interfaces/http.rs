//! Mark HTTP surface — one grammar, one endpoint.

use std::collections::HashMap;

use axum::extract::{Path, Query, RawQuery, State};
use axum::http::{HeaderMap, Uri};
use axum::response::Response;
use serde::Deserialize;

use crate::bootstrap::AppState;
use crate::capabilities::mark::application::tiles::{self, parse_per_line, TileTheme};
use crate::capabilities::mark::domain::paint::css_color;
use crate::capabilities::mark::domain::{
    cap_text, split_badge_path, MarkForm, MarkSpec, MAX_SERVICE_CHARS,
};
use crate::capabilities::mark::interfaces::dialects;
use crate::capabilities::mark::render;
use crate::interfaces::http::response::{
    decode_text, if_none_match, parse_bool, svg_response_conditional,
};

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
    /// Logo left of the label (shields `logo`): slug or base64 data URI.
    pub logo: Option<String>,
    #[serde(rename = "logoColor")]
    pub logo_color: Option<String>,
    #[serde(rename = "logoSize")]
    pub logo_size: Option<String>,
    /// Lenient: a malformed width is ignored, never a 400.
    #[serde(rename = "logoWidth")]
    pub logo_width: Option<String>,
    // score
    pub value: Option<String>,
    pub max: Option<String>,
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
    Query(pairs): Query<HashMap<String, String>>,
    RawQuery(raw): RawQuery,
    headers: HeaderMap,
) -> Response {
    let form = MarkForm::parse(Some(&form));
    let svg = if form == MarkForm::Typing {
        dialects::typing::svg(&pairs, raw.as_deref().unwrap_or(""))
    } else {
        render(&q.to_spec(form, st.default_credit))
    };
    svg_response_conditional(&svg, if_none_match(&headers))
}

/// `/typing?lines=…`: the typing mark (readme-typing-svg parameters).
pub(crate) async fn typing_handler(
    Query(pairs): Query<HashMap<String, String>>,
    RawQuery(raw): RawQuery,
    headers: HeaderMap,
) -> Response {
    let svg = dialects::typing::svg(&pairs, raw.as_deref().unwrap_or(""));
    svg_response_conditional(&svg, if_none_match(&headers))
}

pub(crate) async fn mark_default_handler(
    State(st): State<AppState>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let spec = q.to_spec(MarkForm::Hero, st.default_credit);
    svg_response_conditional(&render(&spec), if_none_match(&headers))
}

/// Shields static badge: `/badge/{label}-{message}-{color}` or
/// `/badge/{message}-{color}`.
///
/// The content is split on the raw path (shields escaping, see
/// `domain::shields`). As on shields, query `label` and `color` override the
/// path tokens, and a color no spelling resolves paints bright green. Grammar
/// query (`style`, `theme`, `animation`, `labelColor`, `font`, `logo`)
/// composes the same way as `/api/v1/mark/pill`.
pub(crate) async fn badge_path(
    State(st): State<AppState>,
    uri: Uri,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let tail = uri.path().strip_prefix("/badge/").unwrap_or_default();
    let (label, message, color) = split_badge_path(tail);
    let mut spec = q.to_spec(MarkForm::Pill, st.default_credit);
    spec.pill.label = spec.pill.label.or(Some(label));
    spec.pill.message = Some(message);
    spec.color = shields_color(spec.color.or(color), None);
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

/// Shields legacy query badge: `/static/v1?label=&message=&color=`.
pub(crate) async fn static_v1(
    State(st): State<AppState>,
    Query(q): Query<MarkQuery>,
    headers: HeaderMap,
) -> Response {
    let mut spec = q.to_spec(MarkForm::Pill, st.default_credit);
    spec.color = shields_color(spec.color, Some("lightgrey"));
    svg_response_conditional(&render(&spec), if_none_match(&headers))
}

/// shields paint defaults: a missing color takes `missing`, an unresolvable
/// one paints bright green (badge-maker's default).
fn shields_color(color: Option<String>, missing: Option<&str>) -> Option<String> {
    match color {
        Some(c) if css_color(&c).is_some() => Some(c),
        Some(_) => Some("brightgreen".into()),
        None => missing.map(str::to_string),
    }
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
                logo: self.logo.clone(),
                logo_color: self.logo_color.clone(),
                logo_size: self.logo_size.clone(),
                logo_width: self
                    .logo_width
                    .as_deref()
                    .and_then(|w| w.trim().parse().ok()),
            },
            score: crate::capabilities::mark::domain::ScoreSpec {
                value: self.value.as_deref().and_then(parse_number),
                max: self.max.as_deref().and_then(parse_number),
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
            typing: Default::default(),
        }
    }
}

/// A finite number, or nothing (`92`, `92.5`, `92%`).
fn parse_number(s: &str) -> Option<f64> {
    s.trim()
        .trim_end_matches('%')
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
}
