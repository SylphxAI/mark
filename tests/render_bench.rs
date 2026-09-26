//! Render latency benchmark (ignored by default; CI runs it in release mode).
//!
//! `cargo test --release --test render_bench -- --ignored --nocapture`
//! prints a markdown table of p50/p95 in-process latency per URL: the full
//! router path (query parsing, render, headers), without network. Each URL is
//! warmed, then timed over `RUNS` sequential requests.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use mark::{app, AppState};
use std::time::Instant;
use tower::ServiceExt;

const RUNS: usize = 400;

const URLS: &[&str] = &[
    "/badge/build-passing-brightgreen",
    "/badge/build-passing-brightgreen?style=for-the-badge",
    "/api/v1/mark/hero?type=waving&text=Hello%20README&desc=One%20URL",
    "/api/v1/mark/hero?type=aurora&text=Aurora&animation=fade",
    "/api/v1/mark/hero?type=aurora&text=Stars&width=1200&height=320",
    "/api/v1/mark/strip?icons=rust,ts,docker,kubernetes,postgres,react,go,python",
    "/api/v1/mark/profile?text=Ada%20Lovelace&desc=Analytical%20engines",
    "/api/v1/mark/deploy?service=mark",
];

fn pct(sorted: &[f64], p: f64) -> f64 {
    let i = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[i]
}

#[tokio::test(flavor = "current_thread")]
#[ignore = "benchmark; run explicitly in release mode"]
async fn render_latency() {
    let router = app(AppState::for_tests());
    println!("| URL | bytes | p50 (µs) | p95 (µs) |");
    println!("| --- | ---: | ---: | ---: |");
    for url in URLS {
        let mut samples = Vec::with_capacity(RUNS);
        let mut bytes = 0;
        for i in 0..RUNS + 20 {
            let req = Request::builder().uri(*url).body(Body::empty()).unwrap();
            let start = Instant::now();
            let res = router.clone().oneshot(req).await.unwrap();
            assert_eq!(res.status(), StatusCode::OK, "{url}");
            let body = res.into_body().collect().await.unwrap().to_bytes();
            let elapsed = start.elapsed().as_secs_f64() * 1e6;
            bytes = body.len();
            if i >= 20 {
                samples.push(elapsed);
            }
        }
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "| `{url}` | {bytes} | {:.0} | {:.0} |",
            pct(&samples, 0.5),
            pct(&samples, 0.95)
        );
    }
}
