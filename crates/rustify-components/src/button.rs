//! Native action buttons, with local visual choices and controlled state.

use crate::geometry::{PropError, StyleSet};
use crate::responsive::Responsive;
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeMode, ThemeRadius};

/// The seven button treatments. Every treatment keeps native button behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    Classic,
    #[default]
    Solid,
    Soft,
    Surface,
    Outline,
    GhostOffset,
    Ghost,
}

impl ButtonVariant {
    pub const ALL: &'static [Self] = &[
        Self::Classic,
        Self::Solid,
        Self::Soft,
        Self::Surface,
        Self::Outline,
        Self::GhostOffset,
        Self::Ghost,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Solid => "solid",
            Self::Soft => "soft",
            Self::Surface => "surface",
            Self::Outline => "outline",
            Self::GhostOffset => "ghost-offset",
            Self::Ghost => "ghost",
        }
    }
}

/// The four theme-scaled button sizes, with size two as the default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    One,
    #[default]
    Two,
    Three,
    Four,
}

impl ButtonSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three, Self::Four];

    pub const fn number(self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

impl From<ButtonSize> for Signal<Responsive<ButtonSize>> {
    fn from(value: ButtonSize) -> Self {
        Self::stored(value.into())
    }
}

/// The semantic pair used when no component-local brand color is selected.
/// Accent and destructive consume authored semantic tokens, not brand colors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonColorRole {
    #[default]
    Primary,
    Accent,
    Destructive,
}

impl ButtonColorRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Accent => "accent",
            Self::Destructive => "destructive",
        }
    }
}

/// Native HTML button behavior, independent of the visual variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

/// A native button whose availability belongs to the application.
///
/// Loading retains its accessible name and geometry, announces busy state,
/// and prevents requests and native default actions. Unavailable remains
/// reachable and delivers requests for the application to judge.
#[component]
pub fn Button(
    on_click: impl Fn() + 'static,
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
    #[prop(optional, into)] aria_label: String,
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
            icon_only: false,
        },
        children,
    )
}

pub(crate) struct ButtonOptions {
    pub variant: Signal<ButtonVariant>,
    pub size: Signal<Responsive<ButtonSize>>,
    pub color: Signal<Option<AccentColor>>,
    pub color_role: Signal<ButtonColorRole>,
    pub radius: Signal<Option<ThemeRadius>>,
    pub high_contrast: Signal<bool>,
    pub loading: Signal<bool>,
    pub disabled: Signal<bool>,
    pub unavailable: Signal<bool>,
    pub button_type: Signal<ButtonType>,
    pub aria_label: String,
    pub controls: Signal<String>,
    pub expanded: Signal<Option<bool>>,
    pub class: Signal<String>,
    pub test_id: String,
    pub icon_only: bool,
}

pub(crate) fn native_button(
    on_click: impl Fn() + 'static,
    options: ButtonOptions,
    children: Children,
) -> impl IntoView {
    let ButtonOptions {
        variant,
        size,
        color,
        color_role,
        radius,
        high_contrast,
        loading,
        disabled,
        unavailable,
        button_type,
        aria_label,
        controls,
        expanded,
        class,
        test_id,
        icon_only,
    } = options;
    let missing_name = icon_only && aria_label.trim().is_empty();
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let element = NodeRef::<leptos::html::Button>::new();
    let theme = rustify_ui::use_resolved_theme();
    let mode = crate::component_color::use_mode();
    let geometry = crate::geometry::style_class(move || {
        if missing_name {
            return Err(PropError {
                field: "aria_label",
                message: "IconButton requires a non-empty accessible name".into(),
            });
        }
        let mut styles = StyleSet::default();
        set_size(&mut styles, &size.get())?;
        let color = color.get();
        let radius = radius.get();
        let resolved = if radius.is_some() {
            theme.map(|theme| theme.get())
        } else {
            None
        };
        crate::component_color::set_color(
            &mut styles,
            color,
            if color.is_some() {
                mode.get()
            } else {
                ThemeMode::Light
            },
        )?;
        if let Some(radius) = radius {
            let scaling = resolved
                .as_ref()
                .map_or(1., |theme| theme.config.selected.scaling.factor());
            for (property, base) in [
                ("--rustify-button-radius-1", 3.),
                ("--rustify-button-radius-2", 4.),
                ("--rustify-button-radius-3", 6.),
                ("--rustify-button-radius-4", 8.),
            ] {
                styles.insert(
                    0,
                    property,
                    format!("{}px", base * radius.factor() * scaling),
                )?;
            }
            styles.insert(
                0,
                "--rustify-button-radius-full",
                if radius == ThemeRadius::Full {
                    "9999px"
                } else {
                    "0px"
                },
            )?;
        }
        Ok(styles)
    });
    let computed = crate::macros::reactive_merge_with(move || {
        vec![
            "rustify-color rustify-button".into(),
            if icon_only {
                "rustify-icon-button".into()
            } else {
                String::new()
            },
            geometry.clone(),
            class.get(),
        ]
    });
    view! {
        <button
            node_ref=element
            type=move || button_type.get().as_str()
            class=computed
            data-name=if icon_only { "IconButton" } else { "Button" }
            data-testid=test_id
            data-variant=move || variant.get().as_str()
            data-color=move || color.get().map(AccentColor::as_str)
            data-color-role=move || color_role.get().as_str()
            data-radius=move || radius.get().map(ThemeRadius::as_str)
            data-high-contrast=move || high_contrast.get().then_some("true")
            aria-label=aria_label
            aria-controls=move || {
                let controls = controls.get();
                (!controls.is_empty()).then_some(controls)
            }
            aria-expanded=move || expanded.get().map(|open| open.to_string())
            aria-disabled=move || unavailable.get().then_some("true")
            aria-busy=move || loading.get().then_some("true")
            disabled=move || disabled.get() || loading.get()
            prop:disabled=move || disabled.get() || loading.get()
            on:click=move |event| dispatch_action(
                disabled.get_untracked(), loading.get_untracked(),
                || {
                    event.prevent_default();
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(button) = element.get_untracked() {
                            button.set_disabled(true);
                        }
                    }
                },
                &on_click,
            )
        >
            <span class="rustify-button-content" aria-hidden=icon_only.then_some("true")>
                {children()}
            </span>
            <Show when=move || loading.get()>
                <span class="rustify-button-spinner" aria-hidden="true">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" focusable="false">
                        <circle cx="12" cy="12" r="9" opacity="0.25" />
                        <path d="M21 12a9 9 0 0 0-9-9" />
                    </svg>
                </span>
            </Show>
        </button>
    }
}

fn dispatch_action(
    disabled: bool,
    loading: bool,
    prevent_default: impl FnOnce(),
    on_click: impl FnOnce(),
) {
    if disabled || loading {
        prevent_default();
    } else {
        on_click();
    }
}

fn set_size(styles: &mut StyleSet, size: &Responsive<ButtonSize>) -> Result<(), PropError> {
    for (width, size) in size.iter() {
        let index = size.number();
        let space =
            |index: usize, fallback: f64| format!("var(--rustify-space-{index}, {fallback}px)");
        let values = match size {
            ButtonSize::One => (24, 12, 16, 2, 8., 1, 4., 2, 8., 4., 4.),
            ButtonSize::Two => (32, 14, 20, 3, 12., 2, 8., 2, 8., 4., 6.),
            ButtonSize::Three => (40, 16, 24, 4, 16., 3, 12., 3, 12., 6., 8.),
            ButtonSize::Four => (48, 18, 26, 5, 24., 3, 12., 4, 16., 8., 12.),
        };
        let (
            height,
            font,
            line,
            padding,
            padding_px,
            gap,
            gap_px,
            ghost_x,
            ghost_x_px,
            ghost_y_px,
            icon_ghost_px,
        ) = values;
        for (property, value) in [
            ("--rustify-button-height", format!("var(--rustify-control-height-{index}, {height}px)")),
            ("--rustify-button-font-size", format!("var(--rustify-font-size-{index}, {font}px)")),
            ("--rustify-button-line-height", format!("var(--rustify-line-height-{index}, {line}px)")),
            ("--rustify-button-padding-x", space(padding, padding_px)),
            ("--rustify-button-gap", space(gap, gap_px)),
            (
                "--rustify-button-ghost-gap",
                if index <= 2 { space(1, 4.) } else { space(2, 8.) },
            ),
            ("--rustify-button-ghost-x", space(ghost_x, ghost_x_px)),
            ("--rustify-button-ghost-y", format!("calc(var(--rustify-space-1, 4px) * {})", ghost_y_px / 4.)),
            ("--rustify-button-icon-ghost", format!("calc(var(--rustify-space-1, 4px) * {})", icon_ghost_px / 4.)),
            ("--rustify-button-radius", format!("max(var(--rustify-button-radius-{index}, var(--rustify-radius-{index}, var(--radius, 6px))), var(--rustify-button-radius-full, var(--rustify-radius-full, 0px)))")),
        ] { styles.insert(width, property, value)?; }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn disabled_and_loading_block_requests_and_native_default_actions() {
        for (disabled, loading) in [(true, false), (false, true), (true, true)] {
            let prevented = Cell::new(false);
            let requested = Cell::new(false);
            dispatch_action(
                disabled,
                loading,
                || prevented.set(true),
                || requested.set(true),
            );
            assert!(prevented.get(), "disabled={disabled}, loading={loading}");
            assert!(!requested.get(), "disabled={disabled}, loading={loading}");
        }
    }

    #[test]
    fn an_enabled_request_runs_once_without_cancelling_native_behavior() {
        let requests = Cell::new(0);
        dispatch_action(
            false,
            false,
            || panic!("enabled native action was cancelled"),
            || requests.set(requests.get() + 1),
        );
        assert_eq!(requests.get(), 1);
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    mod ssr {
        use super::*;

        #[test]
        fn loading_keeps_the_name_and_forces_native_disabled() {
            let owner = Owner::new();
            let html = owner.with(|| view! {
                <Button on_click=|| {} loading=true disabled=false attr:disabled=false r#type=ButtonType::Submit>"Save"</Button>
            }.to_html());
            assert!(html.contains("aria-busy=\"true\""), "{html}");
            assert!(html.contains(" disabled"), "{html}");
            assert!(html.contains("type=\"submit\""), "{html}");
            assert!(html.contains(">Save</span>"), "{html}");
            assert!(html.contains("rustify-button-spinner"), "{html}");
            assert!(html.contains("aria-hidden=\"true\""), "{html}");
            assert!(!html.contains("role=\"status\""), "{html}");
        }

        #[test]
        fn unavailable_stays_enabled_and_retains_aria_wiring() {
            let owner = Owner::new();
            let html = owner.with(|| view! {
                <Button on_click=|| {} unavailable=true aria_label="Save" controls="panel" expanded=Some(false)>"Save"</Button>
            }.to_html());
            assert!(html.contains("aria-disabled=\"true\""), "{html}");
            assert!(html.contains("aria-expanded=\"false\""), "{html}");
            assert!(html.contains("aria-controls=\"panel\""), "{html}");
            assert!(html.contains("aria-label=\"Save\""), "{html}");
            assert!(!html.contains(" disabled"), "{html}");
        }

        #[test]
        fn defaults_and_native_attributes_keep_their_own_semantics() {
            let owner = Owner::new();
            let html = owner.with(|| {
                view! {
                    <Button on_click=|| {} attr:name="operation" attr:value="save">"Save"</Button>
                }
                .to_html()
            });
            assert!(html.contains("type=\"button\""), "{html}");
            assert!(html.contains("data-variant=\"solid\""), "{html}");
            assert!(html.contains("name=\"operation\""), "{html}");
            assert!(html.contains("value=\"save\""), "{html}");
            for attribute in [
                "aria-label=",
                "aria-controls=",
                "aria-expanded=",
                "aria-busy=",
                "data-color=",
                "data-radius=",
            ] {
                assert!(!html.contains(attribute), "unexpected {attribute}: {html}");
            }
        }

        #[test]
        fn responsive_sizes_and_local_radius_are_collected_as_external_css() {
            let owner = Owner::new();
            let styles = crate::GeometryStyles::default();
            let html = owner.with(|| {
                crate::provide_geometry_styles(styles.clone());
                view! {
                    <Button on_click=|| {} size=Responsive { initial: Some(ButtonSize::One), md: Some(ButtonSize::Four), ..Default::default() } radius=ThemeRadius::Full class="h-20 px-6">"Save"</Button>
                }.to_html()
            });
            let css = styles.css();
            assert!(css.contains("--rustify-control-height-1"), "{css}");
            assert!(css.contains("@media (min-width:1024px)"), "{css}");
            assert!(css.contains("--rustify-control-height-4"), "{css}");
            assert!(css.contains("--rustify-button-radius-full:9999px"), "{css}");
            assert!(html.contains("h-20 px-6"), "{html}");
            assert!(!html.contains("style="), "{html}");
        }

        #[test]
        fn each_visual_and_native_type_is_a_public_parameter() {
            let owner = Owner::new();
            owner.with(|| {
                for variant in ButtonVariant::ALL {
                    for size in ButtonSize::ALL {
                        for button_type in [ButtonType::Button, ButtonType::Submit, ButtonType::Reset] {
                            let html = view! { <Button on_click=|| {} variant=*variant size=*size r#type=button_type>"Action"</Button> }.to_html();
                            assert!(html.contains(&format!("data-variant=\"{}\"", variant.as_str())), "{html}");
                            assert!(html.contains(&format!("type=\"{}\"", button_type.as_str())), "{html}");
                        }
                    }
                }
            });
        }

        #[test]
        fn all_brand_colors_use_the_current_mode_without_mutating_the_scope() {
            use rustify_ui::theme::{format_color, ColorFormat};
            use rustify_ui::{ResolveContext, ResolvedPalette, Theme, ThemeDocument};
            use std::sync::Arc;

            for mode in [ThemeMode::Light, ThemeMode::Dark] {
                let owner = Owner::new();
                let document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
                let resolved = Arc::new(
                    rustify_ui::resolve(&document, mode, &ResolveContext::default()).unwrap(),
                );
                let original = resolved.clone();
                owner.with(|| {
                    rustify_ui::provide_static_theme(resolved.clone());
                    for color in AccentColor::ALL {
                        let styles = crate::GeometryStyles::default();
                        crate::provide_geometry_styles(styles.clone());
                        let html = view! { <Button on_click=|| {} color=*color high_contrast=true>"Action"</Button> }.to_html();
                        let palette = ResolvedPalette::for_family(color.family(), mode).unwrap();
                        let expected = format!("--rustify-component-color-9:{};", format_color(palette.solid[8], ColorFormat::Rgb));
                        assert!(styles.css().contains(&expected), "{color:?} {mode:?}: {}", styles.css());
                        assert!(html.contains(&format!("data-color=\"{}\"", color.as_str())), "{html}");
                        assert!(html.contains("data-high-contrast=\"true\""), "{html}");
                    }
                    assert_eq!(rustify_ui::use_resolved_theme().unwrap().get_untracked(), original);
                });
            }
        }

        #[test]
        fn semantic_role_choices_leave_authored_tokens_for_css_to_consume() {
            let owner = Owner::new();
            owner.with(|| {
                for role in [
                    ButtonColorRole::Primary,
                    ButtonColorRole::Accent,
                    ButtonColorRole::Destructive,
                ] {
                    let styles = crate::GeometryStyles::default();
                    crate::provide_geometry_styles(styles.clone());
                    let html = view! { <Button on_click=|| {} color_role=role>"Action"</Button> }
                        .to_html();
                    assert!(
                        html.contains(&format!("data-color-role=\"{}\"", role.as_str())),
                        "{html}"
                    );
                    assert!(
                        !styles.css().contains("--rustify-component-color-"),
                        "{}",
                        styles.css()
                    );
                }
            });
        }

        #[test]
        fn local_radius_applies_scaling_once_and_keeps_full_unscaled() {
            use rustify_ui::{ResolveContext, Theme, ThemeDocument, ThemeScaling};
            use std::sync::Arc;

            let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
            document.config.scaling = ThemeScaling::P110;
            let resolved = Arc::new(
                rustify_ui::resolve(&document, ThemeMode::Light, &ResolveContext::default())
                    .unwrap(),
            );
            let owner = Owner::new();
            owner.with(|| {
                rustify_ui::provide_static_theme(resolved);
                for radius in ThemeRadius::ALL {
                    let styles = crate::GeometryStyles::default();
                    crate::provide_geometry_styles(styles.clone());
                    let _ =
                        view! { <Button on_click=|| {} radius=*radius>"Action"</Button> }.to_html();
                    let css = styles.css();
                    assert!(
                        css.contains(&format!(
                            "--rustify-button-radius-3:{}px",
                            6. * radius.factor() * 1.1
                        )),
                        "{css}"
                    );
                    let full = if *radius == ThemeRadius::Full {
                        9999
                    } else {
                        0
                    };
                    assert!(
                        css.contains(&format!("--rustify-button-radius-full:{full}px")),
                        "{css}"
                    );
                }
            });
        }
    }
}
