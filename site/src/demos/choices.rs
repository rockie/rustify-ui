use leptos::prelude::*;
use rustify_components::checkbox_group::{CheckboxGroup, CheckboxOption};
use rustify_components::{
    checkbox_cards, id, radio_cards, segmented_control, toggle, Button, Checkbox, Label,
    RadioGroup, RadioOption, RangeSlider, Select, SelectOption, Slider, Switch, ThumbLabel,
    ThumbValue, ToggleGroup, ToggleItem,
};

/// A checkbox shows and changes one application-owned boolean. The parent
/// shows the mixed state while only some of its items are included, and
/// choosing it includes or excludes them all.
pub fn checkbox_demo() -> impl IntoView {
    const ITEMS: [(&str, &str); 3] = [
        ("docs", "Docs"),
        ("dom", "DOM SDK"),
        ("components", "Components"),
    ];
    let included = RwSignal::new(vec![true, false, true]);
    let all = Signal::derive(move || included.with(|items| items.iter().all(|item| *item)));
    let some =
        Signal::derive(move || included.with(|items| items.iter().any(|item| *item)) && !all.get());
    let parent_id = id::next("checkbox-demo-all");
    view! {
        <div class="space-y-4">
            <div class="flex items-center gap-3">
                <Checkbox id=parent_id.clone() checked=all indeterminate=some
                    on_change=move |value| included.set(vec![value; ITEMS.len()]) />
                <Label control=parent_id>"Include every package"</Label>
            </div>
            <div class="ml-6 space-y-2">
                {ITEMS.iter().enumerate().map(|(index, (key, label))| {
                    let field_id = id::next(&format!("checkbox-demo-{key}"));
                    view! {
                        <div class="flex items-center gap-3">
                            <Checkbox id=field_id.clone() checked=Signal::derive(move || included.with(|items| items[index]))
                                on_change=move |value| included.update(|items| items[index] = value) />
                            <Label control=field_id>{*label}</Label>
                        </div>
                    }
                }).collect_view()}
            </div>
            <p class="text-sm">"Packages: "
                <output data-demo-state="">{move || {
                    let state = if all.get() { "all" } else if some.get() { "mixed" } else { "none" };
                    let names = included.with(|items| ITEMS.iter().zip(items).filter(|(_, on)| **on).map(|((key, _), _)| *key).collect::<Vec<_>>().join(", "));
                    format!("{state} · {}", if names.is_empty() { "none".to_string() } else { names })
                }}</output>
            </p>
        </div>
    }
}

/// A checkbox group keeps several named choices as one application value.
pub fn checkbox_group_demo() -> impl IntoView {
    let selected = RwSignal::new(vec!["deploy".to_string()]);
    view! {
        <div class="space-y-4">
            <CheckboxGroup value=Signal::derive(move || selected.get()) aria_label="Release tasks"
                options=Signal::stored(vec![
                    CheckboxOption::new("deploy", "Deploy the site"),
                    CheckboxOption::new("verify", "Verify online"),
                    CheckboxOption::new("announce", "Announce (unavailable)").disabled(),
                ])
                on_change=move |next| selected.set(next) />
            <p class="text-sm">"Selected: "
                <output data-demo-state="">{move || selected.with(|selected| if selected.is_empty() { "None".to_string() } else { selected.join(", ") })}</output>
            </p>
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

/// What a native form submission of these controls would carry, read from the
/// form's own controls rather than from application state.
fn submitted_entries(event: &leptos::ev::SubmitEvent) -> String {
    use leptos::wasm_bindgen::JsCast;
    event.prevent_default();
    let Some(form) = event
        .target()
        .and_then(|target| target.dyn_into::<leptos::web_sys::HtmlFormElement>().ok())
    else {
        return String::new();
    };
    let elements = form.elements();
    let entries = (0..elements.length())
        .filter_map(|index| {
            elements
                .item(index)?
                .dyn_into::<leptos::web_sys::HtmlInputElement>()
                .ok()
        })
        .filter(|input| {
            !input.name().is_empty()
                && !input.disabled()
                && (!matches!(input.type_().as_str(), "checkbox" | "radio") || input.checked())
        })
        .map(|input| format!("{}={}", input.name(), input.value()))
        .collect::<Vec<_>>();
    if entries.is_empty() {
        "nothing".to_string()
    } else {
        entries.join("&")
    }
}

/// The card size and variant enumerations, read from the public `ALL` lists.
fn card_controls(
    size: RwSignal<usize>,
    variant: RwSignal<usize>,
    variants: &'static [&'static str],
) -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-3 text-sm">
            <label class="flex items-center gap-2">"Size "
                <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                    if let Ok(index) = event_target_value(&event).parse::<usize>() { size.set(index); }
                }>
                    {(0..3).map(|index| view! { <option value=index selected=move || size.get() == index>{format!("Size {}", index + 1)}</option> }).collect_view()}
                </select>
            </label>
            <label class="flex items-center gap-2">"Variant "
                <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                    if let Ok(index) = event_target_value(&event).parse::<usize>() { variant.set(index); }
                }>
                    {variants.iter().enumerate().map(|(index, name)| view! { <option value=index selected=move || variant.get() == index>{*name}</option> }).collect_view()}
                </select>
            </label>
        </div>
    }
}

/// Card radios pick one option with the whole card as the target; size and
/// variant come from the public enumerations, and the form carries the value.
pub fn radio_cards_demo() -> impl IntoView {
    let plan = RwSignal::new(Some("standard".to_string()));
    let size = RwSignal::new(1_usize);
    let variant = RwSignal::new(0_usize);
    let submitted = RwSignal::new("not submitted".to_string());
    view! {
        <div class="space-y-4">
            {card_controls(size, variant, &["Surface", "Classic"])}
            <form class="space-y-3" on:submit=move |event| submitted.set(submitted_entries(&event))>
            <radio_cards::Root value=Signal::derive(move || plan.get()) aria_label="Support plan" name="plan"
                size=Signal::derive(move || radio_cards::RadioCardsSize::ALL[size.get()].into())
                variant=Signal::derive(move || radio_cards::RadioCardsVariant::ALL[variant.get()])
                on_change=move |next| plan.set(next)>
                <radio_cards::Item value="starter">"Starter"<span class="block text-sm text-muted-foreground">"Community support"</span></radio_cards::Item>
                <radio_cards::Item value="standard">"Standard"<span class="block text-sm text-muted-foreground">"Business hours"</span></radio_cards::Item>
                <radio_cards::Item value="dedicated" disabled=true>"Dedicated"<span class="block text-sm text-muted-foreground">"Currently unavailable"</span></radio_cards::Item>
            </radio_cards::Root>
            <Button r#type=rustify_components::ButtonType::Submit on_click=|| ()>"Submit the form"</Button>
            </form>
            <p class="text-sm">"Plan: "<output data-demo-state="">{move || plan.get().unwrap_or_else(|| "None".into())}</output>
                " · Form data: "<output>{move || submitted.get()}</output></p>
        </div>
    }
}

/// A switch requests a change while its state stays with the application.
pub fn switch_demo() -> impl IntoView {
    let field_id = id::next("switch-demo-field");
    let enabled = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <div class="flex items-center gap-3">
                <Switch id=field_id.clone() checked=enabled disabled=Signal::derive(move || disabled.get()) on_change=move |value| enabled.set(value) />
                <Label control=field_id>"Enable notifications"</Label>
            </div>
            <Button variant=rustify_components::ButtonVariant::Outline on_click=move || disabled.set(!disabled.get_untracked())>
                {move || if disabled.get() { "Enable the control" } else { "Disable the control" }}
            </Button>
            <p class="text-sm">"Notifications: "<output data-demo-state="">{move || if disabled.get() { "Control disabled" } else if enabled.get() { "Enabled" } else { "Disabled" }}</output></p>
        </div>
    }
}

/// A single toggle holds one pressed value for an immediate action.
pub fn toggle_demo() -> impl IntoView {
    let pinned = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <toggle::Root pressed=Signal::derive(move || pinned.get()) on_change=Callback::new(move |next| pinned.set(next)) aria_label="Pin this panel">
                {move || if pinned.get() { "Pinned" } else { "Pin this panel" }}
            </toggle::Root>
            <p class="text-sm">"Panel: "<output data-demo-state="">{move || if pinned.get() { "Pinned" } else { "Unpinned" }}</output></p>
        </div>
    }
}

/// A toolbar group holds several pressed values and supports arrow-key movement.
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

/// A segmented control selects one option; it is not a tab panel.
pub fn segmented_control_demo() -> impl IntoView {
    let period = RwSignal::new(Some("week".to_string()));
    view! {
        <div class="space-y-4">
            <segmented_control::Root value=Signal::derive(move || period.get()) aria_label="Reporting period"
                on_change=Callback::new(move |next| period.set(next))>
                <segmented_control::Item value="day">"Day"</segmented_control::Item>
                <segmented_control::Item value="week">"Week"</segmented_control::Item>
                <segmented_control::Item value="month">"Month"</segmented_control::Item>
            </segmented_control::Root>
            <p class="text-sm">"Period: "<output data-demo-state="">{move || period.get().unwrap_or_else(|| "None".into())}</output></p>
            <p class="text-sm text-muted-foreground">"A segmented control switches one value; panels belong to Tabs."</p>
        </div>
    }
}

/// Card checkboxes select several options with keyboard and form semantics;
/// size and variant come from the public enumerations.
pub fn checkbox_cards_demo() -> impl IntoView {
    let channels = RwSignal::new(vec!["mail".to_string()]);
    let size = RwSignal::new(1_usize);
    let variant = RwSignal::new(0_usize);
    let submitted = RwSignal::new("not submitted".to_string());
    view! {
        <div class="space-y-4">
            {card_controls(size, variant, &["Surface", "Classic"])}
            <form class="space-y-3" on:submit=move |event| submitted.set(submitted_entries(&event))>
            <checkbox_cards::Root value=Signal::derive(move || channels.get()) aria_label="Update channels" name="channel"
                size=Signal::derive(move || checkbox_cards::CheckboxCardsSize::ALL[size.get()].into())
                variant=Signal::derive(move || checkbox_cards::CheckboxCardsVariant::ALL[variant.get()])
                on_change=Callback::new(move |next| channels.set(next))>
                <checkbox_cards::Item value="mail">"Email updates"<span class="block text-sm text-muted-foreground">"One message per release"</span></checkbox_cards::Item>
                <checkbox_cards::Item value="news">"Product news"<span class="block text-sm text-muted-foreground">"Monthly digest"</span></checkbox_cards::Item>
                <checkbox_cards::Item value="alerts" disabled=true>"Security alerts"<span class="block text-sm text-muted-foreground">"Managed by policy"</span></checkbox_cards::Item>
            </checkbox_cards::Root>
            <Button r#type=rustify_components::ButtonType::Submit on_click=|| ()>"Submit the form"</Button>
            </form>
            <p class="text-sm">"Channels: "<output data-demo-state="">{move || channels.with(|channels| {
                if channels.is_empty() { "None".to_string() } else { channels.join(", ") }
            })}</output>" · Form data: "<output>{move || submitted.get()}</output></p>
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
                <Slider id=field_id aria_label="Value" value=amount on_change=move |value| amount.set(value)
                    min=0.0 max=100.0 step=5.0 />
            </div>
            <p class="text-sm">"Value: "<output data-demo-state="">{move || amount.get().to_string()}</output></p>
            <p class="text-sm text-muted-foreground">"Arrow keys move one step at a time; requests outside the range clamp."</p>
        </div>
    }
}

/// A range slider keeps two thumbs with their own accessible names. The
/// thumbs stay two steps apart, the application rejects ranges wider than 60
/// and keeps showing the last accepted range, and a commit is reported when a
/// drag or key press ends. The vertical, inverted slider shares the value.
pub fn range_slider_demo() -> impl IntoView {
    let values = RwSignal::new(vec![
        ThumbValue {
            id: "low".to_string(),
            value: 20.0,
        },
        ThumbValue {
            id: "high".to_string(),
            value: 60.0,
        },
    ]);
    let rejected = RwSignal::new(None::<String>);
    let committed = RwSignal::new("nothing yet".to_string());
    let describe = |values: &[ThumbValue]| format!("{} – {}", values[0].value, values[1].value);
    let request = Callback::new(move |next: Vec<ThumbValue>| {
        if next[1].value - next[0].value > 60.0 {
            rejected.set(Some(describe(&next)));
        } else {
            rejected.set(None);
            values.set(next);
        }
    });
    let commit = Callback::new(move |_: Vec<ThumbValue>| {
        committed.set(values.with_untracked(|values| describe(values)))
    });
    let thumbs = || {
        vec![
            ThumbLabel {
                id: "low".to_string(),
                label: "Low edge".to_string(),
            },
            ThumbLabel {
                id: "high".to_string(),
                label: "High edge".to_string(),
            },
        ]
    };
    view! {
        <div class="space-y-4">
            <div class="flex max-w-md items-center gap-6">
                <div class="min-w-0 flex-1">
                    <RangeSlider values=Signal::derive(move || values.get()) aria_label="Price range" thumbs=thumbs()
                        on_change=request on_commit=commit min=0.0 max=100.0 step=5.0 min_steps_between=2 />
                </div>
                <div class="h-32">
                    <RangeSlider values=Signal::derive(move || values.get()) aria_label="Price range, vertical" thumbs=thumbs()
                        on_change=request on_commit=commit min=0.0 max=100.0 step=5.0 min_steps_between=2
                        orientation=rustify_components::Orientation::Vertical inverted=true />
                </div>
            </div>
            <p class="text-sm">"Range: "
                <output data-demo-state="">{move || values.with(|values| describe(values))}</output>
                " · Committed: "<output>{move || committed.get()}</output>
            </p>
            <p class="text-sm" aria-live="polite">{move || rejected.get().map(|range| format!("Rejected {range}: the range can be at most 60 wide."))}</p>
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
