//! Studio recovery oracles: a public mark URL reconstitutes composer state.

use mark::{parse_public_mark_url, readme_markdown_embed};

fn form(boot: &mark::StudioBoot) -> &str {
    boot.form.as_deref().unwrap_or("")
}

#[test]
fn empty_studio_locator_is_not_recovery() {
    for raw in [
        "",
        "/",
        "https://mark.sylphx.com/",
        "https://mark.sylphx.com",
        "#",
        "/health",
    ] {
        assert_eq!(
            parse_public_mark_url(raw),
            None,
            "locator {raw:?} is not a mark"
        );
    }
}

#[test]
fn recovers_profile_from_absolute_public_url() {
    let boot = parse_public_mark_url(
        "https://mark.sylphx.com/api/v1/mark/profile?text=Ada%20Lovelace&desc=First%20programmer&theme=tokyonight",
    )
    .expect("profile URL");
    assert_eq!(form(&boot), "profile");
    assert_eq!(boot.text.as_deref(), Some("Ada Lovelace"));
    assert_eq!(boot.desc.as_deref(), Some("First programmer"));
    assert_eq!(boot.theme.as_deref(), Some("tokyonight"));
}

#[test]
fn identity_form_recovers_as_profile() {
    let boot =
        parse_public_mark_url("/api/v1/mark/identity?text=Ada%20Lovelace").expect("identity");
    assert_eq!(form(&boot), "profile");
    assert_eq!(boot.text.as_deref(), Some("Ada Lovelace"));
}

#[test]
fn recovers_studio_query() {
    let boot = parse_public_mark_url("/?form=hero&type=wave&text=Mark&height=120").expect("studio");
    assert_eq!(form(&boot), "hero");
    assert_eq!(boot.art.as_deref(), Some("wave"));
    assert_eq!(boot.text.as_deref(), Some("Mark"));
    assert_eq!(boot.height, Some(120));
}

#[test]
fn studio_identity_query_recovers_as_profile() {
    let boot =
        parse_public_mark_url("/?form=identity&text=Ada%20Lovelace").expect("studio identity");
    assert_eq!(form(&boot), "profile");
    assert_eq!(boot.text.as_deref(), Some("Ada Lovelace"));
}

#[test]
fn recovers_badge_shorthand() {
    let boot = parse_public_mark_url("/badge/build-passing-brightgreen").expect("badge");
    assert_eq!(form(&boot), "pill");
    let pill = boot.pill.as_ref().expect("pill fields");
    assert_eq!(pill.label.as_deref(), Some("build"));
    assert_eq!(pill.message.as_deref(), Some("passing"));
    assert_eq!(boot.color.as_deref(), Some("brightgreen"));
}

#[test]
fn badge_double_dash_keeps_hyphens_in_label() {
    let boot = parse_public_mark_url("/badge/build--passing--brightgreen").expect("badge");
    let pill = boot.pill.as_ref().expect("pill");
    assert_eq!(pill.label.as_deref(), Some("build"));
    assert_eq!(pill.message.as_deref(), Some("passing"));
    assert_eq!(boot.color.as_deref(), Some("brightgreen"));
}

#[test]
fn recovers_hash_public_url() {
    let boot = parse_public_mark_url("#/api/v1/mark/hero?type=wave&text=Hi").expect("hash");
    assert_eq!(form(&boot), "hero");
    assert_eq!(boot.art.as_deref(), Some("wave"));
    assert_eq!(boot.text.as_deref(), Some("Hi"));
}

#[test]
fn recovers_studio_host_with_hash_mark() {
    let boot =
        parse_public_mark_url("https://mark.sylphx.com/#/api/v1/mark/hero?type=wave&text=Hi")
            .expect("host hash");
    assert_eq!(form(&boot), "hero");
    assert_eq!(boot.text.as_deref(), Some("Hi"));
}

#[test]
fn recovers_wrapped_url_param() {
    let boot = parse_public_mark_url(
        "/?url=https%3A%2F%2Fmark.sylphx.com%2Fapi%2Fv1%2Fmark%2Fpill%3Flabel%3Dbuild%26message%3Dpassing",
    )
    .expect("wrapped");
    assert_eq!(form(&boot), "pill");
    let pill = boot.pill.as_ref().expect("pill");
    assert_eq!(pill.label.as_deref(), Some("build"));
    assert_eq!(pill.message.as_deref(), Some("passing"));
}

#[test]
fn recovers_strip_and_deploy() {
    let strip = parse_public_mark_url("/api/v1/mark/strip?icons=rust,ts,docker&perline=6").unwrap();
    assert_eq!(form(&strip), "strip");
    assert_eq!(
        strip.strip.as_ref().and_then(|s| s.icons.as_ref()),
        Some(&vec!["rust".into(), "ts".into(), "docker".into()])
    );
    assert_eq!(strip.strip.as_ref().and_then(|s| s.perline), Some(6));

    let deploy =
        parse_public_mark_url("/api/v1/mark/deploy?service=mark&style=for-the-badge").unwrap();
    assert_eq!(form(&deploy), "deploy");
    assert_eq!(
        deploy.deploy.as_ref().and_then(|d| d.service.as_deref()),
        Some("mark")
    );
    assert_eq!(
        deploy.deploy.as_ref().and_then(|d| d.style.as_deref()),
        Some("for-the-badge")
    );
}

#[test]
fn hero_newlines_from_nl_token() {
    let boot = parse_public_mark_url("/api/v1/mark/hero?text=First-nl-Second").unwrap();
    assert_eq!(boot.text.as_deref(), Some("First\nSecond"));
}

#[test]
fn theme_wins_over_color_in_recovery() {
    let boot = parse_public_mark_url("/badge/build-passing-brightgreen?theme=github")
        .expect("themed badge");
    assert_eq!(boot.theme.as_deref(), Some("github"));
    assert_eq!(boot.color, None);
}

#[test]
fn readme_embed_is_the_markdown_image() {
    assert_eq!(
        readme_markdown_embed(
            "Ada Lovelace",
            "http://test.local/api/v1/mark/profile?text=Ada+Lovelace"
        ),
        "![Ada Lovelace](http://test.local/api/v1/mark/profile?text=Ada+Lovelace)"
    );
    assert_eq!(
        readme_markdown_embed("a]b", "http://x"),
        "![a\\]b](http://x)"
    );
}
