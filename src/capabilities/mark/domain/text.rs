//! Text shaping — the single authority for display text.
//!
//! Every mark measures and crops text with the same two metrics and the same
//! ellipsis rule, so a form cannot silently invent its own width model.

/// Which painted metric the text is measured with.
///
/// `Display` is the regular-weight UI sans used by hero/pill/strip/deploy;
/// `Bold` is the heavier profile-card title, whose sidebearings run wider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Metric {
    Display,
    Bold,
}

impl Metric {
    /// Relative advance of one glyph at `font_size`.
    pub(crate) fn advance(self, ch: char, font_size: f32) -> f32 {
        match self {
            Self::Display => font_size * display_unit(ch),
            // Bold name weight plus sidebearings run wider than a regular table.
            Self::Bold => font_size * bold_unit(ch) * 1.12,
        }
    }
}

fn display_unit(ch: char) -> f32 {
    match ch {
        ' ' => 0.30,
        '\u{00A0}' => 0.30,
        'i' | 'l' | 'I' | 'j' | 't' | 'f' | 'r' | '|' | '\'' | '`' | '!' | '.' | ',' | ':'
        | ';' => 0.34,
        'm' | 'w' | 'M' | 'W' | '@' | '%' => 0.78,
        '1' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' => 0.40,
        c if c.is_ascii_uppercase() => 0.58,
        c if c.is_ascii_digit() => 0.54,
        _ => 0.52,
    }
}

fn bold_unit(ch: char) -> f32 {
    match ch {
        ' ' | '\u{00A0}' => 0.30,
        'i' | 'l' | 'I' | 'j' | 't' | 'f' | 'r' | '|' | '\'' | '`' | '!' | '.' | ',' | ':'
        | ';' => 0.34,
        'm' | 'w' | 'M' | 'W' | '@' | '%' => 0.95,
        '1' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' => 0.40,
        c if c.is_ascii_uppercase() => 0.70,
        c if c.is_ascii_digit() => 0.58,
        c if !c.is_ascii() && c.is_alphanumeric() => 1.05,
        _ => 0.60,
    }
}

/// Total advance of a painted line.
pub(crate) fn line_advance(line: &str, font_size: f32, metric: Metric) -> f32 {
    line.chars().map(|c| metric.advance(c, font_size)).sum()
}

/// Crop a line to `max_px`, marking the cut with `…`.
pub(crate) fn fit_line(line: &str, max_px: f32, font_size: f32, metric: Metric) -> String {
    if line_advance(line, font_size, metric) <= max_px {
        return line.to_string();
    }
    let ell = '\u{2026}';
    let budget = (max_px - metric.advance(ell, font_size)).max(0.0);
    let mut out = String::new();
    let mut used = 0.0;
    for ch in line.chars() {
        let adv = metric.advance(ch, font_size);
        if used + adv > budget {
            break;
        }
        out.push(ch);
        used += adv;
    }
    out.push(ell);
    out
}

/// Cap a display string at `max` chars, marking truncation with `…`.
/// Total length never exceeds `max`.
///
/// This is the public bounded-input rule (`docs/capabilities.md`, limits):
/// contract tests assert it directly.
pub fn cap_text(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('\u{2026}');
    out
}

/// Two-letter monogram from a display name (profile tile + hero plate).
///
/// Letters come from the supplied name. The `MK` fallback is only for empty
/// or punctuation-only input — never a substitute for non-Latin letters.
pub(crate) fn monogram(text: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn cap_text_marks_truncation_and_stays_within_budget() {
        assert_eq!(cap_text("short", 10), "short");
        let capped = cap_text(&"x".repeat(500), 100);
        assert_eq!(capped.chars().count(), 100);
        assert!(capped.ends_with('…'));
    }

    #[test]
    fn fit_line_keeps_short_text_and_marks_the_cut() {
        assert_eq!(fit_line("Ship it", 400.0, 16.0, Metric::Display), "Ship it");
        let long = "A Very Long Display Name That Should Not Escape The Canvas";
        let fitted = fit_line(long, 120.0, 16.0, Metric::Display);
        assert!(fitted.ends_with('…'), "cut is marked: {fitted}");
        assert!(
            line_advance(&fitted, 16.0, Metric::Display) <= 120.0 + 0.001,
            "fitted line stays inside the budget"
        );
        assert!(fitted.chars().count() < long.chars().count());
    }

    #[test]
    fn bold_metric_is_wider_than_display() {
        let wide = "WWWWMMMM";
        assert!(
            line_advance(wide, 24.0, Metric::Bold) > line_advance(wide, 24.0, Metric::Display),
            "profile titles measure wider than hero text"
        );
    }
}
