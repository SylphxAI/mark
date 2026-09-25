//! Card frame and shared primitives for every live card.
//!
//! One frame authority keeps the stats, languages, streak, and repo cards on
//! the same geometry: background (solid or gradient), hairline border, radius,
//! title row, and the calm fallback card shown when data is missing.

use super::palette::{Background, ColorOverrides, Palette};
use crate::capabilities::mark::domain::pill::fmt;
use crate::capabilities::mark::domain::svg::{esc, svg_doc};
use crate::capabilities::mark::domain::text::{fit_line, line_advance, Metric, FONT_UI_SANS};

/// Horizontal padding shared by every card.
pub(crate) const PAD_X: f32 = 25.0;
/// Title baseline.
pub(crate) const TITLE_Y: f32 = 35.0;
pub(crate) const TITLE_SIZE: f32 = 18.0;

/// Presentation knobs every card accepts (github-readme-stats names).
#[derive(Debug, Clone)]
pub(crate) struct CardStyle {
    pub palette: Palette,
    pub hide_border: bool,
    pub radius: f32,
    pub hide_title: bool,
    pub custom_title: Option<String>,
    pub card_width: Option<u32>,
}

impl CardStyle {
    pub(crate) fn new(theme: Option<&str>, colors: &ColorOverrides) -> Self {
        Self {
            palette: super::palette::palette(theme).with(colors),
            hide_border: false,
            radius: 8.0,
            hide_title: false,
            custom_title: None,
            card_width: None,
        }
    }

    /// Requested width clamped to `[min, 1000]`, else `default`.
    pub(crate) fn width(&self, default: u32, min: u32) -> u32 {
        self.card_width
            .map(|w| w.clamp(min, 1000))
            .unwrap_or(default)
    }

    pub(crate) fn title_or(&self, default: String) -> String {
        self.custom_title
            .clone()
            .filter(|t| !t.trim().is_empty())
            .unwrap_or(default)
    }
}

/// Compact, stable SVG number (the kernel's shortest-decimal rule).
pub(crate) fn n2(v: f32) -> String {
    fmt(v as f64)
}

/// Background paint: (`<defs>` to emit, `fill` value). A gradient becomes
/// `#cardbg`.
pub(crate) fn background(bg: &Background) -> (String, String) {
    match bg {
        Background::Solid(c) => (String::new(), c.clone()),
        Background::Gradient { angle, stops } => {
            let mut defs = format!(
                "<defs><linearGradient id=\"cardbg\" gradientTransform=\"rotate({})\" gradientUnits=\"userSpaceOnUse\">",
                n2(*angle)
            );
            let last = stops.len().saturating_sub(1).max(1) as f32;
            for (i, c) in stops.iter().enumerate() {
                defs.push_str(&format!(
                    "<stop offset=\"{}%\" stop-color=\"{c}\"/>",
                    n2(i as f32 / last * 100.0)
                ));
            }
            defs.push_str("</linearGradient></defs>");
            (defs, "url(#cardbg)".to_string())
        }
    }
}

/// Whole card: frame, optional title, and `body` (already positioned).
pub(crate) fn frame(style: &CardStyle, width: u32, height: u32, title: &str, body: &str) -> String {
    let p = &style.palette;
    let (w, h) = (width as f32, height as f32);
    let mut out = String::with_capacity(body.len() + 1024);
    out.push_str(&format!("<title>{}</title>", esc(title)));
    let (defs, fill) = background(&p.bg);
    out.push_str(&defs);
    let stroke = if style.hide_border {
        "stroke-opacity=\"0\"".to_string()
    } else {
        format!("stroke=\"{}\"", p.border)
    };
    out.push_str(&format!(
        "<rect x=\"0.5\" y=\"0.5\" rx=\"{}\" width=\"{}\" height=\"{}\" fill=\"{fill}\" {stroke}/>",
        n2(style.radius.clamp(0.0, 50.0)),
        n2(w - 1.0),
        n2(h - 1.0)
    ));
    out.push_str(&format!(
        "<g font-family=\"{FONT_UI_SANS}\" font-feature-settings=\"'tnum'\">"
    ));
    if !style.hide_title {
        let shown = fit_line(title, w - 2.0 * PAD_X, TITLE_SIZE, Metric::Bold);
        out.push_str(&format!(
            "<text x=\"{PAD_X}\" y=\"{TITLE_Y}\" fill=\"{}\" font-size=\"{TITLE_SIZE}\" font-weight=\"600\">{}</text>",
            p.title,
            esc(&shown)
        ));
    }
    out.push_str(body);
    out.push_str("</g>");
    svg_doc(width, height, &out)
}

/// Top of the body area: below the title row, or near the top edge.
pub(crate) fn body_top(style: &CardStyle) -> f32 {
    if style.hide_title {
        22.0
    } else {
        55.0
    }
}

/// A 16×16 octicon path, scaled and placed at (x, y).
pub(crate) fn icon(path: &str, x: f32, y: f32, size: f32, fill: &str) -> String {
    format!(
        "<svg x=\"{}\" y=\"{}\" width=\"{s}\" height=\"{s}\" viewBox=\"0 0 16 16\"><path fill=\"{fill}\" fill-rule=\"evenodd\" d=\"{path}\"/></svg>",
        n2(x),
        n2(y),
        s = n2(size)
    )
}

/// Greedy word wrap to `max_px`, at most `max_lines` (the last line ellipsized
/// when text remains).
pub(crate) fn wrap(text: &str, max_px: f32, size: f32, max_lines: usize) -> Vec<String> {
    // The metric table tracks UI sans; wide fallback fonts need headroom.
    let max_px = max_px * 0.9;
    let mut lines: Vec<String> = Vec::new();
    let mut cur = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        let candidate = if cur.is_empty() {
            word.to_string()
        } else {
            format!("{cur} {word}")
        };
        if cur.is_empty() || line_advance(&candidate, size, Metric::Display) <= max_px {
            cur = candidate;
            continue;
        }
        lines.push(std::mem::take(&mut cur));
        cur = word.to_string();
        if lines.len() == max_lines {
            let rest = words[i..].join(" ");
            let last = lines.pop().unwrap_or_default();
            lines.push(fit_line(
                &format!("{last} {rest}"),
                max_px,
                size,
                Metric::Display,
            ));
            return lines;
        }
    }
    if !cur.is_empty() && lines.len() < max_lines {
        lines.push(fit_line(&cur, max_px, size, Metric::Display));
    }
    lines
}

/// Calm stand-in card when there is no data: same size as the requested card,
/// never a broken image.
pub(crate) fn notice(
    style: &CardStyle,
    width: u32,
    height: u32,
    title: &str,
    lines: &[&str],
) -> String {
    let p = &style.palette;
    let area = height as f32 - body_top(style);
    let top = body_top(style) + ((area - lines.len() as f32 * 22.0) / 2.0).max(0.0) + 6.0;
    let mut body = String::new();
    let max = width as f32 - 2.0 * PAD_X;
    for (i, line) in lines.iter().enumerate() {
        let size = if i == 0 { 14.0 } else { 12.5 };
        let opacity = if i == 0 { "1" } else { "0.75" };
        body.push_str(&format!(
            "<text x=\"{PAD_X}\" y=\"{}\" fill=\"{}\" fill-opacity=\"{opacity}\" font-size=\"{size}\">{}</text>",
            n2(top + i as f32 * 22.0),
            p.text,
            esc(&fit_line(line, max, size, Metric::Display))
        ));
    }
    frame(style, width, height, title, &body)
}

/// Octicons (MIT) used by the cards.
pub(crate) mod icons {
    pub(crate) const STAR: &str = "M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z";
    pub(crate) const COMMITS: &str = "M1.643 3.143L.427 1.927A.25.25 0 000 2.104V5.75c0 .138.112.25.25.25h3.646a.25.25 0 00.177-.427L2.715 4.215a6.5 6.5 0 11-1.18 4.458.75.75 0 10-1.493.154 8.001 8.001 0 101.6-5.684zM7.75 4a.75.75 0 01.75.75v2.992l2.028.812a.75.75 0 01-.557 1.392l-2.5-1A.75.75 0 017 8.25v-3.5A.75.75 0 017.75 4z";
    pub(crate) const PULL: &str = "M7.177 3.073L9.573.677A.25.25 0 0110 .854v4.792a.25.25 0 01-.427.177L7.177 3.427a.25.25 0 010-.354zM3.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122v5.256a2.251 2.251 0 11-1.5 0V5.372A2.25 2.25 0 011.5 3.25zM11 2.5h-1V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5zm1 10.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.75 12a.75.75 0 100 1.5.75.75 0 000-1.5z";
    pub(crate) const ISSUE: &str = "M8 9.5a1.5 1.5 0 100-3 1.5 1.5 0 000 3zM8 0a8 8 0 100 16A8 8 0 008 0zM1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0z";
    pub(crate) const REPO: &str = "M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z";
    pub(crate) const FORK: &str = "M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z";
    pub(crate) const FLAME: &str = "M7.998 14.5c2.832 0 5-1.98 5-4.5 0-1.463-.68-2.19-1.879-3.383l-.036-.037c-1.013-1.008-2.3-2.29-2.834-4.434-.322.256-.63.579-.864.953-.432.696-.621 1.58-.046 2.73.473.947.67 2.284-.278 3.232-.61.61-1.545.84-2.403.633a2.788 2.788 0 01-1.436-.874A3.21 3.21 0 003 10c0 2.53 2.164 4.5 4.998 4.5zM9.533.753C9.496.34 9.16.009 8.77.146 7.035.75 4.34 3.187 5.997 6.5c.344.689.285 1.218.003 1.5-.419.419-1.54.487-2.04-.832-.173-.454-.659-.762-1.035-.454C2.036 7.44 1.5 8.702 1.5 10c0 3.512 2.998 6 6.498 6s6.5-2.5 6.5-6c0-2.137-1.128-3.26-2.312-4.438-1.19-1.184-2.436-2.425-2.653-4.81z";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_respects_width_and_line_cap() {
        let lines = wrap(
            "one two three four five six seven eight nine ten",
            60.0,
            13.0,
            2,
        );
        assert_eq!(lines.len(), 2);
        assert!(lines[1].ends_with('\u{2026}'));
        assert_eq!(wrap("short", 300.0, 13.0, 3), vec!["short".to_string()]);
        assert!(wrap("", 300.0, 13.0, 3).is_empty());
    }

    #[test]
    fn frame_escapes_title_and_honors_border() {
        let mut style = CardStyle::new(None, &ColorOverrides::default());
        let svg = frame(&style, 300, 100, "<x>", "");
        assert!(svg.contains("&lt;x&gt;"));
        assert!(svg.contains("stroke=\"#e4e2e2\""));
        style.hide_border = true;
        assert!(frame(&style, 300, 100, "t", "").contains("stroke-opacity=\"0\""));
    }
}
