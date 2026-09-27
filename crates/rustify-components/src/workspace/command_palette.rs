//! Everything the application can do, findable by typing.
//!
//! A command that cannot run stays in the list and says why. Hiding it is
//! worse than showing it: a person who knows a command exists and cannot find
//! it concludes the application has lost it, and a person who finds it greyed
//! out with a reason has learnt something about their own state.

use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use rustify_ui::{Anchor, Layer};

const PANEL: &str =
    "w-[32rem] max-w-full rounded-lg border border-border bg-popover shadow-lg overflow-hidden";
const SEARCH: &str = "w-full border-0 border-b border-border bg-transparent px-4 py-3 text-sm text-foreground outline-none placeholder:text-muted-foreground";
const ITEM: &str = "flex w-full items-center justify-between gap-3 px-4 py-2 text-sm text-foreground cursor-pointer aria-disabled:opacity-50 aria-disabled:cursor-not-allowed";

/// One thing the application can do.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command {
    pub id: String,
    pub label: String,
    /// Words a person might type looking for it, beyond the label.
    pub keywords: String,
    pub enabled: bool,
    /// Why not, when it is not. Shown beside it and announced with it.
    pub reason: String,
}

impl Command {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            keywords: String::new(),
            enabled: true,
            reason: String::new(),
        }
    }

    pub fn keywords(mut self, keywords: impl Into<String>) -> Self {
        self.keywords = keywords.into();
        self
    }

    pub fn unavailable(mut self, reason: impl Into<String>) -> Self {
        self.enabled = false;
        self.reason = reason.into();
        self
    }
}

/// The commands a search matches, in the order they were declared.
///
/// Case-insensitive, over the label and the keywords, and every word in the
/// query has to appear somewhere - so "close tab" finds "close the current
/// tab" without also finding everything that merely mentions closing.
/// Unavailable commands match like any other: they are the answer to "why can
/// I not do this".
pub fn matching(commands: &[Command], query: &str) -> Vec<Command> {
    let words: Vec<String> = query
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect();
    commands
        .iter()
        .filter(|command| {
            if words.is_empty() {
                return true;
            }
            let haystack = format!("{} {}", command.label, command.keywords).to_lowercase();
            words.iter().all(|word| haystack.contains(word))
        })
        .cloned()
        .collect()
}

/// The command an arrow key reaches. Unavailable ones are included: a person
/// has to be able to land on one to hear why it cannot run.
pub fn neighbour(commands: &[Command], active: &str, step: i32) -> Option<String> {
    let here = commands.iter().position(|command| command.id == active);
    crate::roving::step(commands.len(), |_| true, here, step).map(|at| commands[at].id.clone())
}

#[component]
pub fn CommandPalette(
    #[prop(into)] open: Signal<bool>,
    on_open_change: impl Fn(bool) + Send + Sync + 'static,
    #[prop(into)] commands: Signal<Vec<Command>>,
    /// Run it. Called at most once per activation, and never for a command
    /// that cannot run.
    on_run: impl Fn(String) + Send + Sync + 'static,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let group = StoredValue::new(crate::id::next("commands"));
    let on_open_change = StoredValue::new(on_open_change);
    let on_run = StoredValue::new(on_run);
    let placeholder = StoredValue::new(if placeholder.is_empty() {
        "type a command".to_string()
    } else {
        placeholder
    });
    let label = StoredValue::new(if aria_label.is_empty() {
        "commands".to_string()
    } else {
        aria_label
    });
    let panel_test_id = StoredValue::new(if test_id.is_empty() {
        "command-palette".to_string()
    } else {
        test_id
    });

    let query = RwSignal::new(String::new());
    let active = RwSignal::new(String::new());
    let shown = Memo::new(move |_| matching(&commands.get(), &query.get()));
    let search = NodeRef::<leptos::html::Input>::new();
    let command_id = move |id: &str| format!("{}-{id}", group.get_value());

    // Opening starts clean, and the keyboard starts in the search box: a
    // palette that opens on the last thing somebody typed is a palette that
    // runs the wrong command.
    Effect::new(move || {
        if !open.get() {
            return;
        }
        query.set(String::new());
        active.set(
            commands
                .get_untracked()
                .first()
                .map(|command| command.id.clone())
                .unwrap_or_default(),
        );
        if let Some(input) = search.get() {
            let _ = (*input).focus();
        }
    });

    let run = move |id: String| {
        let Some(command) = shown
            .get_untracked()
            .into_iter()
            .find(|command| command.id == id)
        else {
            return;
        };
        if !command.enabled {
            // Visible, reachable, and refused. The reason is already on
            // screen and announced; running it anyway is the one thing that
            // must not happen.
            return;
        }
        on_run.with_value(|run| run(id));
        on_open_change.with_value(|change| change(false));
    };

    let on_keydown = move |event: KeyboardEvent| {
        let shown = shown.get_untracked();
        match event.key().as_str() {
            "ArrowDown" | "ArrowUp" => {
                let step = if event.key() == "ArrowDown" { 1 } else { -1 };
                if let Some(next) = neighbour(&shown, &active.get_untracked(), step) {
                    event.prevent_default();
                    active.set(next);
                }
            }
            "Enter" => {
                // While an IME is composing, Enter belongs to the composition.
                // Running a command on it would be the wrong command, chosen
                // by somebody who was typing a word.
                if event.is_composing() {
                    return;
                }
                event.prevent_default();
                run(active.get_untracked());
            }
            _ => {}
        }
    };

    view! {
        <Show when=move || open.get() fallback=|| ()>
            <Layer
                modal=true
                anchor=Signal::derive(|| Anchor::Centred)
                on_close=move || on_open_change.with_value(|change| change(false))
                class="rui-command-layer"
            >
                <div
                    class=PANEL
                    data-name="CommandPalette"
                    data-testid=move || panel_test_id.get_value()
                    on:keydown=on_keydown
                >
                    <input
                        node_ref=search
                        type="text"
                        class=SEARCH
                        data-testid="command-search"
                        role="combobox"
                        aria-expanded="true"
                        aria-label=move || label.get_value()
                        aria-controls=move || format!("{}-list", group.get_value())
                        aria-activedescendant=move || command_id(&active.get())
                        placeholder=move || placeholder.get_value()
                        prop:value=move || query.get()
                        on:input:target=move |event| query.set(event.target().value())
                    />
                    <ul
                        id=move || format!("{}-list", group.get_value())
                        class="max-h-80 overflow-auto py-1"
                        role="listbox"
                        aria-label=move || label.get_value()
                    >
                        <For each=move || shown.get() key=|command| command.id.clone() let:command>
                            {
                                let Command { id, label, enabled, reason, .. } = command;
                                let mine = id.clone();
                                let current = Memo::new(move |_| active.get() == mine);
                                let asked = id.clone();
                                let reason_id = format!("{}-reason", command_id(&id));
                                let described = (!reason.is_empty()).then(|| reason_id.clone());
                                view! {
                                    <li
                                        id=command_id(&id)
                                        class=ITEM
                                        class=("bg-muted", move || current.get())
                                        data-name="Command"
                                        data-testid=format!("command-{id}")
                                        role="option"
                                        aria-selected=move || current.get().to_string()
                                        aria-disabled=(!enabled).then_some("true")
                                        aria-describedby=described
                                        on:click=move |_| run(asked.clone())
                                    >
                                        <span>{label}</span>
                                        <Show when={
                                            let reason = reason.clone();
                                            move || !reason.is_empty()
                                        }>
                                            <span
                                                id=reason_id.clone()
                                                class="text-xs text-muted-foreground"
                                            >
                                                {reason.clone()}
                                            </span>
                                        </Show>
                                    </li>
                                }
                            }
                        </For>
                        <Show when=move || shown.get().is_empty() fallback=|| ()>
                            <li
                                class="px-4 py-3 text-sm text-muted-foreground"
                                data-testid="command-empty"
                            >
                                "nothing matches"
                            </li>
                        </Show>
                    </ul>
                </div>
            </Layer>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::{matching, neighbour, Command};

    fn commands() -> Vec<Command> {
        vec![
            Command::new("close", "close the current tab").keywords("panel dismiss"),
            Command::new("open", "open a new tab"),
            Command::new("delete", "delete the selected object").unavailable("nothing is selected"),
        ]
    }

    #[test]
    fn an_empty_search_shows_everything_in_the_order_it_was_declared() {
        let shown = matching(&commands(), "");
        assert_eq!(shown.len(), 3);
        assert_eq!(shown[0].id, "close");
    }

    #[test]
    fn every_word_has_to_appear_somewhere() {
        // The point of requiring all of them: "close tab" is a phrase a person
        // types meaning one command, not everything that mentions closing.
        assert_eq!(
            matching(&commands(), "close tab")
                .iter()
                .map(|command| command.id.as_str())
                .collect::<Vec<_>>(),
            ["close"]
        );
        assert_eq!(matching(&commands(), "tab").len(), 2);
    }

    #[test]
    fn keywords_are_searched_as_well_as_the_label() {
        assert_eq!(matching(&commands(), "dismiss").len(), 1);
    }

    #[test]
    fn a_search_is_not_case_sensitive_and_ignores_surrounding_space() {
        assert_eq!(matching(&commands(), "  CLOSE  ").len(), 1);
    }

    #[test]
    fn a_command_that_cannot_run_is_still_found() {
        // Hiding it would tell a person the application had lost it. Showing
        // it with the reason tells them about their own state.
        let shown = matching(&commands(), "delete");
        assert_eq!(shown.len(), 1);
        assert!(!shown[0].enabled);
        assert_eq!(shown[0].reason, "nothing is selected");
    }

    #[test]
    fn a_search_that_matches_nothing_matches_nothing() {
        assert!(matching(&commands(), "xyzzy").is_empty());
    }

    #[test]
    fn the_arrows_reach_the_unavailable_ones_too() {
        // A person has to be able to land on one to hear why it cannot run.
        assert_eq!(neighbour(&commands(), "open", 1).as_deref(), Some("delete"));
        assert_eq!(
            neighbour(&commands(), "delete", 1).as_deref(),
            Some("close")
        );
    }
}
