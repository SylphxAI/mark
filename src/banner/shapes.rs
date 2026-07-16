//! Banner background shapes.

pub const BANNER_TYPES: &[&str] = &[
    "wave", "waving", "soft", "rounded", "rect", "slice", "cylinder", "blur", "pulse",
    "checkered", "egg", "shark", "venom", "speech", "transparent", "aurora", "mesh",
    "noise", "glass", "grid", "constellation", "terminal", "hud", "circuit", "orbit",
    "ring", "beam", "product", "oss", "org",
];

pub fn is_banner_type(v: &str) -> bool {
    BANNER_TYPES.iter().any(|t| t.eq_ignore_ascii_case(v))
}

pub fn normalize_type(v: &str) -> &'static str {
    BANNER_TYPES
        .iter()
        .find(|t| t.eq_ignore_ascii_case(v))
        .copied()
        .unwrap_or("waving")
}

pub fn shape_defs(ty: &str) -> String {
    if ty == "blur" {
        "<filter id=\"blurf\"><feGaussianBlur stdDeviation=\"24\"/></filter>".into()
    } else {
        String::new()
    }
}

fn wrap(transforms: &[String], inner: String) -> String {
    if transforms.is_empty() {
        inner
    } else {
        format!("<g transform=\"{}\">{inner}</g>", transforms.join(" "))
    }
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
    let rect = || format!("<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>");

    match ty {
        "transparent" => String::new(),
        "rect" => wrap(&transforms, rect()),
        "soft" => {
            let rx = (h / 2).min(40);
            wrap(&transforms, format!("<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"{fill}\"/>"))
        }
        "rounded" => {
            let rx = (h / 3).min(24);
            wrap(&transforms, format!("<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"{fill}\"/>"))
        }
        "cylinder" => {
            let r = (h / 2).min(80);
            wrap(&transforms, format!(
                "<path fill=\"{fill}\" d=\"M{r},0 H{} A{r},{r} 0 0 1 {},{h} H{r} A{r},{r} 0 0 1 {r},0 Z\"/>",
                w - r, w - r
            ))
        }
        "egg" => wrap(&transforms, format!(
            "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"{fill}\"/>",
            wf/2.0, hf/2.0, wf*0.48, hf*0.48
        )),
        "slice" => wrap(&transforms, format!(
            "<polygon fill=\"{fill}\" points=\"0,0 {w},0 {},{h} 0,{h}\"/>", wf*0.85
        )),
        "shark" => wrap(&transforms, format!(
            "<path fill=\"{fill}\" d=\"M0,0 H{w} V{} Q{},{h} {},{} T0,{} Z\"/>",
            hf*0.55, wf*0.7, wf*0.4, hf*0.6, hf*0.75
        )),
        "wave" => wrap(&transforms, format!(
            "<path fill=\"{fill}\" d=\"M0,0 H{w} V{} C{},{} {},{} {},{} C{},{} {},{} 0,{} Z\"/>",
            hf*0.55, wf*0.75, hf*0.85, wf*0.5, hf*0.35, wf*0.25, hf*0.7, wf*0.12, hf*0.9, wf*0.05, hf*0.8, hf*0.65
        )),
        "waving" => {
            let amp = hf * 0.12;
            let y = hf * 0.72;
            let base = wrap(&transforms, rect());
            format!(
                "{base}<path fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.15\" stroke-width=\"3\" d=\"M0,{y} Q{},{},{},{y} T{w},{y}\">\
                 <animate attributeName=\"d\" dur=\"4s\" repeatCount=\"indefinite\" \
                 values=\"M0,{y} Q{},{},{},{y} T{w},{y};M0,{y} Q{},{},{},{y} T{w},{y};M0,{y} Q{},{},{},{y} T{w},{y}\"/></path>",
                wf*0.25, y-amp, wf*0.5,
                wf*0.25, y-amp, wf*0.5,
                wf*0.25, y+amp, wf*0.5,
                wf*0.25, y-amp, wf*0.5,
            )
        }
        "blur" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>\
             <ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"#ffffff\" fill-opacity=\"0.12\" filter=\"url(#blurf)\"/>\
             <ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"#ffffff\" fill-opacity=\"0.1\" filter=\"url(#blurf)\"/>",
            wf*0.3, hf*0.4, wf*0.35, hf*0.5, wf*0.75, hf*0.65, wf*0.3, hf*0.4
        )),
        "pulse" => {
            let r0 = hf * 0.1;
            let r1 = hf * 0.28;
            let base = wrap(&transforms, rect());
            format!(
                "{base}<circle cx=\"{}\" cy=\"{}\" r=\"{r0}\" fill=\"#ffffff\" fill-opacity=\"0.2\">\
                 <animate attributeName=\"r\" values=\"{r0};{r1};{r0}\" dur=\"2.4s\" repeatCount=\"indefinite\"/>\
                 <animate attributeName=\"fill-opacity\" values=\"0.35;0.05;0.35\" dur=\"2.4s\" repeatCount=\"indefinite\"/></circle>",
                wf*0.85, hf*0.5
            )
        }
        "checkered" => {
            let s = (h / 5).max(16);
            let mut cells = String::new();
            let mut y = 0u32;
            while y < h {
                let mut x = 0u32;
                while x < w {
                    if ((x / s) + (y / s)) % 2 == 0 {
                        cells.push_str(&format!(
                            "<rect x=\"{x}\" y=\"{y}\" width=\"{s}\" height=\"{s}\" fill=\"#000000\" fill-opacity=\"0.12\"/>"
                        ));
                    }
                    x += s;
                }
                y += s;
            }
            wrap(&transforms, format!("<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>{cells}"))
        }
        "venom" => wrap(&transforms, format!(
            "<path fill=\"{fill}\" d=\"M0,0 H{w} V{} C{},{} {},{} {},{} C{},{} {},{} {},{} L0,{h} Z\"/>",
            hf*0.4, wf*0.9, hf*0.9, wf*0.7, hf*0.3, wf*0.55, hf*0.75, wf*0.4, hf*1.05, wf*0.25, hf*0.45, wf*0.1, hf*0.8
        )),
        "speech" => {
            let r = 28.0_f32.min(hf * 0.2);
            wrap(&transforms, format!(
                "<path fill=\"{fill}\" d=\"M{r},0 H{} Q{w},0 {w},{r} V{} Q{w},{} {},{} H{} L{},{h} L{},{} H{r} Q0,{} 0,{} V{r} Q0,0 {r},0 Z\"/>",
                wf-r, hf*0.72-r, hf*0.72, wf-r, hf*0.72, wf*0.28, wf*0.18, wf*0.22, hf*0.72, hf*0.72, hf*0.72-r
            ))
        }
        "aurora" => {
            let base = wrap(&transforms, rect());
            format!(
                "{base}<path d=\"M0,{} C{},{} {},{} {},{} S{},{} {w},{} V{h} H0 Z\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>\
                 <path d=\"M0,{} C{},{} {},{} {},{} S{},{} {w},{} V{h} H0 Z\" fill=\"#ffffff\" fill-opacity=\"0.06\"/>",
                hf*0.7, wf*0.2, hf*0.3, wf*0.4, hf*0.9, wf*0.6, hf*0.4, wf*0.9, hf*0.2, hf*0.5,
                hf*0.5, wf*0.25, hf*0.9, wf*0.45, hf*0.1, wf*0.7, hf*0.55, wf*0.95, hf*0.8, hf*0.35
            )
        }
        "mesh" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>\
             <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#ffffff\" fill-opacity=\"0.1\"/>\
             <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#000000\" fill-opacity=\"0.15\"/>\
             <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>",
            wf*0.15, hf*0.2, hf*0.45, wf*0.85, hf*0.8, hf*0.5, wf*0.55, hf*0.35, hf*0.35
        )),
        "noise" => {
            let mut dots = String::new();
            let mut s: u32 = 1;
            for _ in 0..80 {
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let x = s % w.max(1);
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let y = s % h.max(1);
                s = s.wrapping_mul(1103515245).wrapping_add(12345);
                let o = 0.04 + (s % 10) as f32 / 100.0;
                dots.push_str(&format!(
                    "<circle cx=\"{x}\" cy=\"{y}\" r=\"1.2\" fill=\"#ffffff\" fill-opacity=\"{o:.2}\"/>"
                ));
            }
            wrap(&transforms, format!("<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>{dots}"))
        }
        "glass" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>\
             <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"20\" fill=\"#ffffff\" fill-opacity=\"0.1\" stroke=\"#ffffff\" stroke-opacity=\"0.25\"/>",
            wf*0.05, hf*0.12, wf*0.9, hf*0.76
        )),
        "grid" => {
            let step = 28u32;
            let mut lines = String::new();
            let mut x = 0u32;
            while x <= w {
                lines.push_str(&format!(
                    "<line x1=\"{x}\" y1=\"0\" x2=\"{x}\" y2=\"{h}\" stroke=\"#ffffff\" stroke-opacity=\"0.08\"/>"
                ));
                x += step;
            }
            let mut y = 0u32;
            while y <= h {
                lines.push_str(&format!(
                    "<line x1=\"0\" y1=\"{y}\" x2=\"{w}\" y2=\"{y}\" stroke=\"#ffffff\" stroke-opacity=\"0.08\"/>"
                ));
                y += step;
            }
            wrap(&transforms, format!("<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>{lines}"))
        }
        "constellation" => {
            let mut pts = Vec::new();
            let mut s: u32 = 42;
            for _ in 0..18 {
                s = s.wrapping_mul(1664525).wrapping_add(1013904223);
                let x = 40 + s % w.saturating_sub(80).max(1);
                s = s.wrapping_mul(1664525).wrapping_add(1013904223);
                let y = 20 + s % h.saturating_sub(40).max(1);
                pts.push((x, y));
            }
            let mut edges = String::new();
            for i in 0..pts.len().saturating_sub(1) {
                let (x1, y1) = pts[i];
                let (x2, y2) = pts[i + 1];
                edges.push_str(&format!(
                    "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#ffffff\" stroke-opacity=\"0.2\"/>"
                ));
            }
            let stars: String = pts.iter().map(|(x, y)| {
                format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"2.5\" fill=\"#ffffff\" fill-opacity=\"0.85\"/>")
            }).collect();
            wrap(&transforms, format!("<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>{edges}{stars}"))
        }
        "terminal" => {
            let base = wrap(&transforms, format!("<rect width=\"{w}\" height=\"{h}\" rx=\"12\" fill=\"{fill}\"/>"));
            format!(
                "{base}<circle cx=\"28\" cy=\"28\" r=\"7\" fill=\"#FF5F56\"/><circle cx=\"52\" cy=\"28\" r=\"7\" fill=\"#FFBD2E\"/><circle cx=\"76\" cy=\"28\" r=\"7\" fill=\"#27C93F\"/>\
                 <rect x=\"16\" y=\"48\" width=\"{}\" height=\"{}\" rx=\"6\" fill=\"#000000\" fill-opacity=\"0.25\"/>",
                w.saturating_sub(32), h.saturating_sub(64)
            )
        }
        "hud" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>\
             <rect x=\"12\" y=\"12\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.35\" stroke-dasharray=\"6 4\"/>\
             <path d=\"M12,40 H40 M12,12 V40\" stroke=\"#ffffff\" stroke-opacity=\"0.6\" fill=\"none\" stroke-width=\"2\"/>\
             <path d=\"M{},40 H{} M{},12 V40\" stroke=\"#ffffff\" stroke-opacity=\"0.6\" fill=\"none\" stroke-width=\"2\"/>\
             <path d=\"M12,{} H40 M12,{} V{}\" stroke=\"#ffffff\" stroke-opacity=\"0.6\" fill=\"none\" stroke-width=\"2\"/>\
             <path d=\"M{},{} H{} M{},{} V{}\" stroke=\"#ffffff\" stroke-opacity=\"0.6\" fill=\"none\" stroke-width=\"2\"/>",
            w.saturating_sub(24), h.saturating_sub(24),
            w-12, w-40, w-12,
            h-40, h-12, h-40,
            w-12, h-40, w-40, w-12, h-12, h-40
        )),
        "circuit" => {
            let mut traces = String::new();
            for i in 0..8 {
                let y = 20.0 + i as f32 * ((hf - 40.0) / 7.0);
                let mid = wf * (0.2 + (i % 5) as f32 * 0.12);
                traces.push_str(&format!(
                    "<path d=\"M0,{y} H{mid} V{} H{w}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.18\" stroke-width=\"1.5\"/>\
                     <circle cx=\"{mid}\" cy=\"{y}\" r=\"3\" fill=\"#ffffff\" fill-opacity=\"0.45\"/>",
                    y + 18.0
                ));
            }
            wrap(&transforms, format!("<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>{traces}"))
        }
        "orbit" => {
            let cx = wf * 0.78;
            let cy = hf * 0.5;
            let base = wrap(&transforms, rect());
            format!(
                "{base}<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{}\" ry=\"{}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.25\"/>\
                 <ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{}\" ry=\"{}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\"/>\
                 <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{}\" fill=\"#ffffff\" fill-opacity=\"0.35\"/>\
                 <circle cx=\"{}\" cy=\"{cy}\" r=\"4\" fill=\"#ffffff\">\
                 <animateTransform attributeName=\"transform\" type=\"rotate\" from=\"0 {cx} {cy}\" to=\"360 {cx} {cy}\" dur=\"8s\" repeatCount=\"indefinite\"/></circle>",
                hf*0.42, hf*0.28, hf*0.28, hf*0.18, hf*0.08, cx + hf*0.42
            )
        }
        "ring" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>\
             <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\" stroke-width=\"10\"/>\
             <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.15\" stroke-width=\"4\"/>",
            wf*0.82, hf*0.5, hf*0.32, wf*0.82, hf*0.5, hf*0.2
        )),
        "beam" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>\
             <polygon points=\"0,{h} {},0 {},0 {w},{h}\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>\
             <polygon points=\"{},{h} {},{} {},{} {},{h}\" fill=\"#ffffff\" fill-opacity=\"0.06\"/>",
            wf*0.45, wf*0.55, wf*0.2, wf*0.48, hf*0.2, wf*0.52, hf*0.2, wf*0.8
        )),
        "product" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" rx=\"16\" fill=\"{fill}\"/>\
             <rect x=\"20\" y=\"20\" width=\"{}\" height=\"{}\" rx=\"12\" fill=\"#ffffff\" fill-opacity=\"0.12\"/>",
            (wf*0.2).min(120.0), h.saturating_sub(40)
        )),
        "oss" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>\
             <path transform=\"translate({},{}) scale(0.09)\" fill=\"#ffffff\" fill-opacity=\"0.2\" d=\"M512 189l128 256 282 41-204 199 48 280-254-134-254 134 48-280L102 486l282-41z\"/>",
            w.saturating_sub(90), h/2 - 28
        )),
        "org" => wrap(&transforms, format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"{fill}\"/>\
             <rect x=\"{}\" y=\"{}\" width=\"100\" height=\"100\" rx=\"24\" fill=\"#ffffff\" fill-opacity=\"0.12\"/>\
             <rect x=\"{}\" y=\"{}\" width=\"50\" height=\"50\" rx=\"10\" fill=\"#ffffff\" fill-opacity=\"0.2\"/>",
            w.saturating_sub(140), hf*0.22, w.saturating_sub(115), hf*0.38
        )),
        _ => wrap(&transforms, rect()),
    }
}
