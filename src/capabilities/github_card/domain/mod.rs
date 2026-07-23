//! GitHub card domain models and pure aggregation.

mod models;
mod opts;

pub use models::{aggregate, Aggregate, GhLicense, GhRepo, GhUser};
pub use opts::CardOpts;
