//! Shared HTTP response helpers for SVG surfaces.

use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};

use crate::shared::svg::esc;
use crate::shared::theme;

pub fn parse_bool(v: Option<&str>, default: bool) -> bool {
    match v {
        None => default,
        Some(s) => matches!(s.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
    }
}

pub fn decode_text(s: String) -> String {
    let decoded = urlencoding::decode(&s).map(|c| c.into_owned()).unwrap_or(s);
    decoded.replace("-nl-", "\n")
}

pub fn decode_token(s: &str) -> String {
    urlencoding::decode(s)
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| s.to_string())
        .replace('_', " ")
}

pub fn svg_response(svg: &str, cache: &str) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/svg+xml; charset=utf-8"),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_str(cache).unwrap());
    headers.insert(
        header::HeaderName::from_static("access-control-allow-origin"),
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::HeaderName::from_static("cross-origin-resource-policy"),
        HeaderValue::from_static("cross-origin"),
    );
    (headers, svg.to_string()).into_response()
}

pub fn err_svg(msg: &str) -> Response {
    // Always 200 for embed URLs: CDNs/browsers treat 5xx as a dead <img>.
    // Keep a readable SVG so the failure is visible and cacheable only briefly.
    let safe = esc(msg);
    let short: String = safe.chars().take(120).collect();
    let body = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"480\" height=\"96\" role=\"img\">\
         <rect width=\"100%\" height=\"100%\" rx=\"12\" fill=\"#141821\"/>\
         <text x=\"20\" y=\"38\" fill=\"#ff7b86\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"14\" font-weight=\"600\">Mark couldn&apos;t load this card</text>\
         <text x=\"20\" y=\"62\" fill=\"#9aa3b5\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\">{short}</text>\
         </svg>"
    );
    svg_response(&body, "public, max-age=30")
}

/// Sample the process clock and format a pure hour-bucket seed for time-based fills.
pub fn current_time_seed() -> String {
    use chrono::{Datelike, Timelike};
    let n = chrono::Utc::now();
    theme::time_seed_from_parts(
        n.year(),
        n.month(),
        n.day(),
        n.hour(),
    )
}
