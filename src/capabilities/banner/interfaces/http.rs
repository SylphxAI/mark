//! Banner HTTP surface.

use axum::extract::{Query, State};
use axum::response::Response;
use serde::Deserialize;

use crate::bootstrap::AppState;
use crate::capabilities::banner::{self, BannerInput};
use crate::interfaces::http::response::{current_time_seed, decode_text, parse_bool, svg_response};
use crate::shared::svg::SVG_CACHE;

#[derive(Debug, Deserialize)]
pub struct BannerQuery {
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    pub color: Option<String>,
    pub theme: Option<String>,
    pub section: Option<String>,
    pub reversal: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub text: Option<String>,
    pub desc: Option<String>,
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
    pub animation: Option<String>,
    pub credit: Option<String>,
    pub layout: Option<String>,
}

pub async fn banner_handler(
    State(st): State<AppState>,
    Query(q): Query<BannerQuery>,
) -> Response {
    let credit = parse_bool(q.credit.as_deref(), st.default_credit);
    let input = BannerInput {
        type_name: q.type_name,
        color: q.color,
        theme: q.theme,
        section: q.section,
        reversal: parse_bool(q.reversal.as_deref(), false),
        height: q.height,
        width: q.width,
        text: q.text.map(decode_text),
        desc: q.desc.map(decode_text),
        font_size: q.font_size,
        desc_size: q.desc_size,
        font_color: q.font_color,
        font_align: q.font_align,
        font_align_y: q.font_align_y,
        desc_align: q.desc_align,
        desc_align_y: q.desc_align_y,
        rotate: q.rotate,
        stroke: q.stroke,
        stroke_width: q.stroke_width,
        text_bg: parse_bool(q.text_bg.as_deref(), false),
        animation: q.animation.clone(),
        credit,
        seed: None,
        layout: q.layout,
        clock_seed: Some(current_time_seed()),
    };
    // Animated banners must not sit behind long CDN TTL — otherwise deploys look "dead".
    let anim = q.animation.as_deref().unwrap_or("ambient");
    let cache = if anim.eq_ignore_ascii_case("none") || anim.eq_ignore_ascii_case("static") {
        SVG_CACHE
    } else {
        "public, max-age=60, s-maxage=120, stale-while-revalidate=600"
    };
    svg_response(&banner::render(&input), cache)
}
