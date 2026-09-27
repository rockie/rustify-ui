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

const PANEL: &str = "relative w-full max-w-lg rounded-lg border border-border bg-popover p-6 shadow-lg flex flex-col gap-4";
const BACKDROP: &str = "fixed inset-0 bg-foreground/30";
const CLOSE: &str = "absolute top-4 right-4 rounded-sm p-1 text-muted-foreground transition-colors cursor-pointer outline-none hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px]";

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
    // The application's word if it gave one; otherwise the SDK's, in the
    // scope's language. Read on each render rather than fixed at build time,
    // so switching language relabels a dialog that is already open.
    let locale = rustify_ui::use_locale();
    let close_label = StoredValue::new((!close_label.is_empty()).then_some(close_label));
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
                // Only a modal one has a backdrop. A modeless dialog is meant
                // to be worked beside, and a sheet of glass over the page -
                // even a transparent one - is exactly what stops that.
                <Show when=move || modal fallback=|| ()>
                    <div
                        class=BACKDROP
                        data-name="DialogBackdrop"
                        aria-hidden="true"
                        on:click=move |_| {
                            if close_on_backdrop {
                                close();
                            }
                        }
                    />
                </Show>
                <div class=move || panel_class.get_value() data-name="Dialog">
                    <header class="flex flex-col gap-2 pr-8">
                        <h2
                            id=move || title_id.get_value()
                            class="text-lg leading-none font-semibold text-foreground"
                        >
                            {move || title.get()}
                        </h2>
                        <Show when=move || described.get() fallback=|| ()>
                            <p
                                id=move || description_id.get_value()
                                class="text-sm text-muted-foreground"
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
                        aria-label=move || {
                            close_label
                                .get_value()
                                .unwrap_or_else(|| {
                                    locale.text(rustify_ui::Message::Close).to_string()
                                })
                        }
                        on:click=move |_| close()
                    >
                        <crate::icon::Icon glyph=crate::icon::Glyph::Close />
                    </button>
                </div>
            </Layer>
        </Show>
    }
}
