//! One application-controlled expandable region.
use leptos::prelude::*;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Context {
    open: Signal<bool>,
    disabled: Signal<bool>,
    change: Callback<bool>,
    key: StoredValue<String>,
    trigger: StoredValue<String>,
    content: StoredValue<String>,
    triggers: RwSignal<u32>,
    contents: RwSignal<u32>,
}
#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view=owner.with(move || {
        let context=Context {open, disabled, change:on_open_change, key:StoredValue::new(crate::id::next("collapsible")), trigger:StoredValue::new(crate::id::next("collapsible-trigger")),content:StoredValue::new(crate::id::next("collapsible-content")),triggers:RwSignal::new(0),contents:RwSignal::new(0)};
        provide_context(context);
        let children=children();
        view! { <div class=crate::macros::reactive_merge("", &class) data-rustify-part="collapsible-root" data-testid=test_id
            data-state=move || if open.get() {"open"}else{"closed"} data-disabled=move || disabled.get().then_some("")
            data-rustify-issue={move || if context.triggers.get()==0 {Some("missing-trigger")}else if context.triggers.get()>1 || context.contents.get()>1 {Some("duplicate-part")}else{None}}
            on:click=move |_event| {
                #[cfg(target_arch="wasm32")] {
                    use leptos::wasm_bindgen::JsCast;
                    let owns=_event.target().and_then(|target|target.dyn_into::<web_sys::Element>().ok())
                        .and_then(|el|el.closest("[data-rustify-collapsible-owner]").ok().flatten())
                        .is_some_and(|el|el.get_attribute("data-rustify-collapsible-owner").as_deref()==Some(&context.key.get_value()));
                    if owns && !_event.default_prevented() && !disabled.get_untracked() && context.triggers.get_untracked()==1 { context.change.run(!open.get_untracked()); }
                }
            }>{children}</div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
fn trigger_inner<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    class: String,
    test_id: String,
) -> AnyView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::custom::custom_attribute as data;
    let Some(context) = use_context::<Context>() else {
        return super::Slot(
            target,
            (
                leptos::attr::disabled(true),
                data("data-rustify-part", "collapsible-trigger"),
                data("data-rustify-issue", "missing-root"),
            ),
        )
        .into_any();
    };
    context.triggers.update(|count| *count += 1);
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        context.triggers.try_update(|count| *count -= 1);
    });
    let id = if context.triggers.get_untracked() == 1 {
        context.trigger.get_value()
    } else {
        crate::id::next("collapsible-trigger")
    };
    super::Slot(
        target,
        (
            (
                leptos::attr::id(id),
                leptos::attr::r#type("button"),
                leptos::attr::disabled(move || {
                    context.disabled.get() || context.triggers.get() != 1
                }),
                data("data-rustify-part", "collapsible-trigger"),
                data("data-rustify-collapsible-owner", context.key.get_value()),
                data("aria-expanded", move || context.open.get().to_string()),
                data("aria-controls", move || {
                    context.open.get().then(|| context.content.get_value())
                }),
                data("data-rustify-issue", move || {
                    (context.triggers.get() > 1).then_some("duplicate-part")
                }),
                data("data-state", move || {
                    if context.open.get() {
                        "open"
                    } else {
                        "closed"
                    }
                }),
                data("data-disabled", move || {
                    context.disabled.get().then_some("")
                }),
            ),
            super::tabs::optional_attribute((!class.is_empty()).then(|| {
                leptos::tachys::html::class::class(crate::macros::reactive_merge("", &class))
            })),
            super::tabs::optional_attribute(
                (!test_id.is_empty()).then(|| data("data-testid", test_id)),
            ),
        ),
    )
    .into_any()
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    trigger_inner(leptos::html::button().child(children()), class, test_id)
}
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    trigger_inner(target, String::new(), String::new())
}
#[component]
pub fn Content(
    #[prop(optional)] force_mount: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let Some(context) = use_context::<Context>() else {
        return view!{<div class=class data-rustify-part="collapsible-content" data-rustify-issue="missing-root">{children()}</div>}.into_any();
    };
    context.contents.update(|count| *count += 1);
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        context.contents.try_update(|count| *count -= 1);
    });
    let id = StoredValue::new(if context.contents.get_untracked() == 1 {
        context.content.get_value()
    } else {
        crate::id::next("collapsible-content")
    });
    let class = crate::macros::reactive_merge("", &class);
    let test_id = StoredValue::new(test_id);
    view!{<Show when=move ||force_mount ||context.open.get() fallback=||()>
        <div id=id.get_value() class=class data-testid=test_id.get_value() data-rustify-part="collapsible-content"
            role="region" aria-labelledby=context.trigger.get_value() hidden=move ||!context.open.get() inert=move ||!context.open.get()
            data-state=move ||if context.open.get(){"open"}else{"closed"}
            data-rustify-issue=move ||(context.contents.get()>1).then_some("duplicate-part")>{children()}</div>
    </Show>}.into_any()
}
