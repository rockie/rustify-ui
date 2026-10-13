use leptos::prelude::*;
use rustify_components::{
    id, Checkbox, Label, RadioGroup, RadioOption, Select, SelectOption, Slider, Switch,
    ToggleGroup, ToggleItem,
};

/// A checkbox shows and changes one application-owned boolean.
pub fn checkbox_demo() -> impl IntoView {
    let field_id = id::next("checkbox-demo-field");
    let checked = RwSignal::new(true);
    view! {
        <div class="space-y-4">
            <div class="flex items-center gap-3">
                <Checkbox id=field_id.clone() checked=checked on_change=move |value| checked.set(value) />
                <Label control=field_id>"Include this item"</Label>
            </div>
            <p class="text-sm">"Item: "<output data-demo-state="">{move || if checked.get() { "Included" } else { "Excluded" }}</output></p>
        </div>
    }
}

/// Native radios keep one choice and skip disabled options.
pub fn radio_demo() -> impl IntoView {
    let density = RwSignal::new("comfortable".to_string());
    view! {
        <div class="space-y-4">
            <RadioGroup value=density on_change=move |value| density.set(value) aria_label="Density"
                options=vec![
                    RadioOption::new("comfortable", "Comfortable"),
                    RadioOption::new("compact", "Compact"),
                    RadioOption::new("custom", "Custom (unavailable)").disabled(),
                ] />
            <p class="text-sm">"Density: "<output data-demo-state="">{move || density.get()}</output></p>
        </div>
    }
}

/// A switch requests a change while its state stays with the application.
pub fn switch_demo() -> impl IntoView {
    let field_id = id::next("switch-demo-field");
    let enabled = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <div class="flex items-center gap-3">
                <Switch id=field_id.clone() checked=enabled on_change=move |value| enabled.set(value) />
                <Label control=field_id>"Enable notifications"</Label>
            </div>
            <p class="text-sm">"Notifications: "<output data-demo-state="">{move || if enabled.get() { "Enabled" } else { "Disabled" }}</output></p>
        </div>
    }
}

/// A select opens an SDK layer and returns the chosen value.
pub fn select_demo() -> impl IntoView {
    let field_id = id::next("select-demo-field");
    let language = RwSignal::new("rust".to_string());
    let open = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <div class="max-w-sm space-y-2">
                <Label control=field_id.clone()>"Language"</Label>
                <Select id=field_id value=language on_change=move |value| language.set(value)
                    open=open on_open_change=move |value| open.set(value) aria_label="Language"
                    options=vec![
                        SelectOption::new("rust", "Rust"),
                        SelectOption::new("javascript", "JavaScript"),
                        SelectOption::new("typescript", "TypeScript"),
                    ] />
            </div>
            <p class="text-sm">"Selected: "<output data-demo-state="">{move || language.get()}</output></p>
        </div>
    }
}

/// A slider snaps requests to the declared step before reporting them.
pub fn slider_demo() -> impl IntoView {
    let field_id = id::next("slider-demo-field");
    let amount = RwSignal::new(25.0_f64);
    view! {
        <div class="space-y-4">
            <div class="max-w-sm space-y-2">
                <Label control=field_id.clone()>"Value"</Label>
                <Slider id=field_id value=amount on_change=move |value| amount.set(value)
                    min=0.0 max=100.0 step=5.0 />
            </div>
            <p class="text-sm">"Value: "<output data-demo-state="">{move || amount.get().to_string()}</output></p>
        </div>
    }
}

/// A toolbar holds several pressed values and supports arrow-key movement.
pub fn toggle_group_demo() -> impl IntoView {
    let styles = RwSignal::new(vec!["italic".to_string()]);
    view! {
        <div class="space-y-4">
            <ToggleGroup value=styles on_change=move |value| styles.set(value) multiple=true toolbar=true
                aria_label="Text styles" class="max-w-full flex-wrap"
                items=vec![
                    ToggleItem::new("bold", "Bold"),
                    ToggleItem::new("italic", "Italic"),
                    ToggleItem::new("underline", "Underline"),
                ] />
            <p class="text-sm">"Pressed: "<output data-demo-state="">{move || styles.with(|styles| {
                if styles.is_empty() { "None".to_string() } else { styles.join(", ") }
            })}</output></p>
        </div>
    }
}
