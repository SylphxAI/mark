//! Dynamic badge faces: shields' default label, message, and color per badge.

use super::date::{age_color, parse_timestamp, relative_age};
use super::format::metric;

/// What a dynamic badge says before the user's query restyles it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Face {
    pub label: String,
    pub message: String,
    pub color: String,
}

fn face(label: &str, message: String, color: &str) -> Face {
    Face {
        label: label.into(),
        message,
        color: color.into(),
    }
}

pub(crate) fn stars(n: u64) -> Face {
    face("stars", metric(n), "blue")
}

pub(crate) fn forks(n: u64) -> Face {
    face("forks", metric(n), "blue")
}

/// Shields' license colors: permissive green, copyleft orange, public domain
/// light green, anything else grey.
pub(crate) fn license(spdx: Option<&str>) -> Face {
    let Some(id) = spdx.filter(|s| !s.is_empty() && *s != "NOASSERTION") else {
        return face("license", "not specified".into(), "lightgrey");
    };
    let upper = id.to_ascii_uppercase();
    let color = if ["CC0-1.0", "UNLICENSE", "WTFPL", "0BSD"].contains(&upper.as_str()) {
        "7cd958"
    } else if ["GPL", "AGPL", "LGPL", "MPL", "EPL", "EUPL", "OSL", "CECILL"]
        .iter()
        .any(|p| upper.starts_with(p))
    {
        "orange"
    } else if [
        "MIT", "APACHE", "BSD", "ISC", "ZLIB", "BSL", "PSF", "PYTHON", "X11", "ARTISTIC",
    ]
    .iter()
    .any(|p| upper.starts_with(p))
    {
        "green"
    } else {
        "lightgrey"
    };
    face("license", id.to_string(), color)
}

/// `v`-prefixed version, shields-style: a bare number gains `v`.
fn versioned(v: &str) -> String {
    let v = v.trim();
    if v.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("v{v}")
    } else {
        v.to_string()
    }
}

/// Pre-releases and 0.x read orange, stable releases blue.
fn version_color(v: &str) -> &'static str {
    let bare = v.trim_start_matches(['v', 'V']);
    if bare.contains('-') || bare.starts_with("0.") {
        "orange"
    } else {
        "blue"
    }
}

pub(crate) fn release(tag: Option<&str>, prerelease: bool) -> Face {
    match tag {
        Some(t) => face(
            "release",
            versioned(t),
            if prerelease {
                "orange"
            } else {
                version_color(t)
            },
        ),
        None => face("release", "no releases found".into(), "lightgrey"),
    }
}

pub(crate) fn last_commit(committed_at: Option<&str>, now_unix: i64) -> Face {
    match committed_at.and_then(parse_timestamp) {
        Some(t) => {
            let age = now_unix - t;
            face("last commit", relative_age(age), age_color(age))
        }
        None => face("last commit", "no commits".into(), "lightgrey"),
    }
}

pub(crate) fn npm_version(v: &str) -> Face {
    face("npm", versioned(v), version_color(v))
}

/// Downloads per `period` (`month`, `week`, or total when `None`).
pub(crate) fn downloads(n: u64, period: Option<&str>) -> Face {
    let message = match period {
        Some(p) => format!("{}/{p}", metric(n)),
        None => metric(n),
    };
    face(
        "downloads",
        message,
        if n > 0 { "brightgreen" } else { "red" },
    )
}

/// A badge whose subject does not exist upstream.
pub(crate) fn not_found(label: &str, what: &str) -> Face {
    face(label, format!("{what} not found"), "lightgrey")
}

/// Upstream failed and nothing is cached: still a calm, readable badge.
pub(crate) fn unavailable(label: &str) -> Face {
    face(label, "unavailable".into(), "lightgrey")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn faces_follow_shields_defaults() {
        assert_eq!(stars(12_345).message, "12k");
        assert_eq!(license(Some("MIT")).color, "green");
        assert_eq!(license(Some("GPL-3.0")).color, "orange");
        assert_eq!(license(None).message, "not specified");
        assert_eq!(release(Some("1.2.0"), false).message, "v1.2.0");
        assert_eq!(release(Some("v2.0.0-rc.1"), false).color, "orange");
        assert_eq!(npm_version("0.3.1").color, "orange");
        assert_eq!(downloads(1_234_567, Some("month")).message, "1.2M/month");
        let lc = last_commit(Some("2026-09-20T00:00:00Z"), 1_790_294_400);
        assert!(lc.message.ends_with("ago"));
    }
}
