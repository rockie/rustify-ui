//! A list of commands, opened against something.
//!
//! Rust/UI's dropdown menu was 570 lines of Rust around a `format!`-built
//! `<script>` that measured the trigger, flipped the panel and closed it on an
//! outside click - none of which a strict policy will run. The measuring, the
//! Escape order and the return of focus are the SDK overlay stack's here; what
//! is left is a list with the roles and the arrow keys a menu is expected to
//! have.

use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use rustify_ui::{Anchor, Layer};

const PANEL: &str =
    "rui:min-w-40 rui:rounded-md rui:border rui:border-border rui:bg-popover rui:p-1 rui:shadow-lg";
const ITEM: &str = "rui:flex rui:w-full rui:items-center rui:gap-2 rui:rounded-sm rui:px-2 rui:py-1.5 rui:text-sm rui:text-foreground rui:text-left rui:transition-colors rui:outline-none rui:cursor-pointer rui:hover:bg-muted rui:focus-visible:bg-muted rui:aria-disabled:opacity-50 rui:aria-disabled:cursor-not-allowed rui:aria-disabled:hover:bg-transparent";

/// One command in a menu.
///
/// A command that cannot run stays in the list and says why, rather than
/// disappearing: a menu whose contents change with the state is a menu nobody
/// can learn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    /// Why it cannot run. Announced with the item and shown beside it.
    pub reason: String,
}

impl MenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            reason: String::new(),
        }
    }

    pub fn disabled(mut self, reason: impl Into<String>) -> Self {
        self.disabled = true;
        self.reason = reason.into();
        self
    }
}

#[component]
pub fn Menu(
    #[prop(into)] open: Signal<bool>,
    on_open_change: impl Fn(bool) + Send + Sync + 'static,
    /// What the menu is placed against: an element of the scope, or a
    /// rectangle inside a GPU region.
    #[prop(into)]
    anchor: Signal<Anchor>,
    #[prop(into)] items: Signal<Vec<MenuItem>>,
    on_activate: impl Fn(String) + Send + Sync + 'static,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let group = StoredValue::new(crate::id::next("menu"));
    let panel_class = StoredValue::new(crate::macros::merge(PANEL, &class));
    let panel_test_id = StoredValue::new(test_id);
    let label = StoredValue::new((!aria_label.is_empty()).then_some(aria_label));
    // Stored rather than cloned: `Show`, `Layer` and `For` each take a
    // children function, and a value moved through three of them makes the
    // outermost callable once.
    let on_open_change = StoredValue::new(on_open_change);
    let on_activate = StoredValue::new(on_activate);
    let list = NodeRef::<leptos::html::Ul>::new();

    let item_id = move |id: &str| format!("{}-item-{id}", group.get_value());

    // The keyboard lands in the menu, not behind it: a menu is not modal, so
    // the stack does not move focus for it.
    Effect::new(move || {
        if !open.get() || list.get().is_none() {
            return;
        }
        let items = items.get_untracked();
        if let Some(first) = crate::roving::edge(items.len(), |index| !items[index].disabled, false)
        {
            crate::dom::focus_id(&item_id(&items[first].id));
        }
    });

    let close = move || on_open_change.with_value(|close| close(false));

    let on_keydown = move |ev: KeyboardEvent| {
        let items = items.get_untracked();
        let here = leptos::prelude::document()
            .active_element()
            .and_then(|active| active.get_attribute("data-item"))
            .and_then(|id| items.iter().position(|item| item.id == id));
        let reachable = |index: usize| !items[index].disabled;
        let next = match ev.key().as_str() {
            "ArrowDown" => crate::roving::step(items.len(), reachable, here, 1),
            "ArrowUp" => crate::roving::step(items.len(), reachable, here, -1),
            "Home" => crate::roving::edge(items.len(), reachable, false),
            "End" => crate::roving::edge(items.len(), reachable, true),
            _ => None,
        };
        let Some(next) = next else {
            return;
        };
        ev.prevent_default();
        crate::dom::focus_id(&item_id(&items[next].id));
    };

    view! {
        <Show when=move || open.get() fallback=|| ()>
            <Layer anchor=anchor on_close=close class="rui-menu-layer">
                <ul
                    node_ref=list
                    class=move || panel_class.get_value()
                    data-name="Menu"
                    data-testid=move || panel_test_id.get_value()
                    role="menu"
                    aria-label=move || label.get_value()
                    on:keydown=on_keydown
                >
                    <For each=move || items.get() key=|item| item.id.clone() let:item>
                        {
                            let MenuItem { id, label, disabled, reason } = item;
                            let element_id = item_id(&id);
                            let reason_id = format!("{element_id}-reason");
                            let described = (!reason.is_empty()).then(|| reason_id.clone());
                            let asked = id.clone();
                            view! {
                                <li role="none">
                                    <button
                                        type="button"
                                        role="menuitem"
                                        id=element_id
                                        class=ITEM
                                        data-name="MenuItem"
                                        data-item=id.clone()
                                        data-testid=format!("menu-item-{id}")
                                        aria-disabled=disabled.then_some("true")
                                        aria-describedby=described
                                        on:click=move |_| {
                                            if disabled {
                                                return;
                                            }
                                            on_activate.with_value(|run| run(asked.clone()));
                                            on_open_change.with_value(|close| close(false));
                                        }
                                    >
                                        <span class="rui:flex-1">{label}</span>
                                        <Show when={
                                            let reason = reason.clone();
                                            move || !reason.is_empty()
                                        }>
                                            <span
                                                id=reason_id.clone()
                                                class="rui:text-xs rui:text-muted-foreground"
                                            >
                                                {reason.clone()}
                                            </span>
                                        </Show>
                                    </button>
                                </li>
                            }
                        }
                    </For>
                </ul>
            </Layer>
        </Show>
    }
}
