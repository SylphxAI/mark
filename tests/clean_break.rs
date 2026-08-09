//! Clean-break contract tests (ADR-0003): strict SVG attribute grammar,
//! escaping, bounded inputs, determinism — no legacy, no clock, no upstream.

use mark::capabilities::mark::domain::{MarkSpec, PillSpec, StripSpec};
use mark::mark::{render, MarkForm};
use mark::svg::cap_text;

fn hero(ty: &str, text: &str) -> MarkSpec {
    MarkSpec {
        form: MarkForm::Hero,
        art: Some(ty.into()),
        text: Some(text.into()),
        ..Default::default()
    }
}

// ---------- strict attribute grammar ----------

#[test]
fn hero_font_color_cannot_inject_attributes() {
    for evil in [
        "\" onload=\"alert(1)",
        "red\" onmouseover=\"x",
        "#ff0000\"><script>",
        "url(javascript:alert(1))",
        "expression(alert(1))",
        "red;fill:url(#x)",
    ] {
        let mut spec = hero("soft", "Hi");
        spec.hero.font_color = Some(evil.into());
        spec.animation = Some("none".into());
        let svg = render(&spec);
        assert!(!svg.contains("onload="), "fontColor injection: {evil}");
        assert!(!svg.contains("onmouseover="), "fontColor injection: {evil}");
        assert!(!svg.contains("<script"), "fontColor injection: {evil}");
        assert!(!svg.contains("javascript:"), "fontColor injection: {evil}");
    }
}

#[test]
fn hero_stroke_cannot_inject_attributes() {
    for evil in ["\" onload=\"alert(1)", "red\" onmouseover=\"x", "#fff\"><x"] {
        let mut spec = hero("soft", "Hi");
        spec.hero.stroke = Some(evil.into());
        spec.hero.stroke_width = Some(2.0);
        spec.animation = Some("none".into());
        let svg = render(&spec);
        assert!(!svg.contains("onload="), "stroke injection: {evil}");
        assert!(!svg.contains("onmouseover="), "stroke injection: {evil}");
        assert!(!svg.contains("<x"), "stroke injection: {evil}");
    }
}

#[test]
fn hero_accepts_valid_hex_tokens() {
    let mut spec = hero("soft", "Hi");
    spec.hero.font_color = Some("f00".into());
    spec.hero.stroke = Some("#00ff00".into());
    spec.hero.stroke_width = Some(2.0);
    spec.animation = Some("none".into());
    let svg = render(&spec);
    assert!(svg.contains("#ff0000"), "3-digit shorthand must expand");
    assert!(svg.contains("stroke=\"#00ff00\""), "valid stroke token kept");
}

// ---------- escaping across every form ----------

#[test]
fn hero_text_and_desc_are_escaped() {
    let mut spec = hero("soft", "<script>alert(1)</script>");
    spec.desc = Some("\" onload=\"x".into());
    spec.animation = Some("none".into());
    let svg = render(&spec);
    assert!(!svg.contains("<script>"));
    assert!(svg.contains("&lt;script&gt;"));
}

#[test]
fn pill_label_and_message_are_escaped() {
    let spec = MarkSpec {
        form: MarkForm::Pill,
        pill: PillSpec {
            label: Some("<img src=x onerror=alert(1)>".into()),
            message: Some("\" onload=\"x".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(!svg.contains("<img"), "raw tag must not survive");
    assert!(svg.contains("&lt;img"));
    assert!(svg.contains("&quot; onload=&quot;x"));
}

#[test]
fn profile_name_and_tagline_are_escaped() {
    let spec = MarkSpec {
        form: MarkForm::Profile,
        text: Some("<script>x</script>".into()),
        desc: Some("\" onload=\"x".into()),
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(!svg.contains("<script>"));
    assert!(svg.contains("&lt;script&gt;"));
}

#[test]
fn strip_ids_are_escaped() {
    let spec = MarkSpec {
        form: MarkForm::Strip,
        strip: StripSpec {
            icons: Some("<script>,x".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    let svg = render(&spec);
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
fn hero_text_is_capped() {
    let mut spec = hero("soft", &"x".repeat(5000));
    spec.animation = Some("type".into()); // worst-case amplification path
    let svg = render(&spec);
    assert!(
        svg.len() < 250_000,
        "typewriter output must stay bounded; len={}",
        svg.len()
    );
    assert!(svg.contains('…'), "truncation must be marked");
}

#[test]
fn pill_message_is_capped() {
    let spec = MarkSpec {
        form: MarkForm::Pill,
        pill: PillSpec {
            label: Some("l".into()),
            message: Some("y".repeat(5000)),
            ..Default::default()
        },
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(svg.contains('…'));
    assert!(svg.len() < 5_000, "pill width must stay bounded");
}

#[test]
fn strip_is_capped() {
    let spec = MarkSpec {
        form: MarkForm::Strip,
        strip: StripSpec {
            icons: Some("rust,ts,docker,".repeat(200)),
            ..Default::default()
        },
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(svg.len() < 40_000, "strip must stay bounded");
}

#[test]
fn profile_text_and_tagline_are_capped() {
    let spec = MarkSpec {
        form: MarkForm::Profile,
        text: Some("x".repeat(2000)),
        desc: Some("y".repeat(2000)),
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(svg.len() < 20_000, "profile must stay bounded");
    assert!(svg.contains('…'));
}

#[test]
fn pill_theme_defines_palette_over_color() {
    let themed = MarkSpec {
        form: MarkForm::Pill,
        color: Some("red".into()),
        theme: Some("neon".into()),
        pill: PillSpec {
            label: Some("build".into()),
            message: Some("passing".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    // theme_only must not carry the explicit color
    let mut theme_only = themed.clone();
    theme_only.color = None;
    assert_eq!(
        render(&themed),
        render(&theme_only),
        "theme defines the full palette"
    );
}

// ---------- determinism ----------

#[test]
fn determinism_no_clock_no_upstream() {
    let a = render(&hero("aurora", "Ship your release"));
    let b = render(&hero("aurora", "Ship your release"));
    assert_eq!(a, b);
    // The grammar has no time/clock vocabulary at all.
    for needle in ["timeAuto", "timeGradient", "clock_seed"] {
        assert!(!a.contains(needle));
    }
}

#[test]
fn unknown_inputs_normalize_never_fail() {
    // Unknown art and unknown form both normalize to flagship defaults.
    let svg = render(&hero("not-a-real-type", "Hi"));
    assert!(svg.contains("<svg"));
    let spec = MarkSpec {
        form: MarkForm::parse(Some("not-a-form")),
        ..Default::default()
    };
    assert_eq!(spec.form, MarkForm::Hero, "unknown form normalizes to hero");
    let _ = render(&spec);
}
