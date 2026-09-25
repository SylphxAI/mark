//! Badge composition: one laid-out pill for every pill-shaped form.
//!
//! `pill`, `score`, and `deploy` all build a [`Badge`] and hand it to
//! [`compose`], which lays it out with badge-maker's geometry for the chosen
//! style. Geometry constants, text measurement, and text paint come from the
//! domain authority (`domain/pill.rs`); this module only places boxes.

use crate::capabilities::mark::domain::color::resolve_paint;
use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::pill::{
    bold_run, fmt, ink, measure, text_group_open, text_run, Part,
};
use crate::capabilities::mark::domain::shields::data_uri_logo;
use crate::capabilities::mark::domain::svg::{ensure_hash, esc, svg_doc};
use crate::capabilities::mark::domain::theme;
use crate::capabilities::mark::domain::{
    cap_text, normalize_animation, MarkSpec, PillStyle, MAX_LABEL_CHARS, MAX_MESSAGE_CHARS,
};

/// Logo height in px (badge-maker `DEFAULT_LOGO_HEIGHT`).
const LOGO_H: f64 = 14.0;
/// Gap between a label-side logo and the label text (shields `logoPadding`).
const LOGO_PAD: f64 = 3.0;

/// What sits left of the label.
#[derive(Debug, Clone)]
pub(crate) enum Logo {
    /// A validated base64 image data URI.
    Image { href: String, width: f64 },
    /// The deploy conversion mark: a dot in a ring, painted as vectors.
    DeployMark,
}

impl Logo {
    pub(super) fn width(&self) -> f64 {
        match self {
            Self::Image { width, .. } => *width,
            Self::DeployMark => LOGO_H,
        }
    }

    pub(super) fn paint(&self, x: f64, h: f64) -> String {
        let y = (h - LOGO_H) / 2.0;
        match self {
            Self::Image { href, width } => format!(
                "<image x=\"{}\" y=\"{}\" width=\"{}\" height=\"14\" href=\"{href}\"/>",
                fmt(x),
                fmt(y),
                fmt(*width)
            ),
            Self::DeployMark => {
                let (cx, cy) = (fmt(x + 7.0), fmt(h / 2.0));
                format!(
                    "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"2.5\" fill=\"#fff\"/>\
                     <circle cx=\"{cx}\" cy=\"{cy}\" r=\"5.75\" fill=\"none\" stroke=\"#fff\" stroke-width=\"1.5\"/>"
                )
            }
        }
    }
}

/// A pill ready to lay out.
pub(crate) struct Badge {
    pub style: PillStyle,
    pub label: String,
    pub message: String,
    /// Canonical `#hex` paints.
    pub label_color: String,
    pub color: String,
    /// shields: an explicit `labelColor` keeps the label box even when empty.
    pub explicit_label_color: bool,
    pub logo: Option<Logo>,
    /// Score badges: fraction `0..=1` shown as a ring before the message.
    pub ring: Option<f64>,
    pub mono: bool,
    pub anim: &'static str,
}

impl Badge {
    /// Shared grammar for pill-shaped forms: style, theme paint, explicit
    /// colors, logo, font, and text-level motion. `default_color` paints the
    /// message when neither a theme nor `color` says otherwise.
    pub(crate) fn from_spec(spec: &MarkSpec, message: &str, default_color: &str) -> Self {
        let style = PillStyle::parse(spec.pill.style.as_deref().unwrap_or("flat"));
        let theme = spec.theme.as_deref().and_then(theme::get);
        let (color, label_color) = match theme {
            Some(t) => (ensure_hash(t.accent), ensure_hash(t.bg)),
            None => (
                ensure_hash(&resolve_paint(
                    spec.color.as_deref(),
                    &resolve_paint(Some(default_color), "4A90E2"),
                )),
                ensure_hash(&resolve_paint(spec.pill.label_color.as_deref(), "555555")),
            ),
        };
        let logo = spec
            .pill
            .logo
            .as_deref()
            .and_then(data_uri_logo)
            .map(|href| {
                let width = spec.pill.logo_width.unwrap_or(14).clamp(1, 64) as f64;
                Logo::Image { href, width }
            });
        // Motion at pill scale is text-level only; ambient is static here.
        let anim = match normalize_animation(spec.animation.as_deref()) {
            "ambient" | "none" => "none",
            a => a,
        };
        Self {
            style,
            label: cap_text(spec.pill.label.as_deref().unwrap_or(""), MAX_LABEL_CHARS),
            message: cap_text(message, MAX_MESSAGE_CHARS),
            label_color,
            color,
            explicit_label_color: theme.is_none() && spec.pill.label_color.is_some(),
            logo,
            ring: None,
            mono: spec
                .font
                .as_deref()
                .is_some_and(|f| f.eq_ignore_ascii_case("mono")),
            anim,
        }
    }

    /// Wrap a text run in the requested text-level motion.
    fn animated(&self, index: usize, w: f64, h: f64, run: String) -> String {
        if self.anim == "none" {
            return run;
        }
        let (w, h) = (w.ceil() as u32, h as u32);
        format!(
            "<g{}>{}{run}</g>",
            text_open_attrs(self.anim, index, w, h),
            text_children(self.anim, index, w, h)
        )
    }

    /// badge-maker pads a logo from the label only when there is a label.
    pub(super) fn logo_pad(&self) -> f64 {
        if self.label.is_empty() {
            0.0
        } else {
            LOGO_PAD
        }
    }

    /// Accessible name, shields style: `label: message`.
    pub(super) fn title(&self, label: &str, message: &str) -> String {
        let text = match (label.is_empty(), message.is_empty()) {
            (true, _) => message.to_string(),
            (false, true) => label.to_string(),
            (false, false) => format!("{label}: {message}"),
        };
        format!("<title>{}</title>", esc(&text))
    }
}

/// Lay out and paint a badge in its style.
pub(crate) fn compose(b: &Badge) -> String {
    match b.style {
        PillStyle::ForTheBadge => for_the_badge(b),
        PillStyle::Social => super::badge_social::social(b),
        _ => flat(b),
    }
}

/// A progress ring of diameter `d` whose left edge is at `x`.
pub(crate) fn ring(x: f64, cy: f64, d: f64, frac: f64, ink: &str, track: &str) -> String {
    let r = d / 2.0 - 1.0;
    let (cx, cyf, rf) = (fmt(x + d / 2.0), fmt(cy), fmt(r));
    let mut out = format!(
        "<circle cx=\"{cx}\" cy=\"{cyf}\" r=\"{rf}\" fill=\"none\" stroke=\"{track}\" stroke-width=\"2\"/>"
    );
    let frac = frac.clamp(0.0, 1.0);
    if frac >= 1.0 {
        out.push_str(&format!(
            "<circle cx=\"{cx}\" cy=\"{cyf}\" r=\"{rf}\" fill=\"none\" stroke=\"{ink}\" stroke-width=\"2\"/>"
        ));
    } else if frac > 0.0 {
        let c = std::f64::consts::TAU * r;
        out.push_str(&format!(
            "<circle cx=\"{cx}\" cy=\"{cyf}\" r=\"{rf}\" fill=\"none\" stroke=\"{ink}\" stroke-width=\"2\" \
             stroke-linecap=\"round\" stroke-dasharray=\"{} {}\" transform=\"rotate(-90 {cx} {cyf})\"/>",
            fmt(c * frac),
            fmt(c)
        ));
    }
    out
}

/// Ring ink and track on a colored message box.
fn ring_paint(bg: &str) -> (&'static str, &'static str) {
    match ink(bg).0 {
        "#fff" => ("#fff", "rgba(255,255,255,.35)"),
        _ => ("#333", "rgba(0,0,0,.18)"),
    }
}

/// flat, flat-square, plastic, and pill: badge-maker's `Badge` layout.
fn flat(b: &Badge) -> String {
    let (st, ch) = (b.style, b.style.chrome());
    let h = ch.height as f64;
    let has_label = !b.label.is_empty() || b.explicit_label_color;
    let has_msg = !b.message.is_empty();
    let logo_w = b
        .logo
        .as_ref()
        .map(|l| l.width() + b.logo_pad())
        .unwrap_or(0.0);
    let has_logo = b.logo.is_some();
    let label_w = if b.label.is_empty() {
        0.0
    } else {
        measure(&b.label, st, Part::Label, b.mono)
    };
    let msg_w = measure(&b.message, st, Part::Message, b.mono);
    let ring_slot = b.ring.map(|_| st.ring_diameter() + 3.0).unwrap_or(0.0);

    let left = if has_label {
        label_w + 10.0 + logo_w
    } else {
        0.0
    };
    let mut msg_margin = left - if has_msg { 1.0 } else { 0.0 };
    if !has_label {
        msg_margin += if has_logo { logo_w + 5.0 } else { 1.0 };
    }
    msg_margin += ring_slot;
    let mut right = msg_w + 10.0 + ring_slot;
    if has_logo && !has_label {
        right += logo_w + if has_msg { 4.0 } else { 0.0 };
    }
    let w = left + right;
    let label_fill = if has_label || has_logo {
        &b.label_color
    } else {
        &b.color
    };
    let (ws, hs, ls, rs) = (fmt(w), fmt(h), fmt(left), fmt(right));

    let mut body = b.title(&b.label, &b.message);
    body.push_str(ch.defs);
    let group = if ch.radius > 0.0 {
        body.push_str(&format!(
            "<clipPath id=\"r\"><rect width=\"{ws}\" height=\"{hs}\" rx=\"{}\"/></clipPath>",
            fmt(ch.radius)
        ));
        "clip-path=\"url(#r)\""
    } else {
        "shape-rendering=\"crispEdges\""
    };
    body.push_str(&format!(
        "<g {group}><rect width=\"{ls}\" height=\"{hs}\" fill=\"{label_fill}\"/>\
         <rect x=\"{ls}\" width=\"{rs}\" height=\"{hs}\" fill=\"{}\"/>",
        b.color
    ));
    if ch.overlay {
        body.push_str(&format!(
            "<rect width=\"{ws}\" height=\"{hs}\" fill=\"url(#s)\"/>"
        ));
    }
    body.push_str("</g>");
    body.push_str(&text_group_open(st, b.mono));
    if let Some(logo) = &b.logo {
        body.push_str(&logo.paint(5.0, h));
    }
    if let Some(frac) = b.ring {
        let d = st.ring_diameter();
        let (ink, track) = ring_paint(&b.color);
        body.push_str(&ring(
            msg_margin + 5.0 - 3.0 - d,
            h / 2.0,
            d,
            frac,
            ink,
            track,
        ));
    }
    if !b.label.is_empty() {
        let cx = logo_w + 1.0 + 5.0 + label_w / 2.0;
        let run = text_run(st, cx, label_w, &b.label, label_fill);
        body.push_str(&b.animated(0, w, h, run));
    }
    if has_msg {
        let cx = msg_margin + 5.0 + msg_w / 2.0;
        let run = text_run(st, cx, msg_w, &b.message, &b.color);
        body.push_str(&b.animated(1, w, h, run));
    }
    body.push_str("</g>");
    svg_doc(ws, hs, &body)
}

/// for-the-badge: badge-maker's `forTheBadge` layout (square, upper case,
/// bold message, 1.25 px tracking).
fn for_the_badge(b: &Badge) -> String {
    const TEXT_MARGIN: f64 = 12.0;
    const LOGO_MARGIN: f64 = 9.0;
    const GUTTER: f64 = 6.0;
    let st = PillStyle::ForTheBadge;
    let h = st.chrome().height as f64;
    let (label, message) = (b.label.to_uppercase(), b.message.to_uppercase());
    let label_tw = measure(&label, st, Part::Label, b.mono);
    let msg_tw = measure(&message, st, Part::Message, b.mono);
    let has_label = !label.is_empty();
    let no_text = !has_label && message.is_empty();
    let logo_w = b.logo.as_ref().map(Logo::width).unwrap_or(0.0);
    let needs_label_rect = has_label || (b.logo.is_some() && b.explicit_label_color);
    let gutter = if no_text {
        GUTTER - LOGO_MARGIN
    } else {
        GUTTER
    };
    let label_text_min_x = if b.logo.is_some() {
        LOGO_MARGIN + logo_w + gutter
    } else {
        TEXT_MARGIN
    };
    let (label_rect_w, mut msg_text_min_x, mut msg_rect_w) = if needs_label_rect {
        let lw = if has_label {
            label_text_min_x + label_tw + TEXT_MARGIN
        } else {
            2.0 * LOGO_MARGIN + logo_w
        };
        (lw, lw + TEXT_MARGIN, 2.0 * TEXT_MARGIN + msg_tw)
    } else if b.logo.is_some() {
        (
            0.0,
            TEXT_MARGIN + logo_w + gutter,
            2.0 * TEXT_MARGIN + logo_w + gutter + msg_tw,
        )
    } else {
        (0.0, TEXT_MARGIN, 2.0 * TEXT_MARGIN + msg_tw)
    };
    let ring_slot = b.ring.map(|_| st.ring_diameter() + 3.0).unwrap_or(0.0);
    msg_text_min_x += ring_slot;
    msg_rect_w += ring_slot;
    let w = label_rect_w + msg_rect_w;
    let (ws, hs) = (fmt(w), fmt(h));

    let mut body = b.title(&label, &message);
    body.push_str("<g shape-rendering=\"crispEdges\">");
    if needs_label_rect {
        body.push_str(&format!(
            "<rect width=\"{}\" height=\"{hs}\" fill=\"{}\"/>",
            fmt(label_rect_w),
            b.label_color
        ));
    }
    body.push_str(&format!(
        "<rect x=\"{}\" width=\"{}\" height=\"{hs}\" fill=\"{}\"/></g>",
        fmt(label_rect_w),
        fmt(msg_rect_w),
        b.color
    ));
    body.push_str(&text_group_open(st, b.mono));
    if let Some(logo) = &b.logo {
        body.push_str(&logo.paint(LOGO_MARGIN, h));
    }
    if let Some(frac) = b.ring {
        let d = st.ring_diameter();
        let (ink, track) = ring_paint(&b.color);
        body.push_str(&ring(
            msg_text_min_x - 3.0 - d,
            h / 2.0,
            d,
            frac,
            ink,
            track,
        ));
    }
    if has_label {
        let cx = label_text_min_x + label_tw / 2.0;
        let run = text_run(st, cx, label_tw, &label, &b.label_color);
        body.push_str(&b.animated(0, w, h, run));
    }
    let run = bold_run(msg_text_min_x + msg_tw / 2.0, msg_tw, &message, &b.color);
    body.push_str(&b.animated(1, w, h, run));
    body.push_str("</g>");
    svg_doc(ws, hs, &body)
}
