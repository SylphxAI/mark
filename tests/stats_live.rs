//! Optional live GitHub check (needs network + rate budget).
#[tokio::test]
#[ignore = "live network; run with --ignored when validating GitHub"]
async fn stats_shtse8_renders() {
    let opts = mark::github_card::CardOpts {
        theme: Some("tokyonight".into()),
        ..Default::default()
    };
    let res = mark::github_card::user_stats(&mark::github_card::HttpGitHubSource, "shtse8", &opts).await;
    match res {
        Ok(svg) => {
            assert!(svg.contains("<svg"), "got: {}", &svg[..svg.len().min(200)]);
            assert!(svg.contains("shtse8") || svg.contains("Repos"));
        }
        Err(e) => panic!("stats failed: {e}"),
    }
}
