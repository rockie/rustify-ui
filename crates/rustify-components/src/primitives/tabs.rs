//! Controlled tab panels with focus independent of the accepted selection.
use crate::Orientation;
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PanelEntry {
    pub value: String,
    pub disabled: bool,
}
impl PanelEntry {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActivationMode {
    #[default]
    Automatic,
    Manual,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Index {
    entries: Vec<PanelEntry>,
    invalid: HashSet<String>,
}
impl Index {
    fn new(entries: Vec<PanelEntry>) -> Self {
        let mut seen = HashSet::new();
        let mut invalid = HashSet::new();
        for entry in &entries {
            if entry.value.is_empty() || !seen.insert(entry.value.clone()) {
                invalid.insert(entry.value.clone());
            }
        }
        Self { entries, invalid }
    }
    fn available(&self, key: &str) -> bool {
        !self.invalid.contains(key) && self.entries.iter().any(|e| e.value == key && !e.disabled)
    }
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn next(
        &self,
        key: Option<&str>,
        keypress: &str,
        orientation: Orientation,
        direction: rustify_ui::Direction,
        looping: bool,
    ) -> Option<String> {
        let here = key.and_then(|key| self.entries.iter().position(|e| e.value == key));
        let available = |i: usize| self.available(&self.entries[i].value);
        let index = match keypress {
            "Home" => crate::roving::edge(self.entries.len(), available, false),
            "End" => crate::roving::edge(self.entries.len(), available, true),
            _ => {
                let mut step = orientation.steps(keypress)?;
                if orientation == Orientation::Horizontal
                    && direction == rustify_ui::Direction::RightToLeft
                {
                    step = -step;
                }
                if looping {
                    crate::roving::step(self.entries.len(), available, here, step)
                } else {
                    let reachable: Vec<_> =
                        (0..self.entries.len()).filter(|i| available(*i)).collect();
                    let at = here.and_then(|here| reachable.iter().position(|i| *i == here));
                    let next = at.map_or(
                        if step > 0 {
                            0
                        } else {
                            reachable.len().saturating_sub(1)
                        },
                        |at| {
                            (at as i32 + step).clamp(0, reachable.len().saturating_sub(1) as i32)
                                as usize
                        },
                    );
                    reachable.get(next).copied()
                }
            }
        }?;
        Some(self.entries[index].value.clone())
    }
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug)]
struct Binding {
    key: String,
    id: String,
    trigger: bool,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Context {
    id: StoredValue<String>,
    value: Signal<Option<String>>,
    change: Callback<Option<String>>,
    index: Memo<Index>,
    ids: StoredValue<HashMap<String, (String, String)>>,
    bindings: RwSignal<Vec<Binding>>,
    focused: RwSignal<Option<String>>,
    orientation: Orientation,
    mode: ActivationMode,
    looping: bool,
    direction: Signal<rustify_ui::Direction>,
    root: NodeRef<leptos::html::Div>,
    owned_focus: StoredValue<bool>,
}
impl Context {
    fn pair(self, key: &str) -> (String, String) {
        self.ids.update_value(|ids| {
            ids.entry(key.into()).or_insert_with(|| {
                (
                    crate::id::next("tab-trigger"),
                    crate::id::next("tab-content"),
                )
            });
        });
        self.ids.with_value(|ids| ids[key].clone())
    }
    fn issue(self, key: &str, trigger: bool) -> Option<&'static str> {
        if key.is_empty() {
            Some("empty-entry")
        } else if self.index.with(|index| index.invalid.contains(key)) {
            Some("duplicate-entry")
        } else if !self
            .index
            .with(|index| index.entries.iter().any(|e| e.value == key))
        {
            Some("missing-entry")
        } else if self.bindings.with(|parts| {
            parts
                .iter()
                .filter(|p| p.key == key && p.trigger == trigger)
                .count()
                > 1
        }) {
            Some("duplicate-part")
        } else {
            None
        }
    }
    fn usable(self, key: &str) -> bool {
        self.issue(key, true).is_none()
            && self.bindings.with(|parts| {
                parts
                    .iter()
                    .filter(|part| part.trigger && part.key == key)
                    .count()
                    == 1
            })
            && self.index.with(|index| index.available(key))
    }
    fn selected(self, key: &str) -> bool {
        self.issue(key, false).is_none()
            && self.index.with(|index| {
                !index.invalid.contains(key) && index.entries.iter().any(|e| e.value == key)
            })
            && self.value.get().as_deref() == Some(key)
    }
    fn tab_stop(self) -> Option<String> {
        self.focused
            .get()
            .filter(|key| self.usable(key))
            .or_else(|| self.value.get().filter(|key| self.usable(key)))
            .or_else(|| {
                self.index.with(|index| {
                    index
                        .entries
                        .iter()
                        .find(|e| self.usable(&e.value))
                        .map(|e| e.value.clone())
                })
            })
    }
    fn bind(self, key: String, trigger: bool) -> String {
        let pair = self.pair(&key);
        let first = self
            .bindings
            .with_untracked(|parts| !parts.iter().any(|p| p.key == key && p.trigger == trigger));
        let id = if first {
            if trigger {
                pair.0
            } else {
                pair.1
            }
        } else {
            crate::id::next(if trigger {
                "tab-trigger"
            } else {
                "tab-content"
            })
        };
        self.bindings.update(|parts| {
            parts.push(Binding {
                key,
                id: id.clone(),
                trigger,
            })
        });
        #[cfg(target_arch = "wasm32")]
        let removed = id.clone();
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            self.bindings
                .try_update(|parts| parts.retain(|p| p.id != removed));
        });
        id
    }
}
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>>,
    #[prop(into)] entries: Signal<Vec<PanelEntry>>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] activation_mode: ActivationMode,
    #[prop(optional_no_strip, into)] direction: Option<Signal<rustify_ui::Direction>>,
    #[prop(default = true)] loop_focus: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let context = Context { id: StoredValue::new(crate::id::next("tabs")), value, change: on_change, index: Memo::new(move |_| Index::new(entries.get())),
            ids: StoredValue::new(HashMap::new()), bindings: RwSignal::new(Vec::new()), focused: RwSignal::new(None), orientation, mode: activation_mode,
            looping: loop_focus, direction: direction.unwrap_or_else(rustify_ui::use_direction), root: NodeRef::new(), owned_focus: StoredValue::new(false) };
        for entry in entries.get_untracked() { context.pair(&entry.value); }
        provide_context(context);
        #[cfg(target_arch = "wasm32")] browser::connect(context);
        let children = children();
        view! { <div id=context.id.get_value() node_ref=context.root class=crate::macros::reactive_merge("",&class) data-rustify-part="tabs-root" data-testid=test_id
            data-orientation=orientation.attribute() dir=move || context.direction.get().attribute() tabindex="-1"
            on:click=move |_event| { #[cfg(target_arch = "wasm32")] browser::click(context, _event); }
            on:keydown=move |_event| { #[cfg(target_arch = "wasm32")] browser::keydown(context, _event); }
            on:focusin=move |_event| { #[cfg(target_arch = "wasm32")] browser::focusin(context, _event); }
            on:focusout=move |_event| { #[cfg(target_arch = "wasm32")] browser::focusout(context, _event); }
        >{children}</div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn List(
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let context = use_context::<Context>();
    view! { <div role="tablist" aria-label=(!aria_label.is_empty()).then_some(aria_label) class=crate::macros::reactive_merge("",&class) data-rustify-part="tabs-list"
    data-rustify-issue=context.is_none().then_some("missing-root") aria-orientation=context.map(|c| c.orientation.attribute())>{children()}</div> }
}
fn trigger_inner<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    value: String,
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
                data("data-rustify-part", "tab-trigger"),
                data("data-rustify-issue", "missing-root"),
                leptos::attr::disabled(true),
            ),
        )
        .into_any();
    };
    let id = context.bind(value.clone(), true);
    let value = StoredValue::new(value);
    let pair = context.pair(&value.get_value());
    super::Slot(
        target,
        (
            (
                leptos::attr::id(id),
                leptos::attr::r#type("button"),
                leptos::attr::role("tab"),
                leptos::attr::disabled(move || !context.usable(&value.get_value())),
                data("aria-controls", pair.1),
                data("aria-selected", move || {
                    context.selected(&value.get_value()).to_string()
                }),
                data("tabindex", move || {
                    if context.tab_stop().as_deref() == Some(&value.get_value()) {
                        "0"
                    } else {
                        "-1"
                    }
                }),
                data("data-rustify-tabs-owner", context.id.get_value()),
                data("data-rustify-part", "tab-trigger"),
                data("data-rustify-issue", move || {
                    context.issue(&value.get_value(), true)
                }),
                data("data-disabled", move || {
                    (!context.usable(&value.get_value())).then_some("")
                }),
                data("data-state", move || {
                    if context.selected(&value.get_value()) {
                        "active"
                    } else {
                        "inactive"
                    }
                }),
            ),
            (
                optional_attribute((!class.is_empty()).then(|| {
                    leptos::tachys::html::class::class(crate::macros::reactive_merge("", &class))
                })),
                optional_attribute((!test_id.is_empty()).then(|| data("data-testid", test_id))),
            ),
        ),
    )
    .into_any()
}
#[component]
pub fn Trigger(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    trigger_inner(
        leptos::html::button().child(children()),
        value,
        class,
        test_id,
    )
}
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    value: impl Into<String>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    trigger_inner(target, value.into(), String::new(), String::new())
}
#[component]
pub fn Content(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let Some(context) = use_context::<Context>() else {
        return view! { <div class=crate::macros::reactive_merge("",&class) data-rustify-part="tab-content" data-rustify-issue="missing-root">{children()}</div> }.into_any();
    };
    let id = context.bind(value.clone(), false);
    let pair = context.pair(&value);
    let value = StoredValue::new(value);
    view! { <div id=id class=crate::macros::reactive_merge("",&class) data-rustify-part="tab-content" data-testid=test_id role="tabpanel" aria-labelledby=pair.0 tabindex="0"
        data-rustify-issue=move || context.issue(&value.get_value(),false)
        data-state=move || if context.selected(&value.get_value()) {"active"}else{"inactive"}
        hidden=move || !context.selected(&value.get_value()) inert=move || !context.selected(&value.get_value())>{children()}</div> }.into_any()
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::*;
    use leptos::{
        ev::{FocusEvent, KeyboardEvent, MouseEvent},
        wasm_bindgen::JsCast,
    };
    fn binding(context: Context, target: Option<web_sys::EventTarget>) -> Option<Binding> {
        let element = target?
            .dyn_into::<web_sys::Element>()
            .ok()?
            .closest("[data-rustify-tabs-owner]")
            .ok()??;
        if element.get_attribute("data-rustify-tabs-owner").as_deref()
            != Some(&context.id.get_value())
        {
            return None;
        }
        context.bindings.with_untracked(|parts| {
            parts
                .iter()
                .find(|p| p.trigger && p.id == element.id())
                .cloned()
        })
    }
    pub fn click(context: Context, event: MouseEvent) {
        if event.default_prevented() {
            return;
        }
        if let Some(part) = binding(context, event.target()).filter(|p| context.usable(&p.key)) {
            context.focused.set(Some(part.key.clone()));
            context.change.run(Some(part.key));
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
        let Some(part) = binding(context, event.target()).filter(|p| context.usable(&p.key)) else {
            return;
        };
        // Native button Enter/Space synthesize one click; the caller can cancel that event.
        if let Some(next) = context.index.with_untracked(|index| {
            index.next(
                Some(&part.key),
                &event.key(),
                context.orientation,
                context.direction.get_untracked(),
                context.looping,
            )
        }) {
            event.prevent_default();
            context.focused.set(Some(next.clone()));
            crate::dom::focus_id(&context.pair(&next).0);
            if context.mode == ActivationMode::Automatic {
                context.change.run(Some(next));
            }
        }
    }
    pub fn focusin(context: Context, event: FocusEvent) {
        context.owned_focus.set_value(true);
        if let Some(part) = binding(context, event.target()) {
            context.focused.set(Some(part.key));
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
                context.owned_focus.set_value(false);
            }
        }
    }
    pub fn connect(context: Context) {
        let previous = StoredValue::new(Vec::<String>::new());
        Effect::new(move |_| {
            let entries = context.index.get();
            let parts = context.bindings.get();
            let focused = context.focused.get_untracked();
            let enabled: Vec<_> = entries
                .entries
                .iter()
                .filter(|e| {
                    context.usable(&e.value) && parts.iter().any(|p| p.trigger && p.key == e.value)
                })
                .map(|e| e.value.clone())
                .collect();
            if focused.as_ref().is_some_and(|key| !enabled.contains(key)) {
                let old = previous.get_value();
                let at = focused
                    .as_ref()
                    .and_then(|key| old.iter().position(|k| k == key))
                    .unwrap_or(0);
                let next = crate::roving::replacement(
                    entries.entries.len(),
                    |i| context.usable(&entries.entries[i].value),
                    at,
                )
                .map(|i| entries.entries[i].value.clone());
                context.focused.set(next.clone());
                let active = document().active_element();
                let lost = active.as_ref().is_none_or(|el| el.tag_name() == "BODY");
                let inside = context
                    .root
                    .get_untracked()
                    .is_some_and(|root| active.as_ref().is_some_and(|el| root.contains(Some(el))));
                if context.owned_focus.get_value() && (lost || inside) {
                    crate::dom::focus_id(
                        &next
                            .map(|key| context.pair(&key).0)
                            .unwrap_or_else(|| context.id.get_value()),
                    );
                }
            }
            previous.set_value(
                entries
                    .entries
                    .iter()
                    .map(|entry| entry.value.clone())
                    .collect(),
            );
        });
    }
}

pub(super) fn optional_attribute<A: leptos::attr::Attribute + 'static>(
    attribute: Option<A>,
) -> Vec<leptos::attr::any_attribute::AnyAttribute> {
    use leptos::attr::any_attribute::IntoAnyAttribute;
    attribute
        .into_iter()
        .map(IntoAnyAttribute::into_any_attr)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_ui::Direction::{LeftToRight as Ltr, RightToLeft as Rtl};
    #[test]
    fn arrows_respect_direction_orientation_and_loop_edges() {
        let index = Index::new(vec![
            PanelEntry::new("a"),
            PanelEntry::new("b").disabled(),
            PanelEntry::new("c"),
        ]);
        assert_eq!(
            index.next(Some("a"), "ArrowRight", Orientation::Horizontal, Ltr, true),
            Some("c".into())
        );
        assert_eq!(
            index.next(Some("a"), "ArrowLeft", Orientation::Horizontal, Rtl, true),
            Some("c".into())
        );
        assert_eq!(
            index.next(Some("c"), "ArrowRight", Orientation::Horizontal, Ltr, false),
            Some("c".into())
        );
        assert_eq!(
            index.next(Some("c"), "ArrowRight", Orientation::Horizontal, Ltr, true),
            Some("a".into())
        );
        assert_eq!(
            index.next(Some("a"), "ArrowRight", Orientation::Vertical, Ltr, true),
            None
        );
        assert_eq!(
            index.next(Some("a"), "ArrowDown", Orientation::Vertical, Rtl, true),
            Some("c".into())
        );
    }
    #[test]
    fn home_end_and_missing_selection_reach_only_valid_keys() {
        let index = Index::new(vec![
            PanelEntry::new("a"),
            PanelEntry::new("dup"),
            PanelEntry::new("dup"),
            PanelEntry::new("c"),
        ]);
        assert_eq!(
            index.next(None, "Home", Orientation::Horizontal, Ltr, true),
            Some("a".into())
        );
        assert_eq!(
            index.next(None, "End", Orientation::Horizontal, Ltr, true),
            Some("c".into())
        );
        assert_eq!(
            index.next(
                Some("gone"),
                "ArrowRight",
                Orientation::Horizontal,
                Ltr,
                true
            ),
            Some("a".into())
        );
        assert_eq!(
            Index::default().next(None, "Home", Orientation::Horizontal, Ltr, true),
            None
        );
    }
}
