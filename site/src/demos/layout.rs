use leptos::prelude::*;
use rustify_components::layout::{
    Box, Container, Flex, Grid, Inset, InsetSide, Layout, Padding, Spacing,
};
use rustify_components::primitives::{
    checkbox, AccessibleIcon, CheckedState, DirectionProvider, VisuallyHidden,
};
use rustify_components::{Button, Checkbox, Reset, Responsive, Slider};
use rustify_ui::Direction;

fn padded(step: i8) -> Layout {
    Layout {
        padding: Padding {
            p: Spacing::Scale(step).into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

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
            <Box layout=Signal::derive(move || padded(columns.get() as i8)) class="max-w-md rounded-md border border-border">
                <p class="rounded-md bg-muted p-2 text-sm">{move || format!("A box with theme padding step {}", columns.get())}</p>
            </Box>
            <p class="text-sm">"Layout: "<output data-demo-state="">{move || format!("{} columns, 8 cells, 2 flex items, box padding step {}", columns.get(), columns.get())}</output></p>
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

/// Inset cancels the padding of its nearest layout container on the chosen
/// sides, whatever that padding is.
pub fn inset_demo() -> impl IntoView {
    let step = RwSignal::new(4_i8);
    view! {
        <div class="space-y-4">
            <label class="flex items-center gap-2 text-sm">"Container padding step "
                <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                    if let Ok(value) = event_target_value(&event).parse::<i8>() { step.set(value); }
                }>
                    {[2_i8, 4, 6].into_iter().map(|value| view! {
                        <option value=value selected=move || step.get() == value>{value}</option>
                    }).collect_view()}
                </select>
            </label>
            <Box layout=Signal::derive(move || padded(step.get())) class="max-w-md rounded-md border border-border">
                <Inset side=Signal::stored(Responsive::from(InsetSide::Top)) class="bg-muted">
                    <p class="p-3 text-sm">"This band reaches the card's top, left and right edges."</p>
                </Inset>
                <p class="mt-3 text-sm">"The text below keeps the card's padding."</p>
            </Box>
            <p class="text-sm">"Inset: "<output data-demo-state="">{move || format!("cancels padding step {} on top, left and right", step.get())}</output></p>
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

/// The same checkbox raw and themed, both requesting changes to one
/// application value. The raw side decorates the page's own native input with
/// `checkbox::control_as`; accessible icons and visually hidden text name the
/// raw control without a theme.
pub fn primitives_demo() -> impl IntoView {
    let state = RwSignal::new(CheckedState::Unchecked);
    let raw_input = leptos::html::input().attr("class", "size-4 accent-current");
    view! {
        <div class="space-y-4">
            <div class="grid max-w-xl gap-4 sm:grid-cols-2">
                <div class="space-y-2 rounded-md border border-border p-3">
                    <p class="text-sm font-medium">"Raw primitive"</p>
                    <checkbox::Root checked=state on_change=Callback::new(move |next| state.set(next)) aria_label="Raw subscribe">
                        <label class="flex items-center gap-2 text-sm">
                            {checkbox::control_as(raw_input)}
                            <AccessibleIcon><span>"✉"</span></AccessibleIcon>
                            "Subscribe"
                            <VisuallyHidden>" to release notes"</VisuallyHidden>
                        </label>
                    </checkbox::Root>
                    <p class="text-xs text-muted-foreground">"rustify_components::primitives::checkbox::{Root, control_as}"</p>
                </div>
                <div class="space-y-2 rounded-md border border-border p-3">
                    <p class="text-sm font-medium">"Themed"</p>
                    <label class="flex items-center gap-2 text-sm">
                        <Checkbox checked=Signal::derive(move || state.get() == CheckedState::Checked)
                            on_change=move |next: bool| state.set(next.into()) aria_label="Themed subscribe" />
                        "Subscribe"
                    </label>
                    <p class="text-xs text-muted-foreground">"rustify_components::Checkbox"</p>
                </div>
            </div>
            <p class="text-sm">"Value: "
                <output data-demo-state="">{move || state.get().as_str()}</output>
            </p>
        </div>
    }
}

/// A direction provider flips the controls inside it: the slider fills from
/// the right and its arrow keys follow reading order.
pub fn direction_demo() -> impl IntoView {
    let rtl = RwSignal::new(false);
    let amount = RwSignal::new(30.0_f64);
    let direction = Signal::derive(move || {
        if rtl.get() {
            Direction::RightToLeft
        } else {
            Direction::LeftToRight
        }
    });
    view! {
        <div class="space-y-4">
            <Button variant=rustify_components::ButtonVariant::Outline on_click=move || rtl.set(!rtl.get_untracked())>
                {move || if rtl.get() { "Switch to left-to-right" } else { "Switch to right-to-left" }}
            </Button>
            <DirectionProvider direction=direction>
                <div class="max-w-md space-y-3 rounded-md border border-border p-3">
                    <p class="text-sm">"Reading order and the slider below follow the direction you picked."</p>
                    <Slider value=amount on_change=move |value| amount.set(value) aria_label="Amount"
                        min=0.0 max=100.0 step=10.0 />
                </div>
            </DirectionProvider>
            <p class="text-sm">"Direction: "<output data-demo-state="">{move || format!("{} · amount {}", if rtl.get() { "rtl" } else { "ltr" }, amount.get())}</output></p>
        </div>
    }
}
