//! Banner motion — SMIL-first so animations work when the SVG is loaded as `<img>`.
//!
//! CSS `@keyframes` often do nothing for external SVG images; SMIL (`<animate*>`) does.

/// Catalog exported to API / studio (order = UI order).
pub const ANIMATIONS: &[&str] = &[
    "none", "ambient", "fade", "rise", "scale", "float", "glow", "breathe", "slide", "cascade",
    "shimmer", "glitch", "wave", "orbit", "neon", "bounce", "type",
];

/// Legacy aliases accepted for compatibility.
pub fn normalize_animation(raw: Option<&str>) -> &'static str {
    match raw.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        None => "ambient",
        Some(s) => match s.to_ascii_lowercase().as_str() {
            "0" | "false" | "off" | "static" | "none" => "none",
            "ambient" | "bg" | "idle" => "ambient",
            "fade" | "fadein" => "fade",
            "rise" | "up" | "lift" => "rise",
            "scale" | "scalein" | "pop" => "scale",
            "float" | "hover" => "float",
            "glow" | "twinkling" | "twinkle" => "glow",
            "breathe" | "breath" | "pulse" | "blinking" | "blink" => "breathe",
            "slide" | "swipe" => "slide",
            "cascade" | "stagger" => "cascade",
            "shimmer" | "shine" => "shimmer",
            "glitch" | "jitter" => "glitch",
            "wave" | "waving" => "wave",
            "orbit" | "spin" => "orbit",
            "neon" | "flicker" | "cyber" => "neon",
            "bounce" | "spring" | "elastic" => "bounce",
            "type" | "typewriter" | "reveal" => "type",
            _ => "ambient",
        },
    }
}

/// Background motion intensity 0.0–1.0 (none freezes decorative layers).
pub fn ambient_gain(anim: &str) -> f32 {
    match anim {
        "none" => 0.0,
        "ambient" => 0.9,
        "wave" | "orbit" => 1.0,
        "glitch" => 0.6,
        _ => 0.95,
    }
}

/// Extra opening attributes (no trailing `>`) for a text node.
pub fn text_open_attrs(anim: &str, line_index: usize, width: u32, height: u32) -> String {
    let delay = line_index as f32 * 0.12;
    match anim {
        "fade" => " opacity=\"0\"".into(),
        "rise" => {
            let dy = (height as f32 * 0.12).clamp(14.0, 36.0);
            format!(" opacity=\"0\" transform=\"translate(0 {dy})\"")
        }
        "scale" => {
            let cx = width as f32 / 2.0;
            let cy = height as f32 * 0.44;
            format!(
                " opacity=\"0\" transform=\"translate({cx} {cy}) scale(0.84) translate({}, {})\"",
                -cx, -cy
            )
        }
        "glow" | "neon" => " opacity=\"0.7\"".into(),
        "shimmer" => " opacity=\"0.55\"".into(),
        "bounce" => {
            let dy = -(height as f32 * 0.1).clamp(12.0, 28.0);
            format!(" opacity=\"0\" transform=\"translate(0 {dy})\"")
        }
        "type" => " opacity=\"0\"".into(),
        "slide" => {
            let dx = -(width as f32 * 0.09).clamp(24.0, 72.0);
            format!(" opacity=\"0\" transform=\"translate({dx} 0)\"")
        }
        "cascade" => {
            let dy = 18.0 + line_index as f32 * 4.0;
            format!(" opacity=\"0\" transform=\"translate(0 {dy})\" data-d=\"{delay}\"")
        }
        _ => String::new(),
    }
}

/// SMIL children placed inside a text element.
pub fn text_children(anim: &str, line_index: usize, width: u32, height: u32) -> String {
    let delay = line_index as f32 * 0.12;
    match anim {
        "none" | "ambient" => String::new(),

        "fade" => format!(
            "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"1.05s\" begin=\"{delay}s\" fill=\"freeze\" \
             calcMode=\"spline\" keySplines=\"0.22 1 0.36 1\" keyTimes=\"0;1\"/>"
        ),

        "rise" => {
            let dy = (height as f32 * 0.12).clamp(14.0, 36.0);
            format!(
                "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.95s\" begin=\"{delay}s\" fill=\"freeze\" \
                   calcMode=\"spline\" keySplines=\"0.22 1 0.36 1\" keyTimes=\"0;1\"/>\
                 <animateTransform attributeName=\"transform\" type=\"translate\" from=\"0 {dy}\" to=\"0 0\" \
                   dur=\"1.05s\" begin=\"{delay}s\" fill=\"freeze\" calcMode=\"spline\" keySplines=\"0.16 1 0.3 1\" keyTimes=\"0;1\"/>"
            )
        }

        "scale" => format!(
            "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.75s\" begin=\"{delay}s\" fill=\"freeze\"/>\
             <animateTransform attributeName=\"transform\" type=\"scale\" from=\"0.84\" to=\"1\" \
               dur=\"0.9s\" begin=\"{delay}s\" fill=\"freeze\" additive=\"sum\" \
               calcMode=\"spline\" keySplines=\"0.34 1.45 0.64 1\" keyTimes=\"0;1\"/>"
        ),

        "float" => {
            let amp = (height as f32 * 0.028).clamp(3.5, 9.0);
            format!(
                "<animateTransform attributeName=\"transform\" type=\"translate\" \
                   values=\"0 0; 0 -{amp}; 0 0; 0 {half}; 0 0\" keyTimes=\"0;0.25;0.5;0.75;1\" \
                   dur=\"5.2s\" begin=\"{delay}s\" repeatCount=\"indefinite\" \
                   calcMode=\"spline\" keySplines=\"0.45 0 0.55 1;0.45 0 0.55 1;0.45 0 0.55 1;0.45 0 0.55 1\"/>",
                half = amp * 0.55
            )
        }

        "glow" => format!(
            "<animate attributeName=\"opacity\" values=\"0.7;1;0.78;1;0.7\" keyTimes=\"0;0.28;0.52;0.78;1\" \
               dur=\"3.1s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>"
        ),

        "breathe" => {
            let cx = width as f32 / 2.0;
            let cy = height as f32 * 0.44;
            // scale around center via nested group is cleaner; approximate with translate+scale sum
            format!(
                "<animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; 0 -1.5; 0 0\" \
                   dur=\"3.4s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>\
                 <animate attributeName=\"opacity\" values=\"0.92;1;0.92\" dur=\"3.4s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>\
                 <!-- center bias {cx},{cy} -->"
            )
        }

        "slide" => {
            let dx = -(width as f32 * 0.09).clamp(24.0, 72.0);
            format!(
                "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.85s\" begin=\"{delay}s\" fill=\"freeze\"/>\
                 <animateTransform attributeName=\"transform\" type=\"translate\" from=\"{dx} 0\" to=\"0 0\" \
                   dur=\"0.95s\" begin=\"{delay}s\" fill=\"freeze\" calcMode=\"spline\" keySplines=\"0.16 1 0.3 1\" keyTimes=\"0;1\"/>"
            )
        }

        "cascade" => {
            let dy = 18.0 + line_index as f32 * 4.0;
            let d = delay + 0.04;
            format!(
                "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.7s\" begin=\"{d}s\" fill=\"freeze\"/>\
                 <animateTransform attributeName=\"transform\" type=\"translate\" from=\"0 {dy}\" to=\"0 0\" \
                   dur=\"0.85s\" begin=\"{d}s\" fill=\"freeze\" calcMode=\"spline\" keySplines=\"0.22 1 0.36 1\" keyTimes=\"0;1\"/>"
            )
        }

        "shimmer" => format!(
            "<animate attributeName=\"opacity\" values=\"0.55;1;0.68;1;0.55\" keyTimes=\"0;0.22;0.48;0.72;1\" \
               dur=\"2.5s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>\
             <animateTransform attributeName=\"transform\" type=\"translate\" \
               values=\"0 0; 1.8 0; 0 0; -1.2 0; 0 0\" dur=\"2.5s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>"
        ),

        "glitch" => format!(
            "<animateTransform attributeName=\"transform\" type=\"translate\" \
               values=\"0 0; 2.5 0; -2 0; 1.5 -1; -1.5 1; 0 0; 0 0; 3 0; -2.5 0; 0 0\" \
               keyTimes=\"0;0.07;0.11;0.15;0.19;0.26;0.68;0.76;0.84;1\" \
               dur=\"2.7s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>\
             <animate attributeName=\"opacity\" values=\"1;1;0.5;1;1;0.65;1\" \
               keyTimes=\"0;0.08;0.12;0.16;0.72;0.78;1\" dur=\"2.7s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>"
        ),

        "wave" => {
            let amp = (height as f32 * 0.032).clamp(4.0, 11.0);
            format!(
                "<animateTransform attributeName=\"transform\" type=\"translate\" \
                   values=\"0 0; 0 -{amp}; 0 0; 0 {amp}; 0 0\" keyTimes=\"0;0.25;0.5;0.75;1\" \
                   dur=\"3.8s\" begin=\"{delay}s\" repeatCount=\"indefinite\" \
                   calcMode=\"spline\" keySplines=\"0.45 0 0.55 1;0.45 0 0.55 1;0.45 0 0.55 1;0.45 0 0.55 1\"/>"
            )
        }

        "orbit" => format!(
            "<animateTransform attributeName=\"transform\" type=\"translate\" \
               values=\"0 0; 3.5 -2; 0 -3.5; -3.5 -1; 0 0\" keyTimes=\"0;0.25;0.5;0.75;1\" \
               dur=\"6.5s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>"
        ),

        // Neon flicker — cyber/signage pulse with occasional hard cut.
        "neon" => format!(
            "<animate attributeName=\"opacity\" \
               values=\"0.55;1;0.92;1;0.35;1;0.88;1;0.55\" \
               keyTimes=\"0;0.12;0.28;0.42;0.48;0.55;0.72;0.88;1\" \
               dur=\"2.4s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>\
             <animateTransform attributeName=\"transform\" type=\"translate\" \
               values=\"0 0; 0.6 0; 0 0; -0.8 0; 0 0; 0 0\" \
               keyTimes=\"0;0.45;0.5;0.55;0.6;1\" \
               dur=\"2.4s\" begin=\"{delay}s\" repeatCount=\"indefinite\"/>"
        ),

        // Spring bounce entry then soft settle.
        "bounce" => {
            let dy = -(height as f32 * 0.1).clamp(12.0, 28.0);
            format!(
                "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.35s\" begin=\"{delay}s\" fill=\"freeze\"/>\
                 <animateTransform attributeName=\"transform\" type=\"translate\" \
                   values=\"0 {dy}; 0 6; 0 -3; 0 1.5; 0 0\" keyTimes=\"0;0.4;0.62;0.82;1\" \
                   dur=\"0.95s\" begin=\"{delay}s\" fill=\"freeze\" \
                   calcMode=\"spline\" keySplines=\"0.22 1.4 0.36 1;0.34 1.2 0.64 1;0.34 1.1 0.64 1;0.25 1 0.5 1\"/>"
            )
        },

        // Staggered typewriter / line reveal (per-line delay).
        "type" => {
            let d = delay * 1.35 + line_index as f32 * 0.08;
            format!(
                "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"0.08s\" begin=\"{d}s\" fill=\"freeze\"/>\
                 <animateTransform attributeName=\"transform\" type=\"translate\" \
                   from=\"0 8\" to=\"0 0\" dur=\"0.28s\" begin=\"{d}s\" fill=\"freeze\" \
                   calcMode=\"spline\" keySplines=\"0.16 1 0.3 1\" keyTimes=\"0;1\"/>"
            )
        }

        _ => String::new(),
    }
}
