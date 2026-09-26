//! Hero application: pure MarkSpec → hero SVG (the flagship mark).
//!
//! Pure and deterministic (ADR-0003): the same spec renders the same SVG
//! forever — no clock, no upstream, no state.

use crate::capabilities::mark::domain::art::Art;
use crate::capabilities::mark::domain::color::resolve_palette;
use crate::capabilities::mark::domain::motion::{background_moves, text_children, text_open_attrs};
use crate::capabilities::mark::domain::shapes::{stage, Stage};
use crate::capabilities::mark::domain::svg::{credit_mark, esc, svg_doc};
use crate::capabilities::mark::domain::text::{
    content_family, fit_line, line_advance, Metric, FONT_MONO,
};
use crate::capabilities::mark::domain::{
    cap_text, normalize_animation, normalize_layout, MarkSpec, MAX_DESC_CHARS, MAX_LINES,
    MAX_TEXT_CHARS,
};

/// Share of the text column a line may fill. Widths are estimated from the
/// system UI sans; viewers without it fall back to wider faces, so text is
/// fitted with headroom instead of edge to edge.
const FIT_MARGIN: f32 = 0.94;

/// Adaptive ink for the transparent art: GitHub's own text colours, switched
/// by the viewer's colour scheme.
const ADAPTIVE_INK: &str =
    "<style>.mi{fill:#1F2328}@media (prefers-color-scheme:dark){.mi{fill:#E6EDF3}}</style>";

/// Largest size ≤ `size` (down to `min_ratio` of it, never under 16px unless
/// `size` already is) at which every line fits `budget`; lines that still
/// overflow at the floor are cropped by the caller.
fn shrink_to_fit(lines: &[&str], budget: f32, size: f32, min_ratio: f32, metric: Metric) -> f32 {
    let widest = lines
        .iter()
        .map(|l| line_advance(l, size, metric))
        .fold(0.0_f32, f32::max);
    if widest <= budget || widest <= 0.0 {
        return size;
    }
    let floor = (size * min_ratio).ceil().max(16.0_f32.min(size));
    (size * budget / widest).floor().max(floor)
}

/// How one text role is painted.
struct Ink {
    /// `fill="…"` or `class="mi"` attribute text.
    paint: String,
    family: &'static str,
    anchor: &'static str,
    x: f32,
}

pub fn render(spec: &MarkSpec) -> String {
    let art = Art::parse(spec.art.as_deref().unwrap_or("waving"));
    let height = spec.height.unwrap_or(220).clamp(40, 900);
    let width = spec.width.unwrap_or(880).clamp(200, 1600);
    let (wf, hf) = (width as f32, height as f32);
    let anim = normalize_animation(spec.animation.as_deref());

    let text = cap_text(spec.text.as_deref().unwrap_or(""), MAX_TEXT_CHARS);
    let desc = cap_text(spec.desc.as_deref().unwrap_or(""), MAX_DESC_CHARS);
    let seed = format!("{art}-{text}");
    let palette = resolve_palette(spec.color.as_deref(), spec.theme.as_deref(), &seed);
    let st: Stage = stage(art, width, height, &palette, background_moves(anim));

    let left = st.terminal || normalize_layout(spec.hero.layout.as_deref()) == "left";
    let family = if st.terminal {
        FONT_MONO
    } else {
        content_family(spec.font.as_deref())
    };
    let paint = match (&st.ink, art) {
        (Some(ink), _) => format!("fill=\"{ink}\""),
        // Transparent: a named theme or colour is honoured; otherwise the ink
        // follows the viewer's light/dark scheme.
        (None, _) if spec.theme.is_some() => format!("fill=\"{}\"", palette.ink),
        (None, _) if spec.color.is_some() => format!("fill=\"{}\"", palette.accents[0]),
        (None, _) => "class=\"mi\"".into(),
    };
    let ink = Ink {
        paint,
        family,
        anchor: if left { "start" } else { "middle" },
        x: if left { st.x0 } else { (st.x0 + st.x1) / 2.0 },
    };

    // Type scale: the title follows the canvas height, capped by its width;
    // the description is a fixed ratio of the title.
    let has_desc = !desc.is_empty();
    let title_size = (hf * if has_desc { 0.235 } else { 0.28 })
        .min(wf * 0.062)
        .clamp(14.0, 140.0);
    // Monospace runs wide and reads loud: the terminal sets it smaller.
    let (title_size, desc_ratio) = if st.terminal {
        ((title_size * 0.74).round(), 0.44)
    } else {
        (title_size.round(), 0.36)
    };
    let desc_size = (title_size * desc_ratio).clamp(11.0, 34.0).round();

    let budget = (st.x1 - st.x0) * FIT_MARGIN;
    let prompt = if st.terminal { "$ " } else { "" };
    let raw_lines: Vec<&str> = text
        .split('\n')
        .filter(|l| !l.is_empty())
        .take(MAX_LINES)
        .collect();
    let title_metric = Metric::Bold;
    let measured: Vec<String> = raw_lines.iter().map(|l| format!("{prompt}{l}")).collect();
    let measured: Vec<&str> = measured.iter().map(String::as_str).collect();
    let title_size = shrink_to_fit(&measured, budget, title_size, 0.45, title_metric);
    let prompt_px = line_advance(prompt, title_size, title_metric);
    let lines: Vec<String> = raw_lines
        .iter()
        .map(|l| fit_line(l, budget - prompt_px, title_size, title_metric))
        .collect();
    let desc_size = shrink_to_fit(&[desc.as_str()], budget, desc_size, 0.8, Metric::Display);
    let desc_line = fit_line(&desc, budget, desc_size, Metric::Display);

    // Vertical rhythm: centre the whole block (title lines + description) on
    // the stage's text centre.
    let line_h = title_size * 1.14;
    let n = lines.len().max(1) as f32;
    let span = (n - 1.0) * line_h;
    let to_desc = if lines.is_empty() {
        0.0
    } else {
        title_size * 0.56 + desc_size * 0.95
    };
    let block = span + if has_desc { to_desc } else { 0.0 };
    let first_y = st.cy - block / 2.0;

    let tracking = if st.terminal {
        "0"
    } else if title_size >= 40.0 {
        "-0.025em"
    } else {
        "-0.015em"
    };
    let weight = if st.terminal { 600 } else { 700 };
    let typing = anim == "type";
    let mut title_nodes = String::new();
    for (i, line) in lines.iter().enumerate() {
        let y = first_y + i as f32 * line_h;
        let (open, children) = if typing {
            (String::new(), String::new())
        } else {
            (
                text_open_attrs(anim, i, width, height),
                text_children(anim, i, width, height),
            )
        };
        let prompt_span = if st.terminal {
            format!("<tspan fill=\"{}\">{prompt}</tspan>", palette.accents[0])
        } else {
            String::new()
        };
        title_nodes.push_str(&format!(
            "<text x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\" dominant-baseline=\"central\" \
             font-family=\"{family}\" font-size=\"{title_size}\" font-weight=\"{weight}\" \
             letter-spacing=\"{tracking}\" {paint}{open}>{prompt_span}{content}{children}</text>",
            x = ink.x,
            anchor = ink.anchor,
            family = ink.family,
            paint = ink.paint,
            content = esc(line),
        ));
    }

    let title_px = if st.terminal {
        // Monospace advances are uniform: ~0.6em in every common face.
        lines
            .iter()
            .map(|l| (l.chars().count() + prompt.chars().count()) as f32 * title_size * 0.62)
            .fold(0.0, f32::max)
    } else {
        measured_width(&lines, prompt, title_size, title_metric)
    };
    let reveal = if typing && !lines.is_empty() {
        Some(type_reveal(&ink, title_px, first_y, span, title_size, wf))
    } else {
        None
    };
    let reveal_end = reveal.as_ref().map_or(0.0, |r| r.1);
    if let Some((defs, _)) = &reveal {
        title_nodes = format!("{defs}<g mask=\"url(#mt)\">{title_nodes}</g>");
    }

    let cursor = if st.terminal && lines.len() == 1 {
        let cx = st.x0 + (lines[0].chars().count() + 2) as f32 * title_size * 0.6 + 2.0;
        let blink = if background_moves(anim) {
            format!(
                "<animate attributeName=\"opacity\" values=\"1;0\" dur=\"1.1s\" begin=\"{reveal_end:.2}s\" \
                 repeatCount=\"indefinite\" calcMode=\"discrete\"/>"
            )
        } else {
            String::new()
        };
        format!(
            "<rect x=\"{cx:.1}\" y=\"{y:.1}\" width=\"{cw:.1}\" height=\"{ch:.1}\" rx=\"1\" fill=\"{a}\" \
             fill-opacity=\"0.85\"{hidden}>{blink}</rect>",
            hidden = if typing { " opacity=\"0\"" } else { "" },
            y = first_y - title_size * 0.42,
            cw = (title_size * 0.5).round(),
            ch = (title_size * 0.84).round(),
            a = palette.accents[0],
        )
    } else {
        String::new()
    };

    let desc_node = if has_desc {
        let y = first_y + span + to_desc;
        let (open, children) = if typing {
            (
                " opacity=\"0\"".to_string(),
                format!(
                    "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.8s\" \
                     begin=\"{reveal_end:.2}s\" fill=\"freeze\"/>"
                ),
            )
        } else {
            (
                text_open_attrs(anim, lines.len(), width, height),
                text_children(anim, lines.len(), width, height),
            )
        };
        let muted = if palette.light { 0.7 } else { 0.72 };
        format!(
            "<text x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\" dominant-baseline=\"central\" \
             font-family=\"{family}\" font-size=\"{desc_size}\" font-weight=\"400\" {paint} \
             fill-opacity=\"{muted}\"{open}>{content}{children}</text>",
            x = ink.x,
            anchor = ink.anchor,
            family = ink.family,
            paint = ink.paint,
            content = esc(&desc_line),
        )
    } else {
        String::new()
    };

    let style = if st.ink.is_none() { ADAPTIVE_INK } else { "" };
    let body = format!(
        "{style}<defs>{defs}</defs>{back}{title_nodes}{cursor}{desc_node}{credit}",
        defs = st.defs,
        back = st.back,
        credit = credit_mark(width, height, spec.credit),
    );
    svg_doc(width, height, &body)
}

fn measured_width(lines: &[String], prompt: &str, size: f32, metric: Metric) -> f32 {
    lines
        .iter()
        .map(|l| line_advance(&format!("{prompt}{l}"), size, metric))
        .fold(0.0, f32::max)
}

/// The `type` motion: a feathered mask sweeps across the title once.
/// Returns the mask defs and when the sweep ends (seconds); from then on the
/// mask opens to the whole canvas, so a width estimate never crops a glyph.
fn type_reveal(
    ink: &Ink,
    width: f32,
    first_y: f32,
    span: f32,
    size: f32,
    canvas: f32,
) -> (String, f32) {
    let pad = size * 0.6;
    let x = match ink.anchor {
        "start" => ink.x - pad,
        _ => ink.x - width / 2.0 - pad,
    };
    let full = width + pad * 2.0;
    let y = first_y - size;
    let h = span + size * 2.0;
    let dur = (width / (size * 9.0)).clamp(0.7, 2.2);
    let begin = 0.2;
    let defs = format!(
        "<defs><linearGradient id=\"mtg\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
         <stop offset=\"0.85\" stop-color=\"#fff\"/><stop offset=\"1\" stop-color=\"#fff\" stop-opacity=\"0\"/>\
         </linearGradient><mask id=\"mt\" maskUnits=\"userSpaceOnUse\">\
         <rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"0\" height=\"{h:.1}\" fill=\"url(#mtg)\">\
         <animate attributeName=\"width\" from=\"0\" to=\"{full:.1}\" dur=\"{dur:.2}s\" begin=\"{begin}s\" \
         fill=\"freeze\" calcMode=\"spline\" keyTimes=\"0;1\" keySplines=\"0.4 0 0.6 1\"/></rect>\
         <rect width=\"{canvas}\" height=\"{ch:.1}\" fill=\"#fff\" opacity=\"0\">\
         <set attributeName=\"opacity\" to=\"1\" begin=\"{end:.2}s\"/></rect></mask></defs>",
        end = begin + dur,
        ch = first_y + span + size * 2.0,
    );
    (defs, begin + dur)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hero(q: &[(&str, &str)]) -> String {
        let mut spec = MarkSpec {
            text: Some("Hello".into()),
            animation: Some("none".into()),
            ..Default::default()
        };
        for (k, v) in q {
            match *k {
                "type" => spec.art = Some(v.to_string()),
                "theme" => spec.theme = Some(v.to_string()),
                "color" => spec.color = Some(v.to_string()),
                "animation" => spec.animation = Some(v.to_string()),
                "layout" => spec.hero.layout = Some(v.to_string()),
                "desc" => spec.desc = Some(v.to_string()),
                "text" => spec.text = Some(v.to_string()),
                _ => {}
            }
        }
        render(&spec)
    }

    #[test]
    fn transparent_ink_follows_the_viewer_scheme() {
        let svg = hero(&[("type", "transparent")]);
        assert!(svg.contains("prefers-color-scheme:dark"));
        assert!(svg.contains("class=\"mi\""));
        assert!(!hero(&[("type", "transparent"), ("theme", "light")]).contains("class=\"mi\""));
    }

    #[test]
    fn terminal_sets_a_left_aligned_mono_prompt() {
        let svg = hero(&[("type", "terminal")]);
        assert!(svg.contains("text-anchor=\"start\""));
        assert!(svg.contains(FONT_MONO));
        assert!(svg.contains(">$ </tspan>"));
    }

    #[test]
    fn left_layout_aligns_to_the_column() {
        assert!(hero(&[("layout", "left")]).contains("text-anchor=\"start\""));
        assert!(hero(&[]).contains("text-anchor=\"middle\""));
    }

    #[test]
    fn type_reveals_the_title_as_one_run() {
        let svg = hero(&[("animation", "type"), ("text", "readme-mark")]);
        assert!(svg.contains("mask=\"url(#mt)\""));
        // One text node: glyph spacing stays the font's own.
        assert_eq!(svg.matches(">readme-mark<").count(), 1);
    }

    #[test]
    fn long_titles_shrink_before_they_crop() {
        let long = "A long project title that still fits: shrunk";
        let svg = hero(&[("text", long)]);
        assert!(svg.contains(long), "shrunk, not cropped");
        let huge = "x".repeat(400);
        assert!(hero(&[("text", &huge)]).contains('…'));
    }
}
