//! Badge capability — shields-style status badges as SVG.
//!
//! Consumer outcome: label/message/color/style parameters become a compact SVG badge.

pub mod application;
pub mod domain;
pub mod interfaces;

pub use application::render;
pub use domain::{BadgeInput, BadgeStyle};
