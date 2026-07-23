//! GitHub card capability — user / org / repo stats cards as SVG.
//!
//! Consumer outcome: a GitHub identity or repository becomes a themed stats card.
//! Upstream network I/O is confined to adapters behind `GitHubSource`.

pub mod adapters;
pub mod application;
pub mod domain;
pub mod interfaces;

pub use application::{org_stats, repo_card, user_stats, GitHubSource};
pub use domain::CardOpts;
pub use adapters::HttpGitHubSource;
