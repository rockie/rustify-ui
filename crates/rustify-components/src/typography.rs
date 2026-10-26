//! Native text semantics with theme-backed, responsive typography.

use crate::geometry::{PropError, StyleSet};
use crate::layout::Margin;
use crate::responsive::Responsive;
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeMode};

/// A theme font size. The semantic element is selected separately.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextSize {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TextSize {
    fn number(self) -> u8 {
        self as u8 + 1
    }

    fn tracking(self) -> &'static str {
        match self {
            Self::One => "0.0025em",
            Self::Two | Self::Three => "0em",
            Self::Four => "-0.0025em",
            Self::Five => "-0.005em",
            Self::Six => "-0.00625em",
            Self::Seven => "-0.0075em",
            Self::Eight => "-0.01em",
            Self::Nine => "-0.025em",
        }
    }
}

/// A semantic text weight.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWeight {
    Light,
    Regular,
    Medium,
    Bold,
}

/// Horizontal text alignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Removes the leading space above the first or below the last line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextTrim {
    Normal,
    Start,
    End,
    Both,
}

/// Text wrapping, including the browser's pretty and balanced algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWrap {
    Wrap,
    Nowrap,
    Pretty,
    Balance,
}

/// The native element rendered by [`Text`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextElement {
    #[default]
    Span,
    Div,
    Label,
    P,
}

/// The document heading level, independent of visual size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HeadingLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

/// An inline code treatment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeVariant {
    Solid,
    #[default]
    Soft,
    Outline,
    Ghost,
}

/// A keyboard-key treatment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KbdVariant {
    #[default]
    Classic,
    Soft,
}

impl TextElement {
    fn tag(self) -> &'static str {
        match self {
            Self::Span => "span",
            Self::Div => "div",
            Self::Label => "label",
            Self::P => "p",
        }
    }
}

impl HeadingLevel {
    fn tag(self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        }
    }
}

impl CodeVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Soft => "soft",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
        }
    }
}

impl KbdVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Soft => "soft",
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum TypeKind {
    Text,
    Heading,
    Code,
    Kbd,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct TypographyStyle {
    pub size: Signal<Responsive<TextSize>>,
    pub weight: Signal<Responsive<TextWeight>>,
    pub align: Signal<Responsive<TextAlign>>,
    pub trim: Signal<Responsive<TextTrim>>,
    pub wrap: Signal<Responsive<TextWrap>>,
}

impl TypographyStyle {
    pub(crate) fn write(self, styles: &mut StyleSet, kind: TypeKind) -> Result<(), PropError> {
        let mut size = self.size.get();
        if matches!(kind, TypeKind::Heading) && size.initial.is_none() {
            size.initial = Some(TextSize::Six);
        }
        styles.set("font-size", &size, |size| {
            let token = format!("var(--rustify-font-size-{})", size.number());
            match kind {
                TypeKind::Code => format!("calc({token} * var(--rustify-code-size-adjust))"),
                TypeKind::Kbd => format!("calc({token} * 0.8)"),
                TypeKind::Text | TypeKind::Heading => token,
            }
        })?;
        if !matches!(kind, TypeKind::Kbd) {
            let property = if matches!(kind, TypeKind::Code) {
                "line-height"
            } else {
                "--rustify-text-line-height"
            };
            styles.set(property, &size, |size| {
                let prefix = match kind {
                    TypeKind::Heading => "heading-line-height",
                    _ => "line-height",
                };
                format!("var(--rustify-{prefix}-{})", size.number())
            })?;
        }
        styles.set("letter-spacing", &size, |size| {
            let code = if matches!(kind, TypeKind::Code) {
                " - 0.007em"
            } else {
                ""
            };
            format!(
                "calc(var(--rustify-letter-spacing, 0em) + {}{code})",
                size.tracking()
            )
        })?;
        styles.set("--rustify-text-tracking", &size, |size| {
            size.tracking().into()
        })?;
        styles.set("font-weight", &self.weight.get(), |weight| match weight {
            TextWeight::Light => "300".into(),
            TextWeight::Regular => "400".into(),
            TextWeight::Medium => "500".into(),
            TextWeight::Bold => "700".into(),
        })?;
        styles.set("text-align", &self.align.get(), |align| match align {
            TextAlign::Left => "left".into(),
            TextAlign::Center => "center".into(),
            TextAlign::Right => "right".into(),
        })?;
        let trim = self.trim.get();
        styles.set("--rustify-trim-before-content", &trim, |trim| {
            if matches!(trim, TextTrim::Start | TextTrim::Both) {
                "\"\""
            } else {
                "none"
            }
            .into()
        })?;
        styles.set("--rustify-trim-after-content", &trim, |trim| {
            if matches!(trim, TextTrim::End | TextTrim::Both) {
                "\"\""
            } else {
                "none"
            }
            .into()
        })?;
        write_wrap(styles, &self.wrap.get())
    }
}

pub(crate) fn write_wrap(
    styles: &mut StyleSet,
    wrap: &Responsive<TextWrap>,
) -> Result<(), PropError> {
    styles.set("white-space", wrap, |wrap| {
        if matches!(wrap, TextWrap::Nowrap) {
            "nowrap"
        } else {
            "normal"
        }
        .into()
    })?;
    styles.set("text-wrap", wrap, |wrap| match wrap {
        TextWrap::Pretty => "pretty".into(),
        TextWrap::Balance => "balance".into(),
        TextWrap::Wrap | TextWrap::Nowrap => "wrap".into(),
    })
}

fn styled_typography(
    typography: TypographyStyle,
    kind: TypeKind,
    margin: Signal<Margin>,
    color: Signal<Option<AccentColor>>,
) -> String {
    let mode = crate::component_color::use_mode();
    crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        typography.write(&mut styles, kind)?;
        margin.get().write(&mut styles)?;
        let selected = color.get();
        let active_mode = if selected.is_some() {
            mode.get()
        } else {
            ThemeMode::Light
        };
        crate::component_color::set_color(&mut styles, selected, active_mode)?;
        Ok(styles)
    })
}

fn classes(
    base: &'static str,
    geometry: String,
    class: Signal<String>,
    truncate: Signal<bool>,
) -> impl Fn() -> String + Send + Sync + Copy + 'static {
    crate::macros::reactive_merge_with(move || {
        vec![
            base.into(),
            geometry.clone(),
            if truncate.get() {
                "rustify-truncate".into()
            } else {
                String::new()
            },
            class.get(),
        ]
    })
}

/// Text that inherits its size unless a theme size is selected.
#[component]
pub fn Text(
    #[prop(optional)] as_: TextElement,
    #[prop(optional, into)] size: Signal<Responsive<TextSize>>,
    #[prop(optional, into)] weight: Signal<Responsive<TextWeight>>,
    #[prop(optional, into)] align: Signal<Responsive<TextAlign>>,
    #[prop(optional, into)] trim: Signal<Responsive<TextTrim>>,
    #[prop(optional, into)] wrap: Signal<Responsive<TextWrap>>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = styled_typography(
        TypographyStyle {
            size,
            weight,
            align,
            trim,
            wrap,
        },
        TypeKind::Text,
        margin,
        color,
    );
    leptos::html::custom(as_.tag())
        .class(classes(
            "rustify-text rustify-color",
            geometry,
            class,
            truncate,
        ))
        .attr("data-name", "Text")
        .attr("data-testid", test_id)
        .attr("data-color", move || color.get().map(AccentColor::as_str))
        .attr("data-high-contrast", move || {
            high_contrast.get().then_some("true")
        })
        .child(children())
}

/// A native heading whose visual size is independent of its level.
#[component]
pub fn Heading(
    #[prop(optional)] as_: HeadingLevel,
    #[prop(default = Signal::stored(Responsive::from(TextSize::Six)), into)] size: Signal<
        Responsive<TextSize>,
    >,
    #[prop(optional, into)] weight: Signal<Responsive<TextWeight>>,
    #[prop(optional, into)] align: Signal<Responsive<TextAlign>>,
    #[prop(optional, into)] trim: Signal<Responsive<TextTrim>>,
    #[prop(optional, into)] wrap: Signal<Responsive<TextWrap>>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = styled_typography(
        TypographyStyle {
            size,
            weight,
            align,
            trim,
            wrap,
        },
        TypeKind::Heading,
        margin,
        color,
    );
    leptos::html::custom(as_.tag())
        .class(classes(
            "rustify-heading rustify-color",
            geometry,
            class,
            truncate,
        ))
        .attr("data-name", "Heading")
        .attr("data-testid", test_id)
        .attr("data-color", move || color.get().map(AccentColor::as_str))
        .attr("data-high-contrast", move || {
            high_contrast.get().then_some("true")
        })
        .child(children())
}

/// A block quotation with a theme-colored leading rule.
#[component]
pub fn Blockquote(
    #[prop(optional, into)] size: Signal<Responsive<TextSize>>,
    #[prop(optional, into)] weight: Signal<Responsive<TextWeight>>,
    #[prop(optional, into)] wrap: Signal<Responsive<TextWrap>>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = styled_typography(
        TypographyStyle {
            size,
            weight,
            wrap,
            ..Default::default()
        },
        TypeKind::Text,
        margin,
        color,
    );
    view! {
        <blockquote class=classes("rustify-blockquote rustify-color", geometry, class, truncate)
            data-name="Blockquote" data-testid=test_id
            data-color=move || color.get().map(AccentColor::as_str)
            data-high-contrast=move || high_contrast.get().then_some("true")>
            {children()}
        </blockquote>
    }
}

/// Inline code in the theme's monospace font.
#[component]
pub fn Code(
    #[prop(optional, into)] variant: Signal<CodeVariant>,
    #[prop(optional, into)] size: Signal<Responsive<TextSize>>,
    #[prop(optional, into)] weight: Signal<Responsive<TextWeight>>,
    #[prop(optional, into)] wrap: Signal<Responsive<TextWrap>>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = styled_typography(
        TypographyStyle {
            size,
            weight,
            wrap,
            ..Default::default()
        },
        TypeKind::Code,
        margin,
        color,
    );
    view! {
        <code class=classes("rustify-code rustify-color", geometry, class, truncate)
            data-name="Code" data-testid=test_id
            data-variant=move || variant.get().as_str()
            data-color=move || color.get().map(AccentColor::as_str)
            data-high-contrast=move || high_contrast.get().then_some("true")>
            {children()}
        </code>
    }
}

/// A keyboard key, sized relative to surrounding text by default.
#[component]
pub fn Kbd(
    #[prop(optional, into)] variant: Signal<KbdVariant>,
    #[prop(optional, into)] size: Signal<Responsive<TextSize>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = styled_typography(
        TypographyStyle {
            size,
            ..Default::default()
        },
        TypeKind::Kbd,
        margin,
        Signal::default(),
    );
    let mode = crate::component_color::use_mode();
    view! {
        <kbd class=classes("rustify-kbd", geometry, class, Signal::default())
            data-name="Kbd" data-testid=test_id data-variant=move || variant.get().as_str()
            data-appearance=move || mode.get().as_str()>
            {children()}
        </kbd>
    }
}

/// Native emphasized text, inheriting the surrounding font size.
#[component]
pub fn Em(
    #[prop(optional, into)] wrap: Signal<Responsive<TextWrap>>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        write_wrap(&mut styles, &wrap.get())?;
        Ok(styles)
    });
    view! { <em class=classes("rustify-em", geometry, class, truncate) data-name="Em" data-testid=test_id>{children()}</em> }
}

/// A native inline quotation, including the browser's quotation marks.
#[component]
pub fn Quote(
    #[prop(optional, into)] wrap: Signal<Responsive<TextWrap>>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        write_wrap(&mut styles, &wrap.get())?;
        Ok(styles)
    });
    view! { <q class=classes("rustify-quote", geometry, class, truncate) data-name="Quote" data-testid=test_id>{children()}</q> }
}

/// Native strong importance, inheriting the surrounding font size.
#[component]
pub fn Strong(
    #[prop(optional, into)] wrap: Signal<Responsive<TextWrap>>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        write_wrap(&mut styles, &wrap.get())?;
        Ok(styles)
    });
    view! { <strong class=classes("rustify-strong", geometry, class, truncate) data-name="Strong" data-testid=test_id>{children()}</strong> }
}

#[cfg(all(test, feature = "ssr", not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::geometry::{provide_geometry_styles, GeometryStyles};

    fn render(view: impl FnOnce() -> AnyView) -> (String, String) {
        let styles = GeometryStyles::default();
        let owner = Owner::new();
        let html = owner.with(|| {
            rustify_ui::id::provide("typography");
            provide_geometry_styles(styles.clone());
            view().to_html()
        });
        drop(owner);
        (html, styles.css())
    }

    #[test]
    fn text_defaults_preserve_the_inherited_size_and_native_span() {
        let (html, css) = render(|| view! { <Text>"Inherited"</Text> }.into_any());
        assert!(html.starts_with("<span "), "{html}");
        assert!(html.ends_with(">Inherited</span>"), "{html}");
        assert!(!css.contains("font-size:"), "{css}");
        assert!(!css.contains("--rustify-component-color-"), "{css}");
    }

    #[test]
    fn heading_visual_size_does_not_change_its_level() {
        let (html, css) = render(|| {
            view! { <Heading as_=HeadingLevel::H3 size=Responsive::from(TextSize::Nine)>"Title"</Heading> }.into_any()
        });
        assert!(html.starts_with("<h3 "), "{html}");
        assert!(
            css.contains("font-size:var(--rustify-font-size-9)"),
            "{css}"
        );
        assert!(
            css.contains("--rustify-text-line-height:var(--rustify-heading-line-height-9)"),
            "{css}"
        );
    }

    #[test]
    fn heading_sparse_size_starts_at_six_without_mutating_the_input() {
        let size = Responsive {
            md: Some(TextSize::Nine),
            ..Default::default()
        };
        let (html, css) = render(|| view! { <Heading size=size>"Heading"</Heading> }.into_any());
        assert!(html.starts_with("<h1 "), "{html}");
        assert!(
            css.contains("font-size:var(--rustify-font-size-6)"),
            "{css}"
        );
        assert!(css.contains("min-width:1024px"), "{css}");
        assert_eq!(size.initial, None);
    }

    #[test]
    fn all_native_text_elements_and_heading_levels_are_available() {
        for (element, tag) in [
            (TextElement::Span, "span"),
            (TextElement::Div, "div"),
            (TextElement::Label, "label"),
            (TextElement::P, "p"),
        ] {
            let (html, _) = render(|| view! { <Text as_=element>"Text"</Text> }.into_any());
            assert!(html.starts_with(&format!("<{tag} ")), "{html}");
        }
        for (level, tag) in [
            (HeadingLevel::H1, "h1"),
            (HeadingLevel::H2, "h2"),
            (HeadingLevel::H3, "h3"),
            (HeadingLevel::H4, "h4"),
            (HeadingLevel::H5, "h5"),
            (HeadingLevel::H6, "h6"),
        ] {
            let (html, _) = render(|| view! { <Heading as_=level>"Heading"</Heading> }.into_any());
            assert!(html.starts_with(&format!("<{tag} ")), "{html}");
        }
    }

    #[test]
    fn every_size_consumes_resolved_theme_metrics_without_rescaling() {
        for (size, number) in [
            (TextSize::One, 1),
            (TextSize::Two, 2),
            (TextSize::Three, 3),
            (TextSize::Four, 4),
            (TextSize::Five, 5),
            (TextSize::Six, 6),
            (TextSize::Seven, 7),
            (TextSize::Eight, 8),
            (TextSize::Nine, 9),
        ] {
            let (_, css) = render(|| {
                view! {
                    <Text size=Responsive::from(size)>"Text"</Text>
                    <Heading size=Responsive::from(size)>"Heading"</Heading>
                    <Blockquote size=Responsive::from(size)>"Blockquote"</Blockquote>
                    <Code size=Responsive::from(size)>"Code"</Code>
                    <Kbd size=Responsive::from(size)>"Kbd"</Kbd>
                }
                .into_any()
            });
            for declaration in [
                format!("font-size:var(--rustify-font-size-{number})"),
                format!("--rustify-text-line-height:var(--rustify-line-height-{number})"),
                format!("--rustify-text-line-height:var(--rustify-heading-line-height-{number})"),
                format!("font-size:calc(var(--rustify-font-size-{number}) * var(--rustify-code-size-adjust))"),
                format!("font-size:calc(var(--rustify-font-size-{number}) * 0.8)"),
            ] {
                assert!(css.contains(&declaration), "{declaration}: {css}");
            }
            assert!(!css.contains("scaling"), "{css}");
        }
    }

    #[test]
    fn all_weight_alignment_and_trim_values_write_their_own_behavior() {
        for (weight, expected) in [
            (TextWeight::Light, "300"),
            (TextWeight::Regular, "400"),
            (TextWeight::Medium, "500"),
            (TextWeight::Bold, "700"),
        ] {
            let (_, css) =
                render(|| view! { <Text weight=Responsive::from(weight)>"Text"</Text> }.into_any());
            assert!(css.contains(&format!("font-weight:{expected}")), "{css}");
        }
        for (align, expected) in [
            (TextAlign::Left, "left"),
            (TextAlign::Center, "center"),
            (TextAlign::Right, "right"),
        ] {
            let (_, css) =
                render(|| view! { <Text align=Responsive::from(align)>"Text"</Text> }.into_any());
            assert!(css.contains(&format!("text-align:{expected}")), "{css}");
        }
        for (trim, before, after) in [
            (TextTrim::Normal, "none", "none"),
            (TextTrim::Start, "\"\"", "none"),
            (TextTrim::End, "none", "\"\""),
            (TextTrim::Both, "\"\"", "\"\""),
        ] {
            let (_, css) =
                render(|| view! { <Text trim=Responsive::from(trim)>"Text"</Text> }.into_any());
            assert!(
                css.contains(&format!("--rustify-trim-before-content:{before}")),
                "{css}"
            );
            assert!(
                css.contains(&format!("--rustify-trim-after-content:{after}")),
                "{css}"
            );
        }
    }

    #[test]
    fn every_wrapping_mode_resets_the_previous_mode() {
        for (wrap, whitespace, algorithm) in [
            (TextWrap::Wrap, "normal", "wrap"),
            (TextWrap::Nowrap, "nowrap", "wrap"),
            (TextWrap::Pretty, "normal", "pretty"),
            (TextWrap::Balance, "normal", "balance"),
        ] {
            let (_, css) =
                render(|| view! { <Em wrap=Responsive::from(wrap)>"Text"</Em> }.into_any());
            assert!(css.contains(&format!("white-space:{whitespace}")), "{css}");
            assert!(css.contains(&format!("text-wrap:{algorithm}")), "{css}");
        }
    }

    #[test]
    fn responsive_text_properties_keep_the_reference_breakpoints() {
        let (html, css) = render(|| {
            view! {
                <Text
                    as_=TextElement::P
                    size=Responsive { initial: Some(TextSize::Two), xs: Some(TextSize::Three), sm: Some(TextSize::Four), md: Some(TextSize::Five), lg: Some(TextSize::Six), xl: Some(TextSize::Seven) }
                    weight=Responsive { initial: Some(TextWeight::Light), sm: Some(TextWeight::Bold), ..Default::default() }
                    align=Responsive::from(TextAlign::Center)
                    trim=Responsive { initial: Some(TextTrim::Both), md: Some(TextTrim::Normal), ..Default::default() }
                    wrap=Responsive { initial: Some(TextWrap::Nowrap), xs: Some(TextWrap::Pretty), xl: Some(TextWrap::Balance), ..Default::default() }
                    truncate=true
                    class="text-right"
                    attr:title="Native title"
                >"Responsive"</Text>
            }.into_any()
        });
        assert!(html.starts_with("<p "), "{html}");
        assert!(html.contains("title=\"Native title\""), "{html}");
        assert!(!html.contains(" style="), "{html}");
        assert!(html.contains("text-right"), "{html}");
        for width in [520, 768, 1024, 1280, 1640] {
            assert!(css.contains(&format!("min-width:{width}px")), "{css}");
        }
        for declaration in [
            "font-weight:300",
            "font-weight:700",
            "text-align:center",
            "text-wrap:pretty",
            "text-wrap:balance",
            "--rustify-trim-before-content:\"\"",
            "--rustify-trim-before-content:none",
        ] {
            assert!(css.contains(declaration), "{declaration}: {css}");
        }
    }

    #[test]
    fn code_kbd_and_inline_emphasis_keep_native_elements() {
        let (html, css) = render(|| {
            view! {
                <Blockquote>"Quoted"</Blockquote>
                <Code variant=CodeVariant::Ghost size=Responsive::from(TextSize::Four)>"let value = 1;"</Code>
                <Kbd>"Enter"</Kbd>
                <Em wrap=Responsive::from(TextWrap::Pretty)>"Emphasis"</Em>
                <Quote truncate=true>"Quote"</Quote>
                <Strong>"Strong"</Strong>
            }.into_any()
        });
        for tag in ["blockquote", "code", "kbd", "em", "q", "strong"] {
            assert!(html.contains(&format!("<{tag} ")), "{html}");
            assert!(html.contains(&format!("</{tag}>")), "{html}");
        }
        assert!(html.contains("data-variant=\"ghost\""), "{html}");
        assert!(html.contains("data-variant=\"classic\""), "{html}");
        assert!(
            css.contains(
                "font-size:calc(var(--rustify-font-size-4) * var(--rustify-code-size-adjust))"
            ),
            "{css}"
        );
    }

    #[test]
    fn code_and_kbd_variants_preserve_their_declared_defaults() {
        let (html, _) = render(|| view! { <Code>"Code"</Code><Kbd>"Kbd"</Kbd> }.into_any());
        assert!(html.contains("data-variant=\"soft\""), "{html}");
        assert!(html.contains("data-variant=\"classic\""), "{html}");
        for (variant, expected) in [
            (CodeVariant::Solid, "solid"),
            (CodeVariant::Soft, "soft"),
            (CodeVariant::Outline, "outline"),
            (CodeVariant::Ghost, "ghost"),
        ] {
            let (html, _) = render(|| view! { <Code variant=variant>"Code"</Code> }.into_any());
            assert!(
                html.contains(&format!("data-variant=\"{expected}\"")),
                "{html}"
            );
        }
        let (html, css) = render(|| view! { <Kbd variant=KbdVariant::Soft>"Kbd"</Kbd> }.into_any());
        assert!(html.contains("data-variant=\"soft\""), "{html}");
        assert!(!css.contains("font-size:"), "{css}");
    }

    #[test]
    fn supported_margin_and_local_color_props_reach_each_component() {
        let margin = Margin {
            mx: Responsive::from(crate::layout::Spacing::Scale(-2)),
            mt: Responsive {
                sm: Some(crate::layout::Spacing::Css("7px".into())),
                ..Default::default()
            },
            ..Default::default()
        };
        let (html, css) = render(|| {
            view! {
            <Text color=Some(AccentColor::Blue) high_contrast=true margin=margin.clone()>"Text"</Text>
            <Heading color=Some(AccentColor::Blue) high_contrast=true margin=margin.clone()>"Heading"</Heading>
            <Blockquote color=Some(AccentColor::Blue) high_contrast=true margin=margin.clone()>"Blockquote"</Blockquote>
            <Code color=Some(AccentColor::Blue) high_contrast=true margin=margin.clone()>"Code"</Code>
            <Kbd margin=margin>"Kbd"</Kbd>
        }.into_any()
        });
        assert_eq!(html.matches("data-color=\"blue\"").count(), 4, "{html}");
        assert_eq!(
            html.matches("data-high-contrast=\"true\"").count(),
            4,
            "{html}"
        );
        assert_eq!(css.matches("margin-top:7px").count(), 5, "{css}");
        assert!(css.contains("--rustify-component-color-12:"), "{css}");
        assert!(css.contains("--rustify-space-2"), "{css}");
        assert!(!html.contains(" style="), "{html}");
    }
}
