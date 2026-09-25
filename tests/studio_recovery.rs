//! Studio recovery oracles: a public mark URL reconstitutes composer state.

use mark::capabilities::mark::domain::recovery::parse_public_mark_url;

fn form(boot: &mark::capabilities::mark::domain::recovery::StudioBoot) -> &str {
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
fn badge_double_dash_is_a_literal_dash() {
    // shields escaping: `--` is a literal dash, never a separator.
    let boot = parse_public_mark_url("/badge/agent--ready-92%2F100-brightgreen").expect("badge");
    let pill = boot.pill.as_ref().expect("pill");
    assert_eq!(pill.label.as_deref(), Some("agent-ready"));
    assert_eq!(pill.message.as_deref(), Some("92/100"));
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
fn recovers_pill_label_color_when_no_theme_pack() {
    let boot = parse_public_mark_url("/badge/build-passing-brightgreen?labelColor=red")
        .expect("labelColor badge");
    assert_eq!(form(&boot), "pill");
    assert_eq!(boot.color.as_deref(), Some("brightgreen"));
    assert_eq!(
        boot.pill.as_ref().and_then(|p| p.label_color.as_deref()),
        Some("red")
    );
}

#[test]
fn theme_pack_wins_over_label_color_in_recovery() {
    let boot = parse_public_mark_url(
        "/api/v1/mark/pill?label=build&message=passing&theme=github&labelColor=red",
    )
    .expect("themed pill");
    assert_eq!(boot.theme.as_deref(), Some("github"));
    assert_eq!(
        boot.pill.as_ref().and_then(|p| p.label_color.as_ref()),
        None
    );
}

#[test]
fn unknown_theme_is_not_a_theme_pack_in_recovery() {
    let boot =
        parse_public_mark_url("/badge/build-passing-brightgreen?theme=not-a-theme&labelColor=red")
            .expect("unknown theme");
    assert_eq!(boot.theme, None);
    assert_eq!(boot.color.as_deref(), Some("brightgreen"));
    assert_eq!(
        boot.pill.as_ref().and_then(|p| p.label_color.as_deref()),
        Some("red")
    );
}

#[test]
fn studio_page_ships_the_markdown_embed_control() {
    // The README embed is built in the studio page (the browser owns that
    // string; there is no server-side embed writer). This asserts the shipped
    // page still offers the control and the `![alt](url)` shape.
    let html = std::fs::read_to_string("static/index.html").expect("studio page");
    assert!(
        html.contains("Copy markdown"),
        "studio offers the embed control"
    );
    assert!(
        html.contains("buildMarkdown"),
        "studio builds the markdown embed"
    );
    assert!(
        html.contains("![") && html.contains("]("),
        "embed is ![alt](url)"
    );
}

#[test]
fn recovers_score_badge() {
    let boot = parse_public_mark_url("/api/v1/mark/score.svg?label=agent-ready&value=92&max=100")
        .expect("score");
    assert_eq!(form(&boot), "score");
    assert_eq!(
        boot.pill.as_ref().and_then(|p| p.label.as_deref()),
        Some("agent-ready")
    );
    let score = boot.score.as_ref().expect("score fields");
    assert_eq!(score.value.as_deref(), Some("92"));
    assert_eq!(score.max.as_deref(), Some("100"));
}
