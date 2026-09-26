//! Motion — a few quiet, SMIL-first animations.
//!
//! SMIL (`<animate*>`) runs when an SVG is loaded as `<img>`; CSS keyframes
//! often do not. Text motion plays once and settles; only the background
//! drifts, slowly, so a README never has something blinking at its reader.

/// Published animation ids, in studio order.
///
/// - `ambient` (default): the background drifts, the text is still.
/// - `fade` / `rise`: the text fades (and rises) in once, then rests.
/// - `type`: the title is revealed left to right, then rests.
/// - `none`: a still image.
pub(crate) const ANIMATIONS: &[&str] = &["ambient", "fade", "rise", "type", "none"];

/// Normalize `animation=` against [`ANIMATIONS`].
///
/// Ids published before the curated set map to the closest survivor: entry
/// motions become `rise`, looping text effects become `ambient` (still text).
/// Anything else is unknown input and renders `ambient`.
pub(crate) fn normalize_animation(raw: Option<&str>) -> &'static str {
    let Some(s) = raw.map(|s| s.trim().to_ascii_lowercase()) else {
        return "ambient";
    };
    if let Some(a) = ANIMATIONS.iter().find(|a| **a == s) {
        return a;
    }
    match s.as_str() {
        "scale" | "slide" | "cascade" | "bounce" => "rise",
        _ => "ambient",
    }
}

/// Whether decorative background layers move.
pub(crate) fn background_moves(anim: &str) -> bool {
    anim != "none"
}

/// Soft deceleration (ease-out-quint-ish) used by every entry.
const EASE_OUT: &str = "0.16 1 0.3 1";

/// Opening attributes (no trailing `>`) for a text node with entry motion.
pub(crate) fn text_open_attrs(anim: &str, _line: usize, _width: u32, height: u32) -> String {
    match anim {
        "fade" | "blink" => " opacity=\"0\"".into(),
        "rise" => format!(
            " opacity=\"0\" transform=\"translate(0 {})\"",
            rise_distance(height)
        ),
        _ => String::new(),
    }
}

/// SMIL children placed inside a text node. Lines enter 90 ms apart.
pub(crate) fn text_children(anim: &str, line: usize, _width: u32, height: u32) -> String {
    let delay = 0.15 + line as f32 * 0.09;
    let fade = |dur: f32| {
        format!(
            "<animate attributeName=\"opacity\" from=\"0\" to=\"1\" dur=\"{dur}s\" begin=\"{delay:.2}s\" \
             fill=\"freeze\" calcMode=\"spline\" keyTimes=\"0;1\" keySplines=\"{EASE_OUT}\"/>"
        )
    };
    match anim {
        "fade" => fade(1.1),
        "rise" => format!(
            "{}<animateTransform attributeName=\"transform\" type=\"translate\" from=\"0 {}\" to=\"0 0\" \
             dur=\"1.1s\" begin=\"{delay:.2}s\" fill=\"freeze\" calcMode=\"spline\" keyTimes=\"0;1\" \
             keySplines=\"{EASE_OUT}\"/>",
            fade(0.9),
            rise_distance(height)
        ),
        other => dialect_children(other),
    }
}

fn rise_distance(height: u32) -> f32 {
    (height as f32 * 0.05).clamp(6.0, 16.0).round()
}

/// Dialect-only motion (capsule-render `blink`/`blinking`/`twinkling`):
/// reachable through the capsule dialect's overrides, never `animation=`.
fn dialect_children(anim: &str) -> String {
    match anim {
        "blink" => "<animate attributeName=\"opacity\" values=\"1;0;1;0;1\" keyTimes=\"0;0.1;0.25;0.4;0.7\" \
             calcMode=\"discrete\" dur=\"0.6s\" begin=\"0s\" fill=\"freeze\"/>"
            .into(),
        "blinking" => "<animate attributeName=\"opacity\" values=\"1;0;1\" keyTimes=\"0;0.2;0.5\" \
             calcMode=\"discrete\" dur=\"1.6s\" begin=\"0s\" repeatCount=\"indefinite\"/>"
            .into(),
        "twinkling" => "<animate attributeName=\"opacity\" values=\"1;1;0.5;1;0.5;1;1\" \
             keyTimes=\"0;0.4;0.5;0.6;0.7;0.8;1\" dur=\"4s\" begin=\"0s\" repeatCount=\"indefinite\"/>"
            .into(),
        _ => String::new(),
    }
}

/// Wrap a row/group (strip icons) with the same text-level entry motion.
/// Empty pair means the group is still.
pub(crate) fn group_wrap(anim: &str, index: usize, width: u32, height: u32) -> (String, String) {
    let attrs = text_open_attrs(anim, index, width, height);
    let children = text_children(anim, index, width, height);
    if attrs.is_empty() && children.is_empty() {
        (String::new(), String::new())
    } else {
        (format!("<g{attrs}>{children}"), "</g>".into())
    }
}

/// A slow looping drift for a background layer: `translate` through the
/// given offsets and back, eased, over `dur` seconds.
pub(crate) fn drift(points: &[(f32, f32)], dur: f32) -> String {
    let mut values: Vec<String> = points
        .iter()
        .map(|(x, y)| format!("{x:.0} {y:.0}"))
        .collect();
    values.push(values[0].clone());
    let n = values.len() - 1;
    let times: Vec<String> = (0..=n)
        .map(|i| format!("{:.3}", i as f32 / n as f32))
        .collect();
    let splines = vec!["0.45 0 0.55 1"; n].join(";");
    format!(
        "<animateTransform attributeName=\"transform\" type=\"translate\" values=\"{}\" keyTimes=\"{}\" \
         dur=\"{dur}s\" repeatCount=\"indefinite\" calcMode=\"spline\" keySplines=\"{splines}\"/>",
        values.join(";"),
        times.join(";"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retired_ids_map_to_the_closest_survivor() {
        assert_eq!(normalize_animation(Some("bounce")), "rise");
        assert_eq!(normalize_animation(Some("slide")), "rise");
        assert_eq!(normalize_animation(Some("glitch")), "ambient");
        assert_eq!(normalize_animation(Some("neon")), "ambient");
        assert_eq!(normalize_animation(Some(" FADE ")), "fade");
        assert_eq!(normalize_animation(None), "ambient");
    }

    #[test]
    fn text_entries_play_once() {
        for a in ["fade", "rise"] {
            let c = text_children(a, 0, 880, 220);
            assert!(c.contains("fill=\"freeze\""), "{a}");
            assert!(!c.contains("indefinite"), "{a} must not loop");
        }
    }

    #[test]
    fn drift_is_a_closed_eased_loop() {
        let d = drift(&[(0.0, 0.0), (10.0, -4.0)], 20.0);
        assert!(d.contains("values=\"0 0;10 -4;0 0\""));
        assert!(d.contains("keySplines=\"0.45 0 0.55 1;0.45 0 0.55 1\""));
    }
}
