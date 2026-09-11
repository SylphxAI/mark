//! Theme packs — fleet brands + popular dev themes.

#[derive(Clone, Debug)]
pub(crate) struct Theme {
    pub bg: &'static str,
    pub bg2: &'static str,
    pub fg: &'static str,
    pub accent: &'static str,
}

pub(crate) fn get(name: &str) -> Option<&'static Theme> {
    THEMES
        .iter()
        .find(|t| t.0.eq_ignore_ascii_case(name))
        .map(|t| &t.1)
}

pub(crate) fn list_names() -> Vec<&'static str> {
    THEMES.iter().map(|t| t.0).collect()
}

static THEMES: &[(&str, Theme)] = &[
    (
        "dark",
        Theme {
            bg: "0D1117",
            bg2: "161B22",
            fg: "E6EDF3",
            accent: "58A6FF",
        },
    ),
    (
        "light",
        Theme {
            bg: "FFFFFF",
            bg2: "F6F8FA",
            fg: "1F2328",
            accent: "0969DA",
        },
    ),
    (
        "radical",
        Theme {
            bg: "141321",
            bg2: "FE428E",
            fg: "A9FEF7",
            accent: "FE428E",
        },
    ),
    (
        "gruvbox",
        Theme {
            bg: "282828",
            bg2: "FABD2F",
            fg: "EBDBB2",
            accent: "FE8019",
        },
    ),
    (
        "tokyonight",
        Theme {
            bg: "1A1B27",
            bg2: "7AA2F7",
            fg: "A9B1D6",
            accent: "BB9AF7",
        },
    ),
    (
        "dracula",
        Theme {
            bg: "282A36",
            bg2: "BD93F9",
            fg: "F8F8F2",
            accent: "FF79C6",
        },
    ),
    (
        "nord",
        Theme {
            bg: "2E3440",
            bg2: "88C0D0",
            fg: "ECEFF4",
            accent: "81A1C1",
        },
    ),
    (
        "monokai",
        Theme {
            bg: "272822",
            bg2: "F92672",
            fg: "F8F8F2",
            accent: "A6E22E",
        },
    ),
    (
        "ocean",
        Theme {
            bg: "0B1D36",
            bg2: "00B4D8",
            fg: "CAF0F8",
            accent: "0077B6",
        },
    ),
    (
        "sunset",
        Theme {
            bg: "2B0A0A",
            bg2: "FF6B35",
            fg: "FFF3E0",
            accent: "FF9F1C",
        },
    ),
    (
        "forest",
        Theme {
            bg: "0B1F14",
            bg2: "2D6A4F",
            fg: "D8F3DC",
            accent: "52B788",
        },
    ),
    (
        "neon",
        Theme {
            bg: "0A0A12",
            bg2: "00F5D4",
            fg: "F0F0FF",
            accent: "F15BB5",
        },
    ),
    (
        "github",
        Theme {
            bg: "0D1117",
            bg2: "238636",
            fg: "C9D1D9",
            accent: "1F6FEB",
        },
    ),
];

pub(crate) const PALETTE: &[&str] = &[
    "667EEA", "764BA2", "F093FB", "F5576C", "4FACFE", "00F2FE", "43E97B", "38F9D7", "FA709A",
    "FEE140", "A18CD1", "FBC2EB", "D87000", "4A90E2", "E03840", "7C3AED", "C9A227", "00F5D4",
    "FF6B35", "2D6A4F",
];

pub(crate) const GRADIENTS: &[(&str, &str)] = &[
    // High-chroma signature pairs (capsule-class liquid fields)
    ("667EEA", "F093FB"),
    ("F093FB", "F5576C"),
    ("4FACFE", "00F2FE"),
    ("43E97B", "38F9D7"),
    ("FA709A", "FEE140"),
    ("FF6B35", "F15BB5"),
    ("7C3AED", "00F5D4"),
    ("E03840", "FF6B35"),
    ("00F5D4", "F15BB5"),
    ("FF6B35", "FEE140"),
    ("4338CA", "F093FB"),
    ("0EA5E9", "A78BFA"),
    ("F43F5E", "FB923C"),
    ("14B8A6", "6366F1"),
    ("D87000", "4A90E2"),
    ("1A1B27", "7AA2F7"),
    ("282A36", "FF79C6"),
    ("2D6A4F", "95D5B2"),
];

pub(crate) fn hash_seed(s: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in s.as_bytes() {
        h ^= u32::from(*b);
        h = h.wrapping_mul(16777619);
    }
    h
}

pub(crate) fn pick_auto(seed: &str) -> &'static str {
    PALETTE[(hash_seed(seed) as usize) % PALETTE.len()]
}

pub(crate) fn pick_gradient(seed: &str) -> (&'static str, &'static str) {
    GRADIENTS[(hash_seed(seed) as usize) % GRADIENTS.len()]
}
