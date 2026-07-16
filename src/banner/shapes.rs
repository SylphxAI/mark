//! Banner backgrounds with ambient SMIL motion (works in SVG-as-`<img>`).
//!
//! `gain` (0..1) scales motion intensity; 0 freezes decorative layers.

pub const BANNER_TYPES: &[&str] = &[
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
    "aurora", "mesh", "glass", "soft", "horizon", "dusk", "orbit", "beam", "wave", "waving",
    "terminal", "constellation",
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

pub fn shape_defs(ty: &str, gain: f32) -> String {
    let mut d = String::new();
    d.push_str(
        r##"<filter id="softGlow" x="-30%" y="-30%" width="160%" height="160%">
          <feGaussianBlur stdDeviation="26" result="b"/>
          <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
        </filter>
        <filter id="blurf"><feGaussianBlur stdDeviation="34"/></filter>
        <linearGradient id="shine" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="#ffffff" stop-opacity="0.14"/>
          <stop offset="45%" stop-color="#ffffff" stop-opacity="0"/>
        </linearGradient>
        <radialGradient id="vignette" cx="50%" cy="40%" r="75%">
          <stop offset="0%" stop-color="#ffffff" stop-opacity="0"/>
          <stop offset="100%" stop-color="#000000" stop-opacity="0.28"/>
        </radialGradient>
        <linearGradient id="scan" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="#ffffff" stop-opacity="0"/>
          <stop offset="45%" stop-color="#ffffff" stop-opacity="0.12"/>
          <stop offset="55%" stop-color="#ffffff" stop-opacity="0.12"/>
          <stop offset="100%" stop-color="#ffffff" stop-opacity="0"/>
        </linearGradient>"##,
    );
    if ty == "glass" {
        d.push_str(
            r##"<linearGradient id="glassEdge" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="#ffffff" stop-opacity="0.38"/>
              <stop offset="50%" stop-color="#ffffff" stop-opacity="0.06"/>
              <stop offset="100%" stop-color="#ffffff" stop-opacity="0.2"/>
            </linearGradient>"##,
        );
    }
    // slow drifting gradient for ambient sheen
    if gain > 0.01 {
        d.push_str(
            r##"<linearGradient id="drift" x1="0%" y1="0%" x2="100%" y2="0%">
              <stop offset="0%" stop-color="#ffffff" stop-opacity="0">
                <animate attributeName="offset" values="-0.2;1.2;-0.2" dur="9s" repeatCount="indefinite"/>
              </stop>
              <stop offset="50%" stop-color="#ffffff" stop-opacity="0.16">
                <animate attributeName="offset" values="0.15;0.85;0.15" dur="9s" repeatCount="indefinite"/>
              </stop>
              <stop offset="100%" stop-color="#ffffff" stop-opacity="0">
                <animate attributeName="offset" values="0.5;1.4;0.5" dur="9s" repeatCount="indefinite"/>
              </stop>
            </linearGradient>"##,
        );
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

fn sheen(w: u32, h: u32, gain: f32) -> String {
    if gain > 0.01 {
        format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"url(#shine)\"/>\
             <rect width=\"{w}\" height=\"{h}\" fill=\"url(#drift)\" opacity=\"{o:.2}\"/>",
            o = 0.55 * gain
        )
    } else {
        format!("<rect width=\"{w}\" height=\"{h}\" fill=\"url(#shine)\"/>")
    }
}

fn vignette(w: u32, h: u32) -> String {
    format!("<rect width=\"{w}\" height=\"{h}\" fill=\"url(#vignette)\"/>")
}

/// Soft blob that drifts when gain > 0.
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
    if gain < 0.01 {
        return format!(
            "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"{color}\" fill-opacity=\"{opacity}\" filter=\"url(#softGlow)\"/>"
        );
    }
    let adx = dx * gain;
    let ady = dy * gain;
    format!(
        "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"{color}\" fill-opacity=\"{opacity}\" filter=\"url(#softGlow)\">\
           <animate attributeName=\"cx\" values=\"{cx};{x2};{cx};{x3};{cx}\" keyTimes=\"0;0.25;0.5;0.75;1\" dur=\"{dur}s\" begin=\"{phase}s\" repeatCount=\"indefinite\"/>\
           <animate attributeName=\"cy\" values=\"{cy};{y2};{cy};{y3};{cy}\" keyTimes=\"0;0.25;0.5;0.75;1\" dur=\"{dur}s\" begin=\"{phase}s\" repeatCount=\"indefinite\"/>\
           <animate attributeName=\"fill-opacity\" values=\"{opacity};{o2};{opacity};{o3};{opacity}\" dur=\"{dur}s\" begin=\"{phase}s\" repeatCount=\"indefinite\"/>\
         </ellipse>",
        x2 = cx + adx,
        y2 = cy - ady,
        x3 = cx - adx * 0.7,
        y3 = cy + ady * 0.6,
        o2 = (opacity * 1.25).min(0.42),
        o3 = opacity * 0.75,
    )
}

pub fn shape_background(
    ty: &str,
    w: u32,
    h: u32,
    fill: &str,
    section: &str,
    reversal: bool,
    gain: f32,
) -> String {
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

        "rect" => format!(
            "{}{}{}",
            base_fill(w, h, fill),
            sheen(w, h, g),
            vignette(w, h)
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
                sheen(w, h, g),
                vignette(w, h)
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
                    "<animate attributeName=\"d\" dur=\"8s\" repeatCount=\"indefinite\" \
                       values=\"M0,{y1} C{w1},{y2} {w2},{y3} {w},{y4} L{w},{h} L0,{h} Z;\
M0,{y1b} C{w1},{y2b} {w2},{y3b} {w},{y4b} L{w},{h} L0,{h} Z;\
M0,{y1} C{w1},{y2} {w2},{y3} {w},{y4} L{w},{h} L0,{h} Z\"/>"
                )
            } else {
                String::new()
            };
            format!(
                "{base}{b1}{b2}{b3}\
                 <path d=\"M0,{y1} C{w1},{y2} {w2},{y3} {w},{y4} L{w},{h} L0,{h} Z\" fill=\"#ffffff\" fill-opacity=\"0.06\">{wave}</path>\
                 <path d=\"M0,{y5} C{w3},{y6} {w4},{y7} {w},{y8} L{w},{h} L0,{h} Z\" fill=\"#000000\" fill-opacity=\"0.12\"/>\
                 {sheen}{vig}",
                base = base_fill(w, h, fill),
                b1 = blob(wf * 0.18, hf * 0.35, wf * 0.28, hf * 0.55, "#ffffff", 0.16, g, wf * 0.06, hf * 0.08, 11.0, 0.0),
                b2 = blob(wf * 0.72, hf * 0.25, wf * 0.32, hf * 0.5, "#a5b4fc", 0.22, g, -wf * 0.05, hf * 0.1, 13.0, 1.2),
                b3 = blob(wf * 0.55, hf * 0.85, wf * 0.22, hf * 0.35, "#fbbf24", 0.12, g, wf * 0.04, -hf * 0.06, 9.5, 0.6),
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
            )
        }

        "mesh" => format!(
            "{base}{a}{b}{c}{d}{sheen}{vig}",
            base = base_fill(w, h, fill),
            a = blob(wf * 0.2, hf * 0.3, hf * 0.7, hf * 0.7, "#ffffff", 0.14, g, wf * 0.08, hf * 0.1, 12.0, 0.0),
            b = blob(wf * 0.85, hf * 0.2, hf * 0.65, hf * 0.65, "#c4b5fd", 0.18, g, -wf * 0.07, hf * 0.09, 14.0, 0.8),
            c = blob(wf * 0.55, hf * 0.95, hf * 0.55, hf * 0.55, "#67e8f9", 0.12, g, wf * 0.05, -hf * 0.08, 10.0, 1.5),
            d = blob(wf * 0.4, hf * 0.15, hf * 0.4, hf * 0.4, "#fcd34d", 0.1, g, -wf * 0.04, hf * 0.06, 11.5, 0.4),
            sheen = sheen(w, h, g),
            vig = vignette(w, h),
        ),

        "glass" => {
            let panel = if g > 0.01 {
                format!(
                    "<g>\
                       <animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; 0 -3; 0 0; 0 2; 0 0\" dur=\"7s\" repeatCount=\"indefinite\"/>\
                       <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"22\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"url(#glassEdge)\" stroke-width=\"1.2\"/>\
                       <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"18\" fill=\"#ffffff\" fill-opacity=\"0.05\">\
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
                base = base_fill(w, h, fill),
                blob = blob(wf * 0.75, hf * 0.3, wf * 0.25, hf * 0.45, "#ffffff", 0.1, g, -wf * 0.04, hf * 0.05, 9.0, 0.0),
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
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
                base = base_fill(w, h, fill),
                band = hf * (mid - 0.02),
                bh = hf * 0.04,
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
            )
        }

        "wave" | "waving" => {
            let y = hf * 0.64;
            let amp = hf * 0.1 * g.max(0.15);
            let path_anim = if g > 0.01 {
                format!(
                    "<animate attributeName=\"d\" dur=\"4.5s\" repeatCount=\"indefinite\" values=\"\
M0,{y} C{a},{y1} {b},{y2} {c},{y} S{d},{y3} {w},{y} L{w},{h} L0,{h} Z;\
M0,{y} C{a},{y2} {b},{y1} {c},{y} S{d},{y4} {w},{y} L{w},{h} L0,{h} Z;\
M0,{y} C{a},{y1} {b},{y2} {c},{y} S{d},{y3} {w},{y} L{w},{h} L0,{h} Z\"/>",
                    a = wf * 0.2,
                    y1 = y - amp,
                    b = wf * 0.38,
                    y2 = y + amp,
                    c = wf * 0.52,
                    d = wf * 0.78,
                    y3 = y - amp * 0.6,
                    y4 = y + amp * 0.5,
                )
            } else {
                String::new()
            };
            format!(
                "{base}\
                 <path d=\"M0,{y} C{a},{y1} {b},{y2} {c},{y} S{d},{y3} {w},{y} L{w},{h} L0,{h} Z\" fill=\"#ffffff\" fill-opacity=\"0.09\">{path_anim}</path>\
                 <path d=\"M0,{y4} C{a2},{y5} {b2},{y6} {c2},{y4} S{d2},{y7} {w},{y4} L{w},{h} L0,{h} Z\" fill=\"#000000\" fill-opacity=\"0.14\">{path_anim2}</path>\
                 {sheen}{vig}",
                base = base_fill(w, h, fill),
                a = wf * 0.2,
                y1 = y - amp.max(hf * 0.06),
                b = wf * 0.38,
                y2 = y + amp.max(hf * 0.06),
                c = wf * 0.52,
                d = wf * 0.78,
                y3 = y - amp.max(hf * 0.04),
                y4 = hf * 0.74,
                a2 = wf * 0.18,
                y5 = hf * 0.88,
                b2 = wf * 0.4,
                y6 = hf * 0.62,
                c2 = wf * 0.55,
                d2 = wf * 0.82,
                y7 = hf * 0.78,
                path_anim2 = if g > 0.01 {
                    format!(
                        "<animate attributeName=\"d\" dur=\"5.5s\" repeatCount=\"indefinite\" values=\"\
M0,{y4} C{a2},{y5} {b2},{y6} {c2},{y4} S{d2},{y7} {w},{y4} L{w},{h} L0,{h} Z;\
M0,{y4} C{a2},{y6} {b2},{y5} {c2},{y4} S{d2},{y5} {w},{y4} L{w},{h} L0,{h} Z;\
M0,{y4} C{a2},{y5} {b2},{y6} {c2},{y4} S{d2},{y7} {w},{y4} L{w},{h} L0,{h} Z\"/>",
                        y4 = hf * 0.74,
                        a2 = wf * 0.18,
                        y5 = hf * 0.88,
                        b2 = wf * 0.4,
                        y6 = hf * 0.62,
                        c2 = wf * 0.55,
                        d2 = wf * 0.82,
                        y7 = hf * 0.78,
                    )
                } else {
                    String::new()
                },
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
            )
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
                base = base_fill(w, h, fill),
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
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
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
                base = base_fill(w, h, fill),
                r3 = hf * 0.12,
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
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
                     <polygon points=\"{p3}\" fill=\"#ffffff\" fill-opacity=\"0.05\"/>",
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
                base = base_fill(w, h, fill),
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
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
                base = base_fill(w, h, fill),
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
                sheen = sheen(w, h, g),
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
                base_fill(w, h, fill),
                edges,
                stars,
                sheen(w, h, g),
                vignette(w, h)
            )
        }

        "blur" => format!(
            "{base}{a}{b}{sheen}{vig}",
            base = base_fill(w, h, fill),
            a = blob(wf * 0.28, hf * 0.4, wf * 0.35, hf * 0.55, "#ffffff", 0.2, g, wf * 0.06, hf * 0.07, 10.0, 0.0),
            b = blob(wf * 0.78, hf * 0.65, wf * 0.3, hf * 0.45, "#c4b5fd", 0.16, g, -wf * 0.05, -hf * 0.06, 12.0, 1.0),
            sheen = sheen(w, h, g),
            vig = vignette(w, h),
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
                base_fill(w, h, fill),
                lines,
                scan,
                sheen(w, h, g),
                vignette(w, h)
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
                base_fill(w, h, fill),
                traces,
                sheen(w, h, g),
                vignette(w, h)
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
                base = base_fill(w, h, fill),
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
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
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
                base = base_fill(w, h, fill),
                sheen = sheen(w, h, g),
                vig = vignette(w, h),
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
                base_fill(w, h, fill),
                dots,
                sheen(w, h, g),
                vignette(w, h)
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
                sheen(w, h, g),
                vignette(w, h)
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
                sheen(w, h, g),
                vignette(w, h)
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
                base_fill(w, h, fill),
                ell,
                sheen(w, h, g),
                vignette(w, h)
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
                base_fill(w, h, fill),
                path,
                sheen(w, h, g),
                vignette(w, h)
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
            format!("{}{}{}", bob, sheen(w, h, g), vignette(w, h))
        }

        "checkered" => {
            let s = (h / 6).max(20);
            let mut cells = String::new();
            let mut y = 0u32;
            let mut i = 0u32;
            while y < h {
                let mut x = 0u32;
                while x < w {
                    if ((x / s) + (y / s)) % 2 == 0 {
                        if g > 0.01 && i % 4 == 0 {
                            cells.push_str(&format!(
                                "<rect x=\"{x}\" y=\"{y}\" width=\"{s}\" height=\"{s}\" fill=\"#ffffff\" fill-opacity=\"0.05\">\
                                   <animate attributeName=\"fill-opacity\" values=\"0.03;0.1;0.03\" dur=\"3s\" begin=\"{b}s\" repeatCount=\"indefinite\"/>\
                                 </rect>",
                                b = (i % 8) as f32 * 0.15,
                            ));
                        } else {
                            cells.push_str(&format!(
                                "<rect x=\"{x}\" y=\"{y}\" width=\"{s}\" height=\"{s}\" fill=\"#ffffff\" fill-opacity=\"0.05\"/>"
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
                base_fill(w, h, fill),
                cells,
                sheen(w, h, g),
                vignette(w, h)
            )
        }

        "product" | "oss" | "org" => {
            let card = if g > 0.01 {
                format!(
                    "<g>\
                       <animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; 0 -3; 0 0\" dur=\"4.5s\" repeatCount=\"indefinite\"/>\
                       <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"16\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"#ffffff\" stroke-opacity=\"0.12\"/>\
                       <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"8\" fill=\"#ffffff\" fill-opacity=\"0.06\">\
                         <animate attributeName=\"fill-opacity\" values=\"0.04;0.12;0.04\" dur=\"2.8s\" repeatCount=\"indefinite\"/>\
                       </rect>\
                     </g>",
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
                    "<rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"16\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"#ffffff\" stroke-opacity=\"0.1\"/>",
                    x = wf * 0.72,
                    y = hf * 0.22,
                    rw = wf * 0.2,
                    rh = hf * 0.56,
                )
            };
            format!(
                "{}{}{}{}",
                base_fill(w, h, fill),
                card,
                sheen(w, h, g),
                vignette(w, h)
            )
        }

        _ => format!(
            "{}{}{}",
            base_fill(w, h, fill),
            sheen(w, h, g),
            vignette(w, h)
        ),
    };

    wrap(&transforms, body)
}
