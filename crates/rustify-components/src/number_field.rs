//! A number the application owns, typed as text.
//!
//! A strictly controlled field cannot hold a number on its way to being one:
//! `-`, `1e` and `0.` are not values, an application refuses them, and the
//! field would wipe them out from under the person typing. So this one is
//! built on the SDK's `Draft`, the one sanctioned exception to controlled
//! values: while it has focus it keeps what is typed, every number typed is a
//! preview request, and Enter or leaving the field is one commit request.
//! Whatever the application then holds is what it shows.
//!
//! The bounds are announced and not enforced. An arrow key asks for one step
//! from the value the application holds, and the application clamps it - or
//! does not; it owns the value, and a field that clamped for it would be a
//! second authority on what the value may be.

/// The number `text` spells, if it spells one.
///
/// A finite number in Rust's own syntax, blanks around it ignored: `12`,
/// `-3.5`, `.5`, `1e3`. The words Rust also reads as numbers - `inf`, `NaN` -
/// are not values a person means to type.
pub fn parse_number(text: &str) -> Option<f64> {
    text.trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
}

/// How the field writes a value: the shortest text that reads back as the
/// same number, and never `-0`.
pub fn format_number(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else {
        value.to_string()
    }
}

/// How many decimals a number needs, capped where binary floating point
/// stops being able to tell.
fn decimals(value: f64) -> i32 {
    let text = format_number(value.abs());
    let digits = text
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    digits.min(12) as i32
}

/// One arrow key's worth: `from` moved by `delta`, rounded to the decimals
/// the two of them have between them, so that `0.1` and a step of `0.2` make
/// `0.3` and not `0.30000000000000004`.
pub fn stepped(from: f64, delta: f64) -> f64 {
    let scale = 10f64.powi(decimals(from).max(decimals(delta)));
    ((from + delta) * scale).round() / scale
}

use crate::{primitives, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeRadius};
crate::choice_style::visual_types!(NumberFieldSize, NumberFieldVariant);
#[component]
pub fn Root(
    #[prop(into)] value: Signal<f64>,
    #[prop(into)] on_change: Callback<f64>,
    #[prop(optional_no_strip, into)] on_preview: Option<Callback<f64>>,
    #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
    #[prop(optional_no_strip, into)] min: Option<f64>,
    #[prop(optional_no_strip, into)] max: Option<f64>,
    #[prop(optional_no_strip, into)] step: Option<f64>,
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
    #[prop(optional, into)] size: Signal<Responsive<NumberFieldSize>>,
    #[prop(optional, into)] variant: Signal<NumberFieldVariant>,
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
    view! { <primitives::number_field::Root value=value on_change=on_change min=min max=max step=step
        id=id name=name form=form aria_label=aria_label labelled_by=labelled_by described_by=described_by placeholder=placeholder autocomplete=autocomplete inputmode=inputmode required=required disabled=disabled read_only=read_only invalid=invalid pattern=pattern minlength=minlength maxlength=maxlength class=classes input_class=input_class test_id=test_id on_preview=on_preview on_cancel=on_cancel>
        {children()}
    </primitives::number_field::Root> }.add_any_attr((leptos::attr::custom::custom_attribute("data-name", "NumberField"),
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
pub fn NumberField(
    #[prop(into)] value: Signal<f64>,
    on_change: impl Fn(f64) + Send + Sync + 'static,
    #[prop(optional_no_strip, into)] on_preview: Option<Callback<f64>>,
    #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
    #[prop(optional_no_strip, into)] min: Option<f64>,
    #[prop(optional_no_strip, into)] max: Option<f64>,
    #[prop(optional_no_strip, into)] step: Option<f64>,
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
    #[prop(optional, into)] size: Signal<Responsive<NumberFieldSize>>,
    #[prop(optional, into)] variant: Signal<NumberFieldVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] input_class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let on_change = Callback::new(on_change);
    view! { <Root value=value on_change=on_change min=min max=max step=step
        id=id name=name form=form aria_label=aria_label labelled_by=labelled_by described_by=described_by placeholder=placeholder autocomplete=autocomplete inputmode=inputmode required=required disabled=disabled read_only=read_only invalid=invalid pattern=pattern minlength=minlength maxlength=maxlength size=size variant=variant color=color radius=radius class=class input_class=input_class test_id=test_id on_preview=on_preview on_cancel=on_cancel>
        <Input />{children.map(|children| children())}
    </Root> }
}

#[cfg(test)]
mod tests {
    use super::{format_number, parse_number, stepped};

    #[test]
    fn a_number_is_what_rust_reads_as_a_finite_one() {
        assert_eq!(parse_number("12"), Some(12.0));
        assert_eq!(parse_number(" -3.5 "), Some(-3.5));
        assert_eq!(parse_number(".5"), Some(0.5));
        assert_eq!(parse_number("1e3"), Some(1000.0));
    }

    #[test]
    fn text_on_its_way_to_a_number_is_not_one_yet() {
        for text in ["", "-", "1e", "1e+", "abc", "1,5", "inf", "NaN", "1e999"] {
            assert_eq!(parse_number(text), None, "{text:?}");
        }
    }

    #[test]
    fn a_value_is_written_the_shortest_way_that_reads_back() {
        assert_eq!(format_number(40.0), "40");
        assert_eq!(format_number(12.5), "12.5");
        assert_eq!(format_number(-0.0), "0");
        for value in [0.1, 1.0 / 3.0, -250.75, 1e-7] {
            assert_eq!(parse_number(&format_number(value)), Some(value));
        }
    }

    #[test]
    fn a_step_lands_on_the_decimals_its_two_parts_have() {
        assert_eq!(stepped(40.0, 5.0), 45.0);
        assert_eq!(stepped(0.1, 0.2), 0.3);
        assert_eq!(stepped(1.25, -0.1), 1.15);
        assert_eq!(stepped(100.0, 50.0), 150.0);
        // Nothing is clamped here: that is the application's.
        assert_eq!(stepped(0.0, -1.0), -1.0);
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    #[test]
    fn host_number_field_renders_the_controlled_value_and_native_flags() {
        use leptos::prelude::*;

        let owner = Owner::new();
        let html = owner.with(|| {
            view! {
                <super::NumberField
                    value=-12.5
                    min=-20.0
                    max=10.0
                    disabled=true
                    read_only=true
                    on_change=|_| {}
                />
            }
            .to_html()
        });
        assert!(html.contains("value=\"-12.5\""), "{html}");
        assert!(html.contains("aria-valuenow=\"-12.5\""), "{html}");
        assert!(
            html.contains(" disabled ") || html.contains(" disabled>"),
            "{html}"
        );
        assert!(
            html.contains(" readonly ") || html.contains(" readonly>"),
            "{html}"
        );
    }
}
