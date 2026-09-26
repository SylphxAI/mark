//! Civil calendar arithmetic for live data (no date dependency).
//!
//! Days are counted from 1970-01-01 (Howard Hinnant's `days_from_civil`), so a
//! streak is a run of consecutive integers and a relative age is a subtraction.

/// A proleptic Gregorian day number (days since 1970-01-01).
pub(crate) type Day = i64;

pub(crate) const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub(crate) fn days_from_civil(y: i64, m: u32, d: u32) -> Day {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let m = m as i64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

pub(crate) fn civil_from_days(z: Day) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// `YYYY-MM-DD` (optionally followed by `T…`) → day number.
pub(crate) fn parse_day(s: &str) -> Option<Day> {
    let s = s.get(..10)?;
    let mut parts = s.split('-');
    let y: i64 = parts.next()?.parse().ok()?;
    let m: u32 = parts.next()?.parse().ok()?;
    let d: u32 = parts.next()?.parse().ok()?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some(days_from_civil(y, m, d))
}

/// `YYYY-MM-DDTHH:MM:SSZ` → unix seconds (UTC; offsets other than `Z` are
/// read as UTC, which is what GitHub and npm emit).
pub(crate) fn parse_timestamp(s: &str) -> Option<i64> {
    let day = parse_day(s)?;
    let clock = s.get(11..19).unwrap_or("00:00:00");
    let mut parts = clock.split(':').map(|p| p.parse::<i64>().ok());
    let (h, m, sec) = (
        parts.next().flatten()?,
        parts.next().flatten()?,
        parts.next().flatten()?,
    );
    Some(day * 86_400 + h * 3600 + m * 60 + sec)
}

/// `Sep 21, 2025`.
pub(crate) fn format_long(day: Day) -> String {
    let (y, m, d) = civil_from_days(day);
    format!("{} {d}, {y}", MONTHS[(m - 1) as usize])
}

/// `Sep 21` (the year is implied by the card's range).
pub(crate) fn format_short(day: Day) -> String {
    let (_, m, d) = civil_from_days(day);
    format!("{} {d}", MONTHS[(m - 1) as usize])
}

/// Shields' relative age (`dayjs().to()`): "3 days ago", "a month ago".
pub(crate) fn relative_age(seconds: i64) -> String {
    let s = seconds.max(0) as f64;
    let (minutes, hours, days) = (s / 60.0, s / 3600.0, s / 86_400.0);
    let phrase = if s < 45.0 {
        "a few seconds".to_string()
    } else if s < 90.0 {
        "a minute".to_string()
    } else if minutes < 45.0 {
        format!("{} minutes", minutes.round())
    } else if minutes < 90.0 {
        "an hour".to_string()
    } else if hours < 22.0 {
        format!("{} hours", hours.round())
    } else if hours < 36.0 {
        "a day".to_string()
    } else if days < 26.0 {
        format!("{} days", days.round())
    } else if days < 46.0 {
        "a month".to_string()
    } else if days < 320.0 {
        format!("{} months", (days / 30.4).round())
    } else if days < 548.0 {
        "a year".to_string()
    } else {
        format!("{} years", (days / 365.0).round())
    };
    format!("{phrase} ago")
}

/// Shields' age color scale: fresh is green, abandoned is red.
pub(crate) fn age_color(seconds: i64) -> &'static str {
    match seconds.max(0) / 86_400 {
        0..=6 => "brightgreen",
        7..=29 => "green",
        30..=179 => "yellowgreen",
        180..=364 => "yellow",
        365..=729 => "orange",
        _ => "red",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_round_trip() {
        for (y, m, d) in [(1970, 1, 1), (2000, 2, 29), (2025, 12, 31), (2026, 3, 1)] {
            assert_eq!(civil_from_days(days_from_civil(y, m, d)), (y, m, d));
        }
        assert_eq!(days_from_civil(1970, 1, 2), 1);
        assert_eq!(parse_day("2026-09-25"), Some(days_from_civil(2026, 9, 25)));
        assert_eq!(parse_timestamp("1970-01-02T00:00:10Z"), Some(86_410));
        assert_eq!(format_long(days_from_civil(2025, 9, 21)), "Sep 21, 2025");
    }

    #[test]
    fn relative_age_matches_shields_phrasing() {
        assert_eq!(relative_age(10), "a few seconds ago");
        assert_eq!(relative_age(3 * 86_400), "3 days ago");
        assert_eq!(relative_age(40 * 86_400), "a month ago");
        assert_eq!(relative_age(800 * 86_400), "2 years ago");
        assert_eq!(age_color(86_400), "brightgreen");
        assert_eq!(age_color(1000 * 86_400), "red");
    }
}
