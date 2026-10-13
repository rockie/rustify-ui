use super::{SiteEnvironment, SiteError, Slot};
use leptos::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::Arc;

pub(crate) type ViewFactory = Arc<dyn Fn() -> AnyView + Send + Sync>;
type SurfaceFactory = Arc<dyn Fn(RenderContext) -> AnyView + Send + Sync>;
type ComponentFactory = Arc<dyn Fn(Value) -> Result<PreparedComponent, SiteError> + Send + Sync>;

#[derive(Clone)]
pub(crate) struct Placeholder {
    pub class: String,
    pub message: String,
}

pub(crate) struct PreparedComponent {
    #[cfg(feature = "site-ssr")]
    pub props: Value,
    pub render: Box<dyn FnOnce(AnyView) -> AnyView + Send>,
    pub placeholder: Option<Placeholder>,
}

/// A registry constructed from the shared parser by `site!` on both targets.
#[derive(Clone)]
pub struct SiteRegistry {
    manifest: Value,
    components: BTreeMap<String, ComponentFactory>,
    demos: BTreeMap<String, ViewFactory>,
    pages: BTreeMap<String, SurfaceFactory>,
    slots: BTreeMap<Slot, SurfaceFactory>,
}

impl SiteRegistry {
    /// Starts a registry with the macro's normalized parser manifest.
    #[doc(hidden)]
    pub fn new(manifest: Value) -> Self {
        Self {
            manifest,
            components: BTreeMap::new(),
            demos: BTreeMap::new(),
            pages: BTreeMap::new(),
            slots: BTreeMap::new(),
        }
    }

    /// The exact normalized declaration used by the macro and cold reader.
    pub fn manifest(&self) -> &Value {
        &self.manifest
    }

    /// Registers typed dispatch without constructing a view before its Owner.
    #[doc(hidden)]
    pub fn component<P, F, V>(mut self, name: &str, render: F) -> Self
    where
        P: Serialize + DeserializeOwned + Send + 'static,
        F: Fn(P, AnyView) -> V + Send + Sync + 'static,
        V: IntoView + 'static,
    {
        let placeholder = self
            .manifest
            .get("components")
            .and_then(Value::as_array)
            .and_then(|components| {
                components
                    .iter()
                    .find(|component| component.get("name").and_then(Value::as_str) == Some(name))
            })
            .and_then(|component| component.get("client_only"))
            .and_then(|placeholder| {
                Some(Placeholder {
                    class: placeholder.get("class")?.as_str()?.to_string(),
                    message: placeholder.get("message")?.as_str()?.to_string(),
                })
            });
        let render = Arc::new(render);
        self.components.insert(
            name.to_string(),
            Arc::new(move |json| {
                #[cfg(feature = "site-ssr")]
                let props: P = serde_path_to_error::deserialize(json).map_err(|error| {
                    SiteError::new("invalid_props", error.inner().to_string())
                        .with_property(error.path().to_string())
                })?;
                #[cfg(not(feature = "site-ssr"))]
                let props: P = serde_json::from_value(json)
                    .map_err(|error| SiteError::new("invalid_props", error.to_string()))?;
                #[cfg(feature = "site-ssr")]
                let effective = serde_json::to_value(&props)
                    .map_err(|error| SiteError::new("invalid_props", error.to_string()))?;
                let render = Arc::clone(&render);
                Ok(PreparedComponent {
                    #[cfg(feature = "site-ssr")]
                    props: effective,
                    render: Box::new(move |children| render(props, children).into_any()),
                    placeholder: placeholder.clone(),
                })
            }),
        );
        self
    }

    #[doc(hidden)]
    pub fn demo<F, V>(mut self, name: &str, render: F) -> Self
    where
        F: Fn() -> V + Send + Sync + 'static,
        V: IntoView + 'static,
    {
        self.demos
            .insert(name.to_string(), Arc::new(move || render().into_any()));
        self
    }

    #[doc(hidden)]
    pub fn page<F, V>(mut self, name: &str, render: F) -> Self
    where
        F: Fn(RenderContext) -> V + Send + Sync + 'static,
        V: IntoView + 'static,
    {
        self.pages.insert(
            name.to_string(),
            Arc::new(move |context| render(context).into_any()),
        );
        self
    }

    #[doc(hidden)]
    pub fn slot<F, V>(mut self, slot: Slot, render: F) -> Self
    where
        F: Fn(RenderContext) -> V + Send + Sync + 'static,
        V: IntoView + 'static,
    {
        self.slots
            .insert(slot, Arc::new(move |context| render(context).into_any()));
        self
    }

    pub(crate) fn prepare_component(
        &self,
        name: &str,
        props: Value,
    ) -> Result<PreparedComponent, SiteError> {
        self.components.get(name).ok_or_else(|| {
            SiteError::new("unknown_component", format!("unknown component `{name}`"))
        })?(props)
    }

    #[cfg(feature = "site-ssr")]
    pub(crate) fn demo_view(&self, name: &str) -> Result<AnyView, SiteError> {
        self.demos
            .get(name)
            .map(|render| render())
            .ok_or_else(|| SiteError::new("unknown_demo", format!("unknown demo `{name}`")))
    }

    #[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
    pub(crate) fn prepare_demo(&self, name: &str) -> Result<ViewFactory, SiteError> {
        self.demos
            .get(name)
            .cloned()
            .ok_or_else(|| SiteError::new("unknown_demo", format!("unknown demo `{name}`")))
    }

    #[cfg(feature = "site-ssr")]
    pub(crate) fn surface_view(
        &self,
        name: &str,
        context: RenderContext,
    ) -> Result<AnyView, SiteError> {
        self.pages
            .get(name)
            .map(|render| render(context))
            .ok_or_else(|| SiteError::new("unknown_page", format!("unknown page `{name}`")))
    }

    #[cfg(feature = "site-ssr")]
    pub(crate) fn slot_view(
        &self,
        slot: Slot,
        context: RenderContext,
    ) -> Result<AnyView, SiteError> {
        self.slots
            .get(&slot)
            .map(|render| render(context))
            .ok_or_else(|| {
                SiteError::new("unknown_slot", format!("slot `{slot:?}` has no override"))
            })
    }
}

type DemoRenderer = Arc<dyn Fn(&str) -> AnyView + Send + Sync>;

/// Immutable page/slot context. `demo` uses the shared island rendering path.
#[derive(Clone)]
pub struct RenderContext {
    pub route: String,
    pub lang: String,
    pub base: String,
    render_demo: DemoRenderer,
    render_markdown_intro: DemoRenderer,
}

impl RenderContext {
    #[cfg(feature = "site-ssr")]
    pub(crate) fn new(
        route: String,
        environment: &SiteEnvironment,
        render_demo: DemoRenderer,
        render_markdown_intro: DemoRenderer,
    ) -> Self {
        Self {
            route,
            lang: environment.lang.clone(),
            base: environment.base.clone(),
            render_demo,
            render_markdown_intro,
        }
    }

    /// Requests the same demo block, source and island record as a fence.
    /// The host bridge collects its result for the pipeline's common wrapper.
    pub fn demo(&self, name: &str) -> AnyView {
        (self.render_demo)(name)
    }

    /// Renders the original Markdown between its leading H1 and first H2.
    /// `path` is project-relative; links retain their source-file meaning.
    /// The source remains a build input and is not compiled into browser wasm.
    pub fn markdown_intro(&self, path: &str) -> AnyView {
        (self.render_markdown_intro)(path)
    }
}

pub(crate) fn children_view(html: String) -> AnyView {
    view! { <span data-rustify-children="" class="contents" inner_html=html /> }.into_any()
}

pub(crate) fn placeholder_view(placeholder: Placeholder) -> AnyView {
    view! {
        <div data-rustify-client-only="" class=placeholder.class>
            <p data-rustify-client-message="" role="status">{placeholder.message}</p>
            <div data-rustify-client-host="" />
        </div>
    }
    .into_any()
}

#[derive(Clone)]
pub(crate) struct PreparedEnvironment {
    locale: rustify_ui::Locale,
    theme: Option<PreparedTheme>,
}

#[derive(Clone)]
struct PreparedTheme {
    initial: rustify_ui::ThemeMode,
    light: Arc<rustify_ui::ResolvedTheme>,
    dark: Arc<rustify_ui::ResolvedTheme>,
}

impl PreparedEnvironment {
    #[cfg(any(feature = "site-ssr", test))]
    pub(crate) fn provide(&self) {
        rustify_ui::provide_locale(self.locale);
        if let Some(theme) = &self.theme {
            rustify_ui::provide_static_theme(theme.resolved(theme.initial));
        }
    }

    #[cfg(any(test, all(feature = "hydrate", target_arch = "wasm32")))]
    pub(crate) fn provide_reactive(&self) -> Option<RwSignal<Arc<rustify_ui::ResolvedTheme>>> {
        rustify_ui::provide_locale(self.locale);
        self.theme.as_ref().map(|theme| {
            let current = RwSignal::new(theme.resolved(theme.initial));
            rustify_ui::provide_static_theme(current);
            current
        })
    }

    #[cfg(any(test, all(feature = "hydrate", target_arch = "wasm32")))]
    pub(crate) fn resolved(
        &self,
        mode: rustify_ui::ThemeMode,
    ) -> Option<Arc<rustify_ui::ResolvedTheme>> {
        self.theme.as_ref().map(|theme| theme.resolved(mode))
    }
}

impl PreparedTheme {
    fn resolved(&self, mode: rustify_ui::ThemeMode) -> Arc<rustify_ui::ResolvedTheme> {
        Arc::clone(match mode {
            rustify_ui::ThemeMode::Light => &self.light,
            rustify_ui::ThemeMode::Dark => &self.dark,
        })
    }
}

/// Mirrors the static selectors: only explicit system mode consults the OS.
#[cfg(any(test, all(feature = "hydrate", target_arch = "wasm32")))]
pub(crate) fn document_mode(attribute: Option<&str>, prefers_dark: bool) -> rustify_ui::ThemeMode {
    match attribute {
        Some("dark") => rustify_ui::ThemeMode::Dark,
        Some("system") if prefers_dark => rustify_ui::ThemeMode::Dark,
        _ => rustify_ui::ThemeMode::Light,
    }
}

pub(crate) fn prepare_environment(
    environment: &SiteEnvironment,
) -> Result<PreparedEnvironment, SiteError> {
    let locale = rustify_ui::Locale::from_tag(&environment.lang).ok_or_else(|| {
        SiteError::new(
            "invalid_environment",
            format!("unsupported locale `{}`", environment.lang),
        )
    })?;
    let theme = if let Some(theme) = &environment.theme {
        let context = rustify_ui::theme::font_context(16., false);
        let resolve = |mode| {
            rustify_ui::resolve(&theme.document, mode, &context)
                .map(Arc::new)
                .map_err(|error| SiteError::new("invalid_theme", error.to_string()))
        };
        Some(PreparedTheme {
            initial: theme.mode,
            light: resolve(rustify_ui::ThemeMode::Light)?,
            dark: resolve(rustify_ui::ThemeMode::Dark)?,
        })
    } else {
        None
    };
    Ok(PreparedEnvironment { locale, theme })
}

#[cfg(feature = "site-ssr")]
pub(crate) fn provide_environment(environment: &SiteEnvironment) -> Result<(), SiteError> {
    prepare_environment(environment)?.provide();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::site::ThemeInput;
    use rustify_ui::{Theme, ThemeDocument, ThemeMode};

    fn themed(mode: ThemeMode) -> SiteEnvironment {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        document
            .styles
            .light
            .0
            .insert("font-sans".into(), "Noto Sans".into());
        document
            .styles
            .dark
            .0
            .insert("font-sans".into(), "Rustify WenKai".into());
        document
            .styles
            .light
            .0
            .insert("primary".into(), "#102030".into());
        document
            .styles
            .dark
            .0
            .insert("primary".into(), "#fedcba".into());
        SiteEnvironment {
            theme: Some(ThemeInput { document, mode }),
            ..SiteEnvironment::default()
        }
    }

    #[test]
    fn both_modes_use_the_same_offline_font_context_as_static_css() {
        let environment = prepare_environment(&themed(ThemeMode::Light)).unwrap();
        let light = environment.resolved(ThemeMode::Light).unwrap();
        let dark = environment.resolved(ThemeMode::Dark).unwrap();
        assert_eq!(light.fonts[0].family, "Noto Sans");
        assert_eq!(
            light.fonts[0].resource_path.as_deref(),
            Some("makepad_widgets/resources/NotoSans-Regular.ttf")
        );
        assert_eq!(dark.fonts[0].family, "Rustify WenKai");
        assert_eq!(
            dark.fonts[0].resource_path.as_deref(),
            Some("makepad_widgets/resources/LXGWWenKaiRegular.ttf")
        );
        assert_ne!(light.colors["primary"], dark.colors["primary"]);
        assert!(Arc::ptr_eq(
            &light,
            &environment.resolved(ThemeMode::Light).unwrap()
        ));
        assert!(Arc::ptr_eq(
            &dark,
            &environment.resolved(ThemeMode::Dark).unwrap()
        ));
    }

    #[test]
    fn static_binding_and_reactive_initial_value_follow_the_serialized_ssr_mode() {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            let environment = prepare_environment(&themed(mode)).unwrap();
            let owner = Owner::new();
            owner.with(|| {
                environment.provide();
                let static_theme = rustify_ui::use_resolved_theme().unwrap().get_untracked();
                assert_eq!(static_theme.mode, mode);
                let current = environment.provide_reactive().unwrap();
                assert!(Arc::ptr_eq(&static_theme, &current.get_untracked()));
                assert_eq!(current.get_untracked().mode, mode);
            });
            owner.cleanup();
        }
    }

    #[test]
    fn reactive_binding_is_shared_with_descendant_owners_and_survives_their_cleanup() {
        let environment = prepare_environment(&themed(ThemeMode::Light)).unwrap();
        let owner = Owner::new();
        owner.with(|| {
            let current = environment.provide_reactive().unwrap();
            let descendant = Owner::new();
            let inherited = descendant.with(|| rustify_ui::use_resolved_theme().unwrap());
            assert_eq!(inherited.get_untracked().mode, ThemeMode::Light);
            current.set(environment.resolved(ThemeMode::Dark).unwrap());
            assert_eq!(inherited.get_untracked().mode, ThemeMode::Dark);
            assert_eq!(inherited.get_untracked().fonts[0].family, "Rustify WenKai");
            descendant.cleanup();
            assert_eq!(current.get_untracked().mode, ThemeMode::Dark);
        });
        owner.cleanup();
    }

    #[test]
    fn no_theme_keeps_the_legacy_context_absent() {
        let environment = prepare_environment(&SiteEnvironment::default()).unwrap();
        let owner = Owner::new();
        owner.with(|| {
            environment.provide();
            assert!(rustify_ui::use_resolved_theme().is_none());
            assert!(environment.provide_reactive().is_none());
            assert!(rustify_ui::use_resolved_theme().is_none());
        });
        owner.cleanup();
    }

    #[test]
    fn an_invalid_inactive_mode_is_rejected_before_hydration() {
        let mut environment = themed(ThemeMode::Light);
        environment
            .theme
            .as_mut()
            .unwrap()
            .document
            .styles
            .dark
            .0
            .insert("primary".into(), "invalid color".into());
        let error = prepare_environment(&environment).err().unwrap();
        assert_eq!(error.code, "invalid_theme");
        assert!(error.message.contains("styles.dark.primary"));
    }

    #[test]
    fn document_mode_matches_explicit_and_system_css_selectors() {
        for (attribute, prefers_dark, expected) in [
            (Some("light"), false, ThemeMode::Light),
            (Some("light"), true, ThemeMode::Light),
            (Some("dark"), false, ThemeMode::Dark),
            (Some("dark"), true, ThemeMode::Dark),
            (Some("system"), false, ThemeMode::Light),
            (Some("system"), true, ThemeMode::Dark),
            (None, true, ThemeMode::Light),
            (Some("unknown"), true, ThemeMode::Light),
        ] {
            assert_eq!(document_mode(attribute, prefers_dark), expected);
        }
    }
}
