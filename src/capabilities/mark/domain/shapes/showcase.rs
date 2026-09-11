//! Showcase art family — the saturated "SOTA" effects.
//!
//! One function per art type; `super::shape_background` dispatches exhaustively.

use super::{Canvas, Rng};
use crate::capabilities::mark::domain::art::Art;
pub(super) fn plasma(art: Art, c: &Canvas) -> String {
    let (w, h, g) = (c.w, c.h, c.gain);
    let blobs = c.cloud(art);
    let layers = format!(
                "{blobs}                 <rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgHolo)\" opacity=\"0.58\">                   {sweep}                 </rect>                 ",
                sweep = if g > 0.01 {
                    "<animateTransform attributeName=\"transform\" type=\"translate\" values=\"-90 0; 90 0; -90 0\" dur=\"7s\" repeatCount=\"indefinite\"/>"
                } else {
                    ""
                },
            );
    c.finish(c.stack(), &layers)
}
pub(super) fn holo(art: Art, c: &Canvas) -> String {
    let (w, h, wf, hf, g) = (c.w, c.h, c.wf, c.hf, c.gain);
    let accent = c.plan.accent.as_str();
    let warm = c.plan.warm.as_str();
    let glow = c.plan.glow.as_str();
    let bars: String = (0..8)
                .map(|i| {
                    let x = wf * (0.05 + i as f32 * 0.12);
                    let o = 0.05 + (i % 3) as f32 * 0.035;
                    let color = match i % 3 {
                        0 => accent,
                        1 => warm,
                        _ => glow,
                    };
                    let anim = if g > 0.01 {
                        format!(
                            "<animate attributeName=\"opacity\" values=\"{o};{o2};{o}\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>",
                            o2 = (o * 2.6).min(0.42),
                            dur = 2.8 + (i as f32) * 0.25,
                            b = i as f32 * 0.15,
                        )
                    } else {
                        String::new()
                    };
                    format!(
                        "<rect x=\"{x}\" y=\"0\" width=\"{bw}\" height=\"{h}\" fill=\"{color}\" opacity=\"{o}\">{anim}</rect>",
                        bw = wf * 0.045,
                    )
                })
                .collect();
    let layers = format!(
                "                 <rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgHolo)\" opacity=\"0.85\">                   {holo_anim}                 </rect>                 {bars}                 {glow_blob}                 ",
                holo_anim = if g > 0.01 {
                    format!(
                        "<animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; {dx} {dy}; 0 0\" dur=\"6s\" repeatCount=\"indefinite\"/>",
                        dx = wf * 0.1,
                        dy = -hf * 0.05,
                    )
                } else {
                    String::new()
                },
                glow_blob = c.cloud(art),
            );
    c.finish(c.stack(), &layers)
}
pub(super) fn neon(art: Art, c: &Canvas) -> String {
    let (w, h) = (c.w, c.h);
    let g = c.gain;
    let frame = if g > 0.01 {
        format!(
                    "<rect x=\"10\" y=\"10\" width=\"{iw}\" height=\"{ih}\" rx=\"14\" fill=\"none\" stroke=\"#00f5d4\" stroke-width=\"2\" filter=\"url(#neonGlow)\">\
                       <animate attributeName=\"stroke-opacity\" values=\"0.55;1;0.55\" dur=\"2.2s\" repeatCount=\"indefinite\"/>\
                     </rect>\
                     <rect x=\"18\" y=\"18\" width=\"{iw2}\" height=\"{ih2}\" rx=\"10\" fill=\"none\" stroke=\"#7b61ff\" stroke-width=\"1.2\" opacity=\"0.55\">\
                       <animate attributeName=\"stroke-opacity\" values=\"0.25;0.8;0.25\" dur=\"2.8s\" begin=\"0.4s\" repeatCount=\"indefinite\"/>\
                     </rect>",
                    iw = w.saturating_sub(20),
                    ih = h.saturating_sub(20),
                    iw2 = w.saturating_sub(36),
                    ih2 = h.saturating_sub(36),
                )
    } else {
        format!(
                    "<rect x=\"10\" y=\"10\" width=\"{iw}\" height=\"{ih}\" rx=\"14\" fill=\"none\" stroke=\"#00f5d4\" stroke-width=\"2\" opacity=\"0.7\"/>",
                    iw = w.saturating_sub(20),
                    ih = h.saturating_sub(20),
                )
    };
    let layers = format!("{blob}{frame}", blob = c.cloud(art));
    c.finish(c.stack(), &layers)
}
pub(super) fn meteor(art: Art, c: &Canvas) -> String {
    let (wf, hf) = (c.wf, c.hf);
    let g = c.gain;
    let mut streaks = String::new();
    for i in 0..6 {
        let y = hf * (0.12 + i as f32 * 0.14);
        let x0 = -wf * 0.1;
        let x1 = wf * 1.15;
        let dur = 2.4 + (i % 3) as f32 * 0.7;
        let delay = i as f32 * 0.45;
        if g > 0.01 {
            streaks.push_str(&format!(
                        "<g opacity=\"0\">\
                           <animate attributeName=\"opacity\" values=\"0;0.9;0\" keyTimes=\"0;0.15;1\" dur=\"{dur}s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>\
                           <animateTransform attributeName=\"transform\" type=\"translate\" from=\"{x0} 0\" to=\"{x1} {dy}\" dur=\"{dur}s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>\
                           <line x1=\"0\" y1=\"{y}\" x2=\"{len}\" y2=\"{y2}\" stroke=\"#ffffff\" stroke-width=\"2\" stroke-linecap=\"round\" opacity=\"0.85\"/>\
                           <line x1=\"0\" y1=\"{y}\" x2=\"{len2}\" y2=\"{y3}\" stroke=\"#a5b4fc\" stroke-width=\"6\" stroke-linecap=\"round\" opacity=\"0.2\" filter=\"url(#softGlow)\"/>\
                         </g>",
                        dy = hf * 0.08,
                        len = wf * 0.18,
                        len2 = wf * 0.28,
                        y2 = y + 6.0,
                        y3 = y + 10.0,
                    ));
        } else {
            streaks.push_str(&format!(
                        "<line x1=\"{x}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#ffffff\" stroke-width=\"2\" opacity=\"0.35\"/>",
                        x = wf * (0.1 + i as f32 * 0.12),
                        x2 = wf * (0.25 + i as f32 * 0.12),
                        y2 = y + 12.0,
                    ));
        }
    }
    let layers = format!("{blobs}{streaks}", blobs = c.cloud(art));
    c.finish(c.stack(), &layers)
}
pub(super) fn liquid(art: Art, c: &Canvas) -> String {
    let (w, h, wf, hf, g) = (c.w, c.h, c.wf, c.hf, c.gain);
    let path_anim = if g > 0.01 {
        format!(
            "<animate attributeName=\"d\" dur=\"6s\" repeatCount=\"indefinite\" values=\"\
M0,{a} C{c1},{a1} {c2},{a2} {w},{a3} L{w},{h} L0,{h} Z;\
M0,{b} C{c1},{b1} {c2},{b2} {w},{b3} L{w},{h} L0,{h} Z;\
M0,{a} C{c1},{a1} {c2},{a2} {w},{a3} L{w},{h} L0,{h} Z\"/>",
            a = hf * 0.45,
            a1 = hf * 0.25,
            a2 = hf * 0.7,
            a3 = hf * 0.5,
            b = hf * 0.55,
            b1 = hf * 0.75,
            b2 = hf * 0.3,
            b3 = hf * 0.48,
            c1 = wf * 0.28,
            c2 = wf * 0.62,
        )
    } else {
        String::new()
    };
    let layers = format!(
                "<path d=\"M0,{y} C{c1},{y1} {c2},{y2} {w},{y3} L{w},{h} L0,{h} Z\" fill=\"#ffffff\" fill-opacity=\"0.1\" filter=\"url(#softBloom)\">{path_anim}</path>\
                 <path d=\"M0,{y4} C{c3},{y5} {c4},{y6} {w},{y7} L{w},{h} L0,{h} Z\" fill=\"#a5b4fc\" fill-opacity=\"0.12\">{path_anim2}</path>\
                 {blob}",
                y = hf * 0.45,
                c1 = wf * 0.28,
                y1 = hf * 0.25,
                c2 = wf * 0.62,
                y2 = hf * 0.7,
                y3 = hf * 0.5,
                y4 = hf * 0.62,
                c3 = wf * 0.22,
                y5 = hf * 0.85,
                c4 = wf * 0.7,
                y6 = hf * 0.45,
                y7 = hf * 0.7,
                path_anim2 = if g > 0.01 {
                    format!(
                        "<animate attributeName=\"d\" dur=\"7.5s\" begin=\"0.6s\" repeatCount=\"indefinite\" values=\"\
M0,{y4} C{c3},{y5} {c4},{y6} {w},{y7} L{w},{h} L0,{h} Z;\
M0,{y4b} C{c3},{y6} {c4},{y5} {w},{y7b} L{w},{h} L0,{h} Z;\
M0,{y4} C{c3},{y5} {c4},{y6} {w},{y7} L{w},{h} L0,{h} Z\"/>",
                        y4 = hf * 0.62,
                        y4b = hf * 0.7,
                        c3 = wf * 0.22,
                        y5 = hf * 0.85,
                        c4 = wf * 0.7,
                        y6 = hf * 0.45,
                        y7 = hf * 0.7,
                        y7b = hf * 0.58,
                    )
                } else {
                    String::new()
                },
                blob = c.cloud(art),
            );
    c.finish(c.stack(), &layers)
}
pub(super) fn prism(art: Art, c: &Canvas) -> String {
    let (h, wf) = (c.h, c.wf);
    let g = c.gain;
    let warm = c.plan.warm.as_str();
    let beams: String = (0..5)
                .map(|i| {
                    let x = wf * (0.15 + i as f32 * 0.15);
                    let colors = ["#ff6ad5", "#7b61ff", warm, "#5efc8d", "#ffe66d"];
                    let color = colors[i % colors.len()];
                    let anim = if g > 0.01 {
                        format!(
                            "<animate attributeName=\"opacity\" values=\"0.08;0.32;0.08\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>",
                            dur = 3.0 + i as f32 * 0.35,
                            b = i as f32 * 0.2,
                        )
                    } else {
                        String::new()
                    };
                    format!(
                        "<polygon points=\"{x},0 {x2},0 {x3},{h} {x4},{h}\" fill=\"{color}\" opacity=\"0.14\">{anim}</polygon>",
                        x2 = x + wf * 0.08,
                        x3 = x + wf * 0.14,
                        x4 = x - wf * 0.02,
                    )
                })
                .collect();
    let layers = format!("{beams}{blobs}", blobs = c.cloud(art));
    c.finish(c.stack(), &layers)
}
// duplicate-exception: void/firefly are two emitters over the one deterministic scatter walk (Rng::points + per-point pulse); scripts/check-module-budget.py notices regression — the read budget forces extraction if either emitter grows.
pub(super) fn void(art: Art, c: &Canvas) -> String {
    let (w, h, g) = (c.w, c.h, c.gain);
    let mut stars = String::new();
    let pts = Rng::lcg(91).points(40, w, h, (40, 24), (20, 12));
    for (i, p) in pts.iter().enumerate() {
        let (x, y) = (p.x, p.y);
        let r = 0.8 + (p.draw % 3) as f32 * 0.5;
        if g > 0.01 && i % 2 == 0 {
            stars.push_str(&format!(
                        "<circle cx=\"{x}\" cy=\"{y}\" r=\"{r}\" fill=\"#ffffff\" fill-opacity=\"0.75\">\
                           <animate attributeName=\"fill-opacity\" values=\"0.2;1;0.25;1\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                         </circle>",
                        dur = 2.0 + (i % 5) as f32 * 0.4,
                        b = (i % 7) as f32 * 0.17,
                    ));
        } else {
            stars.push_str(&format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"{r}\" fill=\"#ffffff\" fill-opacity=\"0.55\"/>"
            ));
        }
    }
    let layers = format!("{blobs}{stars}", blobs = c.cloud(art));
    c.finish(c.stack(), &layers)
}
pub(super) fn firefly(art: Art, c: &Canvas) -> String {
    let (w, h, g) = (c.w, c.h, c.gain);
    let mut dots = String::new();
    let pts = Rng::scatter(17).points(28, w, h, (60, 40), (30, 20));
    for (i, p) in pts.iter().enumerate() {
        let (x, y) = (p.x, p.y);
        if g > 0.01 {
            let dx = 12.0 + (i % 5) as f32 * 4.0;
            let dy = 8.0 + (i % 4) as f32 * 3.0;
            dots.push_str(&format!(
                        "<g>\
                           <animateTransform attributeName=\"transform\" type=\"translate\" \
                             values=\"0 0; {dx} -{dy}; 0 0; -{dx2} {dy2}; 0 0\" \
                             dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                           <circle cx=\"{x}\" cy=\"{y}\" r=\"2.2\" fill=\"#fde68a\" fill-opacity=\"0.85\" filter=\"url(#neonGlow)\">\
                             <animate attributeName=\"fill-opacity\" values=\"0.25;1;0.3;1\" dur=\"{dur2}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                           </circle>\
                         </g>",
                        dx2 = dx * 0.7,
                        dy2 = dy * 0.5,
                        dur = 5.0 + (i % 6) as f32 * 0.6,
                        dur2 = 1.8 + (i % 4) as f32 * 0.3,
                        b = (i % 9) as f32 * 0.2,
                    ));
        } else {
            dots.push_str(&format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"2\" fill=\"#fde68a\" fill-opacity=\"0.55\"/>"
            ));
        }
    }
    let layers = format!("{blobs}{dots}", blobs = c.cloud(art));
    c.finish(c.stack(), &layers)
}
pub(super) fn silk(art: Art, c: &Canvas) -> String {
    let (w, wf, hf, g) = (c.w, c.wf, c.hf, c.gain);
    let mut waves = String::new();
    for i in 0..4 {
        let y = hf * (0.25 + i as f32 * 0.16);
        let amp = hf * (0.06 + i as f32 * 0.015) * g.max(0.2);
        let anim = if g > 0.01 {
            format!(
                        "<animate attributeName=\"d\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\" values=\"\
M0,{y} Q{q1},{y1} {m},{y} T{w},{y};\
M0,{y} Q{q1},{y2} {m},{y} T{w},{y};\
M0,{y} Q{q1},{y1} {m},{y} T{w},{y}\"/>",
                        q1 = wf * 0.25,
                        m = wf * 0.5,
                        y1 = y - amp,
                        y2 = y + amp,
                        dur = 5.0 + i as f32 * 0.8,
                        b = i as f32 * 0.25,
                    )
        } else {
            String::new()
        };
        waves.push_str(&format!(
                    "<path d=\"M0,{y} Q{q1},{y1} {m},{y} T{w},{y}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"{o}\" stroke-width=\"1.4\">{anim}</path>",
                    q1 = wf * 0.25,
                    m = wf * 0.5,
                    y1 = y - amp.max(hf * 0.04),
                    o = 0.1 + i as f32 * 0.04,
                ));
    }
    let layers = format!("{blobs}{waves}", blobs = c.cloud(art));
    c.finish(c.stack(), &layers)
}
pub(super) fn iridescent(art: Art, c: &Canvas) -> String {
    let (w, h, wf) = (c.w, c.h, c.wf);
    let g = c.gain;
    let sweep = if g > 0.01 {
        format!(
                    "<rect width=\"{w}\" height=\"{h}\" fill=\"url(#holoSweep)\" opacity=\"0.55\">\
                       <animateTransform attributeName=\"transform\" type=\"translate\" values=\"-{dx} 0; {dx} 0; -{dx} 0\" dur=\"5.5s\" repeatCount=\"indefinite\"/>\
                       <animate attributeName=\"opacity\" values=\"0.35;0.7;0.35\" dur=\"4s\" repeatCount=\"indefinite\"/>\
                     </rect>",
                    dx = wf * 0.2,
                )
    } else {
        format!("<rect width=\"{w}\" height=\"{h}\" fill=\"url(#holoSweep)\" opacity=\"0.4\"/>")
    };
    let layers = format!("{blobs}{sweep}", blobs = c.cloud(art));
    c.finish(c.stack(), &layers)
}
