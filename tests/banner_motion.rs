//! Gating tests: background SMIL + credit defaults for shipped banner renderer.

use mark::banner::{self, BannerInput};
use mark::svg::credit_mark;

fn strip_text_elements(svg: &str) -> String {
    let mut out = String::with_capacity(svg.len());
    let mut rest = svg;
    while let Some(start) = rest.find("<text") {
        out.push_str(&rest[..start]);
        if let Some(end_rel) = rest[start..].find("</text>") {
            rest = &rest[start + end_rel + "</text>".len()..];
        } else {
            // self-closing or broken — drop from start
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
    let svg = banner::render(&BannerInput {
        type_name: Some("aurora".into()),
        theme: Some("tokyonight".into()),
        text: Some("Hello".into()),
        animation: Some("ambient".into()),
        credit: false,
        ..Default::default()
    });
    assert!(svg.contains("<svg"), "expected svg root");
    assert!(svg.contains("Hello"));
    let bg = strip_text_elements(&svg);
    let bg_smil = count_smil(&bg);
    assert!(
        bg_smil >= 1,
        "ambient aurora must animate background layers; bg_smil={bg_smil} sample={}",
        &bg[..bg.len().min(400)]
    );
    // ambient should not require text SMIL
    assert!(
        bg.contains("animateTransform") || bg.contains("<animate"),
        "expected SMIL outside <text>"
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
        let svg = banner::render(&BannerInput {
            type_name: Some(ty.into()),
            text: Some("Hello".into()),
            animation: Some("ambient".into()),
            credit: false,
            height: Some(180),
            ..Default::default()
        });
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
        let svg = banner::render(&BannerInput {
            type_name: Some(ty.into()),
            text: Some("Waves".into()),
            animation: Some("ambient".into()),
            credit: false,
            height: Some(200),
            ..Default::default()
        });
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
        let svg = banner::render(&BannerInput {
            type_name: Some("plasma".into()),
            text: Some("Motion".into()),
            animation: Some(anim.into()),
            credit: false,
            ..Default::default()
        });
        assert!(
            svg.contains("<animate") || svg.contains("animateTransform"),
            "{anim} must emit SMIL"
        );
        assert!(svg.contains("Motion"), "{anim} must keep text");
    }
}

#[test]
fn rise_animates_text_and_keeps_background_motion() {
    let svg = banner::render(&BannerInput {
        type_name: Some("aurora".into()),
        text: Some("Ship".into()),
        animation: Some("rise".into()),
        credit: false,
        ..Default::default()
    });
    let bg = strip_text_elements(&svg);
    assert!(count_smil(&bg) >= 1, "rise must keep ambient background motion");
    // text rise uses animateTransform translate into rest pose
    assert!(
        svg.contains("animateTransform") && svg.contains("opacity"),
        "rise text motion missing"
    );
}

#[test]
fn animation_none_freezes_background_smil() {
    let svg = banner::render(&BannerInput {
        type_name: Some("aurora".into()),
        text: Some("Static".into()),
        animation: Some("none".into()),
        credit: false,
        ..Default::default()
    });
    let bg = strip_text_elements(&svg);
    assert_eq!(
        count_smil(&bg),
        0,
        "animation=none must not emit background SMIL"
    );
}

#[test]
fn credit_off_has_no_watermark_or_company_stamp() {
    let svg = banner::render(&BannerInput {
        type_name: Some("aurora".into()),
        text: Some("Hello".into()),
        animation: Some("ambient".into()),
        credit: false,
        ..Default::default()
    });
    assert!(!svg.to_ascii_lowercase().contains("sylphx"));
    // product watermark only when credit=true
    assert!(!svg.contains(">mark</text>"));
    assert_eq!(credit_mark(100, 40, false), "");
}

#[test]
fn credit_on_uses_mark_product_watermark_not_company_brand() {
    let mark = credit_mark(400, 200, true);
    assert!(mark.contains(">mark</text>"), "watermark text: {mark}");
    // Visible stamp is "mark", not company brand "sylphx" (host may still be mark.sylphx.com).
    assert!(!mark.contains(">sylphx</text>"), "company stamp: {mark}");
    let svg = banner::render(&BannerInput {
        type_name: Some("soft".into()),
        text: Some("Hello".into()),
        credit: true,
        animation: Some("none".into()),
        ..Default::default()
    });
    assert!(svg.contains(">mark</text>"));
    assert!(!svg.contains(">sylphx</text>"));
}

#[test]
fn animation_none_freezes_wave_blobs_too() {
    let svg = banner::render(&BannerInput {
        type_name: Some("wave".into()),
        theme: Some("sunset".into()),
        text: Some("Static Wave".into()),
        animation: Some("none".into()),
        credit: false,
        ..Default::default()
    });
    let bg = strip_text_elements(&svg);
    assert_eq!(
        count_smil(&bg),
        0,
        "wave + animation=none must freeze blob and path SMIL"
    );
}
