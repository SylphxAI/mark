use mark::badge::{self, BadgeInput, BadgeStyle};
use mark::banner::{self, BannerInput};
use mark::brand;
use mark::icons;

#[test]
fn banner_waving_contains_svg() {
    let svg = banner::render(&BannerInput {
        type_name: Some("waving".into()),
        theme: Some("tokyonight".into()),
        text: Some("Hello".into()),
        desc: Some("test".into()),
        credit: false,
        ..Default::default()
    });
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Hello"));
    assert!(!svg.to_ascii_lowercase().contains("sylphx"));
}

#[test]
fn banner_all_types_render() {
    for ty in banner::BANNER_TYPES {
        let svg = banner::render(&BannerInput {
            type_name: Some((*ty).into()),
            color: Some("gradient".into()),
            text: Some("T".into()),
            credit: false,
            height: Some(120),
            ..Default::default()
        });
        assert!(svg.starts_with("<?xml"), "type {ty}");
        assert!(svg.contains("</svg>"), "type {ty}");
    }
}

#[test]
fn layout_plate_has_monogram_and_left_anchor() {
    let svg = banner::render(&BannerInput {
        type_name: Some("aurora".into()),
        theme: Some("tokyonight".into()),
        text: Some("PDF Reader MCP".into()),
        desc: Some("The PDF intelligence layer".into()),
        layout: Some("plate".into()),
        animation: Some("none".into()),
        height: Some(768),
        width: Some(1376),
        credit: false,
        ..Default::default()
    });
    assert!(svg.contains("text-anchor=\"start\""), "plate titles left-aligned");
    assert!(svg.contains("plateScrim") || svg.contains("PR"), "monogram/scrim present");
    assert!(svg.contains("PDF Reader MCP"));
    // taller card canvas allowed
    assert!(svg.contains("height=\"768\"") || svg.contains("height='768'"));
}

#[test]
fn animation_type_is_per_character_typewriter() {
    let svg = banner::render(&BannerInput {
        type_name: Some("soft".into()),
        text: Some("Hi".into()),
        animation: Some("type".into()),
        credit: false,
        ..Default::default()
    });
    // Two characters → at least two text nodes with opacity animate
    let opacity_anims = svg.matches("attributeName=\"opacity\"").count();
    assert!(
        opacity_anims >= 2,
        "typewriter should animate each character; anims={opacity_anims}"
    );
    assert!(svg.contains(">H<") || svg.contains(">H</text>") || svg.contains("H"));
}

#[test]
fn badge_flat() {
    let svg = badge::render(&BadgeInput {
        label: Some("build".into()),
        message: "passing".into(),
        color: Some("brightgreen".into()),
        label_color: None,
        style: BadgeStyle::Flat,
        theme: None,
    });
    assert!(svg.contains("passing"));
    assert!(svg.contains("<svg"));
}

#[test]
fn brand_sylphx() {
    let svg = brand::render_brand_card("sylphx", None, true);
    assert!(svg.contains("Sylphx"));
}

#[test]
fn icons_row() {
    let svg = icons::render_row("rust,ts,docker", Some("dark"), 8);
    assert!(svg.contains("<svg"));
}

#[test]
fn deploy_badge() {
    let svg = mark::stats::deploy_badge("mark", None, "flat");
    assert!(svg.contains("Sylphx") || svg.contains("deployed"));
}
