use leptos::prelude::*;
use rustify_components::{
    Button, ButtonVariant, Dialog, Menu, MenuItem, Progress, Toaster, Tooltip,
};
use rustify_ui::{provide_toasts, Anchor, ToastOptions, ToastTone};

/// Progress uses external geometry rules for its application-owned value.
pub fn progress_demo() -> impl IntoView {
    let progress = RwSignal::new(25.0_f64);
    view! {
        <div class="space-y-4">
            <Progress value=progress max=100.0 aria_label="Example progress" />
            <div class="flex flex-wrap items-center gap-3">
                <Button on_click=move || progress.update(|value| *value = (*value + 25.0).min(100.0))
                    disabled=Signal::derive(move || progress.get() >= 100.0)>
                    "Advance"
                </Button>
                <Button variant=ButtonVariant::Outline on_click=move || progress.set(0.0)>"Reset"</Button>
            </div>
            <p class="text-sm">"Complete: "<output data-demo-state="">{move || progress.get().to_string()}</output>"%"</p>
        </div>
    }
}

/// A tooltip opens on focus or hover and describes its real trigger.
pub fn tooltip_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <Tooltip open=open on_open_change=move |value| open.set(value) label="Help also appears when this button receives keyboard focus.">
                <Button variant=ButtonVariant::Outline on_click=|| {}>"Hover or focus for help"</Button>
            </Tooltip>
            <p class="text-sm">"Tooltip: "<output data-demo-state="">{move || if open.get() { "Open" } else { "Closed" }}</output></p>
        </div>
    }
}

/// A menu anchors to its trigger and reports the selected action.
pub fn menu_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    let action = RwSignal::new("None".to_string());
    let trigger = NodeRef::<leptos::html::Button>::new();
    let anchor = Signal::derive(move || {
        #[cfg(target_arch = "wasm32")]
        if let Some(button) = trigger.get() {
            return Anchor::element(button.as_ref());
        }
        Anchor::Centred
    });
    view! {
        <div class="space-y-4">
            <button type="button" node_ref=trigger aria-haspopup="menu"
                aria-expanded=move || open.get().to_string() on:click=move |_| open.set(true)
                class="inline-flex h-9 items-center rounded-md border border-border bg-background px-4 text-sm font-medium outline-none hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px]">
                "Open menu"
            </button>
            <Menu open=open on_open_change=move |value| open.set(value) anchor=anchor
                aria_label="Example actions" on_activate=move |value| action.set(value)
                items=vec![
                    MenuItem::heading("Actions"),
                    MenuItem::new("copy", "Copy"),
                    MenuItem::new("rename", "Rename"),
                    MenuItem::separator(),
                    MenuItem::new("delete", "Delete").disabled("Deletion is unavailable in this example."),
                ] />
            <p class="text-sm">"Last action: "<output data-demo-state="">{move || action.get()}</output></p>
        </div>
    }
}

/// A modal dialog contains focus until confirmation or dismissal.
pub fn dialog_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    let result = RwSignal::new("Not confirmed".to_string());
    view! {
        <div class="space-y-4">
            <Button on_click=move || open.set(true) expanded=Signal::derive(move || Some(open.get()))>
                "Open dialog"
            </Button>
            <Dialog open=open on_open_change=move |value| open.set(value)
                title="Confirm an action" description="Press Confirm, or close the dialog without confirming."
                close_label="Close dialog">
                <Button on_click=move || { result.set("Confirmed".to_string()); open.set(false); }>
                    "Confirm"
                </Button>
            </Dialog>
            <p class="text-sm">"Result: "<output data-demo-state="">{move || result.get()}</output></p>
        </div>
    }
}

/// A notification is transient; the application can observe its lifetime.
pub fn toast_demo() -> impl IntoView {
    let toasts = provide_toasts();
    view! {
        <div class="space-y-4">
            <Button on_click=move || {
                toasts.show("The example action completed.", ToastOptions {
                    duration_ms: 5000,
                    tone: ToastTone::Success,
                });
            }>"Show notification"</Button>
            <p class="text-sm">"Notification: "<output data-demo-state="">{move || {
                toasts.current().map(|toast| toast.message).unwrap_or_else(|| "None".to_string())
            }}</output></p>
            <Toaster test_id="toast-demo" />
        </div>
    }
}
