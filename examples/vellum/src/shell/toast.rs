use leptos::prelude::*;
use rustify_ui::toast::{Description, Root, Viewport};

use crate::app::Editor;

/// Vellum keeps one measurable `#toast` box, hidden between messages.
/// Each accepted message uses the core's scheduling and announcement parts.
#[component]
pub fn Toast(editor: Editor) -> impl IntoView {
    let toasts = editor.toasts;
    view! {
        <Viewport>
            <div id="toast" class=move || if toasts.items().is_empty() { "toast hidden" } else { "toast" }>
                <For each=move || toasts.items() key=|toast|toast.seq let:toast>
                    {
                        let seq=toast.seq;
                        view! {
                            <Root open=true duration_ms=toast.duration_ms importance=toast.importance
                                on_open_change=move |open: bool| {if !open {toasts.dismiss(seq);}}>
                                <Description>{toast.message}</Description>
                            </Root>
                        }
                    }
                </For>
            </div>
        </Viewport>
    }
}
