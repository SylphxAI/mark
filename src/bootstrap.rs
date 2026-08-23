//! Composition root: configuration, process state, and server wiring.
//!
//! Binds stable ports to adapters and owns process lifecycle. Domain modules
//! never locate dependencies through this module.

use std::sync::OnceLock;
use crate::interfaces::http::app;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

/// Process-level shell state shared with HTTP handlers.
///
/// Mark is stateless by design (ADR-0003): the only process state is product
/// defaults and the canonical base URL. There is no upstream, no cache, no
/// secret — every mark is a pure function of its URL.
#[derive(Clone)]
pub struct AppState {
    pub default_credit: bool,
    pub public_base: String,
}

/// Runtime configuration loaded from the environment (imperative shell).
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub default_credit: bool,
    pub public_base: String,
}

impl Config {
    pub fn from_env() -> Self {
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
        Self {
            host,
            port,
            default_credit,
            public_base,
        }
    }

    pub fn state(&self) -> AppState {
        AppState {
            default_credit: self.default_credit,
            public_base: self.public_base.clone(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("invalid HOST:PORT")
    }
}

/// Print CLI help/version and exit without binding (Docker prove step).
pub fn maybe_print_cli_and_exit() -> bool {
    if std::env::args()
        .skip(1)
        .any(|a| a == "--help" || a == "-h" || a == "-V" || a == "--version")
    {
        println!(
            "Sylphx Mark {} (rev {})",
            env!("CARGO_PKG_VERSION"),
            build_revision()
        );
        println!("Usage: mark");
        println!("  Serves embeddable SVG marks (hero, pill, strip, profile, deploy).");
        println!("  Env: PORT HOST PUBLIC_BASE_URL DEFAULT_CREDIT RUST_LOG");
        return true;
    }
    false
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("mark=info".parse().unwrap()))
        .init();
}

/// Bind and serve the HTTP composition root.
pub async fn serve(config: Config) {
    let state = config.state();
    let addr = config.addr();
    tracing::info!(
        "Sylphx Mark listening on {addr} (base={})",
        config.public_base
    );
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app(state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("serve");
}

/// Drain in-flight requests on SIGTERM/SIGINT before exiting (rolling deploy).
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("install SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("install SIGINT handler");
        tokio::select! {
            _ = sigterm.recv() => {}
            _ = sigint.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
    tracing::info!("shutdown signal received; draining in-flight requests");
}

/// Process/build revision for liveness metadata (not product capability proof).
pub fn build_revision() -> &'static str {
    static REV: OnceLock<String> = OnceLock::new();
    REV.get_or_init(|| {
        // Runtime env wins (platform may inject after image build).
        for key in [
            "GIT_SHA",
            "SOURCE_COMMIT",
            "SYLPHX_GIT_COMMIT_SHA",
            "SYLPHX_GIT_SHA",
            "COMMIT_SHA",
        ] {
            if let Ok(v) = std::env::var(key) {
                let v = v.trim().to_string();
                if !v.is_empty() && v != "unknown" {
                    return v;
                }
            }
        }
        // Compile-time embed from build.rs (git HEAD or build-arg).
        option_env!("MARK_GIT_SHA").unwrap_or("unknown").to_string()
    })
    .as_str()
}
