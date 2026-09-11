//! Geometry art family — solid shapes, cards, and the transparent canvas.
//!
//! One function per art type; `super::shape_background` dispatches exhaustively.

use super::Canvas;
use crate::capabilities::mark::domain::art::Art;
pub(super) fn transparent(_art: Art, _c: &Canvas) -> String {
    String::new()
}
pub(super) fn rect(_art: Art, c: &Canvas) -> String {
    c.finish(c.stack(), "")
}
pub(super) fn blur(art: Art, c: &Canvas) -> String {
    c.finish(c.stack(), &c.cloud(art))
}
pub(super) fn cylinder(_art: Art, c: &Canvas) -> String {
    let (w, h, hf) = (c.w, c.h, c.hf);
    let fill = c.plan.fill.as_str();
    let r = (h / 2).min(72);
    let gloss = if c.gain > 0.01 {
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
    let base = format!(
                "<path fill=\"{fill}\" d=\"M{r},0 H{} A{r},{r} 0 0 1 {},{h} H{r} A{r},{r} 0 0 1 {r},0 Z\"/>\
                 {gloss}",
                w - r,
                w - r,
            );
    c.finish(base, "")
}
pub(super) fn slice(_art: Art, c: &Canvas) -> String {
    let (w, h, wf) = (c.w, c.h, c.wf);
    let fill = c.plan.fill.as_str();
    let edge = if c.gain > 0.01 {
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
    let base = format!(
        "<polygon fill=\"{fill}\" points=\"0,0 {w},0 {},{h} 0,{h}\"/>{edge}",
        wf * 0.88,
    );
    c.finish(base, "")
}
pub(super) fn egg(_art: Art, c: &Canvas) -> String {
    let (wf, hf) = (c.wf, c.hf);
    let ry = hf * 0.38;
    let ry2 = hf * 0.42;
    let ell = if c.gain > 0.01 {
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
    c.finish(c.stack(), &ell)
}
pub(super) fn shark(_art: Art, canvas: &Canvas) -> String {
    let (w, h, wf, hf) = (canvas.w, canvas.h, canvas.wf, canvas.hf);
    let (y, a, y1, b, y2, cx, y3, d, y4, y5) = (
        hf * 0.45,
        wf * 0.2,
        hf * 0.2,
        wf * 0.45,
        hf * 0.75,
        wf * 0.6,
        hf * 0.4,
        wf * 0.85,
        hf * 0.65,
        hf * 0.5,
    );
    let path = if canvas.gain > 0.01 {
        format!(
                    "<path fill=\"#ffffff\" fill-opacity=\"0.08\" d=\"M0,{y} C{a},{y1} {b},{y2} {cx},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z\">\
                       <animate attributeName=\"d\" dur=\"6s\" repeatCount=\"indefinite\" values=\"\
M0,{y} C{a},{y1} {b},{y2} {cx},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z;\
M0,{y} C{a},{y2} {b},{y1} {cx},{y4} S{d},{y3} {w},{y5} L{w},{h} L0,{h} Z;\
M0,{y} C{a},{y1} {b},{y2} {cx},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z\"/>\
                     </path>",
                )
    } else {
        format!(
                    "<path fill=\"#ffffff\" fill-opacity=\"0.07\" d=\"M0,{y} C{a},{y1} {b},{y2} {cx},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z\"/>",
                )
    };
    canvas.finish(canvas.stack(), &path)
}
pub(super) fn speech(_art: Art, c: &Canvas) -> String {
    let (w, h, wf, hf) = (c.w, c.h, c.wf, c.hf);
    let fill = c.plan.fill.as_str();
    let r = 20.0_f32.min(hf * 0.15);
    let bob = if c.gain > 0.01 {
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
    c.finish(bob, "")
}
pub(super) fn checkered(_art: Art, c: &Canvas) -> String {
    let mut cells = String::new();
    let glow = c.plan.glow.as_str();
    let s = (c.h / 6).max(20);
    let mut y = 0u32;
    let mut i = 0u32;
    while y < c.h {
        let mut x = 0u32;
        while x < c.w {
            if ((x / s) + (y / s)).is_multiple_of(2) {
                if c.gain > 0.01 && i.is_multiple_of(4) {
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
    c.finish(c.stack(), &cells)
}
pub(super) fn product(_art: Art, c: &Canvas) -> String {
    super::pane::product(c)
}
