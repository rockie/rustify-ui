use leptos::prelude::*;
use rustify_components::{
    id, Button, ButtonSize, ButtonVariant, Glyph, Icon, IconButton, Label, Link, Spinner, TextField,
};
use rustify_ui::AccentColor;

fn variant_label(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Classic => "Classic",
        ButtonVariant::Solid => "Solid",
        ButtonVariant::Soft => "Soft",
        ButtonVariant::Surface => "Surface",
        ButtonVariant::Outline => "Outline",
        ButtonVariant::GhostOffset => "GhostOffset",
        ButtonVariant::Ghost => "Ghost",
    }
}

/// Buttons request changes to state owned by the application, across every
/// declared variant, size, color, contrast and state.
pub fn button_demo() -> impl IntoView {
    let count = RwSignal::new(0_u32);
    let variant = RwSignal::new(ButtonVariant::Solid);
    let size = RwSignal::new(ButtonSize::Two);
    let color = RwSignal::new(None::<AccentColor>);
    let high_contrast = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let loading = RwSignal::new(false);
    view! {
        <div class="flex flex-col gap-4">
            <div class="demo-controls border-b border-border pb-4 flex flex-wrap items-center gap-3 text-sm">
                <label class="flex items-center gap-2">"Variant "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                        let chosen = event_target_value(&event);
                        if let Some(found) = ButtonVariant::ALL.iter().find(|candidate| variant_label(**candidate) == chosen) {
                            variant.set(*found);
                        }
                    }>
                        {ButtonVariant::ALL.iter().map(|candidate| view! {
                            <option value=variant_label(*candidate) selected=move || variant_label(*candidate) == variant_label(variant.get())>
                                {variant_label(*candidate)}
                            </option>
                        }).collect_view()}
                    </select>
                </label>
                <label class="flex items-center gap-2">"Size "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                        if let Ok(index) = event_target_value(&event).parse::<usize>() {
                            if let Some(found) = ButtonSize::ALL.get(index) { size.set(*found); }
                        }
                    }>
                        {ButtonSize::ALL.iter().enumerate().map(|(index, _)| view! {
                            <option value=index selected=move || ButtonSize::ALL.iter().position(|candidate| *candidate == size.get()) == Some(index)>
                                {format!("Size {}", index + 1)}
                            </option>
                        }).collect_view()}
                    </select>
                </label>
                <label class="flex items-center gap-2">"Color "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                        let chosen = event_target_value(&event);
                        color.set(AccentColor::ALL.iter().copied().find(|candidate| candidate.as_str() == chosen));
                    }>
                        <option value="" selected=move || color.get().is_none()>"Theme accent"</option>
                        {AccentColor::ALL.iter().map(|candidate| view! {
                            <option value=candidate.as_str() selected=move || color.get() == Some(*candidate)>{candidate.as_str()}</option>
                        }).collect_view()}
                    </select>
                </label>
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=high_contrast on:change=move |event| high_contrast.set(event_target_checked(&event)) />
                    "High contrast"
                </label>
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=disabled on:change=move |event| disabled.set(event_target_checked(&event)) />
                    "Disabled"
                </label>
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=loading on:change=move |event| loading.set(event_target_checked(&event)) />
                    "Loading"
                </label>
            </div>
            <div class="flex flex-wrap items-center gap-3">
                <Button variant=Signal::derive(move || variant.get()) size=Signal::derive(move || size.get().into())
                    color=Signal::derive(move || color.get())
                    high_contrast=Signal::derive(move || high_contrast.get()) disabled=Signal::derive(move || disabled.get())
                    loading=Signal::derive(move || loading.get())
                    on_click=move || count.update(|count| *count += 1)>
                    "Increment"
                </Button>
                <Button variant=Signal::derive(move || variant.get()) size=Signal::derive(move || size.get().into())
                    color=Signal::derive(move || color.get())
                    high_contrast=Signal::derive(move || high_contrast.get()) disabled=Signal::derive(move || disabled.get())
                    on_click=move || count.set(0)>
                    "Reset"
                </Button>
                <Button disabled=true on_click=move || count.update(|count| *count += 100)>
                    "Unavailable"
                </Button>
            </div>
            <p data-demo-state-line="" class="text-sm">"Count: "<output data-demo-state="">{move || count.get()}</output></p>
            <p class="demo-help text-sm text-muted-foreground">"A disabled or loading button never fires the click; loading keeps its label and size."</p>
        </div>
    }
}

/// Icon buttons carry the same states with one glyph and an explicit name.
pub fn icon_button_demo() -> impl IntoView {
    let count = RwSignal::new(0_u32);
    let disabled = RwSignal::new(false);
    let loading = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <div class="demo-controls border-b border-border pb-4 flex flex-wrap items-center gap-3 text-sm">
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=disabled on:change=move |event| disabled.set(event_target_checked(&event)) />
                    "Disabled"
                </label>
                <label class="flex items-center gap-2">
                    <input type="checkbox" checked=loading on:change=move |event| loading.set(event_target_checked(&event)) />
                    "Loading"
                </label>
            </div>
            <div class="flex items-center gap-3">
                <IconButton aria_label="Approve the draft" disabled=Signal::derive(move || disabled.get())
                    loading=Signal::derive(move || loading.get()) on_click=move || count.update(|count| *count += 1)>
                    <Icon glyph=Glyph::Check label="" />
                </IconButton>
                <IconButton aria_label="Discard the draft" variant=ButtonVariant::Outline
                    disabled=Signal::derive(move || disabled.get()) on_click=move || count.set(0)>
                    <Icon glyph=Glyph::Close label="" />
                </IconButton>
            </div>
            <p data-demo-state-line="" class="text-sm">"Approvals: "<output data-demo-state="">{move || count.get()}</output></p>
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
            <p data-demo-state-line="" class="text-sm break-words">"Current name: "<output data-demo-state="">{move || name.get()}</output></p>
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
                <Button variant=ButtonVariant::Outline on_click=move || disabled.set(!disabled.get_untracked())>
                    "Toggle availability"
                </Button>
            </div>
            <p id=destination_id tabindex="-1">"The link points to this paragraph."</p>
            <p data-demo-state-line="" class="text-sm">"Link: "<output data-demo-state="">{move || if disabled.get() { "Unavailable" } else { "Available" }}</output></p>
        </div>
    }
}

/// Named icons are images; decorative icons can omit their label.
pub fn icon_demo() -> impl IntoView {
    let checked = RwSignal::new(true);
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap items-center gap-3">
                {move || {
                    let glyphs = [Glyph::Check, Glyph::ChevronDown, Glyph::ChevronUp, Glyph::ChevronRight, Glyph::Dot, Glyph::DividerHorizontal];
                    glyphs.iter().map(|glyph| view! { <Icon glyph=*glyph label=format!("{:?} icon", glyph) /> }).collect_view()
                }}
                <Show when=move || checked.get() fallback=|| view! { <Icon glyph=Glyph::Close label="Not selected" /> }>
                    <Icon glyph=Glyph::Check label="Selected" />
                </Show>
                <Button variant=ButtonVariant::Outline on_click=move || checked.set(!checked.get_untracked())>
                    "Change icon"
                </Button>
            </div>
            <p data-demo-state-line="" class="text-sm">"Last icon: "<output data-demo-state="">{move || if checked.get() { "Check" } else { "Close" }}</output></p>
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
            </div>
            <p data-demo-state-line="" class="text-sm">"Spinner: "<output data-demo-state="">{move || if loading.get() { "Loading" } else { "Ready" }}</output></p>
            <Button variant=ButtonVariant::Outline on_click=move || loading.set(!loading.get_untracked())>
                "Toggle loading"
            </Button>
        </div>
    }
}
