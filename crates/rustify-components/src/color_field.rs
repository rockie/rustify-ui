//! A colour the application owns, typed as hex.
//!
//! Built on the SDK's `Draft` for the same reason as the number field: `#1f`
//! is on its way to a colour without being one, and a field that put the
//! application's value back after every keystroke would never let anyone type
//! a colour at all. Beside the text is a swatch of the colour the application
//! holds. Its initial colour is an external geometry rule, and browser
//! updates change that rule through the CSS object model.

/// The colour `text` spells, as lower-case `#rrggbb`, or `#rrggbbaa` when
/// `alpha` allows an opacity.
///
/// The `#` is optional and blanks around the digits are ignored, along with a
/// byte-order mark a paste can carry. Three digits are shorthand for six and,
/// with `alpha`, four for eight, the way CSS reads them.
pub fn parse_hex(text: &str, alpha: bool) -> Option<String> {
    let text = text.trim_matches(|ch: char| ch.is_whitespace() || ch == '\u{feff}');
    let digits = text.strip_prefix('#').unwrap_or(text);
    if !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let digits = digits.to_ascii_lowercase();
    match (digits.len(), alpha) {
        (6, _) | (8, true) => Some(format!("#{digits}")),
        (3, _) | (4, true) => Some(format!(
            "#{}",
            digits.chars().flat_map(|ch| [ch, ch]).collect::<String>()
        )),
        _ => None,
    }
}

use crate::{primitives, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeRadius};
crate::choice_style::visual_types!(ColorFieldSize, ColorFieldVariant);
#[component]
pub fn Root(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional_no_strip, into)] on_preview: Option<Callback<String>>,
    #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
    #[prop(optional)] alpha: bool,
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
    #[prop(optional, into)] size: Signal<Responsive<ColorFieldSize>>,
    #[prop(optional, into)] variant: Signal<ColorFieldVariant>,
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
    view! { <primitives::color_field::Root value=value on_change=on_change alpha=alpha
        id=id name=name form=form aria_label=aria_label labelled_by=labelled_by described_by=described_by placeholder=placeholder autocomplete=autocomplete inputmode=inputmode required=required disabled=disabled read_only=read_only invalid=invalid pattern=pattern minlength=minlength maxlength=maxlength class=classes input_class=input_class test_id=test_id on_preview=on_preview on_cancel=on_cancel>
        {children()}
    </primitives::color_field::Root> }.add_any_attr((leptos::attr::custom::custom_attribute("data-name", "ColorField"),
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
pub fn Swatch(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! { <primitives::color_field::Swatch class=format!("rustify-field-swatch {class}") test_id=test_id /> }
}
#[component]
pub fn ColorField(
    #[prop(into)] value: Signal<String>,
    on_change: impl Fn(String) + Send + Sync + 'static,
    #[prop(optional_no_strip, into)] on_preview: Option<Callback<String>>,
    #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
    #[prop(optional)] alpha: bool,
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
    #[prop(optional, into)] size: Signal<Responsive<ColorFieldSize>>,
    #[prop(optional, into)] variant: Signal<ColorFieldVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] input_class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let on_change = Callback::new(on_change);
    let swatch_test = format!("{test_id}-swatch");
    view! { <Root value=value on_change=on_change alpha=alpha
        id=id name=name form=form aria_label=aria_label labelled_by=labelled_by described_by=described_by placeholder=placeholder autocomplete=autocomplete inputmode=inputmode required=required disabled=disabled read_only=read_only invalid=invalid pattern=pattern minlength=minlength maxlength=maxlength size=size variant=variant color=color radius=radius class=class input_class=input_class test_id=test_id on_preview=on_preview on_cancel=on_cancel>
        <Swatch test_id=swatch_test />
        <Input />{children.map(|children| children())}
    </Root> }
}

#[cfg(test)]
mod tests {
    use super::parse_hex;

    #[test]
    fn six_digits_are_a_colour_with_or_without_the_hash() {
        assert_eq!(parse_hex("#1F6FEB", false).as_deref(), Some("#1f6feb"));
        assert_eq!(parse_hex("1f6feb", false).as_deref(), Some("#1f6feb"));
        assert_eq!(
            parse_hex(" \u{feff}#1f6feb\n", false).as_deref(),
            Some("#1f6feb")
        );
    }

    #[test]
    fn three_digits_are_shorthand_for_six() {
        assert_eq!(parse_hex("#abc", false).as_deref(), Some("#aabbcc"));
        assert_eq!(parse_hex("F0A", false).as_deref(), Some("#ff00aa"));
    }

    #[test]
    fn an_opacity_is_a_colour_only_where_alpha_is_allowed() {
        assert_eq!(parse_hex("#1f6feb80", false), None);
        assert_eq!(parse_hex("#abcd", false), None);
        assert_eq!(parse_hex("#1f6feb80", true).as_deref(), Some("#1f6feb80"));
        assert_eq!(parse_hex("#abcd", true).as_deref(), Some("#aabbccdd"));
        // And alpha does not stop an opaque colour from being one.
        assert_eq!(parse_hex("#1f6feb", true).as_deref(), Some("#1f6feb"));
    }

    #[test]
    fn text_on_its_way_to_a_colour_is_not_one_yet() {
        for text in [
            "", "#", "#1", "#1f", "#1f6fe", "#1f6febx", "#ggg", "red", "# 1f6feb",
        ] {
            assert_eq!(parse_hex(text, true), None, "{text:?}");
            assert_eq!(parse_hex(text, false), None, "{text:?}");
        }
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    #[test]
    fn host_color_field_renders_the_controlled_value_and_external_swatch() {
        use leptos::prelude::*;

        let owner = Owner::new();
        let styles = crate::geometry::GeometryStyles::default();
        let html = owner.with(|| {
            rustify_ui::id::provide("color-test");
            crate::geometry::provide_geometry_styles(styles.clone());
            view! {
                <super::ColorField
                    value="#1a2b3c80".to_string()
                    alpha=true
                    disabled=true
                    read_only=true
                    on_change=|_| {}
                />
            }
            .to_html()
        });
        assert!(html.contains("value=\"#1a2b3c80\""), "{html}");
        assert!(
            html.contains(" disabled ") || html.contains(" disabled>"),
            "{html}"
        );
        assert!(
            html.contains(" readonly ") || html.contains(" readonly>"),
            "{html}"
        );
        assert!(!html.contains(" style="), "{html}");
        assert!(
            styles.css().contains("background-color:#1a2b3c80;"),
            "{}",
            styles.css()
        );
    }
}
