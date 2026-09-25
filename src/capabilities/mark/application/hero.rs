//! Hero application: pure MarkSpec → hero SVG (the flagship mark).
//!
//! Pure and deterministic (ADR-0003): the same spec renders the same SVG
//! forever — no clock, no upstream, no state.

use crate::capabilities::mark::domain::art::Art;
use crate::capabilities::mark::domain::color::{legible_ink, resolve_fill, FillPlan};
use crate::capabilities::mark::domain::motion::{ambient_gain, text_children, text_open_attrs};
use crate::capabilities::mark::domain::shapes::{shape_background, shape_defs};
use crate::capabilities::mark::domain::svg::{credit_mark, ensure_hash, esc, svg_doc};
use crate::capabilities::mark::domain::text::{
    content_family, fit_line, line_advance, monogram, Metric,
};
use crate::capabilities::mark::domain::{
    cap_text, normalize_animation, normalize_layout, MarkSpec, MAX_DESC_CHARS, MAX_LINES,
    MAX_TEXT_CHARS,
};

/// Share of the line budget text may fill. The width metric follows the
/// system UI sans; viewers without it fall back to wider faces (DejaVu Sans is
/// ~8% wider), so text is fitted with headroom instead of edge to edge.
const FIT_MARGIN: f32 = 0.92;

/// Largest size ≤ `size` (down to `min_ratio` of it) at which every line fits
/// `budget`; lines that still overflow at the floor are truncated by the
/// caller. Short text keeps `size` exactly.
fn shrink_to_fit(lines: &[&str], budget: f32, size: u32, min_ratio: f32, metric: Metric) -> u32 {
    let widest = lines
        .iter()
        .map(|l| line_advance(l, size as f32, metric))
        .fold(0.0_f32, f32::max);
    if widest <= budget || widest <= 0.0 {
        return size;
    }
    let fitted = (size as f32 * budget / widest).floor();
    fitted.max((size as f32 * min_ratio).ceil()) as u32
}

fn line_max_px(width: u32, x: f32, anchor: &str) -> f32 {
    let pad = (width as f32 * 0.04).clamp(16.0, 36.0);
    match anchor {
        "start" | "left" => (width as f32 - x - pad).max(24.0),
        "end" | "right" => (x - pad).max(24.0),
        _ => (width as f32 - pad * 2.0).max(24.0),
    }
}

/// Paint/style choices shared by every text node of one hero render.
struct TextStyle<'a> {
    font_color: &'a str,
    font_family: &'a str,
    anchor: &'a str,
}

impl TextStyle<'_> {
    fn typewriter_line(
        &self,
        line: &str,
        x: f32,
        y: f32,
        font_size: u32,
        begin: f32,
        char_dur: f32,
    ) -> String {
        let (font_color, font_family, anchor) = (self.font_color, self.font_family, self.anchor);
        let fs = font_size as f32;
        let total = line_advance(line, fs, Metric::Display);
        // Anchor the run as a whole, then place glyphs left→right.
        let start_x = match anchor {
            "start" | "left" => x,
            "end" | "right" => x - total,
            _ => x - total / 2.0,
        };

        let mut out = String::new();
        let mut cx = start_x;
        let mut t = begin;
        for ch in line.chars() {
            let adv = Metric::Display.advance(ch, fs);
            let glyph_x = cx;
            out.push_str(&format!(
                "<text x=\"{glyph_x}\" y=\"{y}\" text-anchor=\"start\" dominant-baseline=\"middle\" \
             font-family=\"{font_family}\" font-weight=\"650\" letter-spacing=\"0\" font-size=\"{font_size}\" \
             fill=\"{font_color}\" opacity=\"0\">\
               <animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.01s\" begin=\"{t}s\" fill=\"freeze\"/>\
               {}</text>",
                esc(&ch.to_string()),
            ));
            cx += adv;
            t += char_dur;
        }
        // Blinking cursor after typed line
        let cursor_x = cx + fs * 0.06;
        let cy = y - fs * 0.42;
        let cw = (fs * 0.08).clamp(2.0, 5.0);
        let chh = fs * 0.78;
        let cursor_begin = begin + line.chars().count() as f32 * char_dur;
        out.push_str(&format!(
            "<rect x=\"{cursor_x}\" y=\"{cy}\" width=\"{cw}\" height=\"{chh}\" rx=\"1.5\" fill=\"{font_color}\" opacity=\"0\">\
           <animate attributeName=\"opacity\" values=\"0;0;1;1;0;0\" keyTimes=\"0;0.01;0.02;0.48;0.52;1\" \
             dur=\"1.05s\" begin=\"{cursor_begin}s\" repeatCount=\"indefinite\"/>\
         </rect>"
        ));
        // Full string kept for accessibility / crawl / tests (invisible).
        out.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" text-anchor=\"{anchor}\" dominant-baseline=\"middle\" font-size=\"1\" fill=\"{font_color}\" opacity=\"0\">{}</text>",
        esc(line),
    ));
        out
    }
}

fn plate_chrome(width: u32, height: u32, mono: &str, plan: &FillPlan, ink: &str) -> String {
    let (accent, base, warm, glow) = (
        plan.accent.as_str(),
        plan.base.as_str(),
        plan.warm.as_str(),
        plan.glow.as_str(),
    );
    let hf = height as f32;
    let wf = width as f32;
    // Left calm field so type always wins — tinted, not pure black.
    let scrim_w = (wf * 0.46).clamp(180.0, 420.0);
    let tile = (hf * 0.28).clamp(56.0, 120.0);
    let tile_x = wf * 0.055;
    let tile_y = hf * 0.14;
    format!(
        "         <defs>           <linearGradient id=\"plateScrim\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">             <stop offset=\"0%\" stop-color=\"{base}\" stop-opacity=\"0.72\"/>             <stop offset=\"55%\" stop-color=\"{base}\" stop-opacity=\"0.28\"/>             <stop offset=\"100%\" stop-color=\"{base}\" stop-opacity=\"0\"/>           </linearGradient>           <linearGradient id=\"plateTile\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\">             <stop offset=\"0%\" stop-color=\"{accent}\" stop-opacity=\"0.95\"/>             <stop offset=\"55%\" stop-color=\"{warm}\" stop-opacity=\"0.72\"/>             <stop offset=\"100%\" stop-color=\"{glow}\" stop-opacity=\"0.55\"/>           </linearGradient>           <radialGradient id=\"plateGlow\" cx=\"35%\" cy=\"30%\" r=\"70%\">             <stop offset=\"0%\" stop-color=\"{glow}\" stop-opacity=\"0.55\"/>             <stop offset=\"100%\" stop-color=\"{accent}\" stop-opacity=\"0\"/>           </radialGradient>         </defs>         <rect x=\"0\" y=\"0\" width=\"{scrim_w}\" height=\"{hf}\" fill=\"url(#plateScrim)\"/>         <rect x=\"{tile_x}\" y=\"{tile_y}\" width=\"{tile}\" height=\"{tile}\" rx=\"{rx}\"            fill=\"url(#plateTile)\" stroke=\"{accent}\" stroke-opacity=\"0.75\" stroke-width=\"1.5\"/>         <rect x=\"{tile_x}\" y=\"{tile_y}\" width=\"{tile}\" height=\"{tile}\" rx=\"{rx}\" fill=\"url(#plateGlow)\"/>         <text x=\"{tx}\" y=\"{ty}\" text-anchor=\"middle\" dominant-baseline=\"middle\"            font-family=\"ui-sans-serif,system-ui,sans-serif\" font-weight=\"750\"            font-size=\"{fs}\" letter-spacing=\"-0.04em\" fill=\"{ink}\">{mono}</text>",
        rx = (tile * 0.18).clamp(10.0, 22.0),
        tx = tile_x + tile / 2.0,
        ty = tile_y + tile / 2.0 + 1.0,
        fs = (tile * 0.38).clamp(22.0, 48.0),
        mono = esc(mono),
    )
}

pub fn render(spec: &MarkSpec) -> String {
    let art = Art::parse(spec.art.as_deref().unwrap_or("waving"));
    // Cards need taller canvases (e.g. 768); strips stay ~200–320.
    let height = spec.height.unwrap_or(220).clamp(40, 900);
    let width = spec.width.unwrap_or(880).clamp(200, 1600);
    let layout = normalize_layout(spec.hero.layout.as_deref());
    // product/oss/org types default into plate composition when layout omitted
    let layout = if layout == "default"
        && matches!(art, Art::Product | Art::Oss | Art::Org)
        && spec.hero.layout.is_none()
    {
        "plate"
    } else {
        layout
    };

    let anim = normalize_animation(spec.animation.as_deref());
    let gain = ambient_gain(anim);

    let seed = format!("{art}-{}", spec.text.as_deref().unwrap_or(""));
    let fill = resolve_fill(spec.color.as_deref(), spec.theme.as_deref(), &seed, "mg");

    // Strict color grammar: ink is derived from the resolved palette, so only
    // canonical hex tokens reach SVG attributes.
    let font_color = if art == Art::Transparent {
        ensure_hash(&fill.fg)
    } else {
        ensure_hash(&legible_ink(&fill.fg, &fill.base, &fill.accent))
    };
    let font_family = content_family(spec.font.as_deref());

    let text = cap_text(spec.text.as_deref().unwrap_or(""), MAX_TEXT_CHARS);
    let desc = cap_text(spec.desc.as_deref().unwrap_or(""), MAX_DESC_CHARS);

    // Typography and placement come from the layout family: the grammar exposes
    // no other hero geometry.
    let (align, align_y, desc_align, desc_align_y, default_fs, desc_size, anchor) = match layout {
        "plate" => {
            let fs = if height >= 480 {
                56
            } else if height >= 320 {
                48
            } else {
                42
            };
            let ds = if height >= 480 { 20 } else { 16 };
            let ay = if desc.is_empty() { 58.0 } else { 52.0 };
            let dy = if height >= 480 { 66.0 } else { 72.0 };
            (14.0, ay, 14.0, dy, fs, ds, "start")
        }
        "terminal" => {
            let fs = if height >= 400 { 44 } else { 36 };
            (
                12.0,
                if desc.is_empty() { 50.0 } else { 46.0 },
                12.0,
                68.0,
                fs,
                15,
                "start",
            )
        }
        _ => (
            50.0,
            if desc.is_empty() { 50.0 } else { 44.0 },
            50.0,
            68.0,
            48,
            18,
            "middle",
        ),
    };
    // Large canvases (social previews, tall headers) scale type with the
    // canvas; the default 880×220 banner and anything narrower or shorter keep
    // the layout's sizes exactly.
    let scale = (width as f32 / 700.0)
        .min(height as f32 / 220.0)
        .clamp(1.0, 2.4);
    let default_fs = (default_fs as f32 * scale).round() as u32;
    let desc_size = (desc_size as f32 * scale).round() as u32;
    let font_size = if text.is_empty() { 40 } else { default_fs };

    // Plate lifts title below monogram row
    let title_y_bias = if layout == "plate" && height >= 280 {
        height as f32 * 0.08
    } else {
        0.0
    };

    let x0 = width as f32 * align / 100.0;
    let title_budget = line_max_px(width, x0, anchor) * FIT_MARGIN;
    let raw_lines: Vec<&str> = text
        .split('\n')
        .filter(|l| !l.is_empty())
        .take(MAX_LINES)
        .collect();
    // Titles are drawn at weight 650: measure with the bold table.
    let font_size = shrink_to_fit(&raw_lines, title_budget, font_size, 0.6, Metric::Bold);
    let lines: Vec<String> = raw_lines
        .iter()
        .map(|l| fit_line(l, title_budget, font_size as f32, Metric::Bold))
        .collect();
    let mut text_nodes = String::new();
    let n = lines.len().max(1) as f32;
    let use_typewriter = anim == "type";
    let style = TextStyle {
        font_color: &font_color,
        font_family,
        anchor,
    };

    for (i, line) in lines.iter().enumerate() {
        let dy = (i as f32 - (n - 1.0) / 2.0) * font_size as f32 * 1.15;
        let x = x0;
        let y = height as f32 * align_y / 100.0 + dy + title_y_bias;

        if use_typewriter {
            let base = i as f32 * 0.55;
            text_nodes.push_str(&style.typewriter_line(line, x, y, font_size, base, 0.055));
            continue;
        }

        let open_extra = text_open_attrs(anim, i, width, height);
        let children = text_children(anim, i, width, height);
        text_nodes.push_str(&format!(
            "<text x=\"{x}\" y=\"{y}\" text-anchor=\"{anchor}\" dominant-baseline=\"middle\" \
             font-family=\"{font_family}\" font-weight=\"650\" letter-spacing=\"-0.02em\" font-size=\"{font_size}\" \
             fill=\"{font_color}\"{open_extra}>{content}{children}</text>",
            content = esc(line),
        ));
    }

    let desc_node = if !desc.is_empty() {
        let dx = width as f32 * desc_align / 100.0;
        let dy = height as f32 * desc_align_y / 100.0
            + if layout == "plate" {
                title_y_bias * 0.35
            } else {
                0.0
            };
        let desc_budget = line_max_px(width, dx, anchor) * FIT_MARGIN;
        let desc_size = shrink_to_fit(
            &[desc.as_str()],
            desc_budget,
            desc_size,
            0.8,
            Metric::Display,
        );
        let desc = fit_line(&desc, desc_budget, desc_size as f32, Metric::Display);
        if use_typewriter {
            let base = lines.len() as f32 * 0.55 + 0.2;
            style.typewriter_line(&desc, dx, dy, desc_size, base, 0.04)
        } else {
            let open_extra = text_open_attrs(anim, lines.len().max(1), width, height);
            let children = text_children(anim, lines.len().max(1), width, height);
            format!(
                "<text x=\"{dx}\" y=\"{dy}\" text-anchor=\"{anchor}\" dominant-baseline=\"middle\" \
                 font-family=\"{font_family}\" font-size=\"{desc_size}\" \
                 font-weight=\"450\" letter-spacing=\"0.01em\" fill=\"{font_color}\" fill-opacity=\"0.82\"{open_extra}>{}{children}</text>",
                esc(&desc),
            )
        }
    } else {
        String::new()
    };

    let plate = if layout == "plate" && !text.is_empty() {
        plate_chrome(width, height, &monogram(&text), &fill, &font_color)
    } else {
        String::new()
    };

    // Terminal: faint top rule
    let terminal_rule = if layout == "terminal" {
        format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"2\" rx=\"1\" fill=\"{font_color}\" fill-opacity=\"0.35\"/>",
            x = width as f32 * 0.06,
            y = height as f32 * 0.12,
            w = width as f32 * 0.28,
        )
    } else {
        String::new()
    };

    let body = format!(
        "<defs>{}{}</defs>{}{}{}{}{}{}",
        fill.defs,
        shape_defs(art, gain, &fill),
        shape_background(art, width, height, &fill, gain),
        plate,
        terminal_rule,
        text_nodes,
        desc_node,
        credit_mark(width, height, spec.credit),
    );

    svg_doc(width, height, &body)
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use crate::capabilities::mark::domain::normalize_layout;

    #[test]
    fn normalize_layout_keeps_only_the_dest_vocabulary() {
        assert_eq!(normalize_layout(Some("plate")), "plate");
        assert_eq!(normalize_layout(Some("terminal")), "terminal");
        assert_eq!(normalize_layout(Some("signal")), "signal");
        assert_eq!(normalize_layout(None), "default");
        // Retired predecessor aliases are unknown input, not a second vocabulary.
        assert_eq!(normalize_layout(Some("card")), "default");
        assert_eq!(normalize_layout(Some("mono")), "default");
    }

    #[test]
    fn monogram_two_words() {
        assert_eq!(monogram("PDF Reader MCP"), "PR");
        assert_eq!(monogram("coderag"), "CO");
    }
}
