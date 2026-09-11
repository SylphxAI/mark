//! Pane chrome — the floating `glass` panel and `product` card skins.
//!
//! Both skins are the same component (an optional lift animation, two plates,
//! an inner pulse), so the chrome exists once here instead of once per family
//! module; the family functions only bind the skin to their art.

use super::Canvas;
use crate::capabilities::mark::domain::art::Art;

/// The `glass` skin: frosted plate on the glass edge, drifting glow insert.
// duplicate-exception: glass/product are the two skins of one pane skeleton (lift + two plates + inner pulse); scripts/check-module-budget.py notices regression — the read budget forces extraction if the chrome grows.
pub(super) fn glass(c: &Canvas) -> String {
    let (wf, hf) = (c.wf, c.hf);
    let glow = c.plan.glow.as_str();
    let (x, y, rw, rh) = (wf * 0.06, hf * 0.14, wf * 0.88, hf * 0.72);
    let (x2, y2, rw2, rh2) = (wf * 0.1, hf * 0.2, wf * 0.4, hf * 0.2);
    let chrome = if c.gain > 0.01 {
        format!(
                    "<g>\
                       <animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; 0 -3; 0 0; 0 2; 0 0\" dur=\"7s\" repeatCount=\"indefinite\"/>\
                       <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"22\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"url(#glassEdge)\" stroke-width=\"1.2\"/>\
                       <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"18\" fill=\"{glow}\" fill-opacity=\"0.1\">\
                         <animate attributeName=\"fill-opacity\" values=\"0.03;0.08;0.03\" dur=\"4s\" repeatCount=\"indefinite\"/>\
                       </rect>\
                     </g>",
                )
    } else {
        format!(
                    "<rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"22\" fill=\"#ffffff\" fill-opacity=\"0.08\" stroke=\"url(#glassEdge)\" stroke-width=\"1.2\"/>\
                     <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"18\" fill=\"#ffffff\" fill-opacity=\"0.04\"/>",
                )
    };
    c.finish(
        c.stack(),
        &format!("{blobs}{chrome}", blobs = c.cloud(Art::Glass)),
    )
}

/// The `product` skin (shared by `product`/`oss`/`org`): the lifted card with
/// an accent tile.
pub(super) fn product(c: &Canvas) -> String {
    let (wf, hf) = (c.wf, c.hf);
    let accent = c.plan.accent.as_str();
    let glow = c.plan.glow.as_str();
    let (x, y, rw, rh) = (wf * 0.72, hf * 0.22, wf * 0.2, hf * 0.56);
    let (x2, y2, rw2, rh2) = (wf * 0.75, hf * 0.32, wf * 0.14, hf * 0.08);
    let chrome = if c.gain > 0.01 {
        format!(
                    "<g>                       <animateTransform attributeName=\"transform\" type=\"translate\" values=\"0 0; 0 -3; 0 0\" dur=\"4.5s\" repeatCount=\"indefinite\"/>                       <rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"16\" fill=\"{glow}\" fill-opacity=\"0.12\" stroke=\"{accent}\" stroke-opacity=\"0.35\" stroke-width=\"1.2\"/>                       <rect x=\"{x2}\" y=\"{y2}\" width=\"{rw2}\" height=\"{rh2}\" rx=\"8\" fill=\"{accent}\" fill-opacity=\"0.16\">                         <animate attributeName=\"fill-opacity\" values=\"0.1;0.28;0.1\" dur=\"2.8s\" repeatCount=\"indefinite\"/>                       </rect>                     </g>",
                )
    } else {
        format!(
                    "<rect x=\"{x}\" y=\"{y}\" width=\"{rw}\" height=\"{rh}\" rx=\"16\" fill=\"{glow}\" fill-opacity=\"0.12\" stroke=\"{accent}\" stroke-opacity=\"0.3\"/>",
                )
    };
    c.finish(
        c.stack(),
        &format!("{blobs}{chrome}", blobs = c.cloud(Art::Product)),
    )
}
