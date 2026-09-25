//! Live inbound adapters: card and dynamic-badge routes.

mod badges;
mod cards;

pub(crate) use badges::{github_badge, last_commit_branch, npm_badge, release_badge};
pub(crate) use cards::{
    card_handler, pin_handler, stats_card, streak_card, streak_handler, top_langs_handler,
    trophy_card, trophy_handler, CardQuery,
};
