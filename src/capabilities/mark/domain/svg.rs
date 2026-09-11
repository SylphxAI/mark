//! SVG primitives.

pub(crate) fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

pub(crate) fn strip_hash(hex: &str) -> &str {
    hex.strip_prefix('#').unwrap_or(hex)
}

pub(crate) fn ensure_hash(hex: &str) -> String {
    let h = strip_hash(hex);
    if h.is_empty() {
        "#000000".into()
    } else {
        format!("#{h}")
    }
}

pub(crate) fn is_hex_color(v: &str) -> bool {
    let h = strip_hash(v);
    matches!(h.len(), 3 | 6 | 8) && h.chars().all(|c| c.is_ascii_hexdigit())
}

/// Canonical color token for SVG attribute values.
///
/// Accepts `#rgb`, `#rrggbb`, `#rrggbbaa` (with or without `#`); expands
/// 3-digit shorthand to 6. Anything else returns `None` so callers fall back
/// to a trusted paint instead of emitting attacker-controlled attribute text.
pub(crate) fn normalize_hex_token(v: &str) -> Option<String> {
    let h = strip_hash(v.trim());
    if !is_hex_color(h) {
        return None;
    }
    if h.len() == 3 {
        Some(format!(
            "#{}",
            h.chars().flat_map(|c| [c, c]).collect::<String>()
        ))
    } else {
        Some(format!("#{h}"))
    }
}

pub(crate) fn svg_doc(width: u32, height: u32, body: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\" role=\"img\">{body}</svg>"
    )
}

pub fn credit_mark(width: u32, height: u32, enabled: bool) -> String {
    if !enabled {
        return String::new();
    }
    // Product watermark only — not a company brand stamp. Opt-in via credit=1.
    let x = width.saturating_sub(8);
    let y = height.saturating_sub(6);
    format!(
        "<a href=\"https://mark.sylphx.com\" target=\"_blank\" rel=\"noopener\">\
           <text x=\"{x}\" y=\"{y}\" text-anchor=\"end\" font-family=\"ui-sans-serif,system-ui,sans-serif\" \
             font-size=\"9\" fill=\"#ffffff\" fill-opacity=\"0.22\">mark</text></a>"
    )
}

/// CDN-forever contract (MARK-HOST): every mark URL pins its bytes — the render
/// is a pure function of the URL (ADR-0003, no clock/upstream/state), including
/// SMIL-animated variants whose `<animate*>` declarations are part of the bytes.
/// Query-pinned content is therefore immutable: browsers and edge may cache for
/// one year without revalidation. `stale-while-revalidate` keeps edge refresh
/// async. `CDN-Cache-Control` / `Cloudflare-CDN-Cache-Control` carry the same
/// edge TTL so the Apps Cache Rule has origin intent to honor.
///
/// Origin headers are this product's write. Live edge `HIT` on dest
/// extensionless `/api/v1/mark*` + `/badge/*` is Apps (Cloudflare for SaaS
/// Custom Hostname + grey CNAME to `cname.sylphx.com`, plus Cache Everything
/// / eligible-for-cache keyed on the full query string). Hands is generic
/// kube origin only. Origin headers alone cannot flip `cf-cache-status`
/// from DYNAMIC on extensionless API paths.
pub(crate) const SVG_CACHE: &str =
    "public, max-age=31536000, s-maxage=31536000, stale-while-revalidate=86400, immutable";
/// Edge TTL mirror for `CDN-Cache-Control` / `Cloudflare-CDN-Cache-Control`.
pub(crate) const SVG_EDGE_CACHE: &str = "public, s-maxage=31536000, stale-while-revalidate=86400";
