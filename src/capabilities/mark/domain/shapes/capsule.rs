//! Capsule silhouettes: the flat banner shapes of the capsule-render dialect.
//!
//! A capsule-render URL (`/api?type=waving…`) is embedded above or below
//! README prose, so its silhouette (the wave edge, the egg arches, the shark
//! teeth) is what makes the banner sit in the page. Mark's own art types are
//! full-bleed fields; these are the outlines a ported URL needs to look the
//! same after a host swap. Geometry follows capsule-render (MIT, © 2020
//! Ye-Chan Kang) on its fixed 854-wide canvas; paint is a validated fill.
//!
//! Every animated shape is SMIL, so it moves inside GitHub's `<img>` proxy.

use crate::capabilities::mark::domain::svg::normalize_hex_token;

/// One capsule-render `type=`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Silhouette {
    Wave,
    Waving,
    Egg,
    Shark,
    Slice,
    Rect,
    Soft,
    Rounded,
    Cylinder,
    Venom,
    Speech,
    Transparent,
    Blur,
    Pulse,
    Checkered,
}

impl Silhouette {
    /// capsule-render's rule: lowercase, letters only; unknown or missing
    /// input is its default `wave` (Mark never answers an error page).
    pub(crate) fn parse(raw: Option<&str>) -> Self {
        let id: String = raw
            .unwrap_or("")
            .chars()
            .filter(char::is_ascii_alphabetic)
            .map(|c| c.to_ascii_lowercase())
            .collect();
        match id.as_str() {
            "waving" => Self::Waving,
            "egg" => Self::Egg,
            "shark" => Self::Shark,
            "slice" => Self::Slice,
            "rect" => Self::Rect,
            "soft" => Self::Soft,
            "rounded" => Self::Rounded,
            "cylinder" => Self::Cylinder,
            "venom" => Self::Venom,
            "speech" => Self::Speech,
            "transparent" => Self::Transparent,
            "blur" => Self::Blur,
            "pulse" => Self::Pulse,
            "checkered" => Self::Checkered,
            _ => Self::Wave,
        }
    }
}

/// The canvas width every capsule-render banner uses.
pub(crate) const CAPSULE_WIDTH: u32 = 854;

/// Solid or horizontal-gradient paint from validated stops.
///
/// Returns `(defs, fill)`. Stops are `(offset %, color)`; colors that are not
/// canonical hex tokens are dropped, and an empty list paints capsule-render's
/// default lavender.
pub(crate) fn capsule_paint(stops: &[(f32, String)]) -> (String, String) {
    let valid: Vec<(f32, String)> = stops
        .iter()
        .filter(|(o, _)| o.is_finite())
        .filter_map(|(o, c)| normalize_hex_token(c).map(|c| (o.clamp(0.0, 100.0), c)))
        .collect();
    match valid.as_slice() {
        [] => (String::new(), "#B897FF".into()),
        [(_, only)] => (String::new(), only.clone()),
        many => {
            let stops: String = many
                .iter()
                .map(|(o, c)| format!("<stop offset=\"{o}%\" stop-color=\"{c}\"/>"))
                .collect();
            (
                format!("<linearGradient id=\"capsuleLinear\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">{stops}</linearGradient>"),
                "url(#capsuleLinear)".into(),
            )
        }
    }
}

/// Paint one silhouette. `flip` is `section=footer` (rotate 180° about the
/// canvas center); `mirror` is `reversal=true` (horizontal mirror). As in the
/// browsers capsule-render users see, the footer rotation replaces the mirror.
pub(crate) fn silhouette(
    shape: Silhouette,
    w: u32,
    h: u32,
    fill: &str,
    flip: bool,
    mirror: bool,
) -> String {
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let transform = if flip {
        format!(" transform=\"rotate(180 {cx} {cy})\"")
    } else if mirror {
        format!(" transform=\"translate({w} 0) scale(-1 1)\"")
    } else {
        String::new()
    };
    let body = match shape {
        Silhouette::Transparent => return String::new(),
        Silhouette::Waving => waving(w, h, fill),
        Silhouette::Venom => venom(w, h, fill),
        Silhouette::Blur => blur(h, fill),
        Silhouette::Pulse => pulse(w, h, fill),
        Silhouette::Checkered => checkered(h, fill),
        Silhouette::Soft => {
            format!("<rect rx=\"4.5\" height=\"{h}\" width=\"{w}\" fill=\"{fill}\"/>")
        }
        path_shape => format!(
            "<path fill=\"{fill}\" d=\"{}\"/>",
            outline(path_shape, w, h)
        ),
    };
    format!("<g{transform}>{body}</g>")
}

/// Static outlines (capsule-render's normal models).
fn outline(shape: Silhouette, w: u32, h: u32) -> String {
    let (wf, hf) = (w as f32, h as f32);
    match shape {
        Silhouette::Egg => {
            let t = hf / 3.0;
            format!(
                "M 427 {hf} Q 327 {t} 213.5 {hf} Q 100 {t} 0 {hf} Q 50 0 427 0 Q 750 0 {wf} {hf} \
                 Q 727 {t} 639.5 {hf} Q 527 {t} 427 {hf}"
            )
        }
        Silhouette::Shark => {
            let (c1, c2) = (hf - 95.0, hf + 27.0);
            let mut d = format!("M 0 {c1}");
            for i in 0..7 {
                let x0 = i as f32 * 122.0;
                let end = if i == 6 { wf } else { x0 + 122.0 };
                d.push_str(&format!(" C {m} {c2} {m} {c2} {end} {c1}", m = x0 + 61.0));
            }
            d.push_str(&format!(" L {wf} 0 L 0 0 Z"));
            d
        }
        Silhouette::Slice => format!("m 0 0 l {wf} {hf} l 0 -{hf} l -{wf} 0 z"),
        Silhouette::Rounded => {
            let q = hf - 61.0;
            format!(
                "M 61 0 L 793 0 Q {wf} 0 {wf} 61 L {wf} {q} Q {wf} {hf} 793 {hf} L 61 {hf} \
                 Q 0 {hf} 0 {q} L 0 61 Q 0 0 61 0 z"
            )
        }
        Silhouette::Cylinder => {
            let r = hf / 2.0;
            format!(
                "M {r} 0 L {e} 0 A {r} {r} 0 1 1 {e} {hf} L {r} {hf} A {r} {r} 0 1 1 {r} 0 Z",
                e = wf - r
            )
        }
        Silhouette::Speech => {
            let tail = (hf * 0.2).min(40.0);
            let at = wf - 100.0;
            let (b, c) = (hf - tail, hf - tail - 30.0);
            format!(
                "M 30 0 H {r30} C {r15} 0, {wf} 15, {wf} 30 V {c} C {wf} {c15}, {r15} {b}, {r30} {b} \
                 H {t1} L {at} {hf} L {t0} {b} H 30 C 15 {b}, 0 {c15}, 0 {c} V 30 C 0 15, 15 0, 30 0 Z",
                r30 = wf - 30.0,
                r15 = wf - 15.0,
                c15 = hf - tail - 15.0,
                t1 = at + tail,
                t0 = at - tail,
            )
        }
        Silhouette::Wave => {
            let o = hf - 120.0;
            format!(
                "M 0 0 L 0 {a} Q 110 {b} 220 {c} T 440 {d} T 660 {e} T 880 {f} L 880 0 Z",
                a = 70.0 + o,
                b = -55.0 + o,
                c = 55.0 + o,
                d = 60.0 + o,
                e = 50.0 + o,
                f = 75.0 + o,
            )
        }
        // Rect, and the animated shapes painted by their own functions.
        _ => format!("M 0 0 L 0 {hf} L {wf} {hf} L {wf} 0 Z"),
    }
}

/// Two translucent waves drifting over 20 s (capsule-render `waving`).
fn waving(w: u32, h: u32, fill: &str) -> String {
    let (wf, hf) = (w as f32, h as f32);
    let frame = |a: f32, b: f32, c: f32, d: f32| {
        format!(
            "M0 0L 0 {}Q 213.5 {} 427 {}T {wf} {}L {wf} 0 Z",
            hf - a,
            hf - b,
            hf - c,
            hf - d
        )
    };
    let layer = |frames: [String; 3], begin: &str| {
        let values = format!("{};{};{};{}", frames[0], frames[1], frames[2], frames[0]);
        format!(
            "<path d=\"{first}\" fill=\"{fill}\" opacity=\"0.4\">\
             <animate attributeName=\"d\" dur=\"20s\" repeatCount=\"indefinite\" keyTimes=\"0;0.333;0.667;1\" \
             calcMode=\"spline\" keySplines=\"0.2 0 0.2 1;0.2 0 0.2 1;0.2 0 0.2 1\" begin=\"{begin}\" values=\"{values}\"/></path>",
            first = frames[0],
        )
    };
    let front = [
        frame(80.0, 40.0, 70.0, 45.0),
        frame(55.0, 40.0, 60.0, 70.0),
        frame(35.0, 65.0, 35.0, 70.0),
    ];
    let back = [
        frame(65.0, 20.0, 50.0, 40.0),
        frame(50.0, 80.0, 80.0, 60.0),
        frame(55.0, 75.0, 50.0, 35.0),
    ];
    format!("{}{}", layer(front, "0s"), layer(back, "-10s"))
}

/// A morphing blob at the canvas center (capsule-render `venom`).
fn venom(w: u32, h: u32, fill: &str) -> String {
    const FRAMES: [&str; 3] = [
        "M52.8,-67C68,-61.6,79.7,-45.6,82.5,-28.7C85.4,-11.8,79.5,6,71.4,20.8C63.2,35.5,53,47.1,40.7,53.3C28.5,59.4,14.2,60,-2.1,63C-18.5,65.9,-37,71.2,-52.3,66C-67.6,60.9,-79.7,45.4,-81.5,29C-83.2,12.7,-74.6,-4.4,-68.7,-22.2C-62.8,-40.1,-59.5,-58.7,-48.4,-65.4C-37.4,-72.2,-18.7,-67.1,0,-67.1C18.8,-67.2,37.6,-72.4,52.8,-67Z",
        "M42.3,-55.2C55.2,-48.7,66.6,-37,69.3,-23.7C72,-10.4,66,4.6,58.3,16.1C50.6,27.6,41.2,35.6,31.1,43.9C21.1,52.1,10.6,60.6,-1.3,62.5C-13.2,64.3,-26.4,59.4,-37.6,51.5C-48.8,43.6,-57.9,32.7,-61,20.5C-64.1,8.3,-61.1,-5.3,-57.9,-19.9C-54.7,-34.6,-51.3,-50.3,-41.6,-57.8C-32,-65.3,-16,-64.7,-0.7,-63.7C14.6,-62.8,29.3,-61.6,42.3,-55.2Z",
        "M49.2,-63.6C62.2,-58.3,70.1,-41.8,74.6,-24.9C79.1,-8.1,80.3,9.2,76.5,26.2C72.8,43.2,64.2,59.9,50.7,68.4C37.1,76.9,18.5,77.2,2.3,74.1C-13.9,70.9,-27.9,64.2,-41.2,55.6C-54.6,47.1,-67.4,36.6,-74.5,22.4C-81.7,8.2,-83.2,-9.7,-74.7,-20.8C-66.3,-31.9,-47.9,-36.3,-33.8,-41.2C-19.7,-46.2,-9.8,-51.8,4.1,-57.5C18.1,-63.1,36.2,-68.9,49.2,-63.6Z",
    ];
    format!(
        "<g transform=\"translate({} {})\"><path fill=\"{fill}\" d=\"{a}\">\
         <animate attributeName=\"d\" dur=\"10s\" repeatCount=\"indefinite\" values=\"{a};{b};{c};{a}\"/></path></g>",
        w as f32 / 2.0,
        h as f32 / 2.0,
        a = FRAMES[0],
        b = FRAMES[1],
        c = FRAMES[2],
    )
}

/// Three soft orbs drifting at the center (capsule-render `blur`).
fn blur(h: u32, fill: &str) -> String {
    let hf = h as f32;
    let orbs = [
        (0.25, 0.0, 10.0, 0.2, 0.0),
        (0.3, 5.0, 30.0, 0.3, 1.0),
        (0.28, -10.0, 15.0, 0.15, 2.0),
    ];
    let mut out = String::from(
        "<defs><filter id=\"capsuleBlur\"><feGaussianBlur stdDeviation=\"12\"/></filter></defs>",
    );
    for (r, dx, dur, opacity, begin) in orbs {
        let spline = "0.4 0 0.6 1;0.4 0 0.6 1;0.4 0 0.6 1;0.4 0 0.6 1";
        out.push_str(&format!(
            "<circle cx=\"{a}%\" cy=\"50%\" r=\"{rad}\" fill=\"{fill}\" opacity=\"{opacity}\" filter=\"url(#capsuleBlur)\">\
             <animate attributeName=\"cx\" values=\"{a}%;50%;{b}%;50%;{a}%\" dur=\"{dur}s\" begin=\"{begin}s\" \
             repeatCount=\"indefinite\" calcMode=\"spline\" keySplines=\"{spline}\"/></circle>",
            a = 50.0 - dx,
            b = 50.0 + dx,
            rad = hf * r,
        ));
    }
    out
}

/// A breathing dot at the center (capsule-render `pulse`).
fn pulse(w: u32, h: u32, fill: &str) -> String {
    let hf = h as f32;
    let size = (hf * 0.16).max(18.0);
    let glow = (hf * 0.28).max(30.0);
    format!(
        "<g transform=\"translate({cx} {cy})\">\
         <circle r=\"{glow}\" fill=\"{fill}\" opacity=\"0.08\">\
         <animate attributeName=\"r\" values=\"{g0};{g1};{g0}\" dur=\"4.5s\" repeatCount=\"indefinite\"/>\
         <animate attributeName=\"opacity\" values=\"0.05;0.12;0.05\" dur=\"4.5s\" repeatCount=\"indefinite\"/></circle>\
         <circle r=\"{size}\" fill=\"{fill}\" opacity=\"0.28\">\
         <animateTransform attributeName=\"transform\" type=\"scale\" values=\"1;1.2;1\" dur=\"4.5s\" repeatCount=\"indefinite\"/></circle>\
         <circle r=\"{core}\" fill=\"#ffffff\" opacity=\"0.45\">\
         <animate attributeName=\"opacity\" values=\"0.28;0.55;0.28\" dur=\"4.5s\" repeatCount=\"indefinite\"/></circle></g>",
        cx = w as f32 / 2.0,
        cy = hf / 2.0,
        g0 = glow * 0.85,
        g1 = glow * 1.1,
        core = size * 0.4,
    )
}

/// A slow-breathing checker over the paint (capsule-render `checkered`).
fn checkered(h: u32, fill: &str) -> String {
    let tile = (h as f32 * 0.2).round().max(28.0);
    let cell = tile / 2.0;
    let cells: String = [
        (0.0, 0.0, "#ffffff", 0.06, 0),
        (cell, 0.0, "#000000", 0.05, 2),
        (0.0, cell, "#000000", 0.05, 4),
        (cell, cell, "#ffffff", 0.06, 6),
    ]
    .iter()
    .map(|(x, y, c, o, begin)| {
        format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{cell}\" height=\"{cell}\" fill=\"{c}\" opacity=\"{o}\">\
             <animate attributeName=\"opacity\" values=\"{lo};{o};{hi};{o};{lo}\" keyTimes=\"0;0.2;0.5;0.8;1\" \
             dur=\"16s\" begin=\"{begin}s\" repeatCount=\"indefinite\"/></rect>",
            lo = o * 0.7,
            hi = o * 1.15,
        )
    })
    .collect();
    format!(
        "<defs><pattern id=\"capsuleChecker\" width=\"{tile}\" height=\"{tile}\" patternUnits=\"userSpaceOnUse\">{cells}</pattern></defs>\
         <rect width=\"100%\" height=\"100%\" fill=\"{fill}\"/><rect width=\"100%\" height=\"100%\" fill=\"url(#capsuleChecker)\"/>"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_follows_capsule_rules() {
        assert_eq!(Silhouette::parse(Some("Waving")), Silhouette::Waving);
        assert_eq!(Silhouette::parse(Some("egg!")), Silhouette::Egg);
        assert_eq!(Silhouette::parse(Some("aurora")), Silhouette::Wave);
        assert_eq!(Silhouette::parse(None), Silhouette::Wave);
    }

    #[test]
    fn paint_is_validated() {
        assert_eq!(capsule_paint(&[]).1, "#B897FF");
        assert_eq!(capsule_paint(&[(0.0, "fff".into())]).1, "#ffffff");
        let (defs, fill) = capsule_paint(&[(0.0, "EEFF00".into()), (100.0, "a82da8".into())]);
        assert_eq!(fill, "url(#capsuleLinear)");
        assert!(defs.contains("offset=\"100%\" stop-color=\"#a82da8\""));
        let (_, fill) = capsule_paint(&[(0.0, "\"onload=x".into())]);
        assert_eq!(fill, "#B897FF");
    }

    #[test]
    fn footer_rotates_and_reversal_mirrors() {
        let footer = silhouette(Silhouette::Wave, 854, 120, "#000", true, true);
        assert!(footer.contains("rotate(180 427 60)"));
        assert!(!footer.contains("scale(-1 1)"));
        let mirrored = silhouette(Silhouette::Wave, 854, 120, "#000", false, true);
        assert!(mirrored.contains("scale(-1 1)"));
    }
}
