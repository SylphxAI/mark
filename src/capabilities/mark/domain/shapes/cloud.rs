//! Ambient cloud — the drifting blob geometry every family shares.
//!
//! One table of art-keyed specs and one renderer, so the blob skeleton and its
//! paint roles exist once instead of once per family module.

use super::Canvas;
use crate::capabilities::mark::domain::art::Art;

/// Soft blob that drifts when gain > 0.
///
/// Motion is applied on a parent `<g>` via `animateTransform` (more reliable than
/// animating `cx`/`cy` on filtered ellipses inside SVG-as-`<img>`).
/// Typed geometry for one drifting blob.
pub(super) struct Blob<'a> {
    pub center: (f32, f32),
    pub size: (f32, f32),
    pub color: &'a str,
    pub opacity: f32,
    pub drift: (f32, f32),
    pub dur: f32,
    pub phase: f32,
}

pub(super) fn blob(b: Blob<'_>, gain: f32) -> String {
    let Blob {
        center: (cx, cy),
        size: (rx, ry),
        color,
        opacity,
        drift: (dx, dy),
        dur,
        phase,
    } = b;
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

/// Which resolved paint role an ambient blob uses.
#[derive(Clone, Copy)]
enum Paint {
    Accent,
    Accent2,
    Warm,
    Glow,
    White,
    Lilac,
    Mint,
    Indigo,
    Blue,
    Amber,
    Haze,
    Fuchsia,
}

/// One drifting blob, in canvas fractions (multiplied by `wf`/`hf` at paint
/// time so every art declares geometry relative to its own canvas).
struct CloudSpec {
    center: (f32, f32),
    size: (f32, f32),
    paint: Paint,
    opacity: f32,
    drift: (f32, f32),
    dur: f32,
    phase: f32,
}

const fn cloud(
    center: (f32, f32),
    size: (f32, f32),
    paint: Paint,
    opacity: f32,
    drift: (f32, f32),
    dur: f32,
    phase: f32,
) -> CloudSpec {
    CloudSpec {
        center,
        size,
        paint,
        opacity,
        drift,
        dur,
        phase,
    }
}

/// The drifting ambient clouds, keyed by art: one authority for the blob
/// geometry every family used to spell out inline.
static CLOUD_0: &[CloudSpec] = &[
    cloud(
        (0.15, 0.4),
        (0.34, 0.7),
        Paint::Accent,
        0.36,
        (0.1, 0.12),
        8.0,
        0.0,
    ),
    cloud(
        (0.55, 0.2),
        (0.4, 0.55),
        Paint::Accent2,
        0.38,
        (-0.08, 0.1),
        9.5,
        0.4,
    ),
    cloud(
        (0.85, 0.55),
        (0.32, 0.6),
        Paint::Warm,
        0.32,
        (-0.06, -0.1),
        7.5,
        0.9,
    ),
    cloud(
        (0.4, 0.85),
        (0.28, 0.4),
        Paint::Glow,
        0.24,
        (0.05, -0.08),
        10.0,
        1.2,
    ),
    cloud(
        (0.72, 0.18),
        (0.24, 0.36),
        Paint::Accent,
        0.18,
        (-0.04, 0.06),
        6.5,
        0.2,
    ),
];
static CLOUD_1: &[CloudSpec] = &[cloud(
    (0.7, 0.35),
    (0.28, 0.45),
    Paint::Accent,
    0.2,
    (-0.05, 0.04),
    8.0,
    0.3,
)];
static CLOUD_2: &[CloudSpec] = &[cloud(
    (0.75, 0.35),
    (0.2, 0.4),
    Paint::Mint,
    0.12,
    (-0.04, 0.05),
    7.0,
    0.0,
)];
static CLOUD_3: &[CloudSpec] = &[cloud(
    (0.2, 0.3),
    (0.25, 0.4),
    Paint::Accent2,
    0.14,
    (0.04, 0.05),
    9.0,
    0.0,
)];
static CLOUD_4: &[CloudSpec] = &[cloud(
    (0.7, 0.3),
    (0.25, 0.4),
    Paint::Warm,
    0.16,
    (-0.05, 0.06),
    8.0,
    0.3,
)];
static CLOUD_5: &[CloudSpec] = &[cloud(
    (0.5, 0.2),
    (0.3, 0.3),
    Paint::Glow,
    0.16,
    (0.0, 0.05),
    7.0,
    0.0,
)];
static CLOUD_6: &[CloudSpec] = &[
    cloud(
        (0.25, 0.55),
        (0.3, 0.45),
        Paint::Indigo,
        0.35,
        (0.05, -0.04),
        11.0,
        0.0,
    ),
    cloud(
        (0.75, 0.35),
        (0.28, 0.4),
        Paint::Blue,
        0.28,
        (-0.05, 0.05),
        10.0,
        0.6,
    ),
];
static CLOUD_7: &[CloudSpec] = &[cloud(
    (0.5, 0.7),
    (0.4, 0.35),
    Paint::Amber,
    0.2,
    (0.0, -0.03),
    9.0,
    0.0,
)];
static CLOUD_8: &[CloudSpec] = &[cloud(
    (0.3, 0.4),
    (0.3, 0.5),
    Paint::Haze,
    0.14,
    (0.05, 0.04),
    10.0,
    0.0,
)];
static CLOUD_9: &[CloudSpec] = &[
    cloud(
        (0.2, 0.3),
        (0.28, 0.45),
        Paint::Fuchsia,
        0.22,
        (0.06, 0.05),
        8.0,
        0.0,
    ),
    cloud(
        (0.8, 0.65),
        (0.3, 0.4),
        Paint::Warm,
        0.2,
        (-0.05, -0.05),
        9.0,
        0.5,
    ),
];
static CLOUD_10: &[CloudSpec] = &[
    cloud(
        (0.25, 0.35),
        (0.3, 0.5),
        Paint::Accent,
        0.22,
        (0.05, -0.04),
        9.0,
        0.0,
    ),
    cloud(
        (0.7, 0.45),
        (0.32, 0.48),
        Paint::Accent2,
        0.24,
        (-0.06, 0.05),
        10.0,
        0.5,
    ),
    cloud(
        (0.5, 0.2),
        (0.22, 0.3),
        Paint::Warm,
        0.16,
        (0.03, 0.04),
        8.0,
        1.0,
    ),
];
static CLOUD_11: &[CloudSpec] = &[
    cloud(
        (0.2, 0.35),
        (0.36, 0.55),
        Paint::Accent,
        0.3,
        (0.07, 0.05),
        8.5,
        0.0,
    ),
    cloud(
        (0.7, 0.3),
        (0.38, 0.52),
        Paint::Accent2,
        0.32,
        (-0.06, 0.06),
        9.5,
        0.4,
    ),
    cloud(
        (0.5, 0.75),
        (0.34, 0.42),
        Paint::Warm,
        0.24,
        (0.04, -0.05),
        10.5,
        0.9,
    ),
    cloud(
        (0.85, 0.65),
        (0.26, 0.36),
        Paint::Glow,
        0.18,
        (-0.04, -0.04),
        7.5,
        1.3,
    ),
];
static CLOUD_12: &[CloudSpec] = &[cloud(
    (0.75, 0.3),
    (0.25, 0.45),
    Paint::Glow,
    0.14,
    (-0.04, 0.05),
    9.0,
    0.0,
)];
static CLOUD_13: &[CloudSpec] = &[
    cloud(
        (0.28, 0.4),
        (0.35, 0.55),
        Paint::White,
        0.2,
        (0.06, 0.07),
        10.0,
        0.0,
    ),
    cloud(
        (0.78, 0.65),
        (0.3, 0.45),
        Paint::Lilac,
        0.16,
        (-0.05, -0.06),
        12.0,
        1.0,
    ),
];
static CLOUD_14: &[CloudSpec] = &[cloud(
    (0.3, 0.4),
    (0.28, 0.4),
    Paint::Accent,
    0.16,
    (0.04, 0.0),
    9.0,
    0.0,
)];

fn cloud_specs(art: Art) -> &'static [CloudSpec] {
    match art {
        Art::Plasma => CLOUD_0,
        Art::Holo => CLOUD_1,
        Art::Neon => CLOUD_2,
        Art::Meteor => CLOUD_3,
        Art::Liquid => CLOUD_4,
        Art::Prism => CLOUD_5,
        Art::Void => CLOUD_6,
        Art::Firefly => CLOUD_7,
        Art::Silk => CLOUD_8,
        Art::Iridescent => CLOUD_9,
        Art::Aurora => CLOUD_10,
        Art::Mesh => CLOUD_11,
        Art::Glass => CLOUD_12,
        Art::Blur => CLOUD_13,
        Art::Product | Art::Oss | Art::Org => CLOUD_14,
        _ => &[],
    }
}

/// Resolve a paint role against the canvas plan.
fn paint<'a>(canvas: &'a Canvas<'_>, paint: Paint) -> &'a str {
    match paint {
        Paint::Accent => canvas.plan.accent.as_str(),
        Paint::Accent2 => canvas.plan.accent2.as_str(),
        Paint::Warm => canvas.plan.warm.as_str(),
        Paint::Glow => canvas.plan.glow.as_str(),
        Paint::White => "#ffffff",
        Paint::Lilac => "#c4b5fd",
        Paint::Mint => "#00f5d4",
        Paint::Indigo => "#4c1d95",
        Paint::Blue => "#1e3a8a",
        Paint::Amber => "#78350f",
        Paint::Haze => "#e9d5ff",
        Paint::Fuchsia => "#f0abfc",
    }
}

/// Render the art's drifting ambient cloud: one table, one renderer.
pub(super) fn render(canvas: &Canvas<'_>, art: Art) -> String {
    let mut out = String::new();
    for spec in cloud_specs(art) {
        out.push_str(&blob(
            Blob {
                center: (spec.center.0 * canvas.wf, spec.center.1 * canvas.hf),
                size: (spec.size.0 * canvas.wf, spec.size.1 * canvas.hf),
                color: paint(canvas, spec.paint),
                opacity: spec.opacity,
                drift: (spec.drift.0 * canvas.wf, spec.drift.1 * canvas.hf),
                dur: spec.dur,
                phase: spec.phase,
            },
            canvas.gain,
        ));
    }
    out
}
