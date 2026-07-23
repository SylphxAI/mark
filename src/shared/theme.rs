//! Theme packs — fleet brands + popular dev themes.

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: &'static str,
    pub bg: &'static str,
    pub bg2: &'static str,
    pub fg: &'static str,
    pub muted: &'static str,
    pub accent: &'static str,
}

pub fn get(name: &str) -> Option<&'static Theme> {
    THEMES.iter().find(|t| t.0.eq_ignore_ascii_case(name)).map(|t| &t.1)
}

pub fn list() -> Vec<&'static Theme> {
    THEMES.iter().map(|t| &t.1).collect()
}

pub fn list_names() -> Vec<&'static str> {
    THEMES.iter().map(|t| t.0).collect()
}

static THEMES: &[(&str, Theme)] = &[
    (
        "sylphx",
        Theme {
            name: "Sylphx",
            bg: "1A1A2E",
            bg2: "4A90E2",
            fg: "FFFFFF",
            muted: "A8B3C7",
            accent: "D87000",
        },
    ),
    (
        "cubeage",
        Theme {
            name: "Cubeage",
            bg: "1A1A1A",
            bg2: "E03840",
            fg: "FFFFFF",
            muted: "D0D0D0",
            accent: "E03840",
        },
    ),
    (
        "epiow",
        Theme {
            name: "Epiow",
            bg: "0B1020",
            bg2: "7C3AED",
            fg: "FFFFFF",
            muted: "C4B5FD",
            accent: "4338CA",
        },
    ),
    (
        "ozyrix",
        Theme {
            name: "Ozyrix",
            bg: "0A0A0A",
            bg2: "C9A227",
            fg: "F5F5F0",
            muted: "A3A3A3",
            accent: "C9A227",
        },
    ),
    (
        "kyle",
        Theme {
            name: "Kyle",
            bg: "0F172A",
            bg2: "38BDF8",
            fg: "F8FAFC",
            muted: "94A3B8",
            accent: "22D3EE",
        },
    ),
    (
        "dark",
        Theme {
            name: "Dark",
            bg: "0D1117",
            bg2: "161B22",
            fg: "E6EDF3",
            muted: "8B949E",
            accent: "58A6FF",
        },
    ),
    (
        "light",
        Theme {
            name: "Light",
            bg: "FFFFFF",
            bg2: "F6F8FA",
            fg: "1F2328",
            muted: "656D76",
            accent: "0969DA",
        },
    ),
    (
        "radical",
        Theme {
            name: "Radical",
            bg: "141321",
            bg2: "FE428E",
            fg: "A9FEF7",
            muted: "F8D847",
            accent: "FE428E",
        },
    ),
    (
        "gruvbox",
        Theme {
            name: "Gruvbox",
            bg: "282828",
            bg2: "FABD2F",
            fg: "EBDBB2",
            muted: "A89984",
            accent: "FE8019",
        },
    ),
    (
        "tokyonight",
        Theme {
            name: "Tokyo Night",
            bg: "1A1B27",
            bg2: "7AA2F7",
            fg: "A9B1D6",
            muted: "565F89",
            accent: "BB9AF7",
        },
    ),
    (
        "dracula",
        Theme {
            name: "Dracula",
            bg: "282A36",
            bg2: "BD93F9",
            fg: "F8F8F2",
            muted: "6272A4",
            accent: "FF79C6",
        },
    ),
    (
        "nord",
        Theme {
            name: "Nord",
            bg: "2E3440",
            bg2: "88C0D0",
            fg: "ECEFF4",
            muted: "D8DEE9",
            accent: "81A1C1",
        },
    ),
    (
        "monokai",
        Theme {
            name: "Monokai",
            bg: "272822",
            bg2: "F92672",
            fg: "F8F8F2",
            muted: "75715E",
            accent: "A6E22E",
        },
    ),
    (
        "ocean",
        Theme {
            name: "Ocean",
            bg: "0B1D36",
            bg2: "00B4D8",
            fg: "CAF0F8",
            muted: "90E0EF",
            accent: "0077B6",
        },
    ),
    (
        "sunset",
        Theme {
            name: "Sunset",
            bg: "2B0A0A",
            bg2: "FF6B35",
            fg: "FFF3E0",
            muted: "FFAB91",
            accent: "FF9F1C",
        },
    ),
    (
        "forest",
        Theme {
            name: "Forest",
            bg: "0B1F14",
            bg2: "2D6A4F",
            fg: "D8F3DC",
            muted: "95D5B2",
            accent: "52B788",
        },
    ),
    (
        "neon",
        Theme {
            name: "Neon",
            bg: "0A0A12",
            bg2: "00F5D4",
            fg: "F0F0FF",
            muted: "9B5DE5",
            accent: "F15BB5",
        },
    ),
    (
        "github",
        Theme {
            name: "GitHub",
            bg: "0D1117",
            bg2: "238636",
            fg: "C9D1D9",
            muted: "8B949E",
            accent: "1F6FEB",
        },
    ),
];

pub const PALETTE: &[&str] = &[
    "667EEA", "764BA2", "F093FB", "F5576C", "4FACFE", "00F2FE", "43E97B", "38F9D7", "FA709A",
    "FEE140", "A18CD1", "FBC2EB", "D87000", "4A90E2", "E03840", "7C3AED", "C9A227", "00F5D4",
    "FF6B35", "2D6A4F",
];

pub const GRADIENTS: &[(&str, &str)] = &[
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

pub fn hash_seed(s: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in s.as_bytes() {
        h ^= u32::from(*b);
        h = h.wrapping_mul(16777619);
    }
    h
}

pub fn pick_auto(seed: &str) -> &'static str {
    PALETTE[(hash_seed(seed) as usize) % PALETTE.len()]
}

pub fn pick_gradient(seed: &str) -> (&'static str, &'static str) {
    GRADIENTS[(hash_seed(seed) as usize) % GRADIENTS.len()]
}

pub fn time_seed() -> String {
    let n = chrono::Utc::now();
    format!(
        "{}-{}-{}-{}",
        n.format("%Y"),
        n.format("%m"),
        n.format("%d"),
        n.format("%H")
    )
}
