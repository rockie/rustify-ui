//! Themed TextField parts over the shared native editing boundary.

use crate::{primitives, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeRadius};
crate::choice_style::visual_types!(TextFieldSize, TextFieldVariant);
pub use primitives::field::{SlotSide, TextKind};
#[component]
pub fn Root(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional)] kind: TextKind,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] autocomplete: String,
    #[prop(optional, into)] inputmode: String,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional_no_strip, into)] pattern: Option<String>,
    #[prop(optional_no_strip, into)] minlength: Option<u32>,
    #[prop(optional_no_strip, into)] maxlength: Option<u32>,
    #[prop(optional, into)] size: Signal<Responsive<TextFieldSize>>,
    #[prop(optional, into)] variant: Signal<TextFieldVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] input_class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let classes = crate::field_style::classes(
        Signal::derive(move || size.get().map(|size| size.number())),
        color,
        radius,
        class,
        false,
    );
    view! { <primitives::text_field::Root value=value on_change=on_change kind=kind
        id=id name=name form=form aria_label=aria_label labelled_by=labelled_by described_by=described_by placeholder=placeholder autocomplete=autocomplete inputmode=inputmode required=required disabled=disabled read_only=read_only invalid=invalid pattern=pattern minlength=minlength maxlength=maxlength class=classes input_class=input_class test_id=test_id>
        {children()}
    </primitives::text_field::Root> }.add_any_attr((leptos::attr::custom::custom_attribute("data-name", "TextField"),
        leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str())))
}
#[component]
pub fn Input(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! { <primitives::field::Input class=format!("rustify-field-input {class}") test_id=test_id /> }
}
/// Styles the actual caller-supplied input without adding another editor.
pub fn control_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    primitives::text_field::control_as(target).add_any_attr(leptos::tachys::html::class::class((
        "rustify-field-input",
        true,
    )))
}
#[component]
pub fn TextField(
    #[prop(into)] value: Signal<String>,
    on_change: impl Fn(String) + 'static,
    #[prop(optional)] kind: TextKind,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] autocomplete: String,
    #[prop(optional, into)] inputmode: String,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional_no_strip, into)] pattern: Option<String>,
    #[prop(optional_no_strip, into)] minlength: Option<u32>,
    #[prop(optional_no_strip, into)] maxlength: Option<u32>,
    #[prop(optional, into)] size: Signal<Responsive<TextFieldSize>>,
    #[prop(optional, into)] variant: Signal<TextFieldVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] input_class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let on_change = StoredValue::new_local(on_change);
    let on_change = Callback::new(move |value| on_change.with_value(|change| change(value)));
    view! { <Root value=value on_change=on_change kind=kind
        id=id name=name form=form aria_label=aria_label labelled_by=labelled_by described_by=described_by placeholder=placeholder autocomplete=autocomplete inputmode=inputmode required=required disabled=disabled read_only=read_only invalid=invalid pattern=pattern minlength=minlength maxlength=maxlength size=size variant=variant color=color radius=radius class=class input_class=input_class test_id=test_id>
        <Input />{children.map(|children| children())}
    </Root> }
}

/// Themed logical decoration. Its color is independent of the Root's color.
#[component]
pub fn Slot(
    #[prop(optional)] side: SlotSide,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] gap: Signal<Responsive<crate::Spacing>>,
    #[prop(optional, into)] px: Signal<Responsive<crate::Spacing>>,
    #[prop(optional, into)] pl: Signal<Responsive<crate::Spacing>>,
    #[prop(optional, into)] pr: Signal<Responsive<crate::Spacing>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let mode = crate::component_color::use_mode();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        crate::Gap {
            gap: gap.get(),
            ..Default::default()
        }
        .write(&mut styles)?;
        crate::Padding {
            px: px.get(),
            pl: pl.get(),
            pr: pr.get(),
            ..Default::default()
        }
        .write(&mut styles)?;
        crate::component_color::set_color(&mut styles, color.get(), mode.get())?;
        Ok(styles)
    });
    view! { <primitives::text_field::Slot side=side class=format!("rustify-color rustify-field-slot {geometry} {class}")>
        {children()}
    </primitives::text_field::Slot> }.add_any_attr(leptos::attr::custom::custom_attribute("data-color", move || color.get().map(AccentColor::as_str)))
}
