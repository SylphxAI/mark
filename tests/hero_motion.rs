//! Gating tests: background SMIL + credit defaults for the hero mark.

use mark::mark::{render, MarkForm, MarkSpec};
use mark::svg::credit_mark;

fn hero(ty: &str, text: &str) -> MarkSpec {
    MarkSpec {
        form: MarkForm::Hero,
        art: Some(ty.into()),
        text: Some(text.into()),
        ..Default::default()
    }
}

fn strip_text_elements(svg: &str) -> String {
    let mut out = String::with_capacity(svg.len());
    let mut rest = svg;
    while let Some(start) = rest.find("<text") {
        out.push_str(&rest[..start]);
        if let Some(end_rel) = rest[start..].find("</text>") {
            rest = &rest[start + end_rel + "</text>".len()..];
        } else {
            rest = &rest[start + 5..];
        }
    }
    out.push_str(rest);
    out
}

fn count_smil(svg: &str) -> usize {
    svg.matches("<animate").count() + svg.matches("animateTransform").count()
}

#[test]
fn ambient_aurora_has_background_smil_not_only_text() {
    let svg = render(&hero("aurora", "Hello"));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Hello"));
    let bg = strip_text_elements(&svg);
    let bg_smil = count_smil(&bg);
    assert!(
        bg_smil >= 1,
        "ambient aurora must animate background layers; bg_smil={bg_smil}"
    );
}

#[test]
fn ambient_orbit_and_wave_have_background_smil() {
    for ty in [
        "orbit",
        "wave",
        "mesh",
        "constellation",
        "plasma",
        "holo",
        "neon",
        "meteor",
        "liquid",
        "prism",
        "void",
        "firefly",
        "silk",
        "iridescent",
    ] {
        let mut spec = hero(ty, "Hello");
        spec.height = Some(180);
        let svg = render(&spec);
        let bg = strip_text_elements(&svg);
        assert!(
            count_smil(&bg) >= 1,
            "style {ty} ambient missing background SMIL"
        );
    }
}

#[test]
fn wave_and_waving_have_multi_layer_path_smil() {
    for ty in ["wave", "waving"] {
        let mut spec = hero(ty, "Waves");
        spec.height = Some(200);
        let svg = render(&spec);
        let bg = strip_text_elements(&svg);
        let path_anims = bg.matches("attributeName=\"d\"").count();
        assert!(
            path_anims >= 3,
            "{ty} should morph multiple wave layers; path_anims={path_anims}"
        );
        assert!(bg.contains("stroke"), "{ty} should include foam crest stroke");
    }
}

#[test]
fn neon_bounce_type_text_motion_emit_smil() {
    for anim in ["neon", "bounce", "type"] {
        let mut spec = hero("plasma", "Motion");
        spec.animation = Some(anim.into());
        let svg = render(&spec);
        assert!(
            svg.contains("<animate") || svg.contains("animateTransform"),
            "{anim} must emit SMIL"
        );
        assert!(svg.contains("Motion"), "{anim} must keep text");
    }
}

#[test]
fn rise_animates_text_and_keeps_background_motion() {
    let mut spec = hero("aurora", "Ship");
    spec.animation = Some("rise".into());
    let svg = render(&spec);
    let bg = strip_text_elements(&svg);
    assert!(count_smil(&bg) >= 1, "rise must keep ambient background motion");
    assert!(
        svg.contains("animateTransform") && svg.contains("opacity"),
        "rise text motion missing"
    );
}

#[test]
fn animation_none_freezes_background_smil() {
    let mut spec = hero("aurora", "Static");
    spec.animation = Some("none".into());
    let svg = render(&spec);
    let bg = strip_text_elements(&svg);
    assert_eq!(
        count_smil(&bg),
        0,
        "animation=none must not emit background SMIL"
    );
}

#[test]
fn credit_off_has_no_watermark_or_company_stamp() {
    let svg = render(&hero("aurora", "Hello"));
    assert!(!svg.to_ascii_lowercase().contains("sylphx"));
    assert!(!svg.contains(">mark</text>"));
    assert_eq!(credit_mark(100, 40, false), "");
}

#[test]
fn credit_on_uses_mark_product_watermark_not_company_brand() {
    let mark = credit_mark(400, 200, true);
    assert!(mark.contains(">mark</text>"), "watermark text: {mark}");
    assert!(!mark.contains(">sylphx</text>"), "company stamp: {mark}");
    let mut spec = hero("soft", "Hello");
    spec.credit = true;
    spec.animation = Some("none".into());
    let svg = render(&spec);
    assert!(svg.contains(">mark</text>"));
    assert!(!svg.contains(">sylphx</text>"));
}

#[test]
fn animation_none_freezes_wave_blobs_too() {
    let mut spec = hero("wave", "Static Wave");
    spec.theme = Some("sunset".into());
    spec.animation = Some("none".into());
    let svg = render(&spec);
    let bg = strip_text_elements(&svg);
    assert_eq!(
        count_smil(&bg),
        0,
        "wave + animation=none must freeze blob and path SMIL"
    );
}
