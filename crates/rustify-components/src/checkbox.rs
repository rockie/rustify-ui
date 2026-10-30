//! Themed native checkbox parts backed by the unstyled controlled behavior.

use crate::{primitives, Responsive};
use leptos::prelude::*;
pub use primitives::CheckedState;
use rustify_ui::AccentColor;
crate::choice_style::visual_types!(CheckboxSize, CheckboxVariant);

#[component]
pub fn Root(
    #[prop(into)] checked: Signal<CheckedState>,
    #[prop(into)] on_change: Callback<CheckedState>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(default = "on".into(), into)] value: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] size: Signal<Responsive<CheckboxSize>>,
    #[prop(optional, into)] variant: Signal<CheckboxVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let geometry = crate::choice_style::geometry(
        Signal::derive(move || size.get().map(|size| size.number())),
        color,
        Signal::stored(None),
    );
    let classes = crate::macros::reactive_merge_with(move || {
        vec![
            "rustify-color rustify-choice rustify-checkbox".into(),
            geometry.clone(),
            class.get(),
        ]
    });
    view! {
        <primitives::checkbox::Root checked=checked on_change=on_change id=id name=name value=value form=form
            aria_label=aria_label labelled_by=labelled_by described_by=described_by disabled=disabled
            read_only=read_only invalid=invalid required=required class=Signal::derive(classes)>
            {children()}
        </primitives::checkbox::Root>
    }.add_any_attr((leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str()),
        leptos::attr::custom::custom_attribute("data-high-contrast", move || high_contrast.get().then_some("")),
        leptos::attr::custom::custom_attribute("data-name", "Checkbox")))
}
#[component]
pub fn Control(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! { <primitives::checkbox::Control class=format!("rustify-choice-native {class}") test_id=test_id /> }
}
/// Styles a caller-supplied native control while retaining its attributes and listeners.
pub fn control_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    primitives::checkbox::control_as(target).add_any_attr(leptos::tachys::html::class::class((
        "rustify-choice-native",
        true,
    )))
}
#[component]
pub fn Indicator(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! { <primitives::checkbox::Indicator class=format!("rustify-choice-indicator {class}")>
        {children.map_or_else(|| view! { <crate::choice_style::Mark /> }.into_any(), |children| children().into_any())}
    </primitives::checkbox::Indicator> }
}
/// Boolean convenience projection; indeterminate takes precedence over checked.
#[component]
pub fn Checkbox(
    #[prop(into)] checked: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
    #[prop(optional, into)] indeterminate: Signal<bool>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] name: String,
    #[prop(default = "on".into(), into)] value: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] size: Signal<Responsive<CheckboxSize>>,
    #[prop(optional, into)] variant: Signal<CheckboxVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let on_change = StoredValue::new_local(on_change);
    view! { <Root checked=Signal::derive(move || if indeterminate.get() { CheckedState::Indeterminate } else { checked.get().into() })
        on_change=Callback::new(move |state| on_change.with_value(|callback| callback(state == CheckedState::Checked)))
        id=id aria_label=aria_label labelled_by=labelled_by name=name value=value form=form required=required disabled=disabled
        read_only=read_only invalid=invalid described_by=described_by size=size variant=variant color=color high_contrast=high_contrast class=class>
        <Control test_id=test_id /><Indicator />
    </Root> }
}
