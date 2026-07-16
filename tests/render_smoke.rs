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
