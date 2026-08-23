//! Mark HTTP surface — one grammar, one endpoint.

use axum::extract::{Path, Query, State};
use axum::response::Response;
use serde::Deserialize;

use crate::bootstrap::AppState;
use crate::capabilities::mark::domain::{cap_text, MarkForm, MarkSpec, MAX_SERVICE_CHARS};
use crate::capabilities::mark::render;
use crate::interfaces::http::response::{decode_text, decode_token, parse_bool, svg_response};

#[derive(Debug, Deserialize)]
pub struct MarkQuery {
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
    pub section: Option<String>,
    pub reversal: Option<String>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<u32>,
    #[serde(rename = "descSize")]
    pub desc_size: Option<u32>,
    #[serde(rename = "fontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "fontAlign")]
    pub font_align: Option<f32>,
    #[serde(rename = "fontAlignY")]
    pub font_align_y: Option<f32>,
    #[serde(rename = "descAlign")]
    pub desc_align: Option<f32>,
    #[serde(rename = "descAlignY")]
    pub desc_align_y: Option<f32>,
    pub rotate: Option<f32>,
    pub stroke: Option<String>,
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Option<f32>,
    #[serde(rename = "textBg")]
    pub text_bg: Option<String>,
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

pub async fn mark_handler(
    State(st): State<AppState>,
    Path(form): Path<String>,
    Query(q): Query<MarkQuery>,
) -> Response {
    let spec = q.to_spec(MarkForm::parse(Some(&form)), st.default_credit);
    svg_response(&render(&spec), cache_for(&spec))
}

pub async fn mark_default_handler(
    State(st): State<AppState>,
    Query(q): Query<MarkQuery>,
) -> Response {
    let spec = q.to_spec(MarkForm::Hero, st.default_credit);
    svg_response(&render(&spec), cache_for(&spec))
}

/// Shields-style pill shorthand: `/badge/{label}-{message}-{color}`.
///
/// Path tokens stay the shields embed. Grammar query (`style`, `theme`,
/// `animation`, `labelColor`, `font`, `credit`) composes the same way as
/// `/api/v1/mark/pill` — a `?style=for-the-badge` URL is a valid mark.
pub async fn badge_path(
    State(st): State<AppState>,
    Path(tail): Path<String>,
    Query(q): Query<MarkQuery>,
) -> Response {
    let (label, message, color) = split_badge_path(&tail);
    let mut spec = q.to_spec(MarkForm::Pill, st.default_credit);
    spec.pill.label = Some(label);
    spec.pill.message = Some(message);
    // Path tokens stay the shields embed.
    // Query `color` only fills a missing path token.
    spec.color = color.or(spec.color);
    svg_response(&render(&spec), cache_for(&spec))
}

fn split_badge_path(tail: &str) -> (String, String, Option<String>) {
    if tail.contains("--") {
        let parts: Vec<&str> = tail.split("--").collect();
        (
            decode_token(parts.first().copied().unwrap_or("")),
            decode_token(parts.get(1).copied().unwrap_or("ok")),
            parts.get(2).map(|s| decode_token(s)),
        )
    } else {
        let parts: Vec<&str> = tail.rsplitn(3, '-').collect();
        match parts.len() {
            3 => (
                decode_token(parts[2]),
                decode_token(parts[1]),
                Some(decode_token(parts[0])),
            ),
            2 => (
                String::new(),
                decode_token(parts[1]),
                Some(decode_token(parts[0])),
            ),
            _ => (String::new(), decode_token(tail), None),
        }
    }
}

impl MarkQuery {
    pub fn to_spec(&self, form: MarkForm, default_credit: bool) -> MarkSpec {
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
                section: self.section.clone(),
                reversal: parse_bool(self.reversal.as_deref(), false),
                font_size: self.font_size,
                desc_size: self.desc_size,
                font_color: self.font_color.clone(),
                font_align: self.font_align,
                font_align_y: self.font_align_y,
                desc_align: self.desc_align,
                desc_align_y: self.desc_align_y,
                rotate: self.rotate,
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                text_bg: parse_bool(self.text_bg.as_deref(), false),
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
                service: self.service.clone().map(|s| {
                    cap_text(&s, MAX_SERVICE_CHARS)
                }),
            },
        }
    }
}

/// Deterministic marks cache long; animated marks cache short.
fn cache_for(spec: &MarkSpec) -> &'static str {
    let anim = spec.animation.as_deref().unwrap_or("ambient");
    if anim.eq_ignore_ascii_case("none") || anim.eq_ignore_ascii_case("static") {
        crate::capabilities::mark::domain::svg::SVG_CACHE
    } else {
        "public, max-age=60, s-maxage=120, stale-while-revalidate=600"
    }
}
