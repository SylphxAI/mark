//! Language colors (GitHub Linguist) and top-language aggregation.

/// Linguist colors for the languages that dominate public profiles, as
/// `Name=rrggbb` pairs.
const COLORS: &str =
    "JavaScript=f1e05a;TypeScript=3178c6;Python=3572a5;Java=b07219;C=555555;C++=f34b7d;\
     C#=178600;Go=00add8;Rust=dea584;Ruby=701516;PHP=4f5d95;Swift=f05138;Kotlin=a97bff;\
     Dart=00b4ab;Scala=c22d40;Shell=89e051;PowerShell=012456;HTML=e34c26;CSS=663399;\
     SCSS=c6538c;Less=1d365d;Vue=41b883;Svelte=ff3e00;Astro=ff5a03;\
     Jupyter Notebook=da5b0b;R=198ce7;Julia=a270ba;MATLAB=e16737;Lua=000080;Perl=0298c3;\
     Haskell=5e5086;Elixir=6e4a7e;Erlang=b83998;Clojure=db5855;OCaml=ef7a08;F#=b845fc;\
     Zig=ec915c;Nim=ffc200;Crystal=000100;Objective-C=438eff;Objective-C++=6866fb;\
     Assembly=6e4c13;Dockerfile=384d54;Makefile=427819;CMake=da3434;Nix=7e7eff;\
     HCL=844fba;Solidity=aa6746;GDScript=355570;Groovy=4298b8;Vim Script=199f4b;\
     Emacs Lisp=c065db;TeX=3d6117;Markdown=083fa1;MDX=fcb32c;Handlebars=f7931e;\
     Pug=a86454;Elm=60b5cc;PureScript=1d222d;Racket=3c5caa;Scheme=1e4aec;\
     Common Lisp=3fb68b;Fortran=4d41b1;COBOL=555555;Visual Basic .NET=945db7;Apex=1797c0;\
     V=4f87c4;Mojo=ff4c1f;WebAssembly=04133b;Jsonnet=0064bd;Starlark=76d275;SQL=e38c00;\
     PLpgSQL=336790;Batchfile=c1f12e;Smarty=f0c040;ShaderLab=222c37;HLSL=aace60;\
     GLSL=5686a5;Cuda=3a4e3a;Verilog=b2b7f8;VHDL=adb2cb";

/// Neutral dot for a language Linguist gives no color (or we do not list).
pub(crate) const UNKNOWN_COLOR: &str = "#8b949e";

/// `#rrggbb` for a language, or [`UNKNOWN_COLOR`].
pub(crate) fn color(name: &str) -> String {
    COLORS
        .split(';')
        .filter_map(|row| row.trim().split_once('='))
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, c)| format!("#{c}"))
        .unwrap_or_else(|| UNKNOWN_COLOR.to_string())
}

/// One language's share of a profile.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LangShare {
    pub name: String,
    pub color: String,
    pub weight: u64,
}

/// Sum `(language, weight)` samples into shares, heaviest first (ties by name
/// so the order is deterministic), dropping `hide`d names (case-insensitive).
pub(crate) fn aggregate<'a>(
    samples: impl Iterator<Item = (&'a str, Option<&'a str>, u64)>,
    hide: &[String],
) -> Vec<LangShare> {
    let mut out: Vec<LangShare> = Vec::new();
    for (name, color_hint, weight) in samples {
        if weight == 0 || hide.iter().any(|h| h.eq_ignore_ascii_case(name)) {
            continue;
        }
        match out.iter_mut().find(|s| s.name == name) {
            Some(s) => s.weight += weight,
            None => out.push(LangShare {
                name: name.to_string(),
                color: color_hint
                    .and_then(crate::capabilities::mark::domain::svg::normalize_hex_token)
                    .unwrap_or_else(|| color(name)),
                weight,
            }),
        }
    }
    out.sort_by(|a, b| b.weight.cmp(&a.weight).then_with(|| a.name.cmp(&b.name)));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_sums_hides_and_orders() {
        let samples = vec![
            ("Rust", None, 30),
            ("TypeScript", None, 50),
            ("Rust", None, 30),
            ("HTML", None, 90),
            ("Go", Some("#00ADD8"), 0),
        ];
        let out = aggregate(samples.into_iter(), &["html".to_string()]);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].name, "Rust");
        assert_eq!(out[0].weight, 60);
        assert_eq!(out[0].color, "#dea584");
        assert_eq!(color("Brainfuck"), UNKNOWN_COLOR);
    }
}
