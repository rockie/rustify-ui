//! Select skins use the public composed and convenience entries.
use leptos::prelude::*;
use rustify_components::{select as s, Responsive, Select, SelectOption};
use rustify_ui::{AccentColor, ThemeRadius};

#[component]
pub fn SelectSamples() -> impl IntoView {
    let value = RwSignal::new(Some("a".to_string()));
    let open = RwSignal::new(false);
    let variant = RwSignal::new(s::SelectContentVariant::Solid);
    let contrast = RwSignal::new(false);
    let legacy_value = RwSignal::new("b".to_string());
    let legacy_open = RwSignal::new(false);
    let empty = RwSignal::new(false);
    let entries = Signal::stored(vec![
        s::SelectEntry::new("a", "Apricot"),
        s::SelectEntry::new("b", "Banana"),
        s::SelectEntry::new("c", "Cherry").disabled(),
    ]);
    view! {
        <section aria-label="Select themes" data-testid="select-skins">
            <h2>"Select sizes and treatments"</h2>
            <div class="field-matrix">
                {s::SelectSize::ALL.iter().flat_map(|size| s::SelectTriggerVariant::ALL.iter().map(move |variant| (*size, *variant))).map(|(size,variant)| {
                    let open = RwSignal::new(false);
                    view! {
                        <label>{format!("Size {} {}", size.number(), variant.as_str())}
                            <s::Root value=value on_change={move |next: Option<String>| value.set(next)} open=open on_open_change=move |next| open.set(next) entries=entries size=size>
                                <s::Trigger variant=variant color=Some(AccentColor::Blue) test_id=format!("select-{}-{}", size.number(), variant.as_str()) />
                                <s::Content test_id=format!("select-{}-{}-list", size.number(), variant.as_str())><s::Viewport>
                                    <s::Item value="a" /><s::Item value="b" /><s::Item value="c" />
                                </s::Viewport></s::Content>
                            </s::Root>
                        </label>
                    }
                }).collect_view()}
            </div>
            <div class="field-actions">
                <button type="button" data-testid="select-skin-variant" on:click=move |_| variant.update(|value| *value = if *value == s::SelectContentVariant::Solid { s::SelectContentVariant::Soft } else { s::SelectContentVariant::Solid })>"Change list treatment"</button>
                <button type="button" data-testid="select-skin-contrast" on:click=move |_| contrast.update(|flag| *flag = !*flag)>"Toggle high contrast"</button>
                <button type="button" data-testid="select-skin-empty" on:click=move |_| empty.update(|flag| *flag = !*flag)>"Toggle empty entries"</button>
            </div>
            <div class="field-stack">
                <s::Root value=value on_change={move |next: Option<String>| value.set(next)} open=open on_open_change=move |next| open.set(next)
                    entries=entries size=Signal::stored(Responsive { initial: Some(s::SelectSize::One), md: Some(s::SelectSize::Three), ..Default::default() })>
                    <s::Trigger color=Some(AccentColor::Blue) radius=Some(ThemeRadius::Full) test_id="select-responsive" />
                    <s::Content color=Some(AccentColor::Red) variant=variant high_contrast=contrast test_id="select-responsive-list" attr:data-caller="kept">
                        <s::ScrollUpButton /><s::Viewport><s::Group><s::Label>"Seasonal fruit"</s::Label>
                            <s::Item value="a"><s::ItemText>"Apricot"</s::ItemText><span>" · Orchard"</span><s::ItemIndicator /></s::Item>
                            <s::Item value="b" /><s::Item value="c" />
                        </s::Group><s::Separator /></s::Viewport><s::ScrollDownButton />
                    </s::Content>
                </s::Root>
                <s::Root value=value on_change=|_| {} open=false on_open_change=|_| {} entries=entries>
                    {s::trigger_as(leptos::html::button().class("select-utility").attr("data-testid", "select-typed").child(view! { <s::Value /><s::Icon /> }))}
                </s::Root>
                <form id="select-skin-form" data-testid="select-skin-form">
                    <Select value=legacy_value on_change=move |next| legacy_value.set(next) open=legacy_open on_open_change=move |next| legacy_open.set(next)
                        options=Signal::derive(move || if empty.get() { Vec::new() } else { vec![SelectOption::new("a", "Apricot"), SelectOption::new("b", "Banana")] })
                        name="fruit" required=true aria_label="Favourite fruit" test_id="select-legacy" />
                    <button type="submit" data-testid="select-skin-submit">"Submit fruit"</button>
                </form>
            </div>
        </section>
    }
}
