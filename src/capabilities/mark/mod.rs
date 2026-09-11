//! Mark capability — the one concept (ADR-0003).
//!
//! Consumer outcome: URL parameters become a beautiful, deterministic, branded
//! SVG mark — hero, pill, strip, profile, or deploy — from one grammar.

pub mod domain;

pub(crate) mod application;
pub(crate) mod interfaces;

/// Render any mark: the capability's single use case.
pub use application::render;
