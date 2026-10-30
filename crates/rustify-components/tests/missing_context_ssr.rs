#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use leptos::prelude::*;
use rustify_components::{form::field, primitives as p};

fn missing(view: impl FnOnce() -> AnyView, part: &str, content: &str, issue: &str) {
    let html = Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "missing-context".into(),
                ..Default::default()
            },
            view,
        )
        .to_html()
    });
    assert!(
        html.contains(&format!("data-rustify-part=\"{part}\"")),
        "{html}"
    );
    assert!(
        html.contains(&format!("data-rustify-issue=\"{issue}\"")),
        "{html}"
    );
    assert!(html.contains(content), "{html}");
    if part.starts_with("select-scroll-") {
        assert!(html.contains(" disabled"), "{html}");
    }
}

#[test]
fn select_viewport_without_root_retains_content() {
    missing(
        || view! { <p::select::Viewport>"Caller viewport"</p::select::Viewport> }.into_any(),
        "select-viewport",
        "Caller viewport",
        "missing-root",
    );
}
#[test]
fn select_content_without_root_retains_content() {
    missing(
        || view! { <p::select::Content>"Caller content"</p::select::Content> }.into_any(),
        "select-content",
        "Caller content",
        "missing-root",
    );
}
#[test]
fn select_value_without_root_retains_placeholder() {
    missing(
        || view! { <p::select::Value placeholder="Caller placeholder" /> }.into_any(),
        "select-value",
        "Caller placeholder",
        "missing-root",
    );
}
#[test]
fn select_item_text_without_root_retains_content() {
    missing(
        || view! { <p::select::ItemText>"Caller text"</p::select::ItemText> }.into_any(),
        "select-item-text",
        "Caller text",
        "missing-root",
    );
}
#[test]
fn select_item_indicator_without_root_retains_content() {
    missing(
        || {
            view! { <p::select::ItemIndicator>"Caller indicator"</p::select::ItemIndicator> }
                .into_any()
        },
        "select-item-indicator",
        "Caller indicator",
        "missing-root",
    );
}
#[test]
fn select_scroll_up_without_root_is_disabled() {
    missing(
        || view! { <p::select::ScrollUpButton>"Caller up"</p::select::ScrollUpButton> }.into_any(),
        "select-scroll-up",
        "Caller up",
        "missing-root",
    );
}
#[test]
fn select_scroll_down_without_root_is_disabled() {
    missing(
        || {
            view! { <p::select::ScrollDownButton>"Caller down"</p::select::ScrollDownButton> }
                .into_any()
        },
        "select-scroll-down",
        "Caller down",
        "missing-root",
    );
}
#[test]
fn select_item_parts_without_item_retain_content() {
    missing(
        || {
            view! {
        <p::select::Root value=None on_change=|_| {} open=false on_open_change=|_| {} entries=vec![]>
            <p::select::ItemText>"Orphan text"</p::select::ItemText>
            <p::select::ItemIndicator>"Orphan indicator"</p::select::ItemIndicator>
        </p::select::Root>
    }.into_any()
        },
        "select-item-text",
        "Orphan text",
        "missing-item",
    );
}
#[test]
fn field_description_without_root_retains_content() {
    missing(
        || view! { <field::Description>"Caller description"</field::Description> }.into_any(),
        "field-description",
        "Caller description",
        "missing-root",
    );
}
#[test]
fn field_message_without_root_emits_diagnostic() {
    missing(
        || view! { <field::Message /> }.into_any(),
        "field-message",
        "",
        "missing-root",
    );
}
