//! Tech icon row — curated monochrome tiles.

use crate::svg::{esc, svg_doc};
use crate::themes;

fn glyph(id: &str) -> Option<&'static str> {
    Some(match id {
        "rust" => "<path d=\"M16 4 L26 10 L26 22 L16 28 L6 22 L6 10 Z\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><circle cx=\"16\" cy=\"16\" r=\"3\" fill=\"currentColor\"/>",
        "go" | "golang" => "<ellipse cx=\"16\" cy=\"16\" rx=\"12\" ry=\"8\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><circle cx=\"12\" cy=\"14\" r=\"1.5\" fill=\"currentColor\"/><circle cx=\"20\" cy=\"14\" r=\"1.5\" fill=\"currentColor\"/>",
        "ts" | "typescript" => "<rect x=\"5\" y=\"5\" width=\"22\" height=\"22\" rx=\"3\" fill=\"currentColor\"/><text x=\"16\" y=\"21\" text-anchor=\"middle\" font-size=\"11\" font-weight=\"700\" font-family=\"sans-serif\" fill=\"#0D1117\">TS</text>",
        "js" | "javascript" => "<rect x=\"5\" y=\"5\" width=\"22\" height=\"22\" rx=\"3\" fill=\"currentColor\"/><text x=\"16\" y=\"21\" text-anchor=\"middle\" font-size=\"11\" font-weight=\"700\" font-family=\"sans-serif\" fill=\"#0D1117\">JS</text>",
        "py" | "python" => "<circle cx=\"12\" cy=\"12\" r=\"6\" fill=\"currentColor\"/><circle cx=\"20\" cy=\"20\" r=\"6\" fill=\"currentColor\" fill-opacity=\"0.7\"/>",
        "react" => "<ellipse cx=\"16\" cy=\"16\" rx=\"12\" ry=\"5\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\" transform=\"rotate(0 16 16)\"/><ellipse cx=\"16\" cy=\"16\" rx=\"12\" ry=\"5\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\" transform=\"rotate(60 16 16)\"/><ellipse cx=\"16\" cy=\"16\" rx=\"12\" ry=\"5\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\" transform=\"rotate(120 16 16)\"/><circle cx=\"16\" cy=\"16\" r=\"2\" fill=\"currentColor\"/>",
        "node" | "nodejs" => "<path d=\"M16 4 L26 10 V22 L16 28 L6 22 V10 Z\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/>",
        "docker" => "<rect x=\"6\" y=\"14\" width=\"5\" height=\"5\" fill=\"currentColor\"/><rect x=\"12\" y=\"14\" width=\"5\" height=\"5\" fill=\"currentColor\"/><rect x=\"18\" y=\"14\" width=\"5\" height=\"5\" fill=\"currentColor\"/><rect x=\"12\" y=\"8\" width=\"5\" height=\"5\" fill=\"currentColor\"/><path d=\"M4 20 H28 Q26 26 16 26 Q6 26 4 20 Z\" fill=\"currentColor\" fill-opacity=\"0.5\"/>",
        "k8s" | "kubernetes" => "<circle cx=\"16\" cy=\"16\" r=\"10\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><circle cx=\"16\" cy=\"16\" r=\"3\" fill=\"currentColor\"/><circle cx=\"16\" cy=\"6\" r=\"2\" fill=\"currentColor\"/><circle cx=\"24.5\" cy=\"11\" r=\"2\" fill=\"currentColor\"/><circle cx=\"24.5\" cy=\"21\" r=\"2\" fill=\"currentColor\"/><circle cx=\"16\" cy=\"26\" r=\"2\" fill=\"currentColor\"/><circle cx=\"7.5\" cy=\"21\" r=\"2\" fill=\"currentColor\"/><circle cx=\"7.5\" cy=\"11\" r=\"2\" fill=\"currentColor\"/>",
        "linux" => "<ellipse cx=\"16\" cy=\"18\" rx=\"9\" ry=\"8\" fill=\"currentColor\"/><circle cx=\"16\" cy=\"10\" r=\"6\" fill=\"currentColor\"/><circle cx=\"13\" cy=\"9\" r=\"1\" fill=\"#0D1117\"/><circle cx=\"19\" cy=\"9\" r=\"1\" fill=\"#0D1117\"/>",
        "git" => "<circle cx=\"10\" cy=\"22\" r=\"3\" fill=\"currentColor\"/><circle cx=\"22\" cy=\"10\" r=\"3\" fill=\"currentColor\"/><circle cx=\"22\" cy=\"22\" r=\"3\" fill=\"currentColor\"/><path d=\"M10 22 L22 10 M10 22 L22 22\" stroke=\"currentColor\" stroke-width=\"2\"/>",
        "github" => "<path fill=\"currentColor\" d=\"M16 4c-6.6 0-12 5.4-12 12 0 5.3 3.4 9.8 8.2 11.4.6.1.8-.3.8-.6v-2.1c-3.3.7-4-1.6-4-1.6-.5-1.4-1.3-1.7-1.3-1.7-1.1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1.1 1.8 2.8 1.3 3.5 1 .1-.8.4-1.3.8-1.6-2.7-.3-5.5-1.3-5.5-5.9 0-1.3.5-2.4 1.2-3.2-.1-.3-.5-1.5.1-3.1 0 0 1-.3 3.3 1.2a11.4 11.4 0 0 1 6 0c2.3-1.5 3.3-1.2 3.3-1.2.6 1.6.2 2.8.1 3.1.8.8 1.2 1.9 1.2 3.2 0 4.6-2.8 5.6-5.5 5.9.4.4.8 1.1.8 2.2v3.2c0 .3.2.7.8.6A12 12 0 0 0 28 16c0-6.6-5.4-12-12-12z\"/>",
        "postgres" | "postgresql" => "<ellipse cx=\"16\" cy=\"10\" rx=\"9\" ry=\"4\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M7 10 V20 C7 23 11 25 16 25 C21 25 25 23 25 20 V10\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/>",
        "redis" => "<path d=\"M6 12 L16 7 L26 12 L16 17 Z\" fill=\"currentColor\"/><path d=\"M6 17 L16 22 L26 17\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M6 21 L16 26 L26 21\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/>",
        "aws" => "<path d=\"M6 18 Q16 26 26 18\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><text x=\"16\" y=\"14\" text-anchor=\"middle\" font-size=\"9\" font-weight=\"700\" font-family=\"sans-serif\" fill=\"currentColor\">aws</text>",
        "gcp" => "<circle cx=\"16\" cy=\"12\" r=\"5\" fill=\"currentColor\"/><circle cx=\"10\" cy=\"20\" r=\"5\" fill=\"currentColor\" fill-opacity=\"0.7\"/><circle cx=\"22\" cy=\"20\" r=\"5\" fill=\"currentColor\" fill-opacity=\"0.5\"/>",
        "azure" => "<path d=\"M8 24 L14 8 L20 16 L24 12 L24 24 Z\" fill=\"currentColor\"/>",
        "next" | "nextjs" => "<circle cx=\"16\" cy=\"16\" r=\"11\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M12 10 V22 M12 10 L22 22\" stroke=\"currentColor\" stroke-width=\"2\"/>",
        "vue" => "<path d=\"M4 8 L16 26 L28 8 H22 L16 18 L10 8 Z\" fill=\"currentColor\"/>",
        "svelte" => "<path d=\"M10 8 C18 4 26 10 22 18 C18 24 8 22 10 14\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.5\"/>",
        "bun" => "<ellipse cx=\"16\" cy=\"17\" rx=\"11\" ry=\"9\" fill=\"currentColor\"/><circle cx=\"12\" cy=\"15\" r=\"1.5\" fill=\"#0D1117\"/><circle cx=\"20\" cy=\"15\" r=\"1.5\" fill=\"#0D1117\"/>",
        "deno" => "<circle cx=\"16\" cy=\"16\" r=\"11\" fill=\"currentColor\"/><circle cx=\"20\" cy=\"13\" r=\"2\" fill=\"#0D1117\"/>",
        "css" | "css3" => "<path d=\"M8 4 H24 L22 26 L16 28 L10 26 Z\" fill=\"currentColor\"/><text x=\"16\" y=\"18\" text-anchor=\"middle\" font-size=\"8\" font-weight=\"700\" fill=\"#0D1117\">CSS</text>",
        "html" | "html5" => "<path d=\"M8 4 H24 L22 26 L16 28 L10 26 Z\" fill=\"currentColor\"/><text x=\"16\" y=\"18\" text-anchor=\"middle\" font-size=\"7\" font-weight=\"700\" fill=\"#0D1117\">HTML</text>",
        "graphql" => "<polygon points=\"16,5 26,11 26,21 16,27 6,21 6,11\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><circle cx=\"16\" cy=\"5\" r=\"2\" fill=\"currentColor\"/><circle cx=\"26\" cy=\"11\" r=\"2\" fill=\"currentColor\"/><circle cx=\"26\" cy=\"21\" r=\"2\" fill=\"currentColor\"/><circle cx=\"16\" cy=\"27\" r=\"2\" fill=\"currentColor\"/><circle cx=\"6\" cy=\"21\" r=\"2\" fill=\"currentColor\"/><circle cx=\"6\" cy=\"11\" r=\"2\" fill=\"currentColor\"/>",
        "tailwind" => "<path d=\"M8 16 C10 10 14 10 16 14 C18 18 22 18 24 12 C22 18 18 18 16 14 C14 10 10 10 8 16 Z\" fill=\"currentColor\"/>",
        "prisma" => "<path d=\"M10 26 L16 4 L24 20 L18 26 Z\" fill=\"currentColor\"/>",
        "sqlite" => "<rect x=\"6\" y=\"6\" width=\"20\" height=\"20\" rx=\"3\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M10 12 H22 M10 16 H20 M10 20 H18\" stroke=\"currentColor\" stroke-width=\"1.5\"/>",
        "nginx" => "<path d=\"M8 24 L16 6 L24 24 Z\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/>",
        "cloudflare" => "<path d=\"M6 18 H22 C26 18 26 12 20 12 C19 8 14 8 12 11 C8 11 6 14 6 18 Z\" fill=\"currentColor\"/>",
        "vercel" => "<path d=\"M16 6 L26 24 H6 Z\" fill=\"currentColor\"/>",
        "sylphx" => "<circle cx=\"16\" cy=\"16\" r=\"10\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><circle cx=\"16\" cy=\"16\" r=\"3\" fill=\"currentColor\"/>",
        _ => return None,
    })
}

pub fn available() -> Vec<&'static str> {
    [
        "rust", "go", "ts", "js", "python", "react", "node", "docker", "kubernetes", "linux",
        "git", "github", "postgres", "redis", "aws", "gcp", "azure", "nextjs", "vue", "svelte",
        "bun", "deno", "css", "html", "graphql", "tailwind", "prisma", "sqlite", "nginx",
        "cloudflare", "vercel", "sylphx",
    ]
    .to_vec()
}

fn normalize_id(raw: &str) -> String {
    match raw.to_ascii_lowercase().as_str() {
        "typescript" => "ts".into(),
        "javascript" => "js".into(),
        "py" => "python".into(),
        "golang" => "go".into(),
        "nodejs" => "node".into(),
        "k8s" => "kubernetes".into(),
        "postgresql" => "postgres".into(),
        "next" => "nextjs".into(),
        other => other.to_string(),
    }
}

pub fn render_row(icons: &str, theme: Option<&str>, per_line: u32) -> String {
    let ids: Vec<String> = icons
        .split(|c| c == ',' || c == '|' || c == ' ')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(normalize_id)
        .collect();

    if ids.is_empty() {
        return svg_doc(
            32,
            32,
            "<text x=\"2\" y=\"20\" font-size=\"10\" fill=\"#888\">no icons</text>",
        );
    }

    let (bg, fg) = if let Some(t) = theme.and_then(themes::get) {
        (format!("#{}", t.bg), format!("#{}", t.fg))
    } else {
        ("#0D1117".into(), "#E6EDF3".into())
    };

    let tile = 48u32;
    let gap = 8u32;
    let per = per_line.max(1);
    let cols = ids.len().min(per as usize) as u32;
    let rows = ((ids.len() as u32) + per - 1) / per;
    let w = cols * tile + (cols.saturating_sub(1)) * gap + 16;
    let h = rows * tile + (rows.saturating_sub(1)) * gap + 16;

    let mut body = format!("<rect width=\"{w}\" height=\"{h}\" rx=\"12\" fill=\"{bg}\"/>");
    let fallback = "<rect x=\"6\" y=\"6\" width=\"20\" height=\"20\" rx=\"4\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><text x=\"16\" y=\"20\" text-anchor=\"middle\" font-size=\"8\" fill=\"currentColor\">?</text>";
    for (i, id) in ids.iter().enumerate() {
        let col = (i as u32) % per;
        let row = (i as u32) / per;
        let x = 8 + col * (tile + gap);
        let y = 8 + row * (tile + gap);
        let g = glyph(id).unwrap_or(fallback);
        body.push_str(&format!(
            "<g transform=\"translate({x},{y})\" color=\"{fg}\">\
             <rect width=\"{tile}\" height=\"{tile}\" rx=\"10\" fill=\"#ffffff\" fill-opacity=\"0.06\"/>\
             <g transform=\"translate(8,8)\">{g}</g>\
             <title>{}</title></g>",
            esc(id)
        ));
    }
    svg_doc(w, h, &body)
}
