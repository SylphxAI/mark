//! Shared kernel used by multiple capabilities.
//!
//! These modules are not capabilities: they own reusable pure primitives
//! (color planning, theme catalog, SVG document helpers) with no product
//! outcome of their own.

pub mod color;
pub mod svg;
pub mod theme;
