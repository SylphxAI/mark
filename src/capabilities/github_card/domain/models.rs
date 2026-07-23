//! GitHub card domain models (product views of upstream snapshots).

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GhUser {
    pub login: String,
    pub name: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub avatar_url: String,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GhRepo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub html_url: String,
    pub open_issues_count: u32,
    pub license: Option<GhLicense>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GhLicense {
    pub spdx_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Aggregate {
    pub stars: u32,
    pub forks: u32,
    pub repos: u32,
    /// name, count, pct
    pub top_langs: Vec<(String, u32, u32)>,
}

/// Pure aggregation of repo snapshots into card metrics.
pub fn aggregate(repos: &[GhRepo]) -> Aggregate {
    let mut stars = 0u32;
    let mut forks = 0u32;
    let mut lang_count: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for r in repos {
        stars += r.stargazers_count;
        forks += r.forks_count;
        if let Some(ref lang) = r.language {
            *lang_count.entry(lang.clone()).or_default() += 1;
        }
    }
    let total: u32 = lang_count.values().sum::<u32>().max(1);
    let mut top: Vec<_> = lang_count.into_iter().collect();
    top.sort_by_key(|b| std::cmp::Reverse(b.1));
    top.truncate(6);
    let top_langs = top
        .into_iter()
        .map(|(name, count)| (name, count, (count * 100) / total))
        .collect();
    Aggregate {
        stars,
        forks,
        repos: repos.len() as u32,
        top_langs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_sums_and_ranks_languages() {
        let repos = vec![
            GhRepo {
                name: "a".into(),
                full_name: "o/a".into(),
                description: None,
                stargazers_count: 10,
                forks_count: 2,
                language: Some("Rust".into()),
                html_url: String::new(),
                open_issues_count: 0,
                license: None,
            },
            GhRepo {
                name: "b".into(),
                full_name: "o/b".into(),
                description: None,
                stargazers_count: 5,
                forks_count: 1,
                language: Some("Rust".into()),
                html_url: String::new(),
                open_issues_count: 0,
                license: None,
            },
            GhRepo {
                name: "c".into(),
                full_name: "o/c".into(),
                description: None,
                stargazers_count: 1,
                forks_count: 0,
                language: Some("Go".into()),
                html_url: String::new(),
                open_issues_count: 0,
                license: None,
            },
        ];
        let agg = aggregate(&repos);
        assert_eq!(agg.stars, 16);
        assert_eq!(agg.forks, 3);
        assert_eq!(agg.repos, 3);
        assert_eq!(agg.top_langs[0].0, "Rust");
        assert_eq!(agg.top_langs[0].1, 2);
    }
}
