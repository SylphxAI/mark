//! Render smoke: every form of the one grammar renders.

use mark::capabilities::mark::domain::shapes::ART_TYPES;
use mark::mark::{render, MarkForm, MarkSpec};

fn hero(art: &str, text: &str) -> MarkSpec {
    MarkSpec {
        form: MarkForm::Hero,
        art: Some(art.into()),
        hero: mark::capabilities::mark::domain::HeroSpec {
            text: Some(text.into()),
            ..Default::default()
        },
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
        hero: mark::capabilities::mark::domain::HeroSpec {
            text: Some("PDF Reader MCP".into()),
            desc: Some("The PDF intelligence layer".into()),
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
    let svg = render(&hero("soft", "Hi").with_anim("type"));
    let anims = svg.matches("attributeName=\"opacity\"").count();
    assert!(anims >= 2, "typewriter animates each character; anims={anims}");
}

trait WithAnim {
    fn with_anim(self, a: &str) -> Self;
}
impl WithAnim for MarkSpec {
    fn with_anim(mut self, a: &str) -> Self {
        self.animation = Some(a.into());
        self
    }
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
fn identity_renders_brand_and_art() {
    let plain = render(&MarkSpec {
        form: MarkForm::Identity,
        identity: mark::capabilities::mark::domain::IdentitySpec {
            brand: Some("sylphx".into()),
            ..Default::default()
        },
        ..Default::default()
    });
    assert!(plain.contains("Sylphx"));
    let art = render(&MarkSpec {
        form: MarkForm::Identity,
        art: Some("aurora".into()),
        theme: Some("neon".into()),
        identity: mark::capabilities::mark::domain::IdentitySpec {
            brand: Some("cubeage".into()),
            tagline: Some("Games".into()),
            },
        ..Default::default()
    });
    assert!(art.contains("Cubeage"));
    assert!(art.contains("aurora") || art.contains("mg"));
}

#[test]
fn identity_scales_to_any_width() {
    let wide = render(&MarkSpec {
        form: MarkForm::Identity,
        width: Some(320),
        identity: mark::capabilities::mark::domain::IdentitySpec {
            brand: Some("sylphx".into()),
            ..Default::default()
        },
        ..Default::default()
    });
    assert!(wide.contains("scale(0.5)"), "identity must scale to width");
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
fn composition_pill_with_motion_and_identity_with_art() {
    let pill = render(&MarkSpec {
        form: MarkForm::Pill,
        theme: Some("sylphx".into()),
        animation: Some("glow".into()),
        pill: mark::capabilities::mark::domain::PillSpec {
            label: Some("build".into()),
            message: Some("passing".into()),
            ..Default::default()
        },
        ..Default::default()
    });
    assert!(pill.contains("<animate"), "pill with motion composes");

    let identity = render(&MarkSpec {
        form: MarkForm::Identity,
        art: Some("wave".into()),
        theme: Some("ocean".into()),
        identity: mark::capabilities::mark::domain::IdentitySpec {
            brand: Some("ozyrix".into()),
            ..Default::default()
        },
        ..Default::default()
    });
    assert!(identity.contains("<svg"));
}

#[test]
fn same_spec_renders_same_svg_forever() {
    let a = render(&hero("aurora", "Ship your release"));
    let b = render(&hero("aurora", "Ship your release"));
    assert_eq!(a, b, "determinism: same URL, same mark, forever");
}
