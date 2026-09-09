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
    /// Body text size in CSS pixels.
    pub font_size: f64,
    /// The gap the scope lays out with, in CSS pixels.
    pub spacing: f64,
    /// The user asked for less movement. Both halves honour it: the DOM as a
    /// zero transition duration, the region by not animating a change it
    /// would otherwise ease into.
    pub reduce_motion: bool,
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
            font_size: 15.0,
            spacing: 8.0,
            reduce_motion: false,
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
            font_size: 15.0,
            spacing: 8.0,
            reduce_motion: false,
        }
    }

    /// The tokens as a stylesheet writes them: custom property name and value.
    pub fn properties(&self) -> [(&'static str, String); 15] {
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
            ("--radius", px(self.radius)),
            ("--font-size", px(self.font_size)),
            ("--spacing", px(self.spacing)),
            ("--motion-duration", motion(self.reduce_motion)),
        ]
    }

    /// This theme with the local changes applied. Anything the patch does not
    /// name keeps the value it inherits.
    pub fn patched(&self, patch: &ThemePatch) -> Theme {
        let mut theme = *self;
        if let Some(background) = patch.background {
            theme.background = background;
        }
        if let Some(foreground) = patch.foreground {
            theme.foreground = foreground;
        }
        if let Some(primary) = patch.primary {
            theme.primary = primary;
        }
        if let Some(accent) = patch.accent {
            theme.accent = accent;
        }
        if let Some(font_size) = patch.font_size {
            theme.font_size = font_size;
        }
        if let Some(spacing) = patch.spacing {
            theme.spacing = spacing;
        }
        if let Some(radius) = patch.radius {
            theme.radius = radius;
        }
        theme
    }
}

/// A local change to part of a theme.
///
/// R16 lets an application override one area's colour, text size and spacing
/// without touching the rest of the scope or any other scope. A patch names
/// only what it changes; everything else inherits, in the DOM because custom
/// properties inherit and in a region because the region is handed the patched
/// table.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ThemePatch {
    pub background: Option<Color>,
    pub foreground: Option<Color>,
    pub primary: Option<Color>,
    pub accent: Option<Color>,
    pub font_size: Option<f64>,
    pub spacing: Option<f64>,
    pub radius: Option<f64>,
}

impl ThemePatch {
    /// Every property a patch can name.
    ///
    /// A patch that stops naming one has to take it off the element, not leave
    /// the value an earlier patch wrote: an override that cannot be turned off
    /// is not local, it is permanent.
    pub const PROPERTIES: [&'static str; 7] = [
        "--background",
        "--foreground",
        "--primary",
        "--accent",
        "--font-size",
        "--spacing",
        "--radius",
    ];

    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// Only the properties this patch names. Writing the others would end the
    /// inheritance that makes an override local.
    pub fn properties(&self) -> Vec<(&'static str, String)> {
        let mut out = Vec::new();
        if let Some(background) = self.background {
            out.push(("--background", hex(background)));
        }
        if let Some(foreground) = self.foreground {
            out.push(("--foreground", hex(foreground)));
        }
        if let Some(primary) = self.primary {
            out.push(("--primary", hex(primary)));
        }
        if let Some(accent) = self.accent {
            out.push(("--accent", hex(accent)));
        }
        if let Some(font_size) = self.font_size {
            out.push(("--font-size", px(font_size)));
        }
        if let Some(spacing) = self.spacing {
            out.push(("--spacing", px(spacing)));
        }
        if let Some(radius) = self.radius {
            out.push(("--radius", px(radius)));
        }
        out
    }
}

/// CSS pixels, printed the way a stylesheet reads them.
fn px(value: f64) -> String {
    format!("{value}px")
}

/// How long a transition may take. Zero is the honest answer to "less
/// movement": the result still arrives, it just does not travel.
fn motion(reduce: bool) -> String {
    if reduce { "0ms" } else { "150ms" }.to_string()
}

/// `#rrggbb`, which is what a stylesheet and a shader can both read without
/// either of them parsing a colour space.
pub fn hex(color: Color) -> String {
    format!("#{color:06x}")
}

#[cfg(target_arch = "wasm32")]
pub use dom::{use_theme, use_theme_values, ThemeOverride, ThemedScope};

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{Theme, ThemePatch};
    use crate::mount::ScopeRoots;
    use leptos::prelude::*;

    /// The values in force where the caller is rendered, override included.
    /// Provided by `ThemedScope` and narrowed by every `ThemeOverride` around
    /// the caller.
    #[derive(Clone, Copy)]
    struct ThemeValues(Signal<Theme>);

    /// The scope's theme, as something the application can write. An override
    /// does not change it: it is the scope's own setting, not the values in
    /// force at one place in the view.
    pub fn use_theme() -> Option<RwSignal<Theme>> {
        use_context::<RwSignal<Theme>>()
    }

    /// The values in force here: the scope's theme with every override around
    /// the caller applied. A region reads this to know what to draw with.
    pub fn use_theme_values() -> Option<Signal<Theme>> {
        use_context::<ThemeValues>().map(|values| values.0)
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
        provide_context(ThemeValues(published.into()));
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

    /// One area of a scope with some of the theme changed.
    ///
    /// Only the patched properties are written, onto this element; everything
    /// else inherits, so the rest of the scope, the regions outside it and any
    /// other scope on the page keep the values they had. The children are
    /// handed the patched table too, so a region inside an override draws with
    /// the same numbers the stylesheet uses.
    #[component]
    pub fn ThemeOverride(
        #[prop(into)] patch: Signal<ThemePatch>,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
        children: Children,
    ) -> impl IntoView {
        let outer = use_theme_values();
        let values = Signal::derive(move || {
            let base = outer.map(|outer| outer.get()).unwrap_or_else(Theme::light);
            base.patched(&patch.get())
        });
        provide_context(ThemeValues(values));
        let node = NodeRef::<leptos::html::Div>::new();
        // Written through the CSSOM, like the scope root's: a strict
        // `style-src` never has to be relaxed for a theme.
        Effect::new(move || {
            let patch = patch.get();
            let Some(element) = node.get() else {
                return;
            };
            let element: leptos::web_sys::HtmlElement = element.into();
            let style = element.style();
            let named = patch.properties();
            for (name, value) in &named {
                let _ = style.set_property(name, value);
            }
            // What this patch does not name goes back to being inherited.
            for name in ThemePatch::PROPERTIES {
                if !named.iter().any(|(named, _)| *named == name) {
                    let _ = style.remove_property(name);
                }
            }
        });
        view! {
            <div node_ref=node class=class data-testid=test_id data-rustify-theme-override="">
                {children()}
            </div>
        }
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
    fn the_two_themes_name_the_same_tokens_and_agree_on_nothing_but_the_metrics() {
        let light = Theme::light().properties();
        let dark = Theme::dark().properties();
        let names: Vec<&str> = light.iter().map(|(name, _)| *name).collect();
        assert_eq!(
            names,
            dark.iter().map(|(name, _)| *name).collect::<Vec<_>>()
        );
        // A theme is a set of colours over one set of metrics: every colour
        // differs, and the sizes are the same in both.
        let shared: Vec<&str> = light
            .iter()
            .zip(dark.iter())
            .filter(|((_, a), (_, b))| a == b)
            .map(|((name, _), _)| *name)
            .collect();
        assert_eq!(
            shared,
            ["--radius", "--font-size", "--spacing", "--motion-duration"]
        );
    }

    #[test]
    fn an_override_writes_only_what_it_names() {
        let patch = ThemePatch {
            foreground: Some(0x123456),
            font_size: Some(12.0),
            spacing: Some(4.0),
            ..ThemePatch::default()
        };
        assert_eq!(
            patch.properties(),
            [
                ("--foreground", "#123456".to_string()),
                ("--font-size", "12px".to_string()),
                ("--spacing", "4px".to_string()),
            ]
        );
        assert!(!patch.is_empty());
        assert!(ThemePatch::default().is_empty());
        assert!(ThemePatch::default().properties().is_empty());
    }

    #[test]
    fn every_property_a_patch_can_write_is_one_it_can_also_take_back() {
        let everything = ThemePatch {
            background: Some(1),
            foreground: Some(2),
            primary: Some(3),
            accent: Some(4),
            font_size: Some(5.0),
            spacing: Some(6.0),
            radius: Some(7.0),
        };
        let written: Vec<&str> = everything
            .properties()
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert_eq!(written, ThemePatch::PROPERTIES);
    }

    #[test]
    fn a_patched_theme_keeps_everything_the_patch_did_not_name() {
        let base = Theme::dark();
        let patched = base.patched(&ThemePatch {
            foreground: Some(0x123456),
            font_size: Some(12.0),
            ..ThemePatch::default()
        });
        assert_eq!(patched.foreground, 0x123456);
        assert_eq!(patched.font_size, 12.0);
        assert_eq!(patched.background, base.background);
        assert_eq!(patched.spacing, base.spacing);
        // An override is a local change to a theme, not a different theme.
        assert_eq!(patched.name, base.name);
        assert_eq!(base.patched(&ThemePatch::default()), base);
    }

    #[test]
    fn less_movement_is_no_movement_rather_than_no_result() {
        let mut theme = Theme::light();
        theme.reduce_motion = true;
        let duration = theme
            .properties()
            .into_iter()
            .find(|(name, _)| *name == "--motion-duration")
            .map(|(_, value)| value);
        assert_eq!(duration.as_deref(), Some("0ms"));
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
