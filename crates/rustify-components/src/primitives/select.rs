//! A controlled select-only combobox assembled from independent native parts.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod model;
use leptos::prelude::*;

/// Stable value, typeahead text and native option state, independent of Item layout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectEntry {
    pub value: String,
    pub text_value: String,
    pub disabled: bool,
}
impl SelectEntry {
    pub fn new(value: impl Into<String>, text_value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            text_value: text_value.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[cfg(target_arch = "wasm32")]
mod browser;
use model::Index;
use rustify_ui::{Cancelable, LayerDismiss, LayerPlacement};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
struct ItemBinding {
    value: StoredValue<String>,
    id: StoredValue<String>,
    node: NodeRef<leptos::html::Div>,
}
#[derive(Clone, Default, PartialEq)]
struct Items {
    by_value: HashMap<String, ItemBinding>,
    by_id: HashMap<String, ItemBinding>,
    duplicates: HashSet<String>,
}
impl PartialEq for ItemBinding {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Items {
    fn one(&self, value: &str) -> Option<ItemBinding> {
        (!self.duplicates.contains(value))
            .then(|| self.by_value.get(value).copied())
            .flatten()
    }
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone)]
enum Initial {
    Edge(bool),
    Match(String),
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Context {
    key: StoredValue<String>,
    requested_id: StoredValue<String>,
    trigger_id: StoredValue<String>,
    list_id: StoredValue<String>,
    form: StoredValue<String>,
    placeholder: StoredValue<String>,
    value: Signal<Option<String>>,
    open: Signal<bool>,
    change: Callback<Option<String>>,
    change_open: Callback<bool>,
    select: Option<Callback<Cancelable<String>>>,
    escape: Option<Callback<Cancelable<LayerDismiss>>>,
    outside: Option<Callback<Cancelable<LayerDismiss>>>,
    focus_open: Option<Callback<Cancelable<()>>>,
    focus_close: Option<Callback<Cancelable<()>>>,
    index: Memo<Index>,
    items: RwSignal<Vec<ItemBinding>>,
    registry: Memo<Items>,
    active: RwSignal<Option<String>>,
    initial: StoredValue<Option<Initial>>,
    search: StoredValue<String>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    invalid: Signal<bool>,
    described_by: Signal<String>,
    labelled_by: StoredValue<String>,
    aria_label: StoredValue<String>,
    trigger: NodeRef<leptos::html::Button>,
    content: NodeRef<leptos::html::Div>,
    viewport: NodeRef<leptos::html::Div>,
    mirror: NodeRef<leptos::html::Select>,
    root: NodeRef<leptos::html::Div>,
    triggers: RwSignal<u32>,
    present: RwSignal<bool>,
    placement: RwSignal<LayerPlacement>,
    width: RwSignal<f64>,
    scroll_up: RwSignal<bool>,
    scroll_down: RwSignal<bool>,
    direction: Signal<rustify_ui::Direction>,
    anchor_failed: StoredValue<bool>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<browser::SearchTimer>, leptos::reactive::owner::LocalStorage>,
}
impl Context {
    fn accepted(self) -> Option<String> {
        self.value.with(|value| {
            value
                .as_ref()
                .filter(|value| self.index.with(|index| index.entry(value).is_some()))
                .cloned()
        })
    }
    fn disabled(self) -> bool {
        self.disabled.get() || self.triggers.get() != 1
    }
    fn selected(self, value: &str) -> bool {
        self.accepted().as_deref() == Some(value)
    }
    #[cfg(target_arch = "wasm32")]
    fn usable(self, value: &str) -> bool {
        self.index.with(|index| index.available(value))
            && (!self.present.get() || self.registry.with(|registry| registry.one(value).is_some()))
    }
    fn issue(self) -> Option<&'static str> {
        if self.triggers.get() == 0 {
            return Some("missing-trigger");
        }
        if self.triggers.get() > 1 {
            return Some("multiple-triggers");
        }
        self.index.with(|index| {
            if index.invalid.contains("") {
                Some("empty-entry")
            } else if !index.invalid.is_empty() {
                Some("duplicate-entry")
            } else if !self
                .registry
                .with(|registry| registry.duplicates.is_empty())
            {
                Some("duplicate-item")
            } else if self.present.get()
                && index.enabled().any(|entry| {
                    !self
                        .registry
                        .with(|registry| registry.one(&entry.value).is_some())
                })
            {
                Some("missing-item")
            } else {
                None
            }
        })
    }
    fn item_issue(self, item: ItemBinding) -> Option<&'static str> {
        item.value.with_value(|value| {
            if self.index.with(|index| index.invalid.contains(value)) {
                Some("invalid-entry")
            } else if self.index.with(|index| index.entry(value).is_none()) {
                Some("unknown-item")
            } else if self
                .registry
                .with(|registry| registry.duplicates.contains(value))
            {
                Some("duplicate-item")
            } else {
                None
            }
        })
    }
    fn item_disabled(self, item: ItemBinding) -> bool {
        self.disabled()
            || self.item_issue(item).is_some()
            || item.value.with_value(|value| {
                self.index
                    .with(|index| index.entry(value).is_none_or(|entry| entry.disabled))
            })
    }
    fn register(self, item: ItemBinding) {
        self.items.update(|items| items.push(item));
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            self.items
                .try_update(|items| items.retain(|entry| entry.id != item.id));
        });
    }
    fn expanded(self) -> bool {
        self.open.get() && !self.disabled() && self.present.get()
    }
    fn active_id(self) -> Option<String> {
        if !self.expanded() {
            return None;
        }
        self.active.with(|active| {
            active
                .as_ref()
                .and_then(|active| self.registry.with(|registry| registry.one(active)))
                .map(|item| item.id.get_value())
        })
    }
}
fn optional(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}
fn optional_attr<A: leptos::attr::Attribute + 'static>(
    attribute: Option<A>,
) -> Vec<leptos::attr::any_attribute::AnyAttribute> {
    use leptos::attr::any_attribute::IntoAnyAttribute;
    attribute
        .into_iter()
        .map(IntoAnyAttribute::into_any_attr)
        .collect()
}
fn classes(class: String) -> Vec<leptos::attr::any_attribute::AnyAttribute> {
    optional_attr(
        (!class.is_empty())
            .then(|| leptos::tachys::html::class::class(crate::macros::reactive_merge("", &class))),
    )
}

/// Application-owned value, open state and metadata; rendered Item text never defines identity.
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>>,
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(into)] entries: Signal<Vec<SelectEntry>>,
    #[prop(optional_no_strip, into)] on_select: Option<Callback<Cancelable<String>>>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
        Callback<Cancelable<LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_open_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional_no_strip, into)] on_close_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        #[cfg(feature = "ssr")]
        if open.get_untracked() { rustify_ui::StaticRenderIssues::record("select content must start closed during static rendering"); }
        let (invalid, described_by) = crate::form::field::metadata(invalid, described_by);
        let items = RwSignal::new(Vec::<ItemBinding>::new());
        let registry = Memo::new(move |_| items.with(|items| {
            let mut registry = Items::default();
            for item in items { let value = item.value.get_value();
                if registry.by_value.insert(value.clone(), *item).is_some() { registry.duplicates.insert(value); }
                registry.by_id.insert(item.id.get_value(), *item);
            }
            registry
        }));
        let index = Memo::new(move |_| Index::new(entries.get()));
        let context = Context {
            key: StoredValue::new(crate::id::next("select")), requested_id: StoredValue::new(id), trigger_id: StoredValue::new(String::new()),
            list_id: StoredValue::new(crate::id::next("select-list")), form: StoredValue::new(super::form::associated_form(form)), placeholder: StoredValue::new(placeholder),
            value, open, change: on_change, change_open: on_open_change, select: on_select, escape: on_escape_key_down, outside: on_pointer_down_outside,
            focus_open: on_open_auto_focus, focus_close: on_close_auto_focus,
            index, items, registry, active: RwSignal::new(None), initial: StoredValue::new(None), search: StoredValue::new(String::new()),
            disabled, read_only, invalid, described_by, labelled_by: StoredValue::new(labelled_by), aria_label: StoredValue::new(aria_label),
            trigger: NodeRef::new(), content: NodeRef::new(), viewport: NodeRef::new(), mirror: NodeRef::new(), root: NodeRef::new(),
            triggers: RwSignal::new(0), present: RwSignal::new(false), placement: RwSignal::new(LayerPlacement::default()),
            width: RwSignal::new(0.0), scroll_up: RwSignal::new(false), scroll_down: RwSignal::new(false), direction: rustify_ui::use_direction(), anchor_failed: StoredValue::new(false),
            #[cfg(target_arch = "wasm32")]
            timer: StoredValue::new_local(None),
        };
        provide_context(context);
        super::form::register(context.key, &context.form.get_value(), super::form::ControlSpec {
            composing: Signal::stored(false), commit: None,
            flush: Callback::new(move |_| { #[cfg(target_arch = "wasm32")] context.settle(); }),
            focus: Callback::new(move |_| { #[cfg(target_arch = "wasm32")] context.focus(); }),
            owns: Callback::new(move |_element: web_sys::Element| {
                #[cfg(target_arch = "wasm32")] { context.root.try_get_untracked().flatten().is_some_and(|root| root.contains(Some(&_element))) }
                #[cfg(not(target_arch = "wasm32"))] { false }
            }),
        });
        #[cfg(target_arch = "wasm32")]
        context.connect();
        let content = children();
        let class = crate::macros::reactive_merge_with(move || vec![class.get()]);
        view! {
            <div node_ref=context.root class=class data-rustify-part="select-root" data-rustify-select-root=context.key.get_value()
                data-state=move || if open.get() { "open" } else { "closed" } data-rustify-issue=move || context.issue()
                data-disabled=move || context.disabled().then_some("") data-readonly=move || read_only.get().then_some("") data-invalid=move || invalid.get().then_some("")
                dir=move || context.direction.get().attribute()
                on:click=move |_event| { #[cfg(target_arch = "wasm32")] context.trigger_click(_event); }
                on:keydown=move |_event| { #[cfg(target_arch = "wasm32")] context.key(_event); }>
                {content}
                <select node_ref=context.mirror name=optional(name) form=optional(context.form.get_value()) required=move || required.get()
                    disabled=move || context.disabled() tabindex="-1" aria-hidden="true" data-rustify-part="select-native"
                    data-rustify-form-control=context.key.get_value() prop:value=move || context.accepted().unwrap_or_default()
                    on:change=move |_| { #[cfg(target_arch = "wasm32")] context.settle(); }
                    on:invalid=move |event| { event.prevent_default(); #[cfg(target_arch = "wasm32")] context.focus(); }>
                    <option value="" selected=move || context.accepted().is_none()></option>
                    {move || context.index.with(|index| index.entries.iter().map(|entry| {
                        let value = entry.value.clone(); let selected_value = value.clone();
                        let disabled = entry.disabled || index.invalid.contains(&value);
                        view! { <option value=value selected=move || context.selected(&selected_value) disabled=disabled>{entry.text_value.clone()}</option> }
                    }).collect_view())}
                </select>
            </div>
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

fn trigger_inner<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    class: String,
    test_id: String,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::{self, custom::custom_attribute as data};
    let Some(context) = use_context::<Context>() else {
        return super::Slot(
            target,
            (
                attr::disabled(true),
                data("data-rustify-part", "select-trigger"),
                data("data-rustify-issue", "missing-root"),
            ),
        )
        .into_any();
    };
    let first = context.triggers.get_untracked() == 0;
    context.triggers.update(|count| *count += 1);
    let id = if first {
        crate::form::field::control_id(context.requested_id.get_value(), "select-trigger")
    } else {
        crate::id::next("select-trigger")
    };
    if first {
        context.trigger_id.set_value(id.clone());
    }
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        context.triggers.try_update(|count| *count -= 1);
        rustify_ui::defer(move || {
            if context.key.try_get_value().is_some()
                && context.triggers.try_get_untracked() == Some(0)
                && context.open.try_get_untracked() == Some(true)
            {
                context.close_from_layer();
            }
        });
    });
    super::Slot(
        target,
        (
            (
                leptos::tachys::html::node_ref::node_ref(context.trigger),
                attr::id(id),
                attr::r#type("button"),
                attr::role("combobox"),
                attr::form(optional(context.form.get_value())),
                attr::disabled(move || context.disabled()),
                classes(class),
                optional_attr(optional(test_id).map(|id| data("data-testid", id))),
                data("data-rustify-part", "select-trigger"),
                data("data-rustify-select-owner", context.key.get_value()),
                data("data-rustify-form-control", context.key.get_value()),
            ),
            (
                data("aria-label", optional(context.aria_label.get_value())),
                data("aria-labelledby", optional(context.labelled_by.get_value())),
                data("aria-expanded", move || context.expanded().to_string()),
                data("aria-controls", move || {
                    context.expanded().then(|| context.list_id.get_value())
                }),
                data("aria-activedescendant", move || context.active_id()),
                data("aria-haspopup", "listbox"),
                data("aria-readonly", move || {
                    context.read_only.get().then_some("true")
                }),
                data("aria-invalid", move || {
                    context.invalid.get().then_some("true")
                }),
                data("aria-describedby", move || {
                    optional(context.described_by.get())
                }),
                data("data-state", move || {
                    if context.open.get() {
                        "open"
                    } else {
                        "closed"
                    }
                }),
                data("data-placeholder", move || {
                    context.accepted().is_none().then_some("")
                }),
                data("data-disabled", move || context.disabled().then_some("")),
                data("data-readonly", move || {
                    context.read_only.get().then_some("")
                }),
                data("data-invalid", move || context.invalid.get().then_some("")),
                data("dir", move || context.direction.get().attribute()),
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
/// Uses the caller's native button and its listeners as the combobox trigger.
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
pub fn Value(
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let Some(context) = use_context::<Context>() else {
        return view! { <span class=crate::macros::reactive_merge("", &class) data-rustify-part="select-value" data-rustify-issue="missing-root">{placeholder}</span> }.into_any();
    };
    let locale = rustify_ui::use_locale();
    let placeholder = StoredValue::new(placeholder);
    view! { <span class=crate::macros::reactive_merge("", &class) data-rustify-part="select-value" data-placeholder=move || context.accepted().is_none().then_some("")>
        {move || context.accepted().and_then(|value| context.index.with(|index| index.entry(&value).map(|entry| entry.text_value.clone()))).unwrap_or_else(|| {
            let placeholder = placeholder.get_value();
            if !placeholder.is_empty() { placeholder } else if !context.placeholder.read_value().is_empty() { context.placeholder.get_value() }
            else { match locale.get() { rustify_ui::Locale::English => "Choose".into(), rustify_ui::Locale::Chinese => "请选择".into() } }
        })}
    </span> }.into_any()
}
#[component]
pub fn Icon(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! { <span class=crate::macros::reactive_merge("", &class) data-rustify-part="select-icon" aria-hidden="true">{children.map(|children| children())}</span> }
}

#[component]
fn ContentBody(
    #[prop(into)] class: String,
    #[prop(into)] test_id: String,
    attributes: leptos::attr::any_attribute::AnyAttribute,
    children: ChildrenFn,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let context = expect_context::<Context>();
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;
            let held = StoredValue::new_local(None::<Vec<rustify_ui::Listener>>);
            Effect::new(move |_| {
                let Some(content) = context.content.get() else { return; };
                context.present.set(true);
                let Some(parent) = content.parent_node() else { return; };
                // Caller's native handlers run first, before our bubbling defaults.
                held.set_value(Some(vec![
                    rustify_ui::listen(parent.unchecked_ref(), "mousedown", rustify_ui::ListenOptions::default(), move |event| context.content_down(event.unchecked_into())),
                    rustify_ui::listen(parent.unchecked_ref(), "click", rustify_ui::ListenOptions::default(), move |event| context.content_click(event.unchecked_into())),
                    rustify_ui::listen(parent.unchecked_ref(), "pointermove", rustify_ui::ListenOptions::default(), move |event| context.hover(event.unchecked_into())),
                ]));
            });
            on_cleanup(move || {
                held.try_update_value(|held| { held.take(); });
                let _ = context.present.try_set(false);
                let _ = context.scroll_up.try_set(false); let _ = context.scroll_down.try_set(false);
                context.content.try_update(|node| *node = None);
                context.viewport.try_update(|node| *node = None);
            });
        }
        let geometry = crate::geometry::geometry_class(move || format!("min-width:{}px;max-width:calc(100vw - 2rem);", context.width.get().max(1.0)));
        let classes = crate::macros::reactive_merge_with(move || vec![geometry.clone(), class.clone()]);
        view! { <div node_ref=context.content id=context.list_id.get_value() role="listbox" aria-labelledby=optional(context.trigger_id.get_value())
            class=classes data-rustify-part="select-content" data-rustify-select-owner=context.key.get_value() data-testid=optional(test_id)
            data-state="open" data-side=move || context.placement.get().side.as_str() data-align=move || context.placement.get().align.as_str()
            data-rustify-issue=move || context.issue() data-readonly=move || context.read_only.get().then_some("")
            data-invalid=move || context.invalid.get().then_some("") dir=move || context.direction.get().attribute()
            {..attributes}>
            {children()}
        </div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
/// A scope-owned listbox; it opens only after browser attachment and keeps Trigger focus.
#[component]
pub fn Content(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_context::<Context>();
    let class = StoredValue::new(class);
    let test_id = StoredValue::new(test_id);
    let children = StoredValue::new(children);
    view! { <leptos::attribute_interceptor::AttributeInterceptor let:attributes>
    {let attributes = StoredValue::new_local(attributes);
    if let Some(context) = context {
    view! { <Show when=move || {
        #[cfg(target_arch = "wasm32")] { context.open.get() && !context.disabled() && context.trigger.get().is_some() }
        #[cfg(not(target_arch = "wasm32"))] { false }
    }>
        <rustify_ui::Layer fit=true direction=Some(context.direction)
            anchor=Signal::derive(move || {
                #[cfg(target_arch = "wasm32")]
                if let Some(trigger) = context.trigger.get() { return rustify_ui::Anchor::element(trigger.as_ref()); }
                rustify_ui::Anchor::Centred
            })
            on_close=move || { #[cfg(target_arch = "wasm32")] context.close_from_layer(); }
            on_escape_key_down=context.escape
            on_pointer_down_outside=Some(Callback::new(move |event| { if let Some(callback) = context.outside { callback.try_run(event); } }))
            on_open_auto_focus=Some(Callback::new(move |event: Cancelable<()>| {
                let request = Cancelable::new(());
                if let Some(callback) = context.focus_open { callback.try_run(request.clone()); }
                #[cfg(target_arch = "wasm32")]
                if !request.default_prevented() { context.focus(); }
                event.prevent_default();
            }))
            on_close_auto_focus=context.focus_close
            on_placement=Some(Callback::new(move |placement| { let _ = context.placement.try_set(placement); })) class="rui-select-layer">
            <ContentBody class=class.get_value() test_id=test_id.get_value() attributes=attributes.get_value()>{children.with_value(|children| children())}</ContentBody>
        </rustify_ui::Layer>
    </Show> }.into_any()
    } else {
        view! { <div class=crate::macros::reactive_merge("", &class.get_value()) data-testid=optional(test_id.get_value())
            data-rustify-part="select-content" data-rustify-issue="missing-root" {..attributes.get_value()}>
            {children.with_value(|children| children())}
        </div> }.into_any()
    }}
    </leptos::attribute_interceptor::AttributeInterceptor> }
}
#[component]
pub fn Viewport(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let Some(context) = use_context::<Context>() else {
        return view! { <div class=crate::macros::reactive_merge("", &class) data-rustify-part="select-viewport" data-rustify-issue="missing-root">{children()}</div> }.into_any();
    };
    view! { <div node_ref=context.viewport class=crate::macros::reactive_merge("", &class) role="presentation" data-rustify-part="select-viewport"
    on:scroll=move |_| { #[cfg(target_arch = "wasm32")] context.measure_scroll(); }>{children()}</div> }.into_any()
}
#[derive(Clone, Copy)]
struct GroupContext {
    label: StoredValue<String>,
    labels: RwSignal<u32>,
}
#[component]
pub fn Group(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let group = GroupContext { label: StoredValue::new(crate::id::next("select-label")), labels: RwSignal::new(0) };
        provide_context(group);
        view! { <div class=crate::macros::reactive_merge("", &class) role="group" data-rustify-part="select-group"
            aria-labelledby=move || (group.labels.get() > 0).then(|| group.label.get_value())>{children()}</div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn Label(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let group = use_context::<GroupContext>();
    if let Some(group) = group {
        group.labels.update(|count| *count += 1);
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            group.labels.try_update(|count| *count -= 1);
        });
    }
    view! { <div id=group.map(|group| group.label.get_value()) class=crate::macros::reactive_merge("", &class) data-rustify-part="select-label">{children()}</div> }
}
fn item_boundary(
    value: String,
    render: impl FnOnce(Option<Context>, ItemBinding) -> AnyView,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let context = use_context::<Context>();
        let item = ItemBinding {
            value: StoredValue::new(value),
            id: StoredValue::new(crate::id::next("select-item")),
            node: NodeRef::new(),
        };
        if let Some(context) = context {
            context.register(item);
        }
        provide_context(item);
        render(context, item)
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
fn item_inner<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Div, At, Ch>,
    context: Option<Context>,
    item: ItemBinding,
    class: String,
    test_id: String,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::{self, custom::custom_attribute as data};
    let Some(context) = context else {
        return super::Slot(
            target,
            (
                data("data-rustify-part", "select-item"),
                data("data-rustify-issue", "missing-root"),
            ),
        )
        .into_any();
    };
    super::Slot(
        target,
        (
            (
                leptos::tachys::html::node_ref::node_ref(item.node),
                attr::id(item.id.get_value()),
                attr::role("option"),
                classes(class),
                optional_attr(optional(test_id).map(|id| data("data-testid", id))),
                data("data-rustify-part", "select-item"),
                data("data-rustify-select-owner", context.key.get_value()),
                data("data-state", move || {
                    if item.value.with_value(|value| context.selected(value)) {
                        "checked"
                    } else {
                        "unchecked"
                    }
                }),
                data("data-highlighted", move || {
                    context
                        .active
                        .with(|active| active.as_deref() == Some(item.value.read_value().as_str()))
                        .then_some("")
                }),
            ),
            (
                data("aria-selected", move || {
                    item.value
                        .with_value(|value| context.selected(value))
                        .to_string()
                }),
                data("aria-disabled", move || {
                    context.item_disabled(item).then_some("true")
                }),
                data("data-disabled", move || {
                    context.item_disabled(item).then_some("")
                }),
                data("data-readonly", move || {
                    context.read_only.get().then_some("")
                }),
                data("data-rustify-issue", move || context.item_issue(item)),
                data("dir", move || context.direction.get().attribute()),
            ),
        ),
    )
    .into_any()
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    item_boundary(value, move |context, item| {
        let content = children.map(|children| children()).unwrap_or_else(|| {
            if context.is_some() {
                view! { <ItemText /> }.into_any()
            } else {
                ().into_any()
            }
        });
        item_inner(
            leptos::html::div().child(content),
            context,
            item,
            class,
            test_id,
        )
        .into_any()
    })
}
/// Applies one option's semantics to the caller's native div and listeners.
pub fn item_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Div, At, Ch>,
    value: impl Into<String>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    item_boundary(value.into(), move |context, item| {
        item_inner(target, context, item, String::new(), String::new()).into_any()
    })
}
#[component]
pub fn ItemText(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<Context>();
    let item = use_context::<ItemBinding>();
    let issue = if context.is_none() {
        Some("missing-root")
    } else if item.is_none() {
        Some("missing-item")
    } else {
        None
    };
    let text = move || {
        let Some((context, item)) = context.zip(item) else {
            return String::new();
        };
        item.value.with_value(|value| {
            context.index.with(|index| {
                index
                    .entry(value)
                    .map(|entry| entry.text_value.clone())
                    .unwrap_or_default()
            })
        })
    };
    view! { <span class=crate::macros::reactive_merge("", &class) data-rustify-part="select-item-text" data-rustify-issue=issue>{children.map(|children| children()).unwrap_or_else(|| text.into_any())}</span> }
}
#[component]
pub fn ItemIndicator(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<Context>();
    let item = use_context::<ItemBinding>();
    let issue = if context.is_none() {
        Some("missing-root")
    } else if item.is_none() {
        Some("missing-item")
    } else {
        None
    };
    view! { <span class=crate::macros::reactive_merge("", &class) aria-hidden="true" data-rustify-part="select-item-indicator"
    data-rustify-issue=issue data-state=move || context.zip(item).map(|(context, item)| if item.value.with_value(|value| context.selected(value)) { "checked" } else { "unchecked" })>{children.map(|children| children())}</span> }
}
#[component]
pub fn Separator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div class=crate::macros::reactive_merge("", &class) role="separator" data-rustify-part="select-separator" /> }
}
fn scroll_button(up: bool, class: String, children: Option<Children>) -> impl IntoView {
    let context = use_context::<Context>();
    view! { <button type="button" tabindex="-1" aria-hidden="true" class=crate::macros::reactive_merge("", &class)
            data-rustify-part=if up { "select-scroll-up" } else { "select-scroll-down" }
            data-rustify-issue=context.is_none().then_some("missing-root")
            data-rustify-select-owner=context.map(|context| context.key.get_value())
            disabled=move || context.is_none_or(|context| context.disabled() || !(if up { context.scroll_up.get() } else { context.scroll_down.get() }))
    >{children.map(|children| children())}</button> }
}
#[component]
pub fn ScrollUpButton(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    scroll_button(true, class, children)
}
#[component]
pub fn ScrollDownButton(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    scroll_button(false, class, children)
}
