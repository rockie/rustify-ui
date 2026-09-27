//! Tabs over panels that can be closed.
//!
//! Different from the `Tabs` in the catalogue, and the difference is the close
//! button: a strip whose tabs come and go has to answer what happens to the
//! one showing when it is the one that goes. The answer is a function, because
//! getting it wrong is silent - the user is simply somewhere they did not ask
//! to be.

use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use std::sync::Arc;

const STRIP: &str = "flex items-stretch gap-1 border-b border-border overflow-x-auto";
const TAB: &str = "group inline-flex items-center gap-1.5 rounded-t-md border border-transparent px-3 py-1.5 text-sm whitespace-nowrap cursor-pointer outline-none transition-colors hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-selected:bg-background aria-selected:border-border aria-selected:text-foreground";

/// One panel in the strip.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PanelTab {
    pub id: String,
    pub label: String,
    /// Whether this one can be closed. A workspace usually keeps one that
    /// cannot, so there is always somewhere to be.
    pub closable: bool,
}

impl PanelTab {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            closable: true,
        }
    }

    pub fn permanent(mut self) -> Self {
        self.closable = false;
        self
    }
}

/// Which panel is showing after `closing` is closed.
///
/// The right-hand neighbour, or the left-hand one when there is nothing to the
/// right, or nothing at all when that was the last panel. Closing something
/// that is not showing changes nothing, which is the case that is easy to get
/// wrong by recomputing anyway.
pub fn after_close(tabs: &[PanelTab], active: &str, closing: &str) -> Option<String> {
    if active != closing {
        return Some(active.to_string());
    }
    let at = tabs.iter().position(|tab| tab.id == closing)?;
    tabs.get(at + 1)
        .or_else(|| at.checked_sub(1).and_then(|left| tabs.get(left)))
        .map(|tab| tab.id.clone())
}

/// The panel an arrow key reaches from the one showing.
pub fn neighbour(tabs: &[PanelTab], active: &str, step: i32) -> Option<String> {
    let here = tabs.iter().position(|tab| tab.id == active);
    crate::roving::step(tabs.len(), |_| true, here, step).map(|at| tabs[at].id.clone())
}

#[component]
pub fn PanelTabs(
    #[prop(into)] tabs: Signal<Vec<PanelTab>>,
    #[prop(into)] active: Signal<String>,
    on_activate: impl Fn(String) + Send + Sync + 'static,
    /// Asked to close one. The application decides what that means - what it
    /// must not do is assume the strip has already done it.
    on_close: impl Fn(String) + Send + Sync + 'static,
    #[prop(optional, into)] aria_label: String,
    /// The words on the close control, in the application's language.
    #[prop(optional, into)]
    close_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let group = StoredValue::new(crate::id::next("panels"));
    let on_activate = StoredValue::new(Arc::new(on_activate));
    let on_close = StoredValue::new(Arc::new(on_close));
    let close_label = StoredValue::new(if close_label.is_empty() {
        "close".to_string()
    } else {
        close_label
    });
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let tab_id = move |id: &str| format!("{}-{id}", group.get_value());

    let on_keydown = move |event: KeyboardEvent| {
        let tabs = tabs.get_untracked();
        let step = match event.key().as_str() {
            "ArrowRight" => 1,
            "ArrowLeft" => -1,
            _ => return,
        };
        let Some(next) = neighbour(&tabs, &active.get_untracked(), step) else {
            return;
        };
        event.prevent_default();
        on_activate.with_value(|activate| activate(next.clone()));
        crate::dom::focus_id(&tab_id(&next));
    };

    view! {
        <div
            class=crate::macros::merge(STRIP, &class)
            data-name="PanelTabs"
            data-testid=test_id
            role="tablist"
            aria-label=aria_label
            on:keydown=on_keydown
        >
            <For each=move || tabs.get() key=|tab| tab.id.clone() let:tab>
                {
                    let PanelTab { id, label, closable } = tab;
                    let element_id = tab_id(&id);
                    let test_id = format!("panel-tab-{id}");
                    // Inside a `Show`, which takes a children function: a
                    // `String` moved into one makes it callable once.
                    let close_test_id = StoredValue::new(format!("close-{id}"));
                    let mine = id.clone();
                    let showing = Memo::new(move |_| active.get() == mine);
                    let asked = id.clone();
                    let closed = StoredValue::new(id);
                    view! {
                        <div
                            role="tab"
                            id=element_id
                            class=TAB
                            data-name="PanelTab"
                            data-testid=test_id
                            aria-selected=move || showing.get().to_string()
                            tabindex=move || if showing.get() { "0" } else { "-1" }
                            on:click=move |_| {
                                on_activate.with_value(|activate| activate(asked.clone()))
                            }
                        >
                            <span>{label}</span>
                            <Show when=move || closable fallback=|| ()>
                                <button
                                    type="button"
                                    class="rounded-sm p-0.5 text-muted-foreground outline-none hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                    data-name="PanelTabClose"
                                    data-testid=move || close_test_id.get_value()
                                    aria-label=move || {
                                        format!(
                                            "{} {}",
                                            close_label.get_value(),
                                            closed.get_value(),
                                        )
                                    }
                                    on:click=move |event| {
                                        // Closing is not choosing: without this
                                        // the tab about to go would be
                                        // activated on the way out.
                                        event.stop_propagation();
                                        on_close
                                            .with_value(|close| close(closed.get_value()));
                                    }
                                >
                                    <crate::icon::Icon
                                        glyph=crate::icon::Glyph::Close
                                        class="size-3"
                                    />
                                </button>
                            </Show>
                        </div>
                    }
                }
            </For>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{after_close, neighbour, PanelTab};

    fn strip() -> Vec<PanelTab> {
        vec![
            PanelTab::new("all", "all").permanent(),
            PanelTab::new("blue", "blue"),
            PanelTab::new("green", "green"),
        ]
    }

    #[test]
    fn closing_the_panel_showing_moves_to_its_right_hand_neighbour() {
        assert_eq!(
            after_close(&strip(), "blue", "blue").as_deref(),
            Some("green")
        );
    }

    #[test]
    fn closing_the_last_panel_moves_left_because_there_is_no_right() {
        assert_eq!(
            after_close(&strip(), "green", "green").as_deref(),
            Some("blue")
        );
    }

    #[test]
    fn closing_one_that_is_not_showing_leaves_the_selection_alone() {
        // The case that is easy to get wrong by recomputing anyway: the user
        // closed something they were not looking at, and nothing should move.
        assert_eq!(
            after_close(&strip(), "all", "green").as_deref(),
            Some("all")
        );
    }

    #[test]
    fn closing_the_only_panel_leaves_nowhere_to_be() {
        let one = vec![PanelTab::new("all", "all")];
        assert_eq!(after_close(&one, "all", "all"), None);
    }

    #[test]
    fn closing_a_panel_that_is_not_there_answers_nothing_rather_than_guessing() {
        assert_eq!(after_close(&strip(), "gone", "gone"), None);
    }

    #[test]
    fn the_arrows_walk_the_strip_and_join_its_ends() {
        assert_eq!(neighbour(&strip(), "all", 1).as_deref(), Some("blue"));
        assert_eq!(neighbour(&strip(), "green", 1).as_deref(), Some("all"));
        assert_eq!(neighbour(&strip(), "all", -1).as_deref(), Some("green"));
    }
}
