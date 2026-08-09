//! HTTP response helpers for the mark surface.

use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};

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
    // Defense-in-depth: SVG is served as a navigable document on a public
    // first-party origin. Inputs are validated/escaped; CSP blocks script
    // execution even if a future render bug slips an attribute through.
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(
            "default-src 'none'; style-src 'unsafe-inline'; script-src 'none'; object-src 'none'; base-uri 'none'",
        ),
    );
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    (headers, svg.to_string()).into_response()
}

// Render is total by construction (ADR-0003): every spec normalizes, nothing
// fails, so there is no error-SVG path and no clock sampling.
