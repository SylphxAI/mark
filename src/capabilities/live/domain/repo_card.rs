//! Repository pin card: name, description, language, stars, forks.

use super::card::{frame, icon, icons, n2, wrap, CardStyle, PAD_X};
use super::format::metric;
use super::languages;
use super::model::RepoInfo;
use crate::capabilities::mark::domain::svg::esc;
use crate::capabilities::mark::domain::text::{fit_line, line_advance, Metric};

pub(crate) const WIDTH: u32 = 400;
const DESC_SIZE: f32 = 13.0;
const DESC_STEP: f32 = 19.0;

#[derive(Debug, Clone, Default)]
pub(crate) struct RepoOptions {
    pub show_owner: bool,
    /// 1..=3 description lines; `None` fits the text (up to 3).
    pub description_lines: Option<usize>,
}

fn height(lines: usize) -> u32 {
    (70.0 + lines.max(1) as f32 * DESC_STEP + 26.0) as u32
}

fn tag(style: &CardStyle, x: f32, text: &str) -> String {
    let w = line_advance(text, 11.0, Metric::Display) + 14.0;
    format!(
        "<rect x=\"{}\" y=\"21\" width=\"{}\" height=\"19\" rx=\"9.5\" fill=\"none\" stroke=\"{}\" stroke-opacity=\"0.5\"/>\
         <text x=\"{}\" y=\"34.5\" text-anchor=\"middle\" fill=\"{}\" font-size=\"11\">{}</text>",
        n2(x),
        n2(w),
        style.palette.text,
        n2(x + w / 2.0),
        style.palette.text,
        esc(text)
    )
}

pub(crate) fn render(repo: &RepoInfo, style: &CardStyle, o: &RepoOptions) -> String {
    let width = style.width(WIDTH, 280) as f32;
    let p = &style.palette;
    let name = if o.show_owner {
        format!("{}/{}", repo.owner, repo.name)
    } else {
        repo.name.clone()
    };
    let badge = if repo.archived {
        Some("Archived")
    } else if repo.template {
        Some("Template")
    } else {
        None
    };
    let badge_w = badge
        .map(|b| line_advance(b, 11.0, Metric::Display) + 24.0)
        .unwrap_or(0.0);
    let name_max = width - 2.0 * PAD_X - 24.0 - badge_w;
    let shown = fit_line(&style.title_or(name), name_max, 16.0, Metric::Bold);
    let mut body = icon(icons::REPO, PAD_X, 22.0, 16.0, &p.icon);
    body.push_str(&format!(
        "<text x=\"{}\" y=\"36\" fill=\"{}\" font-size=\"16\" font-weight=\"600\">{}</text>",
        n2(PAD_X + 24.0),
        p.title,
        esc(&shown)
    ));
    if let Some(b) = badge {
        let x = PAD_X + 24.0 + line_advance(&shown, 16.0, Metric::Bold) + 10.0;
        body.push_str(&tag(style, x, b));
    }
    let desc = repo
        .description
        .as_deref()
        .filter(|d| !d.trim().is_empty())
        .unwrap_or("No description provided");
    let cap = o.description_lines.unwrap_or(3).clamp(1, 3);
    let mut lines = wrap(desc, width - 2.0 * PAD_X, DESC_SIZE, cap);
    if let Some(fixed) = o.description_lines {
        lines.truncate(fixed.clamp(1, 3));
    }
    for (i, line) in lines.iter().enumerate() {
        body.push_str(&format!(
            "<text x=\"{PAD_X}\" y=\"{}\" fill=\"{}\" font-size=\"{DESC_SIZE}\">{}</text>",
            n2(66.0 + i as f32 * DESC_STEP),
            p.text,
            esc(line)
        ));
    }
    let slots = o
        .description_lines
        .map(|n| n.clamp(1, 3))
        .unwrap_or(lines.len());
    let h = height(slots);
    let fy = h as f32 - 22.0;
    let mut x = PAD_X;
    if let Some(lang) = repo.language.as_deref() {
        let color = repo
            .language_color
            .clone()
            .unwrap_or_else(|| languages::color(lang));
        body.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"6\" fill=\"{color}\"/>\
             <text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"12\">{}</text>",
            n2(x + 6.0),
            n2(fy - 4.0),
            n2(x + 17.0),
            n2(fy),
            p.text,
            esc(lang)
        ));
        x += 17.0 + line_advance(lang, 12.0, Metric::Display) + 20.0;
    }
    for (path, n) in [(icons::STAR, repo.stars), (icons::FORK, repo.forks)] {
        let label = metric(n);
        body.push_str(&icon(path, x, fy - 12.0, 16.0, &p.icon));
        body.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"12\">{label}</text>",
            n2(x + 21.0),
            n2(fy),
            p.text
        ));
        x += 21.0 + line_advance(&label, 12.0, Metric::Display) + 20.0;
    }
    let mut untitled = style.clone();
    untitled.hide_title = true;
    frame(
        &untitled,
        width as u32,
        h,
        &format!("{}/{}", repo.owner, repo.name),
        &body,
    )
}

pub(crate) fn fallback_size(style: &CardStyle, o: &RepoOptions) -> (u32, u32) {
    (
        style.width(WIDTH, 280),
        height(o.description_lines.unwrap_or(2)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::live::domain::palette::ColorOverrides;

    #[test]
    fn repo_card_shows_owner_badge_and_counts() {
        let repo = RepoInfo {
            owner: "SylphxAI".into(),
            name: "mark".into(),
            description: Some("Beautiful README images from one URL".into()),
            language: Some("Rust".into()),
            language_color: None,
            stars: 1520,
            forks: 12,
            size: 1,
            fork: false,
            archived: true,
            template: false,
            license: None,
            pushed_at: None,
        };
        let style = CardStyle::new(None, &ColorOverrides::default());
        let svg = render(
            &repo,
            &style,
            &RepoOptions {
                show_owner: true,
                description_lines: None,
            },
        );
        assert!(svg.contains("SylphxAI/mark"));
        assert!(svg.contains("Archived"));
        assert!(svg.contains("1.5k"));
        assert!(svg.contains("#dea584"));
    }
}
