//! HTTP response helpers for the mark surface.

use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
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

/// Stable strong ETag for byte-identical SVG URLs (FNV-1a/64 over the bytes).
///
/// std `DefaultHasher` is explicitly unstable across releases, so ETags use an
/// inline FNV-1a/64 — deterministic across processes and deploys for identical
/// bytes, with no new dependency. The tag changes iff the bytes change, which
/// is exactly the immutable-by-URL contract (query-pinned content).
pub fn etag_for(svg: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut h = FNV_OFFSET;
    for b in svg.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    format!("\"{h:016x}\"")
}

fn cache_headers(headers: &mut HeaderMap, cache: &str, etag: &str) {
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_str(cache).unwrap());
    // Explicit edge TTL: Cloudflare honors Cloudflare-CDN-Cache-Control >
    // CDN-Cache-Control > Cache-Control for edge. Origin intent alone cannot
    // flip cf-cache-status on extensionless API paths (needs a Cache Rule),
    // but the edge directive must be present so the rule has TTL to honor.
    headers.insert(
        header::HeaderName::from_static("cdn-cache-control"),
        HeaderValue::from_str(
            crate::capabilities::mark::domain::svg::SVG_EDGE_CACHE,
        )
        .unwrap(),
    );
    headers.insert(
        header::HeaderName::from_static("cloudflare-cdn-cache-control"),
        HeaderValue::from_str(
            crate::capabilities::mark::domain::svg::SVG_EDGE_CACHE,
        )
        .unwrap(),
    );
    headers.insert(header::ETAG, HeaderValue::from_str(etag).unwrap());
}

fn security_headers(headers: &mut HeaderMap) {
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
}

/// Returns true when `If-None-Match` matches `etag` (exact, weak, or `*`).
fn etag_matches(if_none_match: Option<&str>, etag: &str) -> bool {
    fn norm(s: &str) -> &str {
        let s = s.trim();
        let s = s.strip_prefix("W/").unwrap_or(s);
        s.trim().trim_matches('"')
    }
    match if_none_match {
        None => false,
        Some(v) => {
            let v = v.trim();
            if v == "*" {
                return true;
            }
            let want = norm(etag);
            v.split(',').any(|t| norm(t) == want)
        }
    }
}

pub fn svg_response(svg: &str, cache: &str) -> Response {
    svg_response_conditional(svg, cache, None)
}

/// SVG response with immutable long-cache + ETag + conditional-GET support.
///
/// `if_none_match` is the raw `If-None-Match` request header value, if any.
/// On match returns `304 Not Modified` with the same cache/ETag headers and
/// no body (HIT-equivalent verifiable without a CDN); otherwise `200` with
/// identical bytes. Security headers (CSP/nosniff/CORP) are preserved on 200;
/// 304 carries cache + ETag per RFC 7232 (no body, no content-type).
pub fn svg_response_conditional(
    svg: &str,
    cache: &str,
    if_none_match: Option<&str>,
) -> Response {
    let etag = etag_for(svg);
    if etag_matches(if_none_match, &etag) {
        let mut headers = HeaderMap::new();
        cache_headers(&mut headers, cache, &etag);
        return (StatusCode::NOT_MODIFIED, headers).into_response();
    }
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/svg+xml; charset=utf-8"),
    );
    cache_headers(&mut headers, cache, &etag);
    security_headers(&mut headers);
    (headers, svg.to_string()).into_response()
}

// Render is total by construction (ADR-0003): every spec normalizes, nothing
// fails, so there is no error-SVG path and no clock sampling.
