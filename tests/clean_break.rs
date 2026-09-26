//! Clean-break contract tests (ADR-0003): strict SVG attribute grammar,
//! escaping, bounded inputs, determinism — no legacy, no clock, no upstream.

use mark::capabilities::mark::domain::text::cap_text;
use mark::capabilities::mark::domain::{HeroSpec, MarkForm, MarkSpec, PillSpec, StripSpec};
use mark::capabilities::mark::render;

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
fn hero_paint_cannot_inject_attributes() {
    // `color` is the live paint entry point; anything that is not a validated
    // hex token or a parsed gradient falls back to theme paint.
    for evil in [
        "\" onload=\"alert(1)",
        "red\" onmouseover=\"x",
        "#ff0000\"><script>",
        "url(javascript:alert(1))",
        "expression(alert(1))",
        "red;fill:url(#x)",
    ] {
        let mut spec = hero("soft", "Hi");
        spec.color = Some(evil.into());
        spec.animation = Some("none".into());
        let svg = render(&spec);
        for needle in ["onload=", "onmouseover=", "<script", "javascript:"] {
            assert!(!svg.contains(needle), "paint injection {evil}: {needle}");
        }
    }
}

#[test]
fn hero_accepts_a_custom_hex_gradient() {
    let mut spec = hero("soft", "Hi");
    spec.color = Some("0:FF6B6B,100:C44569".into());
    spec.animation = Some("none".into());
    let svg = render(&spec);
    assert!(svg.contains("#FF6B6B"), "first stop kept");
    assert!(svg.contains("#C44569"), "last stop kept");
}

#[test]
fn hero_keeps_valid_three_digit_hex_paint() {
    let mut spec = hero("soft", "Hi");
    spec.color = Some("#f00".into());
    spec.animation = Some("none".into());
    let svg = render(&spec);
    assert!(
        svg.contains("#FF0000"),
        "valid paint token kept, canonical: {svg:.200}"
    );
    assert!(!svg.contains("NaN"), "no non-finite geometry can reach SVG");
}

#[test]
fn hero_spec_is_the_dest_geometry_only() {
    // Compile-time guard: this literal lists every `HeroSpec` field. If a
    // retired capsule-render knob (size, colour, align, rotate, stroke, text
    // background, section, reversal) is reintroduced, the test stops compiling.
    let spec = MarkSpec {
        form: MarkForm::Hero,
        art: Some("soft".into()),
        text: Some("Hi".into()),
        hero: HeroSpec {
            layout: Some("signal".into()),
        },
        animation: Some("none".into()),
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(
        svg.contains("text-anchor=\"middle\""),
        "signal layout applies"
    );
    for retired_output in ["paint-order=", "transform=\"rotate("] {
        assert!(
            !svg.contains(retired_output),
            "retired knob output must not exist: {retired_output}"
        );
    }
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
    // Bounded by icon count (MAX_ICONS = 60); bytes follow the brand paths.
    assert_eq!(
        svg.matches("<title>").count(),
        60,
        "strip must stay bounded"
    );
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
    assert_eq!(a, b, "same spec, same SVG; a clock would diverge");
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
    assert_eq!(
        MarkForm::parse(Some("identity")),
        MarkForm::Profile,
        "retired identity form must not silently fall back to hero"
    );
    for retired_id in ["badge", "icons", "iconsrow", "card", "deploymark"] {
        assert_eq!(
            MarkForm::parse(Some(retired_id)),
            MarkForm::Hero,
            "retired predecessor id {retired_id} is unknown input, not a form"
        );
    }
    let _ = render(&spec);
}
