//! GitHub card application layer: ports, pure render, and use cases.

pub mod features;
pub mod ports;
mod render;

pub use features::{org_stats, repo_card, user_stats};
pub use ports::GitHubSource;
pub use render::{render_org_card, render_repo_card, render_user_card};
