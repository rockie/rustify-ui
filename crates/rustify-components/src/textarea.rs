//! Themed TextArea parts over the shared native editing boundary.

use crate::{primitives, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeRadius};
crate::choice_style::visual_types!(TextAreaSize, TextAreaVariant);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAreaResize {
    #[default]
    None,
    Vertical,
    Horizontal,
    Both,
}
impl TextAreaResize {
    pub const ALL: &'static [Self] = &[Self::None, Self::Vertical, Self::Horizontal, Self::Both];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Both => "both",
        }
    }
}
impl From<TextAreaResize> for Signal<Responsive<TextAreaResize>> {
    fn from(value: TextAreaResize) -> Self {
        Self::stored(value.into())
    }
}
#[component]
pub fn TextArea(
    #[prop(into)] value: Signal<String>,
    on_change: impl Fn(String) + 'static,
    #[prop(optional_no_strip, into)] rows: Option<u32>,
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
    #[prop(optional, into)] size: Signal<Responsive<TextAreaSize>>,
    #[prop(optional, into)] variant: Signal<TextAreaVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] resize: Signal<Responsive<TextAreaResize>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] input_class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let on_change = StoredValue::new_local(on_change);
    let on_change = Callback::new(move |value| on_change.with_value(|change| change(value)));
    let class = Signal::stored(class);
    let classes = crate::field_style::classes(
        Signal::derive(move || size.get().map(|size| size.number())),
        color,
        radius,
        class,
        true,
    );
    let resize_class = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        styles.set("resize", &resize.get(), |resize| resize.as_str().into())?;
        Ok(styles)
    });
    let classes = Signal::derive(move || format!("{} {resize_class}", classes.get()));
    view! { <primitives::text_area::Root value=value on_change=on_change rows=rows
        id=id name=name form=form aria_label=aria_label labelled_by=labelled_by described_by=described_by placeholder=placeholder autocomplete=autocomplete inputmode=inputmode required=required disabled=disabled read_only=read_only invalid=invalid pattern=pattern minlength=minlength maxlength=maxlength class=classes input_class=format!("rustify-field-input {input_class}") test_id=test_id /> }.add_any_attr((leptos::attr::custom::custom_attribute("data-name", "TextArea"),
        leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str())))
}
pub use TextArea as Root;
pub use TextAreaProps as RootProps;
