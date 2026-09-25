//! HTTP response helpers for the mark surface.

use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};

use crate::capabilities::mark::domain::hash;
use crate::capabilities::mark::domain::svg::{SVG_CACHE, SVG_EDGE_CACHE};

/// Cache directives are compile-time constants: an invalid header token fails
/// the build instead of panicking on a request path.
const CACHE_CONTROL: HeaderValue = HeaderValue::from_static(SVG_CACHE);
const EDGE_CACHE_CONTROL: HeaderValue = HeaderValue::from_static(SVG_EDGE_CACHE);

pub(crate) fn parse_bool(v: Option<&str>, default: bool) -> bool {
    match v {
        None => default,
        Some(s) => matches!(s.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
    }
}

pub(crate) fn decode_text(s: String) -> String {
    let decoded = urlencoding::decode(&s).map(|c| c.into_owned()).unwrap_or(s);
    decoded.replace("-nl-", "\n")
}
/// The raw `If-None-Match` request header, if any.
pub(crate) fn if_none_match(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
}

/// Stable strong ETag for byte-identical SVG URLs (FNV-1a/64 over the bytes).
///
/// std `DefaultHasher` is explicitly unstable across releases, so ETags use
/// [`hash::fnv1a_64`] — deterministic across processes and deploys for
/// identical bytes, with no new dependency. The tag changes iff the bytes
/// change, which is exactly the immutable-by-URL contract (query-pinned
/// content).
///
/// The emitted tag is always `"` + 16 lowercase hex digits + `"`, so the header
/// conversion below cannot fail for any input.
pub(crate) fn etag_header(svg: &str) -> HeaderValue {
    let h = hash::fnv1a_64(svg.as_bytes());
    let tag = format!("\"{h:016x}\"");
    HeaderValue::from_str(&tag).expect("hex digest tag is a valid header value")
}

fn cache_headers(headers: &mut HeaderMap, etag: &HeaderValue) {
    headers.insert(header::CACHE_CONTROL, CACHE_CONTROL.clone());
    // Explicit edge TTL: Cloudflare honors Cloudflare-CDN-Cache-Control >
    // CDN-Cache-Control > Cache-Control for edge. Origin headers are this
    // product's write; they cannot flip cf-cache-status on dest extensionless
    // `/api/v1/mark*` + `/badge/*`. Live edge HIT is Apps (SaaS Custom
    // Hostname + Cache Rule keyed on the full query string). The edge
    // directive must be present so that rule has TTL to honor.
    headers.insert(
        header::HeaderName::from_static("cdn-cache-control"),
        EDGE_CACHE_CONTROL.clone(),
    );
    headers.insert(
        header::HeaderName::from_static("cloudflare-cdn-cache-control"),
        EDGE_CACHE_CONTROL.clone(),
    );
    headers.insert(header::ETAG, etag.clone());
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
    // `img-src data:` lets a validated base64 badge logo paint when the SVG
    // is opened directly (image-mode SVG never runs script).
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(
            "default-src 'none'; img-src data:; style-src 'unsafe-inline'; script-src 'none'; object-src 'none'; base-uri 'none'",
        ),
    );
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
}

/// Returns true when `If-None-Match` matches `etag` (exact, weak, or `*`).
fn etag_matches(if_none_match: Option<&str>, etag: &HeaderValue) -> bool {
    fn norm(s: &str) -> &str {
        let s = s.trim();
        let s = s.strip_prefix("W/").unwrap_or(s);
        s.trim().trim_matches('"')
    }
    let etag = etag.to_str().unwrap_or_default();
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

/// SVG response with immutable long-cache + ETag + conditional-GET support.
///
/// `if_none_match` is the raw `If-None-Match` request header value, if any.
/// On match returns `304 Not Modified` with the same cache/ETag headers and
/// no body (HIT-equivalent verifiable without a CDN); otherwise `200` with
/// identical bytes. Security headers (CSP/nosniff/CORP) are preserved on 200;
/// 304 carries cache + ETag per RFC 7232 (no body, no content-type).
///
/// Every SVG body is immutable by URL contract (`MARK-CDN`), so the cache
/// policy lives here, beside the header write, instead of being threaded in.
pub(crate) fn svg_response_conditional(svg: &str, if_none_match: Option<&str>) -> Response {
    let etag = etag_header(svg);
    if etag_matches(if_none_match, &etag) {
        let mut headers = HeaderMap::new();
        cache_headers(&mut headers, &etag);
        return (StatusCode::NOT_MODIFIED, headers).into_response();
    }
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/svg+xml; charset=utf-8"),
    );
    cache_headers(&mut headers, &etag);
    security_headers(&mut headers);
    (headers, svg.to_string()).into_response()
}

// Render is total by construction (ADR-0003): every spec normalizes, nothing
// fails, so there is no error-SVG path and no clock sampling.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn etag_header_is_a_quoted_hex_digest() {
        let tag = etag_header("<svg/>");
        let s = tag.to_str().expect("etag is ASCII by construction");
        assert!(s.starts_with('"') && s.ends_with('"'), "quoted: {s}");
        let hex = &s[1..s.len() - 1];
        assert_eq!(hex.len(), 16, "FNV-1a/64 is 16 hex digits: {s}");
        assert!(
            hex.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
            "lowercase hex only: {s}"
        );
        assert_eq!(tag, etag_header("<svg/>"), "same bytes, same tag");
        assert_ne!(
            tag,
            etag_header("<svg />"),
            "different bytes, different tag"
        );
    }

    #[test]
    fn etag_matches_exact_weak_and_star() {
        let tag = etag_header("<svg/>");
        let strong = tag.to_str().unwrap().to_string();
        let weak = format!("W/{}", strong);
        assert!(etag_matches(Some(&strong), &tag));
        assert!(etag_matches(Some(&weak), &tag));
        assert!(etag_matches(Some("*"), &tag));
        assert!(etag_matches(Some(&format!("{}, {}", weak, strong)), &tag));
        assert!(!etag_matches(Some("\"deadbeefdeadbeef\""), &tag));
        assert!(!etag_matches(None, &tag));
    }
}
