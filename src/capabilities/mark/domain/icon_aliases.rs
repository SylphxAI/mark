//! skill-icons vocabulary (tandpfun/skill-icons) mapped onto Mark's glyphs.
//!
//! `data/skill-icons.tsv` holds skill-icons' full icon-name list (its `all`
//! order) and its short-name table, copied verbatim so a `skillicons.dev` URL
//! resolves here by swapping the host. Each name points at a Simple Icons
//! slug, a hand-drawn fallback glyph (brands Simple Icons removed), or a
//! lettermark in the brand color.

use std::sync::OnceLock;

const DATA: &str = include_str!("../../../../data/skill-icons.tsv");

/// Where a vocabulary id gets its glyph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    /// A Simple Icons slug.
    Brand(&'static str),
    /// A hand-drawn glyph id (see `icons::glyph`) and its brand hex.
    Hand(&'static str, &'static str),
    /// A short lettermark and its brand hex.
    Mono(&'static str, &'static str),
}

pub(crate) struct Vocabulary {
    /// skill-icons names in skill-icons order.
    pub icons: Vec<(&'static str, Target)>,
    /// skill-icons `shortNames` (short → name).
    pub skill_short: Vec<(&'static str, &'static str)>,
    /// Mark's own pre-compatibility spellings (short → name).
    pub mark_short: Vec<(&'static str, &'static str)>,
}

pub(crate) fn vocabulary() -> &'static Vocabulary {
    static VOCABULARY: OnceLock<Vocabulary> = OnceLock::new();
    VOCABULARY.get_or_init(|| {
        let mut v = Vocabulary {
            icons: Vec::new(),
            skill_short: Vec::new(),
            mark_short: Vec::new(),
        };
        for line in DATA.lines().filter(|l| !l.starts_with('#')) {
            let cols: Vec<&'static str> = line.split('\t').collect();
            match cols.as_slice() {
                ["icon", name, target, rest @ ..] => {
                    let hex: &'static str = rest.first().copied().unwrap_or("");
                    let target = if let Some(id) = target.strip_prefix("hand:") {
                        Target::Hand(id, hex)
                    } else if let Some(text) = target.strip_prefix("mono:") {
                        Target::Mono(text, hex)
                    } else {
                        Target::Brand(target)
                    };
                    v.icons.push((*name, target));
                }
                ["short", short, name] => v.skill_short.push((*short, *name)),
                ["mark", short, name] => v.mark_short.push((*short, *name)),
                _ => {}
            }
        }
        v
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vocabulary_is_complete_and_well_formed() {
        let v = vocabulary();
        assert!(v.icons.len() > 230, "{} skill-icons names", v.icons.len());
        assert_eq!(v.skill_short.len(), 40);
        for (name, target) in &v.icons {
            if let Target::Hand(_, hex) | Target::Mono(_, hex) = target {
                assert_eq!(hex.len(), 6, "{name} needs a brand hex");
                assert!(hex.bytes().all(|b| b.is_ascii_hexdigit()), "{name}");
            }
        }
        for (short, name) in v.skill_short.iter().chain(&v.mark_short) {
            assert!(
                v.icons.iter().any(|(n, _)| n == name),
                "short name {short} points at unknown {name}"
            );
        }
    }
}
