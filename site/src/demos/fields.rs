use leptos::prelude::*;
use rustify_components::{id, ColorField, Label, NumberField, TextArea, TextField};

/// A text field reports every edit to application state; invalid, read-only
/// and disabled are application decisions.
pub fn text_field_demo() -> impl IntoView {
    let field_id = id::next("text-demo-field");
    let hint_id = id::next("text-demo-hint");
    let text = RwSignal::new("Build with Rust".to_string());
    let invalid = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <div class="max-w-sm space-y-2">
                <Label control=field_id.clone()>"Project name"</Label>
                <TextField id=field_id value=text on_change=move |value| text.set(value)
                    invalid=Signal::derive(move || invalid.get()) read_only=Signal::derive(move || read_only.get())
                    disabled=Signal::derive(move || disabled.get())
                    described_by=hint_id.clone() placeholder="Your project" />
                <p id=hint_id class="text-sm text-muted-foreground">"Changes appear below as you type."</p>
            </div>
            <div class="flex flex-wrap items-center gap-4 text-sm">
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=invalid on:change=move |event| invalid.set(event_target_checked(&event)) />
                    "Invalid"
                </label>
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=read_only on:change=move |event| read_only.set(event_target_checked(&event)) />
                    "Read-only"
                </label>
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=disabled on:change=move |event| disabled.set(event_target_checked(&event)) />
                    "Disabled"
                </label>
            </div>
            <p data-demo-state-line="" class="text-sm break-words">"Current value: "
                <output data-demo-state="">{move || text.get()}</output>
            </p>
        </div>
    }
}

/// A text area preserves newlines while the application owns the value.
pub fn text_area_demo() -> impl IntoView {
    let field_id = id::next("area-demo-field");
    let text = RwSignal::new(
        "Keep the view and state together.\nUse the same function for SSR and hydration."
            .to_string(),
    );
    let read_only = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <div class="space-y-2">
                <Label control=field_id.clone()>"Notes"</Label>
                <TextArea id=field_id value=text read_only=Signal::derive(move || read_only.get())
                    on_change=move |value| text.set(value) rows=3 />
            </div>
            <label class="flex items-center gap-2 text-sm">
                <input type="checkbox" checked=read_only on:change=move |event| read_only.set(event_target_checked(&event)) />
                "Read-only"
            </label>
            <div data-demo-state-line="" class="space-y-1">
                <p>"Current text:"</p>
                <output data-demo-state="" class="block whitespace-pre-wrap break-words">{move || text.get()}</output>
            </div>
        </div>
    }
}

/// A number field keeps a draft until Enter, blur or an arrow key commits it;
/// out-of-range requests clamp.
pub fn number_field_demo() -> impl IntoView {
    let field_id = id::next("number-demo-field");
    let hint_id = id::next("number-demo-hint");
    let number = RwSignal::new(12.5_f64);
    view! {
        <div class="space-y-4">
            <div class="max-w-sm space-y-2">
                <Label control=field_id.clone()>"Amount"</Label>
                <NumberField id=field_id value=number on_change=move |value: f64| number.set(value.clamp(0.0, 100.0))
                    min=0.0 max=100.0 step=0.5 described_by=hint_id.clone() />
                <p id=hint_id class="text-sm text-muted-foreground">"Use 0–100. Arrow keys step by 0.5; Escape restores the draft; typing 500 commits as 100."</p>
            </div>
            <p data-demo-state-line="" class="text-sm">"Committed value: "<output data-demo-state="">{move || number.get().to_string()}</output></p>
        </div>
    }
}

/// A color field commits hexadecimal color and optional alpha together.
pub fn color_field_demo() -> impl IntoView {
    let field_id = id::next("color-demo-field");
    let hint_id = id::next("color-demo-hint");
    let color = RwSignal::new("#1f6feb80".to_string());
    view! {
        <div class="space-y-4">
            <div class="max-w-sm space-y-2">
                <Label control=field_id.clone()>"Color"</Label>
                <ColorField id=field_id value=color on_change=move |value| color.set(value)
                    alpha=true described_by=hint_id.clone() test_id="color-demo" />
                <p id=hint_id class="text-sm text-muted-foreground">"Enter six or eight hex digits, then press Enter or leave the field."</p>
            </div>
            <p data-demo-state-line="" class="text-sm">"Committed color: "<output data-demo-state="">{move || color.get()}</output></p>
        </div>
    }
}
