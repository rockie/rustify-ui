//! Themed RadioGroup parts using the shared native group implementation.
use crate::{primitives, Orientation, Responsive};
use leptos::prelude::*;
use rustify_ui::AccentColor;
crate::choice_style::visual_types!(RadioGroupSize, RadioGroupVariant);

#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>>,
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
    #[prop(optional, into)] size: Signal<Responsive<RadioGroupSize>>,
    #[prop(optional, into)] variant: Signal<RadioGroupVariant>,
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
            "rustify-color rustify-choice-group rustify-radio-group".into(),
            geometry.clone(),
            class.get(),
        ]
    });
    view! { <primitives::radio_group::Root value=value on_change=on_change name=name form=form aria_label=aria_label
        described_by=described_by disabled=disabled read_only=read_only invalid=invalid required=required orientation=orientation
        r#loop=r#loop class=Signal::derive(classes) test_id=test_id>{children()}</primitives::radio_group::Root> }
    .add_any_attr((leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str()),
        leptos::attr::custom::custom_attribute("data-high-contrast", move || high_contrast.get().then_some("")),
        leptos::attr::custom::custom_attribute("data-name", "RadioGroup")))
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
    primitives::radio_group::item_as(target, attributes).add_any_attr(
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
    view! { <primitives::radio_group::Item value=value disabled=disabled id=id aria_label=aria_label labelled_by=labelled_by described_by=described_by
        class=format!("rustify-choice-item {class}")
        control_class=format!("rustify-choice-native {control_class}") test_id=test_id>
        <primitives::radio_group::Indicator class="rustify-choice-indicator"><crate::choice_style::Mark radio=true /></primitives::radio_group::Indicator>
        {children()}
    </primitives::radio_group::Item> }
}
pub use crate::radio::Indicator;

/// Array convenience entry point. Parts remain available for custom content.
#[component]
pub fn RadioGroup(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] options: Signal<Vec<crate::radio::RadioOption>>,
    on_change: impl Fn(String) + Send + Sync + 'static,
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
    #[prop(optional, into)] size: Signal<Responsive<RadioGroupSize>>,
    #[prop(optional, into)] variant: Signal<RadioGroupVariant>,
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
    let selected = Signal::derive(move || Some(value.get()));
    let change = Callback::new(move |value: Option<String>| on_change(value.unwrap_or_default()));
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
