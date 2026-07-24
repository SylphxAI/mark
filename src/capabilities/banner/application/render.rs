//! Banner application: pure BannerInput → SVG composition.

use crate::capabilities::banner::domain::{
    ambient_gain, normalize_animation, normalize_layout, normalize_type, shape_background,
    shape_defs, text_children, text_open_attrs, BannerInput,
};
use crate::shared::color::resolve_fill;
use crate::shared::svg::{credit_mark, ensure_hash, esc, svg_doc};

fn monogram(text: &str) -> String {
    let parts: Vec<&str> = text
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() >= 2 {
        let a = parts[0].chars().next().unwrap_or('O');
        let b = parts[1].chars().next().unwrap_or('S');
        format!("{}{}", a.to_ascii_uppercase(), b.to_ascii_uppercase())
    } else {
        let alnum: String = text
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .take(2)
            .collect::<String>()
            .to_ascii_uppercase();
        if alnum.is_empty() {
            "OS".into()
        } else if alnum.len() == 1 {
            format!("{alnum}{alnum}")
        } else {
            alnum
        }
    }
}

/// True typewriter: per-character opacity + optional cursor.
#[allow(clippy::too_many_arguments)]
fn typewriter_line(
    line: &str,
    x: f32,
    y: f32,
    font_size: u32,
    font_color: &str,
    anchor: &str,
    stroke_attr: &str,
    base_delay: f32,
    char_dur: f32,
) -> String {
    let mut nodes = String::new();
    // Accessible / crawlable full string (characters render as separate nodes)
    nodes.push_str(&format!("<title>{}</title>", esc(line)));
    let mut t = base_delay;
    // Approximate advance for cursor: monospaced-ish width factor
    let advance = font_size as f32 * 0.56;
    let start_x = match anchor {
        "start" => x,
        "end" => x - line.chars().count() as f32 * advance,
        _ => x - (line.chars().count() as f32 * advance) / 2.0,
    };
    let mut cx = start_x;
    for ch in line.chars() {
        let content = esc(&ch.to_string());
        nodes.push_str(&format!(
            "<text x=\"{cx}\" y=\"{y}\" text-anchor=\"start\" dominant-baseline=\"middle\" \
             font-family=\"ui-sans-serif,system-ui,-apple-system,Segoe UI,Helvetica,sans-serif\" \
             font-weight=\"650\" letter-spacing=\"-0.02em\" font-size=\"{font_size}\" \
             fill=\"{font_color}\" opacity=\"0\"{stroke_attr}>\
             {content}\
             <animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.04s\" begin=\"{t}s\" fill=\"freeze\"/>\
             </text>"
        ));
        cx += if ch == ' ' { advance * 0.45 } else { advance };
        t += char_dur;
    }
    // Blinking cursor after typed line
    let cursor_x = cx + 2.0;
    let ch = font_size as f32 * 0.85;
    nodes.push_str(&format!(
        "<rect x=\"{cursor_x}\" y=\"{cy}\" width=\"{cw}\" height=\"{ch}\" rx=\"1.5\" fill=\"{font_color}\" opacity=\"0\">\
           <animate attributeName=\"opacity\" values=\"0;1;1;0;0\" keyTimes=\"0;0.05;0.5;0.55;1\" \
             dur=\"1.0s\" begin=\"{t}s\" repeatCount=\"indefinite\"/>\
         </rect>",
        cy = y - ch * 0.55,
        cw = (font_size as f32 * 0.12).max(2.5),
    ));
    nodes
}

#[allow(clippy::too_many_arguments)]
fn plate_chrome(
    width: u32,
    height: u32,
    mono: &str,
    accent: &str,
    base: &str,
    warm: &str,
    glow: &str,
    ink: &str,
) -> String {
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

pub fn render(input: &BannerInput) -> String {
    let ty = normalize_type(input.type_name.as_deref().unwrap_or("aurora"));
    // Cards need taller canvases (e.g. 768); strips stay ~200–320.
    let height = input.height.unwrap_or(220).clamp(40, 900);
    let width = input.width.unwrap_or(880).clamp(200, 1600);
    let layout = normalize_layout(input.layout.as_deref());
    // product/oss/org types default into plate composition when layout omitted
    let layout = if layout == "default"
        && matches!(ty, "product" | "oss" | "org")
        && input.layout.is_none()
    {
        "plate"
    } else {
        layout
    };

    let section = if input.section.as_deref() == Some("footer") {
        "footer"
    } else {
        "header"
    };
    let anim = normalize_animation(input.animation.as_deref());
    let gain = ambient_gain(anim);

    let seed = input
        .seed
        .clone()
        .unwrap_or_else(|| format!("{ty}-{}", input.text.as_deref().unwrap_or("")));
    let fill = resolve_fill(
        input.color.as_deref(),
        input.theme.as_deref(),
        &seed,
        "mg",
        input.clock_seed.as_deref(),
    );

    let font_color = input
        .font_color
        .as_deref()
        .map(|c| ensure_hash(c.trim_start_matches('#')))
        .unwrap_or_else(|| ensure_hash(&fill.fg));

    let text = input.text.as_deref().unwrap_or("");
    let desc = input.desc.as_deref().unwrap_or("");

    // Layout-driven defaults (explicit query params still win)
    let (def_align, def_align_y, def_desc_align, def_desc_y, def_fs, def_ds, anchor) = match layout {
        "plate" => {
            let fs = if height >= 480 { 56 } else if height >= 320 { 48 } else { 42 };
            let ds = if height >= 480 { 20 } else { 16 };
            let ay = if desc.is_empty() { 58.0 } else { 52.0 };
            let dy = if height >= 480 { 66.0 } else { 72.0 };
            (14.0, ay, 14.0, dy, fs, ds, "start")
        }
        "terminal" => {
            let fs = if height >= 400 { 44 } else { 36 };
            (12.0, if desc.is_empty() { 50.0 } else { 46.0 }, 12.0, 68.0, fs, 15, "start")
        }
        "signal" => (
            50.0,
            if desc.is_empty() { 50.0 } else { 44.0 },
            50.0,
            68.0,
            48,
            18,
            "middle",
        ),
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

    let font_size = input
        .font_size
        .unwrap_or(if text.is_empty() { 40 } else { def_fs })
        .clamp(10, 120);
    let desc_size = input.desc_size.unwrap_or(def_ds).clamp(8, 60);
    let font_align = input.font_align.unwrap_or(def_align).clamp(0.0, 100.0);
    let font_align_y = input.font_align_y.unwrap_or(def_align_y).clamp(0.0, 100.0);
    let desc_align = input.desc_align.unwrap_or(def_desc_align).clamp(0.0, 100.0);
    let desc_align_y = input.desc_align_y.unwrap_or(def_desc_y).clamp(0.0, 100.0);
    let rotate = input.rotate.unwrap_or(0.0);
    let stroke = input
        .stroke
        .as_deref()
        .map(|s| ensure_hash(s.trim_start_matches('#')));
    let stroke_width = input
        .stroke_width
        .unwrap_or(if stroke.is_some() { 1.0 } else { 0.0 });

    // Plate lifts title below monogram row
    let title_y_bias = if layout == "plate" && height >= 280 {
        height as f32 * 0.08
    } else {
        0.0
    };

    let lines: Vec<&str> = text.split('\n').filter(|l| !l.is_empty()).collect();
    let mut text_nodes = String::new();
    let n = lines.len().max(1) as f32;
    let use_typewriter = anim == "type";

    for (i, line) in lines.iter().enumerate() {
        let dy = (i as f32 - (n - 1.0) / 2.0) * font_size as f32 * 1.15;
        let x = width as f32 * font_align / 100.0;
        let y = height as f32 * font_align_y / 100.0 + dy + title_y_bias;
        if input.text_bg {
            let bw = (line.chars().count() as f32 * font_size as f32 * 0.56).max(40.0);
            let bx = if anchor == "start" {
                x - 8.0
            } else {
                x - bw / 2.0
            };
            text_nodes.push_str(&format!(
                "<rect x=\"{bx}\" y=\"{}\" width=\"{bw}\" height=\"{}\" rx=\"8\" fill=\"#000000\" fill-opacity=\"0.25\"/>",
                y - font_size as f32 * 0.8,
                font_size as f32 * 1.15
            ));
        }
        let stroke_attr = if let Some(ref s) = stroke {
            format!(" stroke=\"{s}\" stroke-width=\"{stroke_width}\" paint-order=\"stroke\"")
        } else {
            String::new()
        };

        if use_typewriter {
            let base = i as f32 * 0.55;
            text_nodes.push_str(&typewriter_line(
                line,
                x,
                y,
                font_size,
                &font_color,
                anchor,
                &stroke_attr,
                base,
                0.055,
            ));
            continue;
        }

        let rot = if rotate.abs() > 0.01 {
            format!(" transform=\"rotate({rotate} {x} {y})\"")
        } else {
            String::new()
        };
        let open_extra = text_open_attrs(anim, i, width, height);
        let rot_attr = if open_extra.contains("transform=") {
            String::new()
        } else {
            rot
        };
        let children = text_children(anim, i, width, height);
        text_nodes.push_str(&format!(
            "<text x=\"{x}\" y=\"{y}\" text-anchor=\"{anchor}\" dominant-baseline=\"middle\" \
             font-family=\"ui-sans-serif,system-ui,-apple-system,Segoe UI,Helvetica,sans-serif\" \
             font-weight=\"650\" letter-spacing=\"-0.02em\" font-size=\"{font_size}\" \
             fill=\"{font_color}\"{stroke_attr}{rot_attr}{open_extra}>{content}{children}</text>",
            content = esc(line),
        ));
    }

    let desc_node = if !desc.is_empty() {
        let dx = width as f32 * desc_align / 100.0;
        let dy = height as f32 * desc_align_y / 100.0
            + if layout == "plate" { title_y_bias * 0.35 } else { 0.0 };
        if use_typewriter {
            let base = lines.len() as f32 * 0.55 + 0.2;
            typewriter_line(
                desc,
                dx,
                dy,
                desc_size,
                &font_color,
                anchor,
                "",
                base,
                0.04,
            )
        } else {
            let open_extra = text_open_attrs(anim, lines.len().max(1), width, height);
            let children = text_children(anim, lines.len().max(1), width, height);
            format!(
                "<text x=\"{dx}\" y=\"{dy}\" text-anchor=\"{anchor}\" dominant-baseline=\"middle\" \
                 font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"{desc_size}\" \
                 font-weight=\"450\" letter-spacing=\"0.01em\" fill=\"{font_color}\" fill-opacity=\"0.82\"{open_extra}>{}{children}</text>",
                esc(desc),
            )
        }
    } else {
        String::new()
    };

    let plate = if layout == "plate" && !text.is_empty() {
        plate_chrome(
            width,
            height,
            &monogram(text),
            &fill.accent,
            &fill.base,
            &fill.warm,
            &fill.glow,
            &font_color,
        )
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
        shape_defs(ty, gain, &fill),
        shape_background(ty, width, height, &fill, section, input.reversal, gain),
        plate,
        terminal_rule,
        text_nodes,
        desc_node,
        credit_mark(width, height, input.credit),
    );

    svg_doc(width, height, &body)
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use crate::capabilities::banner::domain::normalize_layout;

    #[test]
    fn normalize_layout_aliases() {
        assert_eq!(normalize_layout(Some("plate")), "plate");
        assert_eq!(normalize_layout(Some("card")), "plate");
        assert_eq!(normalize_layout(Some("terminal")), "terminal");
        assert_eq!(normalize_layout(None), "default");
    }

    #[test]
    fn monogram_two_words() {
        assert_eq!(monogram("PDF Reader MCP"), "PR");
        assert_eq!(monogram("coderag"), "CO");
    }
}
