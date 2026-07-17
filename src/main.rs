use mark::routes::{app, AppState};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Image builds must be able to smoke the binary without binding forever.
    // `mark --help` / `mark -V` exit immediately (used by Dockerfile prove step).
    if std::env::args().skip(1).any(|a| a == "--help" || a == "-h" || a == "-V" || a == "--version")
    {
        println!("Sylphx Mark {}", env!("CARGO_PKG_VERSION"));
        println!("Usage: mark");
        println!("  Serves embeddable SVG marks (banners, badges, stats, …).");
        println!("  Env: PORT HOST PUBLIC_BASE_URL DEFAULT_CREDIT RUST_LOG");
        return;
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("mark=info".parse().unwrap()))
        .init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8787);
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let default_credit = std::env::var("DEFAULT_CREDIT")
        .map(|v| matches!(v.as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false);
    let public_base =
        std::env::var("PUBLIC_BASE_URL").unwrap_or_else(|_| format!("http://{host}:{port}"));

    let state = AppState {
        default_credit,
        public_base: public_base.clone(),
    };

    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("invalid HOST:PORT");
    tracing::info!("Sylphx Mark listening on {addr} (base={public_base})");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app(state)).await.expect("serve");
}
