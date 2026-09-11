//! Canvas art family — the core polished backgrounds.
//!
//! One function per art type; `super::shape_background` dispatches exhaustively.

use super::{blob, Blob};
use super::{field_stack, sheen, vignette};
use crate::capabilities::mark::domain::art::Art;
use crate::capabilities::mark::domain::color::{ink_canvas, FillPlan};
pub(super) fn soft(art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let fill = plan.fill.as_str();
    let g = gain;
    let rx = if art == Art::Soft {
        (h / 2).min(48)
    } else {
        (h / 4).min(28)
    };
    let border = format!(
                "<rect x=\"1\" y=\"1\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\"/>",
                w.saturating_sub(2),
                h.saturating_sub(2),
                rx.saturating_sub(1),
            );
    let pulse = if g > 0.01 {
        format!(
                    "<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"#ffffff\" fill-opacity=\"0.04\">\
                       <animate attributeName=\"fill-opacity\" values=\"0.02;0.07;0.02\" dur=\"4.5s\" repeatCount=\"indefinite\"/>\
                     </rect>"
                )
    } else {
        String::new()
    };
    let shine = format!("<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"url(#shine)\"/>");
    format!(
        "<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"{fill}\"/>\
                 {pulse}{shine}{border}{sheen}{vignette}",
        sheen = sheen(w, h, g, plan),
        vignette = vignette(w, h, plan),
    )
}
pub(super) fn aurora(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let accent = plan.accent.as_str();
    let accent2 = plan.accent2.as_str();
    let warm = plan.warm.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let y1 = hf * 0.55;
    let w1 = wf * 0.25;
    let y2 = hf * 0.25;
    let w2 = wf * 0.55;
    let y3 = hf * 0.85;
    let y4 = hf * 0.5;
    let y5 = hf * 0.7;
    let w3 = wf * 0.3;
    let y6 = hf * 0.95;
    let w4 = wf * 0.7;
    let y7 = hf * 0.55;
    let y8 = hf * 0.8;
    let y1b = hf * 0.5;
    let y2b = hf * 0.32;
    let y3b = hf * 0.78;
    let y4b = hf * 0.58;
    let wave = if g > 0.01 {
        format!(
                    "<animate attributeName=\"d\" dur=\"8s\" repeatCount=\"indefinite\"                        values=\"M0,{y1} C{w1},{y2} {w2},{y3} {w},{y4} L{w},{h} L0,{h} Z;M0,{y1b} C{w1},{y2b} {w2},{y3b} {w},{y4b} L{w},{h} L0,{h} Z;M0,{y1} C{w1},{y2} {w2},{y3} {w},{y4} L{w},{h} L0,{h} Z\"/>"
                )
    } else {
        String::new()
    };
    let wave2 = if g > 0.01 {
        format!(
                    "<animate attributeName=\"d\" dur=\"10.5s\" begin=\"0.4s\" repeatCount=\"indefinite\"                        values=\"M0,{y5} C{w3},{y6} {w4},{y7} {w},{y8} L{w},{h} L0,{h} Z;M0,{y5b} C{w3},{y6b} {w4},{y7b} {w},{y8b} L{w},{h} L0,{h} Z;M0,{y5} C{w3},{y6} {w4},{y7} {w},{y8} L{w},{h} L0,{h} Z\"/>",
                    y5b = hf * 0.62,
                    y6b = hf * 0.88,
                    y7b = hf * 0.48,
                    y8b = hf * 0.72,
                )
    } else {
        String::new()
    };
    format!(
                "{base}{b1}{b2}{b3}                 <path d=\"M0,{y1} C{w1},{y2} {w2},{y3} {w},{y4} L{w},{h} L0,{h} Z\" fill=\"{accent}\" fill-opacity=\"0.22\">{wave}</path>                 <path d=\"M0,{y5} C{w3},{y6} {w4},{y7} {w},{y8} L{w},{h} L0,{h} Z\" fill=\"{warm}\" fill-opacity=\"0.18\">{wave2}</path>                 <path d=\"M0,{y1} C{w1},{y2} {w2},{y3} {w},{y4} L{w},{h} L0,{h} Z\" fill=\"url(#mgWaveA)\" fill-opacity=\"0.28\"/>                 {sheen}{vig}",
                base = field_stack(w, h, plan),
                b1 = blob(
            Blob {
                center: (wf * 0.25, hf * 0.35),
                size: (wf * 0.3, hf * 0.5),
                color: accent,
                opacity: 0.22,
                drift: (wf * 0.05, -hf * 0.04),
                dur: 9.0,
                phase: 0.0,
            },
            g,
        ),
                b2 = blob(
            Blob {
                center: (wf * 0.7, hf * 0.45),
                size: (wf * 0.32, hf * 0.48),
                color: accent2,
                opacity: 0.24,
                drift: (-wf * 0.06, hf * 0.05),
                dur: 10.0,
                phase: 0.5,
            },
            g,
        ),
                b3 = blob(
            Blob {
                center: (wf * 0.5, hf * 0.2),
                size: (wf * 0.22, hf * 0.3),
                color: warm,
                opacity: 0.16,
                drift: (wf * 0.03, hf * 0.04),
                dur: 8.0,
                phase: 1.0,
            },
            g,
        ),
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
}
pub(super) fn mesh(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let accent = plan.accent.as_str();
    let accent2 = plan.accent2.as_str();
    let warm = plan.warm.as_str();
    let glow = plan.glow.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    format!(
        "{base}{a}{b}{c}{d}{sheen}{vig}",
        base = field_stack(w, h, plan),
        a = blob(
            Blob {
                center: (wf * 0.2, hf * 0.35),
                size: (wf * 0.36, hf * 0.55),
                color: accent,
                opacity: 0.3,
                drift: (wf * 0.07, hf * 0.05),
                dur: 8.5,
                phase: 0.0,
            },
            g,
        ),
        b = blob(
            Blob {
                center: (wf * 0.7, hf * 0.3),
                size: (wf * 0.38, hf * 0.52),
                color: accent2,
                opacity: 0.32,
                drift: (-wf * 0.06, hf * 0.06),
                dur: 9.5,
                phase: 0.4,
            },
            g,
        ),
        c = blob(
            Blob {
                center: (wf * 0.5, hf * 0.75),
                size: (wf * 0.34, hf * 0.42),
                color: warm,
                opacity: 0.24,
                drift: (wf * 0.04, -hf * 0.05),
                dur: 10.5,
                phase: 0.9,
            },
            g,
        ),
        d = blob(
            Blob {
                center: (wf * 0.85, hf * 0.65),
                size: (wf * 0.26, hf * 0.36),
                color: glow,
                opacity: 0.18,
                drift: (-wf * 0.04, -hf * 0.04),
                dur: 7.5,
                phase: 1.3,
            },
            g,
        ),
        sheen = sheen(w, h, g, plan),
        vig = vignette(w, h, plan),
    )
}
pub(super) fn glass(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let glow = plan.glow.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let panel = if g > 0.01 {
        format!(
                    "<g>\
                       <animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; 0 -3; 0 0; 0 2; 0 0\" dur=\"7s\" repeatCount=\"indefinite\"/>\
                       <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"22\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"url(#glassEdge)\" stroke-width=\"1.2\"/>\
                       <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"18\" fill=\"{glow}\" fill-opacity=\"0.1\">\
                         <animate attributeName=\"fill-opacity\" values=\"0.03;0.08;0.03\" dur=\"4s\" repeatCount=\"indefinite\"/>\
                       </rect>\
                     </g>",
                    x = wf * 0.06,
                    y = hf * 0.14,
                    rw = wf * 0.88,
                    rh = hf * 0.72,
                    x2 = wf * 0.1,
                    y2 = hf * 0.2,
                    rw2 = wf * 0.4,
                    rh2 = hf * 0.2,
                )
    } else {
        format!(
                    "<rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"22\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"url(#glassEdge)\" stroke-width=\"1.2\"/>\
                     <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"18\" fill=\"#ffffff\" fill-opacity=\"0.04\"/>",
                    x = wf * 0.06,
                    y = hf * 0.14,
                    rw = wf * 0.88,
                    rh = hf * 0.72,
                    x2 = wf * 0.1,
                    y2 = hf * 0.2,
                    rw2 = wf * 0.4,
                    rh2 = hf * 0.2,
                )
    };
    format!(
        "{base}{blob}{panel}{sheen}{vig}",
        base = field_stack(w, h, plan),
        blob = blob(
            Blob {
                center: (wf * 0.75, hf * 0.3),
                size: (wf * 0.25, hf * 0.45),
                color: glow,
                opacity: 0.14,
                drift: (-wf * 0.04, hf * 0.05),
                dur: 9.0,
                phase: 0.0,
            },
            g,
        ),
        sheen = sheen(w, h, g, plan),
        vig = vignette(w, h, plan),
    )
}
pub(super) fn horizon(art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    {
        let mid = if art == Art::Horizon { 0.58 } else { 0.52 };
        let sun = if g > 0.01 {
            format!(
                    "<ellipse cx=\"{sun}\" cy=\"{horizon}\" rx=\"{sr}\" ry=\"{sry}\" fill=\"#ffffff\" fill-opacity=\"0.22\" filter=\"url(#softGlow)\">\
                       <animate attributeName=\"ry\" values=\"{sry};{sry2};{sry}\" dur=\"6s\" repeatCount=\"indefinite\"/>\
                       <animate attributeName=\"fill-opacity\" values=\"0.18;0.32;0.18\" dur=\"6s\" repeatCount=\"indefinite\"/>\
                     </ellipse>",
                    sun = wf * 0.72,
                    horizon = hf * mid,
                    sr = hf * 0.22,
                    sry = hf * 0.12,
                    sry2 = hf * 0.16,
                )
        } else {
            format!(
                    "<ellipse cx=\"{sun}\" cy=\"{horizon}\" rx=\"{sr}\" ry=\"{sry}\" fill=\"#ffffff\" fill-opacity=\"0.22\" filter=\"url(#softGlow)\"/>",
                    sun = wf * 0.72,
                    horizon = hf * mid,
                    sr = hf * 0.22,
                    sry = hf * 0.12,
                )
        };
        let ground = if g > 0.01 {
            format!(
                    "<path fill=\"#000000\" fill-opacity=\"0.22\" d=\"M0,{hy} Q{w1},{hy2} {w2},{hy} T{w},{hy} L{w},{h} L0,{h} Z\">\
                       <animate attributeName=\"d\" dur=\"7s\" repeatCount=\"indefinite\" values=\"\
M0,{hy} Q{w1},{hy2} {w2},{hy} T{w},{hy} L{w},{h} L0,{h} Z;\
M0,{hy} Q{w1},{hy3} {w2},{hy} T{w},{hy} L{w},{h} L0,{h} Z;\
M0,{hy} Q{w1},{hy2} {w2},{hy} T{w},{hy} L{w},{h} L0,{h} Z\"/>\
                     </path>",
                    hy = hf * mid,
                    w1 = wf * 0.25,
                    hy2 = hf * (mid + 0.08),
                    hy3 = hf * (mid + 0.02),
                    w2 = wf * 0.5,
                )
        } else {
            format!(
                    "<path d=\"M0,{hy} Q{w1},{hy2} {w2},{hy} T{w},{hy} L{w},{h} L0,{h} Z\" fill=\"#000000\" fill-opacity=\"0.22\"/>",
                    hy = hf * mid,
                    w1 = wf * 0.25,
                    hy2 = hf * (mid + 0.08),
                    w2 = wf * 0.5,
                )
        };
        format!(
                "{base}{sun}\
                 <rect y=\"{band}\" width=\"{w}\" height=\"{bh}\" fill=\"#000000\" fill-opacity=\"0.18\"/>\
                 {ground}{sheen}{vig}",
                base = field_stack(w, h, plan),
                band = hf * (mid - 0.02),
                bh = hf * 0.04,
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
    }

    // Signature liquid-banner waves (multi-layer SMIL morph — readable at README size).
}
pub(super) fn wave(art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    // Capsule-class restrained waves (ADR-0004): the canvas is the
    // theme's deep base — never a full-color wash; color lives only in
    // the layered gradient waves along the bottom (header) / top
    // (footer, via the flip wrapper). Three quiet layers + one foam
    // crest, nothing else.
    let wild = art == Art::Waving;
    let gain = if g < 0.01 { 0.0 } else { g.max(0.25) };
    let mut body = format!(
        "<rect width=\"{w}\" height=\"{h}\" fill=\"{canvas}\"/>",
        canvas = ink_canvas(&plan.base)
    );
    // mid_y_ratio, amp_ratio, opacity, dur, paint
    let layers: [(f32, f32, f32, f32, &str); 3] = if wild {
        [
            (0.74, 0.22, 0.5, 6.5, "url(#mgWaveA)"),
            (0.83, 0.17, 0.4, 8.0, "url(#mgWaveB)"),
            (0.92, 0.12, 0.28, 9.5, "url(#mgWaveC)"),
        ]
    } else {
        [
            (0.78, 0.18, 0.48, 7.0, "url(#mgWaveA)"),
            (0.86, 0.14, 0.38, 8.5, "url(#mgWaveB)"),
            (0.94, 0.10, 0.26, 10.0, "url(#mgWaveC)"),
        ]
    };
    for (k, (mid_y, amp_r, opac, dur, paint)) in layers.iter().enumerate() {
        let y = hf * mid_y;
        let amp = (hf * amp_r * gain).max(hf * 0.05);
        let a = wf * 0.25;
        let b = wf * 0.5;
        let c = wf * 0.75;
        let y_up = y - amp;
        let y_dn = y + amp;
        let y2 = y - amp * 0.35;
        let y3 = y + amp * 0.45;
        let anim = if g > 0.01 {
            format!(
                        "<animate attributeName=\"d\" dur=\"{dur}s\" begin=\"{beg}s\" repeatCount=\"indefinite\" values=\"M0,{y} C{a},{y_up} {b},{y_dn} {c},{y} S{w},{y_up} {w},{y2} L{w},{h} L0,{h} Z;M0,{y} C{a},{y_dn} {b},{y_up} {c},{y} S{w},{y_dn} {w},{y3} L{w},{h} L0,{h} Z;M0,{y} C{a},{y_up} {b},{y_dn} {c},{y} S{w},{y_up} {w},{y2} L{w},{h} L0,{h} Z\"/>",
                        beg = k as f32 * 0.35,
                        y2 = y2,
                        y3 = y3,
                    )
        } else {
            String::new()
        };
        body.push_str(&format!(
                    "<path d=\"M0,{y} C{a},{y_up} {b},{y_dn} {c},{y} S{w},{y_up} {w},{y2} L{w},{h} L0,{h} Z\" fill=\"{paint}\" fill-opacity=\"{opac}\">{anim}</path>",
                    y2 = y2,
                ));
    }
    // Foam crest rides the top wave.
    let cy = hf * if wild { 0.72 } else { 0.76 };
    let camp = hf * if wild { 0.17 } else { 0.13 } * gain;
    let y1 = cy - camp;
    let y2 = cy + camp;
    let dur = if wild { 6.5 } else { 7.0 };
    let foam = if g > 0.01 {
        format!(
                    "<path d=\"M0,{cy} C{a},{y1} {b},{y2} {c},{cy} S{w},{y1} {w},{cy}\" fill=\"none\" stroke=\"{stroke}\" stroke-opacity=\"0.5\" stroke-width=\"1.5\" stroke-linecap=\"round\">\
                       <animate attributeName=\"d\" dur=\"{dur}s\" begin=\"0.18s\" repeatCount=\"indefinite\" values=\"M0,{cy} C{a},{y1} {b},{y2} {c},{cy} S{w},{y1} {w},{cy};M0,{cy} C{a},{y2} {b},{y1} {c},{cy} S{w},{y2} {w},{cy};M0,{cy} C{a},{y1} {b},{y2} {c},{cy} S{w},{y1} {w},{cy}\"/>\
                       <animate attributeName=\"stroke-opacity\" values=\"0.25;0.7;0.25\" dur=\"{dur}s\" begin=\"0.18s\" repeatCount=\"indefinite\"/>\
                     </path>",
                    a = wf * 0.22,
                    b = wf * 0.45,
                    c = wf * 0.6,
                    stroke = plan.glow,
                )
    } else {
        format!(
                    "<path d=\"M0,{cy} C{a},{y1} {b},{y2} {c},{cy} S{w},{y1} {w},{cy}\" fill=\"none\" stroke=\"{stroke}\" stroke-opacity=\"0.35\" stroke-width=\"1.5\" stroke-linecap=\"round\"/>",
                    a = wf * 0.22,
                    b = wf * 0.45,
                    c = wf * 0.6,
                    stroke = plan.glow,
                )
    };
    body.push_str(&foam);
    body
}
pub(super) fn orbit(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let cx = wf * 0.78;
    let cy = hf * 0.5;
    let rx = hf * 0.38;
    let dur = if g > 0.01 { 8.0 / g.max(0.4) } else { 0.0 };
    let planet = if g > 0.01 {
        format!(
                    "<circle cx=\"{px}\" cy=\"{cy}\" r=\"3.8\" fill=\"#ffffff\" fill-opacity=\"0.95\">\
                       <animateTransform attributeName=\"transform\" type=\"rotate\" from=\"0 {cx} {cy}\" to=\"360 {cx} {cy}\" dur=\"{dur}s\" repeatCount=\"indefinite\"/>\
                     </circle>\
                     <circle cx=\"{px2}\" cy=\"{cy}\" r=\"2.4\" fill=\"#ffffff\" fill-opacity=\"0.55\">\
                       <animateTransform attributeName=\"transform\" type=\"rotate\" from=\"180 {cx} {cy}\" to=\"540 {cx} {cy}\" dur=\"{dur2}s\" repeatCount=\"indefinite\"/>\
                     </circle>",
                    px = cx + rx,
                    px2 = cx + hf * 0.26,
                    dur2 = dur * 1.45,
                )
    } else {
        format!(
            "<circle cx=\"{px}\" cy=\"{cy}\" r=\"3.5\" fill=\"#ffffff\" fill-opacity=\"0.9\"/>",
            px = cx + rx
        )
    };
    let rings = if g > 0.01 {
        format!(
                    "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\" stroke-width=\"1.5\">\
                       <animate attributeName=\"stroke-opacity\" values=\"0.12;0.28;0.12\" dur=\"4s\" repeatCount=\"indefinite\"/>\
                     </ellipse>\
                     <ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx2}\" ry=\"{ry2}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\" stroke-width=\"1\">\
                       <animateTransform attributeName=\"transform\" type=\"rotate\" from=\"0 {cx} {cy}\" to=\"-360 {cx} {cy}\" dur=\"{dur3}s\" repeatCount=\"indefinite\"/>\
                     </ellipse>",
                    ry = hf * 0.24,
                    rx2 = hf * 0.26,
                    ry2 = hf * 0.16,
                    dur3 = dur * 1.8,
                )
    } else {
        format!(
                    "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.18\" stroke-width=\"1.5\"/>\
                     <ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx2}\" ry=\"{ry2}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\" stroke-width=\"1\"/>",
                    ry = hf * 0.24,
                    rx2 = hf * 0.26,
                    ry2 = hf * 0.16,
                )
    };
    format!(
                "{base}{rings}\
                 <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{core}\" fill=\"#ffffff\" fill-opacity=\"0.2\" filter=\"url(#softGlow)\">{core_anim}</circle>\
                 {planet}{sheen}{vig}",
                base = field_stack(w, h, plan),
                core = hf * 0.07,
                core_anim = if g > 0.01 {
                    format!(
                        "<animate attributeName=\"r\" values=\"{r};{r2};{r}\" dur=\"3.2s\" repeatCount=\"indefinite\"/>",
                        r = hf * 0.07,
                        r2 = hf * 0.095,
                    )
                } else {
                    String::new()
                },
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
}
pub(super) fn ring(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let cx = wf * 0.82;
    let cy = hf * 0.5;
    let r = hf * 0.32;
    let spin = if g > 0.01 {
        format!(
                    "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.22\" stroke-width=\"2\" stroke-dasharray=\"18 14\">\
                       <animateTransform attributeName=\"transform\" type=\"rotate\" from=\"0 {cx} {cy}\" to=\"360 {cx} {cy}\" dur=\"14s\" repeatCount=\"indefinite\"/>\
                     </circle>\
                     <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r2}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.14\" stroke-width=\"10\">\
                       <animate attributeName=\"stroke-opacity\" values=\"0.08;0.2;0.08\" dur=\"3.5s\" repeatCount=\"indefinite\"/>\
                     </circle>",
                    r2 = hf * 0.22,
                )
    } else {
        format!(
                    "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\" stroke-width=\"14\"/>\
                     <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r2}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\" stroke-width=\"2\"/>",
                    r2 = hf * 0.22,
                )
    };
    format!(
                "{base}{spin}\
                 <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r3}\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>\
                 {sheen}{vig}",
                base = field_stack(w, h, plan),
                r3 = hf * 0.12,
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
}
pub(super) fn beam(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let glow = plan.glow.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let p1 = format!("0,{h} {} ,0 {} ,0 {w},{h}", wf * 0.42, wf * 0.58);
    let p2 = format!(
        "{},{} {} ,{} {} ,{} {},{}",
        wf * 0.25,
        h,
        wf * 0.48,
        hf * 0.18,
        wf * 0.52,
        hf * 0.18,
        wf * 0.75,
        h
    );
    let p3 = format!(
        "{},{} {} ,0 {} ,0 {},{}",
        wf * 0.35,
        h,
        wf * 0.48,
        wf * 0.52,
        wf * 0.65,
        h
    );
    let beams = if g > 0.01 {
        format!(
                    "<g opacity=\"0.9\">\
                       <animate attributeName=\"opacity\" values=\"0.65;1;0.7;1\" dur=\"5s\" repeatCount=\"indefinite\"/>\
                       <polygon points=\"{p1}\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>\
                       <polygon points=\"{p2}\" fill=\"#ffffff\" fill-opacity=\"0.12\">\
                         <animate attributeName=\"fill-opacity\" values=\"0.08;0.16;0.08\" dur=\"3.2s\" repeatCount=\"indefinite\"/>\
                       </polygon>\
                       <polygon points=\"{p3}\" fill=\"#ffffff\" fill-opacity=\"0.06\"/>\
                     </g>",
                    p1 = p1,
                    p2 = p2,
                    p3 = p3,
                )
    } else {
        format!(
            "<polygon points=\"{p1}\" fill=\"#ffffff\" fill-opacity=\"0.07\"/>\
                     <polygon points=\"{p2}\" fill=\"#ffffff\" fill-opacity=\"0.1\"/>\
                     <polygon points=\"{p3}\" fill=\"{glow}\" fill-opacity=\"0.1\"/>",
            p1 = p1,
            p2 = p2,
            p3 = p3,
        )
    };
    format!(
        "{base}{beams}{sheen}{vig}",
        base = field_stack(w, h, plan),
        sheen = sheen(w, h, g, plan),
        vig = vignette(w, h, plan),
    )
}
pub(super) fn terminal(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let cursor = if g > 0.01 {
        format!(
                    "<rect x=\"{cx}\" y=\"{cy}\" width=\"10\" height=\"{ch}\" rx=\"2\" fill=\"#ffffff\" fill-opacity=\"0.75\">\
                       <animate attributeName=\"fill-opacity\" values=\"0.85;0.1;0.85\" dur=\"1.05s\" repeatCount=\"indefinite\"/>\
                     </rect>",
                    cx = wf * 0.1,
                    cy = hf * 0.48,
                    ch = hf * 0.12,
                )
    } else {
        String::new()
    };
    format!(
                "{base}\
                 <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"14\" fill=\"#000000\" fill-opacity=\"0.28\" stroke=\"#ffffff\" stroke-opacity=\"0.1\"/>\
                 <circle cx=\"{d1}\" cy=\"{dy}\" r=\"5\" fill=\"#FF5F56\"/><circle cx=\"{d2}\" cy=\"{dy}\" r=\"5\" fill=\"#FFBD2E\"/><circle cx=\"{d3}\" cy=\"{dy}\" r=\"5\" fill=\"#27C93F\"/>\
                 <rect x=\"{ix}\" y=\"{iy}\" width=\"{iw}\" height=\"{ih}\" rx=\"8\" fill=\"#000000\" fill-opacity=\"0.22\"/>\
                 {cursor}{sheen}",
                base = field_stack(w, h, plan),
                x = wf * 0.04,
                y = hf * 0.1,
                rw = wf * 0.92,
                rh = hf * 0.8,
                d1 = wf * 0.08,
                d2 = wf * 0.08 + 18.0,
                d3 = wf * 0.08 + 36.0,
                dy = hf * 0.2,
                ix = wf * 0.07,
                iy = hf * 0.32,
                iw = wf * 0.86,
                ih = hf * 0.5,
                sheen = sheen(w, h, g, plan),
            )
}
pub(super) fn constellation(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let g = gain;
    let mut pts = Vec::new();
    let mut s: u32 = 42;
    for _ in 0..16 {
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        let x = 50 + s % w.saturating_sub(100).max(1);
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        let y = 30 + s % h.saturating_sub(60).max(1);
        pts.push((x, y));
    }
    let mut edges = String::new();
    for i in 0..pts.len().saturating_sub(1) {
        if i % 2 == 0 {
            continue;
        }
        let (x1, y1) = pts[i];
        let (x2, y2) = pts[(i + 3) % pts.len()];
        let edge = if g > 0.01 {
            format!(
                        "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#ffffff\" stroke-opacity=\"0.14\" stroke-width=\"1\">\
                           <animate attributeName=\"stroke-opacity\" values=\"0.06;0.22;0.06\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                         </line>",
                        dur = 3.5 + (i % 4) as f32 * 0.4,
                        b = (i % 5) as f32 * 0.25,
                    )
        } else {
            format!(
                        "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#ffffff\" stroke-opacity=\"0.14\" stroke-width=\"1\"/>"
                    )
        };
        edges.push_str(&edge);
    }
    let stars: String = pts
                .iter()
                .enumerate()
                .map(|(i, (x, y))| {
                    if g > 0.01 {
                        format!(
                            "<circle cx=\"{x}\" cy=\"{y}\" r=\"2\" fill=\"#ffffff\" fill-opacity=\"0.85\">\
                               <animate attributeName=\"fill-opacity\" values=\"0.35;1;0.45;1\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                               <animate attributeName=\"r\" values=\"1.4;2.4;1.4\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                             </circle>\
                             <circle cx=\"{x}\" cy=\"{y}\" r=\"6\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>",
                            dur = 2.4 + (i % 5) as f32 * 0.35,
                            b = (i % 7) as f32 * 0.18,
                        )
                    } else {
                        format!(
                            "<circle cx=\"{x}\" cy=\"{y}\" r=\"2\" fill=\"#ffffff\" fill-opacity=\"0.85\"/>\
                             <circle cx=\"{x}\" cy=\"{y}\" r=\"6\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>"
                        )
                    }
                })
                .collect();
    format!(
        "{}{}{}{}{}",
        field_stack(w, h, plan),
        edges,
        stars,
        sheen(w, h, g, plan),
        vignette(w, h, plan)
    )
}
