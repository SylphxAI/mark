//! Generator UI surface.

use axum::extract::State;
use axum::http::Uri;
use axum::response::{Html, IntoResponse, Response};
use serde::Serialize;

use crate::bootstrap::AppState;
use crate::capabilities::mark::domain::recovery::parse_public_mark_url;

pub async fn index_page(State(st): State<AppState>, uri: Uri) -> Response {
    let locator = match uri.query() {
        Some(q) => format!("{}?{q}", uri.path()),
        None => uri.path().to_string(),
    };
    let boot_json = match parse_public_mark_url(&locator) {
        Some(boot) => json_for_script(&boot),
        None => "null".to_string(),
    };
    let path = std::path::Path::new("static/index.html");
    if path.exists() {
        if let Ok(mut html) = std::fs::read_to_string(path) {
            html = html.replace("{{BASE}}", &st.public_base);
            html = html.replace("{{BOOT}}", &boot_json);
            return Html(html).into_response();
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

fn json_for_script(value: &impl Serialize) -> String {
    let json = serde_json::to_string(value).unwrap_or_else(|_| "null".into());
    json.replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}
