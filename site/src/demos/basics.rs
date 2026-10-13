use leptos::prelude::*;
use rustify_components::{id, Button, ButtonVariant, Glyph, Icon, Label, Link, Spinner, TextField};

/// Buttons request changes to state owned by the application.
pub fn button_demo() -> impl IntoView {
    let count = RwSignal::new(0_u32);
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap items-center gap-3">
                <Button on_click=move || count.update(|count| *count += 1)>
                    "Increment"
                </Button>
                <Button variant=ButtonVariant::Outline on_click=move || count.set(0)>
                    "Reset"
                </Button>
                <Button disabled=true on_click=move || count.update(|count| *count += 100)>
                    "Unavailable"
                </Button>
            </div>
            <p class="text-sm">"Count: "<output data-demo-state="">{move || count.get()}</output></p>
        </div>
    }
}

/// A label points to a field through a deterministic, scope-specific ID.
pub fn label_demo() -> impl IntoView {
    let field_id = id::next("label-demo-field");
    let name = RwSignal::new("Rustify UI".to_string());
    view! {
        <div class="space-y-4">
            <div class="max-w-sm space-y-2">
                <Label control=field_id.clone()>"Project name"</Label>
                <TextField id=field_id value=name on_change=move |value| name.set(value) />
            </div>
            <p class="text-sm break-words">"Current name: "<output data-demo-state="">{move || name.get()}</output></p>
        </div>
    }
}

/// A disabled link keeps its label and stops offering a destination.
pub fn link_demo() -> impl IntoView {
    let destination_id = id::next("link-demo-destination");
    let disabled = RwSignal::new(false);
    // This anchor targets the document, so `_self` bypasses the island's local router.
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap items-center gap-3">
                <Link href=format!("#{destination_id}") disabled=disabled attr:target="_self">"Jump to the destination"</Link>
                <Button variant=ButtonVariant::Outline on_click=move || disabled.update(|disabled| *disabled = !*disabled)>
                    "Toggle availability"
                </Button>
            </div>
            <p id=destination_id tabindex="-1">"The link points to this paragraph."</p>
            <p class="text-sm">"Link: "<output data-demo-state="">{move || if disabled.get() { "Unavailable" } else { "Available" }}</output></p>
        </div>
    }
}

/// Named icons are images; decorative icons can omit their label.
pub fn icon_demo() -> impl IntoView {
    let checked = RwSignal::new(true);
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap items-center gap-3">
                <Show when=move || checked.get() fallback=|| view! { <Icon glyph=Glyph::Close label="Not selected" /> }>
                    <Icon glyph=Glyph::Check label="Selected" />
                </Show>
                <Button variant=ButtonVariant::Outline on_click=move || checked.update(|checked| *checked = !*checked)>
                    "Change icon"
                </Button>
            </div>
            <p class="text-sm">"Icon: "<output data-demo-state="">{move || if checked.get() { "Check" } else { "Close" }}</output></p>
        </div>
    }
}

/// A loading indicator uses the scope's motion preference.
pub fn loading_demo() -> impl IntoView {
    let loading = RwSignal::new(true);
    view! {
        <div class="space-y-4">
            <div class="flex h-8 items-center gap-3">
                <Show when=move || loading.get() fallback=|| ()>
                    <Spinner label="Loading example" />
                </Show>
                <output data-demo-state="">{move || if loading.get() { "Loading" } else { "Ready" }}</output>
            </div>
            <Button variant=ButtonVariant::Outline on_click=move || loading.update(|loading| *loading = !*loading)>
                "Toggle loading"
            </Button>
        </div>
    }
}
