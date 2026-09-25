//! Paint token vocabulary — every spelling shields accepts for a color.
//!
//! Order matches badge-maker: shields' own names (and aliases) first, then
//! 3/6-digit hex, then CSS named colors, then `rgb()/rgba()/hsl()/hsla()`.
//! Every accepted token becomes a canonical `#rrggbb`/`#rrggbbaa` hex, so no
//! user spelling ever reaches an SVG attribute.

use crate::capabilities::mark::domain::svg::normalize_hex_token;

/// badge-maker's named colors and aliases (current shields palette).
fn shields_named(c: &str) -> Option<&'static str> {
    Some(match c {
        "brightgreen" | "success" => "#44bb00",
        "green" => "#67ac09",
        "yellow" => "#d8b800",
        "yellowgreen" => "#95991a",
        "orange" | "important" => "#ea7233",
        "red" | "critical" => "#dd4343",
        "blue" | "informational" => "#007ec6",
        "grey" | "gray" => "#555555",
        "lightgrey" | "lightgray" | "inactive" => "#939393",
        _ => return None,
    })
}

/// CSS Color Module named colors (`name:rrggbb`, space separated).
const CSS_NAMED: &[&str] = &[
    "aliceblue:f0f8ff antiquewhite:faebd7 aqua:00ffff aquamarine:7fffd4 azure:f0ffff ",
    "beige:f5f5dc bisque:ffe4c4 black:000000 blanchedalmond:ffebcd blue:0000ff ",
    "blueviolet:8a2be2 brown:a52a2a burlywood:deb887 cadetblue:5f9ea0 chartreuse:7fff00 ",
    "chocolate:d2691e coral:ff7f50 cornflowerblue:6495ed cornsilk:fff8dc crimson:dc143c ",
    "cyan:00ffff darkblue:00008b darkcyan:008b8b darkgoldenrod:b8860b darkgray:a9a9a9 ",
    "darkgreen:006400 darkgrey:a9a9a9 darkkhaki:bdb76b darkmagenta:8b008b ",
    "darkolivegreen:556b2f darkorange:ff8c00 darkorchid:9932cc darkred:8b0000 ",
    "darksalmon:e9967a darkseagreen:8fbc8f darkslateblue:483d8b darkslategray:2f4f4f ",
    "darkslategrey:2f4f4f darkturquoise:00ced1 darkviolet:9400d3 deeppink:ff1493 ",
    "deepskyblue:00bfff dimgray:696969 dimgrey:696969 dodgerblue:1e90ff firebrick:b22222 ",
    "floralwhite:fffaf0 forestgreen:228b22 fuchsia:ff00ff gainsboro:dcdcdc ",
    "ghostwhite:f8f8ff gold:ffd700 goldenrod:daa520 gray:808080 green:008000 ",
    "greenyellow:adff2f grey:808080 honeydew:f0fff0 hotpink:ff69b4 indianred:cd5c5c ",
    "indigo:4b0082 ivory:fffff0 khaki:f0e68c lavender:e6e6fa lavenderblush:fff0f5 ",
    "lawngreen:7cfc00 lemonchiffon:fffacd lightblue:add8e6 lightcoral:f08080 ",
    "lightcyan:e0ffff lightgoldenrodyellow:fafad2 lightgray:d3d3d3 lightgreen:90ee90 ",
    "lightgrey:d3d3d3 lightpink:ffb6c1 lightsalmon:ffa07a lightseagreen:20b2aa ",
    "lightskyblue:87cefa lightslategray:778899 lightslategrey:778899 ",
    "lightsteelblue:b0c4de lightyellow:ffffe0 lime:00ff00 limegreen:32cd32 linen:faf0e6 ",
    "magenta:ff00ff maroon:800000 mediumaquamarine:66cdaa mediumblue:0000cd ",
    "mediumorchid:ba55d3 mediumpurple:9370db mediumseagreen:3cb371 mediumslateblue:7b68ee ",
    "mediumspringgreen:00fa9a mediumturquoise:48d1cc mediumvioletred:c71585 ",
    "midnightblue:191970 mintcream:f5fffa mistyrose:ffe4e1 moccasin:ffe4b5 ",
    "navajowhite:ffdead navy:000080 oldlace:fdf5e6 olive:808000 olivedrab:6b8e23 ",
    "orange:ffa500 orangered:ff4500 orchid:da70d6 palegoldenrod:eee8aa palegreen:98fb98 ",
    "paleturquoise:afeeee palevioletred:db7093 papayawhip:ffefd5 peachpuff:ffdab9 ",
    "peru:cd853f pink:ffc0cb plum:dda0dd powderblue:b0e0e6 purple:800080 ",
    "rebeccapurple:663399 red:ff0000 rosybrown:bc8f8f royalblue:4169e1 saddlebrown:8b4513 ",
    "salmon:fa8072 sandybrown:f4a460 seagreen:2e8b57 seashell:fff5ee sienna:a0522d ",
    "silver:c0c0c0 skyblue:87ceeb slateblue:6a5acd slategray:708090 slategrey:708090 ",
    "snow:fffafa springgreen:00ff7f steelblue:4682b4 tan:d2b48c teal:008080 ",
    "thistle:d8bfd8 tomato:ff6347 turquoise:40e0d0 violet:ee82ee wheat:f5deb3 ",
    "white:ffffff whitesmoke:f5f5f5 yellow:ffff00 yellowgreen:9acd32 ",
];

fn css_named(c: &str) -> Option<String> {
    CSS_NAMED
        .iter()
        .flat_map(|row| row.split_ascii_whitespace())
        .find_map(|pair| {
            let (name, hex) = pair.split_once(':')?;
            (name == c).then(|| format!("#{hex}"))
        })
}

/// Canonical hex for a named, hex, or functional color token; `None` for
/// anything else.
pub(crate) fn css_color(token: &str) -> Option<String> {
    let t = token.trim().to_ascii_lowercase();
    if let Some(hex) = shields_named(&t) {
        return Some(hex.to_string());
    }
    if let Some(hex) = normalize_hex_token(&t) {
        return Some(hex);
    }
    css_named(&t).or_else(|| functional(&t))
}

/// `rgb(r,g,b)`, `rgba(r,g,b,a)`, `hsl(h,s%,l%)`, `hsla(h,s%,l%,a)`; commas,
/// spaces, and `/` all separate arguments.
fn functional(t: &str) -> Option<String> {
    let (name, rest) = t.split_once('(')?;
    let args: Vec<&str> = rest
        .strip_suffix(')')?
        .split(|c: char| c == ',' || c == '/' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();
    if !(3..=4).contains(&args.len()) {
        return None;
    }
    let num = |s: &str| -> Option<f64> {
        let v: f64 = s
            .trim_end_matches('%')
            .trim_end_matches("deg")
            .parse()
            .ok()?;
        v.is_finite().then_some(v)
    };
    let alpha = match args.get(3) {
        Some(a) if a.ends_with('%') => Some(num(a)? / 100.0),
        Some(a) => Some(num(a)?),
        None => None,
    };
    let rgb = match name.trim() {
        "rgb" | "rgba" => {
            let ch = |s: &str| -> Option<f64> {
                let v = num(s)?;
                Some(if s.ends_with('%') { v * 2.55 } else { v })
            };
            [ch(args[0])?, ch(args[1])?, ch(args[2])?]
        }
        "hsl" | "hsla" => hsl_to_rgb(num(args[0])?, num(args[1])? / 100.0, num(args[2])? / 100.0),
        _ => return None,
    };
    let byte = |v: f64| v.round().clamp(0.0, 255.0) as u8;
    let mut hex = format!(
        "#{:02x}{:02x}{:02x}",
        byte(rgb[0]),
        byte(rgb[1]),
        byte(rgb[2])
    );
    if let Some(a) = alpha.filter(|a| *a < 1.0) {
        hex.push_str(&format!("{:02x}", byte(a.clamp(0.0, 1.0) * 255.0)));
    }
    Some(hex)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [f64; 3] {
    let (s, l) = (s.clamp(0.0, 1.0), l.clamp(0.0, 1.0));
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h.rem_euclid(360.0) / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    [(r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0]
}

/// badge-maker `brightness`: YIQ luma in `0..=1` of a canonical hex token.
pub(crate) fn brightness(hex: &str) -> f64 {
    let h = hex.trim_start_matches('#');
    let ch = |i: usize| {
        h.get(i..i + 2)
            .and_then(|s| u8::from_str_radix(s, 16).ok())
            .unwrap_or(0) as f64
    };
    if h.len() < 6 {
        return 0.0;
    }
    ((ch(0) * 299.0 + ch(2) * 587.0 + ch(4) * 114.0) / 255_000.0 * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shields_names_and_aliases() {
        assert_eq!(css_color("brightgreen").as_deref(), Some("#44bb00"));
        assert_eq!(css_color("success").as_deref(), Some("#44bb00"));
        assert_eq!(css_color("Critical").as_deref(), Some("#dd4343"));
        assert_eq!(css_color("lightgray").as_deref(), Some("#939393"));
        assert_eq!(css_color("inactive").as_deref(), Some("#939393"));
        assert_eq!(css_color("gray").as_deref(), Some("#555555"));
        assert_eq!(css_color("informational").as_deref(), Some("#007ec6"));
    }

    #[test]
    fn hex_css_names_and_functions() {
        assert_eq!(css_color("ff69b4").as_deref(), Some("#ff69b4"));
        assert_eq!(css_color("#ABC").as_deref(), Some("#aabbcc"));
        assert_eq!(css_color("blueviolet").as_deref(), Some("#8a2be2"));
        assert_eq!(css_color("rebeccapurple").as_deref(), Some("#663399"));
        assert_eq!(css_color("rgb(255, 0, 0)").as_deref(), Some("#ff0000"));
        assert_eq!(css_color("rgba(0,0,0,0.5)").as_deref(), Some("#00000080"));
        assert_eq!(css_color("hsl(120,100%,25%)").as_deref(), Some("#008000"));
        for bad in [
            "",
            "nope",
            "#12",
            "rgb(1,2)",
            "url(#x)",
            "\"><script>",
            "rgb(nan,1,2)",
        ] {
            assert_eq!(css_color(bad), None, "{bad}");
        }
    }

    #[test]
    fn brightness_matches_badge_maker() {
        assert_eq!(brightness("#555555"), 0.33);
        assert_eq!(brightness("#44bb00"), 0.51);
        assert_eq!(brightness("#ffffff"), 1.0);
    }
}
