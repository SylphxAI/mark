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
    let svg = mark::deploy_mark::render("mark", None, "flat");
    assert!(svg.contains("Sylphx") || svg.contains("deployed"));
}

#[test]
fn theme_drives_motif_colors_not_only_field() {
    // sunset accents must appear in decorative motif layers (wave/plasma),
    // not only as a muted base wash.
    let sunset = banner::render(&BannerInput {
        type_name: Some("wave".into()),
        theme: Some("sunset".into()),
        text: Some("Sunset Wave".into()),
        animation: Some("ambient".into()),
        ..Default::default()
    });
    let neon = banner::render(&BannerInput {
        type_name: Some("wave".into()),
        theme: Some("neon".into()),
        text: Some("Neon Wave".into()),
        animation: Some("ambient".into()),
        ..Default::default()
    });
    assert_ne!(
        sunset, neon,
        "different themes must produce different banner SVGs"
    );
    // Theme accent tokens from themes.rs
    assert!(
        sunset.to_ascii_lowercase().contains("ff6b35")
            || sunset.to_ascii_lowercase().contains("ff9f1c")
            || sunset.to_ascii_lowercase().contains("ffab91"),
        "sunset wave should carry warm sunset chroma"
    );
    assert!(
        neon.to_ascii_lowercase().contains("00f5d4")
            || neon.to_ascii_lowercase().contains("f15bb5")
            || neon.to_ascii_lowercase().contains("9b5de5"),
        "neon wave should carry neon chroma"
    );
    // Motif should use chromatic wave gradients
    assert!(sunset.contains("mgWaveA") || sunset.contains("url(#mgWave"));
    assert!(neon.contains("mgHolo") || neon.contains("mgWave") || neon.contains("mgBloom"));
}

#[test]
fn plate_uses_tinted_scrim_and_gradient_tile() {
    let svg = banner::render(&BannerInput {
        type_name: Some("product".into()),
        theme: Some("tokyonight".into()),
        text: Some("Mark".into()),
        layout: Some("plate".into()),
        animation: Some("none".into()),
        height: Some(360),
        ..Default::default()
    });
    assert!(svg.contains("plateScrim"));
    assert!(svg.contains("plateTile"));
    assert!(svg.contains("plateGlow"));
    // Not pure black scrim only
    assert!(
        !svg.contains("stop-color=\"#000000\" stop-opacity=\"0.48\""),
        "plate scrim should be base-tinted, not legacy pure black"
    );
}

#[test]
fn plasma_and_holo_are_theme_bound() {
    let a = banner::render(&BannerInput {
        type_name: Some("plasma".into()),
        theme: Some("ocean".into()),
        text: Some("Ocean".into()),
        animation: Some("ambient".into()),
        ..Default::default()
    });
    let b = banner::render(&BannerInput {
        type_name: Some("plasma".into()),
        theme: Some("radical".into()),
        text: Some("Radical".into()),
        animation: Some("ambient".into()),
        ..Default::default()
    });
    assert_ne!(a, b);
    assert!(a.contains("mgHolo"));
    assert!(b.contains("mgBloom"));
}

#[test]
fn typewriter_uses_proportional_advances() {
    let svg = banner::render(&BannerInput {
        type_name: Some("soft".into()),
        text: Some("Ship".into()),
        animation: Some("type".into()),
        color: Some("7C3AED,00F5D4".into()),
        height: Some(220),
        ..Default::default()
    });
    // Parse glyph x positions for visible typewriter nodes (skip a11y font-size=1).
    let mut xs = Vec::new();
    for chunk in svg.split("<text ").skip(1) {
        if chunk.contains("font-size=\"1\"") {
            continue;
        }
        let Some(x_str) = chunk.split("x=\"").nth(1).and_then(|s| s.split('"').next()) else {
            continue;
        };
        let Ok(x) = x_str.parse::<f32>() else { continue };
        let Some(body) = chunk.split("</text>").next() else { continue };
        let mut cleaned = body.to_string();
        while let Some(a) = cleaned.find("<animate") {
            let Some(rel) = cleaned[a..].find("/>") else { break };
            let end = a + rel + 2;
            cleaned.replace_range(a..end, "");
        }
        let ch = cleaned
            .rsplit('>')
            .next()
            .and_then(|s| s.chars().next())
            .filter(|c| c.is_ascii_alphabetic());
        if let Some(ch) = ch {
            xs.push((ch, x));
        }
    }
    assert_eq!(xs.len(), 4, "expected S,h,i,p got {xs:?}");
    let advances: Vec<f32> = xs.windows(2).map(|w| w[1].1 - w[0].1).collect();
    for a in &advances {
        assert!(*a < 36.0, "advance too wide {a:?} xs={xs:?}");
        assert!(*a > 10.0, "advance too small {a:?} xs={xs:?}");
    }
    let max_a = advances.iter().cloned().fold(f32::MIN, f32::max);
    let min_a = advances.iter().cloned().fold(f32::MAX, f32::min);
    assert!(
        max_a - min_a > 2.0,
        "expected variable advances for Ship, got {advances:?}"
    );
    assert!(
        advances[1] < advances[0] || advances[1] < advances[2],
        "i-related advance should be relatively narrow: {advances:?}"
    );
}
