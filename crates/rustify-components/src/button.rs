//! The class strings are Rust/UI's; what a caller can do with them is not.
//!
//! Rust/UI's button is the `variants!` macro's own component: classes, a
//! `href`, and nothing else. This one is a control - it can be disabled, and a
//! disabled one leaves the tab order and calls nothing, which is the whole
//! reason an application marks one.

use crate::variants;
use leptos::prelude::*;

variants! {
    Button {
        base: "rui:inline-flex rui:items-center rui:justify-center rui:gap-2 rui:whitespace-nowrap rui:rounded-md rui:text-sm rui:font-medium rui:shrink-0 rui:w-fit rui:select-none rui:transition-all rui:outline-none rui:cursor-pointer rui:disabled:pointer-events-none rui:disabled:opacity-50 rui:focus-visible:ring-ring/50 rui:focus-visible:ring-[3px] rui:aria-invalid:border-destructive",
        variants: {
            variant: {
                Default: "rui:bg-primary rui:text-primary-foreground rui:hover:bg-primary/90",
                Secondary: "rui:bg-secondary rui:text-secondary-foreground rui:hover:bg-secondary/80",
                Outline: "rui:border rui:border-border rui:bg-background rui:hover:bg-muted",
                Ghost: "rui:hover:bg-muted rui:hover:text-foreground",
                Accent: "rui:bg-accent rui:text-accent-foreground rui:hover:bg-accent/80",
                Destructive: "rui:bg-destructive rui:text-destructive-foreground rui:hover:bg-destructive/90",
                Link: "rui:text-primary rui:underline-offset-4 rui:hover:underline",
            },
            size: {
                Default: "rui:h-9 rui:px-4 rui:py-2",
                Sm: "rui:h-8 rui:px-3 rui:text-xs",
                Lg: "rui:h-10 rui:px-6",
                Icon: "rui:size-9 rui:p-0",
            }
        }
    }
}

/// A button that does one thing.
///
/// `disabled` is a signal because whether an action is available is usually
/// derived from the same state the action changes; the native `disabled`
/// attribute is what takes it out of the tab order, and the click handler
/// checks again because an attribute is not a guarantee about what a
/// synthetic event can reach.
#[component]
pub fn Button(
    on_click: impl Fn() + 'static,
    #[prop(optional, into)] variant: Signal<ButtonVariant>,
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// The accessible name, for a button whose own content is not one - an
    /// icon, or a label that changes with the state.
    #[prop(optional, into)]
    aria_label: String,
    /// The id of the thing this button opens, while it is open.
    #[prop(optional, into)]
    controls: Signal<String>,
    /// Whether the thing it opens is open. Left `None` the attribute is
    /// absent, which is what a button that opens nothing should say.
    #[prop(optional, into)]
    expanded: Signal<Option<bool>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    // An empty `aria-label` would take the name away rather than leave the
    // content to provide it.
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let computed = move || {
        ButtonClass {
            variant: variant.get(),
            size: size.get(),
        }
        .with_class(class.get())
    };
    view! {
        <button
            type="button"
            class=computed
            data-name="Button"
            data-testid=test_id
            aria-label=aria_label
            aria-controls=move || {
                let controls = controls.get();
                (!controls.is_empty()).then_some(controls)
            }
            aria-expanded=move || expanded.get().map(|open| open.to_string())
            prop:disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get_untracked() {
                    on_click();
                }
            }
        >
            {children()}
        </button>
    }
}
