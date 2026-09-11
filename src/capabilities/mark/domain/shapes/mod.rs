//! Banner backgrounds: one module per art family, one exhaustive dispatch.
//!
//! `gain` (0..1) scales ambient motion intensity; 0 freezes decorative layers.
//! The published vocabulary lives in [`super::art::Art`]; every family module
//! paints exactly the art types it owns, so adding an art type without an
//! implementation fails the build instead of silently rendering the default.
//! Ambient motion is SMIL so it works in SVG-as-`<img>`.

use crate::capabilities::mark::domain::art::Art;
use crate::capabilities::mark::domain::color::FillPlan;

mod canvas;
mod cloud;
mod geometry;
mod pane;
mod patterns;
mod showcase;

/// The resolved canvas every art function paints on.
///
/// One authority for the axes, the ambient gain, and the paint plan, so the
/// per-art preamble (`wf`/`hf`/`g`, role strings) exists once instead of once
/// per family function.
pub(super) struct Canvas<'a> {
    pub w: u32,
    pub h: u32,
    pub wf: f32,
    pub hf: f32,
    pub gain: f32,
    pub plan: &'a FillPlan,
}

impl<'a> Canvas<'a> {
    pub(super) fn new(w: u32, h: u32, plan: &'a FillPlan, gain: f32) -> Self {
        Self {
            w,
            h,
            wf: w as f32,
            hf: h as f32,
            gain,
            plan,
        }
    }

    /// The field stack every family paints its own layers on.
    pub(super) fn stack(&self) -> String {
        field_stack(self.w, self.h, self.plan)
    }

    /// The shared composition tail: base, family layers, sheen, vignette.
    pub(super) fn finish(&self, base: String, layers: &str) -> String {
        let mut out = base;
        out.push_str(layers);
        out.push_str(&sheen(self.w, self.h, self.gain, self.plan));
        out.push_str(&vignette(self.w, self.h, self.plan));
        out
    }

    /// Accent gloss without a vignette: the terminal pane keeps full contrast.
    pub(super) fn gloss(&self) -> String {
        sheen(self.w, self.h, self.gain, self.plan)
    }

    /// The art's drifting ambient cloud: one table, one renderer.
    pub(super) fn cloud(&self, art: Art) -> String {
        cloud::render(self, art)
    }
}

/// Deterministic scatter stream: decorative positions must stay a pure
/// function of the URL, so no clock or OS entropy is ever consulted.
pub(super) struct Rng {
    state: u32,
    mul: u32,
    add: u32,
}

/// One scattered canvas point, plus the draw that produced `y`: callers may
/// fold a radius or size out of the same draw, so the stream stays identical
/// to the historic inline walk.
pub(super) struct Point {
    pub x: u32,
    pub y: u32,
    pub draw: u32,
}

impl Rng {
    /// The canvas LCG (star fields, constellation walks).
    pub(super) fn lcg(seed: u32) -> Self {
        Self {
            state: seed,
            mul: 1_664_525,
            add: 1_013_904_223,
        }
    }

    /// The finer noise-field LCG (dot scatter).
    pub(super) fn scatter(seed: u32) -> Self {
        Self {
            state: seed,
            mul: 1_103_515_245,
            add: 12_345,
        }
    }

    pub(super) fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(self.mul).wrapping_add(self.add);
        self.state
    }

    /// The current draw without advancing: callers that fold two fields out of
    /// one draw (a star's radius shares its y draw).
    pub(super) fn current(&self) -> u32 {
        self.state
    }

    /// `count` deterministic points inside an inset box: one authority for the
    /// scatter loops every family used to spell out inline.
    pub(super) fn points(
        &mut self,
        count: u32,
        w: u32,
        h: u32,
        inset: (u32, u32),
        base: (u32, u32),
    ) -> Vec<Point> {
        let mut out = Vec::new();
        for _ in 0..count {
            let x = base.0 + self.next() % w.saturating_sub(inset.0).max(1);
            let y = base.1 + self.next() % h.saturating_sub(inset.1).max(1);
            out.push(Point {
                x,
                y,
                draw: self.current(),
            });
        }
        out
    }
}

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

/// Paint the background for one art type.
///
/// `gain` scales ambient motion (0 freezes decorative layers). The match is
/// exhaustive over [`Art`]; an unknown `type=` already normalized to
/// [`Art::Waving`], so there is no catch-all to hide a missing family.
pub(crate) fn shape_background(art: Art, w: u32, h: u32, plan: &FillPlan, gain: f32) -> String {
    let canvas = Canvas::new(w, h, plan, gain);
    match art {
        Art::Transparent => geometry::transparent(art, &canvas),
        Art::Plasma => showcase::plasma(art, &canvas),
        Art::Holo => showcase::holo(art, &canvas),
        Art::Neon => showcase::neon(art, &canvas),
        Art::Meteor => showcase::meteor(art, &canvas),
        Art::Liquid => showcase::liquid(art, &canvas),
        Art::Prism => showcase::prism(art, &canvas),
        Art::Void => showcase::void(art, &canvas),
        Art::Firefly => showcase::firefly(art, &canvas),
        Art::Silk => showcase::silk(art, &canvas),
        Art::Iridescent => showcase::iridescent(art, &canvas),
        Art::Rect => geometry::rect(art, &canvas),
        Art::Soft => canvas::soft(art, &canvas),
        Art::Rounded => canvas::soft(art, &canvas),
        Art::Aurora => canvas::aurora(art, &canvas),
        Art::Mesh => canvas::mesh(art, &canvas),
        Art::Glass => canvas::glass(art, &canvas),
        Art::Horizon => canvas::horizon(art, &canvas),
        Art::Dusk => canvas::horizon(art, &canvas),
        Art::Wave => canvas::wave(art, &canvas),
        Art::Waving => canvas::wave(art, &canvas),
        Art::Orbit => canvas::orbit(art, &canvas),
        Art::Ring => canvas::ring(art, &canvas),
        Art::Beam => canvas::beam(art, &canvas),
        Art::Terminal => canvas::terminal(art, &canvas),
        Art::Constellation => canvas::constellation(art, &canvas),
        Art::Blur => geometry::blur(art, &canvas),
        Art::Grid => patterns::grid(art, &canvas),
        Art::Circuit => patterns::circuit(art, &canvas),
        Art::Hud => patterns::hud(art, &canvas),
        Art::Pulse => patterns::pulse(art, &canvas),
        Art::Noise => patterns::noise(art, &canvas),
        Art::Cylinder => geometry::cylinder(art, &canvas),
        Art::Slice => geometry::slice(art, &canvas),
        Art::Egg => geometry::egg(art, &canvas),
        Art::Shark => geometry::shark(art, &canvas),
        Art::Venom => geometry::shark(art, &canvas),
        Art::Speech => geometry::speech(art, &canvas),
        Art::Checkered => geometry::checkered(art, &canvas),
        Art::Product => geometry::product(art, &canvas),
        Art::Oss => geometry::product(art, &canvas),
        Art::Org => geometry::product(art, &canvas),
    }
}
