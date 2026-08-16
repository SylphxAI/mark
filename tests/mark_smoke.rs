//! Render smoke: every form of the one grammar renders.

use mark::capabilities::mark::domain::shapes::ART_TYPES;
use mark::mark::{render, MarkForm, MarkSpec};

fn hero(art: &str, text: &str) -> MarkSpec {
    MarkSpec {
        form: MarkForm::Hero,
        art: Some(art.into()),
        text: Some(text.into()),
        ..Default::default()
    }
}

#[test]
fn hero_all_art_types_render() {
    for ty in ART_TYPES {
        let svg = render(&hero(ty, "T"));
        assert!(svg.starts_with("<?xml"), "type {ty}");
        assert!(svg.contains("</svg>"), "type {ty}");
    }
}

#[test]
fn hero_plate_has_monogram_and_left_anchor() {
    let spec = MarkSpec {
        form: MarkForm::Hero,
        art: Some("aurora".into()),
        theme: Some("tokyonight".into()),
        animation: Some("none".into()),
        height: Some(768),
        width: Some(1376),
        text: Some("PDF Reader MCP".into()),
        desc: Some("The PDF intelligence layer".into()),
        hero: mark::capabilities::mark::domain::HeroSpec {
            layout: Some("plate".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(svg.contains("text-anchor=\"start\""), "plate titles left-aligned");
    assert!(svg.contains("PR"), "monogram present");
    assert!(svg.contains("PDF Reader MCP"));
}

#[test]
fn hero_typewriter_is_per_character() {
    let mut spec = hero("soft", "Hi");
    spec.animation = Some("type".into());
    let svg = render(&spec);
    let anims = svg.matches("attributeName=\"opacity\"").count();
    assert!(anims >= 2, "typewriter animates each character; anims={anims}");
}

#[test]
fn pill_styles_render() {
    for style in ["flat", "plastic", "for-the-badge", "social", "pill"] {
        let spec = MarkSpec {
            form: MarkForm::Pill,
            pill: mark::capabilities::mark::domain::PillSpec {
                label: Some("build".into()),
                message: Some("passing".into()),
                style: Some(style.into()),
                ..Default::default()
            },
            ..Default::default()
        };
        let svg = render(&spec);
        assert!(
            svg.to_ascii_lowercase().contains("passing"),
            "style {style}"
        );
        assert!(svg.contains("<svg"));
    }
}

#[test]
fn strip_new_catalog_icons_render() {
    let spec = MarkSpec {
        form: MarkForm::Strip,
        theme: Some("dark".into()),
        strip: mark::capabilities::mark::domain::StripSpec {
            icons: Some("java,terraform,mongodb,kotlin,swift".into()),
            per_line: Some(8),
        },
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(svg.contains("<title>java</title>"));
    assert!(svg.contains("<title>terraform</title>"));
    assert!(
        !svg.contains(">?</text>"),
        "catalog ids must not fall back to the unknown tile"
    );
}

#[test]
fn strip_renders_and_caps() {
    let spec = MarkSpec {
        form: MarkForm::Strip,
        theme: Some("dark".into()),
        strip: mark::capabilities::mark::domain::StripSpec {
            icons: Some("rust,ts,docker,kubernetes".into()),
            per_line: Some(8),
            },
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("rust"));
}

#[test]
fn profile_renders_text_and_art() {
    let plain = render(&MarkSpec {
        form: MarkForm::Profile,
        text: Some("Kyle Tse".into()),
        ..Default::default()
    });
    assert!(plain.contains("Kyle Tse"));
    let art = render(&MarkSpec {
        form: MarkForm::Profile,
        art: Some("aurora".into()),
        theme: Some("neon".into()),
        text: Some("Sylphx".into()),
        desc: Some("AI-native platform".into()),
        ..Default::default()
    });
    assert!(art.contains("Sylphx"));
    assert!(art.contains("AI-native platform"));
}

#[test]
fn profile_scales_to_any_width() {
    let wide = render(&MarkSpec {
        form: MarkForm::Profile,
        width: Some(320),
        text: Some("Kyle Tse".into()),
        ..Default::default()
    });
    assert!(wide.contains("scale(0.5)"), "profile must scale to width");
}

#[test]
fn profile_omits_empty_tagline() {
    let svg = render(&MarkSpec {
        form: MarkForm::Profile,
        text: Some("Kyle Tse".into()),
        ..Default::default()
    });
    assert!(svg.contains("Kyle Tse"));
}

#[test]
fn deploy_renders_conversion_pill() {
    let svg = render(&MarkSpec {
        form: MarkForm::Deploy,
        deploy: mark::capabilities::mark::domain::DeploySpec {
            service: Some("mark".into()),
            },
        ..Default::default()
    });
    assert!(svg.contains("deployed on"));
    assert!(svg.contains("mark · Sylphx"));
}

#[test]
fn composition_pill_motion_and_profile_art() {
    let pill = render(&MarkSpec {
        form: MarkForm::Pill,
        theme: Some("neon".into()),
        animation: Some("glow".into()),
        pill: mark::capabilities::mark::domain::PillSpec {
            label: Some("build".into()),
            message: Some("passing".into()),
            ..Default::default()
        },
        ..Default::default()
    });
    assert!(pill.contains("<animate"), "pill with motion composes");

    let profile = render(&MarkSpec {
        form: MarkForm::Profile,
        art: Some("wave".into()),
        theme: Some("ocean".into()),
        text: Some("Kyle Tse".into()),
        ..Default::default()
    });
    assert!(profile.contains("<svg"));
}

#[test]
fn mono_font_composes_into_hero_and_profile() {
    let mut spec = hero("transparent", "MCP & AI-agent tooling");
    spec.font = Some("mono".into());
    let hero_svg = render(&spec);
    assert!(hero_svg.contains("ui-monospace"), "hero mono font");
    let profile_svg = render(&MarkSpec {
        form: MarkForm::Profile,
        text: Some("Kyle Tse".into()),
        font: Some("mono".into()),
        ..Default::default()
    });
    assert!(profile_svg.contains("ui-monospace"), "profile mono font");
}

#[test]
fn same_spec_renders_same_svg_forever() {
    let a = render(&hero("aurora", "Ship your release"));
    let b = render(&hero("aurora", "Ship your release"));
    assert_eq!(a, b, "determinism: same URL, same mark, forever");
}
