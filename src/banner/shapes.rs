//! Banner backgrounds — designed for README beauty, not shape-demo novelty.
//!
//! Every style fills the full canvas with layered light, not a crude clip path.

pub const BANNER_TYPES: &[&str] = &[
    // Featured first (gallery order)
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

/// Styles shown first in the product gallery (curated for quality).
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

pub fn shape_defs(ty: &str) -> String {
    let mut d = String::new();
    // Shared soft light / grain helpers for premium styles
    d.push_str(
        r##"<filter id="softGlow" x="-20%" y="-20%" width="140%" height="140%">
          <feGaussianBlur stdDeviation="28" result="b"/>
          <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
        </filter>
        <filter id="blurf"><feGaussianBlur stdDeviation="32"/></filter>
        <filter id="grain">
          <feTurbulence type="fractalNoise" baseFrequency="0.85" numOctaves="2" stitchTiles="stitch" result="n"/>
          <feColorMatrix type="matrix" values="0 0 0 0 1  0 0 0 0 1  0 0 0 0 1  0 0 0 0.05 0" in="n"/>
          <feComposite operator="in" in2="SourceGraphic"/>
        </filter>
        <linearGradient id="shine" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="#ffffff" stop-opacity="0.14"/>
          <stop offset="45%" stop-color="#ffffff" stop-opacity="0"/>
        </linearGradient>
        <radialGradient id="vignette" cx="50%" cy="40%" r="75%">
          <stop offset="0%" stop-color="#ffffff" stop-opacity="0"/>
          <stop offset="100%" stop-color="#000000" stop-opacity="0.28"/>
        </radialGradient>"##,
    );
    if ty == "glass" {
        d.push_str(
            r##"<linearGradient id="glassEdge" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="#ffffff" stop-opacity="0.35"/>
              <stop offset="50%" stop-color="#ffffff" stop-opacity="0.05"/>
              <stop offset="100%" stop-color="#ffffff" stop-opacity="0.18"/>
            </linearGradient>"##,
        );
    }
    d
}

fn wrap(transforms: &[String], inner: String) -> String {
    if transforms.is_empty() {
        inner
    } else {
        format!(
            "<g transform=\"{}\">{inner}</g>",
            transforms.join(" ")
        )
    }
}

fn base_fill(w: u32, h: u32, fill: &str) -> String {
    format!("<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>")
}

fn sheen(w: u32, h: u32) -> String {
    format!("<rect width=\"{w}\" height=\"{h}\" fill=\"url(#shine)\"/>")
}

fn vignette(w: u32, h: u32) -> String {
    format!("<rect width=\"{w}\" height=\"{h}\" fill=\"url(#vignette)\"/>")
}

pub fn shape_background(
    ty: &str,
    w: u32,
    h: u32,
    fill: &str,
    section: &str,
    reversal: bool,
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

    let body = match ty {
        "transparent" => String::new(),

        "rect" => format!(
            "{}{}{}",
            base_fill(w, h, fill),
            sheen(w, h),
            vignette(w, h)
        ),

        "soft" | "rounded" => {
            let rx = if ty == "soft" {
                (h / 2).min(48)
            } else {
                (h / 4).min(28)
            };
            format!(
                "<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"{fill}\"/>\
                 <rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"url(#shine)\"/>\
                 <rect x=\"1\" y=\"1\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\"/>\
                 {}",
                w.saturating_sub(2),
                h.saturating_sub(2),
                rx.saturating_sub(1),
                vignette(w, h)
            )
        }

        "aurora" => format!(
            "{base}\
             <ellipse cx=\"{c1x}\" cy=\"{c1y}\" rx=\"{r1}\" ry=\"{r1y}\" fill=\"#ffffff\" fill-opacity=\"0.16\" filter=\"url(#softGlow)\"/>\
             <ellipse cx=\"{c2x}\" cy=\"{c2y}\" rx=\"{r2}\" ry=\"{r2y}\" fill=\"#a5b4fc\" fill-opacity=\"0.22\" filter=\"url(#softGlow)\"/>\
             <ellipse cx=\"{c3x}\" cy=\"{c3y}\" rx=\"{r3}\" ry=\"{r3y}\" fill=\"#fbbf24\" fill-opacity=\"0.12\" filter=\"url(#softGlow)\"/>\
             <path d=\"M0,{y1} C{w1},{y2} {w2},{y3} {w},{y4} L{w},{h} L0,{h} Z\" fill=\"#ffffff\" fill-opacity=\"0.06\"/>\
             <path d=\"M0,{y5} C{w3},{y6} {w4},{y7} {w},{y8} L{w},{h} L0,{h} Z\" fill=\"#000000\" fill-opacity=\"0.12\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            c1x = wf * 0.18,
            c1y = hf * 0.35,
            r1 = wf * 0.28,
            r1y = hf * 0.55,
            c2x = wf * 0.72,
            c2y = hf * 0.25,
            r2 = wf * 0.32,
            r2y = hf * 0.5,
            c3x = wf * 0.55,
            c3y = hf * 0.85,
            r3 = wf * 0.22,
            r3y = hf * 0.35,
            y1 = hf * 0.55,
            w1 = wf * 0.25,
            y2 = hf * 0.25,
            w2 = wf * 0.55,
            y3 = hf * 0.85,
            y4 = hf * 0.5,
            y5 = hf * 0.7,
            w3 = wf * 0.3,
            y6 = hf * 0.95,
            w4 = wf * 0.7,
            y7 = hf * 0.55,
            y8 = hf * 0.8,
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "mesh" => format!(
            "{base}\
             <circle cx=\"{a}\" cy=\"{b}\" r=\"{r}\" fill=\"#ffffff\" fill-opacity=\"0.14\" filter=\"url(#softGlow)\"/>\
             <circle cx=\"{c}\" cy=\"{d}\" r=\"{r2}\" fill=\"#c4b5fd\" fill-opacity=\"0.18\" filter=\"url(#softGlow)\"/>\
             <circle cx=\"{e}\" cy=\"{f}\" r=\"{r3}\" fill=\"#67e8f9\" fill-opacity=\"0.12\" filter=\"url(#softGlow)\"/>\
             <circle cx=\"{g}\" cy=\"{hh}\" r=\"{r4}\" fill=\"#fcd34d\" fill-opacity=\"0.1\" filter=\"url(#softGlow)\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            a = wf * 0.2,
            b = hf * 0.3,
            r = hf * 0.7,
            c = wf * 0.85,
            d = hf * 0.2,
            r2 = hf * 0.65,
            e = wf * 0.55,
            f = hf * 0.95,
            r3 = hf * 0.55,
            g = wf * 0.4,
            hh = hf * 0.15,
            r4 = hf * 0.4,
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "glass" => format!(
            "{base}\
             <ellipse cx=\"{c1}\" cy=\"{c2}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"#ffffff\" fill-opacity=\"0.1\" filter=\"url(#softGlow)\"/>\
             <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"22\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"url(#glassEdge)\" stroke-width=\"1.2\"/>\
             <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"18\" fill=\"#ffffff\" fill-opacity=\"0.04\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            c1 = wf * 0.75,
            c2 = hf * 0.3,
            rx = wf * 0.25,
            ry = hf * 0.45,
            x = wf * 0.06,
            y = hf * 0.14,
            rw = wf * 0.88,
            rh = hf * 0.72,
            x2 = wf * 0.1,
            y2 = hf * 0.2,
            rw2 = wf * 0.4,
            rh2 = hf * 0.2,
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "horizon" | "dusk" => {
            let mid = if ty == "horizon" { 0.58 } else { 0.52 };
            format!(
                "{base}\
                 <ellipse cx=\"{sun}\" cy=\"{horizon}\" rx=\"{sr}\" ry=\"{sry}\" fill=\"#ffffff\" fill-opacity=\"0.22\" filter=\"url(#softGlow)\"/>\
                 <rect y=\"{band}\" width=\"{w}\" height=\"{bh}\" fill=\"#000000\" fill-opacity=\"0.18\"/>\
                 <path d=\"M0,{hy} Q{w1},{hy2} {w2},{hy} T{w},{hy} L{w},{h} L0,{h} Z\" fill=\"#000000\" fill-opacity=\"0.22\"/>\
                 {sheen}{vig}",
                base = base_fill(w, h, fill),
                sun = wf * 0.72,
                horizon = hf * mid,
                sr = hf * 0.22,
                sry = hf * 0.12,
                band = hf * (mid - 0.02),
                bh = hf * 0.04,
                hy = hf * mid,
                w1 = wf * 0.25,
                hy2 = hf * (mid + 0.08),
                w2 = wf * 0.5,
                sheen = sheen(w, h),
                vig = vignette(w, h),
            )
        }

        "wave" => format!(
            "{base}\
             <path d=\"M0,{y} C{a},{y1} {b},{y2} {c},{y} S{d},{y3} {w},{y} L{w},{h} L0,{h} Z\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>\
             <path d=\"M0,{y4} C{a2},{y5} {b2},{y6} {c2},{y4} S{d2},{y7} {w},{y4} L{w},{h} L0,{h} Z\" fill=\"#000000\" fill-opacity=\"0.14\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            y = hf * 0.62,
            a = wf * 0.2,
            y1 = hf * 0.48,
            b = wf * 0.35,
            y2 = hf * 0.76,
            c = wf * 0.5,
            d = wf * 0.75,
            y3 = hf * 0.5,
            y4 = hf * 0.72,
            a2 = wf * 0.18,
            y5 = hf * 0.88,
            b2 = wf * 0.4,
            y6 = hf * 0.6,
            c2 = wf * 0.55,
            d2 = wf * 0.82,
            y7 = hf * 0.78,
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "waving" => {
            let y = hf * 0.68;
            let amp = hf * 0.08;
            format!(
                "{base}\
                 <path fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\" stroke-width=\"2\" d=\"M0,{y} Q{qx},{y1} {mx},{y} T{w},{y}\">\
                   <animate attributeName=\"d\" dur=\"5s\" repeatCount=\"indefinite\" \
                     values=\"M0,{y} Q{qx},{y1} {mx},{y} T{w},{y};M0,{y} Q{qx},{y2} {mx},{y} T{w},{y};M0,{y} Q{qx},{y1} {mx},{y} T{w},{y}\"/>\
                 </path>\
                 <path fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.1\" stroke-width=\"1.5\" d=\"M0,{y3} Q{qx},{y4} {mx},{y3} T{w},{y3}\"/>\
                 {sheen}{vig}",
                base = base_fill(w, h, fill),
                qx = wf * 0.25,
                mx = wf * 0.5,
                y1 = y - amp,
                y2 = y + amp,
                y3 = y + hf * 0.1,
                y4 = y + hf * 0.02,
                sheen = sheen(w, h),
                vig = vignette(w, h),
            )
        }

        "orbit" => format!(
            "{base}\
             <ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.18\" stroke-width=\"1.5\"/>\
             <ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx2}\" ry=\"{ry2}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\" stroke-width=\"1\"/>\
             <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{core}\" fill=\"#ffffff\" fill-opacity=\"0.2\" filter=\"url(#softGlow)\"/>\
             <circle cx=\"{px}\" cy=\"{cy}\" r=\"3.5\" fill=\"#ffffff\" fill-opacity=\"0.9\">\
               <animateTransform attributeName=\"transform\" type=\"rotate\" from=\"0 {cx} {cy}\" to=\"360 {cx} {cy}\" dur=\"10s\" repeatCount=\"indefinite\"/>\
             </circle>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            cx = wf * 0.78,
            cy = hf * 0.5,
            rx = hf * 0.38,
            ry = hf * 0.24,
            rx2 = hf * 0.26,
            ry2 = hf * 0.16,
            core = hf * 0.07,
            px = wf * 0.78 + hf * 0.38,
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "ring" => format!(
            "{base}\
             <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\" stroke-width=\"14\"/>\
             <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r2}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\" stroke-width=\"2\"/>\
             <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r3}\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            cx = wf * 0.82,
            cy = hf * 0.5,
            r = hf * 0.32,
            r2 = hf * 0.22,
            r3 = hf * 0.12,
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "beam" => format!(
            "{base}\
             <polygon points=\"{p1}\" fill=\"#ffffff\" fill-opacity=\"0.07\"/>\
             <polygon points=\"{p2}\" fill=\"#ffffff\" fill-opacity=\"0.1\"/>\
             <polygon points=\"{p3}\" fill=\"#ffffff\" fill-opacity=\"0.05\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            p1 = format!("0,{h} {} ,0 {} ,0 {w},{h}", wf * 0.42, wf * 0.58),
            p2 = format!(
                "{},{} {} ,{} {} ,{} {},{}",
                wf * 0.25,
                h,
                wf * 0.48,
                hf * 0.18,
                wf * 0.52,
                hf * 0.18,
                wf * 0.75,
                h
            ),
            p3 = format!(
                "{},{} {} ,0 {} ,0 {},{}",
                wf * 0.35,
                h,
                wf * 0.48,
                wf * 0.52,
                wf * 0.65,
                h
            ),
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "terminal" => format!(
            "{base}\
             <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"14\" fill=\"#000000\" fill-opacity=\"0.28\" stroke=\"#ffffff\" stroke-opacity=\"0.1\"/>\
             <circle cx=\"{d1}\" cy=\"{dy}\" r=\"5\" fill=\"#FF5F56\"/><circle cx=\"{d2}\" cy=\"{dy}\" r=\"5\" fill=\"#FFBD2E\"/><circle cx=\"{d3}\" cy=\"{dy}\" r=\"5\" fill=\"#27C93F\"/>\
             <rect x=\"{ix}\" y=\"{iy}\" width=\"{iw}\" height=\"{ih}\" rx=\"8\" fill=\"#000000\" fill-opacity=\"0.22\"/>\
             {sheen}",
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
            sheen = sheen(w, h),
        ),

        "constellation" => {
            let mut pts = Vec::new();
            let mut s: u32 = 42;
            for _ in 0..14 {
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
                edges.push_str(&format!(
                    "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#ffffff\" stroke-opacity=\"0.14\" stroke-width=\"1\"/>"
                ));
            }
            let stars: String = pts
                .iter()
                .map(|(x, y)| {
                    format!(
                        "<circle cx=\"{x}\" cy=\"{y}\" r=\"2\" fill=\"#ffffff\" fill-opacity=\"0.85\"/>\
                         <circle cx=\"{x}\" cy=\"{y}\" r=\"6\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>"
                    )
                })
                .collect();
            format!(
                "{}{}{}{}{}",
                base_fill(w, h, fill),
                edges,
                stars,
                sheen(w, h),
                vignette(w, h)
            )
        }

        "blur" => format!(
            "{base}\
             <ellipse cx=\"{a}\" cy=\"{b}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"#ffffff\" fill-opacity=\"0.2\" filter=\"url(#blurf)\"/>\
             <ellipse cx=\"{c}\" cy=\"{d}\" rx=\"{rx2}\" ry=\"{ry2}\" fill=\"#c4b5fd\" fill-opacity=\"0.16\" filter=\"url(#blurf)\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            a = wf * 0.28,
            b = hf * 0.4,
            rx = wf * 0.35,
            ry = hf * 0.55,
            c = wf * 0.78,
            d = hf * 0.65,
            rx2 = wf * 0.3,
            ry2 = hf * 0.45,
            sheen = sheen(w, h),
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
            format!(
                "{}{}{}{}",
                base_fill(w, h, fill),
                lines,
                sheen(w, h),
                vignette(w, h)
            )
        }

        "circuit" => {
            let mut traces = String::new();
            for i in 0..6 {
                let y = hf * 0.2 + i as f32 * (hf * 0.12);
                let mid = wf * (0.22 + (i % 4) as f32 * 0.14);
                traces.push_str(&format!(
                    "<path d=\"M0,{y:.1} H{mid:.1} V{y2:.1} H{w}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.12\" stroke-width=\"1.25\"/>\
                     <circle cx=\"{mid:.1}\" cy=\"{y:.1}\" r=\"2.5\" fill=\"#ffffff\" fill-opacity=\"0.35\"/>",
                    y2 = y + 14.0
                ));
            }
            format!(
                "{}{}{}{}",
                base_fill(w, h, fill),
                traces,
                sheen(w, h),
                vignette(w, h)
            )
        }

        "hud" => format!(
            "{base}\
             <rect x=\"16\" y=\"16\" width=\"{iw}\" height=\"{ih}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\" stroke-dasharray=\"5 4\"/>\
             <path d=\"M16,40 H40 M16,16 V40\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
             <path d=\"M{r},40 H{r2} M{r},16 V40\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
             <path d=\"M16,{b} H40 M16,{bb} V{b}\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
             <path d=\"M{r},{b} H{r2} M{r},{bb} V{b}\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            iw = w.saturating_sub(32),
            ih = h.saturating_sub(32),
            r = w - 16,
            r2 = w - 40,
            b = h - 40,
            bb = h - 16,
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "pulse" => {
            let r0 = hf * 0.08;
            let r1 = hf * 0.22;
            format!(
                "{base}\
                 <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r0}\" fill=\"#ffffff\" fill-opacity=\"0.18\">\
                   <animate attributeName=\"r\" values=\"{r0};{r1};{r0}\" dur=\"2.8s\" repeatCount=\"indefinite\"/>\
                   <animate attributeName=\"fill-opacity\" values=\"0.28;0.04;0.28\" dur=\"2.8s\" repeatCount=\"indefinite\"/>\
                 </circle>\
                 {sheen}{vig}",
                base = base_fill(w, h, fill),
                cx = wf * 0.86,
                cy = hf * 0.5,
                sheen = sheen(w, h),
                vig = vignette(w, h),
            )
        }

        "noise" => {
            let mut dots = String::new();
            let mut s: u32 = 1;
            for _ in 0..120 {
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let x = s % w.max(1);
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let y = s % h.max(1);
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let o = 0.03 + (s % 12) as f32 / 100.0;
                dots.push_str(&format!(
                    "<circle cx=\"{x}\" cy=\"{y}\" r=\"1\" fill=\"#ffffff\" fill-opacity=\"{o:.2}\"/>"
                ));
            }
            format!(
                "{}{}{}{}",
                base_fill(w, h, fill),
                dots,
                sheen(w, h),
                vignette(w, h)
            )
        }

        "cylinder" => {
            let r = (h / 2).min(72);
            format!(
                "<path fill=\"{fill}\" d=\"M{r},0 H{} A{r},{r} 0 0 1 {},{h} H{r} A{r},{r} 0 0 1 {r},0 Z\"/>\
                 {}{}",
                w - r,
                w - r,
                sheen(w, h),
                vignette(w, h)
            )
        }

        "slice" => format!(
            "<polygon fill=\"{fill}\" points=\"0,0 {w},0 {},{h} 0,{h}\"/>{}{}",
            wf * 0.88,
            sheen(w, h),
            vignette(w, h)
        ),

        "egg" => format!(
            "{}\
             <ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>\
             {}{}",
            base_fill(w, h, fill),
            wf / 2.0,
            hf / 2.0,
            wf * 0.42,
            hf * 0.38,
            sheen(w, h),
            vignette(w, h)
        ),

        "shark" | "venom" => format!(
            "{base}\
             <path fill=\"#ffffff\" fill-opacity=\"0.07\" d=\"M0,{y} C{a},{y1} {b},{y2} {c},{y3} S{d},{y4} {w},{y5} L{w},{h} L0,{h} Z\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
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
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        "speech" => {
            let r = 20.0_f32.min(hf * 0.15);
            format!(
                "<path fill=\"{fill}\" d=\"M{r},0 H{} Q{w},0 {w},{r} V{} Q{w},{} {},{} H{} L{},{h} L{},{} H{r} Q0,{} 0,{} V{r} Q0,0 {r},0 Z\"/>\
                 {}{}",
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
                sheen(w, h),
                vignette(w, h)
            )
        }

        "checkered" => {
            let s = (h / 6).max(20);
            let mut cells = String::new();
            let mut y = 0u32;
            while y < h {
                let mut x = 0u32;
                while x < w {
                    if ((x / s) + (y / s)) % 2 == 0 {
                        cells.push_str(&format!(
                            "<rect x=\"{x}\" y=\"{y}\" width=\"{s}\" height=\"{s}\" fill=\"#ffffff\" fill-opacity=\"0.05\"/>"
                        ));
                    }
                    x += s;
                }
                y += s;
            }
            format!(
                "{}{}{}{}",
                base_fill(w, h, fill),
                cells,
                sheen(w, h),
                vignette(w, h)
            )
        }

        "product" | "oss" | "org" => format!(
            "{base}\
             <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"16\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"#ffffff\" stroke-opacity=\"0.1\"/>\
             {sheen}{vig}",
            base = base_fill(w, h, fill),
            x = wf * 0.72,
            y = hf * 0.22,
            rw = wf * 0.2,
            rh = hf * 0.56,
            sheen = sheen(w, h),
            vig = vignette(w, h),
        ),

        _ => format!(
            "{}{}{}",
            base_fill(w, h, fill),
            sheen(w, h),
            vignette(w, h)
        ),
    };

    wrap(&transforms, body)
}
