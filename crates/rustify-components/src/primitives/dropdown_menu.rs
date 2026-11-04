//! Controlled menus opened by a native button.
pub use super::menu::{
    Arrow, CheckboxItem, Content, Group, Item, ItemIndicator, Label, Portal, RadioGroup, RadioItem,
    Separator, Sub, SubContent, SubTrigger,
};
use super::{floating, menu};
use leptos::prelude::*;

#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(default = true)] modal: bool,
    children: Children,
) -> impl IntoView {
    menu::root(open, on_open_change, modal, children)
}
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    decorate(target, Signal::default())
}
fn decorate<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    disabled: Signal<bool>,
) -> AnyView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::{self, custom::custom_attribute as data};
    let Some(context) = use_context::<menu::Context>() else {
        return super::Slot(
            target,
            (
                attr::disabled(true),
                data("data-rustify-part", "dropdown-menu-trigger"),
                data("data-rustify-issue", "missing-root"),
            ),
        )
        .into_any();
    };
    let id = floating::register(context.floating.triggers, "menu-trigger");
    context.floating.trigger_id.set(id.clone());
    let unavailable = move || {
        disabled.get()
            || context.floating.triggers.get().len() != 1
            || context.floating.portals.get() != 1
    };
    let target = target
        .on(leptos::ev::pointerdown, move |event| {
            if event.default_prevented() || unavailable() || event.button() != 0 || event.ctrl_key()
            {
                return;
            }
            context.focus.set_value(menu::OpenFocus::Content);
            let open = !context.floating.open.get_untracked();
            context.floating.change.run(open);
            if open {
                event.prevent_default();
            }
        })
        .on(leptos::ev::keydown, move |event| {
            if event.default_prevented() || event.is_composing() || unavailable() {
                return;
            }
            let (open, focus) = match event.key().as_str() {
                "Enter" | " " => (
                    !context.floating.open.get_untracked(),
                    menu::OpenFocus::First,
                ),
                "ArrowDown" => (true, menu::OpenFocus::First),
                "ArrowUp" => (true, menu::OpenFocus::Last),
                _ => return,
            };
            event.prevent_default();
            context.focus.set_value(focus);
            context.floating.change.run(open);
        });
    super::Slot(
        target,
        (
            attr::r#type("button"),
            attr::id(id),
            data("data-rustify-part", "dropdown-menu-trigger"),
            data("data-rustify-issue", move || {
                (context.floating.triggers.get().len() != 1 || context.floating.portals.get() != 1)
                    .then_some("duplicate-or-missing-part")
            }),
            data("aria-disabled", move || unavailable().then_some("true")),
            data("aria-haspopup", "menu"),
            data("aria-expanded", move || {
                context.floating.open.get().to_string()
            }),
            data("aria-controls", move || {
                context
                    .floating
                    .open
                    .get()
                    .then(|| context.floating.panel_id.get_value())
            }),
            data("data-state", move || {
                if context.floating.open.get() {
                    "open"
                } else {
                    "closed"
                }
            }),
        ),
    )
    .into_any()
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    decorate(
        leptos::html::button()
            .disabled(move || disabled.get())
            .class(class)
            .attr("data-testid", test_id)
            .child(children()),
        disabled,
    )
}
