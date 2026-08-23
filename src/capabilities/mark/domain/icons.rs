//! Icon catalog and id normalization (pure domain).

pub fn glyph(id: &str) -> Option<&'static str> {
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
        "java" => "<ellipse cx=\"16\" cy=\"20\" rx=\"8\" ry=\"6\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M24 18 Q28 20 24 22\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M12 8 C16 6 20 10 16 14\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\"/>",
        "kotlin" => "<path d=\"M6 6 H16 L6 16 Z M16 6 L26 6 L6 26 L6 16 Z M16 16 L26 26 H16 Z\" fill=\"currentColor\"/>",
        "csharp" | "c#" => "<rect x=\"6\" y=\"6\" width=\"20\" height=\"20\" rx=\"4\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><text x=\"16\" y=\"21\" text-anchor=\"middle\" font-size=\"11\" font-weight=\"700\" font-family=\"sans-serif\" fill=\"currentColor\">C#</text>",
        "php" => "<ellipse cx=\"16\" cy=\"16\" rx=\"12\" ry=\"8\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><text x=\"16\" y=\"20\" text-anchor=\"middle\" font-size=\"8\" font-weight=\"700\" font-family=\"sans-serif\" fill=\"currentColor\">PHP</text>",
        "ruby" => "<path d=\"M16 5 L26 12 L22 26 H10 L6 12 Z\" fill=\"currentColor\"/>",
        "elixir" => "<path d=\"M16 4 C22 10 24 16 16 28 C8 16 10 10 16 4 Z\" fill=\"currentColor\"/>",
        "zig" => "<path d=\"M6 10 H18 L8 22 H26 M10 10 L22 22\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.4\" stroke-linejoin=\"round\"/>",
        "terraform" | "tf" => "<path d=\"M8 6 H14 V14 H8 Z M16 10 H22 V18 H16 Z M8 16 H14 V24 H8 Z\" fill=\"currentColor\"/>",
        "helm" => "<circle cx=\"16\" cy=\"16\" r=\"10\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M16 6 V26 M6 16 H26 M9 9 L23 23 M23 9 L9 23\" stroke=\"currentColor\" stroke-width=\"1.5\"/>",
        "mongodb" | "mongo" => "<path d=\"M16 4 C22 10 22 20 16 28 C10 20 10 10 16 4 Z\" fill=\"currentColor\"/><path d=\"M16 8 V24\" stroke=\"#0D1117\" stroke-width=\"1.5\"/>",
        "mysql" => "<path d=\"M6 20 C8 10 12 8 16 14 C20 20 24 18 26 12\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\"/><ellipse cx=\"10\" cy=\"22\" rx=\"3\" ry=\"2\" fill=\"currentColor\"/>",
        "prometheus" => "<circle cx=\"16\" cy=\"16\" r=\"10\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M16 8 V18 L21 21\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><circle cx=\"16\" cy=\"8\" r=\"1.6\" fill=\"currentColor\"/>",
        "grafana" => "<circle cx=\"16\" cy=\"16\" r=\"10\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M16 16 L24 12 M16 16 L12 24\" stroke=\"currentColor\" stroke-width=\"2\"/><circle cx=\"16\" cy=\"16\" r=\"2.2\" fill=\"currentColor\"/>",
        "flutter" => "<path d=\"M8 16 L16 8 L28 8 L16 20 Z M16 20 L22 26 L16 26 L10 20 Z\" fill=\"currentColor\"/>",
        "swift" => "<path d=\"M6 22 C14 8 24 6 28 10 C18 12 12 20 8 28 C16 22 24 20 26 16\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\"/>",
        "ansible" => "<circle cx=\"16\" cy=\"16\" r=\"11\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M11 22 L16 8 L21 22 M13 17 H19\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/>",
        _ => return None,
    })
}

pub fn available() -> Vec<&'static str> {
    [
        "rust", "go", "ts", "js", "python", "react", "node", "docker", "kubernetes", "linux",
        "git", "github", "postgres", "redis", "aws", "gcp", "azure", "nextjs", "vue", "svelte",
        "bun", "deno", "css", "html", "graphql", "tailwind", "prisma", "sqlite", "nginx",
        "cloudflare", "vercel", "java", "kotlin", "csharp", "php", "ruby", "elixir", "zig",
        "terraform", "helm", "mongodb", "mysql", "prometheus", "grafana", "flutter", "swift",
        "ansible",
    ]
    .to_vec()
}

pub fn normalize_id(raw: &str) -> String {
    match raw.to_ascii_lowercase().as_str() {
        "typescript" => "ts".into(),
        "javascript" => "js".into(),
        "py" => "python".into(),
        "golang" => "go".into(),
        "nodejs" => "node".into(),
        "k8s" => "kubernetes".into(),
        "postgresql" => "postgres".into(),
        "next" => "nextjs".into(),
        "c#" | "c-sharp" | "cs" => "csharp".into(),
        "tf" => "terraform".into(),
        "mongo" => "mongodb".into(),
        other => other.to_string(),
    }
}
