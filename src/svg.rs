//! SVG primitives.

pub fn esc(s: &str) -> String {
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

pub fn strip_hash(hex: &str) -> &str {
    hex.strip_prefix('#').unwrap_or(hex)
}

pub fn ensure_hash(hex: &str) -> String {
    let h = strip_hash(hex);
    if h.is_empty() {
        "#000000".into()
    } else {
        format!("#{h}")
    }
}

pub fn is_hex_color(v: &str) -> bool {
    let h = strip_hash(v);
    matches!(h.len(), 3 | 6 | 8) && h.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn svg_doc(width: u32, height: u32, body: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\" role=\"img\">{body}</svg>"
    )
}

pub fn credit_mark(width: u32, height: u32, enabled: bool) -> String {
    if !enabled {
        return String::new();
    }
    let x = width.saturating_sub(8);
    let y = height.saturating_sub(6);
    format!(
        "<a href=\"https://sylphx.com\" target=\"_blank\" rel=\"noopener\"><text x=\"{x}\" y=\"{y}\" text-anchor=\"end\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"9\" fill=\"#ffffff\" fill-opacity=\"0.35\">sylphx</text></a>"
    )
}

pub const SVG_CACHE: &str =
    "public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800";
pub const SVG_CACHE_SHORT: &str =
    "public, max-age=300, s-maxage=600, stale-while-revalidate=3600";
