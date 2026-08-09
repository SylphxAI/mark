//! Architecture boundary tests for the one-capability / one-grammar end state
//! (ADR-0003): domain purity, no clock, no upstream, no legacy surface.

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
fn mark_domain_is_framework_free_and_clock_free() {
    let roots = ["src/capabilities/mark/domain"];
    let forbidden = [
        "use axum",
        "use reqwest",
        "use tokio",
        "use tower",
        "std::env::",
        "reqwest::",
        "axum::",
        "chrono",
        "Utc::now",
        "timeAuto",
        "timeGradient",
        "clock_seed",
    ];
    for root in roots {
        let files = rust_files_under(Path::new(root));
        assert!(!files.is_empty(), "expected rust files under {root}");
        assert_no_forbidden_imports(&files, &forbidden, root);
    }
}

#[test]
fn mark_application_is_pure() {
    let files = rust_files_under(Path::new("src/capabilities/mark/application"));
    assert!(!files.is_empty(), "expected application files");
    let forbidden = [
        "use axum",
        "use reqwest",
        "use tokio",
        "reqwest::",
        "axum::",
        "std::env::",
        "chrono",
        "Utc::now",
    ];
    assert_no_forbidden_imports(&files, &forbidden, "application");
}

#[test]
fn shell_owns_the_only_surface() {
    let shell = rust_files_under(Path::new("src/interfaces"));
    assert_no_forbidden_imports(&shell, &["timeAuto", "timeGradient", "Utc::now"], "shell");
    let http = fs::read_to_string("src/interfaces/http/mod.rs").unwrap();
    assert!(http.contains("/api/v1/mark"), "single mark surface");
    assert!(http.contains("/badge/{*tail}"), "shields pill path kept");
    for retired in [
        "/api/v1/banner",
        "/api/v1/badge",
        "/api/v1/icons",
        "/api/v1/brand",
        "/api/v1/deploy",
        "/api/v1/stats",
        "/api/v1/org",
        "/api/v1/repo",
    ] {
        assert!(!http.contains(retired), "retired route still present: {retired}");
    }
}

#[test]
fn one_capability_and_no_retired_trees() {
    let caps = fs::read_dir("src/capabilities").unwrap();
    let dirs: Vec<String> = caps
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(dirs, vec!["mark"], "only the mark capability may exist: {dirs:?}");
    for retired in [
        "src/shared",
        "src/capabilities/badge",
        "src/capabilities/banner",
        "src/capabilities/icon_row",
        "src/capabilities/brand_kit",
        "src/capabilities/deploy_mark",
        "src/capabilities/github_card",
        "src/routes.rs",
        "src/badge.rs",
        "src/stats.rs",
        "src/github.rs",
        "src/brand.rs",
        "src/icons.rs",
        "src/color.rs",
        "src/themes.rs",
        "src/svg.rs",
    ] {
        assert!(!Path::new(retired).exists(), "retired path still present: {retired}");
    }
}

#[test]
fn every_advertised_art_type_has_a_shape_arm() {
    let shapes = fs::read_to_string("src/capabilities/mark/domain/shapes.rs").unwrap();
    let body = &shapes[shapes.find("pub fn shape_background").expect("shape_background")..];
    let block = &shapes[shapes.find("ART_TYPES").expect("ART_TYPES")..];
    let block = &block[..block.find("];").expect("end of ART_TYPES")];
    let mut advertised = Vec::new();
    for part in block.split('"') {
        if !part.is_empty()
            && part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        {
            advertised.push(part);
        }
    }
    assert!(!advertised.is_empty(), "parsed art type catalog");
    for ty in advertised {
        assert!(
            body.contains(&format!("\"{ty}\"")),
            "art type {ty} advertised but missing a shape arm"
        );
    }
}

#[test]
fn grammar_is_single_and_total() {
    let spec = fs::read_to_string("src/capabilities/mark/domain/spec.rs").unwrap();
    for concept in ["MarkSpec", "MarkForm", "HeroSpec", "PillSpec", "StripSpec", "IdentitySpec", "DeploySpec"] {
        assert!(spec.contains(concept), "missing grammar concept {concept}");
    }
    let render = fs::read_to_string("src/capabilities/mark/application/render.rs").unwrap();
    for form in ["Hero", "Pill", "Strip", "Identity", "Deploy"] {
        assert!(
            render.contains(&format!("MarkForm::{form}")),
            "render must dispatch form {form}"
        );
    }
    // Total by construction: no error path exists in the response shell.
    let response = fs::read_to_string("src/interfaces/http/response.rs").unwrap();
    assert!(
        !response.contains("err_svg") && !response.contains("current_time_seed"),
        "no error/clock shell may exist"
    );
}

#[test]
fn no_upstream_dependencies() {
    let manifest = fs::read_to_string("Cargo.toml").unwrap();
    let deps = &manifest[manifest.find("[dependencies]").unwrap()
        ..manifest.find("[dev-dependencies]").unwrap()];
    for forbidden in ["reqwest", "moka", "chrono", "once_cell"] {
        assert!(
            !deps.contains(forbidden),
            "upstream/clock dependency still declared: {forbidden}"
        );
    }
}

#[test]
fn determinism_contract_is_enforced_in_catalog() {
    let catalog = fs::read_to_string("src/capabilities/mark/domain/catalog.rs").unwrap();
    assert!(catalog.contains("MAX_TEXT_CHARS"), "limits contract present");
    let interfaces = fs::read_to_string("src/capabilities/mark/interfaces/http.rs").unwrap();
    assert!(interfaces.contains("normalize_hex_token") || interfaces.contains("parse_bool"));
}
