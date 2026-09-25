//! Top-languages card: normal, compact, donut, donut-vertical, and pie layouts.

use super::card::{body_top, frame, n2, CardStyle, PAD_X};
use super::languages::LangShare;
use super::model::TopLangs;
use crate::capabilities::mark::domain::svg::esc;
use crate::capabilities::mark::domain::text::{fit_line, line_advance, Metric};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Layout {
    #[default]
    Normal,
    Compact,
    Donut,
    DonutVertical,
    Pie,
}

impl Layout {
    /// A layout name (`-` or `_` separated); unknown names are `normal`.
    pub(crate) fn parse(raw: Option<&str>) -> Self {
        const NAMES: [(&str, Layout); 4] = [
            ("compact", Layout::Compact),
            ("donut", Layout::Donut),
            ("donut_vertical", Layout::DonutVertical),
            ("pie", Layout::Pie),
        ];
        let want = raw
            .unwrap_or_default()
            .to_ascii_lowercase()
            .replace('-', "_");
        NAMES
            .iter()
            .find(|(n, _)| *n == want)
            .map(|(_, l)| *l)
            .unwrap_or_default()
    }

    fn default_count(self) -> usize {
        match self {
            Self::Normal | Self::Donut | Self::DonutVertical | Self::Pie => 5,
            Self::Compact => 6,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LangsOptions {
    pub layout: Layout,
    pub langs_count: Option<usize>,
    pub hide_progress: bool,
}

pub(crate) const TITLE: &str = "Most Used Languages";

fn percent(share: &LangShare, total: u64) -> f32 {
    if total == 0 {
        0.0
    } else {
        share.weight as f32 / total as f32 * 100.0
    }
}

fn pct_label(p: f32) -> String {
    format!("{p:.2}%")
}

fn legend_item(
    style: &CardStyle,
    x: f32,
    y: f32,
    share: &LangShare,
    pct: f32,
    max_w: f32,
) -> String {
    // Dot, gap, and headroom for wide fallback fonts, then the percentage.
    let pct_w = line_advance(&format!(" {}", pct_label(pct)), 12.0, Metric::Display);
    let budget = ((max_w - 28.0) * 0.88 - pct_w).max(30.0);
    let label = fit_line(&share.name, budget, 12.0, Metric::Display);
    format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{}\"/>\
         <text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"12\">{} <tspan fill-opacity=\"0.7\">{}</tspan></text>",
        n2(x + 5.0),
        n2(y - 4.0),
        share.color,
        n2(x + 16.0),
        n2(y),
        style.palette.text,
        esc(&label),
        pct_label(pct)
    )
}

fn normal(
    style: &CardStyle,
    width: f32,
    top: f32,
    langs: &[LangShare],
    total: u64,
    bars: bool,
) -> (String, f32) {
    let mut body = String::new();
    let step = if bars { 40.0 } else { 25.0 };
    let bar_w = width - 2.0 * PAD_X;
    for (i, share) in langs.iter().enumerate() {
        let y = top + 12.0 + i as f32 * step;
        let pct = percent(share, total);
        body.push_str(&format!(
            "<text x=\"{PAD_X}\" y=\"{}\" fill=\"{}\" font-size=\"13\" font-weight=\"500\">{}</text>\
             <text x=\"{}\" y=\"{}\" text-anchor=\"end\" fill=\"{}\" fill-opacity=\"0.8\" font-size=\"12\">{}</text>",
            n2(y),
            style.palette.text,
            esc(&fit_line(&share.name, bar_w - 70.0, 13.0, Metric::Display)),
            n2(width - PAD_X),
            n2(y),
            style.palette.text,
            pct_label(pct)
        ));
        if bars {
            body.push_str(&format!(
                "<rect x=\"{PAD_X}\" y=\"{}\" width=\"{}\" height=\"8\" rx=\"4\" fill=\"{}\" fill-opacity=\"0.12\"/>\
                 <rect x=\"{PAD_X}\" y=\"{}\" width=\"{}\" height=\"8\" rx=\"4\" fill=\"{}\"/>",
                n2(y + 9.0),
                n2(bar_w),
                style.palette.text,
                n2(y + 9.0),
                n2((bar_w * pct / 100.0).max(8.0)),
                share.color
            ));
        }
    }
    (body, top + langs.len() as f32 * step + 6.0)
}

fn compact(
    style: &CardStyle,
    width: f32,
    top: f32,
    langs: &[LangShare],
    total: u64,
    bar: bool,
) -> (String, f32) {
    let mut body = String::new();
    let bar_w = width - 2.0 * PAD_X;
    let mut y = top;
    if bar {
        body.push_str(&format!(
            "<clipPath id=\"lbar\"><rect x=\"{PAD_X}\" y=\"{}\" width=\"{}\" height=\"8\" rx=\"4\"/></clipPath><g clip-path=\"url(#lbar)\">",
            n2(top),
            n2(bar_w)
        ));
        let mut x = PAD_X;
        for share in langs {
            let w = bar_w * percent(share, total) / 100.0;
            body.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"8\" fill=\"{}\"/>",
                n2(x),
                n2(top),
                n2(w + 0.5),
                share.color
            ));
            x += w;
        }
        body.push_str("</g>");
        y += 30.0;
    } else {
        y += 12.0;
    }
    let col_w = bar_w / 2.0;
    for (i, share) in langs.iter().enumerate() {
        let (col, row) = (i % 2, i / 2);
        body.push_str(&legend_item(
            style,
            PAD_X + col as f32 * col_w,
            y + row as f32 * 25.0,
            share,
            percent(share, total),
            col_w,
        ));
    }
    (body, y + langs.len().div_ceil(2) as f32 * 25.0 - 8.0)
}

/// Donut (or pie when `inner` is 0) slices around (cx, cy).
fn slices(cx: f32, cy: f32, outer: f32, inner: f32, langs: &[LangShare], total: u64) -> String {
    let mut out = String::new();
    if langs.len() == 1 || total == 0 {
        let color = langs.first().map(|s| s.color.as_str()).unwrap_or("#8b949e");
        let ring = if inner > 0.0 {
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{}\"/>",
                n2(cx),
                n2(cy),
                n2((outer + inner) / 2.0),
                n2(outer - inner)
            )
        } else {
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{color}\"/>",
                n2(cx),
                n2(cy),
                n2(outer)
            )
        };
        return ring;
    }
    let mut start = -std::f32::consts::FRAC_PI_2;
    let pt = |r: f32, a: f32| (n2(cx + r * a.cos()), n2(cy + r * a.sin()));
    for share in langs {
        let sweep = std::f32::consts::TAU * share.weight as f32 / total as f32;
        let end = start + sweep;
        let large = if sweep > std::f32::consts::PI { 1 } else { 0 };
        let (ox1, oy1) = pt(outer, start);
        let (ox2, oy2) = pt(outer, end);
        let d = if inner > 0.0 {
            let (ix2, iy2) = pt(inner, end);
            let (ix1, iy1) = pt(inner, start);
            format!(
                "M{ox1} {oy1}A{o} {o} 0 {large} 1 {ox2} {oy2}L{ix2} {iy2}A{i} {i} 0 {large} 0 {ix1} {iy1}Z",
                o = n2(outer),
                i = n2(inner)
            )
        } else {
            format!(
                "M{} {}L{ox1} {oy1}A{o} {o} 0 {large} 1 {ox2} {oy2}Z",
                n2(cx),
                n2(cy),
                o = n2(outer)
            )
        };
        out.push_str(&format!("<path d=\"{d}\" fill=\"{}\"/>", share.color));
        start = end;
    }
    out
}

fn chart(
    style: &CardStyle,
    layout: Layout,
    width: f32,
    top: f32,
    langs: &[LangShare],
    total: u64,
) -> (String, f32) {
    let inner_ratio = if layout == Layout::Pie { 0.0 } else { 0.6 };
    let mut body = String::new();
    if layout == Layout::DonutVertical {
        let outer = 70.0;
        let cy = top + outer + 4.0;
        body.push_str(&slices(
            width / 2.0,
            cy,
            outer,
            outer * inner_ratio,
            langs,
            total,
        ));
        let (legend, bottom) = compact(style, width, cy + outer + 4.0, langs, total, false);
        body.push_str(&legend);
        return (body, bottom);
    }
    let outer = 60.0;
    let rows = langs.len().max(1) as f32;
    let legend_h = rows * 24.0;
    let area_h = legend_h.max(2.0 * outer + 10.0);
    let cx = width - PAD_X - outer;
    let cy = top + area_h / 2.0;
    body.push_str(&slices(cx, cy, outer, outer * inner_ratio, langs, total));
    let legend_top = top + (area_h - legend_h) / 2.0 + 16.0;
    for (i, share) in langs.iter().enumerate() {
        body.push_str(&legend_item(
            style,
            PAD_X,
            legend_top + i as f32 * 24.0,
            share,
            percent(share, total),
            width - 2.0 * PAD_X - 2.0 * outer - 10.0,
        ));
    }
    (body, top + area_h)
}

pub(crate) fn render(data: &TopLangs, style: &CardStyle, o: &LangsOptions) -> String {
    let count = o
        .langs_count
        .unwrap_or(o.layout.default_count())
        .clamp(1, 20);
    let langs: Vec<LangShare> = data.langs.iter().take(count).cloned().collect();
    let total: u64 = langs.iter().map(|l| l.weight).sum();
    let width = style.width(default_width(o.layout), 230) as f32;
    let top = body_top(style);
    let (body, bottom) = if langs.is_empty() {
        (
            format!(
                "<text x=\"{PAD_X}\" y=\"{}\" fill=\"{}\" fill-opacity=\"0.75\" font-size=\"13\">No public languages yet.</text>",
                n2(top + 14.0),
                style.palette.text
            ),
            top + 24.0,
        )
    } else {
        match o.layout {
            Layout::Normal => normal(style, width, top, &langs, total, !o.hide_progress),
            Layout::Compact => compact(style, width, top, &langs, total, !o.hide_progress),
            Layout::Donut | Layout::DonutVertical | Layout::Pie => {
                chart(style, o.layout, width, top, &langs, total)
            }
        }
    };
    let height = (bottom + 18.0).max(90.0) as u32;
    frame(
        style,
        width as u32,
        height,
        &style.title_or(TITLE.into()),
        &body,
    )
}

fn default_width(layout: Layout) -> u32 {
    match layout {
        Layout::Normal | Layout::Compact => 300,
        Layout::Donut | Layout::Pie => 340,
        Layout::DonutVertical => 300,
    }
}

pub(crate) fn fallback_size(style: &CardStyle, o: &LangsOptions) -> (u32, u32) {
    let height = match o.layout {
        Layout::Normal => 245,
        Layout::Compact => 160,
        Layout::Donut | Layout::Pie => 200,
        Layout::DonutVertical => 330,
    };
    (style.width(default_width(o.layout), 230), height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::live::domain::model::LangSource;
    use crate::capabilities::live::domain::palette::ColorOverrides;

    fn data(n: usize) -> TopLangs {
        let names = ["Rust", "TypeScript", "Python", "Go", "Shell", "HTML", "CSS"];
        TopLangs {
            login: "octo".into(),
            langs: names
                .iter()
                .take(n)
                .enumerate()
                .map(|(i, name)| LangShare {
                    name: (*name).into(),
                    color: "#123456".into(),
                    weight: 100 - i as u64 * 10,
                })
                .collect(),
            source: LangSource::PrimaryBySize,
        }
    }

    #[test]
    fn every_layout_renders_and_count_caps() {
        let style = CardStyle::new(None, &ColorOverrides::default());
        for layout in [
            "normal",
            "compact",
            "donut",
            "donut-vertical",
            "pie",
            "bogus",
        ] {
            let o = LangsOptions {
                layout: Layout::parse(Some(layout)),
                langs_count: Some(3),
                ..Default::default()
            };
            let svg = render(&data(7), &style, &o);
            assert!(svg.contains("Rust") && svg.contains("Python"), "{layout}");
            assert!(!svg.contains("Go"), "langs_count caps: {layout}");
        }
        let single = render(
            &data(1),
            &style,
            &LangsOptions {
                layout: Layout::Donut,
                ..Default::default()
            },
        );
        assert!(single.contains("100.00%"));
        let empty = render(&data(0), &style, &LangsOptions::default());
        assert!(empty.contains("No public languages"));
    }
}
