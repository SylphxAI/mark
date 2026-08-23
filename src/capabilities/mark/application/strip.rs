//! Strip application: pure MarkSpec → strip SVG (the tech identity row).

use crate::capabilities::mark::domain::color::contrasting_fg;
use crate::capabilities::mark::domain::icons::{glyph, normalize_id};
use crate::capabilities::mark::domain::motion::group_wrap;
use crate::capabilities::mark::domain::svg::{esc, svg_doc};
use crate::capabilities::mark::domain::theme;
use crate::capabilities::mark::domain::{
    normalize_animation, normalize_hex_token, MarkSpec, MAX_ICONS,
};

pub fn render(spec: &MarkSpec) -> String {
    let ids: Vec<String> = spec
        .strip
        .icons
        .as_deref()
        .unwrap_or("rust,ts,docker")
        .split([',', '|', ' '])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(normalize_id)
        .take(MAX_ICONS)
        .collect();

    if ids.is_empty() {
        return svg_doc(
            32,
            32,
            "<text x=\"2\" y=\"20\" font-size=\"10\" fill=\"#888\">no icons</text>",
        );
    }

    let (bg, fg) = if let Some(t) = spec.theme.as_deref().and_then(theme::get) {
        (format!("#{}", t.bg), format!("#{}", t.fg))
    } else if let Some(c) = spec.color.as_deref().and_then(normalize_hex_token) {
        (c.clone(), ensure_fg(&c))
    } else {
        ("#0D1117".into(), "#E6EDF3".into())
    };

    let tile = 48u32;
    let gap = 8u32;
    let per = spec.strip.per_line.unwrap_or(12).max(1);
    let cols = ids.len().min(per as usize) as u32;
    let rows = (ids.len() as u32).div_ceil(per);
    let w = cols * tile + (cols.saturating_sub(1)) * gap + 16;
    let h = rows * tile + (rows.saturating_sub(1)) * gap + 16;

    let anim = normalize_animation(spec.animation.as_deref());
    let (open, close) = group_wrap(anim, 0, w, h);
    let mut body = format!(
        "<rect width=\"{w}\" height=\"{h}\" rx=\"12\" fill=\"{bg}\"/>{open}"
    );
    let fallback = "<rect x=\"6\" y=\"6\" width=\"20\" height=\"20\" rx=\"4\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><text x=\"16\" y=\"20\" text-anchor=\"middle\" font-size=\"8\" fill=\"currentColor\">?</text>";
    for (i, id) in ids.iter().enumerate() {
        let col = (i as u32) % per;
        let row = (i as u32) / per;
        let x = 8 + col * (tile + gap);
        let y = 8 + row * (tile + gap);
        let g = glyph(id).unwrap_or(fallback);
        body.push_str(&format!(
            "<g transform=\"translate({x},{y})\" color=\"{fg}\">\
             <rect width=\"{tile}\" height=\"{tile}\" rx=\"10\" fill=\"#ffffff\" fill-opacity=\"0.06\"/>\
             <g transform=\"translate(8,8)\">{g}</g>\
             <title>{}</title></g>",
            esc(id)
        ));
    }
    body.push_str(&close);
    svg_doc(w, h, &body)
}

fn ensure_fg(hex: &str) -> String {
    contrasting_fg(hex.trim_start_matches('#'))
}
