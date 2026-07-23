//! Generator UI surface.

use axum::extract::State;
use axum::response::{Html, IntoResponse, Response};

use crate::bootstrap::AppState;

pub async fn index_page(State(st): State<AppState>) -> Response {
    let path = std::path::Path::new("static/index.html");
    if path.exists() {
        if let Ok(mut html) = std::fs::read_to_string(path) {
            html = html.replace("{{BASE}}", &st.public_base);
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
