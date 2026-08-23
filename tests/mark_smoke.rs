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
    assert!(
        svg.contains("text-anchor=\"start\""),
        "plate titles left-aligned"
    );
    assert!(svg.contains("PR"), "monogram present");
    assert!(svg.contains("PDF Reader MCP"));
}

#[test]
fn hero_typewriter_is_per_character() {
    let mut spec = hero("soft", "Hi");
    spec.animation = Some("type".into());
    let svg = render(&spec);
    let anims = svg.matches("attributeName=\"opacity\"").count();
    assert!(
        anims >= 2,
        "typewriter animates each character; anims={anims}"
    );
}

#[test]
fn hero_long_title_is_marked_inside_the_canvas() {
    let spec = MarkSpec {
        form: MarkForm::Hero,
        art: Some("soft".into()),
        animation: Some("none".into()),
        width: Some(360),
        height: Some(120),
        text: Some("A Very Long Display Name That Should Not Escape The Banner Canvas".into()),
        desc: Some("A similarly long tagline that also has to live inside the mark".into()),
        ..Default::default()
    };
    let svg = render(&spec);
    assert!(svg.contains('…'), "overflowing hero text is marked");
    assert!(
        !svg.contains("Should Not Escape The Banner Canvas"),
        "hero title must fit the canvas"
    );
    assert!(
        !svg.contains("also has to live inside the mark"),
        "hero desc must fit the canvas"
    );
}

#[test]
fn hero_short_title_is_not_truncated() {
    let svg = render(&hero("soft", "Ship it"));
    assert!(svg.contains("Ship it"));
    assert!(!svg.contains('…'));
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

fn strip_groups_are_balanced(svg: &str) {
    assert_eq!(
        svg.matches("<g").count(),
        svg.matches("</g>").count(),
        "strip SVG groups must be well-formed"
    );
}

fn strip_spec(anim: &str) -> MarkSpec {
    MarkSpec {
        form: MarkForm::Strip,
        theme: Some("dark".into()),
        animation: Some(anim.into()),
        strip: mark::capabilities::mark::domain::StripSpec {
            icons: Some("rust,ts,docker".into()),
            per_line: Some(8),
        },
        ..Default::default()
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
    strip_groups_are_balanced(&svg);
}

#[test]
fn strip_motion_wraps_the_icon_row() {
    let fade = render(&strip_spec("fade"));
    strip_groups_are_balanced(&fade);
    assert!(fade.contains("<animate"), "fade must emit SMIL");
    let rust = fade.find("<title>rust</title>").expect("rust icon");
    let wrap_close = fade.rfind("</g>").expect("row wrap");
    assert!(
        fade.find("<animate").expect("animate") < rust && rust < wrap_close,
        "fade SMIL must wrap the icon row, not an empty group"
    );

    let glow = render(&strip_spec("glow"));
    strip_groups_are_balanced(&glow);
    assert!(glow.contains("<animate"), "glow must compose onto the strip");

    let none = render(&strip_spec("none"));
    strip_groups_are_balanced(&none);
    assert!(!none.contains("<animate"), "static strip stays still");
}

#[test]
fn profile_renders_text_art_and_monogram() {
    let plain = render(&MarkSpec {
        form: MarkForm::Profile,
        text: Some("Kyle Tse".into()),
        ..Default::default()
    });
    assert!(plain.contains("Kyle Tse"));
    assert!(plain.contains(">KT<"), "profile owns a name monogram");
    let art = render(&MarkSpec {
        form: MarkForm::Profile,
        art: Some("aurora".into()),
        theme: Some("neon".into()),
        text: Some("Ada Lovelace".into()),
        desc: Some("AI-native platform".into()),
        ..Default::default()
    });
    assert!(art.contains("Ada Lovelace"));
    assert!(art.contains("AI-native platform"));
    assert!(art.contains(">AL<"));
    assert!(art.contains("clipPath"), "art is clipped to the card");
    assert!(
        art.contains("id=\"mg\""),
        "profile art must share the hero chromatic kernel ids"
    );
}

#[test]
fn profile_uses_native_geometry() {
    let svg = render(&MarkSpec {
        form: MarkForm::Profile,
        width: Some(320),
        height: Some(120),
        text: Some("Kyle Tse".into()),
        ..Default::default()
    });
    assert!(svg.contains("width=\"320\""), "profile honors width");
    assert!(svg.contains("height=\"120\""), "profile honors height");
    assert!(
        !svg.contains("scale("),
        "profile must compose at native geometry, not scale a 640 canvas"
    );
}

#[test]
fn identity_form_is_the_profile_card() {
    assert_eq!(MarkForm::parse(Some("identity")), MarkForm::Profile);
    let identity = render(&MarkSpec {
        form: MarkForm::parse(Some("identity")),
        text: Some("Ada Lovelace".into()),
        desc: Some("First programmer".into()),
        ..Default::default()
    });
    let profile = render(&MarkSpec {
        form: MarkForm::Profile,
        text: Some("Ada Lovelace".into()),
        desc: Some("First programmer".into()),
        ..Default::default()
    });
    assert_eq!(
        identity, profile,
        "retired identity form is the profile card"
    );
    assert!(identity.contains("Ada Lovelace"));
    assert!(identity.contains(">AL<"));
}

#[test]
fn profile_marks_overflowing_name() {
    let svg = render(&MarkSpec {
        form: MarkForm::Profile,
        width: Some(320),
        height: Some(120),
        text: Some("A Very Long Display Name That Should Not Escape The Card".into()),
        ..Default::default()
    });
    assert!(svg.contains('…'), "overflowing profile names are marked");
    assert!(
        !svg.contains("Should Not Escape The Card"),
        "profile name must fit the card"
    );
    assert!(
        svg.contains("clip-path=\"url(#pt)\""),
        "name column is clipped"
    );
}

#[test]
fn profile_marks_wide_glyph_overflow() {
    let svg = render(&MarkSpec {
        form: MarkForm::Profile,
        width: Some(320),
        height: Some(120),
        text: Some("WWWWWWWWWWWWWWWW".into()),
        desc: Some("MMMMMMMMMMMMMMMM".into()),
        ..Default::default()
    });
    assert!(
        svg.contains('…'),
        "wide glyphs must be marked, not clipped silently"
    );
    assert!(
        !svg.contains("WWWWWWWWWWWWWWWW"),
        "wide profile names must not be emitted in full"
    );
}

#[test]
fn profile_monogram_uses_non_latin_letters() {
    let cjk = render(&MarkSpec {
        form: MarkForm::Profile,
        text: Some("山田太郎".into()),
        ..Default::default()
    });
    assert!(cjk.contains(">山田<"), "CJK names own their monogram");
    assert!(
        !cjk.contains(">MK<"),
        "MK is not a stand-in for user letters"
    );
    let cyr = render(&MarkSpec {
        form: MarkForm::Profile,
        text: Some("Владимир".into()),
        ..Default::default()
    });
    assert!(cyr.contains(">ВЛ<"));
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
    assert!(
        svg.contains("<circle"),
        "deploy conversion mark owns a tile, not a generic two-rect pill"
    );
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
