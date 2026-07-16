//! Stats / repo / org cards.

use crate::color::resolve_fill;
use crate::github::{self, Aggregate, GhRepo, GhUser};
use crate::svg::{credit_mark, ensure_hash, esc, svg_doc};
use crate::themes;

#[derive(Debug, Clone)]
pub struct CardOpts {
    pub theme: Option<String>,
    pub color: Option<String>,
    pub credit: bool,
    pub width: u32,
}

impl Default for CardOpts {
    fn default() -> Self {
        Self {
            theme: Some("dark".into()),
            color: None,
            credit: true,
            width: 420,
        }
    }
}

pub async fn user_stats(username: &str, opts: &CardOpts) -> Result<String, String> {
    let user = github::get_user(username).await?;
    let repos = github::get_user_repos(username).await.unwrap_or_default();
    let agg = github::aggregate(&repos);
    Ok(render_user_card(&user, &agg, opts))
}

pub async fn org_stats(org: &str, opts: &CardOpts) -> Result<String, String> {
    let repos = github::get_org_repos(org).await?;
    let agg = github::aggregate(&repos);
    Ok(render_org_card(org, &agg, opts))
}

pub async fn repo_card(owner: &str, repo: &str, opts: &CardOpts) -> Result<String, String> {
    let r = github::get_repo(owner, repo).await?;
    Ok(render_repo_card(&r, opts))
}

fn palette(opts: &CardOpts, seed: &str) -> (String, String, String, String) {
    if let Some(name) = opts.theme.as_deref() {
        if let Some(t) = themes::get(name) {
            return (
                ensure_hash(t.bg),
                ensure_hash(t.fg),
                ensure_hash(t.muted),
                ensure_hash(t.accent),
            );
        }
    }
    let fill = resolve_fill(opts.color.as_deref(), None, seed, "cg");
    let bg = if fill.fill.starts_with("url") {
        ensure_hash("0D1117")
    } else {
        fill.fill.clone()
    };
    (
        bg,
        ensure_hash(&fill.fg),
        ensure_hash("8B949E"),
        ensure_hash("58A6FF"),
    )
}

fn render_user_card(user: &GhUser, agg: &Aggregate, opts: &CardOpts) -> String {
    let w = opts.width.clamp(280, 800);
    let h = 160u32;
    let (bg, fg, muted, accent) = palette(opts, &user.login);
    let title = user.name.as_deref().unwrap_or(&user.login);
    let bio = user
        .bio
        .as_deref()
        .unwrap_or("GitHub profile")
        .chars()
        .take(64)
        .collect::<String>();

    let metrics = [
        ("Repos", format!("{}", user.public_repos)),
        ("Stars", format_num(agg.stars)),
        ("Followers", format_num(user.followers)),
        ("Forks", format_num(agg.forks)),
    ];

    let mut bars = String::new();
    let bar_x = 24.0_f32;
    let bar_w = (w as f32 - 48.0).max(100.0);
    let mut y = 118.0_f32;
    for (name, _count, pct) in agg.top_langs.iter().take(3) {
        let pw = bar_w * (*pct as f32 / 100.0).max(0.05);
        bars.push_str(&format!(
            "<text x=\"{bar_x}\" y=\"{y}\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"11\" fill=\"{muted}\">{}</text>\
             <rect x=\"{}\" y=\"{}\" width=\"{bar_w}\" height=\"6\" rx=\"3\" fill=\"#ffffff\" fill-opacity=\"0.08\"/>\
             <rect x=\"{}\" y=\"{}\" width=\"{pw}\" height=\"6\" rx=\"3\" fill=\"{accent}\"/>",
            esc(name),
            bar_x + 70.0,
            y - 6.0,
            bar_x + 70.0,
            y - 6.0,
        ));
        y += 14.0;
    }

    let mut metric_xml = String::new();
    let slot = (w as f32 - 48.0) / metrics.len() as f32;
    for (i, (label, val)) in metrics.iter().enumerate() {
        let x = 24.0 + i as f32 * slot;
        metric_xml.push_str(&format!(
            "<text x=\"{x}\" y=\"78\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"18\" font-weight=\"700\" fill=\"{fg}\">{val}</text>\
             <text x=\"{x}\" y=\"96\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"11\" fill=\"{muted}\">{label}</text>"
        ));
    }

    let extra = if agg.top_langs.is_empty() { 0 } else { 20 };
    let body = format!(
        "<rect width=\"{w}\" height=\"{}\" rx=\"12\" fill=\"{bg}\"/>\
         <rect width=\"{w}\" height=\"{}\" rx=\"12\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.08\"/>\
         <text x=\"24\" y=\"32\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"18\" font-weight=\"700\" fill=\"{fg}\">{}</text>\
         <text x=\"24\" y=\"52\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{muted}\">@{} · {}</text>\
         {metric_xml}{bars}{}",
        h + extra,
        h + extra,
        esc(title),
        esc(&user.login),
        esc(&bio),
        credit_mark(w, h + extra, opts.credit)
    );
    svg_doc(w, h + extra, &body)
}

fn render_org_card(org: &str, agg: &Aggregate, opts: &CardOpts) -> String {
    let w = opts.width.clamp(280, 800);
    let h = 140u32;
    let (bg, fg, muted, accent) = palette(opts, org);

    let metrics = [
        ("Public repos", format!("{}", agg.repos)),
        ("Stars", format_num(agg.stars)),
        ("Forks", format_num(agg.forks)),
        (
            "Top lang",
            agg.top_langs
                .first()
                .map(|l| l.0.clone())
                .unwrap_or_else(|| "—".into()),
        ),
    ];

    let mut metric_xml = String::new();
    let slot = (w as f32 - 48.0) / metrics.len() as f32;
    for (i, (label, val)) in metrics.iter().enumerate() {
        let x = 24.0 + i as f32 * slot;
        metric_xml.push_str(&format!(
            "<text x=\"{x}\" y=\"88\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"18\" font-weight=\"700\" fill=\"{fg}\">{}</text>\
             <text x=\"{x}\" y=\"108\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"11\" fill=\"{muted}\">{label}</text>",
            esc(val)
        ));
    }

    let body = format!(
        "<rect width=\"{w}\" height=\"{h}\" rx=\"12\" fill=\"{bg}\"/>\
         <rect x=\"0\" y=\"0\" width=\"6\" height=\"{h}\" fill=\"{accent}\"/>\
         <text x=\"24\" y=\"36\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"20\" font-weight=\"700\" fill=\"{fg}\">{}</text>\
         <text x=\"24\" y=\"56\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{muted}\">organization · powered by Mark</text>\
         {metric_xml}{}",
        esc(org),
        credit_mark(w, h, opts.credit)
    );
    svg_doc(w, h, &body)
}

fn render_repo_card(r: &GhRepo, opts: &CardOpts) -> String {
    let w = opts.width.clamp(280, 800);
    let h = 120u32;
    let (bg, fg, muted, accent) = palette(opts, &r.full_name);
    let desc = r
        .description
        .as_deref()
        .unwrap_or("No description")
        .chars()
        .take(90)
        .collect::<String>();
    let lang = r.language.as_deref().unwrap_or("—");
    let lic = r
        .license
        .as_ref()
        .and_then(|l| l.spdx_id.as_deref())
        .unwrap_or("—");

    let body = format!(
        "<rect width=\"{w}\" height=\"{h}\" rx=\"12\" fill=\"{bg}\"/>\
         <rect width=\"{w}\" height=\"{h}\" rx=\"12\" fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.08\"/>\
         <text x=\"24\" y=\"34\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"{fg}\">{}</text>\
         <text x=\"24\" y=\"56\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{muted}\">{}</text>\
         <circle cx=\"30\" cy=\"88\" r=\"5\" fill=\"{accent}\"/>\
         <text x=\"42\" y=\"92\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{fg}\">{}</text>\
         <text x=\"120\" y=\"92\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{muted}\">★ {}</text>\
         <text x=\"200\" y=\"92\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{muted}\">forks {}</text>\
         <text x=\"300\" y=\"92\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{muted}\">{}</text>\
         {}",
        esc(&r.full_name),
        esc(&desc),
        esc(lang),
        format_num(r.stargazers_count),
        format_num(r.forks_count),
        esc(lic),
        credit_mark(w, h, opts.credit)
    );
    svg_doc(w, h, &body)
}

fn format_num(n: u32) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f32 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f32 / 1_000.0)
    } else {
        n.to_string()
    }
}

/// Deployed-on-Sylphx promo badge (static template).
pub fn deploy_badge(service: &str, theme: Option<&str>, style: &str) -> String {
    crate::badge::render(&crate::badge::BadgeInput {
        label: Some("deployed on".into()),
        message: if service.is_empty() {
            "Sylphx".into()
        } else {
            format!("{service} · Sylphx")
        },
        color: Some("sylphx".into()),
        label_color: Some("1A1A2E".into()),
        style: crate::badge::BadgeStyle::parse(style),
        theme: theme.map(|s| s.to_string()),
    })
}
