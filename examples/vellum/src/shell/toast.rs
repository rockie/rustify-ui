use leptos::portal::Portal;
use leptos::prelude::*;

use crate::app::Editor;

/// Vellum's markup for the SDK's toasts.
///
/// Not the SDK's `ToastRegion`, which renders a message's markup only while
/// it shows: Vellum's specs and stylesheet expect one `#toast` that stays in
/// the page, hidden between messages.
#[component]
pub fn Toast(editor: Editor) -> impl IntoView {
    let message = Memo::new(move |_| editor.toasts.current().map(|toast| toast.message));
    // A status message stays outside inert content without joining the Escape stack.
    rustify_ui::use_overlay().map(|stack| {
        let root = stack.overlay_root();
        view! {
            <Portal mount=root>
                <div id="toast" class=move || if message.with(Option::is_some) { "toast" } else { "toast hidden" }
                    role="status" aria-live="polite">{move || message.get().unwrap_or_default()}</div>
            </Portal>
        }
    })
}
