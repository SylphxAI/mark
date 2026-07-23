//! Architecture boundary tests for Capability-first / FCIS invariants.
//!
//! These are cheap static proofs that domain/application pure cores do not
//! depend on framework or network crates.

use std::fs;
use std::path::{Path, PathBuf};

fn rust_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            out.extend(rust_files_under(&path));
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
    out
}

fn assert_no_forbidden_imports(files: &[PathBuf], forbidden: &[&str], label: &str) {
    for path in files {
        let text = fs::read_to_string(path).unwrap();
        for needle in forbidden {
            assert!(
                !text.contains(needle),
                "{label} must not contain `{needle}`: {}",
                path.display()
            );
        }
    }
}

#[test]
fn domain_modules_are_framework_free() {
    let roots = [
        "src/capabilities/banner/domain",
        "src/capabilities/badge/domain",
        "src/capabilities/github_card/domain",
        "src/capabilities/icon_row/domain",
        "src/shared",
    ];
    let forbidden = [
        "use axum",
        "use reqwest",
        "use tokio",
        "use tower",
        "std::env::",
        "reqwest::",
        "axum::",
    ];
    for root in roots {
        let files = rust_files_under(Path::new(root));
        assert!(!files.is_empty(), "expected rust files under {root}");
        assert_no_forbidden_imports(&files, &forbidden, root);
    }
}

#[test]
fn pure_application_render_modules_are_framework_free() {
    let files = [
        "src/capabilities/banner/application/render.rs",
        "src/capabilities/badge/application/render.rs",
        "src/capabilities/github_card/application/render.rs",
        "src/capabilities/icon_row/application/render.rs",
        "src/capabilities/brand_kit/application/render.rs",
        "src/capabilities/deploy_mark/application/render.rs",
    ];
    let forbidden = [
        "use axum",
        "use reqwest",
        "use tokio",
        "reqwest::",
        "axum::",
        "std::env::",
    ];
    for f in files {
        let path = Path::new(f);
        assert!(path.exists(), "missing {f}");
        let text = fs::read_to_string(path).unwrap();
        for needle in forbidden {
            assert!(
                !text.contains(needle),
                "pure render {f} must not contain `{needle}`"
            );
        }
    }
}

#[test]
fn github_network_effects_live_only_in_adapter() {
    let adapter = fs::read_to_string("src/capabilities/github_card/adapters/github_http.rs").unwrap();
    assert!(adapter.contains("reqwest"), "adapter must own HTTP client");
    assert!(
        adapter.contains("impl GitHubSource"),
        "adapter must implement application port"
    );

    let domain = rust_files_under(Path::new("src/capabilities/github_card/domain"));
    assert_no_forbidden_imports(&domain, &["reqwest", "moka", "once_cell"], "github domain");

    let features = fs::read_to_string("src/capabilities/github_card/application/features.rs").unwrap();
    assert!(
        features.contains("GitHubSource"),
        "use cases depend on port, not concrete client"
    );
    assert!(!features.contains("reqwest"), "use cases must not call reqwest");
}

#[test]
fn capabilities_directory_owns_product_outcomes() {
    for cap in [
        "banner",
        "badge",
        "github_card",
        "icon_row",
        "brand_kit",
        "deploy_mark",
    ] {
        let root = Path::new("src/capabilities").join(cap);
        assert!(root.join("mod.rs").exists(), "missing capability root {cap}");
    }
    // Retired flat modules must not return
    for retired in [
        "src/routes.rs",
        "src/badge.rs",
        "src/stats.rs",
        "src/github.rs",
        "src/brand.rs",
        "src/icons.rs",
        "src/color.rs",
        "src/themes.rs",
        "src/svg.rs",
        "src/banner/mod.rs",
    ] {
        assert!(
            !Path::new(retired).exists(),
            "retired path still present: {retired}"
        );
    }
}

#[test]
fn deploy_mark_is_not_owned_by_github_card() {
    let stats = fs::read_to_string("src/capabilities/github_card/application/render.rs").unwrap();
    assert!(
        !stats.contains("deploy_badge") && !stats.contains("deployed on"),
        "deploy promo must not live under github_card render"
    );
    assert!(
        Path::new("src/capabilities/deploy_mark/application/render.rs").exists(),
        "deploy_mark capability missing"
    );
}

#[test]
fn composition_root_binds_github_adapter() {
    let boot = std::fs::read_to_string("src/bootstrap.rs").unwrap();
    assert!(
        boot.contains("HttpGitHubSource"),
        "bootstrap must bind GitHub adapter into AppState"
    );
    let http = std::fs::read_to_string("src/capabilities/github_card/interfaces/http.rs").unwrap();
    assert!(
        http.contains("st.github"),
        "HTTP interface should use injected adapter, not construct it inline"
    );
    assert!(
        !http.contains("HttpGitHubSource"),
        "HTTP interface must not name the concrete adapter type"
    );
}

#[test]
fn shared_kernel_has_no_process_clock() {
    let theme = std::fs::read_to_string("src/shared/theme.rs").unwrap();
    let color = std::fs::read_to_string("src/shared/color.rs").unwrap();
    for (label, text) in [("theme", &theme), ("color", &color)] {
        assert!(
            !text.contains("Utc::now") && !text.contains("std::env::"),
            "{label} kernel must not sample process clock/env"
        );
        assert!(
            !text.contains("use chrono") && !text.contains("chrono::"),
            "{label} kernel must stay free of chrono; shell formats time seeds"
        );
    }
    let shell = std::fs::read_to_string("src/interfaces/http/response.rs").unwrap();
    assert!(
        shell.contains("current_time_seed") && shell.contains("Utc::now"),
        "HTTP shell must own clock sampling"
    );
}

#[test]
fn unused_effect_crates_are_not_reintroduced_in_domain() {
    // Guard: domain remains free of error/framework crates even if deps return.
    let domain_roots = [
        "src/capabilities/banner/domain",
        "src/capabilities/badge/domain",
        "src/capabilities/github_card/domain",
        "src/capabilities/icon_row/domain",
    ];
    for root in domain_roots {
        for path in rust_files_under(Path::new(root)) {
            let text = std::fs::read_to_string(&path).unwrap();
            for needle in ["anyhow", "thiserror", "axum", "reqwest"] {
                assert!(
                    !text.contains(needle),
                    "{} must not reference {needle}",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn github_domain_is_serde_free() {
    let models = std::fs::read_to_string("src/capabilities/github_card/domain/models.rs").unwrap();
    assert!(
        !models.contains("serde") && !models.contains("Deserialize"),
        "github domain models must not depend on serde; adapter owns wire DTOs"
    );
    let adapter = std::fs::read_to_string("src/capabilities/github_card/adapters/github_http.rs").unwrap();
    assert!(
        adapter.contains("GhUserDto") && adapter.contains("Deserialize"),
        "adapter must own wire DTOs"
    );
}
