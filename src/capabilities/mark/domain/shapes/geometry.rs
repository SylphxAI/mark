//! Geometry art family — solid shapes, cards, and the transparent canvas.
//!
//! One function per art type; `super::shape_background` dispatches exhaustively.

use super::{blob, Blob};
use super::{field_stack, sheen, vignette};
use crate::capabilities::mark::domain::art::Art;
use crate::capabilities::mark::domain::color::FillPlan;
pub(super) fn transparent(_art: Art, _w: u32, _h: u32, _plan: &FillPlan, _gain: f32) -> String {
    String::new()

    // ——— SOTA showcase effects ———
}
pub(super) fn rect(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let g = gain;
    format!(
        "{}{}{}",
        field_stack(w, h, plan),
        sheen(w, h, g, plan),
        vignette(w, h, plan)
    )
}
pub(super) fn blur(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    format!(
        "{base}{a}{b}{sheen}{vig}",
        base = field_stack(w, h, plan),
        a = blob(
            Blob {
                center: (wf * 0.28, hf * 0.4),
                size: (wf * 0.35, hf * 0.55),
                color: "#ffffff",
                opacity: 0.2,
                drift: (wf * 0.06, hf * 0.07),
                dur: 10.0,
                phase: 0.0,
            },
            g,
        ),
        b = blob(
            Blob {
                center: (wf * 0.78, hf * 0.65),
                size: (wf * 0.3, hf * 0.45),
                color: "#c4b5fd",
                opacity: 0.16,
                drift: (-wf * 0.05, -hf * 0.06),
                dur: 12.0,
                phase: 1.0,
            },
            g,
        ),
        sheen = sheen(w, h, g, plan),
        vig = vignette(w, h, plan),
    )
}
pub(super) fn cylinder(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let fill = plan.fill.as_str();
    let hf = h as f32;
    let g = gain;
    let r = (h / 2).min(72);
    let gloss = if g > 0.01 {
        format!(
                    "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"#ffffff\" fill-opacity=\"0.1\">\
                       <animate attributeName=\"cx\" values=\"{cx};{cx2};{cx}\" dur=\"5s\" repeatCount=\"indefinite\"/>\
                     </ellipse>",
                    cx = r as f32 * 0.9,
                    cy = hf * 0.5,
                    rx = r as f32 * 0.35,
                    ry = hf * 0.35,
                    cx2 = (w - r) as f32 * 0.95,
                )
    } else {
        String::new()
    };
    format!(
                "<path fill=\"{fill}\" d=\"M{r},0 H{} A{r},{r} 0 0 1 {},{h} H{r} A{r},{r} 0 0 1 {r},0 Z\"/>\
                 {gloss}{}{}",
                w - r,
                w - r,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
}
pub(super) fn slice(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let fill = plan.fill.as_str();
    let wf = w as f32;
    let g = gain;
    let edge = if g > 0.01 {
        format!(
                    "<polygon fill=\"#ffffff\" fill-opacity=\"0.06\" points=\"{x},0 {w},0 {x2},{h} {x3},{h}\">\
                       <animate attributeName=\"fill-opacity\" values=\"0.03;0.1;0.03\" dur=\"4s\" repeatCount=\"indefinite\"/>\
                     </polygon>",
                    x = wf * 0.82,
                    x2 = wf * 0.88,
                    x3 = wf * 0.72,
                )
    } else {
        String::new()
    };
    format!(
        "<polygon fill=\"{fill}\" points=\"0,0 {w},0 {},{h} 0,{h}\"/>{edge}{}{}",
        wf * 0.88,
        sheen(w, h, g, plan),
        vignette(w, h, plan)
    )
}
pub(super) fn egg(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let ry = hf * 0.38;
    let ry2 = hf * 0.42;
    let ell = if g > 0.01 {
        format!(
                    "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"#ffffff\" fill-opacity=\"0.08\">\
                       <animate attributeName=\"ry\" values=\"{ry};{ry2};{ry}\" dur=\"5s\" repeatCount=\"indefinite\"/>\
                       <animate attributeName=\"fill-opacity\" values=\"0.05;0.12;0.05\" dur=\"5s\" repeatCount=\"indefinite\"/>\
                     </ellipse>",
                    cx = wf / 2.0,
                    cy = hf / 2.0,
                    rx = wf * 0.42,
                )
    } else {
        format!(
                    "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>",
                    cx = wf / 2.0,
                    cy = hf / 2.0,
                    rx = wf * 0.42,
                )
    };
    format!(
        "{}{}{}{}",
        field_stack(w, h, plan),
        ell,
        sheen(w, h, g, plan),
        vignette(w, h, plan)
    )
}
pub(super) fn shark(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let path = if g > 0.01 {
        format!(
                    "<path fill=\"#ffffff\" fill-opacity=\"0.08\" d=\"M0,{y} C{a},{y1} {b},{y2} {c},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z\">\
                       <animate attributeName=\"d\" dur=\"6s\" repeatCount=\"indefinite\" values=\"\
M0,{y} C{a},{y1} {b},{y2} {c},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z;\
M0,{y} C{a},{y2} {b},{y1} {c},{y4} S{d},{y3} {w},{y5} L{w},{h} L0,{h} Z;\
M0,{y} C{a},{y1} {b},{y2} {c},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z\"/>\
                     </path>",
                    y = hf * 0.45,
                    a = wf * 0.2,
                    y1 = hf * 0.2,
                    b = wf * 0.45,
                    y2 = hf * 0.75,
                    c = wf * 0.6,
                    y3 = hf * 0.4,
                    d = wf * 0.85,
                    y4 = hf * 0.65,
                    y5 = hf * 0.5,
                )
    } else {
        format!(
                    "<path fill=\"#ffffff\" fill-opacity=\"0.07\" d=\"M0,{y} C{a},{y1} {b},{y2} {c},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z\"/>",
                    y = hf * 0.45,
                    a = wf * 0.2,
                    y1 = hf * 0.2,
                    b = wf * 0.45,
                    y2 = hf * 0.75,
                    c = wf * 0.6,
                    y3 = hf * 0.4,
                    d = wf * 0.85,
                    y4 = hf * 0.65,
                    y5 = hf * 0.5,
                )
    };
    format!(
        "{}{}{}{}",
        field_stack(w, h, plan),
        path,
        sheen(w, h, g, plan),
        vignette(w, h, plan)
    )
}
pub(super) fn speech(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let fill = plan.fill.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    let r = 20.0_f32.min(hf * 0.15);
    let bob = if g > 0.01 {
        format!(
                    "<g>\
                       <animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; 0 -2.5; 0 0\" dur=\"3.2s\" repeatCount=\"indefinite\"/>\
                       <path fill=\"{fill}\" d=\"M{r},0 H{} Q{w},0 {w},{r} V{} Q{w},{} {},{} H{} L{},{h} L{},{} H{r} Q0,{} 0,{} V{r} Q0,0 {r},0 Z\"/>\
                     </g>",
                    wf - r,
                    hf * 0.72 - r,
                    hf * 0.72,
                    wf - r,
                    hf * 0.72,
                    wf * 0.28,
                    wf * 0.18,
                    wf * 0.22,
                    hf * 0.72,
                    hf * 0.72,
                    hf * 0.72 - r,
                )
    } else {
        format!(
                    "<path fill=\"{fill}\" d=\"M{r},0 H{} Q{w},0 {w},{r} V{} Q{w},{} {},{} H{} L{},{h} L{},{} H{r} Q0,{} 0,{} V{r} Q0,0 {r},0 Z\"/>",
                    wf - r,
                    hf * 0.72 - r,
                    hf * 0.72,
                    wf - r,
                    hf * 0.72,
                    wf * 0.28,
                    wf * 0.18,
                    wf * 0.22,
                    hf * 0.72,
                    hf * 0.72,
                    hf * 0.72 - r,
                )
    };
    format!("{}{}{}", bob, sheen(w, h, g, plan), vignette(w, h, plan))
}
pub(super) fn checkered(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let glow = plan.glow.as_str();
    let g = gain;
    let s = (h / 6).max(20);
    let mut cells = String::new();
    let mut y = 0u32;
    let mut i = 0u32;
    while y < h {
        let mut x = 0u32;
        while x < w {
            if ((x / s) + (y / s)).is_multiple_of(2) {
                if g > 0.01 && i.is_multiple_of(4) {
                    cells.push_str(&format!(
                                "<rect x=\"{x}\" y=\"{y}\" width=\"{s}\" height=\"{s}\" fill=\"{glow}\" fill-opacity=\"0.1\">\
                                   <animate attributeName=\"fill-opacity\" values=\"0.03;0.1;0.03\" dur=\"3s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                                 </rect>",
                                b = (i % 8) as f32 * 0.15,
                            ));
                } else {
                    cells.push_str(&format!(
                                "<rect x=\"{x}\" y=\"{y}\" width=\"{s}\" height=\"{s}\" fill=\"{glow}\" fill-opacity=\"0.1\"/>"
                            ));
                }
                i += 1;
            }
            x += s;
        }
        y += s;
    }
    format!(
        "{}{}{}{}",
        field_stack(w, h, plan),
        cells,
        sheen(w, h, g, plan),
        vignette(w, h, plan)
    )
}
pub(super) fn product(_art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let _fill = plan.fill.as_str();
    let accent = plan.accent.as_str();
    let glow = plan.glow.as_str();
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;
    {
        let card = if g > 0.01 {
            format!(
                    "<g>                       <animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; 0 -3; 0 0\" dur=\"4.5s\" repeatCount=\"indefinite\"/>                       <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"16\" fill=\"{glow}\" fill-opacity=\"0.12\" stroke=\"{accent}\" stroke-opacity=\"0.35\" stroke-width=\"1.2\"/>                       <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"8\" fill=\"{accent}\" fill-opacity=\"0.16\">                         <animate attributeName=\"fill-opacity\" values=\"0.1;0.28;0.1\" dur=\"2.8s\" repeatCount=\"indefinite\"/>                       </rect>                     </g>",
                    x = wf * 0.72,
                    y = hf * 0.22,
                    rw = wf * 0.2,
                    rh = hf * 0.56,
                    x2 = wf * 0.75,
                    y2 = hf * 0.32,
                    rw2 = wf * 0.14,
                    rh2 = hf * 0.08,
                )
        } else {
            format!(
                    "<rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"16\" fill=\"{glow}\" fill-opacity=\"0.12\" stroke=\"{accent}\" stroke-opacity=\"0.3\"/>",
                    x = wf * 0.72,
                    y = hf * 0.22,
                    rw = wf * 0.2,
                    rh = hf * 0.56,
                )
        };
        format!(
            "{}{}{}{}{}",
            field_stack(w, h, plan),
            blob(
                Blob {
                    center: (wf * 0.3, hf * 0.4),
                    size: (wf * 0.28, hf * 0.4),
                    color: accent,
                    opacity: 0.16,
                    drift: (wf * 0.04, 0.0),
                    dur: 9.0,
                    phase: 0.0,
                },
                g,
            ),
            card,
            sheen(w, h, g, plan),
            vignette(w, h, plan)
        )
    }
}
