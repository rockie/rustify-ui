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
        base: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium shrink-0 w-fit select-none transition-all outline-none cursor-pointer disabled:pointer-events-none disabled:opacity-50 aria-disabled:opacity-50 aria-disabled:cursor-not-allowed focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:border-destructive",
        variants: {
            variant: {
                Default: "bg-primary text-primary-foreground hover:bg-primary/90",
                Secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                Outline: "border border-border bg-background hover:bg-muted",
                Ghost: "hover:bg-muted hover:text-foreground",
                Accent: "bg-accent text-accent-foreground hover:bg-accent/80",
                Destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
                Link: "text-primary underline-offset-4 hover:underline",
            },
            size: {
                Default: "h-9 px-4 py-2",
                Sm: "h-8 px-3 text-xs",
                Lg: "h-10 px-6",
                Icon: "size-9 p-0",
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
    /// Says the button will not act, and keeps it reachable anyway.
    ///
    /// The difference from `disabled` is who decides. A natively disabled
    /// button never sees the click, so nothing behind it can judge one; this
    /// one is announced as unavailable, stays in the tab order where it can
    /// say why, and still delivers the click - which means the handler is the
    /// authority and has to refuse for itself. A form's submit is the case it
    /// exists for: what the button shows is a courtesy, and `submit()` is the
    /// rule.
    #[prop(optional, into)]
    unavailable: Signal<bool>,
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
            aria-disabled=move || unavailable.get().then_some("true")
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
