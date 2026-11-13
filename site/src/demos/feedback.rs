use leptos::prelude::*;
use rustify_components::{alert_dialog, popover};
use rustify_components::{
    Button, ButtonVariant, Dialog, Menu, MenuItem, Progress, Skeleton, Toaster, Tooltip,
};
use rustify_ui::{provide_toasts, ToastOptions, ToastTone};

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

/// A spinner is a named, motion-aware loading indicator.
pub fn spinner_demo() -> impl IntoView {
    let busy = RwSignal::new(true);
    view! {
        <div class="space-y-4">
            <div class="flex h-8 items-center gap-3">
                <Show when=move || busy.get() fallback=|| view! { <span class="text-sm">"Idle"</span> }>
                    <rustify_components::Spinner label="Loading accounts" />
                </Show>
            </div>
            <Button variant=ButtonVariant::Outline on_click=move || busy.set(!busy.get_untracked())>
                {move || if busy.get() { "Finish loading" } else { "Load again" }}
            </Button>
            <p class="text-sm">"Status: "<output data-demo-state="">{move || if busy.get() { "Loading accounts" } else { "Idle" }}</output></p>
        </div>
    }
}

/// A skeleton holds the shape of content that is not ready yet.
pub fn skeleton_demo() -> impl IntoView {
    let loaded = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <Skeleton loading=Signal::derive(move || !loaded.get()) class="max-w-md space-y-2">
                <p class="text-sm font-medium">"Accounts loaded"</p>
                <p class="text-sm">"Three real rows arrived from the application."</p>
            </Skeleton>
            <Button variant=ButtonVariant::Outline on_click=move || loaded.set(!loaded.get_untracked())>
                {move || if loaded.get() { "Show the skeleton" } else { "Load the content" }}
            </Button>
            <p class="text-sm">"Content: "<output data-demo-state="">{move || if loaded.get() { "Loaded" } else { "Loading" }}</output></p>
        </div>
    }
}

/// A tooltip opens on focus or hover and describes its real trigger.
pub fn tooltip_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <Tooltip open=open on_open_change=move |value| open.set(value)
                label="Help also appears when this button receives keyboard focus."
                trigger=leptos::html::button().r#type("button")
                    .class("inline-flex h-9 items-center rounded-md border border-border bg-background px-4 text-sm font-medium outline-none hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px]")
                    .child("Hover or focus for help")
            />
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
            return rustify_ui::Anchor::element(button.as_ref());
        }
        rustify_ui::Anchor::Centred
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

/// A modal dialog contains focus until confirmation or dismissal, and the two
/// exits carry different results.
pub fn dialog_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    let result = RwSignal::new("Not confirmed".to_string());
    view! {
        <div class="space-y-4">
            <Button on_click=move || open.set(true) expanded=Signal::derive(move || Some(open.get()))>
                "Open dialog"
            </Button>
            <Dialog open=open on_open_change=move |value| open.set(value)
                title="Confirm an action" description="Press Confirm to accept, or close the dialog to keep the draft."
                close_label="Close dialog">
                <div class="flex gap-3">
                    <Button on_click=move || { result.set("Confirmed".to_string()); open.set(false); }>"Confirm"</Button>
                    <Button variant=ButtonVariant::Outline on_click=move || { result.set("Dismissed".to_string()); open.set(false); }>"Dismiss"</Button>
                </div>
            </Dialog>
            <p class="text-sm">"Result: "<output data-demo-state="">{move || result.get()}</output></p>
        </div>
    }
}

/// An alert dialog separates a destructive action from plain cancellation.
pub fn alert_dialog_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    let outcome = RwSignal::new("Nothing deleted".to_string());
    view! {
        <div class="demo-modal-workspace min-h-96 space-y-4">
            <alert_dialog::Root open=Signal::derive(move || open.get()) on_open_change=Callback::new(move |next| {
                if next { outcome.set("Waiting for a decision".to_string()); }
                else if outcome.get_untracked() == "Waiting for a decision" { outcome.set("Kept the workspace".to_string()); }
                open.set(next);
            })>
                <alert_dialog::Trigger>"Delete a workspace"</alert_dialog::Trigger>
                <alert_dialog::Portal>
                    <alert_dialog::Overlay />
                    <alert_dialog::Content class="rounded-md border border-border bg-background shadow-none">
                        <alert_dialog::Title>"Delete this workspace?"</alert_dialog::Title>
                        <alert_dialog::Description>"The workspace's drafts leave with it; deleting cannot be undone."</alert_dialog::Description>
                        <div class="flex gap-3">
                            <alert_dialog::Cancel>"Cancel"</alert_dialog::Cancel>
                            <alert_dialog::Action on_action=Callback::new(move |_allow: rustify_ui::Cancelable<()>| {
                                outcome.set("Workspace deleted".to_string());
                            })>"Delete"</alert_dialog::Action>
                        </div>
                    </alert_dialog::Content>
                </alert_dialog::Portal>
            </alert_dialog::Root>
            <p class="text-sm">"Outcome: "<output data-demo-state="">{move || outcome.get()}</output></p>
        </div>
    }
}

/// A popover holds real controls, not just text.
pub fn popover_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    let density = RwSignal::new("comfortable".to_string());
    view! {
        <div class="space-y-4">
            <popover::Root open=Signal::derive(move || open.get()) on_open_change=Callback::new(move |next| open.set(next))>
                <popover::Trigger>"Choose a density"</popover::Trigger>
                <popover::Portal>
                    <popover::Content aria_label="Density" class="w-64 space-y-3 rounded-md border border-border bg-background p-3 shadow-none">
                        <p class="text-sm font-medium">"Interface density"</p>
                        <RadioGroupForDensity density=density />
                        <popover::Close>"Done"</popover::Close>
                        <popover::Arrow />
                    </popover::Content>
                </popover::Portal>
            </popover::Root>
            <p class="text-sm">"Density: "<output data-demo-state="">{move || density.get()}</output></p>
        </div>
    }
}

/// A hover card previews the destination of a real link.
pub fn hover_card_demo() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <rustify_components::hover_card::Root open=Signal::derive(move || open.get()) on_open_change=Callback::new(move |next| open.set(next))>
                <rustify_components::hover_card::Trigger href="#hover-card-demo-target">
                    "Hover for the theme guide"
                </rustify_components::hover_card::Trigger>
                <rustify_components::hover_card::Portal>
                    <rustify_components::hover_card::Content class="w-64 space-y-2 rounded-md border border-border bg-background p-3 shadow-none">
                        <p class="text-sm font-medium">"Theme guide"</p>
                        <p class="text-sm">"Scoped themes, boundaries and the exported codec."</p>
                        <rustify_components::hover_card::Arrow />
                    </rustify_components::hover_card::Content>
                </rustify_components::hover_card::Portal>
            </rustify_components::hover_card::Root>
            <p id="hover-card-demo-target" tabindex="-1" class="text-sm">"The link lands on this paragraph."</p>
            <p class="text-sm">"Hover card: "<output data-demo-state="">{move || if open.get() { "Open" } else { "Closed" }}</output></p>
        </div>
    }
}

/// A notification is transient; the application can observe its lifetime.
pub fn toast_demo() -> impl IntoView {
    let toasts = provide_toasts();
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap gap-3">
                <Button on_click=move || {
                    toasts.show("The example action completed.", ToastOptions {
                        duration_ms: 5000,
                        tone: ToastTone::Success,
                        ..Default::default()
                    });
                }>"Show notification"</Button>
                <Button variant=ButtonVariant::Outline on_click=move || {
                    toasts.show("The draft kept your last edit.", ToastOptions {
                        duration_ms: 5000,
                        tone: ToastTone::Warning,
                        ..Default::default()
                    });
                }>"Show warning"</Button>
            </div>
            <p class="text-sm">"Notification: "<output data-demo-state="">{move || {
                toasts.current().map(|toast| toast.message).unwrap_or_else(|| "None".to_string())
            }}</output></p>
            <Toaster test_id="toast-demo" />
        </div>
    }
}

#[component]
fn RadioGroupForDensity(density: RwSignal<String>) -> impl IntoView {
    view! {
        <RadioGroup value=density on_change=move |value| density.set(value) aria_label="Density"
            options=vec![
                RadioOption::new("comfortable", "Comfortable"),
                RadioOption::new("compact", "Compact"),
            ] />
    }
}

use rustify_components::{RadioGroup, RadioOption};
