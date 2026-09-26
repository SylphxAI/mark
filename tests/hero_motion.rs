//! Gating tests: background SMIL + credit defaults for the hero mark.

use mark::capabilities::mark::domain::svg::credit_mark;
use mark::capabilities::mark::domain::{MarkForm, MarkSpec};
use mark::capabilities::mark::render;

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
fn ambient_art_has_background_smil() {
    for ty in ["waving", "aurora", "mesh", "spotlight", "grid", "terminal"] {
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
fn waving_drifts_three_wave_layers() {
    let mut spec = hero("waving", "Waves");
    spec.height = Some(200);
    let bg = strip_text_elements(&render(&spec));
    assert_eq!(bg.matches("<path d=\"M0 ").count(), 3, "three wave layers");
    assert_eq!(
        bg.matches("type=\"translate\" from=").count(),
        3,
        "each layer drifts sideways"
    );
}

#[test]
fn fade_rise_type_text_motion_emit_smil() {
    for anim in ["fade", "rise", "type"] {
        let mut spec = hero("mesh", "Motion");
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
    assert!(
        count_smil(&bg) >= 1,
        "rise must keep ambient background motion"
    );
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
    let mut spec = hero("minimal", "Hello");
    spec.credit = true;
    spec.animation = Some("none".into());
    let svg = render(&spec);
    assert!(svg.contains(">mark</text>"));
    assert!(!svg.contains(">sylphx</text>"));
}

#[test]
fn animation_none_freezes_wave_blobs_too() {
    let mut spec = hero("waving", "Static Wave");
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
