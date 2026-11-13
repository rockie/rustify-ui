use leptos::prelude::*;
use rustify_components::typography as t;
use rustify_components::typography::{TextAlign, TextSize, TextWeight};

const SIZES: [TextSize; 5] = [
    TextSize::One,
    TextSize::Two,
    TextSize::Three,
    TextSize::Four,
    TextSize::Five,
];
const WEIGHTS: [TextWeight; 4] = [
    TextWeight::Light,
    TextWeight::Regular,
    TextWeight::Medium,
    TextWeight::Bold,
];
const ALIGNS: [TextAlign; 3] = [TextAlign::Left, TextAlign::Center, TextAlign::Right];

/// The semantic text family: every element keeps native meaning and real
/// parameters, the same sentence stays visible while they change.
pub fn typography_demo() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <t::Heading as_=t::HeadingLevel::H2 size=Signal::stored(TextSize::Five.into())>"Heading"</t::Heading>
            <t::Text>"Text carries the interface copy with the theme's own type scale."</t::Text>
            <t::Blockquote>"Blockquote marks a quoted passage."</t::Blockquote>
            <p>
                <t::Code>"let theme = ThemeDocument::standard();"</t::Code>
                " runs in every consumer, and "
                <t::Kbd>"Enter"</t::Kbd>
                " commits a draft."
            </p>
            <p>
                "Use "
                <t::Em>"emphasis"</t::Em>
                ", a short "
                <t::Quote>"quote"</t::Quote>
                " and "
                <t::Strong>"strong"</t::Strong>
                " inside one sentence."
            </p>
            <p data-demo-state-line="" class="text-sm">"Family: "
                <output data-demo-state="">"semantic set rendered from one function"</output>
            </p>
        </div>
    }
}

/// One text whose size, weight and alignment follow the declared parameters.
pub fn text_controls_demo() -> impl IntoView {
    let size = RwSignal::new(TextSize::Three);
    let weight = RwSignal::new(TextWeight::Regular);
    let align = RwSignal::new(TextAlign::Left);
    let sample = "The same text follows the size, weight and alignment you pick.";
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap items-center gap-3 text-sm">
                <label class="flex items-center gap-2">
                    "Size "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                        if let Ok(index) = event_target_value(&event).parse::<usize>() { size.set(SIZES[index]); }
                    }>
                        {(0..SIZES.len()).map(|index| view! {
                            <option value=index selected=move || size.get() as usize == SIZES.iter().position(|candidate| *candidate == size.get()).unwrap_or(0) && index == size.get() as usize>
                                {format!("Size {}", index + 1)}
                            </option>
                        }).collect_view()}
                    </select>
                </label>
                <label class="flex items-center gap-2">
                    "Weight "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                        let chosen = event_target_value(&event);
                        if let Some(weight_value) = WEIGHTS.iter().find(|candidate| format!("{candidate:?}").to_lowercase() == chosen.to_lowercase()) {
                            weight.set(*weight_value);
                        }
                    }>
                        {WEIGHTS.iter().map(|candidate| view! {
                            <option value=format!("{candidate:?}") selected=move || format!("{candidate:?}") == format!("{:?}", weight.get())>
                                {format!("{candidate:?}")}
                            </option>
                        }).collect_view()}
                    </select>
                </label>
                <label class="flex items-center gap-2">
                    "Align "
                    <select class="rounded-md border border-border bg-background px-2 py-1" on:change=move |event| {
                        let chosen = event_target_value(&event);
                        if let Some(align_value) = ALIGNS.iter().find(|candidate| format!("{candidate:?}").to_lowercase() == chosen.to_lowercase()) {
                            align.set(*align_value);
                        }
                    }>
                        {ALIGNS.iter().map(|candidate| view! {
                            <option value=format!("{candidate:?}") selected=move || format!("{candidate:?}") == format!("{:?}", align.get())>
                                {format!("{candidate:?}")}
                            </option>
                        }).collect_view()}
                    </select>
                </label>
            </div>
            <div class="rounded-md border border-border p-3">
                <t::Text as_=t::TextElement::P size=Signal::derive(move || size.get().into()) weight=Signal::derive(move || weight.get().into()) align=Signal::derive(move || align.get().into())>
                    {sample}
                </t::Text>
            </div>
            <p data-demo-state-line="" class="text-sm">"Current: "
                <output data-demo-state="">{move || format!("{:?} · {:?} · {:?}", size.get(), weight.get(), align.get())}</output>
            </p>
        </div>
    }
}
