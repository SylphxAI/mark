//! Number formatting for badges and cards.

const PREFIXES: [&str; 6] = ["k", "M", "G", "T", "P", "E"];

/// Shields' `metric()`: `999`, `1.2k`, `12k`, `3.4M` (one decimal below ten,
/// a trailing `.0` dropped, rounding up into the next prefix at 1000).
pub(crate) fn metric(n: u64) -> String {
    for i in (0..PREFIXES.len()).rev() {
        let limit = 1000f64.powi(i as i32 + 1);
        let v = n as f64;
        if v >= limit {
            let scaled = v / limit;
            if scaled < 10.0 {
                let one = format!("{scaled:.1}");
                if !one.ends_with('0') {
                    return format!("{one}{}", PREFIXES[i]);
                }
            }
            let rounded = scaled.round() as u64;
            return if rounded < 1000 {
                format!("{rounded}{}", PREFIXES[i])
            } else {
                format!("1{}", PREFIXES.get(i + 1).copied().unwrap_or("E"))
            };
        }
    }
    n.to_string()
}

/// Card numbers: exact with separators while they stay short (`12,345`),
/// shields-style metric beyond that (`123k`, `4.5M`).
pub(crate) fn card_number(n: u64) -> String {
    if n >= 100_000 {
        return metric(n);
    }
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_matches_shields() {
        assert_eq!(metric(0), "0");
        assert_eq!(metric(999), "999");
        assert_eq!(metric(1000), "1k");
        assert_eq!(metric(1234), "1.2k");
        assert_eq!(metric(12_345), "12k");
        assert_eq!(metric(999_999), "1M");
        assert_eq!(metric(3_456_789), "3.5M");
        assert_eq!(metric(1_563_531_164), "1.6G");
    }

    #[test]
    fn card_numbers_group_then_compact() {
        assert_eq!(card_number(7), "7");
        assert_eq!(card_number(1234), "1,234");
        assert_eq!(card_number(99_999), "99,999");
        assert_eq!(card_number(123_456), "123k");
    }
}
