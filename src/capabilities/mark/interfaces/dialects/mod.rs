//! Drop-in URL dialects (ADR-0005 decision 4, `MARK-DIALECTS`).
//!
//! Each dialect is a parser at the interface edge: it reads another tool's
//! URL and translates it into the one render kernel. Nothing here draws.

pub(crate) mod capsule;
mod capsule_palette;
pub(crate) mod typing;
