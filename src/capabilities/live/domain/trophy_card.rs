//! Trophy card (github-profile-trophy compatible): one cup per achievement,
//! ranked SSS … C, laid out on a `column` × `row` grid.

use super::card::{background, n2};
use super::date::parse_timestamp;
use super::format::metric;
use super::model::UserStats;
use super::palette::Palette;
use crate::capabilities::mark::domain::svg::{esc, svg_doc};
use crate::capabilities::mark::domain::text::FONT_UI_SANS;

const PANEL: u32 = 110;

/// Rank tiers, best first; `?` is below the first threshold.
pub(crate) const RANKS: [&str; 8] = ["SSS", "SS", "S", "AAA", "AA", "A", "B", "C"];
const TIER_NAMES: [&str; 8] = [
    "Legend", "Master", "Expert", "Veteran", "Advanced", "Skilled", "Rising", "Starter",
];

/// One achievement: its title, value, thresholds (aligned with [`RANKS`]),
/// and how its value reads.
struct Kind {
    title: &'static str,
    thresholds: [f64; 8],
    unit: &'static str,
}

const STARS: Kind = Kind {
    title: "Stars",
    thresholds: [2000.0, 700.0, 200.0, 100.0, 50.0, 30.0, 10.0, 1.0],
    unit: "pt",
};
const COMMITS: Kind = Kind {
    title: "Commits",
    thresholds: [4000.0, 2000.0, 1000.0, 500.0, 200.0, 100.0, 10.0, 1.0],
    unit: "pt",
};
const FOLLOWERS: Kind = Kind {
    title: "Followers",
    thresholds: [1000.0, 400.0, 200.0, 100.0, 50.0, 20.0, 10.0, 1.0],
    unit: "pt",
};
const REPOSITORIES: Kind = Kind {
    title: "Repositories",
    thresholds: [100.0, 80.0, 60.0, 40.0, 30.0, 20.0, 10.0, 1.0],
    unit: "pt",
};
const PULL_REQUESTS: Kind = Kind {
    title: "PullRequest",
    thresholds: [1000.0, 500.0, 200.0, 100.0, 50.0, 20.0, 10.0, 1.0],
    unit: "pt",
};
const ISSUES: Kind = Kind {
    title: "Issues",
    thresholds: [1000.0, 500.0, 200.0, 100.0, 50.0, 20.0, 10.0, 1.0],
    unit: "pt",
};
const EXPERIENCE: Kind = Kind {
    title: "Experience",
    thresholds: [15.0, 12.0, 10.0, 8.0, 6.0, 4.0, 2.0, 1.0],
    unit: "yrs",
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Trophy {
    pub title: &'static str,
    /// Index into [`RANKS`], or `None` for `?`.
    pub tier: Option<usize>,
    pub value: String,
}

impl Trophy {
    pub(crate) fn rank(&self) -> &'static str {
        self.tier.map(|t| RANKS[t]).unwrap_or("?")
    }
}

fn trophy(kind: &Kind, value: f64) -> Trophy {
    let tier = kind.thresholds.iter().position(|t| value >= *t);
    let shown = if kind.unit == "yrs" {
        format!("{} {}", n2((value * 10.0).floor() as f32 / 10.0), kind.unit)
    } else {
        format!("{} {}", metric(value.max(0.0) as u64), kind.unit)
    };
    Trophy {
        title: kind.title,
        tier,
        value: shown,
    }
}

/// Every trophy the data supports, best rank first (ties keep catalog order).
pub(crate) fn trophies(s: &UserStats, now_unix: i64) -> Vec<Trophy> {
    let mut out = Vec::new();
    let mut add = |kind: &Kind, v: Option<u64>| {
        if let Some(v) = v {
            out.push(trophy(kind, v as f64));
        }
    };
    add(&STARS, s.stars);
    add(&COMMITS, s.commits.map(|c| c.0));
    add(&FOLLOWERS, Some(s.followers));
    add(&REPOSITORIES, s.repos);
    add(&PULL_REQUESTS, s.prs);
    add(&ISSUES, s.issues);
    if let Some(created) = s.created_at.as_deref().and_then(parse_timestamp) {
        let years = (now_unix - created).max(0) as f64 / (365.25 * 86_400.0);
        out.push(trophy(&EXPERIENCE, years));
    }
    out.sort_by_key(|t| t.tier.unwrap_or(RANKS.len()));
    out
}

#[derive(Debug, Clone)]
pub(crate) struct TrophyOptions {
    /// Panels per row; `-1` puts every trophy on one row.
    pub column: i32,
    pub row: u32,
    pub margin_w: u32,
    pub margin_h: u32,
    pub no_bg: bool,
    pub no_frame: bool,
    /// Keep only these titles (case-insensitive), when non-empty.
    pub titles: Vec<String>,
    /// Keep only these ranks, when non-empty.
    pub ranks: Vec<String>,
}

impl Default for TrophyOptions {
    fn default() -> Self {
        Self {
            column: 6,
            row: 3,
            margin_w: 0,
            margin_h: 0,
            no_bg: false,
            no_frame: false,
            titles: Vec::new(),
            ranks: Vec::new(),
        }
    }
}

/// Cup gradient per tier band: gold (S), silver (A), bronze (B, C), none (?).
fn metal(tier: Option<usize>) -> &'static str {
    match tier {
        Some(0..=2) => "gold",
        Some(3..=5) => "silver",
        Some(_) => "bronze",
        None => "none",
    }
}

const DEFS: &str = "<defs>\
<linearGradient id=\"tg-gold\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#fff3b0\"/><stop offset=\".45\" stop-color=\"#f5c542\"/><stop offset=\"1\" stop-color=\"#c98a0b\"/></linearGradient>\
<linearGradient id=\"tg-silver\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#ffffff\"/><stop offset=\".45\" stop-color=\"#cfd6de\"/><stop offset=\"1\" stop-color=\"#8a96a3\"/></linearGradient>\
<linearGradient id=\"tg-bronze\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#ffd9b8\"/><stop offset=\".45\" stop-color=\"#d9905a\"/><stop offset=\"1\" stop-color=\"#9a5a2c\"/></linearGradient>\
<linearGradient id=\"tg-none\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#d0d7de\" stop-opacity=\".6\"/><stop offset=\"1\" stop-color=\"#8c959f\" stop-opacity=\".6\"/></linearGradient>\
<radialGradient id=\"tg-glow\"><stop offset=\"0\" stop-color=\"#ffd84d\" stop-opacity=\".45\"/><stop offset=\"1\" stop-color=\"#ffd84d\" stop-opacity=\"0\"/></radialGradient>\
</defs>";

fn cup(t: &Trophy) -> String {
    let fill = format!("url(#tg-{})", metal(t.tier));
    let glow = if matches!(t.tier, Some(0 | 1)) {
        "<circle cx=\"20\" cy=\"18\" r=\"28\" fill=\"url(#tg-glow)\"/>"
    } else {
        ""
    };
    let size = if t.rank().len() >= 3 { 8.5 } else { 10.5 };
    format!(
        "<g transform=\"translate(29,27) scale(1.3)\">{glow}\
         <path d=\"M7 8H3.5a4.5 4.5 0 0 0 5 8.5M33 8h3.5a4.5 4.5 0 0 1-5 8.5\" fill=\"none\" stroke=\"{fill}\" stroke-width=\"2.6\" stroke-linecap=\"round\"/>\
         <path d=\"M7 3h26v10c0 8.3-5.8 14-13 14S7 21.3 7 13z\" fill=\"{fill}\"/>\
         <path d=\"M17 27h6l1 5h-8z\" fill=\"{fill}\"/>\
         <rect x=\"11\" y=\"32\" width=\"18\" height=\"5\" rx=\"1.6\" fill=\"{fill}\"/>\
         <text x=\"20\" y=\"{y}\" text-anchor=\"middle\" fill=\"#2b2111\" fill-opacity=\".78\" font-size=\"{size}\" font-weight=\"800\">{rank}</text></g>",
        y = if size < 9.0 { 17.0 } else { 17.8 },
        rank = esc(t.rank()),
    )
}

fn panel(t: &Trophy, x: u32, y: u32, p: &Palette, o: &TrophyOptions, fill: &str) -> String {
    let bg = if o.no_bg { "none" } else { fill };
    let stroke = if o.no_frame {
        "stroke-opacity=\"0\"".to_string()
    } else {
        format!("stroke=\"{}\"", p.border)
    };
    let tier = t.tier.map(|i| TIER_NAMES[i]).unwrap_or("Not yet");
    format!(
        "<g transform=\"translate({x},{y})\">\
         <rect x=\"0.5\" y=\"0.5\" rx=\"6\" width=\"{w}\" height=\"{w}\" fill=\"{bg}\" {stroke}/>\
         <text x=\"55\" y=\"21\" text-anchor=\"middle\" fill=\"{title}\" font-size=\"12\" font-weight=\"600\">{name}</text>\
         {cup}\
         <text x=\"55\" y=\"89\" text-anchor=\"middle\" fill=\"{text}\" font-size=\"10.5\" font-weight=\"600\">{tier}</text>\
         <text x=\"55\" y=\"102\" text-anchor=\"middle\" fill=\"{text}\" fill-opacity=\".7\" font-size=\"10\">{value}</text></g>",
        w = PANEL - 1,
        title = p.title,
        text = p.text,
        name = esc(t.title),
        cup = cup(t),
        value = esc(&t.value),
    )
}

fn keep(t: &Trophy, o: &TrophyOptions) -> bool {
    let title_ok = o.titles.is_empty() || o.titles.iter().any(|w| w.eq_ignore_ascii_case(t.title));
    let rank_ok = o.ranks.is_empty() || o.ranks.iter().any(|r| r.eq_ignore_ascii_case(t.rank()));
    title_ok && rank_ok
}

/// Grid geometry for `n` panels: (columns, rows).
fn grid(n: usize, o: &TrophyOptions) -> (u32, u32) {
    let n = n.max(1) as u32;
    let cols = if o.column < 1 {
        n
    } else {
        (o.column as u32).min(n)
    }
    .clamp(1, 30);
    let rows = n.div_ceil(cols).min(o.row.clamp(1, 10));
    (cols, rows)
}

pub(crate) fn render(all: &[Trophy], p: &Palette, o: &TrophyOptions, label: &str) -> String {
    let shown: Vec<&Trophy> = all.iter().filter(|t| keep(t, o)).collect();
    let (cols, rows) = grid(shown.len(), o);
    let (mw, mh) = (o.margin_w.min(100), o.margin_h.min(100));
    let width = cols * PANEL + (cols - 1) * mw;
    let height = rows * PANEL + (rows - 1) * mh;
    let mut body = format!("<title>{}</title>{DEFS}", esc(label));
    let (defs, fill) = background(&p.bg);
    body.push_str(&defs);
    body.push_str(&format!("<g font-family=\"{FONT_UI_SANS}\">"));
    for (i, t) in shown.iter().take((cols * rows) as usize).enumerate() {
        let (c, r) = (i as u32 % cols, i as u32 / cols);
        body.push_str(&panel(t, c * (PANEL + mw), r * (PANEL + mh), p, o, &fill));
    }
    body.push_str("</g>");
    svg_doc(width, height, &body)
}

/// Fallback geometry: the default grid a trophy URL would have rendered.
pub(crate) fn fallback_size(o: &TrophyOptions) -> (u32, u32) {
    let (cols, _) = grid(7, o);
    (cols * PANEL + (cols - 1) * o.margin_w.min(100), PANEL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::live::domain::model::CommitSource;
    use crate::capabilities::live::domain::palette::palette;

    fn stats() -> UserStats {
        UserStats {
            login: "octo".into(),
            name: None,
            stars: Some(2809),
            followers: 318,
            repos: Some(6),
            created_at: Some("2016-03-14T09:26:53Z".into()),
            commits: Some((1123, CommitSource::ContributionsLastYear)),
            prs: Some(142),
            issues: Some(0),
            reviews: None,
            contributed_to: None,
        }
    }

    #[test]
    fn ranks_follow_thresholds_and_sort_best_first() {
        let now = parse_timestamp("2026-09-25T00:00:00Z").unwrap();
        let ts = trophies(&stats(), now);
        assert_eq!(ts[0].title, "Stars");
        assert_eq!(ts[0].rank(), "SSS");
        let issues = ts.iter().find(|t| t.title == "Issues").unwrap();
        assert_eq!(issues.rank(), "?");
        assert_eq!(ts.last().unwrap().title, "Issues");
        let exp = ts.iter().find(|t| t.title == "Experience").unwrap();
        assert_eq!(exp.rank(), "S");
        assert_eq!(exp.value, "10.5 yrs");
    }

    #[test]
    fn grid_filters_and_sizes() {
        let now = parse_timestamp("2026-09-25T00:00:00Z").unwrap();
        let ts = trophies(&stats(), now);
        let p = palette(None);
        let o = TrophyOptions {
            column: 3,
            row: 1,
            margin_w: 10,
            ..Default::default()
        };
        let svg = render(&ts, &p, &o, "t");
        assert!(
            svg.contains("width=\"350\" height=\"110\""),
            "3 panels + 2 gaps"
        );
        let only = TrophyOptions {
            titles: vec!["followers".into()],
            ..Default::default()
        };
        let svg = render(&ts, &p, &only, "t");
        assert!(svg.contains(">Followers<") && !svg.contains(">Stars<"));
        let ranked = TrophyOptions {
            ranks: vec!["SSS".into()],
            column: -1,
            ..Default::default()
        };
        assert!(render(&ts, &p, &ranked, "t").contains("width=\"110\""));
    }
}
