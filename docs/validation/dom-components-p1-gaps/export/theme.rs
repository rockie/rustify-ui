// Source SDK/CSS revision (source CLI only for Docs): b9d5d356d00ebef4d387e0753cf14e2d8917ea9e
use leptos::prelude::*;
use rustify_ui::{mount, AppHandle, MountConfig, ResolvedTheme, ThemeScope};
use rustify_ui::theme::{font_context, import_json, resolve, ResolveContext, ThemeError, ThemeMode};
use std::sync::Arc;

// Put the downloaded theme.json beside this source file.
// Include sdk.css and then theme-v4.css in one Tailwind v4 build.
// Copy the listed same-origin font files and their licenses; fonts are external.
// Use the matching source SDK/CSS. The published 0.2.1 SDK and CLI reject v2.
pub fn load_exported_theme(mode: ThemeMode) -> Result<Arc<ResolvedTheme>, ThemeError> {
    load_exported_theme_with_context(mode, &font_context(16., false))
}

// Use your host's root rem size, motion preference and font context.
pub fn load_exported_theme_with_context(
    mode: ThemeMode,
    context: &ResolveContext<'_>,
) -> Result<Arc<ResolvedTheme>, ThemeError> {
    let document = import_json(include_str!("theme.json"), context)?.document;
    Ok(Arc::new(resolve(&document, mode, context)?))
}

// A GPU region receives this same Arc<ResolvedTheme> through its typed props.
// Retain the returned AppHandle for as long as the application is mounted.
// This ordinary provider honors the document's authored appearance.
pub fn mount_exported_theme(
    container: leptos::web_sys::HtmlElement,
) -> Result<AppHandle, String> {
    let initial = load_exported_theme(ThemeMode::Light).map_err(|error| error.to_string())?;
    mount(container, MountConfig {
        scope: "theme-preview".into(), url_owner: false, base: "/".into(),
    }, move || {
        let theme = RwSignal::new(initial);
        view! {
            <ThemeScope resolved=theme />
            <main class="bg-background text-foreground font-sans p-4 rounded-lg">
                "Your themed application"
            </main>
        }
    }).map_err(|error| error.to_string())
}

// To match Studio's explicit preview, pass its selected mode here. This does
// not change the document's appearance or either mode's author values.
// Set MountConfig.base to the deployment base, including any subpath.
pub fn mount_exported_preview(
    container: leptos::web_sys::HtmlElement,
    config: MountConfig,
    mode: ThemeMode,
    context: &ResolveContext<'_>,
) -> Result<AppHandle, String> {
    let initial = load_exported_theme_with_context(mode, context)
        .map_err(|error| error.to_string())?;
    mount(container, config, move || {
        let theme = RwSignal::new(initial);
        view! {
            <ThemeScope resolved=theme preview_mode=Signal::derive(move || mode) />
            <main class="bg-background text-foreground font-sans p-4 rounded-lg">
                "Your themed preview"
            </main>
        }
    }).map_err(|error| error.to_string())
}
