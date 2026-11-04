//! Shared controlled command-menu behavior.
#[cfg(target_arch = "wasm32")]
pub(crate) mod browser;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod model;
use super::{floating, CheckedState};
use leptos::prelude::*;
use rustify_ui::{Cancelable, FloatingPosition, LayerDismiss};

pub use super::floating::{Arrow, Portal};

#[derive(Clone, Copy, Default)]
pub(crate) enum OpenFocus {
    #[default]
    Content,
    First,
    Last,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
pub(super) struct Tree {
    pub root: floating::Context,
    pub subs: RwSignal<u32>,
    pub hooks: StoredValue<floating::Hooks>,
    pub boundary: RwSignal<Option<String>>,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
pub(super) struct Context {
    pub floating: floating::Context,
    pub tree: Tree,
    pub focus: StoredValue<OpenFocus>,
    pub direction: Signal<rustify_ui::Direction>,
    #[cfg(target_arch = "wasm32")]
    pub state: StoredValue<browser::State, leptos::reactive::owner::LocalStorage>,
}
#[derive(Clone, Copy)]
struct MenuContent(StoredValue<String>);
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct SubContext {
    parent: Context,
    child: Context,
}

fn install(floating: floating::Context, tree: Option<Tree>) -> Context {
    let root = tree.is_none();
    let context = Context {
        floating,
        tree: tree.unwrap_or_else(|| Tree {
            root: floating,
            boundary: RwSignal::new(None),
            subs: RwSignal::new(0),
            hooks: StoredValue::new(floating::Hooks::default()),
        }),
        focus: StoredValue::new(OpenFocus::Content),
        direction: rustify_ui::use_direction(),
        #[cfg(target_arch = "wasm32")]
        state: StoredValue::new_local(browser::State::default()),
    };
    provide_context(context);
    #[cfg(target_arch = "wasm32")]
    browser::install(context, root);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = root;
    context
}

pub(crate) fn root(
    open: Signal<bool>,
    change: Callback<bool>,
    modal: bool,
    children: Children,
) -> impl IntoView {
    floating::root(
        open,
        change,
        modal,
        floating::Kind::Menu,
        Box::new(move || {
            if let Some(floating) = use_context::<floating::Context>() {
                install(floating, None);
            }
            children()
        }),
    )
}

pub(crate) fn anchored_root(
    open: Signal<bool>,
    change: Callback<bool>,
    anchor: Signal<rustify_ui::Anchor>,
    children: Children,
) -> impl IntoView {
    root(
        open,
        change,
        false,
        Box::new(move || {
            let context = expect_context::<Context>();
            context.focus.set_value(OpenFocus::First);
            context
                .floating
                .virtual_anchor
                .set(Some(anchor.get_untracked()));
            #[cfg(target_arch = "wasm32")]
            Effect::new(move || context.floating.virtual_anchor.set(Some(anchor.get())));
            children()
        }),
    )
}

#[component]
pub fn Sub(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    children: Children,
) -> impl IntoView {
    let Some(parent) = use_context::<Context>() else {
        return view! { <div data-rustify-part="menu-sub" data-rustify-issue="missing-root">{children()}</div> }.into_any();
    };
    floating::root(
        open,
        on_open_change,
        false,
        floating::Kind::Menu,
        Box::new(move || {
            if let Some(floating) = use_context::<floating::Context>() {
                let child = install(floating, Some(parent.tree));
                provide_context(SubContext { parent, child });
                #[cfg(target_arch = "wasm32")]
                browser::sub_lifecycle(child);
            }
            children()
        }),
    )
    .into_any()
}

#[component]
pub fn Content(
    #[prop(optional)] loop_focus: bool,
    #[prop(optional, into)] position: Signal<FloatingPosition>,
    #[prop(optional, into)] aria_label: Signal<String>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
        Callback<Cancelable<LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_focus_outside: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_interact_outside: Option<
        Callback<Cancelable<LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_open_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional_no_strip, into)] on_close_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    content(
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

pub(crate) fn content(
    loop_focus: bool,
    position: Signal<FloatingPosition>,
    aria_label: Signal<String>,
    mut hooks: floating::Hooks,
    class: String,
    test_id: String,
    children: ChildrenFn,
) -> AnyView {
    let Some(context) = use_context::<Context>() else {
        return view! { <div class=class data-testid=test_id data-rustify-part="menu-content" data-rustify-issue="missing-root">{children()}</div> }.into_any();
    };
    let user_open = hooks.open;
    hooks.open = Some(Callback::new(move |event: Cancelable<()>| {
        if let Some(callback) = user_open {
            callback.run(event.clone());
        }
        #[cfg(target_arch = "wasm32")]
        if !event.default_prevented() {
            browser::focus(context);
            event.prevent_default();
        }
    }));
    if context.floating.panel_id == context.tree.root.panel_id {
        context.tree.hooks.set_value(hooks);
    }
    let children = StoredValue::new(children);
    floating::content(
        floating::ContentConfig {
            align: Signal::default(),
            position: Some(position),
            aria_label: Some(aria_label),
        },
        hooks,
        class,
        test_id,
        std::sync::Arc::new(move || {
            provide_context(MenuContent(context.floating.panel_id));
            #[cfg(target_arch = "wasm32")]
            browser::content(context, loop_focus);
            #[cfg(not(target_arch = "wasm32"))]
            let _ = loop_focus;
            children.with_value(|children| children())
        }),
    )
}

pub(crate) fn sub_position() -> Signal<FloatingPosition> {
    let direction = rustify_ui::use_direction();
    Signal::derive(move || FloatingPosition {
        side: if direction.get() == rustify_ui::Direction::RightToLeft {
            rustify_ui::Side::Left
        } else {
            rustify_ui::Side::Right
        },
        align: rustify_ui::Align::Start,
        ..Default::default()
    })
}
#[component]
pub fn SubContent(
    #[prop(optional)] loop_focus: bool,
    #[prop(default=sub_position(), into)] position: Signal<FloatingPosition>,
    #[prop(optional, into)] aria_label: Signal<String>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
        Callback<Cancelable<LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_focus_outside: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_interact_outside: Option<
        Callback<Cancelable<LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_open_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional_no_strip, into)] on_close_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    if use_context::<SubContext>().is_none() {
        return view! { <div class=class data-testid=test_id data-rustify-part="menu-sub-content" data-rustify-issue="missing-sub">{children()}</div> }.into_any();
    }
    content(
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

fn issue(context: Option<Context>) -> Option<&'static str> {
    match context {
        None => Some("missing-root"),
        Some(context)
            if !use_context::<MenuContent>()
                .is_some_and(|content| content.0 == context.floating.panel_id) =>
        {
            Some("missing-content")
        }
        _ => None,
    }
}
#[derive(Clone, Copy)]
struct GroupContext(RwSignal<Vec<String>>);
#[component]
pub fn Group(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let labels = RwSignal::new(Vec::<String>::new());
        provide_context(GroupContext(labels));
        view! { <div role="group" class=class data-testid=test_id data-rustify-part="menu-group"
            data-rustify-issue=issue(use_context::<Context>()) aria-labelledby=move || (!labels.get().is_empty()).then(||labels.get().join(" "))>{children()}</div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let id = use_context::<GroupContext>().map(|group| floating::register(group.0, "menu-label"));
    view! { <div id=id class=class data-testid=test_id data-rustify-part="menu-label" data-rustify-issue=issue(use_context::<Context>())>{children()}</div> }
}
#[component]
pub fn Separator(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! { <div role="separator" aria-orientation="horizontal" class=class data-testid=test_id data-rustify-part="menu-separator" data-rustify-issue=issue(use_context::<Context>()) /> }
}

#[derive(Clone, Copy)]
struct Indicator(Signal<CheckedState>);
struct ItemConfig {
    role: &'static str,
    checked: Option<Signal<CheckedState>>,
    select: Option<Callback<Cancelable<()>>>,
    change: Callback<()>,
    disabled: Signal<bool>,
    invalid: Signal<Option<&'static str>>,
    text: Option<String>,
    described_by: Signal<String>,
    class: String,
    test_id: String,
}
fn item(config: ItemConfig, children: Children) -> impl IntoView {
    let context = use_context::<Context>();
    let issue = issue(context);
    let ItemConfig {
        role,
        checked,
        select,
        change,
        disabled,
        invalid,
        text,
        described_by,
        class,
        test_id,
    } = config;
    let owner = Owner::new();
    let view = owner.with(move || {
        if let Some(checked) = checked { provide_context(Indicator(checked)); }
        let highlighted = RwSignal::new(false);
        let unavailable = move || disabled.get() || issue.is_some() || invalid.get().is_some();
        view! { <div role=role tabindex="-1" class=class data-testid=test_id
            data-rustify-part=match role { "menuitemcheckbox"=>"menu-checkbox-item", "menuitemradio"=>"menu-radio-item", _=>"menu-item" }
            data-rustify-issue=move || issue.or(invalid.get()) data-menu-item="" data-text-value=text
            aria-disabled=move || unavailable().then_some("true") data-disabled=move || unavailable().then_some("")
            aria-describedby=move || (!described_by.get().is_empty()).then(||described_by.get())
            aria-checked=move || checked.map(|state|state.get().aria())
            data-state=move || checked.map(|state|match state.get() {CheckedState::Checked=>"checked",CheckedState::Unchecked=>"unchecked",CheckedState::Indeterminate=>"indeterminate"})
            data-highlighted=move || highlighted.get().then_some("")
            on:focus=move |_| highlighted.set(true) on:blur=move |_| highlighted.set(false)
            on:click=move |event| {
                if event.default_prevented() {return;}
                model::activate(unavailable(), |event| {if let Some(select)=select {select.run(event);}}, ||change.run(()), || {
                    if let Some(context)=context {context.tree.root.change.run(false);}
                });
            }>{children()}</div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn Item(
    #[prop(into)] on_select: Callback<Cancelable<()>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] text_value: Option<String>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    item(
        ItemConfig {
            role: "menuitem",
            checked: None,
            select: Some(on_select),
            change: Callback::new(|()| {}),
            disabled,
            invalid: Signal::default(),
            text: text_value,
            described_by,
            class,
            test_id,
        },
        children,
    )
}
#[component]
pub fn CheckboxItem(
    #[prop(into)] checked: Signal<CheckedState>,
    #[prop(into)] on_checked_change: Callback<bool>,
    #[prop(optional_no_strip, into)] on_select: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] text_value: Option<String>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    item(
        ItemConfig {
            role: "menuitemcheckbox",
            checked: Some(checked),
            select: on_select,
            change: Callback::new(move |()| {
                on_checked_change.run(checked.get_untracked() != CheckedState::Checked)
            }),
            disabled,
            invalid: Signal::default(),
            text: text_value,
            described_by,
            class,
            test_id,
        },
        children,
    )
}
#[derive(Clone, Copy)]
struct RadioContext {
    value: Signal<Option<String>>,
    change: Callback<String>,
    values: RwSignal<Vec<String>>,
}
#[component]
pub fn RadioGroup(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_value_change: Callback<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        provide_context(RadioContext {
            value,
            change: on_value_change,
            values: RwSignal::new(Vec::new()),
        });
        view! { <Group class=class test_id=test_id>{children()}</Group> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn RadioItem(
    #[prop(into)] value: String,
    #[prop(optional_no_strip, into)] on_select: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] text_value: Option<String>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let group = use_context::<RadioContext>();
    let value = StoredValue::new(value);
    if let Some(group) = group {
        group.values.update(|values| values.push(value.get_value()));
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            group.values.try_update(|values| {
                if let Some(index) = values.iter().position(|v| v == &value.get_value()) {
                    values.remove(index);
                }
            });
        });
    }
    let invalid = Signal::derive(move || match group {
        None => Some("missing-radio-group"),
        Some(group)
            if group
                .values
                .with(|values| values.iter().filter(|v| *v == &value.get_value()).count())
                > 1 =>
        {
            Some("duplicate-value")
        }
        _ => None,
    });
    let checked = Signal::derive(move || {
        if group.is_some_and(|group| group.value.get().as_ref() == Some(&value.get_value())) {
            CheckedState::Checked
        } else {
            CheckedState::Unchecked
        }
    });
    item(
        ItemConfig {
            role: "menuitemradio",
            checked: Some(checked),
            select: on_select,
            change: Callback::new(move |()| {
                if let Some(group) = group {
                    if group.value.get_untracked().as_ref() != Some(&value.get_value()) {
                        group.change.run(value.get_value());
                    }
                }
            }),
            disabled,
            invalid,
            text: text_value,
            described_by,
            class,
            test_id,
        },
        children,
    )
}
#[component]
pub fn ItemIndicator(
    #[prop(optional)] force_mount: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let state = use_context::<Indicator>();
    view! { <Show when=move ||force_mount || state.is_some_and(|state|state.0.get()!=CheckedState::Unchecked)>
        <span class=class.clone() data-testid=test_id.clone() data-rustify-part="menu-item-indicator" aria-hidden="true" data-rustify-issue=state.is_none().then_some("missing-item")>{children()}</span>
    </Show> }
}

#[component]
pub fn SubTrigger(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] text_value: Option<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let sub = use_context::<SubContext>();
    let invalid = sub.map_or(Some("missing-sub"), |sub| issue(Some(sub.parent)));
    let node = NodeRef::<leptos::html::Div>::new();
    let id = sub.map(|sub| {
        let id = floating::register(sub.child.floating.triggers, "menu-sub-trigger");
        sub.child.floating.trigger_id.set(id.clone());
        id
    });
    let highlighted = RwSignal::new(false);
    #[cfg(target_arch = "wasm32")]
    if let Some(sub) = sub.filter(|_| invalid.is_none()) {
        browser::sub_trigger(sub, node, disabled);
    }
    view! { <div node_ref=node id=id role="menuitem" tabindex="-1" class=class data-testid=test_id data-menu-item="" data-menu-sub-trigger=""
    data-rustify-part="menu-sub-trigger" data-rustify-issue=invalid data-text-value=text_value
    aria-haspopup="menu" aria-expanded=move ||sub.is_some_and(|sub|sub.child.floating.open.get()).to_string()
    aria-controls=move ||sub.filter(|sub|sub.child.floating.open.get()).map(|sub|sub.child.floating.panel_id.get_value())
    aria-disabled=move ||(disabled.get() || invalid.is_some()).then_some("true")
    data-disabled=move ||(disabled.get() || invalid.is_some()).then_some("")
    data-state=move ||if sub.is_some_and(|sub|sub.child.floating.open.get()) {"open"}else{"closed"}
    data-highlighted=move ||highlighted.get().then_some("") on:focus=move |_|highlighted.set(true) on:blur=move |_|highlighted.set(false)
    on:click=move |event| {if event.default_prevented() || disabled.get() || invalid.is_some() {return;}
        if let Some(sub)=sub {
        sub.child.focus.set_value(OpenFocus::First);sub.child.floating.change.run(true);
    }}>{children()}</div> }
}
