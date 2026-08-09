//! Clean-break contract tests (ADR-0002): strict SVG attribute grammar,
//! escaping, bounded inputs, and fail-closed GitHub cards.

use std::future::Future;
use std::pin::Pin;

use mark::badge::{self, BadgeInput, BadgeStyle};
use mark::banner::{self, BannerInput};
use mark::brand;
use mark::github_card::{self, CardOpts, GitHubSource};
use mark::icons;
use mark::svg::cap_text;

// ---------- strict attribute grammar ----------

#[test]
fn banner_font_color_cannot_inject_attributes() {
    for evil in [
        "\" onload=\"alert(1)",
        "red\" onmouseover=\"x",
        "#ff0000\"><script>",
        "url(javascript:alert(1))",
        "expression(alert(1))",
        "red;fill:url(#x)",
    ] {
        let svg = banner::render(&BannerInput {
            type_name: Some("soft".into()),
            text: Some("Hi".into()),
            font_color: Some(evil.into()),
            animation: Some("none".into()),
            ..Default::default()
        });
        assert!(!svg.contains("onload="), "fontColor injection: {evil}");
        assert!(!svg.contains("onmouseover="), "fontColor injection: {evil}");
        assert!(!svg.contains("<script"), "fontColor injection: {evil}");
        assert!(!svg.contains("javascript:"), "fontColor injection: {evil}");
    }
}

#[test]
fn banner_stroke_cannot_inject_attributes() {
    for evil in ["\" onload=\"alert(1)", "red\" onmouseover=\"x", "#fff\"><x"] {
        let svg = banner::render(&BannerInput {
            type_name: Some("soft".into()),
            text: Some("Hi".into()),
            stroke: Some(evil.into()),
            stroke_width: Some(2.0),
            animation: Some("none".into()),
            ..Default::default()
        });
        assert!(!svg.contains("onload="), "stroke injection: {evil}");
        assert!(!svg.contains("onmouseover="), "stroke injection: {evil}");
        assert!(!svg.contains("<x"), "stroke injection: {evil}");
    }
}

#[test]
fn banner_accepts_valid_hex_tokens() {
    let svg = banner::render(&BannerInput {
        type_name: Some("soft".into()),
        text: Some("Hi".into()),
        font_color: Some("f00".into()), // 3-digit shorthand expands
        stroke: Some("#00ff00".into()),
        stroke_width: Some(2.0),
        animation: Some("none".into()),
        ..Default::default()
    });
    assert!(svg.contains("#ff0000"), "3-digit shorthand must expand");
    assert!(svg.contains("stroke=\"#00ff00\""), "valid stroke token kept");
}

// ---------- escaping ----------

#[test]
fn banner_text_and_desc_are_escaped() {
    let svg = banner::render(&BannerInput {
        type_name: Some("soft".into()),
        text: Some("<script>alert(1)</script>".into()),
        desc: Some("\" onload=\"x".into()),
        animation: Some("none".into()),
        ..Default::default()
    });
    assert!(!svg.contains("<script>"));
    assert!(svg.contains("&lt;script&gt;"));
}

#[test]
fn badge_label_and_message_are_escaped() {
    let svg = badge::render(&BadgeInput {
        label: Some("<img src=x onerror=alert(1)>".into()),
        message: "\" onload=\"x".into(),
        color: None,
        label_color: None,
        style: BadgeStyle::Flat,
        theme: None,
    });
    // Escaped text is inert: no raw tag/attribute syntax, escaped form present.
    assert!(!svg.contains("<img"), "raw tag must not survive");
    assert!(!svg.contains("onerror=\"") || !svg.contains(" onerror="), "no attribute syntax");
    assert!(svg.contains("&lt;img"));
    assert!(svg.contains("&quot; onload=&quot;x"));
}

#[test]
fn brand_tagline_is_escaped() {
    let svg = brand::render_brand_card("sylphx", Some("<script>x</script>"), false);
    assert!(!svg.contains("<script>"));
    assert!(svg.contains("&lt;script&gt;"));
}

#[test]
fn icon_ids_are_escaped() {
    let svg = icons::render_row("<script>,x", None, 8);
    assert!(!svg.contains("<script>"));
}

#[test]
fn cap_text_marks_truncation_and_stays_within_budget() {
    assert_eq!(cap_text("short", 10), "short");
    let capped = cap_text(&"x".repeat(500), 100);
    assert_eq!(capped.chars().count(), 100);
    assert!(capped.ends_with('…'));
}

// ---------- bounded inputs ----------

#[test]
fn banner_text_is_capped() {
    let svg = banner::render(&BannerInput {
        type_name: Some("soft".into()),
        text: Some("x".repeat(5000)),
        animation: Some("type".into()), // worst-case amplification path
        credit: false,
        ..Default::default()
    });
    assert!(
        svg.len() < 250_000,
        "typewriter output must stay bounded; len={}",
        svg.len()
    );
    assert!(svg.contains('…'), "truncation must be marked");
}

#[test]
fn badge_message_is_capped() {
    let svg = badge::render(&BadgeInput {
        label: Some("l".into()),
        message: "y".repeat(5000),
        color: None,
        label_color: None,
        style: BadgeStyle::Flat,
        theme: None,
    });
    assert!(svg.contains('…'));
    assert!(svg.len() < 5_000, "badge width must stay bounded");
}

#[test]
fn icon_row_is_capped() {
    let svg = icons::render_row(&"rust,ts,docker,".repeat(200), Some("dark"), 12);
    assert!(svg.len() < 40_000, "icon row must stay bounded");
}

// ---------- documented precedence ----------

#[test]
fn badge_theme_defines_palette_over_color() {
    let themed = badge::render(&BadgeInput {
        label: Some("build".into()),
        message: "passing".into(),
        color: Some("red".into()),
        label_color: None,
        style: BadgeStyle::Flat,
        theme: Some("neon".into()),
    });
    let theme_only = badge::render(&BadgeInput {
        label: Some("build".into()),
        message: "passing".into(),
        color: None,
        label_color: None,
        style: BadgeStyle::Flat,
        theme: Some("neon".into()),
    });
    assert_eq!(themed, theme_only, "theme defines the full palette");
}

// ---------- fail-closed GitHub cards ----------

struct FailingReposSource;

impl GitHubSource for FailingReposSource {
    fn get_user<'a>(
        &'a self,
        username: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<mark::capabilities::github_card::domain::GhUser, String>> + Send + 'a>>
    {
        Box::pin(async move {
            Ok(mark::capabilities::github_card::domain::GhUser {
                login: username.into(),
                name: None,
                public_repos: 3,
                followers: 1,
                following: 2,
                bio: None,
            })
        })
    }

    fn get_repo<'a>(
        &'a self,
        _owner: &'a str,
        _repo: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<mark::capabilities::github_card::domain::GhRepo, String>> + Send + 'a>>
    {
        Box::pin(async move { Err("boom".into()) })
    }

    fn get_user_repos<'a>(
        &'a self,
        _username: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<mark::capabilities::github_card::domain::GhRepo>, String>> + Send + 'a>>
    {
        Box::pin(async move { Err("repos failed".into()) })
    }

    fn get_org_repos<'a>(
        &'a self,
        _org: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<mark::capabilities::github_card::domain::GhRepo>, String>> + Send + 'a>>
    {
        Box::pin(async move { Err("org failed".into()) })
    }
}

#[tokio::test]
async fn user_stats_fails_closed_when_repo_fetch_fails() {
    let opts = CardOpts::default();
    let res = github_card::user_stats(&FailingReposSource, "shtse8", &opts).await;
    assert!(
        res.is_err(),
        "must not render a zero-data card as truth: {:?}",
        res
    );
}
