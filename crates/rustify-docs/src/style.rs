//! One stylesheet for a whole site, compiled in process.
//!
//! The input is virtual: Tailwind 4.1.13's theme and preflight, its
//! utilities, the SDK's Tailwind layer and this crate's reading rules. The
//! candidates are the classes the rendered pages actually carry (see
//! [`crate::classes`]), so nothing scans the file system and the stylesheet
//! holds the rules those pages use and no others. Lightning CSS then lowers
//! what the target browsers lack and minifies the result.

use std::collections::BTreeSet;

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::{Browsers, Targets};
use rustify_style::compiler::{compile, CompileOptions, Limits, Stylesheet};

use crate::highlight;

const TAILWIND_THEME: &str = include_str!("../assets/tailwindcss/theme.css");
const TAILWIND_PREFLIGHT: &str = include_str!("../assets/tailwindcss/preflight.css");
/// What `tailwindcss/utilities.css` holds upstream; not worth a file.
const TAILWIND_UTILITIES: &str = "@tailwind utilities;\n";
const SDK_LAYER: &str = include_str!("../../rustify-components/css/sdk.css");
const SDK_SLIDER: &str = include_str!("../../rustify-components/css/slider.css");
const SDK_CONTROLS: &str = include_str!("../../rustify-components/css/controls.css");
const DOCS_LAYER: &str = include_str!("../assets/docs.css");

/// The SDK layer's dark variant follows a scope's own `data-theme`. A
/// document's root is the scope here, and in `system` mode it follows the
/// reader's preference without a script.
const SDK_DARK: &str = "@custom-variant dark (&:is([data-rustify-scope][data-theme=\"dark\"] *));";
const DOCS_DARK: &str = "@custom-variant dark {\n  &:where([data-theme=\"dark\"], [data-theme=\"dark\"] *) {\n    @slot;\n  }\n  @media (prefers-color-scheme: dark) {\n    &:where([data-theme=\"system\"], [data-theme=\"system\"] *) {\n      @slot;\n    }\n  }\n}";
/// The SDK points Tailwind's scanner at its component sources; a site
/// supplies its candidates itself.
const SDK_SOURCE: &str = "@source \"../src\";\n";

/// The roots the two dark palettes apply to: a page set to dark, and a page
/// following a system that is dark.
pub const DARK_ROOT: &str = ":root[data-theme=\"dark\"]";
pub const SYSTEM_ROOT: &str = ":root[data-theme=\"system\"]";

/// The browsers the stylesheet is lowered for: Tailwind v4's own floor.
fn targets() -> Targets {
    let version = |major: u32, minor: u32| Some((major << 16) | (minor << 8));
    Targets::from(Browsers {
        chrome: version(111, 0),
        edge: version(111, 0),
        firefox: version(128, 0),
        safari: version(16, 4),
        ios_saf: version(16, 4),
        ..Browsers::default()
    })
}

#[derive(Debug)]
pub enum StyleError {
    /// The fixed inputs no longer have the shape this module edits.
    Input(String),
    Compile(String),
    Minify(String),
}

impl std::fmt::Display for StyleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(message) => write!(f, "stylesheet input: {message}"),
            Self::Compile(message) => write!(f, "compiling the stylesheet: {message}"),
            Self::Minify(message) => write!(f, "minifying the stylesheet: {message}"),
        }
    }
}

impl std::error::Error for StyleError {}

/// The SDK layer as a site compiles it: no `@source`, and the document's
/// dark variant. Each edit asserts it found what it replaces, so a change to
/// `sdk.css` fails here instead of quietly compiling the old behaviour.
fn sdk_layer() -> Result<String, StyleError> {
    let mut sdk = SDK_LAYER.to_owned();
    for (from, to) in [(SDK_SOURCE, ""), (SDK_DARK, DOCS_DARK)] {
        if sdk.matches(from).count() != 1 {
            return Err(StyleError::Input(format!(
                "sdk.css no longer contains exactly one {from:?}"
            )));
        }
        sdk = sdk.replacen(from, to, 1);
    }
    Ok(sdk)
}

/// A site's colours. Each mode's primary also sets the focus ring; the
/// button foreground is whichever of black and white reads better on it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Brand {
    pub primary_light: String,
    pub primary_dark: String,
}

impl Default for Brand {
    fn default() -> Self {
        Self {
            primary_light: "#175cd3".into(),
            primary_dark: "#53b1fd".into(),
        }
    }
}

/// Black or white, whichever has more contrast on `background`.
pub fn foreground_on(background: &str) -> &'static str {
    let black = crate::color::contrast("#000000", background).unwrap_or(0.0);
    let white = crate::color::contrast("#ffffff", background).unwrap_or(0.0);
    if black >= white {
        "#000000"
    } else {
        "#ffffff"
    }
}

fn hex(color: u32) -> String {
    format!("#{color:06x}")
}

/// The token values for one mode: the SDK theme's, with the reading theme's
/// colours and the tokens only documentation pages use.
fn mode_tokens(theme: rustify_ui::Theme, primary: &str, code_background: &str) -> String {
    let mut css = String::new();
    for (name, value) in theme.properties() {
        let value = match name {
            "--primary" | "--ring" => primary.to_owned(),
            "--primary-foreground" => foreground_on(primary).to_owned(),
            _ => value,
        };
        css.push_str(&format!("{name}:{value};"));
    }
    let dark = theme.name == "dark";
    // Alert edges and titles, as GitHub colours them, each kept at or above
    // 4.5:1 on the alert's muted background.
    let alerts: [(&str, &str, &str); 5] = if dark {
        [
            ("note", "#4493f8", "#79c0ff"),
            ("tip", "#3fb950", "#7ee787"),
            ("important", "#ab7df8", "#d2a8ff"),
            ("warning", "#d29922", "#e3b341"),
            ("caution", "#f85149", "#ffa198"),
        ]
    } else {
        [
            ("note", "#0969da", "#0550ae"),
            ("tip", "#1a7f37", "#116329"),
            ("important", "#8250df", "#6639ba"),
            ("warning", "#9a6700", "#7d4e00"),
            ("caution", "#cf222e", "#a40e26"),
        ]
    };
    for (name, edge, text) in alerts {
        css.push_str(&format!("--rd-{name}:{edge};--rd-{name}-text:{text};"));
    }
    css.push_str(&format!("--rd-code-bg:{code_background};"));
    css.push_str(&format!(
        "--rd-line-hl:{};",
        if dark { "#1f2a3d" } else { "#e3ebf8" }
    ));
    let _ = hex;
    css
}

/// The token rules: light on the document root, dark when the page says
/// dark, and dark when it follows a system that is.
pub fn tokens_css(brand: &Brand) -> String {
    let light = mode_tokens(
        rustify_ui::Theme::light(),
        &brand.primary_light,
        highlight::CODE_BACKGROUND.0,
    );
    let dark = mode_tokens(
        rustify_ui::Theme::dark(),
        &brand.primary_dark,
        highlight::CODE_BACKGROUND.1,
    );
    format!(
        ":root[data-rustify-scope]{{{light}color-scheme:light;}}\n\
         :root[data-rustify-scope][data-theme=\"dark\"]{{{dark}color-scheme:dark;}}\n\
         @media (prefers-color-scheme: dark){{:root[data-rustify-scope][data-theme=\"system\"]{{{dark}color-scheme:dark;}}}}\n"
    )
}

/// Bounds for the compiler, sized for a site rather than for the interactive
/// worker's small budget. They are generous on purpose; reaching one means
/// the input is not a documentation site's.
pub fn limits() -> Limits {
    Limits {
        input_bytes: 1 << 20,
        asset_bytes: 1 << 20,
        candidate_count: 50_000,
        ast_depth: 128,
        css_bytes: 8 << 20,
        queued_bytes: 8 << 20,
        cache_bytes: 128 << 20,
    }
}

fn input() -> String {
    "@layer theme, base, components, utilities;\n\
     @import \"tailwindcss/theme.css\" layer(theme);\n\
     @import \"tailwindcss/preflight.css\" layer(base);\n\
     @import \"tailwindcss/utilities.css\" layer(utilities);\n\
     @import \"./sdk.css\";\n\
     @import \"./docs.css\" layer(components);\n"
        .to_owned()
}

fn assets() -> Result<Vec<Stylesheet>, StyleError> {
    let asset = |id: &str, content: &str| Stylesheet {
        id: id.to_owned(),
        path: None,
        base: None,
        content: content.into(),
    };
    Ok(vec![
        asset("tailwindcss/theme.css", TAILWIND_THEME),
        asset("tailwindcss/preflight.css", TAILWIND_PREFLIGHT),
        asset("tailwindcss/utilities.css", TAILWIND_UTILITIES),
        asset("/rustify-docs/sdk.css", &sdk_layer()?),
        asset("/rustify-docs/slider.css", SDK_SLIDER),
        asset("/rustify-docs/controls.css", SDK_CONTROLS),
        asset("/rustify-docs/docs.css", DOCS_LAYER),
    ])
}

/// Compiles the utilities `candidates` name, with the fixed layers, into
/// unminified CSS.
pub fn compile_utilities(candidates: &BTreeSet<String>) -> Result<String, StyleError> {
    let options = CompileOptions {
        base: Some("/rustify-docs".into()),
        from: None,
        ..CompileOptions::default()
    };
    let mut compiler = compile(&input(), options, assets()?, limits())
        .map_err(|error| StyleError::Compile(error.to_string()))?;
    compiler.set_license_banner(false);
    let candidates: Vec<String> = candidates.iter().cloned().collect();
    let css = compiler
        .build(&candidates)
        .map_err(|error| StyleError::Compile(error.to_string()))?;
    css.to_utf8()
        .map_err(|error| StyleError::Compile(error.to_string()))
}

/// Lowers `css` for the target browsers and minifies it.
pub fn minify(css: &str) -> Result<String, StyleError> {
    let mut sheet = StyleSheet::parse(css, ParserOptions::default())
        .map_err(|error| StyleError::Minify(error.to_string()))?;
    let targets = targets();
    sheet
        .minify(MinifyOptions {
            targets,
            ..MinifyOptions::default()
        })
        .map_err(|error| StyleError::Minify(error.to_string()))?;
    let printed = sheet
        .to_css(PrinterOptions {
            minify: true,
            targets,
            ..PrinterOptions::default()
        })
        .map_err(|error| StyleError::Minify(error.to_string()))?;
    Ok(printed.code)
}

/// The site's stylesheet: utilities for `candidates`, the brand's tokens and
/// the highlighting palette, minified.
pub fn site_css(candidates: &BTreeSet<String>, brand: &Brand) -> Result<String, StyleError> {
    let mut css = compile_utilities(candidates)?;
    css.push_str(&tokens_css(brand));
    css.push_str(&highlight::theme_css(DARK_ROOT, SYSTEM_ROOT));
    minify(&css)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(classes: &[&str]) -> BTreeSet<String> {
        classes.iter().map(|class| class.to_string()).collect()
    }

    #[test]
    fn the_sdk_layer_loses_its_source_and_gains_the_document_dark_variant() {
        let sdk = sdk_layer().unwrap();
        assert!(!sdk.contains("@source"));
        assert!(sdk.contains("[data-theme=\"system\"]"));
        assert!(!sdk.contains(SDK_DARK));
    }

    #[test]
    fn only_the_candidates_become_utilities() {
        let css =
            compile_utilities(&set(&["flex", "text-primary", "dark:bg-muted", "md:grid"])).unwrap();
        assert!(css.contains(".flex"));
        assert!(css.contains(".text-primary"));
        assert!(css.contains(".md\\:grid"));
        assert!(
            !css.contains(".hidden {"),
            "a class nobody used was compiled"
        );
        // The SDK's components are not scanned: without a candidate, none of
        // their classes is compiled.
        assert!(!css.contains(".h-9"));
    }

    #[test]
    fn dark_utilities_follow_both_dark_roots() {
        let css = site_css(&set(&["dark:bg-muted"]), &Brand::default()).unwrap();
        assert!(css.contains("[data-theme=dark]"), "{css}");
        assert!(css.contains("prefers-color-scheme:dark"), "{css}");
        assert!(css.contains("[data-theme=system]"), "{css}");
    }

    #[test]
    fn the_site_stylesheet_is_minified_and_carries_tokens_and_palette() {
        let css = site_css(&set(&["flex", "rd-prose"]), &Brand::default()).unwrap();
        assert!(!css.contains("\n\n"));
        assert!(css.contains("--primary:#175cd3"));
        assert!(css.contains(".hl-comment"));
        assert!(css.contains(".rd-prose"));
        // Nesting is lowered for Safari 16.4.
        assert!(!css.contains("&"), "{css}");
    }

    #[test]
    fn brand_tokens_set_ring_and_foreground() {
        let tokens = tokens_css(&Brand {
            primary_light: "#ffd400".into(),
            primary_dark: "#1b1b6b".into(),
        });
        assert!(tokens.contains("--primary:#ffd400;--primary-foreground:#000000;"));
        assert!(tokens.contains("--ring:#ffd400;"));
        assert!(tokens.contains("--primary:#1b1b6b;--primary-foreground:#ffffff;"));
    }

    #[test]
    fn default_reading_theme_values() {
        let tokens = tokens_css(&Brand::default());
        let light = tokens.lines().next().unwrap();
        assert!(light.contains("--primary:#175cd3;--primary-foreground:#ffffff;"));
        assert!(light.contains("--background:#f9fafb;"));
        let dark = tokens.lines().nth(1).unwrap();
        // Black or white, whichever reads better on each mode's primary.
        assert!(
            dark.contains("--primary:#53b1fd;--primary-foreground:#000000;"),
            "{dark}"
        );
        assert!(dark.contains("--background:#0c111d;"));
    }

    #[test]
    fn alert_titles_keep_contrast_on_the_muted_background() {
        for (theme, muted) in [
            (rustify_ui::Theme::light(), "#f2f4f7"),
            (rustify_ui::Theme::dark(), "#161b26"),
        ] {
            let tokens = mode_tokens(theme, "#175cd3", "#000000");
            for name in ["note", "tip", "important", "warning", "caution"] {
                let key = format!("--rd-{name}-text:");
                let start = tokens.find(&key).unwrap() + key.len();
                let color = &tokens[start..start + 7];
                let ratio = crate::color::contrast(color, muted).unwrap();
                assert!(ratio >= 4.5, "{name} title {color} on {muted}: {ratio:.2}");
            }
        }
    }
}
