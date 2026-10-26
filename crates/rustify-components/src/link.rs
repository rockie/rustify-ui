//! A link that knows whether it points at the page you are on.
//!
//! It compares its native destination with the application's current path.
//! Without a current-path provider, a link is simply never marked current.

use crate::geometry::StyleSet;
use crate::responsive::Responsive;
use crate::typography::{TextSize, TextTrim, TextWeight, TextWrap, TypeKind, TypographyStyle};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeMode};

/// How much of the current path a link needs to match to count as current.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkMatch {
    /// The page it points at, or any page under it. A link to `/objects`
    /// stays marked while the page shows `/objects/17`.
    #[default]
    Within,
    /// That page and no other.
    Exact,
}

/// When a link draws an underline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkUnderline {
    #[default]
    Auto,
    Always,
    Hover,
    None,
}

impl LinkUnderline {
    fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Hover => "hover",
            Self::None => "none",
        }
    }
}

#[component]
pub fn Link(
    #[prop(into)] href: String,
    #[prop(optional)] match_: LinkMatch,
    /// A link with nowhere to go keeps its place in the text and loses its
    /// href: `<a>` has no disabled attribute, and a link that still navigates
    /// while looking disabled is worse than one that looks ordinary.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    #[prop(optional, into)] size: Signal<Responsive<TextSize>>,
    #[prop(optional, into)] weight: Signal<Responsive<TextWeight>>,
    #[prop(optional, into)] trim: Signal<Responsive<TextTrim>>,
    #[prop(optional, into)] wrap: Signal<Responsive<TextWrap>>,
    #[prop(optional, into)] underline: Signal<LinkUnderline>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let mode = crate::component_color::use_mode();
    let typography = TypographyStyle {
        size,
        weight,
        trim,
        wrap,
        ..Default::default()
    };
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        typography.write(&mut styles, TypeKind::Text)?;
        let selected = color.get();
        let active_mode = if selected.is_some() {
            mode.get()
        } else {
            ThemeMode::Light
        };
        crate::component_color::set_color(&mut styles, selected, active_mode)?;
        Ok(styles)
    });
    let computed = crate::macros::reactive_merge_with(move || {
        vec![
            "rustify-link rustify-text rustify-color".into(),
            geometry.clone(),
            if truncate.get() {
                "rustify-truncate".into()
            } else {
                String::new()
            },
            class.get(),
        ]
    });
    let path = crate::current_path();
    let target = href.clone();
    let current = move || {
        let path = path.as_ref()?.get();
        let matches = match match_ {
            LinkMatch::Within => crate::is_within(&path, &target),
            LinkMatch::Exact => path.trim_end_matches('/') == target.trim_end_matches('/'),
        };
        matches.then_some("page")
    };
    view! {
        <a
            class=computed
            data-name="Link"
            data-testid=test_id
            data-underline=move || underline.get().as_str()
            data-color=move || color.get().map(AccentColor::as_str)
            data-high-contrast=move || high_contrast.get().then_some("true")
            href=move || (!disabled.get()).then(|| href.clone())
            aria-current=current
            aria-disabled=move || disabled.get().then_some("true")
        >
            {children()}
        </a>
    }
}

#[cfg(all(test, feature = "ssr", not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn link_preserves_native_attributes_current_path_and_underline() {
        let owner = Owner::new();
        let html = owner.with(|| {
            crate::provide_current_path(Signal::stored("/objects/17".to_string()));
            view! {
                <Link href="/objects" underline=LinkUnderline::Always attr:target="_blank" attr:rel="noreferrer">
                    "Objects"
                </Link>
            }.to_html()
        });
        drop(owner);
        assert!(html.starts_with("<a "), "{html}");
        assert!(html.contains("href=\"/objects\""), "{html}");
        assert!(html.contains("aria-current=\"page\""), "{html}");
        assert!(html.contains("target=\"_blank\""), "{html}");
        assert!(html.contains("rel=\"noreferrer\""), "{html}");
        assert!(html.contains("data-underline=\"always\""), "{html}");
    }

    #[test]
    fn disabled_link_removes_href_but_preserves_its_name() {
        let owner = Owner::new();
        let html =
            owner.with(|| view! { <Link href="/objects" disabled=true>"Objects"</Link> }.to_html());
        drop(owner);
        assert!(!html.contains("href="), "{html}");
        assert!(html.contains("aria-disabled=\"true\""), "{html}");
        assert!(html.ends_with(">Objects</a>"), "{html}");
    }

    #[test]
    fn exact_matching_does_not_mark_a_descendant_page_current() {
        let owner = Owner::new();
        let html = owner.with(|| {
            crate::provide_current_path(Signal::stored("/objects/17".to_string()));
            view! { <Link href="/objects" match_=LinkMatch::Exact>"Objects"</Link> }.to_html()
        });
        drop(owner);
        assert!(!html.contains("aria-current="), "{html}");
    }

    #[test]
    fn link_defaults_preserve_inherited_size_and_support_each_underline() {
        for (underline, expected) in [
            (LinkUnderline::Auto, "auto"),
            (LinkUnderline::Always, "always"),
            (LinkUnderline::Hover, "hover"),
            (LinkUnderline::None, "none"),
        ] {
            let owner = Owner::new();
            let styles = crate::GeometryStyles::default();
            let html = owner.with(|| {
                crate::provide_geometry_styles(styles.clone());
                view! { <Link href="/objects" underline=underline>"Objects"</Link> }.to_html()
            });
            drop(owner);
            assert!(
                html.contains(&format!("data-underline=\"{expected}\"")),
                "{html}"
            );
            assert!(!styles.css().contains("font-size:"), "{}", styles.css());
        }
    }

    #[test]
    fn link_responsive_typography_collects_external_rules() {
        let owner = Owner::new();
        let styles = crate::GeometryStyles::default();
        let html = owner.with(|| {
            crate::provide_geometry_styles(styles.clone());
            view! {
                <Link href="/objects"
                    size=Responsive { initial: Some(TextSize::One), xl: Some(TextSize::Nine), ..Default::default() }
                    weight=Responsive::from(TextWeight::Medium)
                    trim=Responsive::from(TextTrim::End)
                    wrap=Responsive::from(TextWrap::Balance)
                    color=Some(AccentColor::Red)
                    high_contrast=true
                    truncate=true
                    class="text-xl underline"
                >"Objects"</Link>
            }.to_html()
        });
        drop(owner);
        assert!(!html.contains(" style="), "{html}");
        assert!(html.contains("rustify-color"), "{html}");
        assert!(html.contains("data-color=\"red\""), "{html}");
        assert!(html.contains("data-high-contrast=\"true\""), "{html}");
        assert!(html.contains("rustify-truncate"), "{html}");
        assert!(html.contains("text-xl underline"), "{html}");
        let css = styles.css();
        for declaration in [
            "font-size:var(--rustify-font-size-1)",
            "font-size:var(--rustify-font-size-9)",
            "min-width:1640px",
            "font-weight:500",
            "--rustify-trim-after-content:\"\"",
            "text-wrap:balance",
            "--rustify-component-color-12:",
        ] {
            assert!(css.contains(declaration), "{declaration}: {css}");
        }
    }
}
