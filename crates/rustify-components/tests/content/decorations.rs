#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use leptos::prelude::*;
use rustify_components::{
    badge, callout, card, GeometryStyles, Layout, Margin, Responsive, Spacing,
};
use rustify_ui::AccentColor;

fn decoration_render(view: impl FnOnce() -> AnyView + Send + 'static) -> (String, GeometryStyles) {
    let styles = GeometryStyles::default();
    let html = Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "decorations".into(),
                ..Default::default()
            },
            || {
                rustify_components::provide_geometry_styles(styles.clone());
                view()
            },
        )
        .to_html()
    });
    (html, styles)
}

#[test]
fn invalid_decoration_geometry_is_diagnosed_without_serializing_the_input() {
    let (html, styles) = decoration_render(|| {
        view! {
            <badge::Badge margin=Margin { m: Spacing::Css("1px; color:red".into()).into(), ..Default::default() }>"Readable badge"</badge::Badge>
            <callout::Root margin=Margin { mx: Spacing::Css("url(https://example.test/a)".into()).into(), ..Default::default() }>
                <callout::Text>"Readable explanation"</callout::Text>
            </callout::Root>
            <card::Card layout=Layout { width: "-2px".to_string().into(), ..Default::default() }>"Readable card"</card::Card>
        }.into_any()
    });
    let fields: Vec<_> = styles
        .diagnostics()
        .into_iter()
        .map(|issue| issue.field)
        .collect();
    assert_eq!(fields.len(), 3, "{fields:?}");
    for field in ["m", "mx", "width"] {
        assert!(fields.contains(&field), "{fields:?}");
    }
    assert!(
        html.contains("Readable badge")
            && html.contains("Readable explanation")
            && html.contains("Readable card"),
        "{html}"
    );
    assert!(
        !styles.css().contains("color:red") && !styles.css().contains("example.test"),
        "{}",
        styles.css()
    );
    assert!(!html.contains(" style="), "{html}");
}

#[test]
fn defaults_are_static_native_content_without_interaction_or_live_announcements() {
    let (html, styles) = decoration_render(|| {
        view! {
            <badge::Badge attr:title="Record state">"Draft"</badge::Badge>
            <card::Card attr:aria-label="Entry"><button type="button">"Edit entry"</button></card::Card>
            <callout::Root>
                <callout::Icon><svg aria-label="Information" viewBox="0 0 16 16"></svg></callout::Icon>
                <callout::Text>"Changes remain local. "<strong>"Keep a copy."</strong></callout::Text>
            </callout::Root>
        }.into_any()
    });
    assert!(
        html.contains("<span")
            && html.contains("title=\"Record state\"")
            && html.contains(">Draft</span>"),
        "{html}"
    );
    assert!(
        html.contains("data-rustify-part=\"card\"") && html.contains("aria-label=\"Entry\""),
        "{html}"
    );
    assert_eq!(html.matches("<button").count(), 1, "{html}");
    for part in ["badge", "card"] {
        let tag = html
            .split('<')
            .find(|tag| tag.contains(&format!("data-rustify-part=\"{part}\"")))
            .unwrap();
        assert!(
            !tag.contains("tabindex=") && !tag.contains("role=\"button\""),
            "{tag}"
        );
    }
    assert!(
        !html.contains("role=\"status\"")
            && !html.contains("role=\"alert\"")
            && !html.contains("aria-live="),
        "{html}"
    );
    assert!(
        html.contains("aria-hidden=\"true\"") && html.contains("<p"),
        "{html}"
    );
    assert!(
        html.contains("Changes remain local. ") && html.contains("<strong>Keep a copy.</strong>"),
        "{html}"
    );
    assert!(
        styles.diagnostics().is_empty(),
        "{:?}",
        styles.diagnostics()
    );
}

#[test]
fn badge_and_card_decorate_one_native_target_and_preserve_authored_attributes() {
    let (html, styles) = decoration_render(|| {
        let badge = badge::badge_as(
            leptos::html::a()
                .attr("href", "/records?state=draft")
                .attr("rel", "bookmark")
                .class("badge-host px-8")
                .child("Draft records"),
            badge::BadgeOptions {
                variant: Signal::stored(badge::BadgeVariant::Outline),
                ..Default::default()
            },
        );
        let card = card::card_as(
            leptos::html::article().attr("aria-labelledby", "record-title").class("card-host p-8").child(view! { <h2 id="record-title">"Record"</h2><a href="/record">"Open record"</a> }),
            card::CardOptions { variant: Signal::stored(card::CardVariant::Classic), ..Default::default() },
        );
        (badge, card).into_any()
    });
    assert!(
        html.contains("href=\"/records?state=draft\"") && html.contains("rel=\"bookmark\""),
        "{html}"
    );
    assert!(
        html.contains("badge-host px-8") && html.contains("rustify-badge"),
        "{html}"
    );
    assert!(
        html.contains("<article") && html.contains("aria-labelledby=\"record-title\""),
        "{html}"
    );
    assert!(
        html.contains("card-host p-8") && html.contains("rustify-card"),
        "{html}"
    );
    assert_eq!(html.matches("<a ").count(), 2, "{html}");
    let targets =
        &html[html.find("<a ").unwrap()..html.rfind("</article>").unwrap() + "</article>".len()];
    assert!(
        !targets.contains("<div") && !targets.contains("<span"),
        "{targets}"
    );
    assert!(
        styles.diagnostics().is_empty(),
        "{:?}",
        styles.diagnostics()
    );
}

#[test]
fn callouts_announce_only_when_the_author_selects_status_or_alert() {
    let (html, _) = decoration_render(|| {
        view! {
            <callout::Root role=Some(callout::CalloutRole::Status)><callout::Text>"Saved locally"</callout::Text></callout::Root>
            <callout::Root role=Some(callout::CalloutRole::Alert)><callout::Text>"Export failed"</callout::Text></callout::Root>
            <callout::Icon decorative=false attr:aria-label="Note">"Note"</callout::Icon>
        }.into_any()
    });
    assert_eq!(html.matches("role=\"status\"").count(), 1, "{html}");
    assert_eq!(html.matches("role=\"alert\"").count(), 1, "{html}");
    assert!(!html.contains("aria-hidden="), "{html}");
    assert!(
        html.contains("Saved locally") && html.contains("Export failed"),
        "{html}"
    );
}

#[test]
fn responsive_sizes_and_layout_are_external_and_deterministic() {
    fn fixture() -> (String, GeometryStyles) {
        decoration_render(|| {
            view! {
                <badge::Badge size=Responsive { initial: Some(badge::BadgeSize::One), sm: Some(badge::BadgeSize::Three), ..Default::default() }
                    color=Some(AccentColor::Red) high_contrast=true radius=Some(rustify_ui::ThemeRadius::Full)>
                    "Review"
                </badge::Badge>
                <callout::Root size=Responsive { initial: Some(callout::CalloutSize::One), md: Some(callout::CalloutSize::Three), ..Default::default() }>
                    <callout::Text>"Retained explanation"</callout::Text>
                </callout::Root>
                <card::Card size=Responsive { initial: Some(card::CardSize::One), lg: Some(card::CardSize::Five), ..Default::default() }
                    layout=Layout { width: "min(36rem, 100%)".to_string().into(), ..Default::default() }
                    margin=Margin { ml: Spacing::Scale(-2).into(), ..Default::default() }>"Card content"</card::Card>
            }.into_any()
        })
    }
    let (html, styles) = fixture();
    let (next_html, next_styles) = fixture();
    assert_eq!(html, next_html);
    assert_eq!(styles.css(), next_styles.css());
    assert!(
        !html.contains(" style=") && !html.contains("<style") && !html.contains("<script"),
        "{html}"
    );
    let css = styles.css();
    for expected in [
        "@layer components",
        ":where(",
        "@media (min-width:768px)",
        "@media (min-width:1024px)",
        "@media (min-width:1280px)",
        "width:min(36rem, 100%)",
        "--rustify-component-color-12:",
        "--rustify-badge-radius-full:9999px",
        "--rustify-card-padding:var(--rustify-space-8, 48px)",
        "margin-left:calc(var(--rustify-space-2) * -1)",
    ] {
        assert!(css.contains(expected), "missing {expected}: {css}");
    }
    assert!(
        styles.diagnostics().is_empty(),
        "{:?}",
        styles.diagnostics()
    );
}

#[test]
fn ghost_card_offsets_authored_margins_while_retaining_auto_edges() {
    let (_, styles) = decoration_render(|| {
        view! { <card::Card variant=card::CardVariant::Ghost margin=Margin { m: Spacing::Scale(2).into(), mx: Spacing::Css("auto".into()).into(), ..Default::default() }>"Ghost"</card::Card> }.into_any()
    });
    let css = styles.css();
    assert!(
        css.contains("margin-top:calc(var(--rustify-space-2) - var(--rustify-card-padding))"),
        "{css}"
    );
    assert!(
        css.contains("margin-left:auto") && css.contains("margin-right:auto"),
        "{css}"
    );
    assert!(
        styles.diagnostics().is_empty(),
        "{:?}",
        styles.diagnostics()
    );
}
