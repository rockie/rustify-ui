//! Themed fields exercise the same parts as the unstyled consumer.
use leptos::prelude::*;
use rustify_components::{
    text_field as t, ColorField, Field, Form, FormRoot, NumberField, Responsive, Spacing,
    SubmitButton, TextArea, TextField,
};
use rustify_ui::{mount, AccentColor, AppHandle, Direction, MountConfig};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
thread_local! { static HANDLE: RefCell<Option<AppHandle>> = const { RefCell::new(None) }; }

#[component]
fn Samples() -> impl IntoView {
    let resolved = rustify_ui::theme::resolve(
        &rustify_ui::ThemeDocument::standard("field-samples", "Field samples"),
        rustify_ui::ThemeMode::Light,
        &rustify_ui::theme::ResolveContext::default(),
    )
    .expect("valid sample theme");
    let resolved = RwSignal::new(std::sync::Arc::new(resolved));
    let dark = RwSignal::new(false);
    let value = RwSignal::new("Search documents".to_string());
    let number = RwSignal::new(12.5);
    let color = RwSignal::new("#336699".to_string());
    let direction = RwSignal::new(Direction::LeftToRight);
    let disabled = RwSignal::new(false);
    let readonly = RwSignal::new(false);
    let saves = RwSignal::new(0);
    let asked = RwSignal::new(Vec::<String>::new());
    let form = Form::new(&["title", "amount", "color"], move || {
        saves.update(|count| *count += 1)
    });
    view! {
        <rustify_ui::ThemeScope resolved=resolved />
        <rustify_components::primitives::DirectionProvider direction=direction>
            <main class="field-samples">
                <h1>"Native field samples"</h1>
                <div class="field-actions">
                    <button type="button" data-testid="skin-direction" on:click=move |_| direction.update(|dir| *dir = if *dir == Direction::LeftToRight { Direction::RightToLeft } else { Direction::LeftToRight })>"Change direction"</button>
                    <button type="button" data-testid="skin-disabled" on:click=move |_| disabled.update(|value| *value = !*value)>"Toggle disabled"</button>
                    <button type="button" data-testid="skin-readonly" on:click=move |_| readonly.update(|value| *value = !*value)>"Toggle read only"</button>
                    <button type="button" data-testid="skin-saved" on:click=move |_| form.submitted(Ok(()))>"Complete save"</button>
                </div>
                <button type="button" data-testid="skin-theme" on:click=move |_| {
                    dark.update(|flag| *flag = !*flag);
                    let mode = if dark.get_untracked() { rustify_ui::ThemeMode::Dark } else { rustify_ui::ThemeMode::Light };
                    resolved.set(std::sync::Arc::new(rustify_ui::theme::resolve(&rustify_ui::ThemeDocument::standard("field-samples", "Field samples"), mode, &rustify_ui::theme::ResolveContext::default()).expect("valid sample theme")));
                }>"Change colour mode"</button>
                <section class="field-matrix" aria-label="Text field sizes and variants">
                    {t::TextFieldSize::ALL.iter().flat_map(|size| t::TextFieldVariant::ALL.iter().map(move |variant| (*size, *variant))).map(|(size,variant)| view! {
                        <label>{format!("Size {} {}", size.number(), variant.as_str())}<TextField value=value on_change=move |next| value.set(next) size=size variant=variant
                            aria_label=format!("Size {} {}", size.number(), variant.as_str()) test_id=format!("skin-{}-{}", size.number(), variant.as_str()) disabled=disabled read_only=readonly />
                        </label>
                    }).collect_view()}
                </section>
                <section class="field-stack" aria-label="Composed slots">
                    <TextField value=value on_change=move |next| value.set(next) color=Some(AccentColor::Blue) test_id="skin-slot-input" aria_label="Composed field"
                        size=Signal::stored(Responsive { initial: Some(t::TextFieldSize::One), md: Some(t::TextFieldSize::Three), ..Default::default() })>
                        <t::Slot side=t::SlotSide::Start><span data-testid="skin-neutral-slot">"Find"</span></t::Slot>
                        <t::Slot side=t::SlotSide::End color=Some(AccentColor::Red)
                            gap=Signal::stored(Responsive { initial: Some(Spacing::Scale(1)), md: Some(Spacing::Scale(4)), ..Default::default() })
                            px=Signal::stored(Responsive { initial: Some(Spacing::Scale(2)), md: Some(Spacing::Scale(4)), ..Default::default() })
                            pl=Signal::stored(Responsive::from(Spacing::Scale(3)))><span data-testid="skin-color-slot">"Hint"</span><span>"Optional"</span></t::Slot>
                    </TextField>
                    <TextField value=value on_change=move |next| value.set(next) test_id="skin-utility-input" aria_label="Utility field">
                        <t::Slot class="field-utility" gap=Signal::stored(Responsive::from(Spacing::Scale(4))) px=Signal::stored(Responsive::from(Spacing::Scale(4)))>
                            <span data-testid="skin-utility-slot">"Custom"</span><span>"Spacing"</span>
                        </t::Slot>
                    </TextField>
                    <t::Root value=value on_change=move |next| value.set(next) aria_label="Custom native editor">
                        {t::control_as(leptos::html::input().class("field-utility").attr("data-testid", "skin-typed-input"))}
                    </t::Root>
                    <TextArea value="First line\nSecond line" on_change=|_| {} aria_label="Notes" test_id="skin-area" resize=rustify_components::textarea::TextAreaResize::Both />
                </section>
                <FormRoot form=form id="skin-form" test_id="skin-form" rules=Vec::new
                    on_asked=Callback::new(move |result| asked.update(|all| all.push(format!("{result:?}"))))>
                    <Field form=form field="title" label="Title" description="Use a short title" control=move |_| view! {
                        <TextField value=value on_change=move |value_| { value.set(value_); form.changed("title"); } name="title" required=true test_id="skin-title" />
                    }.into_any() />
                    <Field form=form field="amount" label="Amount" control=move |_| view! {
                        <NumberField value=number on_change=move |next| { number.set(next); form.changed("amount"); } name="amount" required=true test_id="skin-number" />
                    }.into_any() />
                    <Field form=form field="color" label="Color" control=move |_| view! {
                        <ColorField value=color on_change=move |next| { color.set(next); form.changed("color"); } name="color" required=true test_id="skin-color" />
                    }.into_any() />
                    <SubmitButton form=form rules=move || panic!("FormRoot owns rules") test_id="skin-submit">"Save changes"</SubmitButton>
                    <output data-testid="skin-saves">{move || saves.get()}</output><p data-testid="skin-asked">{move || asked.get().join("|")}</p>
                </FormRoot>
                <crate::select_samples::SelectSamples />
            </main>
        </rustify_components::primitives::DirectionProvider>
    }
}
#[wasm_bindgen]
pub fn field_samples_mount() -> Result<(), JsValue> {
    let container = document()
        .get_element_by_id("field-samples")
        .ok_or_else(|| JsValue::from_str("field container missing"))?
        .unchecked_into::<web_sys::HtmlElement>();
    let handle = mount(
        container,
        MountConfig {
            scope: "field-samples".into(),
            ..Default::default()
        },
        || view! { <Samples /> },
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    HANDLE.with(|held| *held.borrow_mut() = Some(handle));
    Ok(())
}
#[wasm_bindgen]
pub fn field_samples_dispose() {
    HANDLE.with(|held| {
        held.borrow_mut().take();
    });
}
