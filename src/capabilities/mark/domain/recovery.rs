//! Recover studio composer state from a public mark URL or studio query.
//!
//! A mark is a pure function of its URL (ADR-0003). The studio is the
//! no-account composer of that URL: loading a public mark locator, a
//! shields badge shorthand, or `/?form=&…` reconstitutes the same fields
//! the composer edits.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::capabilities::mark::domain::spec::MarkForm;

/// Composer fields recovered from a locator. Absent keys stay studio defaults.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StudioBoot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub art: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pill: Option<StudioPillBoot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip: Option<StudioStripBoot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<StudioDeployBoot>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StudioPillBoot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StudioStripBoot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perline: Option<u32>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StudioDeployBoot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

const GRAMMAR_KEYS: &[&str] = &[
    "form",
    "type",
    "theme",
    "color",
    "animation",
    "font",
    "text",
    "desc",
    "width",
    "height",
    "credit",
    "layout",
    "label",
    "message",
    "style",
    "icons",
    "perline",
    "service",
];

/// Parse a public mark URL, badge shorthand, hash locator, or studio query.
///
/// Accepts absolute `https://mark.sylphx.com/…` locators, `/api/v1/mark/{form}?…`,
/// `/badge/{label}-{message}-{color}`, `/?form=&…`, a leading `#`, and an
/// encoded `url=` wrapper. Empty `/` is not recovery.
pub fn parse_public_mark_url(raw: &str) -> Option<StudioBoot> {
    parse_at(raw, 0)
}

/// README image embed for a public mark URL: `![alt](url)`.
pub fn readme_markdown_embed(alt: &str, url: &str) -> String {
    let alt = alt.replace('\\', "\\\\").replace(']', "\\]");
    format!("![{alt}]({url})")
}

/// Shields path tokens: `/badge/{label}-{message}-{color}`.
///
/// `--` is the explicit separator (labels may contain `-`). Otherwise the
/// last two `-` splits are message and color. Underscores become spaces.
pub fn split_badge_path(tail: &str) -> (String, String, Option<String>) {
    if tail.contains("--") {
        let parts: Vec<&str> = tail.split("--").collect();
        (
            decode_token(parts.first().copied().unwrap_or("")),
            decode_token(parts.get(1).copied().unwrap_or("ok")),
            parts.get(2).map(|s| decode_token(s)),
        )
    } else {
        let parts: Vec<&str> = tail.rsplitn(3, '-').collect();
        match parts.len() {
            3 => (
                decode_token(parts[2]),
                decode_token(parts[1]),
                Some(decode_token(parts[0])),
            ),
            2 => (
                String::new(),
                decode_token(parts[1]),
                Some(decode_token(parts[0])),
            ),
            _ => (String::new(), decode_token(tail), None),
        }
    }
}

fn decode_token(s: &str) -> String {
    urlencoding::decode(s)
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| s.to_string())
        .replace('_', " ")
}

fn percent_decode(s: &str) -> String {
    let plus = s.replace('+', " ");
    urlencoding::decode(&plus)
        .map(|c| c.into_owned())
        .unwrap_or(plus)
}

fn decode_text_value(s: &str) -> String {
    percent_decode(s).replace("-nl-", "\n")
}

fn parse_bool_token(v: &str) -> bool {
    matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on")
}

fn parse_at(raw: &str, depth: u8) -> Option<StudioBoot> {
    if depth > 2 {
        return None;
    }
    let (path, query) = split_path_query(raw)?;
    let pairs = query.as_deref().map(query_pairs).unwrap_or_default();

    if depth < 2 {
        if let Some(url) = pairs
            .get("url")
            .map(String::as_str)
            .filter(|u| !u.is_empty())
        {
            if let Some(nested) = parse_at(url, depth + 1) {
                return Some(overlay(nested, &pairs));
            }
        }
    }

    let badge = badge_tail(&path).map(split_badge_path);
    let form = if badge.is_some() {
        Some(MarkForm::Pill)
    } else if let Some(from_path) = mark_form_from_path(&path) {
        Some(from_path)
    } else {
        pairs.get("form").map(|s| MarkForm::parse(Some(s)))
    };

    let has_grammar = form.is_some() || pairs.keys().any(|k| GRAMMAR_KEYS.contains(&k.as_str()));
    if !has_grammar {
        return None;
    }

    let form = form.unwrap_or(MarkForm::Hero);
    let mut boot = StudioBoot {
        form: Some(form.name().to_string()),
        ..StudioBoot::default()
    };
    apply_pairs(&mut boot, form, &pairs);
    if let Some((label, message, color)) = badge {
        let pill = boot.pill.get_or_insert_with(StudioPillBoot::default);
        pill.label = Some(label);
        pill.message = Some(message);
        if boot.theme.as_deref().unwrap_or("").is_empty() {
            if let Some(c) = color {
                boot.color = Some(c);
            }
        }
    }
    Some(boot)
}

fn overlay(mut boot: StudioBoot, pairs: &HashMap<String, String>) -> StudioBoot {
    let form = boot
        .form
        .as_deref()
        .map(|s| MarkForm::parse(Some(s)))
        .unwrap_or(MarkForm::Hero);
    apply_pairs(&mut boot, form, pairs);
    boot
}

fn apply_pairs(boot: &mut StudioBoot, mut form: MarkForm, pairs: &HashMap<String, String>) {
    if let Some(raw) = pairs.get("form") {
        form = MarkForm::parse(Some(raw));
        boot.form = Some(form.name().to_string());
    }
    if let Some(v) = pairs.get("theme").filter(|s| !s.is_empty()) {
        boot.theme = Some(v.clone());
    }
    if boot.theme.as_deref().unwrap_or("").is_empty() {
        if let Some(v) = pairs.get("color") {
            boot.color = Some(v.clone());
        }
    }
    if let Some(v) = pairs.get("type") {
        boot.art = Some(v.clone());
    }
    if let Some(v) = pairs.get("layout") {
        boot.layout = Some(v.clone());
    }
    if let Some(v) = pairs.get("animation") {
        boot.animation = Some(v.clone());
    }
    if let Some(v) = pairs.get("font") {
        boot.font = Some(v.clone());
    }
    if let Some(v) = pairs.get("text") {
        boot.text = Some(decode_text_value(v));
    }
    if let Some(v) = pairs.get("desc") {
        boot.desc = Some(decode_text_value(v));
    }
    if let Some(v) = pairs.get("width").and_then(|s| s.parse().ok()) {
        boot.width = Some(v);
    }
    if let Some(v) = pairs.get("height").and_then(|s| s.parse().ok()) {
        boot.height = Some(v);
    }
    if let Some(v) = pairs.get("credit") {
        boot.credit = Some(parse_bool_token(v));
    }

    match form {
        MarkForm::Pill => {
            let pill = boot.pill.get_or_insert_with(StudioPillBoot::default);
            if let Some(v) = pairs.get("label") {
                pill.label = Some(v.clone());
            }
            if let Some(v) = pairs.get("message") {
                pill.message = Some(v.clone());
            }
            if let Some(v) = pairs.get("style") {
                pill.style = Some(v.clone());
            }
        }
        MarkForm::Strip => {
            let strip = boot.strip.get_or_insert_with(StudioStripBoot::default);
            if let Some(v) = pairs.get("icons") {
                strip.icons = Some(
                    v.split(',')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(ToOwned::to_owned)
                        .collect(),
                );
            }
            if let Some(v) = pairs.get("perline").and_then(|s| s.parse().ok()) {
                strip.perline = Some(v);
            }
        }
        MarkForm::Deploy => {
            let deploy = boot.deploy.get_or_insert_with(StudioDeployBoot::default);
            if let Some(v) = pairs.get("service") {
                deploy.service = Some(v.clone());
            }
            if let Some(v) = pairs.get("style") {
                deploy.style = Some(v.clone());
            }
        }
        MarkForm::Hero | MarkForm::Profile => {}
    }
}

fn split_path_query(raw: &str) -> Option<(String, Option<String>)> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    let s = s.strip_prefix('#').unwrap_or(s).trim();
    if s.is_empty() {
        return None;
    }
    let after_origin = peel_origin(s);
    let (before_hash, hash) = match after_origin.split_once('#') {
        Some((a, b)) => (a, Some(b)),
        None => (after_origin, None),
    };
    let locator = if let Some(hash) = hash {
        if studio_path(before_hash) && looks_like_locator(hash) {
            hash
        } else {
            before_hash
        }
    } else {
        before_hash
    };
    let locator = locator.trim();
    if locator.is_empty() {
        return Some(("/".into(), None));
    }
    if let Some((path, query)) = locator.split_once('?') {
        let path = if path.is_empty() { "/" } else { path };
        Some((path.to_string(), Some(query.to_string())))
    } else if locator.starts_with('/') {
        Some((locator.to_string(), None))
    } else if locator.contains('=') {
        Some(("/".into(), Some(locator.to_string())))
    } else {
        Some((format!("/{locator}"), None))
    }
}

fn peel_origin(s: &str) -> &str {
    let Some(scheme) = s.find("://") else {
        return s;
    };
    let rest = &s[scheme + 3..];
    if let Some(i) = rest.find('/') {
        &rest[i..]
    } else if let Some(i) = rest.find(['?', '#']) {
        &rest[i..]
    } else {
        "/"
    }
}

fn studio_path(path_and_query: &str) -> bool {
    let path = path_and_query.split('?').next().unwrap_or(path_and_query);
    path.is_empty() || path == "/"
}

fn looks_like_locator(hash: &str) -> bool {
    let h = hash.trim().trim_start_matches('#');
    h.starts_with('/')
        || h.starts_with("api/")
        || h.starts_with("badge/")
        || h.contains("://")
        || h.contains('=')
}

fn badge_tail(path: &str) -> Option<&str> {
    let path = path.trim_end_matches('/');
    path.strip_prefix("/badge/").filter(|s| !s.is_empty())
}

fn mark_form_from_path(path: &str) -> Option<MarkForm> {
    let path = path.trim_end_matches('/');
    const PREFIX: &str = "/api/v1/mark";
    if path == PREFIX {
        return Some(MarkForm::Hero);
    }
    path.strip_prefix("/api/v1/mark/")
        .map(|rest| MarkForm::parse(rest.split('/').next()))
}

fn query_pairs(query: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (k, v) = match pair.split_once('=') {
            Some((k, v)) => (k, v),
            None => (pair, ""),
        };
        let key = percent_decode(k);
        if key == "url" {
            let nested = urlencoding::decode(v)
                .map(|c| c.into_owned())
                .unwrap_or_else(|_| v.to_string());
            out.insert(key, nested);
            continue;
        }
        out.insert(key, percent_decode(v));
    }
    out
}
