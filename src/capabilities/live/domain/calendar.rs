//! Contributions calendar: parsing GitHub's public calendar HTML and streaks.
//!
//! `https://github.com/users/{user}/contributions` is public HTML with no API
//! quota. Each day is a `<td data-date=… id=…>` and its count lives in the
//! matching `<tool-tip for=…>` label ("5 contributions on …", "No
//! contributions on …"). The heading carries the year's total.

use super::date::{parse_day, Day};
use std::collections::HashMap;

/// One user's daily contributions, ascending by day, without gaps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Calendar {
    pub days: Vec<(Day, u32)>,
    pub total: u64,
}

/// A run of consecutive active days (inclusive range).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Streak {
    pub length: u32,
    pub start: Option<Day>,
    pub end: Option<Day>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StreakSummary {
    pub total: u64,
    pub first: Option<Day>,
    pub last: Option<Day>,
    pub current: Streak,
    pub longest: Streak,
}

/// Value of `name="…"` inside one tag's text.
fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let key = format!(" {name}=\"");
    let start = tag.find(&key)? + key.len();
    let end = tag[start..].find('"')?;
    Some(&tag[start..start + end])
}

/// Leading integer of a label ("1,234 contributions" → 1234, "No …" → 0).
fn leading_count(label: &str) -> Option<u32> {
    let label = label.trim();
    if label.starts_with("No ") {
        return Some(0);
    }
    let digits: String = label
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ',')
        .filter(|c| *c != ',')
        .collect();
    digits.parse().ok()
}

/// Each `<tag …>` opening with its remaining text, in document order.
fn tags<'a>(html: &'a str, name: &str) -> Vec<(&'a str, &'a str)> {
    let open = format!("<{name} ");
    html.match_indices(open.as_str())
        .filter_map(|(i, _)| {
            let rest = &html[i..];
            let end = rest.find('>')?;
            Some((&rest[..end], &rest[end + 1..]))
        })
        .collect()
}

/// Parse the public calendar page. `None` when it holds no calendar days.
pub(crate) fn parse_calendar_html(html: &str) -> Option<Calendar> {
    let mut by_id: HashMap<&str, Day> = HashMap::new();
    let mut counts: HashMap<Day, u32> = HashMap::new();
    for (tag, _) in tags(html, "td") {
        let Some(day) = attr(tag, "data-date").and_then(parse_day) else {
            continue;
        };
        if let Some(n) = attr(tag, "data-count").and_then(|c| c.parse().ok()) {
            counts.insert(day, n);
        }
        counts.entry(day).or_insert(0);
        if let Some(id) = attr(tag, "id") {
            by_id.insert(id, day);
        }
    }
    for (tag, rest) in tags(html, "tool-tip") {
        let (Some(day), Some(end)) = (
            attr(tag, "for").and_then(|f| by_id.get(f)),
            rest.find("</tool-tip>"),
        ) else {
            continue;
        };
        if let Some(n) = leading_count(&rest[..end]) {
            counts.insert(*day, n);
        }
    }
    if counts.is_empty() {
        return None;
    }
    let mut days: Vec<(Day, u32)> = counts.into_iter().collect();
    days.sort_unstable();
    let summed: u64 = days.iter().map(|(_, n)| *n as u64).sum();
    let heading = html
        .find("js-contribution-activity-description")
        .and_then(|i| html[i..].find('>').map(|j| &html[i + j + 1..]))
        .and_then(leading_count)
        .map(u64::from);
    Some(Calendar {
        days,
        total: heading.unwrap_or(summed),
    })
}

/// Streaks the way github-readme-streak-stats counts them: the calendar's last
/// day is "today", and a quiet today does not break the current streak yet.
pub(crate) fn summarize(cal: &Calendar) -> StreakSummary {
    let mut longest = Streak::default();
    let mut run = Streak::default();
    for &(day, n) in &cal.days {
        if n > 0 {
            if run.end.is_some_and(|e| e + 1 == day) {
                run.length += 1;
            } else {
                run = Streak {
                    length: 1,
                    start: Some(day),
                    end: Some(day),
                };
            }
            run.end = Some(day);
            if run.length > longest.length {
                longest = run;
            }
        }
    }
    let last = cal.days.last().map(|(d, _)| *d);
    let current = match (last, run.end) {
        (Some(today), Some(end)) if end == today || end + 1 == today => run,
        _ => Streak::default(),
    };
    StreakSummary {
        total: cal.total,
        first: cal.days.first().map(|(d, _)| *d),
        last,
        current,
        longest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::live::domain::date::days_from_civil;

    fn day_html(date: &str, id: &str, label: &str) -> String {
        format!(
            "<td tabindex=\"0\" data-date=\"{date}\" id=\"{id}\" class=\"ContributionCalendar-day\"></td>\
             <tool-tip id=\"t-{id}\" for=\"{id}\" class=\"sr-only\">{label}</tool-tip>"
        )
    }

    #[test]
    fn parses_days_counts_and_heading() {
        let html = format!(
            "<h2 id=\"js-contribution-activity-description\" class=\"f4\">\n  1,234\n contributions\n</h2>{}{}{}",
            day_html("2026-09-23", "c0", "3 contributions on September 23rd."),
            day_html("2026-09-24", "c1", "No contributions on September 24th."),
            day_html("2026-09-25", "c2", "1,002 contributions on September 25th."),
        );
        let cal = parse_calendar_html(&html).expect("calendar");
        assert_eq!(cal.total, 1234);
        let d0 = days_from_civil(2026, 9, 23);
        assert_eq!(cal.days, vec![(d0, 3), (d0 + 1, 0), (d0 + 2, 1002)]);
        assert!(parse_calendar_html("<html>nothing</html>").is_none());
    }

    #[test]
    fn streaks_follow_streak_stats_rules() {
        let d = days_from_civil(2026, 9, 1);
        let counts = [1, 1, 1, 0, 2, 2, 0];
        let cal = Calendar {
            days: counts
                .iter()
                .enumerate()
                .map(|(i, n)| (d + i as i64, *n))
                .collect(),
            total: 7,
        };
        let s = summarize(&cal);
        assert_eq!(s.longest.length, 3);
        assert_eq!(s.longest.start, Some(d));
        // Today (d+6) is quiet, yesterday was active: the streak still stands.
        assert_eq!(s.current.length, 2);
        assert_eq!(s.current.end, Some(d + 5));
        let broken = Calendar {
            days: vec![(d, 1), (d + 1, 0), (d + 2, 0)],
            total: 1,
        };
        assert_eq!(summarize(&broken).current.length, 0);
    }
}
