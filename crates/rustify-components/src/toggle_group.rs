//! Buttons that stay pressed, alone or as a toolbar.
//!
//! Which ones are pressed is the application's. A press is a request for the
//! set the group would have afterwards; the application may grant it, refuse
//! it, or answer with a set of its own, and the buttons show whatever it
//! holds. The group is one tab stop, and the arrows move along it.

use crate::tabs::Orientation;
use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use std::sync::Arc;

const GROUP: &str = "inline-flex w-fit items-center gap-1";
const TOOLBAR: &str =
    "inline-flex w-fit items-center gap-1 rounded-md border border-border bg-card p-1";
const ITEM: &str = "inline-flex h-9 min-w-9 items-center justify-center gap-2 rounded-md px-3 text-sm font-medium whitespace-nowrap text-foreground transition-colors outline-none cursor-pointer select-none hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-pressed:bg-primary aria-pressed:text-primary-foreground aria-pressed:hover:bg-primary/90";

/// One button of a group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToggleItem {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ToggleItem {
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

/// The set a press on `value` asks for.
///
/// With one at a time, a press on the pressed button asks for none and a
/// press on any other asks for that one alone. With several, the press
/// toggles that button and leaves the rest; the set comes back in the order
/// the buttons are in, so that an application comparing sets does not see a
/// change of order as a change.
pub fn pressed_after(
    pressed: &[String],
    items: &[ToggleItem],
    value: &str,
    multiple: bool,
) -> Vec<String> {
    let was = pressed.iter().any(|held| held == value);
    if !multiple {
        return if was {
            Vec::new()
        } else {
            vec![value.to_string()]
        };
    }
    items
        .iter()
        .map(|item| item.value.as_str())
        .filter(|item| {
            if *item == value {
                !was
            } else {
                pressed.iter().any(|held| held == item)
            }
        })
        .map(str::to_string)
        .collect()
}

/// The button that holds the group's tab stop: the one the keyboard was last
/// on, else the first pressed one, else the first that can be reached.
fn tab_stop(items: &[ToggleItem], pressed: &[String], last: Option<&str>) -> Option<String> {
    let open = |value: &str| {
        items
            .iter()
            .any(|item| item.value == value && !item.disabled)
    };
    last.filter(|value| open(value))
        .map(str::to_string)
        .or_else(|| {
            items
                .iter()
                .find(|item| !item.disabled && pressed.contains(&item.value))
                .map(|item| item.value.clone())
        })
        .or_else(|| {
            crate::roving::edge(items.len(), |index| !items[index].disabled, false)
                .map(|index| items[index].value.clone())
        })
}

fn item_id(group: &str, value: &str) -> String {
    format!("{group}-toggle-{value}")
}

/// A group of toggle buttons.
///
/// `toolbar` makes it a toolbar: the role a reader announces as a set of
/// tools, drawn as a bar. Either way each button is `aria-pressed`, and the
/// group or toolbar is named by `aria_label`.
#[component]
pub fn ToggleGroup(
    /// The values of the pressed buttons.
    #[prop(into)]
    value: Signal<Vec<String>>,
    /// Asked for the set a press would leave pressed.
    on_change: impl Fn(Vec<String>) + Send + Sync + 'static,
    #[prop(into)] items: Signal<Vec<ToggleItem>>,
    /// Several pressed at once, rather than at most one.
    #[prop(optional)]
    multiple: bool,
    #[prop(optional)] toolbar: bool,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let group = crate::id::next("toggles");
    let item_test_id = test_id.clone();
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let on_change = Arc::new(on_change);
    // Where the keyboard last was, so Tab comes back to the same button.
    let last = RwSignal::new(None::<String>);
    let stop = Memo::new(move |_| {
        items.with(|items| value.with(|pressed| tab_stop(items, pressed, last.get().as_deref())))
    });
    let base = match (toolbar, orientation) {
        (false, Orientation::Horizontal) => GROUP.to_string(),
        (true, Orientation::Horizontal) => TOOLBAR.to_string(),
        (false, Orientation::Vertical) => format!("{GROUP} flex-col items-stretch"),
        (true, Orientation::Vertical) => format!("{TOOLBAR} flex-col items-stretch"),
    };
    let on_keydown = {
        let group = group.clone();
        move |ev: KeyboardEvent| {
            let items = items.get_untracked();
            let here = stop
                .get_untracked()
                .and_then(|stop| items.iter().position(|item| item.value == stop));
            let reachable = |index: usize| !items[index].disabled;
            let key = ev.key();
            let next = match key.as_str() {
                "Home" => crate::roving::edge(items.len(), reachable, false),
                "End" => crate::roving::edge(items.len(), reachable, true),
                _ => orientation
                    .steps(&key)
                    .and_then(|step| crate::roving::step(items.len(), reachable, here, step)),
            };
            let Some(next) = next else {
                return;
            };
            ev.prevent_default();
            let next = items[next].value.clone();
            crate::dom::focus_id(&item_id(&group, &next));
        }
    };
    view! {
        <div
            class=crate::macros::merge(&base, &class)
            data-name="ToggleGroup"
            data-testid=test_id
            role=if toolbar { "toolbar" } else { "group" }
            aria-label=aria_label
            aria-orientation=toolbar.then(|| orientation.attribute())
            on:keydown=on_keydown
        >
            <For each=move || items.get() key=|item| item.value.clone() let:item>
                {
                    let ToggleItem { value: mine, label, disabled: off } = item;
                    let held = mine.clone();
                    let pressed = Memo::new(move |_| value.with(|pressed| pressed.contains(&held)));
                    let stopping = mine.clone();
                    let focused = mine.clone();
                    let asked = mine.clone();
                    let on_change = on_change.clone();
                    view! {
                        <button
                            type="button"
                            id=item_id(&group, &mine)
                            class=crate::macros::merge(ITEM, if toolbar { "text-card-foreground" } else { "" })
                            data-name="Toggle"
                            data-testid=format!("{item_test_id}-{mine}")
                            aria-pressed=move || pressed.get().to_string()
                            tabindex=move || {
                                if stop.get().as_deref() == Some(stopping.as_str()) { "0" } else { "-1" }
                            }
                            prop:disabled=move || off || disabled.get()
                            on:focus=move |_| last.set(Some(focused.clone()))
                            on:click=move |_| {
                                if off || disabled.get_untracked() {
                                    return;
                                }
                                let next = items
                                    .with_untracked(|items| {
                                        value
                                            .with_untracked(|pressed| {
                                                pressed_after(pressed, items, &asked, multiple)
                                            })
                                    });
                                on_change(next);
                            }
                        >
                            {label}
                        </button>
                    }
                }
            </For>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{pressed_after, tab_stop, ToggleItem};

    fn items() -> Vec<ToggleItem> {
        vec![
            ToggleItem::new("bold", "B"),
            ToggleItem::new("italic", "I"),
            ToggleItem::new("strike", "S").disabled(),
            ToggleItem::new("underline", "U"),
        ]
    }

    fn set(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn one_at_a_time_a_press_asks_for_that_one_or_for_none() {
        let items = items();
        assert_eq!(
            pressed_after(&set(&["bold"]), &items, "italic", false),
            set(&["italic"])
        );
        assert_eq!(
            pressed_after(&set(&["bold"]), &items, "bold", false),
            set(&[])
        );
        assert_eq!(
            pressed_after(&set(&[]), &items, "bold", false),
            set(&["bold"])
        );
    }

    #[test]
    fn several_at_once_a_press_toggles_one_and_keeps_the_buttons_order() {
        let items = items();
        assert_eq!(
            pressed_after(&set(&["underline"]), &items, "bold", true),
            set(&["bold", "underline"])
        );
        assert_eq!(
            pressed_after(&set(&["bold", "underline"]), &items, "bold", true),
            set(&["underline"])
        );
    }

    #[test]
    fn the_tab_stop_is_where_the_keyboard_was_then_what_is_pressed_then_the_first() {
        let items = items();
        assert_eq!(
            tab_stop(&items, &set(&["underline"]), Some("italic")).as_deref(),
            Some("italic")
        );
        assert_eq!(
            tab_stop(&items, &set(&["underline"]), None).as_deref(),
            Some("underline")
        );
        assert_eq!(tab_stop(&items, &set(&[]), None).as_deref(), Some("bold"));
        // A button that cannot be reached never holds the stop.
        assert_eq!(
            tab_stop(&items, &set(&["strike"]), Some("strike")).as_deref(),
            Some("bold")
        );
    }
}
