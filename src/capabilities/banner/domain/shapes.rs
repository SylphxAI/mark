//! Banner backgrounds with ambient SMIL motion (works in SVG-as-`<img>`).
//!
//! `gain` (0..1) scales motion intensity; 0 freezes decorative layers.

use crate::shared::color::FillPlan;

pub const BANNER_TYPES: &[&str] = &[
    // SOTA showcase first
    "plasma",
    "holo",
    "neon",
    "meteor",
    "liquid",
    "prism",
    "void",
    "firefly",
    "silk",
    "iridescent",
    // Core polished set
    "aurora",
    "mesh",
    "glass",
    "soft",
    "horizon",
    "dusk",
    "orbit",
    "beam",
    "wave",
    "waving",
    "terminal",
    "constellation",
    "grid",
    "blur",
    "ring",
    "circuit",
    "hud",
    "pulse",
    "noise",
    "rounded",
    "rect",
    "slice",
    "cylinder",
    "checkered",
    "egg",
    "shark",
    "venom",
    "speech",
    "product",
    "oss",
    "org",
    "transparent",
];

pub const FEATURED_TYPES: &[&str] = &[
    // Capsule-class classics + solid geometry first, then ethereal showcase
    "wave",
    "waving",
    "soft",
    "rounded",
    "rect",
    "slice",
    "glass",
    "product",
    "terminal",
    "aurora",
    "mesh",
    "plasma",
    "holo",
    "neon",
    "liquid",
    "silk",
    "orbit",
    "constellation",
];

pub fn is_banner_type(v: &str) -> bool {
    BANNER_TYPES.iter().any(|t| t.eq_ignore_ascii_case(v))
}

pub fn normalize_type(v: &str) -> &'static str {
    BANNER_TYPES
        .iter()
        .find(|t| t.eq_ignore_ascii_case(v))
        .copied()
        .unwrap_or("aurora")
}

pub fn shape_defs(ty: &str, gain: f32, plan: &FillPlan) -> String {
    // Filters only — chromatic gradients live on FillPlan (mgSheen/mgHolo/mgDrift…).
    let mut d = String::from(
        r##"<filter id="softGlow" x="-30%" y="-30%" width="160%" height="160%">
          <feGaussianBlur stdDeviation="26" result="b"/>
          <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
        </filter>
        <filter id="blurf"><feGaussianBlur stdDeviation="34"/></filter>
        <filter id="neonGlow" x="-40%" y="-40%" width="180%" height="180%">
          <feGaussianBlur stdDeviation="6" result="b"/>
          <feMerge><feMergeNode in="b"/><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
        </filter>
        <filter id="softBloom" x="-25%" y="-25%" width="150%" height="150%">
          <feGaussianBlur stdDeviation="18" result="b"/>
          <feColorMatrix in="b" type="matrix" values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 0.85 0" result="c"/>
          <feMerge><feMergeNode in="c"/><feMergeNode in="SourceGraphic"/></feMerge>
        </filter>"##,
    );
    if ty == "glass" {
        d.push_str(&format!(
            r##"<linearGradient id="glassEdge" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="{glow}" stop-opacity="0.42"/>
              <stop offset="50%" stop-color="{accent}" stop-opacity="0.08"/>
              <stop offset="100%" stop-color="{warm}" stop-opacity="0.28"/>
            </linearGradient>"##,
            glow = plan.glow,
            accent = plan.accent,
            warm = plan.warm,
        ));
    }
    // Keep aliases so legacy shape markup still resolves when gain freezes drift animation.
    d.push_str(
        r##"<linearGradient id="shine" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="#ffffff" stop-opacity="0.06"/>
          <stop offset="45%" stop-color="#ffffff" stop-opacity="0"/>
        </linearGradient>
        <radialGradient id="vignette" cx="50%" cy="40%" r="75%">
          <stop offset="0%" stop-color="#ffffff" stop-opacity="0"/>
          <stop offset="100%" stop-color="#000000" stop-opacity="0.18"/>
        </radialGradient>
        <linearGradient id="holoSweep" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#ffffff" stop-opacity="0"/>
          <stop offset="50%" stop-color="#ffffff" stop-opacity="0.08"/>
          <stop offset="100%" stop-color="#ffffff" stop-opacity="0"/>
        </linearGradient>"##,
    );
    if gain > 0.01 {
        d.push_str(&format!(
            concat!(
                r##"<linearGradient id="mgDriftAnim" x1="0%" y1="0%" x2="100%" y2="0%">"##,
                r##"<stop offset="0%" stop-color="{edge}" stop-opacity="0">"##,
                r##"<animate attributeName="offset" values="-0.25;1.15;-0.25" dur="8.5s" repeatCount="indefinite"/>"##,
                r##"</stop>"##,
                r##"<stop offset="45%" stop-color="{warm}" stop-opacity="0.28">"##,
                r##"<animate attributeName="offset" values="0.1;0.9;0.1" dur="8.5s" repeatCount="indefinite"/>"##,
                r##"</stop>"##,
                r##"<stop offset="100%" stop-color="{end}" stop-opacity="0">"##,
                r##"<animate attributeName="offset" values="0.45;1.35;0.45" dur="8.5s" repeatCount="indefinite"/>"##,
                r##"</stop></linearGradient>"##,
            ),
            edge = plan.accent,
            warm = plan.warm,
            end = plan.accent2,
        ));
    }
    d
}


fn wrap(transforms: &[String], inner: String) -> String {
    if transforms.is_empty() {
        inner
    } else {
        format!("<g transform=\"{}\">{inner}</g>", transforms.join(" "))
    }
}

fn base_fill(w: u32, h: u32, fill: &str) -> String {
    format!("<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>")
}

fn field_stack(w: u32, h: u32, plan: &FillPlan) -> String {
    format!(
        "{base}<rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgBloom)\" opacity=\"0.85\"/>\
         <rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgBloom2)\" opacity=\"0.7\"/>",
        base = base_fill(w, h, &plan.fill),
    )
}

fn sheen(w: u32, h: u32, gain: f32, _plan: &FillPlan) -> String {
    // Accent-tinted gloss — never a heavy white wash.
    if gain > 0.01 {
        format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgSheen)\" opacity=\"0.9\"/>\
             <rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgDriftAnim)\" opacity=\"{o:.2}\"/>\
             <rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgBloom)\" opacity=\"0.55\"/>",
            o = 0.72 * gain
        )
    } else {
        format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgSheen)\" opacity=\"0.75\"/>\
             <rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgBloom)\" opacity=\"0.4\"/>"
        )
    }
}

fn vignette(w: u32, h: u32, _plan: &FillPlan) -> String {
    format!("<rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgVig)\"/>")
}

/// Soft blob that drifts when gain > 0.
///
/// Motion is applied on a parent `<g>` via `animateTransform` (more reliable than
/// animating `cx`/`cy` on filtered ellipses inside SVG-as-`<img>`).
#[allow(clippy::too_many_arguments)]
fn blob(
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    color: &str,
    opacity: f32,
    gain: f32,
    dx: f32,
    dy: f32,
    dur: f32,
    phase: f32,
) -> String {
    // Amplify motion so ambient drift is obvious at README sizes.
    let adx = (dx.abs().max(28.0) * gain.max(0.01)).copysign(if dx == 0.0 { 1.0 } else { dx });
    let ady = (dy.abs().max(18.0) * gain.max(0.01)).copysign(if dy == 0.0 { -1.0 } else { dy });
    let o2 = (opacity * 1.55).min(0.48);
    let o3 = (opacity * 0.55).max(0.04);
    if gain < 0.01 {
        return format!(
            "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"{color}\" fill-opacity=\"{opacity}\" filter=\"url(#softGlow)\"/>"
        );
    }
    // Slightly shorter cycles so motion is visible within a few seconds of loading.
    let dur = (dur * 0.55).clamp(4.5, 9.0);
    format!(
        "<g>\
           <animateTransform attributeName=\"transform\" type=\"translate\" \
             values=\"0 0; {adx} {ady}; 0 0; {adx2} {ady2}; 0 0\" \
             keyTimes=\"0;0.25;0.5;0.75;1\" dur=\"{dur}s\" begin=\"{phase}s\" repeatCount=\"indefinite\" \
             calcMode=\"spline\" keySplines=\"0.45 0 0.55 1;0.45 0 0.55 1;0.45 0 0.55 1;0.45 0 0.55 1\"/>\
           <ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"{color}\" fill-opacity=\"{opacity}\" filter=\"url(#softGlow)\">\
             <animate attributeName=\"fill-opacity\" values=\"{opacity};{o2};{opacity};{o3};{opacity}\" \
               keyTimes=\"0;0.25;0.5;0.75;1\" dur=\"{dur}s\" begin=\"{phase}s\" repeatCount=\"indefinite\"/>\
             <animate attributeName=\"rx\" values=\"{rx};{rx2};{rx};{rx3};{rx}\" \
               keyTimes=\"0;0.25;0.5;0.75;1\" dur=\"{dur}s\" begin=\"{phase}s\" repeatCount=\"indefinite\"/>\
           </ellipse>\
         </g>",
        adx2 = -adx * 0.75,
        ady2 = ady * 0.55,
        rx2 = rx * 1.12,
        rx3 = rx * 0.92,
    )
}

#[allow(clippy::format_in_format_args)]
pub fn shape_background(
    ty: &str,
    w: u32,
    h: u32,
    plan: &FillPlan,
    section: &str,
    reversal: bool,
    gain: f32,
) -> String {
    let fill = plan.fill.as_str();
    let accent = plan.accent.as_str();
    let accent2 = plan.accent2.as_str();
    let warm = plan.warm.as_str();
    let glow = plan.glow.as_str();
    let base = plan.base.as_str();
    let mid = plan.mid.as_str();
    let _ = mid; // available for type arms that want mid-field paint
    let mut transforms = Vec::new();
    if reversal {
        transforms.push(format!("translate({w},0) scale(-1,1)"));
    }
    if section == "footer" {
        transforms.push(format!("translate(0,{h}) scale(1,-1)"));
    }
    let wf = w as f32;
    let hf = h as f32;
    let g = gain;

    let body = match ty {
        "transparent" => String::new(),

        // ——— SOTA showcase effects ———
        "plasma" => {
            let layers = format!(
                "{a}{b}{c}{d}{e}",
                a = blob(wf * 0.15, hf * 0.4, wf * 0.34, hf * 0.7, accent, 0.36, g, wf * 0.1, hf * 0.12, 8.0, 0.0),
                b = blob(wf * 0.55, hf * 0.2, wf * 0.4, hf * 0.55, accent2, 0.38, g, -wf * 0.08, hf * 0.1, 9.5, 0.4),
                c = blob(wf * 0.85, hf * 0.55, wf * 0.32, hf * 0.6, warm, 0.32, g, -wf * 0.06, -hf * 0.1, 7.5, 0.9),
                d = blob(wf * 0.4, hf * 0.85, wf * 0.28, hf * 0.4, glow, 0.24, g, wf * 0.05, -hf * 0.08, 10.0, 1.2),
                e = blob(wf * 0.72, hf * 0.18, wf * 0.24, hf * 0.36, accent, 0.18, g, -wf * 0.04, hf * 0.06, 6.5, 0.2),
            );
            format!(
                "{base}{layers}                 <rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgHolo)\" opacity=\"0.58\">                   {sweep}                 </rect>                 {sheen}{vig}",
                base = field_stack(w, h, plan),
                sweep = if g > 0.01 {
                    "<animateTransform attributeName=\"transform\" type=\"translate\" values=\"-90 0; 90 0; -90 0\" dur=\"7s\" repeatCount=\"indefinite\"/>"
                } else {
                    ""
                },
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

                "holo" => {
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
            format!(
                "{base}                 <rect width=\"{w}\" height=\"{h}\" fill=\"url(#mgHolo)\" opacity=\"0.85\">                   {holo_anim}                 </rect>                 {bars}                 {glow_blob}                 {sheen}{vig}",
                base = field_stack(w, h, plan),
                holo_anim = if g > 0.01 {
                    format!(
                        "<animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; {dx} {dy}; 0 0\" dur=\"6s\" repeatCount=\"indefinite\"/>",
                        dx = wf * 0.1,
                        dy = -hf * 0.05,
                    )
                } else {
                    String::new()
                },
                glow_blob = blob(wf * 0.7, hf * 0.35, wf * 0.28, hf * 0.45, accent, 0.2, g, -wf * 0.05, hf * 0.04, 8.0, 0.3),
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

                "neon" => {
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
            format!(
                "{base}{blob}{frame}{sheen}{vig}",
                base = field_stack(w, h, plan),
                blob = blob(wf * 0.75, hf * 0.35, wf * 0.2, hf * 0.4, "#00f5d4", 0.12, g, -wf * 0.04, hf * 0.05, 7.0, 0.0),
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

        "meteor" => {
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
            format!(
                "{}{}{}{}{}",
                field_stack(w, h, plan),
                blob(wf * 0.2, hf * 0.3, wf * 0.25, hf * 0.4, accent2, 0.14, g, wf * 0.04, hf * 0.05, 9.0, 0.0),
                streaks,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "liquid" => {
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
            format!(
                "{base}\
                 <path d=\"M0,{y} C{c1},{y1} {c2},{y2} {w},{y3} L{w},{h} L0,{h} Z\" fill=\"#ffffff\" fill-opacity=\"0.1\" filter=\"url(#softBloom)\">{path_anim}</path>\
                 <path d=\"M0,{y4} C{c3},{y5} {c4},{y6} {w},{y7} L{w},{h} L0,{h} Z\" fill=\"#a5b4fc\" fill-opacity=\"0.12\">{path_anim2}</path>\
                 {blob}{sheen}{vig}",
                base = field_stack(w, h, plan),
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
                blob = blob(wf * 0.7, hf * 0.3, wf * 0.25, hf * 0.4, warm, 0.16, g, -wf * 0.05, hf * 0.06, 8.0, 0.3),
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

        "prism" => {
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
            format!(
                "{}{}{}{}{}",
                field_stack(w, h, plan),
                beams,
                blob(wf * 0.5, hf * 0.2, wf * 0.3, hf * 0.3, glow, 0.16, g, 0.0, hf * 0.05, 7.0, 0.0),
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "void" => {
            let mut stars = String::new();
            let mut s: u32 = 91;
            for i in 0..40 {
                s = s.wrapping_mul(1664525).wrapping_add(1013904223);
                let x = 20 + s % w.saturating_sub(40).max(1);
                s = s.wrapping_mul(1664525).wrapping_add(1013904223);
                let y = 12 + s % h.saturating_sub(24).max(1);
                let r = 0.8 + (s % 3) as f32 * 0.5;
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
            format!(
                "{}{}{}{}{}{}",
                field_stack(w, h, plan),
                blob(wf * 0.25, hf * 0.55, wf * 0.3, hf * 0.45, "#4c1d95", 0.35, g, wf * 0.05, -hf * 0.04, 11.0, 0.0),
                blob(wf * 0.75, hf * 0.35, wf * 0.28, hf * 0.4, "#1e3a8a", 0.28, g, -wf * 0.05, hf * 0.05, 10.0, 0.6),
                stars,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "firefly" => {
            let mut dots = String::new();
            let mut s: u32 = 17;
            for i in 0..28 {
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let x = 30 + s % w.saturating_sub(60).max(1);
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let y = 20 + s % h.saturating_sub(40).max(1);
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
            format!(
                "{}{}{}{}{}",
                field_stack(w, h, plan),
                blob(wf * 0.5, hf * 0.7, wf * 0.4, hf * 0.35, "#78350f", 0.2, g, 0.0, -hf * 0.03, 9.0, 0.0),
                dots,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "silk" => {
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
            format!(
                "{}{}{}{}{}",
                field_stack(w, h, plan),
                blob(wf * 0.3, hf * 0.4, wf * 0.3, hf * 0.5, "#e9d5ff", 0.14, g, wf * 0.05, hf * 0.04, 10.0, 0.0),
                waves,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "iridescent" => {
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
            format!(
                "{}{}{}{}{}{}",
                field_stack(w, h, plan),
                blob(wf * 0.2, hf * 0.3, wf * 0.28, hf * 0.45, "#f0abfc", 0.22, g, wf * 0.06, hf * 0.05, 8.0, 0.0),
                blob(wf * 0.8, hf * 0.65, wf * 0.3, hf * 0.4, warm, 0.2, g, -wf * 0.05, -hf * 0.05, 9.0, 0.5),
                sweep,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "rect" => format!(
            "{}{}{}",
            field_stack(w, h, plan),
            sheen(w, h, g, plan),
            vignette(w, h, plan)
        ),

        "soft" | "rounded" => {
            let rx = if ty == "soft" {
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
            format!(
                "<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"{fill}\"/>\
                 {pulse}{}{}{}{}",
                format!("<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"url(#shine)\"/>"),
                border,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "aurora" => {
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
                b1 = blob(wf * 0.25, hf * 0.35, wf * 0.3, hf * 0.5, accent, 0.22, g, wf * 0.05, -hf * 0.04, 9.0, 0.0),
                b2 = blob(wf * 0.7, hf * 0.45, wf * 0.32, hf * 0.48, accent2, 0.24, g, -wf * 0.06, hf * 0.05, 10.0, 0.5),
                b3 = blob(wf * 0.5, hf * 0.2, wf * 0.22, hf * 0.3, warm, 0.16, g, wf * 0.03, hf * 0.04, 8.0, 1.0),
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

                "mesh" => format!(
            "{base}{a}{b}{c}{d}{sheen}{vig}",
            base = field_stack(w, h, plan),
            a = blob(wf * 0.2, hf * 0.35, wf * 0.36, hf * 0.55, accent, 0.3, g, wf * 0.07, hf * 0.05, 8.5, 0.0),
            b = blob(wf * 0.7, hf * 0.3, wf * 0.38, hf * 0.52, accent2, 0.32, g, -wf * 0.06, hf * 0.06, 9.5, 0.4),
            c = blob(wf * 0.5, hf * 0.75, wf * 0.34, hf * 0.42, warm, 0.24, g, wf * 0.04, -hf * 0.05, 10.5, 0.9),
            d = blob(wf * 0.85, hf * 0.65, wf * 0.26, hf * 0.36, glow, 0.18, g, -wf * 0.04, -hf * 0.04, 7.5, 1.3),
            sheen = sheen(w, h, g, plan),
            vig = vignette(w, h, plan),
        ),

                "glass" => {
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
                blob = blob(wf * 0.75, hf * 0.3, wf * 0.25, hf * 0.45, glow, 0.14, g, -wf * 0.04, hf * 0.05, 9.0, 0.0),
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

        "horizon" | "dusk" => {
            let mid = if ty == "horizon" { 0.58 } else { 0.52 };
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
        "wave" | "waving" => {
            // Liquid multi-band crest — colors come from the theme kit, not fixed pastels.
            let wild = ty == "waving";
            // Keep a lively floor only when motion is enabled — never override animation=none.
            let gain = if g < 0.01 { 0.0 } else { g.max(0.25) };
            // mid_y_ratio, amp_ratio, opacity, dur, paint
            let layers: [(f32, f32, f32, f32, &str); 5] = if wild {
                [
                    (0.46, 0.15, 0.34, 3.4, accent2),
                    (0.56, 0.13, 0.38, 4.1, accent),
                    (0.66, 0.11, 0.32, 4.8, warm),
                    (0.76, 0.09, 0.28, 5.6, glow),
                    (0.86, 0.07, 0.22, 6.4, base),
                ]
            } else {
                [
                    (0.50, 0.12, 0.32, 4.6, accent2),
                    (0.60, 0.11, 0.36, 5.5, accent),
                    (0.70, 0.09, 0.30, 6.4, warm),
                    (0.80, 0.075, 0.26, 7.4, glow),
                    (0.88, 0.055, 0.18, 8.6, base),
                ]
            };
            let mut body = field_stack(w, h, plan);
            body.push_str(&blob(
                wf * 0.2,
                hf * 0.26,
                wf * 0.34,
                hf * 0.42,
                accent,
                0.18,
                gain,
                wf * 0.06,
                hf * 0.05,
                9.5,
                0.0,
            ));
            body.push_str(&blob(
                wf * 0.8,
                hf * 0.2,
                wf * 0.3,
                hf * 0.38,
                warm,
                0.2,
                gain,
                -wf * 0.05,
                hf * 0.06,
                10.5,
                0.45,
            ));
            for (i, (mid_y, amp_r, opac, dur, color)) in layers.iter().enumerate() {
                let y = hf * mid_y;
                let amp = (hf * amp_r * gain).max(hf * 0.055);
                let a = wf * 0.16;
                let b = wf * 0.34;
                let c = wf * 0.5;
                let d = wf * 0.72;
                let e = wf * 0.88;
                let y_up = y - amp;
                let y_dn = y + amp;
                let y_up2 = y - amp * 0.55;
                let y_dn2 = y + amp * 0.7;
                let anim = if g > 0.01 {
                    format!(
                        "<animate attributeName=\"d\" dur=\"{dur}s\" begin=\"{beg}s\" repeatCount=\"indefinite\" values=\"M0,{y} C{a},{y_up} {b},{y_dn} {c},{y} S{d},{y_up2} {w},{y} L{w},{h} L0,{h} Z;M0,{y} C{a},{y_dn} {b},{y_up} {c},{y} S{d},{y_dn2} {w},{y} L{w},{h} L0,{h} Z;M0,{y} C{a},{y_up2} {b},{y_dn2} {c},{y} S{e},{y_up} {w},{y} L{w},{h} L0,{h} Z;M0,{y} C{a},{y_up} {b},{y_dn} {c},{y} S{d},{y_up2} {w},{y} L{w},{h} L0,{h} Z\"/>",
                        beg = i as f32 * 0.28,
                    )
                } else {
                    String::new()
                };
                // Prefer chroma wave gradients on the brighter crests.
                let paint = if i == 0 {
                    "url(#mgWaveA)"
                } else if i == 1 {
                    "url(#mgWaveB)"
                } else if i == 2 {
                    "url(#mgWaveC)"
                } else {
                    *color
                };
                body.push_str(&format!(
                    "<path d=\"M0,{y} C{a},{y_up} {b},{y_dn} {c},{y} S{d},{y_up2} {w},{y} L{w},{h} L0,{h} Z\"                        fill=\"{paint}\" fill-opacity=\"{opac}\">{anim}</path>"
                ));
            }
            // Foam crest rides the primary wave in warm/glow.
            if g > 0.01 {
                let cy = hf * if wild { 0.56 } else { 0.61 };
                let camp = hf * if wild { 0.11 } else { 0.085 } * gain;
                let y1 = cy - camp;
                let y2 = cy + camp;
                let y3 = cy - camp * 0.55;
                let y4 = cy + camp * 0.65;
                let dur = if wild { 4.2 } else { 5.6 };
                body.push_str(&format!(
                    "<path d=\"M0,{cy} C{a},{y1} {b},{y2} {c},{cy} S{d},{y3} {w},{cy}\" fill=\"none\"                        stroke=\"{stroke}\" stroke-opacity=\"0.55\" stroke-width=\"2\" stroke-linecap=\"round\">                       <animate attributeName=\"d\" dur=\"{dur}s\" repeatCount=\"indefinite\" values=\"M0,{cy} C{a},{y1} {b},{y2} {c},{cy} S{d},{y3} {w},{cy};M0,{cy} C{a},{y2} {b},{y1} {c},{cy} S{d},{y4} {w},{cy};M0,{cy} C{a},{y1} {b},{y2} {c},{cy} S{d},{y3} {w},{cy}\"/>                       <animate attributeName=\"stroke-opacity\" values=\"0.28;0.78;0.34;0.7;0.28\" dur=\"{dur}s\" repeatCount=\"indefinite\"/>                     </path>",
                    a = wf * 0.18,
                    b = wf * 0.4,
                    c = wf * 0.55,
                    d = wf * 0.78,
                    stroke = glow,
                ));
            }
            body.push_str(&sheen(w, h, g, plan));
            body.push_str(&vignette(w, h, plan));
            body
        }

                "orbit" => {
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

        "ring" => {
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

        "beam" => {
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
                    p1 = format!("0,{h} {} ,0 {} ,0 {w},{h}", wf * 0.42, wf * 0.58),
                    p2 = format!(
                        "{},{} {} ,{} {} ,{} {},{}",
                        wf * 0.25, h, wf * 0.48, hf * 0.18, wf * 0.52, hf * 0.18, wf * 0.75, h
                    ),
                    p3 = format!(
                        "{},{} {} ,0 {} ,0 {},{}",
                        wf * 0.35, h, wf * 0.48, wf * 0.52, wf * 0.65, h
                    ),
                )
            } else {
                format!(
                    "<polygon points=\"{p1}\" fill=\"#ffffff\" fill-opacity=\"0.07\"/>\
                     <polygon points=\"{p2}\" fill=\"#ffffff\" fill-opacity=\"0.1\"/>\
                     <polygon points=\"{p3}\" fill=\"{glow}\" fill-opacity=\"0.1\"/>",
                    p1 = format!("0,{h} {} ,0 {} ,0 {w},{h}", wf * 0.42, wf * 0.58),
                    p2 = format!(
                        "{},{} {} ,{} {} ,{} {},{}",
                        wf * 0.25, h, wf * 0.48, hf * 0.18, wf * 0.52, hf * 0.18, wf * 0.75, h
                    ),
                    p3 = format!(
                        "{},{} {} ,0 {} ,0 {},{}",
                        wf * 0.35, h, wf * 0.48, wf * 0.52, wf * 0.65, h
                    ),
                )
            };
            format!(
                "{base}{beams}{sheen}{vig}",
                base = field_stack(w, h, plan),
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

        "terminal" => {
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

        "constellation" => {
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

        "blur" => format!(
            "{base}{a}{b}{sheen}{vig}",
            base = field_stack(w, h, plan),
            a = blob(wf * 0.28, hf * 0.4, wf * 0.35, hf * 0.55, "#ffffff", 0.2, g, wf * 0.06, hf * 0.07, 10.0, 0.0),
            b = blob(wf * 0.78, hf * 0.65, wf * 0.3, hf * 0.45, "#c4b5fd", 0.16, g, -wf * 0.05, -hf * 0.06, 12.0, 1.0),
            sheen = sheen(w, h, g, plan),
            vig = vignette(w, h, plan),
        ),

        "grid" => {
            let step = 32u32;
            let mut lines = String::new();
            let mut x = 0u32;
            while x <= w {
                lines.push_str(&format!(
                    "<line x1=\"{x}\" y1=\"0\" x2=\"{x}\" y2=\"{h}\" stroke=\"#ffffff\" stroke-opacity=\"0.06\"/>"
                ));
                x += step;
            }
            let mut y = 0u32;
            while y <= h {
                lines.push_str(&format!(
                    "<line x1=\"0\" y1=\"{y}\" x2=\"{w}\" y2=\"{y}\" stroke=\"#ffffff\" stroke-opacity=\"0.06\"/>"
                ));
                y += step;
            }
            let scan = if g > 0.01 {
                format!(
                    "<rect y=\"0\" width=\"{w}\" height=\"{hh}\" fill=\"url(#scan)\" opacity=\"0.7\">\
                       <animate attributeName=\"y\" from=\"-{hh}\" to=\"{h}\" dur=\"4.2s\" repeatCount=\"indefinite\"/>\
                     </rect>",
                    hh = hf * 0.22,
                )
            } else {
                String::new()
            };
            format!(
                "{}{}{}{}{}",
                field_stack(w, h, plan),
                lines,
                scan,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "circuit" => {
            let mut traces = String::new();
            for i in 0..6 {
                let y = hf * 0.2 + i as f32 * (hf * 0.12);
                let mid = wf * (0.22 + (i % 4) as f32 * 0.14);
                let node = if g > 0.01 {
                    format!(
                        "<circle cx=\"{mid:.1}\" cy=\"{y:.1}\" r=\"2.5\" fill=\"#ffffff\" fill-opacity=\"0.4\">\
                           <animate attributeName=\"fill-opacity\" values=\"0.2;0.85;0.2\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                           <animate attributeName=\"r\" values=\"2;3.4;2\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                         </circle>",
                        dur = 2.2 + (i as f32) * 0.25,
                        b = i as f32 * 0.2,
                    )
                } else {
                    format!(
                        "<circle cx=\"{mid:.1}\" cy=\"{y:.1}\" r=\"2.5\" fill=\"#ffffff\" fill-opacity=\"0.35\"/>"
                    )
                };
                let pulse = if g > 0.01 {
                    format!(
                        "<circle r=\"3\" fill=\"#ffffff\" fill-opacity=\"0.75\">\
                           <animateMotion dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\" path=\"M0,{y:.1} H{mid:.1} V{y2:.1} H{w}\"/>\
                           <animate attributeName=\"fill-opacity\" values=\"0;0.9;0\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                         </circle>",
                        y2 = y + 14.0,
                        dur = 3.0 + i as f32 * 0.35,
                        b = i as f32 * 0.35,
                    )
                } else {
                    String::new()
                };
                traces.push_str(&format!(
                    "<path d=\"M0,{y:.1} H{mid:.1} V{y2:.1} H{w}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\" stroke-width=\"1.25\"/>\
                     {node}{pulse}",
                    y2 = y + 14.0,
                ));
            }
            format!(
                "{}{}{}{}",
                field_stack(w, h, plan),
                traces,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "hud" => {
            let sweep = if g > 0.01 {
                format!(
                    "<rect x=\"16\" y=\"16\" width=\"{iw}\" height=\"3\" fill=\"#ffffff\" fill-opacity=\"0.2\">\
                       <animate attributeName=\"y\" values=\"16;{max_y};16\" dur=\"3.8s\" repeatCount=\"indefinite\"/>\
                       <animate attributeName=\"fill-opacity\" values=\"0.05;0.35;0.05\" dur=\"3.8s\" repeatCount=\"indefinite\"/>\
                     </rect>",
                    iw = w.saturating_sub(32),
                    max_y = h.saturating_sub(20),
                )
            } else {
                String::new()
            };
            format!(
                "{base}\
                 <rect x=\"16\" y=\"16\" width=\"{iw}\" height=\"{ih}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\" stroke-dasharray=\"5 4\">{dash}</rect>\
                 <path d=\"M16,40 H40 M16,16 V40\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
                 <path d=\"M{r},40 H{r2} M{r},16 V40\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
                 <path d=\"M16,{b} H40 M16,{bb} V{b}\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
                 <path d=\"M{r},{b} H{r2} M{r},{bb} V{b}\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
                 {sweep}{sheen}{vig}",
                base = field_stack(w, h, plan),
                iw = w.saturating_sub(32),
                ih = h.saturating_sub(32),
                r = w - 16,
                r2 = w - 40,
                b = h - 40,
                bb = h - 16,
                dash = if g > 0.01 {
                    "<animate attributeName=\"stroke-dashoffset\" from=\"0\" to=\"36\" dur=\"2.4s\" repeatCount=\"indefinite\"/>"
                } else {
                    ""
                },
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

        "pulse" => {
            let cx = wf * 0.86;
            let cy = hf * 0.5;
            let r0 = hf * 0.08;
            let r1 = hf * 0.28;
            let rings = if g > 0.01 {
                format!(
                    "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r0}\" fill=\"#ffffff\" fill-opacity=\"0.22\">\
                       <animate attributeName=\"r\" values=\"{r0};{r1};{r0}\" dur=\"2.4s\" repeatCount=\"indefinite\"/>\
                       <animate attributeName=\"fill-opacity\" values=\"0.3;0.02;0.3\" dur=\"2.4s\" repeatCount=\"indefinite\"/>\
                     </circle>\
                     <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r0}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.35\" stroke-width=\"2\">\
                       <animate attributeName=\"r\" values=\"{r0};{r2}\" dur=\"2.4s\" begin=\"0.4s\" repeatCount=\"indefinite\"/>\
                       <animate attributeName=\"stroke-opacity\" values=\"0.4;0\" dur=\"2.4s\" begin=\"0.4s\" repeatCount=\"indefinite\"/>\
                     </circle>\
                     <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{core}\" fill=\"#ffffff\" fill-opacity=\"0.55\">\
                       <animate attributeName=\"fill-opacity\" values=\"0.35;0.85;0.35\" dur=\"1.6s\" repeatCount=\"indefinite\"/>\
                     </circle>",
                    r2 = r1 * 1.15,
                    core = r0 * 0.55,
                )
            } else {
                format!(
                    "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r0}\" fill=\"#ffffff\" fill-opacity=\"0.18\"/>"
                )
            };
            format!(
                "{base}{rings}{sheen}{vig}",
                base = field_stack(w, h, plan),
                sheen = sheen(w, h, g, plan),
                vig = vignette(w, h, plan),
            )
        }

        "noise" => {
            let mut dots = String::new();
            let mut s: u32 = 1;
            for i in 0..140 {
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let x = s % w.max(1);
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let y = s % h.max(1);
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let o = 0.03 + (s % 12) as f32 / 100.0;
                if g > 0.01 && i % 3 == 0 {
                    dots.push_str(&format!(
                        "<circle cx=\"{x}\" cy=\"{y}\" r=\"1\" fill=\"#ffffff\" fill-opacity=\"{o:.2}\">\
                           <animate attributeName=\"fill-opacity\" values=\"{o:.2};{o2:.2};{o:.2}\" dur=\"{dur}s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                         </circle>",
                        o2 = (o * 2.2).min(0.35),
                        dur = 1.8 + (i % 5) as f32 * 0.3,
                        b = (i % 9) as f32 * 0.12,
                    ));
                } else {
                    dots.push_str(&format!(
                        "<circle cx=\"{x}\" cy=\"{y}\" r=\"1\" fill=\"#ffffff\" fill-opacity=\"{o:.2}\"/>"
                    ));
                }
            }
            format!(
                "{}{}{}{}",
                field_stack(w, h, plan),
                dots,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

        "cylinder" => {
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

        "slice" => {
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

        "egg" => {
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

        "shark" | "venom" => {
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

        "speech" => {
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

        "checkered" => {
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

        "product" | "oss" | "org" => {
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
                blob(wf * 0.3, hf * 0.4, wf * 0.28, hf * 0.4, accent, 0.16, g, wf * 0.04, 0.0, 9.0, 0.0),
                card,
                sheen(w, h, g, plan),
                vignette(w, h, plan)
            )
        }

                _ => format!(
            "{}{}{}",
            field_stack(w, h, plan),
            sheen(w, h, g, plan),
            vignette(w, h, plan)
        ),
    };

    wrap(&transforms, body)
}
