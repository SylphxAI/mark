//! Badge HTTP surface.

use axum::extract::{Path, Query};
use axum::response::Response;
use serde::Deserialize;

use crate::capabilities::badge::{self, BadgeInput, BadgeStyle};
use crate::interfaces::http::response::{decode_token, svg_response};
use crate::shared::svg::SVG_CACHE;

#[derive(Debug, Deserialize)]
pub struct BadgeQuery {
    pub label: Option<String>,
    pub message: Option<String>,
    pub color: Option<String>,
    #[serde(rename = "labelColor")]
    pub label_color: Option<String>,
    pub style: Option<String>,
    pub theme: Option<String>,
}

pub async fn badge_query(Query(q): Query<BadgeQuery>) -> Response {
    let svg = badge::render(&BadgeInput {
        label: q.label,
        message: q.message.unwrap_or_else(|| "ok".into()),
        color: q.color,
        label_color: q.label_color,
        style: BadgeStyle::parse(q.style.as_deref().unwrap_or("flat")),
        theme: q.theme,
    });
    svg_response(&svg, SVG_CACHE)
}

pub async fn badge_path(Path(tail): Path<String>) -> Response {
    // Support: label-message-color OR label--message--color
    let input = if tail.contains("--") {
        let parts: Vec<&str> = tail.split("--").collect();
        BadgeInput {
            label: parts.first().map(|s| decode_token(s)),
            message: parts
                .get(1)
                .map(|s| decode_token(s))
                .unwrap_or_else(|| "ok".into()),
            color: parts.get(2).map(|s| decode_token(s)),
            label_color: None,
            style: BadgeStyle::Flat,
            theme: None,
        }
    } else {
        // crude: split from right for color
        let parts: Vec<&str> = tail.rsplitn(3, '-').collect();
        // rsplitn gives reverse order
        match parts.len() {
            3 => BadgeInput {
                label: Some(decode_token(parts[2])),
                message: decode_token(parts[1]),
                color: Some(decode_token(parts[0])),
                label_color: None,
                style: BadgeStyle::Flat,
                theme: None,
            },
            2 => BadgeInput {
                label: None,
                message: decode_token(parts[1]),
                color: Some(decode_token(parts[0])),
                label_color: None,
                style: BadgeStyle::Flat,
                theme: None,
            },
            _ => BadgeInput {
                label: None,
                message: decode_token(&tail),
                color: None,
                label_color: None,
                style: BadgeStyle::Flat,
                theme: None,
            },
        }
    };
    svg_response(&badge::render(&input), SVG_CACHE)
}
