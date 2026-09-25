//! readme-typing-svg dialect: `/?lines=…` (host swap) and `/typing?lines=…`.
//!
//! DenverCoder1/readme-typing-svg serves its SVG from the site root, so a
//! README switches by replacing `readme-typing-svg.demolab.com` with Mark's
//! host. The parameter set and its defaults live in
//! [`TypingSpec::from_pairs`].

use std::collections::HashMap;

use crate::capabilities::mark::domain::{MarkForm, MarkSpec, TypingSpec};
use crate::capabilities::mark::render;

/// A root query is a typing URL when it names `lines` (upstream's own rule:
/// without `lines` it shows its demo page, as Mark shows the studio).
pub(crate) fn claims(pairs: &HashMap<String, String>) -> bool {
    pairs.contains_key("lines")
}

pub(crate) fn svg(pairs: &HashMap<String, String>, query: &str) -> String {
    render(&MarkSpec {
        form: MarkForm::Typing,
        typing: TypingSpec::from_pairs(pairs, query),
        ..Default::default()
    })
}
