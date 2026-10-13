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
}

impl RenderContext {
    #[cfg(feature = "site-ssr")]
    pub(crate) fn new(
        route: String,
        environment: &SiteEnvironment,
        render_demo: DemoRenderer,
    ) -> Self {
        Self {
            route,
            lang: environment.lang.clone(),
            base: environment.base.clone(),
            render_demo,
        }
    }

    /// Requests the same demo block, source and island record as a fence.
    /// The host bridge collects its result for the pipeline's common wrapper.
    pub fn demo(&self, name: &str) -> AnyView {
        (self.render_demo)(name)
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
    theme: Option<Arc<rustify_ui::ResolvedTheme>>,
}

impl PreparedEnvironment {
    pub(crate) fn provide(&self) {
        rustify_ui::provide_locale(self.locale);
        if let Some(theme) = &self.theme {
            rustify_ui::provide_static_theme(Arc::clone(theme));
        }
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
        let resolved = rustify_ui::resolve(
            &theme.document,
            theme.mode,
            &rustify_ui::ResolveContext::default(),
        )
        .map_err(|error| SiteError::new("invalid_theme", error.to_string()))?;
        Some(Arc::new(resolved))
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
