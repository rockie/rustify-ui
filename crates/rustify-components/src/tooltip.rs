//! A short explanation that appears beside the thing it explains.
//!
//! Rust/UI's tooltip was `group-hover` and a positioned `<div>`: no role, no
//! keyboard, and nothing tying it to the control, so a screen reader never
//! heard it and a keyboard user never saw it. This one is a layer on the SDK's
//! own stack, it appears on focus as well as on hover, and the control it
//! belongs to points at it.

use leptos::prelude::*;
use rustify_ui::{Anchor, Layer};

const CONTENT: &str =
    "rounded-md bg-foreground text-background px-2.5 py-1.5 text-xs whitespace-nowrap shadow-lg";

/// Wraps a control and describes it.
///
/// The control keeps its own markup - this component does not replace it - so
/// the description is attached to the element the caller rendered rather than
/// to a wrapper the reader would announce instead. That is the one thing here
/// done to somebody else's element, and it is the whole point of a tooltip.
#[component]
pub fn Tooltip(
    #[prop(into)] open: Signal<bool>,
    on_open_change: impl Fn(bool) + Send + Sync + 'static,
    #[prop(into)] label: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let id = crate::id::next("tooltip");
    let described_by = id.clone();
    let trigger = NodeRef::<leptos::html::Span>::new();
    let on_open_change = std::sync::Arc::new(on_open_change);

    let children = children().add_any_attr(leptos::attr::aria_describedby(move || {
        open.get().then(|| described_by.clone())
    }));

    let anchor = Signal::derive(move || {
        #[cfg(target_arch = "wasm32")]
        if let Some(element) = trigger.get() {
            return Anchor::element(&element.into());
        }
        Anchor::Centred
    });
    let close = {
        let on_open_change = on_open_change.clone();
        move || on_open_change(false)
    };
    // Stored rather than cloned: `Show` and `Layer` each take a children
    // function, and a `String` moved through two of them makes the outer one
    // callable once.
    let content_class = crate::macros::reactive_merge(CONTENT, &class);
    let content_id = StoredValue::new(id);
    let content_test_id = StoredValue::new(test_id);
    view! {
        <span
            node_ref=trigger
            class="inline-flex"
            data-name="Tooltip"
            on:pointerenter={
                let on_open_change = on_open_change.clone();
                move |_| on_open_change(true)
            }
            on:pointerleave={
                let on_open_change = on_open_change.clone();
                move |_| on_open_change(false)
            }
            on:focusin={
                let on_open_change = on_open_change.clone();
                move |_| on_open_change(true)
            }
            on:focusout={
                let on_open_change = on_open_change.clone();
                move |_| on_open_change(false)
            }
        >
            {children}
        </span>
        <Show when=move || open.get() fallback=|| ()>
            <Layer anchor=anchor on_close=close.clone() class="rui-tooltip-layer">
                <div
                    id=move || content_id.get_value()
                    class=content_class
                    data-name="TooltipContent"
                    data-testid=move || content_test_id.get_value()
                    role="tooltip"
                >
                    {move || label.get()}
                </div>
            </Layer>
        </Show>
    }
}
