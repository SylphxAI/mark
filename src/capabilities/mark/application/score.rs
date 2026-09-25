//! Score application: a pill whose message carries a graded value.
//!
//! `/api/v1/mark/score?label=agent-ready&value=92&max=100` paints the value
//! with a small progress ring and grades the message color from the fraction
//! (red below 50%, orange below 70%, yellow below 80%, green below 90%,
//! bright green from 90%). An explicit `color` or `theme` wins over grading.

use super::badge::{compose, Badge};
use crate::capabilities::mark::domain::pill::fmt;
use crate::capabilities::mark::domain::MarkSpec;

/// Grade paint for a fraction `0..=1`.
pub(crate) fn grade(frac: f64) -> &'static str {
    match frac * 100.0 {
        p if p < 50.0 => "red",
        p if p < 70.0 => "orange",
        p if p < 80.0 => "yellow",
        p if p < 90.0 => "green",
        _ => "brightgreen",
    }
}

pub fn render(spec: &MarkSpec) -> String {
    let max = spec
        .score
        .max
        .filter(|m| m.is_finite() && *m > 0.0)
        .unwrap_or(100.0);
    let value = spec
        .score
        .value
        .filter(|v| v.is_finite())
        .map(|v| v.clamp(0.0, max));
    let (message, default_color, frac) = match value {
        Some(v) if max == 100.0 => (fmt(v), grade(v / max), v / max),
        Some(v) => (format!("{}/{}", fmt(v), fmt(max)), grade(v / max), v / max),
        None => ("n/a".to_string(), "lightgrey", 0.0),
    };
    let mut spec = spec.clone();
    if spec.pill.label.is_none() {
        spec.pill.label = Some("score".into());
    }
    let mut badge = Badge::from_spec(&spec, &message, default_color);
    badge.ring = Some(frac);
    compose(&badge)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grades_by_fraction() {
        assert_eq!(grade(0.49), "red");
        assert_eq!(grade(0.5), "orange");
        assert_eq!(grade(0.75), "yellow");
        assert_eq!(grade(0.89), "green");
        assert_eq!(grade(0.92), "brightgreen");
    }
}
