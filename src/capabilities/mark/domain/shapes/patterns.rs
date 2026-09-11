//! Pattern art family — grids, circuits, HUDs, and signals.
//!
//! One function per art type; `super::shape_background` dispatches exhaustively.

use super::{Canvas, Rng};
use crate::capabilities::mark::domain::art::Art;

pub(super) fn grid(_art: Art, c: &Canvas) -> String {
    let (w, h, hf) = (c.w, c.h, c.hf);
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
    let scan = if c.gain > 0.01 {
        format!(
                    "<rect y=\"0\" width=\"{w}\" height=\"{hh}\" fill=\"url(#scan)\" opacity=\"0.7\">\
                       <animate attributeName=\"y\" from=\"-{hh}\" to=\"{h}\" dur=\"4.2s\" repeatCount=\"indefinite\"/>\
                     </rect>",
                    hh = hf * 0.22,
                )
    } else {
        String::new()
    };
    c.finish(c.stack(), &format!("{lines}{scan}"))
}

pub(super) fn circuit(_art: Art, c: &Canvas) -> String {
    let (w, wf, hf) = (c.w, c.wf, c.hf);
    let mut traces = String::new();
    for i in 0..6 {
        let y = hf * 0.2 + i as f32 * (hf * 0.12);
        let mid = wf * (0.22 + (i % 4) as f32 * 0.14);
        let node = if c.gain > 0.01 {
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
        let pulse = if c.gain > 0.01 {
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
    c.finish(c.stack(), &traces)
}

pub(super) fn hud(_art: Art, c: &Canvas) -> String {
    let sweep = if c.gain > 0.01 {
        format!(
                    "<rect x=\"16\" y=\"16\" width=\"{iw}\" height=\"3\" fill=\"#ffffff\" fill-opacity=\"0.2\">\
                       <animate attributeName=\"y\" values=\"16;{max_y};16\" dur=\"3.8s\" repeatCount=\"indefinite\"/>\
                       <animate attributeName=\"fill-opacity\" values=\"0.05;0.35;0.05\" dur=\"3.8s\" repeatCount=\"indefinite\"/>\
                     </rect>",
                    iw = c.w.saturating_sub(32),
                    max_y = c.h.saturating_sub(20),
                )
    } else {
        String::new()
    };
    let layers = format!(
                "<rect x=\"16\" y=\"16\" width=\"{iw}\" height=\"{ih}\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.2\" stroke-dasharray=\"5 4\">{dash}</rect>\
                 <path d=\"M16,40 H40 M16,16 V40\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
                 <path d=\"M{r},40 H{r2} M{r},16 V40\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
                 <path d=\"M16,{b} H40 M16,{bb} V{b}\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
                 <path d=\"M{r},{b} H{r2} M{r},{bb} V{b}\" stroke=\"#ffffff\" stroke-opacity=\"0.45\" fill=\"none\" stroke-width=\"1.5\"/>\
                 {sweep}",
                iw = c.w.saturating_sub(32),
                ih = c.h.saturating_sub(32),
                r = c.w - 16,
                r2 = c.w - 40,
                b = c.h - 40,
                bb = c.h - 16,
                dash = if c.gain > 0.01 {
                    "<animate attributeName=\"stroke-dashoffset\" from=\"0\" to=\"36\" dur=\"2.4s\" repeatCount=\"indefinite\"/>"
                } else {
                    ""
                },
            );
    c.finish(c.stack(), &layers)
}

pub(super) fn noise(_art: Art, c: &Canvas) -> String {
    let mut dots = String::new();
    let mut s = Rng::scatter(1);
    for i in 0..140 {
        let x = s.next() % c.w.max(1);
        let y = s.next() % c.h.max(1);
        let o = 0.03 + (s.next() % 12) as f32 / 100.0;
        if c.gain > 0.01 && i % 3 == 0 {
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
    c.finish(c.stack(), &dots)
}

pub(super) fn pulse(_art: Art, c: &Canvas) -> String {
    let (wf, hf) = (c.wf, c.hf);
    let cx = wf * 0.86;
    let cy = hf * 0.5;
    let r0 = hf * 0.08;
    let r1 = hf * 0.28;
    let rings = if c.gain > 0.01 {
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
    c.finish(c.stack(), &rings)
}
