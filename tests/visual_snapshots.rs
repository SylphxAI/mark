//! Visual regression snapshots and the URL-forever contract.
//!
//! Every `tests/snapshots/<case>.url` holds one public path (with query). The
//! test GETs it through the real router, requires `200 image/svg+xml`, and
//! compares the bytes with `tests/snapshots/<case>.svg`.
//!
//! - A changed render fails with the case name; review the new SVG, then
//!   refresh with `UPDATE_SNAPSHOTS=1 cargo test --test visual_snapshots`.
//! - A missing `.svg` fails unless `UPDATE_SNAPSHOTS=1` is set, so a case can
//!   never pass unrecorded.
//! - Cases named `legacy-*` are URLs that were public before ADR-0005. They
//!   are embedded in READMEs we do not control and must answer forever; their
//!   bytes may change (a polish), their status and content type may not.
//!
//! One file pair per case keeps parallel changes free of merge conflicts.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use mark::{app, AppState};
use std::path::{Path, PathBuf};
use tower::ServiceExt;

fn cases() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots");
    let mut urls: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("tests/snapshots exists")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "url"))
        .collect();
    urls.sort();
    urls
}

async fn get(path: &str) -> (StatusCode, String, String) {
    let res = app(AppState::for_tests())
        .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let ctype = res
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let body = res.into_body().collect().await.unwrap().to_bytes();
    (status, ctype, String::from_utf8_lossy(&body).into_owned())
}

#[tokio::test]
async fn every_snapshot_url_renders_its_recorded_svg() {
    let update = std::env::var_os("UPDATE_SNAPSHOTS").is_some();
    let cases = cases();
    assert!(!cases.is_empty(), "no snapshot cases recorded");
    let mut failures = Vec::new();
    for url_file in cases {
        let name = url_file.file_stem().unwrap().to_string_lossy().into_owned();
        let path = std::fs::read_to_string(&url_file).unwrap();
        let path = path.trim();
        let (status, ctype, body) = get(path).await;
        if status != StatusCode::OK || !ctype.starts_with("image/svg+xml") {
            failures.push(format!("{name}: {path} -> {status} {ctype}"));
            continue;
        }
        let svg_file = url_file.with_extension("svg");
        if update {
            std::fs::write(&svg_file, &body).unwrap();
            continue;
        }
        match std::fs::read_to_string(&svg_file) {
            Ok(want) if want == body => {}
            Ok(_) => failures.push(format!("{name}: render changed ({path})")),
            Err(_) => failures.push(format!("{name}: no recorded snapshot ({path})")),
        }
    }
    assert!(
        failures.is_empty(),
        "snapshot failures (refresh with UPDATE_SNAPSHOTS=1 after review):\n{}",
        failures.join("\n")
    );
}
