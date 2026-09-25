//! Streak card: total contributions, current streak, longest streak.

use super::calendar::{Streak, StreakSummary};
use super::card::{frame, icon, icons, n2, CardStyle};
use super::date::{civil_from_days, format_long, format_short, Day};
use super::format::card_number;
use crate::capabilities::mark::domain::svg::{esc, normalize_hex_token};

pub(crate) const WIDTH: u32 = 495;
pub(crate) const HEIGHT: u32 = 195;

/// github-readme-streak-stats color knobs, each defaulting from the palette.
#[derive(Debug, Clone, Default)]
pub(crate) struct StreakColors {
    pub ring: Option<String>,
    pub fire: Option<String>,
    pub curr_streak_num: Option<String>,
    pub side_nums: Option<String>,
    pub curr_streak_label: Option<String>,
    pub side_labels: Option<String>,
    pub dates: Option<String>,
    pub stroke: Option<String>,
}

struct Ink {
    ring: String,
    fire: String,
    curr_num: String,
    side_nums: String,
    curr_label: String,
    side_labels: String,
    dates: String,
    stroke: String,
}

fn ink(style: &CardStyle, c: &StreakColors, themed: bool) -> Ink {
    let p = &style.palette;
    // Unthemed cards keep streak-stats' warm orange signature.
    let (accent, fire, strong, soft) = if themed {
        (
            p.title.clone(),
            p.icon.clone(),
            p.text.clone(),
            p.text.clone(),
        )
    } else {
        (
            "#fb8c00".into(),
            "#fb8c00".into(),
            "#151515".into(),
            "#464646".into(),
        )
    };
    let pick = |v: &Option<String>, d: &str| {
        v.as_deref()
            .and_then(normalize_hex_token)
            .map(|h| h.to_ascii_lowercase())
            .unwrap_or_else(|| d.to_string())
    };
    Ink {
        ring: pick(&c.ring, &accent),
        fire: pick(&c.fire, &fire),
        curr_num: pick(&c.curr_streak_num, &strong),
        side_nums: pick(&c.side_nums, &strong),
        curr_label: pick(&c.curr_streak_label, &accent),
        side_labels: pick(&c.side_labels, &strong),
        dates: pick(&c.dates, &soft),
        stroke: pick(&c.stroke, &p.border),
    }
}

fn range(start: Option<Day>, end: Option<Day>, today: Option<Day>) -> String {
    match (start, end) {
        (Some(s), Some(e)) if s == e => format_long(s),
        (Some(s), Some(e)) => {
            let end_label = if Some(e) == today {
                "Present".to_string()
            } else {
                format_long(e)
            };
            let start_label = if civil_from_days(s).0 == civil_from_days(e).0 {
                format_short(s)
            } else {
                format_long(s)
            };
            format!("{start_label} – {end_label}")
        }
        _ => String::new(),
    }
}

fn streak_range(s: &Streak, today: Option<Day>) -> String {
    range(s.start, s.end, today)
}

fn side(x: f32, num: &str, label: &str, dates: &str, ink: &Ink) -> String {
    format!(
        "<text x=\"{x}\" y=\"96\" text-anchor=\"middle\" fill=\"{}\" font-size=\"28\" font-weight=\"700\">{}</text>\
         <text x=\"{x}\" y=\"128\" text-anchor=\"middle\" fill=\"{}\" font-size=\"14\">{}</text>\
         <text x=\"{x}\" y=\"153\" text-anchor=\"middle\" fill=\"{}\" font-size=\"12\">{}</text>",
        ink.side_nums,
        esc(num),
        ink.side_labels,
        esc(label),
        ink.dates,
        esc(dates),
        x = n2(x)
    )
}

pub(crate) fn render(
    s: &StreakSummary,
    style: &CardStyle,
    colors: &StreakColors,
    themed: bool,
) -> String {
    let ink = ink(style, colors, themed);
    let width = style.width(WIDTH, 300) as f32;
    let col = width / 3.0;
    let mut body = String::new();
    for i in 1..3 {
        body.push_str(&format!(
            "<line x1=\"{x}\" y1=\"28\" x2=\"{x}\" y2=\"170\" stroke=\"{}\" stroke-width=\"1\"/>",
            ink.stroke,
            x = n2(col * i as f32)
        ));
    }
    body.push_str(&side(
        col / 2.0,
        &card_number(s.total),
        "Total Contributions",
        &range(s.first, s.last, s.last),
        &ink,
    ));
    let cx = col * 1.5;
    body.push_str(&format!(
        "<circle cx=\"{}\" cy=\"71\" r=\"40\" fill=\"none\" stroke=\"{}\" stroke-width=\"5\"/>\
         <circle cx=\"{}\" cy=\"31\" r=\"13\" fill=\"{}\"/>",
        n2(cx),
        ink.ring,
        n2(cx),
        style.palette.bg_solid()
    ));
    body.push_str(&icon(icons::FLAME, cx - 10.0, 20.0, 20.0, &ink.fire));
    body.push_str(&format!(
        "<text x=\"{x}\" y=\"81\" text-anchor=\"middle\" fill=\"{}\" font-size=\"28\" font-weight=\"700\">{}</text>\
         <text x=\"{x}\" y=\"140\" text-anchor=\"middle\" fill=\"{}\" font-size=\"14\" font-weight=\"700\">Current Streak</text>\
         <text x=\"{x}\" y=\"163\" text-anchor=\"middle\" fill=\"{}\" font-size=\"12\">{}</text>",
        ink.curr_num,
        s.current.length,
        ink.curr_label,
        ink.dates,
        esc(&streak_range(&s.current, s.last)),
        x = n2(cx)
    ));
    body.push_str(&side(
        col * 2.5,
        &s.longest.length.to_string(),
        "Longest Streak",
        &streak_range(&s.longest, s.last),
        &ink,
    ));
    let mut untitled = style.clone();
    untitled.hide_title = true;
    frame(
        &untitled,
        width as u32,
        HEIGHT,
        &style.title_or("GitHub contribution streak".into()),
        &body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::live::domain::date::days_from_civil;
    use crate::capabilities::live::domain::palette::ColorOverrides;

    #[test]
    fn renders_numbers_and_ranges() {
        let d = days_from_civil(2026, 9, 20);
        let s = StreakSummary {
            total: 1500,
            first: Some(d - 364),
            last: Some(d + 5),
            current: Streak {
                length: 6,
                start: Some(d),
                end: Some(d + 5),
            },
            longest: Streak {
                length: 21,
                start: Some(d - 100),
                end: Some(d - 80),
            },
        };
        let style = CardStyle::new(None, &ColorOverrides::default());
        let svg = render(&s, &style, &StreakColors::default(), false);
        assert!(svg.contains("1,500"));
        assert!(svg.contains(">6<"));
        assert!(svg.contains("Sep 20 – Present"));
        assert!(svg.contains("Jun 12 – Jul 2, 2026"));
        assert!(svg.contains("#fb8c00"));
    }
}
