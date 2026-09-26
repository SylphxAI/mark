//! Banner art: one composition per curated [`Art`], one exhaustive dispatch.
//!
//! Each art paints a rounded card (except `transparent`) and tells the caller
//! where text belongs and which ink reads on it. Light comes from radial
//! gradients, never blur filters, so a banner stays cheap to paint and crisp
//! at any size. Motion is SMIL and only ever a slow drift of the light.

use crate::capabilities::mark::domain::art::Art;
use crate::capabilities::mark::domain::color::{ink_over, mix, Palette};
use crate::capabilities::mark::domain::motion::drift;

pub(crate) mod capsule;

/// The painted background plus the text box it leaves.
pub(crate) struct Stage {
    /// Markup for `<defs>`.
    pub defs: String,
    /// Background layers, painted before text.
    pub back: String,
    /// Left and right edge of the text column.
    pub x0: f32,
    pub x1: f32,
    /// Vertical centre of the text block.
    pub cy: f32,
    /// Text ink (`#hex`), or `None` for the transparent art's adaptive ink.
    pub ink: Option<String>,
    /// Terminal art sets text in the monospace stack, left aligned.
    pub terminal: bool,
}

/// Corner radius for a card of height `h`.
pub(crate) fn card_radius(h: f32) -> f32 {
    (h * 0.07).clamp(6.0, 16.0).round()
}

struct Ctx<'a> {
    w: f32,
    h: f32,
    p: &'a Palette,
    moves: bool,
}

impl Ctx<'_> {
    /// A soft pool of light: an ellipse filled with a radial falloff.
    fn glow(&self, id: &str, color: &str, opacity: f32, c: (f32, f32), r: (f32, f32)) -> Glow {
        Glow {
            def: format!(
                "<radialGradient id=\"{id}\"><stop offset=\"0\" stop-color=\"{color}\" stop-opacity=\"{o0:.2}\"/>\
                 <stop offset=\"0.25\" stop-color=\"{color}\" stop-opacity=\"{o1:.2}\"/>\
                 <stop offset=\"0.5\" stop-color=\"{color}\" stop-opacity=\"{o2:.2}\"/>\
                 <stop offset=\"0.75\" stop-color=\"{color}\" stop-opacity=\"{o3:.2}\"/>\
                 <stop offset=\"1\" stop-color=\"{color}\" stop-opacity=\"0\"/></radialGradient>",
                o0 = opacity,
                o1 = opacity * 0.78,
                o2 = opacity * 0.42,
                o3 = opacity * 0.13,
            ),
            shape: format!(
                "<ellipse cx=\"{:.0}\" cy=\"{:.0}\" rx=\"{:.0}\" ry=\"{:.0}\" fill=\"url(#{id})\"/>",
                c.0, c.1, r.0, r.1
            ),
        }
    }

    /// Wrap a layer in a slow drift when the background moves.
    fn drifting(&self, layer: &str, path: &[(f32, f32)], dur: f32) -> String {
        if self.moves {
            format!("<g>{layer}{}</g>", drift(path, dur))
        } else {
            layer.to_string()
        }
    }

    fn base(&self, fill: &str) -> String {
        format!(
            "<rect width=\"{:.0}\" height=\"{:.0}\" fill=\"{fill}\"/>",
            self.w, self.h
        )
    }

    /// Glow strength scaled for light bases, where colour reads stronger.
    fn strength(&self, dark: f32) -> f32 {
        if self.p.light {
            dark * 0.62
        } else {
            dark
        }
    }
}

struct Glow {
    def: String,
    shape: String,
}

/// Paint the stage for one art type at `w × h`.
pub(crate) fn stage(art: Art, w: u32, h: u32, p: &Palette, moves: bool) -> Stage {
    let c = Ctx {
        w: w as f32,
        h: h as f32,
        p,
        moves,
    };
    let pad = (c.w * 0.06).clamp(20.0, 64.0);
    let rx = card_radius(c.h);
    let (mut defs, mut back) = match art {
        Art::Waving => waving(&c),
        Art::Aurora => aurora(&c),
        Art::Mesh => mesh(&c),
        Art::Spotlight => spotlight(&c),
        Art::Grid => grid(&c),
        Art::Minimal => minimal(&c),
        Art::Terminal => terminal(&c),
        Art::Transparent => (String::new(), String::new()),
    };
    let mut stage = Stage {
        defs: String::new(),
        back: String::new(),
        x0: pad,
        x1: c.w - pad,
        cy: c.h * 0.5,
        ink: Some(p.ink.clone()),
        terminal: false,
    };
    match art {
        Art::Transparent => {
            stage.ink = None;
            return stage;
        }
        Art::Waving => stage.cy = c.h * 0.42,
        Art::Mesh => stage.ink = Some(ink_over(&mesh_field(p))),
        Art::Terminal => {
            let (m, bar) = terminal_frame(&c);
            stage.x0 = m + (c.w * 0.035).clamp(16.0, 32.0);
            stage.x1 = c.w - m - 16.0;
            stage.cy = (m + bar + c.h - m) / 2.0;
            stage.terminal = true;
        }
        Art::Aurora | Art::Spotlight | Art::Grid | Art::Minimal => {}
    }
    // Clip to the card, then a hairline edge so the card reads as an object
    // on a README background of the same tone.
    let edge = if p.light { 0.10 } else { 0.09 };
    defs.push_str(&format!(
        "<clipPath id=\"mc\"><rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\"/></clipPath>"
    ));
    back = format!(
        "<g clip-path=\"url(#mc)\">{back}</g>\
         <rect x=\"0.5\" y=\"0.5\" width=\"{bw}\" height=\"{bh}\" rx=\"{rx2}\" fill=\"none\" \
         stroke=\"{ink}\" stroke-opacity=\"{edge}\"/>",
        bw = c.w - 1.0,
        bh = c.h - 1.0,
        rx2 = (rx - 0.5).max(0.0),
        ink = p.ink,
    );
    stage.defs = defs;
    stage.back = back;
    stage
}

/// Layered waves along the bottom edge, each drifting at its own pace.
fn waving(c: &Ctx) -> (String, String) {
    let [a1, a2, a3] = &c.p.accents;
    let top = c.glow(
        "mg0",
        a1,
        c.strength(0.22),
        (c.w * 0.2, 0.0),
        (c.w * 0.55, c.h * 0.9),
    );
    let mut defs = top.def;
    defs.push_str(&format!(
        "<linearGradient id=\"mw\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
         <stop offset=\"0\" stop-color=\"{a1}\"/><stop offset=\"0.5\" stop-color=\"{a2}\"/>\
         <stop offset=\"1\" stop-color=\"{a3}\"/></linearGradient>"
    ));
    let mut back = c.base(&c.p.bg);
    back.push_str(&top.shape);
    // (baseline, amplitude, periods across the width, opacity, seconds, direction)
    let layers = [
        (0.70, 0.075, 1.0, 0.22, 26.0, 1.0),
        (0.77, 0.06, 2.0, 0.42, 20.0, -1.0),
        (0.85, 0.05, 1.0, c.strength(0.95).max(0.8), 16.0, 1.0),
    ];
    for (base, amp, periods, opacity, dur, dir) in layers {
        let period = c.w / periods;
        let path = wave_path(c.w + period, c.h, c.h * base, c.h * amp, period);
        let (from, to) = if dir > 0.0 {
            (0.0, -period)
        } else {
            (-period, 0.0)
        };
        let motion = if c.moves {
            format!(
                "<animateTransform attributeName=\"transform\" type=\"translate\" from=\"{from:.0} 0\" \
                 to=\"{to:.0} 0\" dur=\"{dur}s\" repeatCount=\"indefinite\"/>"
            )
        } else {
            String::new()
        };
        back.push_str(&format!(
            "<path d=\"{path}\" fill=\"url(#mw)\" fill-opacity=\"{opacity:.2}\" \
             transform=\"translate({from:.0} 0)\">{motion}</path>"
        ));
    }
    (defs, back)
}

/// A smooth periodic wave from x = 0 to `span`, closed along the bottom.
fn wave_path(span: f32, h: f32, y: f32, amp: f32, period: f32) -> String {
    let half = period / 2.0;
    let mut d = format!(
        "M0 {y:.1} Q{:.1} {:.1} {:.1} {y:.1}",
        half / 2.0,
        y - amp * 2.0,
        half
    );
    let mut x = half;
    while x < span {
        x += half;
        d.push_str(&format!(" T{x:.1} {y:.1}"));
    }
    d.push_str(&format!(" L{x:.1} {h:.0} L0 {h:.0}Z"));
    d
}

/// Three pools of coloured light drifting over the base.
fn aurora(c: &Ctx) -> (String, String) {
    let [a1, a2, a3] = &c.p.accents;
    let (w, h) = (c.w, c.h);
    let g1 = c.glow(
        "mg1",
        a1,
        c.strength(0.62),
        (w * 0.2, h * 0.08),
        (w * 0.42, h * 1.0),
    );
    let g2 = c.glow(
        "mg2",
        a2,
        c.strength(0.5),
        (w * 0.8, h * 0.18),
        (w * 0.4, h * 0.95),
    );
    let g3 = c.glow(
        "mg3",
        a3,
        c.strength(0.38),
        (w * 0.52, h * 1.08),
        (w * 0.5, h * 0.7),
    );
    let defs = format!("{}{}{}", g1.def, g2.def, g3.def);
    let back = format!(
        "{}{}{}{}",
        c.base(&c.p.bg),
        c.drifting(
            &g1.shape,
            &[(0.0, 0.0), (w * 0.07, h * 0.1), (w * -0.03, h * 0.05)],
            22.0
        ),
        c.drifting(
            &g2.shape,
            &[(0.0, 0.0), (w * -0.08, h * 0.08), (w * 0.02, h * -0.06)],
            27.0
        ),
        c.drifting(&g3.shape, &[(0.0, 0.0), (w * 0.06, h * -0.12)], 19.0),
    );
    (defs, back)
}

/// The colour a mesh averages to (text ink is chosen against it).
fn mesh_field(p: &Palette) -> String {
    let [a1, a2, a3] = &p.accents;
    let accents = mix(&mix(a1, a2, 0.5), a3, 0.33);
    if p.light {
        mix(&accents, "#FFFFFF", 0.62)
    } else {
        mix(&accents, &p.bg, 0.45)
    }
}

/// A full-bleed mesh gradient: four large pools of the accents.
fn mesh(c: &Ctx) -> (String, String) {
    let tone = |a: &str| {
        if c.p.light {
            mix(a, "#FFFFFF", 0.5)
        } else {
            a.to_string()
        }
    };
    let [a1, a2, a3] = &c.p.accents;
    let (a1, a2, a3) = (tone(a1), tone(a2), tone(a3));
    let a4 = mix(&a2, &a3, 0.5);
    let base = if c.p.light {
        mix(&a1, "#FFFFFF", 0.55)
    } else {
        mix(&a1, &c.p.bg, 0.6)
    };
    let (w, h) = (c.w, c.h);
    let o = if c.p.light { 0.85 } else { 0.8 };
    let g1 = c.glow("mg1", &a1, o, (w * 0.12, h * 0.2), (w * 0.45, h * 1.3));
    let g2 = c.glow("mg2", &a2, o, (w * 0.85, h * 0.05), (w * 0.45, h * 1.2));
    let g3 = c.glow(
        "mg3",
        &a3,
        o * 0.85,
        (w * 0.35, h * 1.05),
        (w * 0.4, h * 0.9),
    );
    let g4 = c.glow(
        "mg4",
        &a4,
        o * 0.8,
        (w * 0.95, h * 0.95),
        (w * 0.35, h * 0.9),
    );
    let defs = format!("{}{}{}{}", g1.def, g2.def, g3.def, g4.def);
    let back = format!(
        "{}{}{}{}{}",
        c.base(&base),
        c.drifting(&g1.shape, &[(0.0, 0.0), (w * 0.1, h * 0.12)], 24.0),
        c.drifting(&g2.shape, &[(0.0, 0.0), (w * -0.12, h * 0.1)], 28.0),
        c.drifting(&g3.shape, &[(0.0, 0.0), (w * 0.12, h * -0.1)], 21.0),
        c.drifting(&g4.shape, &[(0.0, 0.0), (w * -0.1, h * -0.12)], 25.0),
    );
    (defs, back)
}

/// One beam of light falling from the top edge, with a lit hairline.
fn spotlight(c: &Ctx) -> (String, String) {
    let [a1, a2, _] = &c.p.accents;
    let (w, h) = (c.w, c.h);
    let core = mix(a1, "#FFFFFF", 0.45);
    let wide = c.glow(
        "mg1",
        a1,
        c.strength(0.55),
        (w * 0.5, 0.0),
        (w * 0.36, h * 1.15),
    );
    let side = c.glow(
        "mg2",
        a2,
        c.strength(0.22),
        (w * 0.5, h * 0.1),
        (w * 0.62, h * 0.8),
    );
    let beam = c.glow(
        "mg3",
        &core,
        c.strength(0.32),
        (w * 0.5, 0.0),
        (w * 0.12, h * 0.85),
    );
    let mut defs = format!("{}{}{}", wide.def, side.def, beam.def);
    defs.push_str(&format!(
        "<linearGradient id=\"ml\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
         <stop offset=\"0\" stop-color=\"{core}\" stop-opacity=\"0\"/>\
         <stop offset=\"0.5\" stop-color=\"{core}\"/>\
         <stop offset=\"1\" stop-color=\"{core}\" stop-opacity=\"0\"/></linearGradient>"
    ));
    let light = format!("{}{}", wide.shape, beam.shape);
    let light = if c.moves {
        format!(
            "<g>{light}<animate attributeName=\"opacity\" values=\"1;0.8;1\" dur=\"9s\" \
             repeatCount=\"indefinite\" calcMode=\"spline\" keyTimes=\"0;0.5;1\" \
             keySplines=\"0.45 0 0.55 1;0.45 0 0.55 1\"/></g>"
        )
    } else {
        light
    };
    let back = format!(
        "{}{}{}<rect x=\"{:.0}\" y=\"0\" width=\"{:.0}\" height=\"1.5\" fill=\"url(#ml)\"/>",
        c.base(&c.p.bg),
        side.shape,
        c.drifting(
            &light,
            &[(0.0, 0.0), (w * 0.03, 0.0), (w * -0.03, 0.0)],
            16.0
        ),
        w * 0.2,
        w * 0.6,
    );
    (defs, back)
}

/// A fine grid, fading out from a soft glow at the centre.
fn grid(c: &Ctx) -> (String, String) {
    let [a1, a2, _] = &c.p.accents;
    let (w, h) = (c.w, c.h);
    let cell = (h / 7.0).clamp(20.0, 48.0).round();
    let g1 = c.glow(
        "mg1",
        a1,
        c.strength(0.42),
        (w * 0.42, h * 0.5),
        (w * 0.34, h * 0.9),
    );
    let g2 = c.glow(
        "mg2",
        a2,
        c.strength(0.3),
        (w * 0.62, h * 0.55),
        (w * 0.3, h * 0.8),
    );
    let line = if c.p.light { 0.11 } else { 0.12 };
    let defs = format!(
        "{}{}<pattern id=\"mp\" width=\"{cell}\" height=\"{cell}\" patternUnits=\"userSpaceOnUse\" \
         x=\"{ox:.0}\" y=\"{oy:.0}\"><path d=\"M{cell} 0H0V{cell}\" fill=\"none\" stroke=\"{ink}\" \
         stroke-opacity=\"{line}\"/></pattern>\
         <radialGradient id=\"mf\" cx=\"0.5\" cy=\"0.5\" r=\"0.5\"><stop offset=\"0\" stop-color=\"#fff\"/>\
         <stop offset=\"0.6\" stop-color=\"#fff\" stop-opacity=\"0.7\"/>\
         <stop offset=\"1\" stop-color=\"#fff\" stop-opacity=\"0\"/></radialGradient>\
         <mask id=\"mm\"><rect width=\"{w}\" height=\"{h}\" fill=\"url(#mf)\"/></mask>",
        g1.def,
        g2.def,
        ox = (w / 2.0) % cell,
        oy = (h / 2.0) % cell,
        ink = c.p.ink,
    );
    let back = format!(
        "{}{}{}<rect width=\"{w}\" height=\"{h}\" fill=\"url(#mp)\" mask=\"url(#mm)\"/>",
        c.base(&c.p.bg),
        c.drifting(
            &g1.shape,
            &[(0.0, 0.0), (w * 0.08, h * -0.06), (w * -0.04, h * 0.06)],
            20.0
        ),
        c.drifting(&g2.shape, &[(0.0, 0.0), (w * -0.08, h * 0.05)], 24.0),
    );
    (defs, back)
}

/// A flat card with one accent hairline along the top.
fn minimal(c: &Ctx) -> (String, String) {
    let [a1, a2, a3] = &c.p.accents;
    let defs = format!(
        "<linearGradient id=\"ml\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
         <stop offset=\"0\" stop-color=\"{a1}\"/><stop offset=\"0.5\" stop-color=\"{a2}\"/>\
         <stop offset=\"1\" stop-color=\"{a3}\"/></linearGradient>"
    );
    let bar = (c.h * 0.014).clamp(2.0, 4.0);
    let back = format!(
        "{}<rect width=\"{:.0}\" height=\"{bar:.1}\" fill=\"url(#ml)\"/>",
        c.base(&c.p.bg),
        c.w
    );
    (defs, back)
}

/// Window margin and title-bar height of the terminal art.
fn terminal_frame(c: &Ctx) -> (f32, f32) {
    let m = (c.h * 0.09).clamp(10.0, 28.0).round();
    let bar = (c.h * 0.14).clamp(20.0, 40.0).round();
    (m, bar)
}

/// A terminal window floating over a faint glow.
fn terminal(c: &Ctx) -> (String, String) {
    let [a1, a2, _] = &c.p.accents;
    let (w, h) = (c.w, c.h);
    let (m, bar) = terminal_frame(c);
    let g1 = c.glow(
        "mg1",
        a1,
        c.strength(0.45),
        (w * 0.85, h * 1.0),
        (w * 0.4, h * 0.9),
    );
    let g2 = c.glow(
        "mg2",
        a2,
        c.strength(0.3),
        (w * 0.1, 0.0),
        (w * 0.35, h * 0.8),
    );
    let defs = format!("{}{}", g1.def, g2.def);
    let win_w = w - 2.0 * m;
    let win_h = h - 2.0 * m;
    let r = (bar * 0.16).clamp(3.5, 6.0);
    let dots: String = ["#FF5F57", "#FEBC2E", "#28C840"]
        .iter()
        .enumerate()
        .map(|(i, col)| {
            format!(
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\" fill=\"{col}\"/>",
                m + bar * 0.55 + i as f32 * r * 3.3,
                m + bar / 2.0
            )
        })
        .collect();
    let surface = if c.p.light {
        "#FFFFFF".to_string()
    } else {
        c.p.surface.clone()
    };
    let back = format!(
        "{}{}{}\
         <rect x=\"{m}\" y=\"{m}\" width=\"{win_w}\" height=\"{win_h}\" rx=\"10\" fill=\"{surface}\" \
         fill-opacity=\"0.94\" stroke=\"{ink}\" stroke-opacity=\"0.12\"/>\
         <path d=\"M{m} {line}H{end}\" stroke=\"{ink}\" stroke-opacity=\"0.08\"/>{dots}",
        c.base(&c.p.bg),
        c.drifting(&g1.shape, &[(0.0, 0.0), (w * -0.06, h * -0.08)], 22.0),
        c.drifting(&g2.shape, &[(0.0, 0.0), (w * 0.06, h * 0.06)], 26.0),
        line = m + bar,
        end = m + win_w,
        ink = c.p.ink,
    );
    (defs, back)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::mark::domain::color::{contrast_ratio, resolve_palette};

    #[test]
    fn still_stages_carry_no_smil() {
        let p = resolve_palette(None, None, "x");
        for art in Art::ALL {
            let s = stage(art, 880, 220, &p, false);
            assert!(!s.back.contains("<animate"), "{art} moved while still");
        }
    }

    #[test]
    fn moving_stages_drift_their_light() {
        let p = resolve_palette(None, None, "x");
        for art in [
            Art::Waving,
            Art::Aurora,
            Art::Mesh,
            Art::Spotlight,
            Art::Grid,
            Art::Terminal,
        ] {
            let s = stage(art, 880, 220, &p, true);
            assert!(s.back.contains("repeatCount=\"indefinite\""), "{art}");
        }
    }

    #[test]
    fn mesh_ink_reads_on_every_theme() {
        for id in crate::capabilities::mark::domain::theme::list_names() {
            let p = resolve_palette(None, Some(id), "x");
            let s = stage(Art::Mesh, 880, 220, &p, false);
            let ink = s.ink.expect("mesh ink");
            let r = contrast_ratio(&ink, &mesh_field(&p));
            assert!(r >= 3.0, "{id}: mesh ink {r:.2}:1");
        }
    }

    #[test]
    fn wave_path_spans_one_extra_period() {
        let d = wave_path(1320.0, 220.0, 150.0, 10.0, 440.0);
        assert!(d.starts_with("M0 150.0 Q110.0 130.0 220.0 150.0"));
        assert!(d.contains("T1320.0 150.0"));
        assert!(d.ends_with("L0 220Z"));
    }
}
