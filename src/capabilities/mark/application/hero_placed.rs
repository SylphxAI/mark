//! Placed hero: the hero with dialect overrides (capsule-render).
//!
//! The native hero derives typography from its layout family; a dialect URL
//! instead names sizes, colors, and per-line positions, and paints a flat
//! silhouette. The overrides are built only by the capsule dialect module, so
//! the native grammar cannot reach this path. Pure and deterministic.

use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::shapes::capsule::{capsule_paint, silhouette};
use crate::capabilities::mark::domain::svg::{credit_mark, esc, svg_doc};
use crate::capabilities::mark::domain::{
    cap_text, HeroOverrides, MarkSpec, PlacedText, MAX_DESC_CHARS, MAX_LINES, MAX_TEXT_CHARS,
};

pub(crate) fn render_placed(spec: &MarkSpec, ov: &HeroOverrides) -> String {
    let width = spec.width.unwrap_or(854).clamp(200, 1600);
    let height = spec.height.unwrap_or(120).clamp(20, 900);
    let text = cap_text(spec.text.as_deref().unwrap_or(""), MAX_TEXT_CHARS);
    let desc = cap_text(spec.desc.as_deref().unwrap_or(""), MAX_DESC_CHARS);
    let (defs, fill) = capsule_paint(&ov.paint);

    let canvas = Canvas { width, height, ov };
    let title_lines = lines(&text);
    let stroke = ov
        .stroke
        .as_ref()
        .map(|(c, w)| format!(" stroke=\"{c}\" stroke-width=\"{w}\" paint-order=\"stroke fill\""))
        .unwrap_or_default();
    let mut type_nodes = canvas.block(&title_lines, &ov.title, &stroke);
    type_nodes.push_str(&canvas.block(&lines(&desc), &ov.desc, ""));
    if ov.rotate != 0.0 && ov.rotate.is_finite() {
        type_nodes = format!(
            "<g transform=\"rotate({} {} {})\">{type_nodes}</g>",
            ov.rotate,
            width as f32 / 2.0,
            height as f32 / 2.0
        );
    }

    let body = format!(
        "<defs>{defs}</defs>{}{}{type_nodes}{}",
        silhouette(ov.silhouette, width, height, &fill, ov.flip, ov.mirror),
        canvas.text_plate(title_lines.first().map(String::as_str)),
        credit_mark(width, height, spec.credit),
    );
    svg_doc(width, height, &body)
}

fn lines(s: &str) -> Vec<String> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .take(MAX_LINES)
        .map(str::to_string)
        .collect()
}

struct Canvas<'a> {
    width: u32,
    height: u32,
    ov: &'a HeroOverrides,
}

impl Canvas<'_> {
    fn x(&self, pct: f32) -> f32 {
        self.width as f32 * pct / 100.0
    }

    fn y(&self, pct: f32) -> f32 {
        self.height as f32 * pct / 100.0
    }

    /// One `<text>` per role; each line is a `<tspan>` at its own position,
    /// so the role animates as one block.
    fn block(&self, lines: &[String], role: &PlacedText, extra: &str) -> String {
        if lines.is_empty() {
            return String::new();
        }
        let first_x = role.x.first().copied().unwrap_or(50.0);
        let mut y = self.y(role.y.first().copied().unwrap_or(50.0));
        let mut spans = String::new();
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                y = match role.y.get(i) {
                    Some(pct) => self.y(*pct),
                    None => y + role.step_em * role.size as f32,
                };
            }
            let x = self.x(role.x.get(i).copied().unwrap_or(first_x));
            spans.push_str(&format!("<tspan x=\"{x}\" y=\"{y}\">{}</tspan>", esc(line)));
        }
        let motion = self.ov.motion;
        format!(
            "<text text-anchor=\"middle\" dominant-baseline=\"middle\" font-family=\"{family}\" \
             font-size=\"{size}\" font-weight=\"{weight}\" fill=\"{color}\"{extra}{open}>{spans}{children}</text>",
            family = self.ov.family,
            size = role.size,
            weight = role.weight,
            color = role.color,
            open = text_open_attrs(motion, 0, self.width, self.height),
            children = text_children(motion, 0, self.width, self.height),
        )
    }

    /// The rounded plate behind the first title line (capsule `textBg`),
    /// sized by capsule-render's estimate: half an em per character.
    fn text_plate(&self, first_line: Option<&str>) -> String {
        let (Some(color), Some(line)) = (self.ov.text_bg.as_deref(), first_line) else {
            return String::new();
        };
        let fs = self.ov.title.size as f32;
        let h = fs + 40.0;
        let w = line.chars().count() as f32 * fs * 0.5 + 40.0;
        let x = self.x(self.ov.title.x.first().copied().unwrap_or(50.0)) - w / 2.0;
        let y = self.y(self.ov.title.y.first().copied().unwrap_or(50.0)) - h / 2.0;
        format!("<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" rx=\"25\" ry=\"25\" fill=\"{color}\"/>")
    }
}
