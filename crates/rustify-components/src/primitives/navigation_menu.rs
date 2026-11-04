//! Controlled site navigation with inline content and ordinary browser links.
use crate::Orientation;
use leptos::prelude::*;
use rustify_ui::{Cancelable, Direction};
#[cfg(target_arch = "wasm32")]
mod browser;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone)]
struct Entry {
    key: String,
    trigger_id: String,
    trigger: NodeRef<leptos::html::Button>,
    content: NodeRef<leptos::html::Div>,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Context {
    id: StoredValue<String>,
    value: Signal<Option<String>>,
    change: Callback<Option<String>>,
    orientation: Orientation,
    direction: Signal<Direction>,
    entries: RwSignal<Vec<Entry>>,
    viewport: RwSignal<Option<NodeRef<leptos::html::Div>>>,
    indicator: RwSignal<Option<NodeRef<leptos::html::Div>>>,
    delay: u32,
    skip: u32,
    valid: bool,
    #[cfg(target_arch = "wasm32")]
    browser: StoredValue<browser::State, leptos::reactive::owner::LocalStorage>,
}
impl Context {
    fn request(self, value: Option<String>) {
        if self.valid && self.value.get_untracked() != value {
            self.change.run(value);
        }
    }
    fn active(self, key: StoredValue<String>) -> bool {
        self.value.get().as_ref() == Some(&key.get_value())
    }
    fn enter(self, _key: String) {
        #[cfg(target_arch = "wasm32")]
        browser::enter(self, _key);
    }
    fn leave(self) {
        #[cfg(target_arch = "wasm32")]
        browser::leave(self);
    }
    fn cancel(self) {
        #[cfg(target_arch = "wasm32")]
        browser::cancel(self);
    }
    fn toggle(self, key: StoredValue<String>) {
        self.cancel();
        self.request(if self.active(key) {
            None
        } else {
            Some(key.get_value())
        });
    }
}
#[derive(Clone, Copy)]
struct Member {
    context: Context,
    key: StoredValue<String>,
    trigger_id: StoredValue<String>,
    content_id: StoredValue<String>,
    trigger: NodeRef<leptos::html::Button>,
    content: NodeRef<leptos::html::Div>,
}
fn valid_delays(delay: u32, skip: u32) -> bool {
    delay <= i32::MAX as u32 && skip <= i32::MAX as u32
}
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_value_change: Callback<Option<String>>,
    #[prop(default = 200)] delay_ms: u32,
    #[prop(default = 300)] skip_delay_ms: u32,
    #[prop(optional)] orientation: Orientation,
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    root(
        value,
        on_value_change,
        delay_ms,
        skip_delay_ms,
        orientation,
        aria_label,
        class,
        test_id,
        children,
        true,
    )
}
#[component]
pub fn Sub(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_value_change: Callback<Option<String>>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    root(
        value,
        on_value_change,
        0,
        300,
        orientation,
        String::new(),
        class,
        test_id,
        children,
        false,
    )
}
#[allow(clippy::too_many_arguments)]
fn root(
    value: Signal<Option<String>>,
    change: Callback<Option<String>>,
    delay: u32,
    skip: u32,
    orientation: Orientation,
    aria_label: String,
    class: String,
    test_id: String,
    children: Children,
    is_root: bool,
) -> AnyView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let parent = use_context::<Context>();
        let context = Context {
            id: StoredValue::new(crate::id::next("navigation-menu")),
            value,
            change,
            orientation,
            direction: rustify_ui::use_direction(),
            entries: RwSignal::new(Vec::new()),
            viewport: RwSignal::new(None),
            indicator: RwSignal::new(None),
            delay,
            skip,
            valid: valid_delays(delay, skip),
            #[cfg(target_arch = "wasm32")]
            browser: StoredValue::new_local(browser::State::default()),
        };
        provide_context(context);
        #[cfg(target_arch = "wasm32")]
        browser::install(context, is_root);
        let issue = if !context.valid {
            Some("invalid-delay")
        } else if is_root && aria_label.trim().is_empty() {
            Some("missing-name")
        } else if !is_root && parent.is_none() {
            Some("missing-root")
        } else {
            None
        };
        let attrs = (
            leptos::attr::id(context.id.get_value()),
            leptos::tachys::html::class::class(class),
            leptos::attr::custom::custom_attribute("data-testid", test_id),
            leptos::attr::custom::custom_attribute(
                "data-rustify-part",
                if is_root {
                    "navigation-menu-root"
                } else {
                    "navigation-menu-sub"
                },
            ),
            leptos::attr::custom::custom_attribute(
                "data-rustify-navigation",
                context.id.get_value(),
            ),
            leptos::attr::custom::custom_attribute("data-rustify-issue", issue),
            leptos::attr::custom::custom_attribute("data-orientation", orientation.attribute()),
            leptos::attr::custom::custom_attribute("dir", move || {
                context.direction.get().attribute()
            }),
            leptos::attr::custom::custom_attribute("data-viewport", move || {
                context.viewport.get().is_some().then_some("")
            }),
        );
        if is_root {
            leptos::html::nav()
                .attr("aria-label", aria_label)
                .add_any_attr(attrs)
                .child(children())
                .into_any()
        } else {
            leptos::html::div()
                .add_any_attr(attrs)
                .child(children())
                .into_any()
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner).into_any()
}
#[component]
pub fn List(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let context = use_context::<Context>();
    view! {<ul class=class data-testid=test_id data-rustify-part="navigation-menu-list" data-rustify-issue=context.is_none().then_some("missing-root")
    data-orientation=context.map(|context|context.orientation.attribute())
    on:keydown=move |_event|{
        #[cfg(target_arch="wasm32")] if let Some(context)=context {browser::list_key(context,_event);}
    }>{children()}</ul>}
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let Some(context) = use_context::<Context>() else {
        return view!{<li class=class data-testid=test_id data-rustify-part="navigation-menu-item" data-rustify-issue="missing-root">{children()}</li>}.into_any();
    };
    let owner = Owner::new();
    let view=owner.with(move || {
        let member=Member{context,key:StoredValue::new(value),trigger_id:StoredValue::new(crate::id::next("navigation-trigger")),content_id:StoredValue::new(crate::id::next("navigation-content")),trigger:NodeRef::new(),content:NodeRef::new()};
        context.entries.update(|entries|entries.push(Entry{key:member.key.get_value(),trigger_id:member.trigger_id.get_value(),trigger:member.trigger,content:member.content}));
        #[cfg(target_arch="wasm32")]
        on_cleanup(move || {context.entries.try_update(|entries|entries.retain(|entry|entry.trigger_id!=member.trigger_id.get_value()));});
        provide_context(member);
        view!{<li class=class data-testid=test_id data-rustify-part="navigation-menu-item">{children()}</li>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner).into_any()
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let current = use_context::<Context>();
    let Some(member) = use_context::<Member>()
        .filter(|member| current.is_some_and(|context| context.id == member.context.id))
    else {
        return view!{<button type="button" disabled class=class data-testid=test_id data-rustify-part="navigation-menu-trigger" data-rustify-issue="missing-item">{children()}</button>}.into_any();
    };
    let context = member.context;
    view!{<button type="button" node_ref=member.trigger id=member.trigger_id.get_value() class=class data-testid=test_id disabled=move ||disabled.get()||!context.valid
        data-rustify-part="navigation-menu-trigger" data-navigation-owner=context.id.get_value()
        aria-expanded=move ||context.active(member.key).to_string() aria-controls=move ||context.active(member.key).then(||member.content_id.get_value())
        data-state=move ||if context.active(member.key){"open"}else{"closed"}
        on:click=move |event|if !event.default_prevented()&&!disabled.get_untracked(){context.toggle(member.key);}
        on:pointerenter=move |event|if event.pointer_type()=="mouse"&&!event.default_prevented()&&!disabled.get_untracked(){context.enter(member.key.get_value());}
        on:pointerleave=move |event|if event.pointer_type()=="mouse"&&!event.default_prevented(){context.leave();}
        on:keydown=move |_event|{
            #[cfg(target_arch="wasm32")] browser::trigger_key(member,_event);
        }>{children()}</button>}.into_any()
}
#[component]
pub fn Content(
    #[prop(optional)] force_mount: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let current = use_context::<Context>();
    let Some(member) = use_context::<Member>()
        .filter(|member| current.is_some_and(|context| context.id == member.context.id))
    else {
        return view!{<div class=class data-testid=test_id data-rustify-part="navigation-menu-content" data-rustify-issue="missing-item">{children()}</div>}.into_any();
    };
    let context = member.context;
    view!{<Show when=move ||force_mount||context.active(member.key)>
        <div node_ref=member.content id=member.content_id.get_value() class=class.clone() data-testid=test_id.clone() tabindex="-1"
            data-rustify-part="navigation-menu-content" data-navigation-owner=context.id.get_value() aria-labelledby=member.trigger_id.get_value()
            data-state=move ||if context.active(member.key){"open"}else{"closed"} data-orientation=context.orientation.attribute()
            hidden=move ||!context.active(member.key) inert=move ||!context.active(member.key)
            on:pointerenter=move |event|if event.pointer_type()=="mouse"&&!event.default_prevented(){context.cancel();}
            on:pointerleave=move |event|if event.pointer_type()=="mouse"&&!event.default_prevented(){context.leave();}
            on:keydown=move |_event|{
                #[cfg(target_arch="wasm32")] browser::content_key(member,_event);
            }>{children()}</div>
    </Show>}.into_any()
}
pub fn link_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::A, At, Ch>,
    active: Signal<bool>,
    on_select: Option<Callback<Cancelable<()>>>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::custom::custom_attribute as data;
    let context = use_context::<Context>();
    target
        .add_any_attr((
            data("data-rustify-native-navigation", ""),
            data("data-rustify-part", "navigation-menu-link"),
            data(
                "data-navigation-owner",
                context.map(|context| context.id.get_value()),
            ),
            data("data-active", move || active.get().then_some("")),
            data("aria-current", move || active.get().then_some("page")),
        ))
        .on(leptos::ev::click, move |event| {
            if event.default_prevented()
                || event.button() != 0
                || event.ctrl_key()
                || event.alt_key()
                || event.shift_key()
                || event.meta_key()
            {
                return;
            }
            let selected = Cancelable::new(());
            if let Some(on_select) = on_select {
                on_select.run(selected.clone());
            }
            if !selected.default_prevented() {
                if let Some(context) = context {
                    context.cancel();
                    context.request(None);
                }
            }
        })
}
#[component]
pub fn Link(
    #[prop(into)] href: String,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional_no_strip, into)] on_select: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] target: String,
    #[prop(optional, into)] rel: String,
    #[prop(optional_no_strip, into)] download: Option<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    link_as(
        leptos::html::a()
            .href(href)
            .target((!target.is_empty()).then_some(target))
            .rel((!rel.is_empty()).then_some(rel))
            .download(download)
            .class(class)
            .attr("data-testid", test_id)
            .child(children()),
        active,
        on_select,
    )
}
#[component]
pub fn Indicator(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let context = use_context::<Context>();
    let node = NodeRef::new();
    if let Some(context) = context {
        context.indicator.set(Some(node));
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            context.indicator.try_set(None);
        });
    }
    view! {<div node_ref=node class=class data-testid=test_id aria-hidden="true" data-rustify-part="navigation-menu-indicator"
    data-rustify-issue=context.is_none().then_some("missing-root") data-orientation=context.map(|context|context.orientation.attribute())
    data-state=move ||if context.is_some_and(|context|context.value.get().is_some()){"visible"}else{"hidden"} />}
}
#[component]
pub fn Viewport(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let context = use_context::<Context>();
    let node = NodeRef::new();
    if let Some(context) = context {
        context.viewport.set(Some(node));
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            context.viewport.try_set(None);
        });
    }
    view! {<div node_ref=node class=class data-testid=test_id aria-hidden="true" data-rustify-part="navigation-menu-viewport"
    data-rustify-issue=context.is_none().then_some("missing-root") data-orientation=context.map(|context|context.orientation.attribute())
    hidden=move ||context.is_none_or(|context|context.value.get().is_none())
    data-state=move ||if context.is_some_and(|context|context.value.get().is_some()){"open"}else{"closed"} />}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delays_reject_values_the_browser_cannot_represent() {
        assert!(!valid_delays(u32::MAX, 300));
        assert!(!valid_delays(200, u32::MAX));
        assert!(valid_delays(0, 0));
        assert!(valid_delays(i32::MAX as u32, i32::MAX as u32));
    }
    #[test]
    fn requests_leave_the_accepted_value_with_the_application() {
        Owner::new().with(|| {
            let requests = RwSignal::new(Vec::<Option<String>>::new());
            let context = Context {
                id: StoredValue::new("nav".into()),
                value: Signal::stored(Some("learn".into())),
                change: Callback::new(move |value| requests.update(|log| log.push(value))),
                orientation: Orientation::Horizontal,
                direction: Signal::stored(Direction::LeftToRight),
                entries: RwSignal::new(Vec::new()),
                viewport: RwSignal::new(None),
                indicator: RwSignal::new(None),
                delay: 200,
                skip: 300,
                valid: true,
                #[cfg(target_arch = "wasm32")]
                browser: StoredValue::new_local(browser::State::default()),
            };
            context.request(Some("learn".into()));
            context.request(Some("about".into()));
            assert_eq!(requests.get_untracked(), vec![Some("about".into())]);
            assert_eq!(context.value.get_untracked(), Some("learn".into()));
        });
    }
}
