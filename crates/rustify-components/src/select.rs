//! Choosing one of a list that is too long to show all at once.
//!
//! Rust/UI's select measured the trigger and flipped the panel from an inline
//! `<script>`, and carried no `combobox`, `listbox` or `option` role. Here the
//! panel is a layer on the SDK's stack, the roles are the ones a reader
//! expects, and the value is the application's: what the trigger shows is what
//! the application holds, never what was clicked.

use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use rustify_ui::{Anchor, Layer};

const TRIGGER: &str = "rui:flex rui:h-9 rui:w-full rui:items-center rui:justify-between rui:gap-2 rui:rounded-md rui:border rui:border-border rui:bg-input rui:px-3 rui:py-1 rui:text-sm rui:text-foreground rui:transition-colors rui:cursor-pointer rui:outline-none rui:focus-visible:ring-ring/50 rui:focus-visible:ring-[3px] rui:disabled:cursor-not-allowed rui:disabled:opacity-50 rui:aria-invalid:border-destructive";
const PANEL: &str = "rui:min-w-40 rui:max-h-64 rui:overflow-auto rui:rounded-md rui:border rui:border-border rui:bg-popover rui:p-1 rui:shadow-lg rui:outline-none";
const OPTION: &str = "rui:flex rui:w-full rui:items-center rui:gap-2 rui:rounded-sm rui:px-2 rui:py-1.5 rui:text-sm rui:text-foreground rui:cursor-pointer rui:aria-disabled:opacity-50 rui:aria-disabled:cursor-not-allowed";

/// One choice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
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

#[component]
pub fn Select(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] options: Signal<Vec<SelectOption>>,
    on_change: impl Fn(String) + Send + Sync + 'static,
    #[prop(into)] open: Signal<bool>,
    on_open_change: impl Fn(bool) + Send + Sync + 'static,
    /// What the trigger shows when the value matches no option - before a
    /// first choice, or after the options changed under one.
    #[prop(optional, into)]
    placeholder: String,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let group = crate::id::next("select");
    let trigger_id = StoredValue::new(if id.is_empty() {
        format!("{group}-trigger")
    } else {
        id
    });
    let list_id = StoredValue::new(format!("{group}-listbox"));
    let group = StoredValue::new(group);
    let placeholder = StoredValue::new(if placeholder.is_empty() {
        "choose".to_string()
    } else {
        placeholder
    });
    let panel_test_id = StoredValue::new(test_id.clone());
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let on_change = StoredValue::new(on_change);
    let on_open_change = StoredValue::new(on_open_change);
    let trigger = NodeRef::<leptos::html::Button>::new();
    let list = NodeRef::<leptos::html::Ul>::new();

    let option_id = move |value: &str| format!("{}-option-{value}", group.get_value());
    // Which option the keyboard is on while the list is open. It starts at the
    // chosen one, so the arrows continue from what the user picked last rather
    // than from the top.
    let active = RwSignal::new(String::new());

    let shown = Memo::new(move |_| {
        let held = value.get();
        options
            .get()
            .into_iter()
            .find(|option| option.value == held)
            .map(|option| option.label)
    });

    let set_open = move |open: bool| {
        if open {
            active.set(value.get_untracked());
        }
        on_open_change.with_value(|change| change(open));
    };

    let choose = move |asked: String| {
        if read_only.get_untracked() {
            set_open(false);
            return;
        }
        on_change.with_value(|change| change(asked));
        set_open(false);
        // The keyboard goes back where it came from, which is what makes a
        // second choice possible without the mouse.
        if let Some(button) = trigger.get_untracked() {
            let _ = (*button).focus();
        }
    };

    // The keyboard moves into the list when it opens: the list carries the
    // focus and `aria-activedescendant` says which option is current, which is
    // the arrangement a combobox is read with.
    Effect::new(move || {
        if !open.get() {
            return;
        }
        if let Some(list) = list.get() {
            let _ = (*list).focus();
        }
    });

    let on_trigger_key = move |ev: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        if matches!(ev.key().as_str(), "ArrowDown" | "ArrowUp") {
            ev.prevent_default();
            set_open(true);
        }
    };

    let on_list_key = move |ev: KeyboardEvent| {
        let options = options.get_untracked();
        let here = options
            .iter()
            .position(|option| option.value == active.get_untracked());
        let reachable = |index: usize| !options[index].disabled;
        let moved = match ev.key().as_str() {
            "ArrowDown" => crate::roving::step(options.len(), reachable, here, 1),
            "ArrowUp" => crate::roving::step(options.len(), reachable, here, -1),
            "Home" => crate::roving::edge(options.len(), reachable, false),
            "End" => crate::roving::edge(options.len(), reachable, true),
            "Enter" | " " => {
                ev.prevent_default();
                if let Some(option) = here.map(|here| &options[here]) {
                    if !option.disabled {
                        choose(option.value.clone());
                    }
                }
                return;
            }
            _ => return,
        };
        let Some(moved) = moved else {
            return;
        };
        ev.prevent_default();
        active.set(options[moved].value.clone());
    };

    view! {
        <button
            node_ref=trigger
            type="button"
            id=move || trigger_id.get_value()
            class=crate::macros::merge(TRIGGER, &class)
            data-name="Select"
            data-testid=test_id
            role="combobox"
            aria-label=aria_label
            aria-haspopup="listbox"
            aria-controls=move || list_id.get_value()
            aria-expanded=move || open.get().to_string()
            aria-invalid=move || invalid.get().then_some("true")
            aria-readonly=move || read_only.get().then_some("true")
            aria-describedby=move || {
                let described_by = described_by.get();
                (!described_by.is_empty()).then_some(described_by)
            }
            prop:disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get_untracked() {
                    set_open(!open.get_untracked());
                }
            }
            on:keydown=on_trigger_key
        >
            <span
                class=move || {
                    if shown.get().is_some() {
                        "rui:truncate"
                    } else {
                        "rui:truncate rui:text-muted-foreground"
                    }
                }
                data-name="SelectValue"
            >
                {move || shown.get().unwrap_or_else(|| placeholder.get_value())}
            </span>
            <crate::icon::Icon glyph=crate::icon::Glyph::ChevronDown class="rui:text-muted-foreground" />
        </button>
        <Show when=move || open.get() fallback=|| ()>
            <Layer
                anchor=Signal::derive(move || match trigger.get() {
                    Some(element) => Anchor::element(&element.into()),
                    None => Anchor::Centred,
                })
                on_close=move || set_open(false)
                class="rui-select-layer"
            >
                <ul
                    node_ref=list
                    id=move || list_id.get_value()
                    class=PANEL
                    data-name="SelectList"
                    data-testid=move || format!("{}-list", panel_test_id.get_value())
                    role="listbox"
                    tabindex="-1"
                    aria-labelledby=move || trigger_id.get_value()
                    aria-activedescendant=move || option_id(&active.get())
                    on:keydown=on_list_key
                >
                    <For each=move || options.get() key=|option| option.value.clone() let:option>
                        {
                            let SelectOption { value: option_value, label, disabled: option_disabled } = option;
                            let mine = option_value.clone();
                            let chosen = Memo::new(move |_| value.get() == mine);
                            let asked = option_value.clone();
                            view! {
                                <li
                                    id=option_id(&option_value)
                                    class=OPTION
                                    class=("rui:bg-muted", {
                                        let mine = option_value.clone();
                                        move || active.get() == mine
                                    })
                                    data-name="SelectOption"
                                    data-testid=format!("option-{option_value}")
                                    role="option"
                                    aria-selected=move || chosen.get().to_string()
                                    aria-disabled=option_disabled.then_some("true")
                                    on:click=move |_| {
                                        if !option_disabled {
                                            choose(asked.clone());
                                        }
                                    }
                                >
                                    <span class="rui:flex-1">{label}</span>
                                    <Show when=move || chosen.get() fallback=|| ()>
                                        <crate::icon::Icon
                                            glyph=crate::icon::Glyph::Check
                                            class="rui:text-muted-foreground"
                                        />
                                    </Show>
                                </li>
                            }
                        }
                    </For>
                </ul>
            </Layer>
        </Show>
    }
}
