//! The stats-card rank: github-readme-stats' percentile formula.
//!
//! Each signal passes through a CDF around a median and is weighted; the
//! weighted mean becomes a percentile (lower is better) and a letter level.

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RankInput {
    /// True when `commits` counts all time (the median is then higher).
    pub all_commits: bool,
    pub commits: u64,
    pub prs: u64,
    pub issues: u64,
    pub reviews: u64,
    pub stars: u64,
    pub followers: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Rank {
    pub level: &'static str,
    /// Top-percentile (0 = best, 100 = everyone).
    pub percentile: f64,
}

fn exponential_cdf(x: f64) -> f64 {
    1.0 - 2f64.powf(-x)
}

fn log_normal_cdf(x: f64) -> f64 {
    x / (1.0 + x)
}

const THRESHOLDS: [f64; 9] = [1.0, 12.5, 25.0, 37.5, 50.0, 62.5, 75.0, 87.5, 100.0];
const LEVELS: [&str; 9] = ["S", "A+", "A", "A-", "B+", "B", "B-", "C+", "C"];

pub(crate) fn calculate(input: &RankInput) -> Rank {
    let commits_median = if input.all_commits { 1000.0 } else { 250.0 };
    let weighted = [
        (2.0, exponential_cdf(input.commits as f64 / commits_median)),
        (3.0, exponential_cdf(input.prs as f64 / 50.0)),
        (1.0, exponential_cdf(input.issues as f64 / 25.0)),
        (1.0, exponential_cdf(input.reviews as f64 / 2.0)),
        (4.0, log_normal_cdf(input.stars as f64 / 50.0)),
        (1.0, log_normal_cdf(input.followers as f64 / 10.0)),
    ];
    let total: f64 = weighted.iter().map(|(w, _)| w).sum();
    let score: f64 = weighted.iter().map(|(w, v)| w * v).sum();
    let percentile = (1.0 - score / total) * 100.0;
    let level = THRESHOLDS
        .iter()
        .position(|t| percentile <= *t)
        .map(|i| LEVELS[i])
        .unwrap_or("C");
    Rank { level, percentile }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_profile_is_c_and_prolific_profile_is_s() {
        assert_eq!(calculate(&RankInput::default()).level, "C");
        let star = calculate(&RankInput {
            commits: 5000,
            prs: 2000,
            issues: 900,
            reviews: 400,
            stars: 400_000,
            followers: 100_000,
            ..Default::default()
        });
        assert_eq!(star.level, "S");
        assert!(star.percentile < 1.0);
    }
}
