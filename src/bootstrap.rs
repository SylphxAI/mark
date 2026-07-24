//! Composition root: configuration, process state, and server wiring.
//!
//! Binds stable ports to adapters and owns process lifecycle. Domain modules
//! never locate dependencies through this module.

use crate::capabilities::github_card::HttpGitHubSource;
use std::sync::OnceLock;
use crate::interfaces::http::app;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

/// Process-level shell state shared with HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub default_credit: bool,
    pub public_base: String,
    /// Bound GitHub outbound adapter (composition root wiring).
    pub github: HttpGitHubSource,
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
            github: HttpGitHubSource,
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
        println!("Sylphx Mark {}", env!("CARGO_PKG_VERSION"));
        println!("Usage: mark");
        println!("  Serves embeddable SVG marks (banners, badges, stats, …).");
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
    axum::serve(listener, app(state)).await.expect("serve");
}

/// Process/build revision for liveness metadata (not product capability proof).
pub fn build_revision() -> &'static str {
    static REV: OnceLock<String> = OnceLock::new();
    REV.get_or_init(|| {
        // Runtime env wins (platform may inject after image build).
        for key in ["GIT_SHA", "SOURCE_COMMIT", "SYLPHX_GIT_SHA", "COMMIT_SHA"] {
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
