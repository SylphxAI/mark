//! Inbound adapters for GitHub cards.

mod http;

pub use http::{org_stats_handler, repo_card_handler, user_stats_handler, CardQuery};
