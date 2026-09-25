//! Stats card: stars, commits, PRs, issues, contributed-to, and a rank ring.

use super::card::{body_top, frame, icon, icons, n2, CardStyle, PAD_X};
use super::format::card_number;
use super::model::{CommitSource, UserStats};
use super::rank::{calculate, RankInput};
use crate::capabilities::mark::domain::svg::esc;
use crate::capabilities::mark::domain::text::{fit_line, Metric};

pub(crate) const WIDTH: u32 = 495;
pub(crate) const HEIGHT: u32 = 195;
const ROW_H: f32 = 26.0;
const FONT: f32 = 14.0;
const RING_R: f32 = 40.0;

#[derive(Debug, Clone, Default)]
pub(crate) struct StatsOptions {
    /// Row ids to hide: `stars`, `commits`, `prs`, `issues`, `contribs`.
    pub hide: Vec<String>,
    pub show_icons: bool,
    pub hide_rank: bool,
}

/// "Ada's" / "James'" — github-readme-stats' possessive rule.
pub(crate) fn possessive(name: &str) -> String {
    let tail = name.chars().last().map(|c| c.to_ascii_lowercase());
    if matches!(tail, Some('s') | Some('x')) {
        format!("{name}'")
    } else {
        format!("{name}'s")
    }
}

pub(crate) fn default_title(display: &str) -> String {
    format!("{} GitHub Stats", possessive(display))
}

fn rows(data: &UserStats, hide: &[String]) -> Vec<(&'static str, &'static str, String, u64)> {
    let hidden = |id: &str| hide.iter().any(|h| h.eq_ignore_ascii_case(id));
    let mut out = Vec::new();
    let mut push = |id: &'static str, path: &'static str, label: String, v: Option<u64>| {
        if let (false, Some(v)) = (hidden(id), v) {
            out.push((id, path, label, v));
        }
    };
    push(
        "stars",
        icons::STAR,
        "Total Stars Earned".into(),
        data.stars,
    );
    if let Some((n, source)) = data.commits {
        let label = match source {
            CommitSource::CommitsLastYear => "Total Commits (last year)",
            CommitSource::ContributionsLastYear => "Contributions (last year)",
        };
        push("commits", icons::COMMITS, label.into(), Some(n));
    }
    push("prs", icons::PULL, "Total PRs".into(), data.prs);
    push("issues", icons::ISSUE, "Total Issues".into(), data.issues);
    push(
        "contribs",
        icons::REPO,
        "Contributed to (last year)".into(),
        data.contributed_to,
    );
    out
}

fn ring(style: &CardStyle, cx: f32, cy: f32, data: &UserStats) -> String {
    let rank = calculate(&RankInput {
        all_commits: false,
        commits: data.commits.map(|c| c.0).unwrap_or(0),
        prs: data.prs.unwrap_or(0),
        issues: data.issues.unwrap_or(0),
        reviews: data.reviews.unwrap_or(0),
        stars: data.stars.unwrap_or(0),
        followers: data.followers,
    });
    let p = &style.palette;
    let circumference = 2.0 * std::f32::consts::PI * RING_R;
    let progress = (100.0 - rank.percentile as f32).clamp(0.0, 100.0) / 100.0;
    let pct = rank.percentile.clamp(0.1, 100.0);
    let top = if pct < 10.0 {
        format!("Top {pct:.1}%")
    } else {
        format!("Top {pct:.0}%")
    };
    format!(
        "<g transform=\"translate({cx},{cy})\">\
         <circle r=\"{RING_R}\" fill=\"none\" stroke=\"{title}\" stroke-opacity=\"0.18\" stroke-width=\"6\"/>\
         <circle r=\"{RING_R}\" fill=\"none\" stroke=\"{title}\" stroke-width=\"6\" stroke-linecap=\"round\" \
         stroke-dasharray=\"{dash} {circ}\" transform=\"rotate(-90)\"/>\
         <text y=\"4\" text-anchor=\"middle\" fill=\"{text}\" font-size=\"24\" font-weight=\"700\">{level}</text>\
         <text y=\"22\" text-anchor=\"middle\" fill=\"{text}\" fill-opacity=\"0.7\" font-size=\"10\">{top}</text></g>",
        cx = n2(cx),
        cy = n2(cy),
        title = p.title,
        text = p.text,
        dash = n2(circumference * progress),
        circ = n2(circumference),
        level = rank.level,
        top = esc(&top),
    )
}

pub(crate) fn render(data: &UserStats, style: &CardStyle, o: &StatsOptions) -> String {
    let rows = rows(data, &o.hide);
    let width = style.width(
        if o.hide_rank { 380 } else { WIDTH },
        if o.hide_rank { 280 } else { 400 },
    );
    let top = body_top(style);
    let rows_h = rows.len() as f32 * ROW_H;
    let ring_h = if o.hide_rank {
        0.0
    } else {
        2.0 * RING_R + 30.0
    };
    let height =
        (top + rows_h.max(ring_h) + 14.0).max(if style.hide_title { 120.0 } else { 150.0 }) as u32;
    let p = &style.palette;
    let value_right = if o.hide_rank {
        width as f32 - PAD_X
    } else {
        width as f32 - PAD_X - 2.0 * RING_R - 34.0
    };
    let label_x = if o.show_icons { PAD_X + 24.0 } else { PAD_X };
    let mut body = String::new();
    let block_top = top + ((height as f32 - top - 14.0) - rows_h).max(0.0) / 2.0;
    for (i, (id, path, label, value)) in rows.iter().enumerate() {
        let y = block_top + i as f32 * ROW_H;
        let value = card_number(*value);
        let label_max = value_right - label_x - 12.0 - value.len() as f32 * 9.0;
        if o.show_icons {
            body.push_str(&icon(path, PAD_X, y + 4.0, 16.0, &p.icon));
        }
        body.push_str(&format!(
            "<g data-testid=\"{id}\"><text x=\"{lx}\" y=\"{by}\" fill=\"{text}\" font-size=\"{FONT}\">{label}</text>\
             <text x=\"{vx}\" y=\"{by}\" text-anchor=\"end\" fill=\"{text}\" font-size=\"{FONT}\" font-weight=\"700\">{value}</text></g>",
            lx = n2(label_x),
            by = n2(y + 17.0),
            text = p.text,
            label = esc(&fit_line(label, label_max.max(40.0), FONT, Metric::Display)),
            vx = n2(value_right),
        ));
    }
    if !o.hide_rank {
        let cx = width as f32 - PAD_X - RING_R - 6.0;
        let cy = top + (height as f32 - top - 14.0) / 2.0;
        body.push_str(&ring(style, cx, cy, data));
    }
    let display = data
        .name
        .as_deref()
        .filter(|n| !n.trim().is_empty())
        .unwrap_or(&data.login);
    let title = style.title_or(default_title(display));
    frame(style, width, height, &title, &body)
}

/// Fallback geometry: the card size a stats URL would have rendered.
pub(crate) fn fallback_size(style: &CardStyle, o: &StatsOptions) -> (u32, u32) {
    let width = style.width(
        if o.hide_rank { 380 } else { WIDTH },
        if o.hide_rank { 280 } else { 400 },
    );
    (width, HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::live::domain::palette::ColorOverrides;

    fn stats() -> UserStats {
        UserStats {
            login: "octo".into(),
            name: Some("Octo Cat".into()),
            stars: Some(1234),
            followers: 20,
            commits: Some((400, CommitSource::ContributionsLastYear)),
            prs: Some(30),
            issues: Some(5),
            reviews: None,
            contributed_to: None,
        }
    }

    #[test]
    fn rows_hide_unknown_values_and_requested_ids() {
        let style = CardStyle::new(None, &ColorOverrides::default());
        let svg = render(
            &stats(),
            &style,
            &StatsOptions {
                show_icons: true,
                ..Default::default()
            },
        );
        assert!(svg.contains("Octo Cat&apos;s GitHub Stats"));
        assert!(svg.contains("1,234"));
        assert!(svg.contains("Contributions (last year)"));
        assert!(!svg.contains("Contributed to"));
        let hidden = render(
            &stats(),
            &style,
            &StatsOptions {
                hide: vec!["stars".into()],
                hide_rank: true,
                ..Default::default()
            },
        );
        assert!(!hidden.contains("Total Stars"));
        assert!(!hidden.contains("<circle"));
        assert_eq!(possessive("James"), "James'");
    }
}
