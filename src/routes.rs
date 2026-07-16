//! HTTP routes.

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::badge::{self, BadgeInput, BadgeStyle};
use crate::banner::{self, BannerInput, ANIMATIONS, BANNER_TYPES, FEATURED_TYPES};
use crate::brand;
use crate::icons;
use crate::stats::{self, CardOpts};
use crate::svg::{SVG_CACHE, SVG_CACHE_SHORT};
use crate::themes;

#[derive(Clone)]
pub struct AppState {
    pub default_credit: bool,
    pub public_base: String,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api", get(api_index))
        .route("/api/v1", get(api_index))
        .route("/api/v1/catalog", get(catalog))
        .route("/api/v1/banner", get(banner_handler))
        .route("/banner", get(banner_handler))
        .route("/api/v1/badge", get(badge_query))
        .route("/badge", get(badge_query))
        .route("/badge/{*tail}", get(badge_path))
        .route("/api/v1/stats/{user}", get(user_stats))
        .route("/stats/{user}", get(user_stats))
        .route("/api/v1/org/{org}", get(org_stats))
        .route("/org/{org}", get(org_stats))
        .route("/api/v1/repo/{owner}/{repo}", get(repo_card))
        .route("/repo/{owner}/{repo}", get(repo_card))
        .route("/api/v1/icons", get(icons_handler))
        .route("/icons", get(icons_handler))
        .route("/api/v1/brand/{name}", get(brand_handler))
        .route("/brand/{name}", get(brand_handler))
        .route("/api/v1/deploy", get(deploy_handler))
        .route("/deploy", get(deploy_handler))
        .route("/", get(index_page))
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    Json(json!({
        "ok": true,
        "service": "mark",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn api_index(State(st): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "name": "Sylphx Mark",
        "tagline": "Any URL. One image. Your brand.",
        "base": st.public_base,
        "endpoints": [
            "/api/v1/banner",
            "/api/v1/badge",
            "/badge/{label}-{message}-{color}",
            "/api/v1/stats/{user}",
            "/api/v1/org/{org}",
            "/api/v1/repo/{owner}/{repo}",
            "/api/v1/icons?i=rust,ts,k8s",
            "/api/v1/brand/{name}",
            "/api/v1/deploy",
            "/api/v1/catalog",
            "/health"
        ]
    }))
}

async fn catalog() -> impl IntoResponse {
    Json(json!({
        "banner_types": BANNER_TYPES,
        "featured_banner_types": FEATURED_TYPES,
        "themes": themes::list_names(),
        "icons": icons::available(),
        "badge_styles": ["flat", "plastic", "for-the-badge", "social", "pill"],
        "animations": ANIMATIONS,
    }))
}

#[derive(Debug, Deserialize)]
struct BannerQuery {
    #[serde(rename = "type")]
    type_name: Option<String>,
    color: Option<String>,
    theme: Option<String>,
    section: Option<String>,
    reversal: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
    text: Option<String>,
    desc: Option<String>,
    #[serde(rename = "fontSize")]
    font_size: Option<u32>,
    #[serde(rename = "descSize")]
    desc_size: Option<u32>,
    #[serde(rename = "fontColor")]
    font_color: Option<String>,
    #[serde(rename = "fontAlign")]
    font_align: Option<f32>,
    #[serde(rename = "fontAlignY")]
    font_align_y: Option<f32>,
    #[serde(rename = "descAlign")]
    desc_align: Option<f32>,
    #[serde(rename = "descAlignY")]
    desc_align_y: Option<f32>,
    rotate: Option<f32>,
    stroke: Option<String>,
    #[serde(rename = "strokeWidth")]
    stroke_width: Option<f32>,
    #[serde(rename = "textBg")]
    text_bg: Option<String>,
    animation: Option<String>,
    credit: Option<String>,
}

async fn banner_handler(
    State(st): State<AppState>,
    Query(q): Query<BannerQuery>,
) -> Response {
    let credit = parse_bool(q.credit.as_deref(), st.default_credit);
    let input = BannerInput {
        type_name: q.type_name,
        color: q.color,
        theme: q.theme,
        section: q.section,
        reversal: parse_bool(q.reversal.as_deref(), false),
        height: q.height,
        width: q.width,
        text: q.text.map(decode_text),
        desc: q.desc.map(decode_text),
        font_size: q.font_size,
        desc_size: q.desc_size,
        font_color: q.font_color,
        font_align: q.font_align,
        font_align_y: q.font_align_y,
        desc_align: q.desc_align,
        desc_align_y: q.desc_align_y,
        rotate: q.rotate,
        stroke: q.stroke,
        stroke_width: q.stroke_width,
        text_bg: parse_bool(q.text_bg.as_deref(), false),
        animation: q.animation.clone(),
        credit,
        seed: None,
    };
    // Animated banners must not sit behind long CDN TTL — otherwise deploys look "dead".
    let anim = q.animation.as_deref().unwrap_or("ambient");
    let cache = if anim.eq_ignore_ascii_case("none") || anim.eq_ignore_ascii_case("static") {
        SVG_CACHE
    } else {
        "public, max-age=60, s-maxage=120, stale-while-revalidate=600"
    };
    svg_response(&banner::render(&input), cache)
}

#[derive(Debug, Deserialize)]
struct BadgeQuery {
    label: Option<String>,
    message: Option<String>,
    color: Option<String>,
    #[serde(rename = "labelColor")]
    label_color: Option<String>,
    style: Option<String>,
    theme: Option<String>,
}

async fn badge_query(Query(q): Query<BadgeQuery>) -> Response {
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

async fn badge_path(Path(tail): Path<String>) -> Response {
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

#[derive(Debug, Deserialize)]
struct CardQuery {
    theme: Option<String>,
    color: Option<String>,
    credit: Option<String>,
    width: Option<u32>,
}

async fn user_stats(
    State(st): State<AppState>,
    Path(user): Path<String>,
    Query(q): Query<CardQuery>,
) -> Response {
    let opts = card_opts(&st, &q);
    match stats::user_stats(&user, &opts).await {
        Ok(svg) => svg_response(&svg, SVG_CACHE_SHORT),
        Err(e) => err_svg(&e),
    }
}

async fn org_stats(
    State(st): State<AppState>,
    Path(org): Path<String>,
    Query(q): Query<CardQuery>,
) -> Response {
    let opts = card_opts(&st, &q);
    match stats::org_stats(&org, &opts).await {
        Ok(svg) => svg_response(&svg, SVG_CACHE_SHORT),
        Err(e) => err_svg(&e),
    }
}

async fn repo_card(
    State(st): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(q): Query<CardQuery>,
) -> Response {
    let opts = card_opts(&st, &q);
    match stats::repo_card(&owner, &repo, &opts).await {
        Ok(svg) => svg_response(&svg, SVG_CACHE_SHORT),
        Err(e) => err_svg(&e),
    }
}

#[derive(Debug, Deserialize)]
struct IconsQuery {
    i: Option<String>,
    icons: Option<String>,
    theme: Option<String>,
    #[serde(rename = "perline")]
    per_line: Option<u32>,
}

async fn icons_handler(Query(q): Query<IconsQuery>) -> Response {
    let list = q
        .i
        .or(q.icons)
        .unwrap_or_else(|| "rust,ts,docker".into());
    let svg = icons::render_row(&list, q.theme.as_deref(), q.per_line.unwrap_or(12));
    svg_response(&svg, SVG_CACHE)
}

#[derive(Debug, Deserialize)]
struct BrandQuery {
    tagline: Option<String>,
    credit: Option<String>,
}

async fn brand_handler(
    State(st): State<AppState>,
    Path(name): Path<String>,
    Query(q): Query<BrandQuery>,
) -> Response {
    let credit = parse_bool(q.credit.as_deref(), st.default_credit);
    let svg = brand::render_brand_card(&name, q.tagline.as_deref(), credit);
    svg_response(&svg, SVG_CACHE)
}

#[derive(Debug, Deserialize)]
struct DeployQuery {
    service: Option<String>,
    theme: Option<String>,
    style: Option<String>,
}

async fn deploy_handler(Query(q): Query<DeployQuery>) -> Response {
    let svg = stats::deploy_badge(
        q.service.as_deref().unwrap_or(""),
        q.theme.as_deref(),
        q.style.as_deref().unwrap_or("flat"),
    );
    svg_response(&svg, SVG_CACHE)
}

async fn index_page(State(st): State<AppState>) -> impl IntoResponse {
    // Prefer static/index.html if present
    let path = std::path::Path::new("static/index.html");
    if path.exists() {
        match std::fs::read_to_string(path) {
            Ok(mut html) => {
                html = html.replace("{{BASE}}", &st.public_base);
                return Html(html).into_response();
            }
            Err(_) => {}
        }
    }
    Html(format!(
        r##"<!doctype html><meta charset=utf-8><title>Sylphx Mark</title>
        <body style="font-family:system-ui;background:#0d1117;color:#e6edf3;padding:2rem">
        <h1>Sylphx Mark</h1>
        <p>Any URL. One image. Your brand.</p>
        <p>Base: <code>{}</code></p>
        <p><a href="/api/v1" style="color:#58a6ff">API</a> · <a href="/health" style="color:#58a6ff">Health</a></p>
        </body>"##,
        st.public_base
    ))
    .into_response()
}

fn card_opts(st: &AppState, q: &CardQuery) -> CardOpts {
    CardOpts {
        theme: q.theme.clone().or_else(|| Some("dark".into())),
        color: q.color.clone(),
        credit: parse_bool(q.credit.as_deref(), st.default_credit),
        width: q.width.unwrap_or(420),
    }
}

fn svg_response(svg: &str, cache: &str) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/svg+xml; charset=utf-8"),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_str(cache).unwrap());
    headers.insert(
        header::HeaderName::from_static("access-control-allow-origin"),
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::HeaderName::from_static("cross-origin-resource-policy"),
        HeaderValue::from_static("cross-origin"),
    );
    (headers, svg.to_string()).into_response()
}

fn err_svg(msg: &str) -> Response {
    // Always 200 for embed URLs: CDNs/browsers treat 5xx as a dead <img>.
    // Keep a readable SVG so the failure is visible and cacheable only briefly.
    let safe = crate::svg::esc(msg);
    let short: String = safe.chars().take(120).collect();
    let body = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"480\" height=\"96\" role=\"img\">\
         <rect width=\"100%\" height=\"100%\" rx=\"12\" fill=\"#141821\"/>\
         <text x=\"20\" y=\"38\" fill=\"#ff7b86\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"14\" font-weight=\"600\">Mark couldn&apos;t load this card</text>\
         <text x=\"20\" y=\"62\" fill=\"#9aa3b5\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\">{short}</text>\
         </svg>"
    );
    svg_response(&body, "public, max-age=30")
}

fn parse_bool(v: Option<&str>, default: bool) -> bool {
    match v {
        None => default,
        Some(s) => matches!(s.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
    }
}

fn decode_text(s: String) -> String {
    let decoded = urlencoding::decode(&s).map(|c| c.into_owned()).unwrap_or(s);
    decoded.replace("-nl-", "\n")
}

fn decode_token(s: &str) -> String {
    urlencoding::decode(s)
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| s.to_string())
        .replace('_', " ")
}
