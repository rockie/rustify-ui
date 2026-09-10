//! A window over the page that has to be dealt with before anything else.
//!
//! Rust/UI's dialog had no `open` property - the application could not say
//! whether it was showing - and the opening, the backdrop click and the
//! Escape key were an inline `<script>` querying the document by a generated
//! id. There was no `role="dialog"`, no `aria-modal`, and no focus trap. Here
//! the application owns `open`, and the trap, the Escape order and the return
//! of focus belong to the SDK's overlay stack.

use leptos::prelude::*;
use rustify_ui::{Anchor, Layer};

const PANEL: &str = "rui:relative rui:w-full rui:max-w-lg rui:rounded-lg rui:border rui:border-border rui:bg-popover rui:p-6 rui:shadow-lg rui:flex rui:flex-col rui:gap-4";
const BACKDROP: &str = "rui:fixed rui:inset-0 rui:bg-foreground/30";
const CLOSE: &str = "rui:absolute rui:top-4 rui:right-4 rui:rounded-sm rui:p-1 rui:text-muted-foreground rui:transition-colors rui:cursor-pointer rui:outline-none rui:hover:bg-muted rui:focus-visible:ring-ring/50 rui:focus-visible:ring-[3px]";

#[component]
pub fn Dialog(
    #[prop(into)] open: Signal<bool>,
    on_open_change: impl Fn(bool) + Send + Sync + 'static,
    #[prop(into)] title: Signal<String>,
    /// One line under the title. Announced with the dialog when there is one.
    #[prop(optional, into)]
    description: Signal<String>,
    /// A modal dialog makes the rest of its scope inert. A modeless one does
    /// not, and is for a panel the user is meant to keep working beside.
    #[prop(default = true)]
    modal: bool,
    /// Whether a click on the backdrop is a request to close. It is by
    /// convention; a dialog that would lose work should say otherwise.
    #[prop(default = true)]
    close_on_backdrop: bool,
    /// The label on the close button, in the application's language.
    #[prop(optional, into)]
    close_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let group = crate::id::next("dialog");
    let title_id = StoredValue::new(format!("{group}-title"));
    let description_id = StoredValue::new(format!("{group}-description"));
    let panel_class = StoredValue::new(crate::macros::merge(PANEL, &class));
    let panel_test_id = StoredValue::new(test_id);
    let close_label = StoredValue::new(if close_label.is_empty() {
        "close".to_string()
    } else {
        close_label
    });
    let on_open_change = StoredValue::new(on_open_change);
    let close = move || on_open_change.with_value(|close| close(false));
    let described = Memo::new(move |_| !description.get().is_empty());
    let children = StoredValue::new(children);
    view! {
        <Show when=move || open.get() fallback=|| ()>
            <Layer
                modal=modal
                anchor=Signal::derive(|| Anchor::Centred)
                on_close=close
                labelled_by=title_id.get_value()
                described_by=if described.get_untracked() {
                    description_id.get_value()
                } else {
                    String::new()
                }
                class="rui-dialog-layer"
                test_id=panel_test_id.get_value()
            >
                <Show when=move || modal && close_on_backdrop fallback=move || {
                    view! { <div class=BACKDROP aria-hidden="true" /> }
                }>
                    <div
                        class=BACKDROP
                        data-name="DialogBackdrop"
                        aria-hidden="true"
                        on:click=move |_| close()
                    />
                </Show>
                <div class=move || panel_class.get_value() data-name="Dialog">
                    <header class="rui:flex rui:flex-col rui:gap-2 rui:pr-8">
                        <h2
                            id=move || title_id.get_value()
                            class="rui:text-lg rui:leading-none rui:font-semibold rui:text-foreground"
                        >
                            {move || title.get()}
                        </h2>
                        <Show when=move || described.get() fallback=|| ()>
                            <p
                                id=move || description_id.get_value()
                                class="rui:text-sm rui:text-muted-foreground"
                            >
                                {move || description.get()}
                            </p>
                        </Show>
                    </header>
                    {move || children.with_value(|children| children())}
                    <button
                        type="button"
                        class=CLOSE
                        data-name="DialogClose"
                        data-testid="dialog-close"
                        aria-label=move || close_label.get_value()
                        on:click=move |_| close()
                    >
                        <crate::icon::Icon glyph=crate::icon::Glyph::Close />
                    </button>
                </div>
            </Layer>
        </Show>
    }
}
