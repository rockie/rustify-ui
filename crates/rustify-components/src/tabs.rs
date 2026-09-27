//! Several views in one place, one of them showing.
//!
//! Rust/UI's tabs held the selection in a context signal a trigger wrote to,
//! and carried no `tablist`, `tab` or `tabpanel` role and no keyboard beyond
//! the click. This version is controlled, and is the pattern a screen reader
//! expects: one tab stop for the whole strip, the arrows moving inside it.

use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use std::sync::Arc;

const LIST: &str = "inline-flex w-fit items-center justify-center rounded-lg bg-muted p-[3px] text-muted-foreground";
const TRIGGER: &str = "inline-flex items-center justify-center gap-1.5 rounded-md border border-transparent px-2.5 py-1 text-sm font-medium whitespace-nowrap transition-all cursor-pointer select-none outline-none focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-selected:bg-background aria-selected:text-foreground disabled:pointer-events-none disabled:opacity-50";

/// One tab, and whether it can be reached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tab {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl Tab {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Which way the strip runs, and therefore which arrows move along it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    fn attribute(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// The key that moves one step along, and the one that moves one back.
    fn steps(self, key: &str) -> Option<i32> {
        match (self, key) {
            (Self::Horizontal, "ArrowRight") | (Self::Vertical, "ArrowDown") => Some(1),
            (Self::Horizontal, "ArrowLeft") | (Self::Vertical, "ArrowUp") => Some(-1),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct TabsContext {
    group: String,
    active: Signal<String>,
}

fn tab_id(group: &str, value: &str) -> String {
    format!("{group}-tab-{value}")
}

fn panel_id(group: &str, value: &str) -> String {
    format!("{group}-panel-{value}")
}

/// The next tab a person can actually reach, `step` places along from the one
/// showing.
fn neighbour(tabs: &[Tab], active: &str, step: i32) -> Option<String> {
    let here = tabs.iter().position(|tab| tab.value == active);
    crate::roving::step(tabs.len(), |index| !tabs[index].disabled, here, step)
        .map(|index| tabs[index].value.clone())
}

/// The first tab an arrow can reach, or the last.
fn end(tabs: &[Tab], last: bool) -> Option<String> {
    crate::roving::edge(tabs.len(), |index| !tabs[index].disabled, last)
        .map(|index| tabs[index].value.clone())
}

/// The strip and the panels below it. `children` are the [`TabPanel`]s.
#[component]
pub fn Tabs(
    #[prop(into)] active: Signal<String>,
    #[prop(into)] tabs: Signal<Vec<Tab>>,
    on_activate: impl Fn(String) + Send + Sync + 'static,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let group = crate::id::next("tabs");
    provide_context(TabsContext {
        group: group.clone(),
        active,
    });
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let activate = Arc::new(on_activate);
    let list_class = match orientation {
        Orientation::Horizontal => LIST,
        Orientation::Vertical => "inline-flex w-fit flex-col items-stretch rounded-lg bg-muted p-[3px] text-muted-foreground",
    };
    let on_keydown = {
        let activate = activate.clone();
        let group = group.clone();
        move |ev: KeyboardEvent| {
            let key = ev.key();
            let tabs = tabs.get_untracked();
            let next = match key.as_str() {
                "Home" => end(&tabs, false),
                "End" => end(&tabs, true),
                _ => orientation
                    .steps(&key)
                    .and_then(|step| neighbour(&tabs, &active.get_untracked(), step)),
            };
            let Some(next) = next else {
                return;
            };
            ev.prevent_default();
            activate(next.clone());
            crate::dom::focus_id(&tab_id(&group, &next));
        }
    };
    view! {
        <div
            class=crate::macros::merge("flex gap-2 flex-col", &class)
            data-name="Tabs"
            data-testid=test_id
            data-orientation=orientation.attribute()
        >
            <div
                class=list_class
                data-name="TabList"
                role="tablist"
                aria-label=aria_label
                aria-orientation=orientation.attribute()
                on:keydown=on_keydown
            >
                <For each=move || tabs.get() key=|tab| tab.value.clone() let:tab>
                    {
                        let Tab { value, label, disabled } = tab;
                        let group = group.clone();
                        let mine = value.clone();
                        let selected = Memo::new(move |_| active.get() == mine);
                        let activate = activate.clone();
                        let asked = value.clone();
                        view! {
                            <button
                                type="button"
                                role="tab"
                                id=tab_id(&group, &value)
                                class=TRIGGER
                                data-name="Tab"
                                data-testid=format!("tab-{value}")
                                aria-selected=move || selected.get().to_string()
                                aria-controls=panel_id(&group, &value)
                                tabindex=move || if selected.get() { "0" } else { "-1" }
                                prop:disabled=disabled
                                on:click=move |_| {
                                    if !disabled {
                                        activate(asked.clone());
                                    }
                                }
                            >
                                {label}
                            </button>
                        }
                    }
                </For>
            </div>
            {children()}
        </div>
    }
}

/// One panel. It is in the document whether or not it is showing, so that the
/// application's state inside it survives a look at another tab.
#[component]
pub fn TabPanel(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let context = use_context::<TabsContext>().expect("a TabPanel belongs inside Tabs");
    let active = context.active;
    let mine = value.clone();
    let showing = Memo::new(move |_| active.get() == mine);
    view! {
        <div
            id=panel_id(&context.group, &value)
            class=crate::macros::merge("flex-1 text-sm outline-none", &class)
            class=("hidden", move || !showing.get())
            data-name="TabPanel"
            data-testid=format!("panel-{value}")
            role="tabpanel"
            aria-labelledby=tab_id(&context.group, &value)
            tabindex="0"
            hidden=move || !showing.get()
        >
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{end, neighbour, Tab};

    fn strip() -> Vec<Tab> {
        vec![
            Tab::new("a", "A"),
            Tab::new("b", "B").disabled(),
            Tab::new("c", "C"),
        ]
    }

    #[test]
    fn an_arrow_moves_by_value_and_skips_what_cannot_be_chosen() {
        assert_eq!(neighbour(&strip(), "a", 1).as_deref(), Some("c"));
        assert_eq!(neighbour(&strip(), "c", 1).as_deref(), Some("a"));
    }

    #[test]
    fn home_and_end_reach_the_first_and_last_tab_anyone_can_choose() {
        let strip = vec![
            Tab::new("a", "A").disabled(),
            Tab::new("b", "B"),
            Tab::new("c", "C").disabled(),
        ];
        assert_eq!(end(&strip, false).as_deref(), Some("b"));
        assert_eq!(end(&strip, true).as_deref(), Some("b"));
    }

    #[test]
    fn arrows_still_work_when_the_tab_showing_is_not_in_the_strip() {
        // The application changed the tabs under the selection; the arrows
        // have to be a way back rather than a dead key.
        assert_eq!(neighbour(&strip(), "gone", 1).as_deref(), Some("a"));
        assert_eq!(neighbour(&strip(), "gone", -1).as_deref(), Some("c"));
    }
}
