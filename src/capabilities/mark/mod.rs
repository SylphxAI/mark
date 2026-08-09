//! Mark capability — the one concept (ADR-0003).
//!
//! Consumer outcome: URL parameters become a beautiful, deterministic, branded
//! SVG mark — hero, pill, strip, identity, or deploy — from one grammar.

pub mod application;
pub mod domain;
pub mod interfaces;

pub use application::render;
pub use domain::{MarkForm, MarkSpec};
