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
        Some(format!(
            "#{}",
            h.chars().flat_map(|c| [c, c]).collect::<String>()
        ))
    } else {
        Some(format!("#{h}"))
    }
}

/// Two-letter monogram from a display name (profile tile + hero plate).
///
/// Letters come from the supplied name. The `MK` fallback is only for empty
/// or punctuation-only input — never a substitute for non-Latin letters.
pub fn monogram(text: &str) -> String {
    let parts: Vec<&str> = text
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() >= 2 {
        match (first_letter(parts[0]), first_letter(parts[1])) {
            (Some(a), Some(b)) => format!("{a}{b}"),
            (Some(a), None) => format!("{a}{a}"),
            (None, Some(b)) => format!("{b}{b}"),
            (None, None) => "MK".into(),
        }
    } else {
        let mut chars = text.chars().filter(|c| c.is_alphanumeric()).map(upcase);
        match (chars.next(), chars.next()) {
            (None, _) => "MK".into(),
            (Some(a), None) => format!("{a}{a}"),
            (Some(a), Some(b)) => format!("{a}{b}"),
        }
    }
}

fn first_letter(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_alphanumeric()).map(upcase)
}

fn upcase(c: char) -> char {
    c.to_uppercase().next().unwrap_or(c)
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

pub const SVG_CACHE: &str = "public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800";
pub const SVG_CACHE_SHORT: &str = "public, max-age=300, s-maxage=600, stale-while-revalidate=3600";

#[cfg(test)]
mod tests {
    use super::monogram;

    #[test]
    fn monogram_latin_and_empty_fallback() {
        assert_eq!(monogram("PDF Reader MCP"), "PR");
        assert_eq!(monogram("coderag"), "CO");
        assert_eq!(monogram("Kyle Tse"), "KT");
        assert_eq!(monogram(""), "MK");
        assert_eq!(monogram("---"), "MK");
        assert_eq!(monogram("Jo"), "JO");
    }

    #[test]
    fn monogram_uses_supplied_non_latin_letters() {
        assert_eq!(monogram("日本語"), "日本");
        assert_eq!(monogram("李小龙"), "李小");
        assert_eq!(monogram("山田 太郎"), "山太");
        assert_eq!(monogram("Владимир"), "ВЛ");
        assert_eq!(monogram("Émile Zola"), "ÉZ");
    }
}
