//! Themed CheckboxGroup parts using the shared native group implementation.
use crate::{primitives, Orientation, Responsive};
use leptos::prelude::*;
use rustify_ui::AccentColor;
crate::choice_style::visual_types!(CheckboxGroupSize, CheckboxGroupVariant);

#[component]
pub fn Root(
    #[prop(into)] value: Signal<Vec<String>>,
    #[prop(into)] on_change: Callback<Vec<String>>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional)] orientation: Orientation,
    #[prop(default = true)] r#loop: bool,
    #[prop(optional, into)] size: Signal<Responsive<CheckboxGroupSize>>,
    #[prop(optional, into)] variant: Signal<CheckboxGroupVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::choice_style::geometry(
        Signal::derive(move || size.get().map(|size| size.number())),
        color,
        Signal::stored(None),
    );
    let classes = crate::macros::reactive_merge_with(move || {
        vec![
            "rustify-color rustify-choice-group rustify-checkbox-group".into(),
            geometry.clone(),
            class.get(),
        ]
    });
    view! { <primitives::checkbox_group::Root value=value on_change=on_change name=name form=form aria_label=aria_label
        described_by=described_by disabled=disabled read_only=read_only invalid=invalid required=required orientation=orientation
        r#loop=r#loop class=Signal::derive(classes) test_id=test_id>{children()}</primitives::checkbox_group::Root> }
    .add_any_attr((leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str()),
        leptos::attr::custom::custom_attribute("data-high-contrast", move || high_contrast.get().then_some("")),
        leptos::attr::custom::custom_attribute("data-name", "CheckboxGroup")))
}
/// Uses and styles one caller-supplied native group input.
pub fn item_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
    attributes: impl Into<primitives::ItemAttributes>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    primitives::checkbox_group::item_as(target, attributes).add_any_attr(
        leptos::tachys::html::class::class(("rustify-choice-native", true)),
    )
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] control_class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <primitives::checkbox_group::Item value=value disabled=disabled id=id aria_label=aria_label labelled_by=labelled_by described_by=described_by
        class=format!("rustify-choice-item {class}")
        control_class=format!("rustify-choice-native {control_class}") test_id=test_id>
        <primitives::checkbox_group::Indicator class="rustify-choice-indicator"><crate::choice_style::Mark radio=false /></primitives::checkbox_group::Indicator>
        {children()}
    </primitives::checkbox_group::Item> }
}
pub use crate::checkbox::Indicator;

/// Label, stable value and native availability for a checkbox convenience item.
pub type CheckboxOption = crate::radio::RadioOption;

/// Array convenience entry point. Parts remain available for custom content.
#[component]
pub fn CheckboxGroup(
    #[prop(into)] value: Signal<Vec<String>>,
    #[prop(into)] options: Signal<Vec<CheckboxOption>>,
    on_change: impl Fn(Vec<String>) + Send + Sync + 'static,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional)] orientation: Orientation,
    #[prop(default = true)] r#loop: bool,
    #[prop(optional, into)] size: Signal<Responsive<CheckboxGroupSize>>,
    #[prop(optional, into)] variant: Signal<CheckboxGroupVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let index = Memo::new(move |_| {
        options
            .get()
            .into_iter()
            .map(|option| (option.value.clone(), option))
            .collect::<std::collections::HashMap<_, _>>()
    });
    let item_test_id = StoredValue::new(test_id.clone());
    let selected = value;
    let change = Callback::new(on_change);
    view! { <Root value=selected on_change=change name=name form=form aria_label=aria_label described_by=described_by
        disabled=disabled read_only=read_only invalid=invalid required=required orientation=orientation r#loop=r#loop
        size=size variant=variant color=color high_contrast=high_contrast class=class test_id=test_id>
        <For each=move || options.get() key=|option| option.value.clone() let:option>
            {
                let key = StoredValue::new(option.value);
                let off = Signal::derive(move || index.with(|index| index.get(&key.get_value()).is_none_or(|option| option.disabled)));
                view! { <Item value=key.get_value() disabled=off test_id=format!("{}-{}", item_test_id.get_value(), key.get_value())>
                    {move || index.with(|index| index.get(&key.get_value()).map(|option| option.label.clone()).unwrap_or_default())}
                </Item> }
            }
        </For>
    </Root> }
}
