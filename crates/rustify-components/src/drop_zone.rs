//! A rectangle files can be dropped on, from the operating system.
//!
//! This is the one place HTML5 drag and drop is used (D8). Dragging inside the
//! page is the SDK's own pointer session, because a GPU region cannot take
//! part in the browser's; but a file dragged in from a desktop arrives as a
//! `drop` event and nothing else can receive it.
//!
//! The zone decides nothing about the file. It hands what arrived to the
//! application, which checks it against the limits it declared and says what
//! happened - so that a refusal reads the same whether the file came from here
//! or from the picker beside it.

use leptos::prelude::*;
use leptos::web_sys::DragEvent;

const ZONE: &str = "flex flex-col items-center justify-center gap-2 rounded-md border border-dashed border-border bg-background p-6 text-sm text-muted-foreground transition-colors";
const OVER: &str = "border-primary text-foreground";
const OFF: &str = "opacity-50";

/// A drop zone for files.
///
/// `on_files` is handed the browser's own list; `rustify_ui::files::import_files`
/// turns one of those into the three answers an import has. Nothing is read
/// here, because reading is where the limits apply and the limits are the
/// application's.
#[component]
pub fn DropZone(
    on_files: impl Fn(Option<leptos::web_sys::FileList>) + 'static,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    // How many nested elements the drag is currently inside. A drag moving
    // over a child fires `dragleave` for the parent before `dragenter` for the
    // child, so counting is the only way to know it has actually left: a
    // boolean flickers the highlight off and on as the pointer crosses the
    // text inside the zone.
    let depth = RwSignal::new(0i32);
    // The SDK's words when the application supplies none: a drop zone with no
    // label is still a drop zone, and what it says is the SDK's to say.
    let locale = rustify_ui::use_locale();
    let label = if label.is_empty() {
        locale.text(rustify_ui::Message::DropFileHere).to_string()
    } else {
        label
    };
    // Bound out here because `view!` cannot parse an `if` in an attribute.
    let state = move || {
        if depth.get() > 0 {
            "over"
        } else {
            "idle"
        }
    };
    let classes = move || {
        let over = depth.get() > 0 && !disabled.get();
        crate::macros::merge(
            ZONE,
            &format!(
                "{} {} {}",
                if over { OVER } else { "" },
                if disabled.get() { OFF } else { "" },
                class,
            ),
        )
    };
    view! {
        <div
            class=classes
            data-name="DropZone"
            data-testid=test_id
            data-state=state
            aria-disabled=move || disabled.get().then_some("true")
            aria-describedby=move || {
                let described_by = described_by.get();
                (!described_by.is_empty()).then_some(described_by)
            }
            on:dragenter=move |event: DragEvent| {
                if disabled.get_untracked() {
                    return;
                }
                // Without this the browser does its own thing with the file,
                // which is to navigate away from the application and open it.
                event.prevent_default();
                depth.update(|depth| *depth += 1);
            }
            on:dragover=move |event: DragEvent| {
                if !disabled.get_untracked() {
                    event.prevent_default();
                }
            }
            on:dragleave=move |_| {
                depth.update(|depth| *depth = (*depth - 1).max(0));
            }
            on:drop=move |event: DragEvent| {
                depth.set(0);
                if disabled.get_untracked() {
                    return;
                }
                event.prevent_default();
                on_files(event.data_transfer().and_then(|transfer| transfer.files()));
            }
        >
            {label}
        </div>
    }
}
