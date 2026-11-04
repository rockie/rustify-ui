//! Controlled point-anchored menus with native keyboard and pointer triggers.
pub use super::menu::{
    Arrow, CheckboxItem, Group, Item, ItemIndicator, Label, Portal, RadioGroup, RadioItem,
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
pub fn trigger_as<E, At, Ch>(target: leptos::html::HtmlElement<E, At, Ch>) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    decorate(target, Signal::default())
}
fn decorate<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    disabled: Signal<bool>,
) -> AnyView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::{self, custom::custom_attribute as data};
    let Some(context) = use_context::<menu::Context>() else {
        return super::Slot(
            target,
            (
                data("data-rustify-part", "context-menu-trigger"),
                data("data-rustify-issue", "missing-root"),
            ),
        )
        .into_any();
    };
    let id = floating::register(context.floating.triggers, "context-menu-trigger");
    context.floating.trigger_id.set(id.clone());
    let target = target.add_any_attr((
        attr::id(id),
        data("data-rustify-part", "context-menu-trigger"),
        data("data-rustify-issue", move || {
            context.floating.interaction_issue.get().or_else(|| {
                (context.floating.triggers.get().len() != 1 || context.floating.portals.get() != 1)
                    .then_some("duplicate-or-missing-part")
            })
        }),
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
    ));
    #[cfg(target_arch = "wasm32")]
    {
        menu::browser::context_trigger(target, context, disabled).into_any()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = disabled;
        target.into_any()
    }
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    decorate(
        leptos::html::div()
            .tabindex(0)
            .class(class)
            .attr("data-testid", test_id)
            .child(children()),
        disabled,
    )
}
fn position() -> Signal<rustify_ui::FloatingPosition> {
    Signal::stored(rustify_ui::FloatingPosition {
        side: rustify_ui::Side::Right,
        align: rustify_ui::Align::Start,
        side_offset: 2.,
        ..Default::default()
    })
}
#[component]
pub fn Content(
    #[prop(optional)] loop_focus: bool,
    #[prop(default=position(),into)] position: Signal<rustify_ui::FloatingPosition>,
    #[prop(optional, into)] aria_label: Signal<String>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_focus_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_interact_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_open_auto_focus: Option<
        Callback<rustify_ui::Cancelable<()>>,
    >,
    #[prop(optional_no_strip, into)] on_close_auto_focus: Option<
        Callback<rustify_ui::Cancelable<()>>,
    >,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    menu::content(
        loop_focus,
        position,
        aria_label,
        floating::Hooks {
            escape: on_escape_key_down,
            pointer: on_pointer_down_outside,
            focus: on_focus_outside,
            interact: on_interact_outside,
            open: on_open_auto_focus,
            close: on_close_auto_focus,
        },
        class,
        test_id,
        children,
    )
}
