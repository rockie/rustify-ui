//! A controlled set of expandable sections with native heading semantics.
use super::collapsible;
use crate::{HeadingLevel, Orientation};
use leptos::prelude::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccordionValue {
    Single(Option<String>),
    Multiple(Vec<String>),
}
impl Default for AccordionValue {
    fn default() -> Self {
        Self::Single(None)
    }
}
impl AccordionValue {
    fn contains(&self, key: &str) -> bool {
        match self {
            Self::Single(value) => value.as_deref() == Some(key),
            Self::Multiple(values) => values.iter().any(|v| v == key),
        }
    }
    fn request(&self, key: &str, open: bool, collapsible: bool) -> Option<Self> {
        match self {
            Self::Single(value) if !open && value.as_deref() == Some(key) && !collapsible => None,
            Self::Single(_) => Some(Self::Single(open.then(|| key.to_string()))),
            Self::Multiple(values) => {
                let mut next = values.clone();
                next.retain(|value| value != key);
                if open {
                    next.push(key.to_string());
                }
                Some(Self::Multiple(next))
            }
        }
    }
}
#[derive(Clone, Copy)]
struct ItemBinding {
    key: StoredValue<String>,
    id: StoredValue<String>,
    disabled: Signal<bool>,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Context {
    id: StoredValue<String>,
    value: Signal<AccordionValue>,
    change: Callback<AccordionValue>,
    disabled: Signal<bool>,
    collapsible: bool,
    orientation: Orientation,
    direction: Signal<rustify_ui::Direction>,
    items: RwSignal<Vec<(String, ItemBinding)>>,
    root: NodeRef<leptos::html::Div>,
    focused: StoredValue<Option<String>>,
    owned: StoredValue<bool>,
}
impl Context {
    fn issue(self, key: &str) -> Option<&'static str> {
        if key.is_empty() {
            Some("empty-entry")
        } else if self
            .items
            .with(|items| items.iter().filter(|(value, _)| value == key).count() > 1)
        {
            Some("duplicate-entry")
        } else {
            None
        }
    }
    fn available(self, item: ItemBinding) -> bool {
        self.issue(&item.key.get_value()).is_none() && !self.disabled.get() && !item.disabled.get()
    }
}
#[derive(Clone, Copy)]
struct ItemContext {
    root: Context,
    binding: ItemBinding,
}
#[component]
pub fn Root(
    #[prop(into)] value: Signal<AccordionValue>,
    #[prop(into)] on_change: Callback<AccordionValue>,
    #[prop(default=Orientation::Vertical)] orientation: Orientation,
    #[prop(optional)] collapsible: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view=owner.with(move || {
        let context=Context{id:StoredValue::new(crate::id::next("accordion")),value,change:on_change,disabled,collapsible,orientation,direction:rustify_ui::use_direction(),items:RwSignal::new(Vec::new()),root:NodeRef::new(),focused:StoredValue::new(None),owned:StoredValue::new(false)};
        provide_context(context);
        #[cfg(target_arch="wasm32")] browser::connect(context);
        let children=children();
        view!{<div id=context.id.get_value() node_ref=context.root class=crate::macros::reactive_merge("",&class) data-rustify-part="accordion-root" data-testid=test_id tabindex="-1"
            data-orientation=orientation.attribute() data-disabled=move ||disabled.get().then_some("") dir=move ||context.direction.get().attribute()
            on:keydown=move |_event|{#[cfg(target_arch="wasm32")]browser::keydown(context,_event);}
            on:focusin=move |_event|{#[cfg(target_arch="wasm32")]browser::focusin(context,_event);}
            on:focusout=move |_event|{#[cfg(target_arch="wasm32")]browser::focusout(context,_event);}>{children}</div>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let Some(root) = use_context::<Context>() else {
        return view!{<div class=class data-rustify-part="accordion-item" data-rustify-issue="missing-root">{children()}</div>}.into_any();
    };
    let owner = Owner::new();
    let view=owner.with(move || {
        let binding=ItemBinding{key:StoredValue::new(value),id:StoredValue::new(crate::id::next("accordion-item")),disabled};
        root.items.update(|items|items.push((binding.key.get_value(),binding)));
        #[cfg(target_arch="wasm32")]
        on_cleanup(move ||{root.items.try_update(|items|items.retain(|(_,item)|item.id!=binding.id));});
        provide_context(ItemContext{root,binding});
        let open=Signal::derive(move ||root.issue(&binding.key.get_value()).is_none() && root.value.with(|value|value.contains(&binding.key.get_value())));
        let disabled=Signal::derive(move ||!root.available(binding));
        view!{<collapsible::Root open=open disabled=disabled class=class on_open_change=move |open|{
            if root.available(binding) {if let Some(next)=root.value.with_untracked(|value|value.request(&binding.key.get_value(),open,root.collapsible)){root.change.run(next);}}
        } attr:data-rustify-accordion-item=binding.id.get_value() attr:data-rustify-issue=move ||root.issue(&binding.key.get_value())>{children()}</collapsible::Root>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner).into_any()
}
#[component]
pub fn Header(
    #[prop(default=HeadingLevel::H3)] level: HeadingLevel,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let tag = match level {
        HeadingLevel::H1 => "h1",
        HeadingLevel::H2 => "h2",
        HeadingLevel::H3 => "h3",
        HeadingLevel::H4 => "h4",
        HeadingLevel::H5 => "h5",
        HeadingLevel::H6 => "h6",
    };
    leptos::html::custom(tag)
        .class(crate::macros::reactive_merge("", &class))
        .attr("data-rustify-part", "accordion-header")
        .child(children())
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
    let Some(item) = use_context::<ItemContext>() else {
        return super::Slot(
            target,
            (
                leptos::attr::disabled(true),
                data("data-rustify-part", "accordion-trigger"),
                data("data-rustify-issue", "missing-item"),
            ),
        )
        .into_any();
    };
    let target = target.add_any_attr((
        data("data-rustify-accordion-owner", item.root.id.get_value()),
        data(
            "data-rustify-accordion-trigger",
            item.binding.id.get_value(),
        ),
        super::tabs::optional_attribute((!class.is_empty()).then(|| {
            leptos::tachys::html::class::class(crate::macros::reactive_merge("", &class))
        })),
        super::tabs::optional_attribute(
            (!test_id.is_empty()).then(|| data("data-testid", test_id)),
        ),
    ));
    collapsible::trigger_as(target).into_any()
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
    if use_context::<ItemContext>().is_none() {
        return view!{<div class=class data-rustify-part="accordion-content" data-rustify-issue="missing-item">{children()}</div>}.into_any();
    }
    view!{<collapsible::Content force_mount=force_mount class=class test_id=test_id>{children()}</collapsible::Content>}.into_any()
}
#[cfg(target_arch = "wasm32")]
mod browser {
    use super::*;
    use leptos::{
        ev::{FocusEvent, KeyboardEvent},
        wasm_bindgen::JsCast,
    };
    fn item(context: Context, target: Option<web_sys::EventTarget>) -> Option<ItemBinding> {
        let target = target?
            .dyn_into::<web_sys::Element>()
            .ok()?
            .closest("[data-rustify-accordion-owner]")
            .ok()??;
        if target
            .get_attribute("data-rustify-accordion-owner")
            .as_deref()
            != Some(&context.id.get_value())
        {
            return None;
        }
        let id = target.get_attribute("data-rustify-accordion-trigger")?;
        context.items.with_untracked(|items| {
            items
                .iter()
                .map(|(_, item)| item)
                .find(|item| item.id.get_value() == id)
                .copied()
        })
    }
    fn focus(context: Context, id: Option<String>) {
        if let Some(id) = id {
            if let Some(root) = context.root.get_untracked() {
                if let Ok(Some(target)) =
                    root.query_selector(&format!("[data-rustify-accordion-trigger=\"{id}\"]"))
                {
                    if let Some(target) = target.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = target.focus();
                    }
                }
            }
        } else {
            crate::dom::focus_id(&context.id.get_value());
        }
    }
    pub fn keydown(context: Context, event: KeyboardEvent) {
        if event.default_prevented()
            || event.is_composing()
            || event.alt_key()
            || event.ctrl_key()
            || event.meta_key()
        {
            return;
        }
        let Some(current) = item(context, event.target()) else {
            return;
        };
        let items = ordered(context);
        let at = items.iter().position(|i| i.id == current.id);
        let available = |i| context.available(items[i]);
        let next = match event.key().as_str() {
            "Home" => crate::roving::edge(items.len(), available, false),
            "End" => crate::roving::edge(items.len(), available, true),
            _ => context.orientation.steps(&event.key()).and_then(|step| {
                let step = if context.orientation == Orientation::Horizontal
                    && context.direction.get_untracked() == rustify_ui::Direction::RightToLeft
                {
                    -step
                } else {
                    step
                };
                crate::roving::step(items.len(), available, at, step)
            }),
        };
        if let Some(next) = next {
            event.prevent_default();
            focus(context, Some(items[next].id.get_value()));
        }
    }
    pub fn focusin(context: Context, event: FocusEvent) {
        context.owned.set_value(true);
        if let Some(item) = item(context, event.target()) {
            context.focused.set_value(Some(item.id.get_value()));
        }
    }
    pub fn focusout(context: Context, event: FocusEvent) {
        if let Some(target) = event
            .related_target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
        {
            if !context
                .root
                .get_untracked()
                .is_some_and(|root| root.contains(Some(&target)))
            {
                context.owned.set_value(false);
            }
        }
    }
    fn ordered(context: Context) -> Vec<ItemBinding> {
        let Some(root) = context.root.get_untracked() else {
            return Vec::new();
        };
        let by_id: std::collections::HashMap<_, _> = context
            .items
            .get_untracked()
            .into_iter()
            .map(|(_, item)| (item.id.get_value(), item))
            .collect();
        let Ok(nodes) = root.query_selector_all("[data-rustify-accordion-owner]") else {
            return Vec::new();
        };
        (0..nodes.length())
            .filter_map(|i| nodes.item(i))
            .filter_map(|node| node.dyn_into::<web_sys::Element>().ok())
            .filter(|node| {
                node.get_attribute("data-rustify-accordion-owner")
                    .as_deref()
                    == Some(&context.id.get_value())
            })
            .filter_map(|node| {
                node.get_attribute("data-rustify-accordion-trigger")
                    .and_then(|id| by_id.get(&id).copied())
            })
            .collect()
    }
    pub fn connect(context: Context) {
        use leptos::wasm_bindgen::closure::Closure;
        let order = RwSignal::new(Vec::<String>::new());
        Effect::new(move |_| {
            let Some(root) = context.root.get() else {
                return;
            };
            let collect = move || {
                let ids = ordered(context)
                    .iter()
                    .map(|item| item.id.get_value())
                    .collect::<Vec<_>>();
                if order.get_untracked() != ids {
                    order.set(ids);
                }
            };
            let callback =
                Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(move |_, _| {
                    collect()
                });
            let Ok(observer) = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref())
            else {
                return;
            };
            let options = web_sys::MutationObserverInit::new();
            options.set_child_list(true);
            options.set_subtree(true);
            if observer.observe_with_options(&root, &options).is_err() {
                return;
            }
            let abort = js_sys::Reflect::get(&observer, &"disconnect".into())
                .ok()
                .and_then(|method| method.dyn_into::<js_sys::Function>().ok())
                .map(|disconnect| rustify_ui::release_on_abort(disconnect.bind0(&observer)));
            let held = StoredValue::new_local(Some((observer, callback, abort)));
            on_cleanup(move || {
                held.update_value(|held| {
                    if let Some((observer, _, _)) = held.take() {
                        observer.disconnect();
                    }
                })
            });
            collect();
        });
        let previous = StoredValue::new(Vec::<String>::new());
        Effect::new(move |_| {
            context.root.get();
            context.items.track();
            order.track();
            let items = ordered(context);
            let enabled: Vec<_> = items.iter().map(|item| context.available(*item)).collect();
            if let Some(current) = context.focused.get_value().filter(|id| {
                !items
                    .iter()
                    .enumerate()
                    .any(|(i, item)| item.id.get_value() == *id && enabled[i])
            }) {
                let old = previous.get_value();
                let at = old.iter().position(|id| id == &current).unwrap_or(0);
                let next = crate::roving::replacement(items.len(), |i| enabled[i], at)
                    .map(|i| items[i].id.get_value());
                context.focused.set_value(next.clone());
                let active = document().active_element();
                let lost = active.as_ref().is_none_or(|el| el.tag_name() == "BODY");
                let inside = context
                    .root
                    .get_untracked()
                    .is_some_and(|root| active.as_ref().is_some_and(|el| root.contains(Some(el))));
                if context.owned.get_value() && (lost || inside) {
                    focus(context, next);
                }
            }
            let current_order = items
                .iter()
                .map(|item| item.id.get_value())
                .collect::<Vec<_>>();
            if context.owned.get_value()
                && previous.get_value() != current_order
                && document()
                    .active_element()
                    .is_none_or(|element| element.tag_name() == "BODY")
            {
                if let Some(current) = context.focused.get_value().filter(|id| {
                    items
                        .iter()
                        .enumerate()
                        .any(|(i, item)| enabled[i] && item.id.get_value() == *id)
                }) {
                    // Moving an existing DOM node can drop its focus without removing the item.
                    focus(context, Some(current));
                }
            }
            previous.set_value(current_order);
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_requires_explicit_collapsibility() {
        let value = AccordionValue::Single(Some("a".into()));
        assert_eq!(value.request("a", false, false), None);
        assert_eq!(
            value.request("a", false, true),
            Some(AccordionValue::Single(None))
        );
        assert_eq!(
            value.request("b", true, false),
            Some(AccordionValue::Single(Some("b".into())))
        );
    }
    #[test]
    fn multiple_changes_only_the_requested_section() {
        let value = AccordionValue::Multiple(vec!["a".into(), "b".into()]);
        assert_eq!(
            value.request("a", false, false),
            Some(AccordionValue::Multiple(vec!["b".into()]))
        );
        assert_eq!(
            value.request("c", true, false),
            Some(AccordionValue::Multiple(vec![
                "a".into(),
                "b".into(),
                "c".into()
            ]))
        );
    }
}
