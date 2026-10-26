//! A named native button whose content is decorative.

use crate::button::{
    native_button, ButtonColorRole, ButtonOptions, ButtonSize, ButtonType, ButtonVariant,
};
use crate::responsive::Responsive;
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeRadius};

/// An icon action with a required accessible name.
///
/// Its contents are decoration. Loading, disabled, unavailable, and native
/// submit/reset behavior use the same implementation as [`crate::Button`].
#[component]
pub fn IconButton(
    on_click: impl Fn() + 'static,
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] variant: Signal<ButtonVariant>,
    #[prop(optional, into)] size: Signal<Responsive<ButtonSize>>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] color_role: Signal<ButtonColorRole>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] unavailable: Signal<bool>,
    #[prop(optional, into)] r#type: Signal<ButtonType>,
    #[prop(optional, into)] controls: Signal<String>,
    #[prop(optional, into)] expanded: Signal<Option<bool>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    native_button(
        on_click,
        ButtonOptions {
            variant,
            size,
            color,
            color_role,
            radius,
            high_contrast,
            loading,
            disabled,
            unavailable,
            button_type: r#type,
            aria_label,
            controls,
            expanded,
            class,
            test_id,
            icon_only: true,
        },
        children,
    )
}

#[cfg(all(test, feature = "ssr", not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn the_button_owns_the_name_and_the_icon_is_decoration() {
        let owner = Owner::new();
        let html = owner.with(|| {
            view! {
                <IconButton on_click=|| {} aria_label="Close" loading=true size=ButtonSize::One>
                    <crate::Icon glyph=crate::Glyph::Close />
                </IconButton>
            }
            .to_html()
        });
        assert!(html.contains("data-name=\"IconButton\""), "{html}");
        assert!(html.contains("aria-label=\"Close\""), "{html}");
        assert!(
            html.contains("rustify-button-content\" aria-hidden=\"true\"")
                || html.contains("aria-hidden=\"true\" class=\"rustify-button-content\""),
            "{html}"
        );
        assert!(html.contains("aria-busy=\"true\""), "{html}");
        assert!(html.contains(" disabled"), "{html}");
        assert!(html.contains("rustify-icon-button"), "{html}");
    }

    #[test]
    fn an_empty_icon_button_name_reports_the_public_prop() {
        let owner = Owner::new();
        let styles = crate::GeometryStyles::default();
        owner.with(|| {
            crate::provide_geometry_styles(styles.clone());
            view! {
                <IconButton on_click=|| {} aria_label=" "><crate::Icon glyph=crate::Glyph::Close /></IconButton>
            }.to_html()
        });
        let diagnostics = styles.diagnostics();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].field, "aria_label");
        assert!(diagnostics[0].message.contains("non-empty accessible name"));
    }
}
