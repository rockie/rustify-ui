use leptos::portal::Portal;
use leptos::prelude::*;

use crate::app::Editor;

pub fn show(editor: Editor, message: &str) {
    editor.shell.update(|shell| {
        shell.toast = Some(message.into());
        shell.toast_serial = shell.toast_serial.wrapping_add(1);
    });
}

#[component]
pub fn Toast(editor: Editor) -> impl IntoView {
    let state = Memo::new(move |_| {
        editor
            .shell
            .with(|shell| (shell.toast_serial, shell.toast.is_some()))
    });
    Effect::new(move || {
        let (serial, visible) = state.get();
        if visible {
            rustify_makepad::defer_after(3200, move || {
                if !editor.shell.is_disposed()
                    && editor
                        .shell
                        .with_untracked(|shell| shell.toast_serial == serial)
                {
                    editor.shell.update(|shell| shell.toast = None);
                }
            });
        }
    });
    // A status message stays outside inert content without joining the Escape stack.
    rustify_ui::use_overlay().map(|stack| {
        let root = stack.overlay_root();
        view! {
            <Portal mount=root>
                <div id="toast" class=move || if editor.shell.with(|shell| shell.toast.is_some()) { "toast" } else { "toast hidden" }
                    role="status" aria-live="polite">{move || editor.shell.with(|shell| shell.toast.clone().unwrap_or_default())}</div>
            </Portal>
        }
    })
}
