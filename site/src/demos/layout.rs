use leptos::prelude::*;
use rustify_components::layout::{Container, Flex, Grid, Inset};
use rustify_components::{Button, Reset, TextField};

/// Box, Flex and Grid respond to the parameters the application declares.
pub fn layout_demo() -> impl IntoView {
    let columns = RwSignal::new(3_u8);
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap items-center gap-3 text-sm">
                <label class="flex items-center gap-2">"Grid columns "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                        if let Ok(value) = event_target_value(&event).parse::<u8>() { columns.set(value); }
                    }>
                        <option value=2 selected=move || columns.get() == 2>"2"</option>
                        <option value=3 selected=move || columns.get() == 3>"3"</option>
                        <option value=4 selected=move || columns.get() == 4>"4"</option>
                    </select>
                </label>
            </div>
            <Grid columns=Signal::derive(move || rustify_components::layout::GridTracks::from(columns.get()).into()) class="max-w-md gap-2">
                {(1..=8).map(|item| view! {
                    <div class="rounded-md border border-border bg-muted p-3 text-sm">{"Cell "}{item}</div>
                }).collect_view()}
            </Grid>
            <Flex class="max-w-md gap-2">
                <div class="rounded-md border border-border bg-muted p-3 text-sm">"Flex item"</div>
                <div class="rounded-md border border-border bg-muted p-3 text-sm">"Flex item"</div>
            </Flex>
            <p class="text-sm">"Layout: "<output data-demo-state="">{move || format!("{} columns, 8 cells, 2 flex items", columns.get())}</output></p>
        </div>
    }
}

/// Container and Section constrain width on a real document flow.
pub fn container_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="rounded-md border border-border p-3">
                <Container class="rounded-md bg-muted p-3">
                    <p class="text-sm">"The container caps this paragraph's measure inside the page."</p>
                </Container>
            </div>
            <rustify_components::layout::Section class="rounded-md border border-border p-3">
                <p class="text-sm">"A section adds its own vertical rhythm to the flow."</p>
            </rustify_components::layout::Section>
            <p class="text-sm">"Containers: "<output data-demo-state="">"container and section on the same page"</output></p>
        </div>
    }
}

/// Inset relates padding to the nearest container border.
pub fn inset_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="max-w-md rounded-md border border-border p-4">
                <p class="text-sm">"Card edge with its own padding."</p>
                <Inset class="rounded-md bg-muted">
                    <p class="text-sm">"The inset aligns this text with the card's own spacing rules."</p>
                </Inset>
            </div>
            <p class="text-sm">"Inset: "<output data-demo-state="">"aligned to the nearest container"</output></p>
        </div>
    }
}

/// Reset scopes a clean slate without touching the surrounding page.
pub fn reset_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="max-w-md rounded-md border border-border p-4 text-sm">
                <p>"This paragraph keeps the documentation shell's own text style."</p>
                {Reset(
                    leptos::html::div()
                        .attr("class", "mt-3 rounded-md bg-muted p-3")
                        .child(leptos::html::p().child("This paragraph starts from the reset baseline inside its own scope."))
                )}
            </div>
            <p class="text-sm">"Reset: "<output data-demo-state="">"scoped; the outer paragraph is unchanged"</output></p>
        </div>
    }
}

/// The same control raw and themed, both driven by the application's value.
pub fn primitives_demo() -> impl IntoView {
    let raw_name = RwSignal::new("Rustify UI".to_string());
    let themed_name = RwSignal::new("Rustify UI".to_string());
    let raw_count = RwSignal::new(0_u32);
    let themed_count = RwSignal::new(0_u32);
    view! {
        <div class="space-y-4">
            <div class="grid max-w-xl grid-cols-2 gap-4">
                <div class="space-y-2 rounded-md border border-border p-3">
                    <p class="text-sm font-medium">"Raw native"</p>
                    <input type="text" aria-label="Raw project name" class="h-9 w-full rounded-md border border-border bg-background px-3 text-sm"
                        prop:value=move || raw_name.get() on:input=move |event| raw_name.set(event_target_value(&event)) />
                    <button type="button" class="h-9 rounded-md border border-border px-4 text-sm" on:click=move |_| raw_count.update(|count| *count += 1)>
                        "Raw button"
                    </button>
                </div>
                <div class="space-y-2 rounded-md border border-border p-3">
                    <p class="text-sm font-medium">"Themed"</p>
                    <TextField value=themed_name on_change=move |value| themed_name.set(value) aria_label="Themed name" class="w-full" />
                    <Button on_click=move || themed_count.update(|count| *count += 1)>"Themed button"</Button>
                </div>
            </div>
            <p class="text-sm">"Values: "
                <output data-demo-state="">{move || format!("raw {}, themed {}; clicks {} / {}", raw_name.get(), themed_name.get(), raw_count.get(), themed_count.get())}</output>
            </p>
            <p class="text-sm text-muted-foreground">"Typed native decorators also wrap your own elements: select::trigger_as, tab_nav::link_as and context_menu::trigger_as."</p>
        </div>
    }
}

/// Direction handling flips one layout between LTR and RTL.
pub fn direction_demo() -> impl IntoView {
    let rtl = RwSignal::new(false);
    view! {
        <div class="space-y-4">
            <Button variant=rustify_components::ButtonVariant::Outline on_click=move || rtl.set(!rtl.get_untracked())>
                {move || if rtl.get() { "Switch to left-to-right" } else { "Switch to right-to-left" }}
            </Button>
            <div class="max-w-md rounded-md border border-border p-3" dir=move || if rtl.get() { "rtl" } else { "ltr" }>
                <p class="text-sm">{"The browser lays this line out in the direction you picked, and reading order follows it."}</p>
            </div>
            <p class="text-sm">"Direction: "<output data-demo-state="">{move || if rtl.get() { "rtl" } else { "ltr" }}</output></p>
        </div>
    }
}
