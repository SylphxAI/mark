//! Banner backgrounds: one module per art family, one exhaustive dispatch.
//!
//! `gain` (0..1) scales ambient motion intensity; 0 freezes decorative layers.
//! The published vocabulary lives in [`super::art::Art`]; every family module
//! paints exactly the art types it owns, so adding an art type without an
//! implementation fails the build instead of silently rendering the default.

//! Banner backgrounds with ambient SMIL motion (works in SVG-as-`<img>`).
//!
//! `gain` (0..1) scales motion intensity; 0 freezes decorative layers.

use crate::capabilities::mark::domain::art::Art;
use crate::capabilities::mark::domain::color::FillPlan;

pub const ART_TYPES: &[&str] = &[
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

mod canvas;
mod geometry;
mod showcase;

pub(crate) fn shape_defs(art: Art, gain: f32, plan: &FillPlan) -> String {
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
    if art == Art::Glass {
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

#[allow(clippy::format_in_format_args)]
/// Paint the background for one art type.
///
/// `gain` scales ambient motion (0 freezes decorative layers). The match is
/// exhaustive over [`Art`]; an unknown `type=` already normalized to
/// [`Art::Waving`], so there is no catch-all to hide a missing family.
pub(crate) fn shape_background(art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    match art {
        Art::Transparent => geometry::transparent(art, w, h, plan, gain),
        Art::Plasma => showcase::plasma(art, w, h, plan, gain),
        Art::Holo => showcase::holo(art, w, h, plan, gain),
        Art::Neon => showcase::neon(art, w, h, plan, gain),
        Art::Meteor => showcase::meteor(art, w, h, plan, gain),
        Art::Liquid => showcase::liquid(art, w, h, plan, gain),
        Art::Prism => showcase::prism(art, w, h, plan, gain),
        Art::Void => showcase::void(art, w, h, plan, gain),
        Art::Firefly => showcase::firefly(art, w, h, plan, gain),
        Art::Silk => showcase::silk(art, w, h, plan, gain),
        Art::Iridescent => showcase::iridescent(art, w, h, plan, gain),
        Art::Rect => geometry::rect(art, w, h, plan, gain),
        Art::Soft => canvas::soft(art, w, h, plan, gain),
        Art::Rounded => canvas::soft(art, w, h, plan, gain),
        Art::Aurora => canvas::aurora(art, w, h, plan, gain),
        Art::Mesh => canvas::mesh(art, w, h, plan, gain),
        Art::Glass => canvas::glass(art, w, h, plan, gain),
        Art::Horizon => canvas::horizon(art, w, h, plan, gain),
        Art::Dusk => canvas::horizon(art, w, h, plan, gain),
        Art::Wave => canvas::wave(art, w, h, plan, gain),
        Art::Waving => canvas::wave(art, w, h, plan, gain),
        Art::Orbit => canvas::orbit(art, w, h, plan, gain),
        Art::Ring => canvas::ring(art, w, h, plan, gain),
        Art::Beam => canvas::beam(art, w, h, plan, gain),
        Art::Terminal => canvas::terminal(art, w, h, plan, gain),
        Art::Constellation => canvas::constellation(art, w, h, plan, gain),
        Art::Blur => geometry::blur(art, w, h, plan, gain),
        Art::Grid => canvas::grid(art, w, h, plan, gain),
        Art::Circuit => canvas::circuit(art, w, h, plan, gain),
        Art::Hud => canvas::hud(art, w, h, plan, gain),
        Art::Pulse => canvas::pulse(art, w, h, plan, gain),
        Art::Noise => canvas::noise(art, w, h, plan, gain),
        Art::Cylinder => geometry::cylinder(art, w, h, plan, gain),
        Art::Slice => geometry::slice(art, w, h, plan, gain),
        Art::Egg => geometry::egg(art, w, h, plan, gain),
        Art::Shark => geometry::shark(art, w, h, plan, gain),
        Art::Venom => geometry::shark(art, w, h, plan, gain),
        Art::Speech => geometry::speech(art, w, h, plan, gain),
        Art::Checkered => geometry::checkered(art, w, h, plan, gain),
        Art::Product => geometry::product(art, w, h, plan, gain),
        Art::Oss => geometry::product(art, w, h, plan, gain),
        Art::Org => geometry::product(art, w, h, plan, gain),
    }
}
