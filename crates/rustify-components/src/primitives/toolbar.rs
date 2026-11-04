//! One roving focus sequence for native commands, links, and toggle groups.
pub use super::choice::SelectionMode;
use super::choice::{toggle_group, ItemAttributes};
use crate::Orientation;
use leptos::prelude::*;

#[derive(Clone)]
struct Entry {
    key: String,
    disabled: Signal<bool>,
}
#[derive(Clone, Copy)]
struct Toolbar {
    id: StoredValue<String>,
    entries: RwSignal<Vec<Entry>>,
    current: RwSignal<Option<String>>,
    orientation: Orientation,
    #[cfg(target_arch = "wasm32")]
    owned_focus: StoredValue<bool>,
    #[cfg(not(target_arch = "wasm32"))]
    host_owner: StoredValue<leptos::reactive::owner::WeakOwner>,
    #[cfg(not(target_arch = "wasm32"))]
    host_children: StoredValue<Vec<Owner>>,
}
impl Toolbar {
    #[cfg(target_arch = "wasm32")]
    fn recover(self, root: NodeRef<leptos::html::Div>) {
        use leptos::wasm_bindgen::JsCast;
        let previous = StoredValue::new(Vec::<String>::new());
        Effect::new(move || {
            let entries = self.entries.get();
            let enabled: Vec<_> = entries
                .iter()
                .filter(|entry| !entry.disabled.get())
                .collect();
            let Some(root) = root.get() else {
                return;
            };
            let Ok(nodes) = root.query_selector_all("[data-rustify-toolbar-item]") else {
                return;
            };
            let items: Vec<web_sys::Element> = (0..nodes.length())
                .filter_map(|at| nodes.item(at)?.dyn_into().ok())
                .filter(|item: &web_sys::Element| {
                    item.get_attribute("data-rustify-toolbar").as_deref()
                        == Some(self.id.get_value().as_str())
                })
                .collect();
            let keys: Vec<_> = items
                .iter()
                .filter_map(|item| item.get_attribute("data-rustify-toolbar-item"))
                .collect();
            let usable = |at: usize| enabled.iter().any(|entry| entry.key == keys[at]);
            if let Some(current) = self.current.get_untracked().filter(|current| {
                !keys
                    .iter()
                    .enumerate()
                    .any(|(at, key)| key == current && usable(at))
            }) {
                let former = previous
                    .with_value(|keys| keys.iter().position(|key| key == &current))
                    .unwrap_or(0);
                let next = crate::roving::replacement(keys.len(), usable, former);
                self.current.set(next.map(|at| keys[at].clone()));
                let active = document().active_element();
                let lost = active
                    .as_ref()
                    .is_none_or(|active| active.tag_name() == "BODY");
                let inside = active
                    .as_ref()
                    .is_some_and(|active| root.contains(Some(active)));
                if self.owned_focus.get_value() && (lost || inside) {
                    if let Some(next) = next {
                        super::menu::browser::focus_element(&items[next]);
                    } else {
                        let _ = root.focus();
                    }
                }
            }
            previous.set_value(keys);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn retain_item(self) {
        // SSR consumes each child before serializing the remaining siblings.
        let root = self.host_owner.with_value(|owner| owner.upgrade());
        let mut owner = Owner::current();
        self.host_children.update_value(|children| {
            while let Some(current) = owner.take() {
                if root.as_ref() == Some(&current) {
                    break;
                }
                owner = current.parent();
                children.push(current);
            }
        });
    }

    fn stop(self) -> Option<String> {
        let entries = self.entries.get();
        let current = self.current.get();
        entries
            .iter()
            .find(|entry| !entry.disabled.get() && current.as_ref() == Some(&entry.key))
            .or_else(|| entries.iter().find(|entry| !entry.disabled.get()))
            .map(|entry| entry.key.clone())
    }
    #[cfg(target_arch = "wasm32")]
    fn event_item(self, event: &web_sys::Event) -> Option<web_sys::Element> {
        use leptos::wasm_bindgen::JsCast;
        let target = event.target()?.dyn_into::<web_sys::Element>().ok()?;
        let item = target.closest("[data-rustify-toolbar-item]").ok()??;
        (item.get_attribute("data-rustify-toolbar")?.as_str() == self.id.get_value())
            .then_some(item)
    }
    #[cfg(target_arch = "wasm32")]
    fn navigate(
        self,
        event: web_sys::KeyboardEvent,
        looping: bool,
        direction: rustify_ui::Direction,
    ) {
        if event.default_prevented()
            || event.is_composing()
            || event.ctrl_key()
            || event.alt_key()
            || event.meta_key()
            || event
                .target()
                .is_some_and(|target| rustify_ui::is_text_entry(&target))
        {
            return;
        }
        let Some(current) = self.event_item(event.as_ref()) else {
            return;
        };
        let key = event.key();
        let Some(root) = document().get_element_by_id(&self.id.get_value()) else {
            return;
        };
        let Ok(nodes) = root.query_selector_all("[data-rustify-toolbar-item]") else {
            return;
        };
        use leptos::wasm_bindgen::JsCast;
        let entries = self.entries.get_untracked();
        let items: Vec<web_sys::Element> = (0..nodes.length())
            .filter_map(|at| nodes.item(at)?.dyn_into::<web_sys::Element>().ok())
            .filter(|item| {
                item.get_attribute("data-rustify-toolbar").as_deref()
                    == Some(self.id.get_value().as_str())
            })
            .filter(|item| {
                !item.has_attribute("disabled")
                    && item.get_attribute("aria-disabled").as_deref() != Some("true")
            })
            .filter(|item| {
                entries.iter().any(|entry| {
                    !entry.disabled.get_untracked()
                        && item.get_attribute("data-rustify-toolbar-item").as_deref()
                            == Some(entry.key.as_str())
                })
            })
            .collect();
        let here = items.iter().position(|item| *item == current);
        let Some(next) = destination(
            items.len(),
            here,
            &key,
            self.orientation,
            direction,
            looping,
        )
        .and_then(|at| items.get(at)) else {
            return;
        };
        event.prevent_default();
        self.current
            .set(next.get_attribute("data-rustify-toolbar-item"));
        super::menu::browser::focus_element(next);
    }
}
#[cfg(any(target_arch = "wasm32", test))]
fn destination(
    len: usize,
    current: Option<usize>,
    key: &str,
    orientation: Orientation,
    direction: rustify_ui::Direction,
    looping: bool,
) -> Option<usize> {
    match key {
        "Home" => (len > 0).then_some(0),
        "End" => len.checked_sub(1),
        _ => {
            let step = match (orientation, key) {
                (Orientation::Horizontal, "ArrowRight") => {
                    if direction == rustify_ui::Direction::RightToLeft {
                        -1
                    } else {
                        1
                    }
                }
                (Orientation::Horizontal, "ArrowLeft") => {
                    if direction == rustify_ui::Direction::RightToLeft {
                        1
                    } else {
                        -1
                    }
                }
                (Orientation::Vertical, "ArrowDown") => 1,
                (Orientation::Vertical, "ArrowUp") => -1,
                _ => return None,
            };
            super::choice::navigation(len, current, step, looping)
        }
    }
}
#[component]
pub fn Root(
    #[prop(optional)] orientation: Orientation,
    #[prop(default = true)] loop_focus: bool,
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(|| {
        let bar = Toolbar { id: StoredValue::new(crate::id::next("toolbar")), entries: RwSignal::new(Vec::new()), current: RwSignal::new(None), orientation,
            #[cfg(target_arch="wasm32")] owned_focus: StoredValue::new(false),
            #[cfg(not(target_arch="wasm32"))] host_owner:StoredValue::new(owner.downgrade()),
            #[cfg(not(target_arch="wasm32"))] host_children:StoredValue::new(Vec::new()), };
        provide_context(bar);
        let direction = rustify_ui::use_direction();
        let unnamed = aria_label.trim().is_empty();
        let root = NodeRef::<leptos::html::Div>::new();
        #[cfg(target_arch="wasm32")] bar.recover(root);
        #[cfg(not(target_arch="wasm32"))] let _ = loop_focus;
        view!{<div node_ref=root id=bar.id.get_value() tabindex="-1" role="toolbar" aria-label=aria_label aria-orientation=orientation.attribute()
            dir=move || direction.get().attribute() data-orientation=orientation.attribute()
            data-rustify-part="toolbar-root" data-rustify-issue=unnamed.then_some("missing-name") class=class data-testid=test_id
            on:focusin=move |_event| {
                #[cfg(target_arch="wasm32")]
                { bar.owned_focus.set_value(true);
                if let Some(item) = bar.event_item(_event.as_ref()) { bar.current.set(item.get_attribute("data-rustify-toolbar-item")); } }
            }
            on:focusout=move |_event| {
                #[cfg(target_arch="wasm32")]
                { use leptos::wasm_bindgen::JsCast;
                    if let Some(target) = _event.related_target().and_then(|target| target.dyn_into::<web_sys::Node>().ok()) {
                        if !root.get_untracked().is_some_and(|root| root.contains(Some(&target))) { bar.owned_focus.set_value(false); }
                    }
                }
            }
            on:keydown=move |_event| {
                #[cfg(target_arch="wasm32")] bar.navigate(_event, loop_focus, direction.get_untracked());
            }>{children()}</div>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
/// Adds a native focusable target to its enclosing toolbar's focus sequence.
pub fn item(disabled: Signal<bool>) -> impl leptos::attr::Attribute {
    use leptos::attr::custom::custom_attribute as data;
    let bar = use_context::<Toolbar>();
    let key = crate::id::next("toolbar-item");
    if let Some(bar) = bar {
        #[cfg(not(target_arch = "wasm32"))]
        bar.retain_item();
        bar.entries.update(|entries| {
            entries.push(Entry {
                key: key.clone(),
                disabled,
            })
        });
        #[cfg(target_arch = "wasm32")]
        {
            let cleanup = key.clone();
            on_cleanup(move || {
                bar.entries
                    .try_update(|entries| entries.retain(|entry| entry.key != cleanup));
            });
        }
    }
    let marker = key.clone();
    (
        data("data-rustify-toolbar-item", key),
        data("data-rustify-toolbar", bar.map(|bar| bar.id.get_value())),
        data(
            "data-rustify-issue",
            bar.is_none().then_some("missing-root"),
        ),
        data("tabindex", move || {
            if !disabled.get() && bar.is_some_and(|bar| bar.stop().as_ref() == Some(&marker)) {
                0
            } else {
                -1
            }
        }),
    )
}
pub fn button_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    target.add_any_attr((leptos::attr::r#type("button"), item(false.into())))
}
#[component]
pub fn Button(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional_no_strip, into)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    view! {<button type="button" disabled=move || disabled.get() class=class data-testid=test_id data-rustify-part="toolbar-button"
    on:click=move |event| if let Some(on_click)=on_click { if !disabled.get_untracked() { on_click.run(event); } }
    {..item(disabled)}>{children()}</button>}
}
pub fn link_as<At, Ch>(target: leptos::html::HtmlElement<leptos::html::A, At, Ch>) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    target
        .add_any_attr(item(false.into()))
        .on(leptos::ev::keydown, move |event| {
            if event.default_prevented()
                || event.is_composing()
                || event.ctrl_key()
                || event.alt_key()
                || event.meta_key()
                || event.key() != " "
            {
                return;
            }
            event.prevent_default();
            #[cfg(target_arch = "wasm32")]
            {
                use leptos::wasm_bindgen::JsCast;
                if let Some(target) = event
                    .current_target()
                    .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    target.click();
                }
            }
        })
}
#[component]
pub fn Link(
    #[prop(into)] href: String,
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
            .attr("data-rustify-part", "toolbar-link")
            .child(children()),
    )
}
#[component]
pub fn Separator(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let bar = use_context::<Toolbar>();
    let orientation = if bar.is_some_and(|bar| bar.orientation == Orientation::Vertical) {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    };
    view! {<span role="separator" aria-orientation=orientation.attribute() data-orientation=orientation.attribute() data-rustify-part="toolbar-separator" class=class data-testid=test_id />}
}
#[derive(Clone, Copy)]
struct ToggleDisabled(Signal<bool>);
#[component]
pub fn ToggleGroup(
    #[prop(into)] value: Signal<Vec<String>>,
    #[prop(into)] on_change: Callback<Vec<String>>,
    #[prop(optional)] mode: SelectionMode,
    #[prop(default = true)] allow_empty: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    toggle_group::toolbar_root(
        value,
        on_change,
        mode,
        allow_empty,
        disabled,
        class,
        test_id,
        Box::new(move || {
            provide_context(ToggleDisabled(disabled));
            children()
        }),
    )
}
#[component]
pub fn ToggleItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let group_disabled = use_context::<ToggleDisabled>().map(|context| context.0);
    let unavailable = Signal::derive(move || {
        disabled.get() || group_disabled.is_some_and(|disabled| disabled.get())
    });
    toggle_group::item_as(
        leptos::html::button()
            .class(class)
            .attr("data-testid", test_id)
            .add_any_attr(item(unavailable))
            .child(children()),
        ItemAttributes {
            disabled,
            ..ItemAttributes::new(value)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn navigation_obeys_axis_direction_edges_and_loop() {
        use rustify_ui::Direction::{LeftToRight as Ltr, RightToLeft as Rtl};
        assert_eq!(
            destination(3, Some(0), "ArrowRight", Orientation::Horizontal, Ltr, true),
            Some(1)
        );
        assert_eq!(
            destination(3, Some(0), "ArrowRight", Orientation::Horizontal, Rtl, true),
            Some(2)
        );
        assert_eq!(
            destination(3, Some(0), "ArrowUp", Orientation::Vertical, Ltr, false),
            Some(0)
        );
        assert_eq!(
            destination(3, Some(0), "ArrowDown", Orientation::Horizontal, Ltr, true),
            None
        );
        assert_eq!(
            destination(3, Some(1), "Home", Orientation::Vertical, Rtl, true),
            Some(0)
        );
        assert_eq!(
            destination(3, Some(0), "End", Orientation::Horizontal, Ltr, true),
            Some(2)
        );
        assert_eq!(
            destination(0, None, "End", Orientation::Horizontal, Ltr, true),
            None
        );
    }
}
