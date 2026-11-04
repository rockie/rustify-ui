#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use leptos::prelude::*;
use rustify_components::{primitives::toast as raw, toast as themed};
use rustify_ui::Locale;

fn toast_ssg_render(locale: Locale, view: impl FnOnce() -> AnyView + Send + 'static) -> String {
    Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "feedback-toast".into(),
                ..Default::default()
            },
            || {
                raw::provide_toasts();
                rustify_ui::i18n::provide_locale(locale);
                view()
            },
        )
        .to_html()
    })
}

fn toast_ssg_tag<'a>(html: &'a str, part: &str) -> &'a str {
    html.split('<')
        .find(|tag| tag.contains(&format!("data-rustify-part=\"{part}\"")))
        .unwrap()
}

fn toast_ssg_attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    tag.split_once(&format!(" {name}=\""))
        .map(|(_, value)| value.split('"').next().unwrap())
}

fn toast_ssg_assert_parts_are_associated(html: &str) {
    let root = toast_ssg_tag(html, "toast-root");
    for (attribute, part) in [
        ("aria-labelledby", "toast-title"),
        ("aria-describedby", "toast-description"),
    ] {
        let actual = toast_ssg_attribute(toast_ssg_tag(html, part), "id").unwrap();
        assert_eq!(toast_ssg_attribute(root, attribute), Some(actual), "{html}");
    }
    for part in ["toast-action", "toast-close"] {
        let button = toast_ssg_tag(html, part);
        assert!(button.starts_with("button "), "{html}");
        assert_eq!(
            toast_ssg_attribute(button, "type"),
            Some("button"),
            "{html}"
        );
        assert!(
            !button.contains(" disabled") && !button.contains("missing-root"),
            "{html}"
        );
    }
}

#[test]
fn themed_controlled_parts_keep_one_root_and_real_associations() {
    let html = toast_ssg_render(Locale::English, || {
        view! {
            <themed::Root open=true duration_ms=0 importance=themed::ToastImportance::Foreground tone=themed::ToastTone::Success
                on_open_change=|_| panic!("static rendering cannot request close") class="p-8 bg-muted" test_id="controlled-toast">
                <themed::Title attr:data-testid="toast-title">"Local save"</themed::Title>
                <themed::Description>"The exported file is ready."</themed::Description>
                <themed::Action alt_text="Open the persistent exports panel" on_action=|_|{} attr:title="Review exported files">"Open exports"</themed::Action>
                <themed::Close/>
            </themed::Root>
        }.into_any()
    });
    assert_eq!(
        html.matches("data-rustify-part=\"toast-root\"").count(),
        1,
        "{html}"
    );
    toast_ssg_assert_parts_are_associated(&html);
    let root = toast_ssg_tag(&html, "toast-root");
    assert_eq!(
        toast_ssg_attribute(root, "data-tone"),
        Some("success"),
        "{html}"
    );
    assert_eq!(
        toast_ssg_attribute(root, "data-importance"),
        Some("foreground"),
        "{html}"
    );
    let class = toast_ssg_attribute(root, "class").unwrap();
    for token in ["rustify-toast", "p-8", "bg-muted"] {
        assert!(
            class.split_whitespace().any(|class| class == token),
            "{html}"
        );
    }
    assert_eq!(
        toast_ssg_attribute(toast_ssg_tag(&html, "toast-action"), "title"),
        Some("Review exported files"),
        "{html}"
    );
    assert!(
        !html.contains("aria-live=")
            && !html.contains("role=\"alert\"")
            && !html.contains(" style="),
        "{html}"
    );
}

#[test]
fn mapped_snapshot_constructs_the_body_under_its_only_root_and_keeps_legacy_selectors() {
    let html = toast_ssg_render(Locale::Chinese, || {
        let handle = raw::use_toasts();
        handle.show(
            "已保存到本地",
            themed::ToastOptions {
                duration_ms: 0,
                tone: themed::ToastTone::Warning,
                title: Some("导出完成".into()),
                action: Some(themed::ToastAction {
                    label: "查看导出".into(),
                    alt_text: "打开可持续访问的导出面板".into(),
                    on_action: Callback::new(|_| {}),
                }),
                ..Default::default()
            },
        );
        let toast = handle.current().unwrap();
        view! { <themed::Toast toast=toast class="shadow-none" test_id="mapped-toast"/> }.into_any()
    });
    assert_eq!(
        html.matches("data-rustify-part=\"toast-root\"").count(),
        1,
        "{html}"
    );
    toast_ssg_assert_parts_are_associated(&html);
    assert_eq!(
        html.matches("data-testid=\"mapped-toast\"").count(),
        1,
        "{html}"
    );
    let root = toast_ssg_tag(&html, "toast-root");
    assert_eq!(
        toast_ssg_attribute(root, "data-testid"),
        Some("mapped-toast"),
        "{html}"
    );
    assert!(
        toast_ssg_attribute(root, "class")
            .unwrap()
            .split_whitespace()
            .any(|class| class == "shadow-none"),
        "{html}"
    );
    let close = toast_ssg_tag(&html, "toast-close");
    assert_eq!(
        toast_ssg_attribute(close, "data-testid"),
        Some("mapped-toast-dismiss"),
        "{html}"
    );
    assert_eq!(
        toast_ssg_attribute(close, "aria-label"),
        Some("关闭这条消息"),
        "{html}"
    );
    assert!(
        html.contains("data-glyph=\"close\"") && html.contains("aria-hidden=\"true\""),
        "{html}"
    );
    assert!(
        html.contains("已保存到本地") && html.contains("导出完成") && html.contains("查看导出"),
        "{html}"
    );
}

#[test]
fn primitive_facade_keeps_the_same_parts_without_a_theme_skin() {
    let html = toast_ssg_render(Locale::English, || {
        view! {
            <raw::Root open=true duration_ms=0 on_open_change=|_|{}>
                <raw::Title>"Raw notification"</raw::Title>
                <raw::Description>"The author supplies the skin."</raw::Description>
                <raw::Action alt_text="Open all updates" on_action=|_|{}>"Updates"</raw::Action>
                <raw::Close/>
            </raw::Root>
        }
        .into_any()
    });
    toast_ssg_assert_parts_are_associated(&html);
    let class = toast_ssg_attribute(toast_ssg_tag(&html, "toast-root"), "class").unwrap();
    assert!(
        !class
            .split_whitespace()
            .any(|class| class == "rustify-toast"),
        "{html}"
    );
    assert!(
        html.contains("Raw notification") && html.contains("The author supplies the skin."),
        "{html}"
    );
}

#[test]
fn themed_actions_outside_a_root_are_readable_and_cannot_pretend_to_close_a_notification() {
    let html = toast_ssg_render(Locale::English, || {
        view! {
            <themed::Action alt_text="Open updates" on_action=|_|{}>"Updates"</themed::Action>
            <themed::Close/>
        }
        .into_any()
    });
    for part in ["toast-action", "toast-close"] {
        let button = toast_ssg_tag(&html, part);
        assert!(button.contains(" disabled"), "{html}");
        assert_eq!(
            toast_ssg_attribute(button, "data-rustify-issue"),
            Some("missing-root"),
            "{html}"
        );
    }
    assert!(html.contains("Updates"), "{html}");
}

#[test]
fn independent_unseeded_providers_keep_unique_ids_and_text_associations() {
    let render = || {
        Owner::new().with(|| view! {
        <raw::Provider><raw::Root open=false on_open_change=|_|{}>
            <raw::Title>"Saved"</raw::Title><raw::Description>"Draft saved locally"</raw::Description>
            <raw::Action alt_text="Undo saving" on_action=|_|{}>"Undo"</raw::Action><raw::Close />
        </raw::Root></raw::Provider>
    }.to_html())
    };
    let first = render();
    let second = render();
    assert_ne!(
        toast_ssg_attribute(toast_ssg_tag(&first, "toast-root"), "id"),
        toast_ssg_attribute(toast_ssg_tag(&second, "toast-root"), "id")
    );
    toast_ssg_assert_parts_are_associated(&first);
    toast_ssg_assert_parts_are_associated(&second);
}
