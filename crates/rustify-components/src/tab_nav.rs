//! Native page links, distinct from tab-panel selection.
pub use crate::tabs::TabsSize as TabNavSize;
use crate::{FlexWrap, Justify, Margin, Responsive};
use leptos::prelude::*;
use rustify_ui::AccentColor;
#[component]
pub fn Root(
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] size: Signal<Responsive<TabNavSize>>,
    #[prop(optional, into)] wrap: Signal<Responsive<FlexWrap>>,
    #[prop(optional, into)] justify: Signal<Responsive<Justify>>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let missing_name = aria_label.trim().is_empty();
    let mode = crate::component_color::use_mode();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (width, size) in size.get().iter() {
            let n = size.number();
            styles.insert(
                width,
                "--rustify-tab-height",
                if n == 1 {
                    "var(--rustify-space-6, 32px)"
                } else {
                    "var(--rustify-space-7, 40px)"
                },
            )?;
            styles.insert(
                width,
                "--rustify-tab-padding",
                if n == 1 {
                    "var(--rustify-space-1, 4px)"
                } else {
                    "var(--rustify-space-2, 8px)"
                },
            )?;
            styles.insert(
                width,
                "font-size",
                if n == 1 {
                    "var(--rustify-font-size-1, 12px)"
                } else {
                    "var(--rustify-font-size-2, 14px)"
                },
            )?;
        }
        for (width, wrap) in wrap.get().iter() {
            styles.insert(width, "flex-wrap", wrap.css())?;
        }
        for (width, justify) in justify.get().iter() {
            styles.insert(width, "justify-content", justify.css())?;
        }
        margin.get().write(&mut styles)?;
        crate::component_color::set_color(&mut styles, color.get(), mode.get())?;
        Ok(styles)
    });
    view! {<nav aria-label=aria_label data-rustify-part="tab-nav-root" data-rustify-issue=missing_name.then_some("missing-name") class=crate::macros::reactive_merge(format!("rustify-color rustify-tabs-list {geometry}"),&class) data-high-contrast=move ||high_contrast.get().then_some("")>{children()}</nav>}
}
#[component]
pub fn Link(
    #[prop(into)] href: String,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional, into)] target: String,
    #[prop(optional, into)] rel: String,
    #[prop(optional, into)] download: Option<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<a data-rustify-native-navigation="" href=href target=(!target.is_empty()).then_some(target) rel=(!rel.is_empty()).then_some(rel) download=download
    aria-current=move ||active.get().then_some("page") data-state=move ||if active.get(){"active"}else{"inactive"}
    class=crate::macros::reactive_merge("rustify-tab-trigger",&class) data-rustify-part="tab-nav-link" data-testid=test_id>{children()}</a>}
}
pub fn link_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::A, At, Ch>,
    active: impl Into<Signal<bool>>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::custom::custom_attribute as data;
    let active = active.into();
    crate::primitives::Slot(
        target,
        (
            crate::primitives::slot::class_token("rustify-tab-trigger".into()),
            data("aria-current", move || active.get().then_some("page")),
            data("data-state", move || {
                if active.get() {
                    "active"
                } else {
                    "inactive"
                }
            }),
            data("data-rustify-part", "tab-nav-link"),
            data("data-rustify-native-navigation", ""),
        ),
    )
}
