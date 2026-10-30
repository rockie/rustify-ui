//! Themed native radio parts backed by the unstyled controlled behavior.

use crate::{primitives, Responsive};
use leptos::prelude::*;
use rustify_ui::AccentColor;
crate::choice_style::visual_types!(RadioSize, RadioVariant);

#[component]
pub fn Root(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
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
    #[prop(optional, into)] size: Signal<Responsive<RadioSize>>,
    #[prop(optional, into)] variant: Signal<RadioVariant>,
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
            "rustify-color rustify-choice rustify-radio".into(),
            geometry.clone(),
            class.get(),
        ]
    });
    view! {
        <primitives::radio::Root checked=checked on_change=on_change id=id name=name value=value form=form
            aria_label=aria_label labelled_by=labelled_by described_by=described_by disabled=disabled
            read_only=read_only invalid=invalid required=required class=Signal::derive(classes)>
            {children()}
        </primitives::radio::Root>
    }.add_any_attr((leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str()),
        leptos::attr::custom::custom_attribute("data-high-contrast", move || high_contrast.get().then_some("")),
        leptos::attr::custom::custom_attribute("data-name", "Radio")))
}
#[component]
pub fn Control(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! { <primitives::radio::Control class=format!("rustify-choice-native {class}") test_id=test_id /> }
}
/// Styles a caller-supplied native control while retaining its attributes and listeners.
pub fn control_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    primitives::radio::control_as(target).add_any_attr(leptos::tachys::html::class::class((
        "rustify-choice-native",
        true,
    )))
}
#[component]
pub fn Indicator(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! { <primitives::radio::Indicator class=format!("rustify-choice-indicator {class}")>
        {children.map_or_else(|| view! { <crate::choice_style::Mark radio=true /> }.into_any(), |children| children().into_any())}
    </primitives::radio::Indicator> }
}

/// Boolean convenience entry point using the same native parts.
#[component]
pub fn Radio(
    #[prop(into)] checked: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
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
    #[prop(optional, into)] size: Signal<Responsive<RadioSize>>,
    #[prop(optional, into)] variant: Signal<RadioVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let on_change = StoredValue::new_local(on_change);
    view! { <Root checked=checked on_change=Callback::new(move |state| on_change.with_value(|callback| callback(state)))
        id=id aria_label=aria_label labelled_by=labelled_by name=name value=value form=form required=required disabled=disabled
        read_only=read_only invalid=invalid described_by=described_by size=size variant=variant color=color high_contrast=high_contrast class=class >
        <Control test_id=test_id /><Indicator />
    </Root> }
}

/// One choice in a group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl RadioOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

pub mod group;
pub use group::RadioGroup;
