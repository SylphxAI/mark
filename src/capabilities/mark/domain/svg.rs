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

/// Canonical color token for SVG attribute values.
///
/// Accepts `#rgb`, `#rrggbb`, `#rrggbbaa` (with or without `#`); expands
/// 3-digit shorthand to 6. Anything else returns `None` so callers fall back
/// to a trusted paint instead of emitting attacker-controlled attribute text.
pub fn normalize_hex_token(v: &str) -> Option<String> {
    let h = strip_hash(v.trim());
    if !is_hex_color(h) {
        return None;
    }
    if h.len() == 3 {
        Some(format!("#{}", h.chars().flat_map(|c| [c, c]).collect::<String>()))
    } else {
        Some(format!("#{h}"))
    }
}

/// Relative advance for system UI sans at banner/pill sizes.
pub fn char_advance(ch: char, font_size: f32) -> f32 {
    let unit = match ch {
        ' ' => 0.30,
        '\u{00A0}' => 0.30,
        'i' | 'l' | 'I' | 'j' | 't' | 'f' | 'r' | '|' | '\'' | '`' | '!' | '.' | ',' | ':' | ';' => {
            0.34
        }
        'm' | 'w' | 'M' | 'W' | '@' | '%' => 0.78,
        '1' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' => 0.40,
        c if c.is_ascii_uppercase() => 0.58,
        c if c.is_ascii_digit() => 0.54,
        _ => 0.52,
    };
    font_size * unit
}

pub fn line_advance(line: &str, font_size: f32) -> f32 {
    line.chars().map(|c| char_advance(c, font_size)).sum()
}

/// Cap a display string at `max` chars, marking truncation with `…`.
/// Total length never exceeds `max`.
pub fn cap_text(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('\u{2026}');
    out
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
    // Product watermark only — not a company brand stamp. Opt-in via credit=1.
    let x = width.saturating_sub(8);
    let y = height.saturating_sub(6);
    format!(
        "<a href=\"https://mark.sylphx.com\" target=\"_blank\" rel=\"noopener\">\
           <text x=\"{x}\" y=\"{y}\" text-anchor=\"end\" font-family=\"ui-sans-serif,system-ui,sans-serif\" \
             font-size=\"9\" fill=\"#ffffff\" fill-opacity=\"0.22\">mark</text></a>"
    )
}

pub const SVG_CACHE: &str =
    "public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800";
pub const SVG_CACHE_SHORT: &str =
    "public, max-age=300, s-maxage=600, stale-while-revalidate=3600";
