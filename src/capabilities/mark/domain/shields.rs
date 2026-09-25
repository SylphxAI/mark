//! The shields static-badge dialect: path content and logo tokens.
//!
//! `/badge/{content}` follows shields exactly: `label-message-color` or
//! `message-color`, split on single dashes; `--` is a literal dash, `__` a
//! literal underscore, and a lone `_` (or `%20`) a space. The split runs on the
//! raw (still percent-encoded) path, so `%2D` is a dash that never splits.

/// Split a raw `/badge/` tail into `(label, message, color)`.
///
/// Rendering never fails: a single segment is a message with no color, and
/// more than three segments keep the first as label, the last as color, and
/// rejoin the middle as the message.
pub(crate) fn split_badge_path(tail: &str) -> (String, String, Option<String>) {
    let tail = tail.strip_suffix(".svg").unwrap_or(tail);
    let mut segs = split_single_dashes(tail);
    let decoded = |s: &str| escape_format(&percent_decode(s));
    match segs.len() {
        0 | 1 => (String::new(), decoded(tail), None),
        2 => (String::new(), decoded(&segs[0]), Some(decoded(&segs[1]))),
        _ => {
            let color = segs.pop().map(|c| decoded(&c));
            let label = decoded(&segs.remove(0));
            let message = segs
                .iter()
                .map(|s| decoded(s))
                .collect::<Vec<_>>()
                .join("-");
            (label, message, color)
        }
    }
}

/// Split on `-` that is not part of a `--` pair (pairs stay in the segment).
fn split_single_dashes(s: &str) -> Vec<String> {
    let mut out = vec![String::new()];
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '-' {
            if chars.peek() == Some(&'-') {
                chars.next();
                out.last_mut().expect("never empty").push_str("--");
            } else {
                out.push(String::new());
            }
        } else {
            out.last_mut().expect("never empty").push(c);
        }
    }
    out
}

fn percent_decode(s: &str) -> String {
    urlencoding::decode(s)
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| s.to_string())
}

/// shields `escapeFormat`: a run of `n` underscores becomes `n / 2`
/// underscores plus a space when `n` is odd; then `--` becomes `-`.
pub(crate) fn escape_format(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut run = 0usize;
    let flush = |out: &mut String, run: &mut usize| {
        out.extend(std::iter::repeat_n('_', *run / 2));
        if *run % 2 == 1 {
            out.push(' ');
        }
        *run = 0;
    };
    for c in s.chars() {
        if c == '_' {
            run += 1;
        } else {
            flush(&mut out, &mut run);
            out.push(c);
        }
    }
    flush(&mut out, &mut run);
    out.replace("--", "-")
}

/// Largest accepted logo data URI (shields logos are a few KB).
pub(crate) const MAX_LOGO_DATA_URI: usize = 8192;

/// A logo given as a base64 image data URI, validated to a closed alphabet.
///
/// Only `data:image/svg+xml;base64,` and `data:image/png;base64,` are
/// accepted, and the payload may contain only base64 characters, so the value
/// can never break out of the `href` attribute. An SVG referenced from
/// `<image>` renders in image mode (no script, no external loads). A query
/// string decodes `+` to a space, so spaces in the payload are read back as
/// `+`.
pub(crate) fn data_uri_logo(raw: &str) -> Option<String> {
    if raw.len() > MAX_LOGO_DATA_URI {
        return None;
    }
    let (prefix, payload) = ["data:image/svg+xml;base64,", "data:image/png;base64,"]
        .into_iter()
        .find_map(|p| raw.strip_prefix(p).map(|rest| (p, rest)))?;
    let payload = payload.replace(' ', "+");
    let body = payload.trim_end_matches('=');
    let valid = !body.is_empty()
        && payload.len() - body.len() <= 2
        && body
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/');
    valid.then(|| format!("{prefix}{payload}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn split(t: &str) -> (String, String, Option<String>) {
        split_badge_path(t)
    }

    #[test]
    fn label_message_color_and_message_color() {
        assert_eq!(
            split("build-passing-green"),
            ("build".into(), "passing".into(), Some("green".into()))
        );
        assert_eq!(
            split("just%20the%20message-8A2BE2"),
            ("".into(), "just the message".into(), Some("8A2BE2".into()))
        );
        assert_eq!(
            split("-hello-blue"),
            ("".into(), "hello".into(), Some("blue".into()))
        );
        assert_eq!(split("only"), ("".into(), "only".into(), None));
    }

    #[test]
    fn shields_escapes() {
        assert_eq!(
            split("agent--ready-92%2F100-brightgreen"),
            (
                "agent-ready".into(),
                "92/100".into(),
                Some("brightgreen".into())
            )
        );
        assert_eq!(
            split("any_text-you__like-blue"),
            ("any text".into(), "you_like".into(), Some("blue".into()))
        );
        assert_eq!(
            split("a%2Db-c-red"),
            ("a-b".into(), "c".into(), Some("red".into()))
        );
        assert_eq!(escape_format("a___b"), "a_ b");
        assert_eq!(escape_format("a----b"), "a--b");
        assert_eq!(
            split("build--passing--brightgreen"),
            ("".into(), "build-passing-brightgreen".into(), None)
        );
    }

    #[test]
    fn svg_suffix_and_overflow_segments() {
        assert_eq!(
            split("rust-1.80-orange.svg"),
            ("rust".into(), "1.80".into(), Some("orange".into()))
        );
        assert_eq!(
            split("a-b-c-d"),
            ("a".into(), "b-c".into(), Some("d".into()))
        );
    }

    #[test]
    fn data_uri_logos_are_strict() {
        assert!(data_uri_logo("data:image/svg+xml;base64,PHN2Zy8+").is_some());
        assert_eq!(
            data_uri_logo("data:image/svg+xml;base64,PHN2Zy8 ").as_deref(),
            Some("data:image/svg+xml;base64,PHN2Zy8+")
        );
        assert!(data_uri_logo("data:image/png;base64,iVBORw0KGgo=").is_some());
        for bad in [
            "rust",
            "data:image/svg+xml,<svg/>",
            "data:text/html;base64,PHN2Zy8+",
            "data:image/svg+xml;base64,\"/><script>",
            "data:image/svg+xml;base64,",
            "data:image/svg+xml;base64,QQ===",
        ] {
            assert_eq!(data_uri_logo(bad), None, "{bad}");
        }
        let long = format!("data:image/png;base64,{}", "A".repeat(MAX_LOGO_DATA_URI));
        assert_eq!(data_uri_logo(&long), None);
    }
}
