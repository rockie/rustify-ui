//! One table of values for both halves of a scope.
//!
//! The DOM reads the tokens as CSS custom properties on the scope's own root;
//! a GPU region is handed the same numbers as props. There is one table, so a
//! colour cannot mean one thing in a stylesheet and another in a shader, and it
//! is written to the scope root rather than to the document, so two scopes on
//! one page can hold different themes and a host page holds its own.

/// sRGB, `0xRRGGBB`.
pub type Color = u32;

/// The semantic values P1 actually uses. The names are the ones the component
/// library P2 imports already uses, so its stylesheets need no translation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub muted: Color,
    pub accent: Color,
    pub destructive: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
    /// Corner radius in CSS pixels.
    pub radius: f64,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "light",
            background: 0xf9fafb,
            foreground: 0x1d2939,
            primary: 0x2e90fa,
            primary_foreground: 0xffffff,
            secondary: 0xeaecf0,
            muted: 0x667085,
            accent: 0x12b76a,
            destructive: 0xd92d20,
            border: 0xd0d5dd,
            input: 0xffffff,
            ring: 0x2e90fa,
            radius: 6.0,
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "dark",
            background: 0x0c111d,
            foreground: 0xf9fafb,
            primary: 0x53b1fd,
            primary_foreground: 0x0c111d,
            secondary: 0x1d2939,
            muted: 0x98a2b3,
            accent: 0x32d583,
            destructive: 0xf97066,
            border: 0x344054,
            input: 0x101828,
            ring: 0x53b1fd,
            radius: 6.0,
        }
    }

    /// The tokens as a stylesheet writes them: custom property name and value.
    pub fn properties(&self) -> [(&'static str, String); 12] {
        [
            ("--background", hex(self.background)),
            ("--foreground", hex(self.foreground)),
            ("--primary", hex(self.primary)),
            ("--primary-foreground", hex(self.primary_foreground)),
            ("--secondary", hex(self.secondary)),
            ("--muted", hex(self.muted)),
            ("--accent", hex(self.accent)),
            ("--destructive", hex(self.destructive)),
            ("--border", hex(self.border)),
            ("--input", hex(self.input)),
            ("--ring", hex(self.ring)),
            ("--radius", format!("{}px", self.radius)),
        ]
    }
}

/// `#rrggbb`, which is what a stylesheet and a shader can both read without
/// either of them parsing a colour space.
pub fn hex(color: Color) -> String {
    format!("#{color:06x}")
}

#[cfg(target_arch = "wasm32")]
pub use dom::{use_theme, ThemedScope};

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::Theme;
    use crate::mount::ScopeRoots;
    use leptos::prelude::*;

    /// The theme of the scope the caller is rendered in.
    pub fn use_theme() -> Option<RwSignal<Theme>> {
        use_context::<RwSignal<Theme>>()
    }

    /// Writes the scope's theme onto its own root, and rewrites it whenever the
    /// theme changes.
    ///
    /// Rendered once by an application that wants its scope themed. The values
    /// go on through the CSSOM rather than a `style` attribute, so a strict
    /// `style-src` does not have to be relaxed for them.
    #[component]
    pub fn ThemedScope(#[prop(into)] theme: Signal<Theme>) -> impl IntoView {
        let roots = use_context::<ScopeRoots>();
        let published = RwSignal::new(theme.get_untracked());
        provide_context(published);
        Effect::new(move || {
            let theme = theme.get();
            published.set(theme);
            let Some(roots) = roots.as_ref() else {
                return;
            };
            let root = roots.container();
            let style = root.style();
            for (name, value) in theme.properties() {
                let _ = style.set_property(name, &value);
            }
            // The attribute is what a dark variant selector matches, and it is
            // on the scope rather than the document so a host page keeps its
            // own.
            let _ = root.set_attribute("data-theme", theme.name);
        });
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_colour_is_written_the_way_both_halves_read_it() {
        assert_eq!(hex(0x2e90fa), "#2e90fa");
        assert_eq!(hex(0x000000), "#000000");
        assert_eq!(hex(0xffffff), "#ffffff");
    }

    #[test]
    fn the_two_themes_name_the_same_tokens_and_agree_on_nothing_else() {
        let light = Theme::light().properties();
        let dark = Theme::dark().properties();
        let names: Vec<&str> = light.iter().map(|(name, _)| *name).collect();
        assert_eq!(
            names,
            dark.iter().map(|(name, _)| *name).collect::<Vec<_>>()
        );
        // Every colour differs between the two; only the radius is shared.
        let differing = light
            .iter()
            .zip(dark.iter())
            .filter(|((_, a), (_, b))| a != b)
            .count();
        assert_eq!(differing, names.len() - 1);
    }

    #[test]
    fn a_radius_carries_its_unit() {
        let radius = Theme::light()
            .properties()
            .into_iter()
            .find(|(name, _)| *name == "--radius")
            .map(|(_, value)| value);
        assert_eq!(radius.as_deref(), Some("6px"));
    }
}
