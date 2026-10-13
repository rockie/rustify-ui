use super::registry::{children_view, placeholder_view, provide_environment, PreparedComponent};
use super::{
    Description, HelperOperation, HelperRequest, HelperResponse, HelperResult, RenderContext,
    RenderRequest, RenderResult, RenderTarget, RenderedDemo, ResourceAsset, SiteEnvironment,
    SiteError, SiteRegistry, PROTOCOL_VERSION,
};
use any_spawner::{CustomExecutor, Executor, PinnedLocalFuture};
use hydration_context::{
    PinnedFuture, PinnedStream, SerializedDataId, SharedContext, SsrSharedContext,
};
use leptos::error::{Error, ErrorId};
use leptos::prelude::*;
use sha2::{Digest, Sha256};
use std::cell::Cell;
use std::io::{self, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

/// Serves one describe or batch-render request from stdin as one JSON response.
pub fn serve_site(registry: SiteRegistry) -> Result<(), SiteError> {
    let request = serde_json::from_reader(io::stdin().lock())
        .map_err(|error| SiteError::new("invalid_request", error.to_string()))?;
    let response = registry.handle(request)?;
    let mut stdout = io::stdout().lock();
    serde_json::to_writer(&mut stdout, &response)
        .map_err(|error| SiteError::new("response_io", error.to_string()))?;
    stdout
        .write_all(b"\n")
        .map_err(|error| SiteError::new("response_io", error.to_string()))
}

impl SiteRegistry {
    /// Processes a full batch under isolated rendering Owners.
    pub fn handle(&self, request: HelperRequest) -> Result<HelperResponse, SiteError> {
        if request.protocol_version != PROTOCOL_VERSION {
            return Err(SiteError::new(
                "protocol_mismatch",
                format!(
                    "expected helper protocol {PROTOCOL_VERSION}, got {}",
                    request.protocol_version
                ),
            ));
        }
        let result = match request.operation {
            HelperOperation::Describe => HelperResult::Describe {
                description: self.describe(),
            },
            HelperOperation::Render { environment, items } => {
                let mut keys = std::collections::BTreeSet::new();
                let mut seeds = std::collections::BTreeSet::new();
                for item in &items {
                    if item.key.is_empty() || item.seed.is_empty() {
                        return Err(SiteError::new(
                            "invalid_request",
                            "render keys and seeds must not be empty",
                        ));
                    }
                    if !keys.insert(&item.key) || !seeds.insert(&item.seed) {
                        return Err(SiteError::new(
                            "invalid_request",
                            "render keys and seeds must be unique in a batch",
                        ));
                    }
                }
                HelperResult::Render {
                    items: items
                        .iter()
                        .map(|item| self.render(item, &environment))
                        .collect(),
                }
            }
        };
        Ok(HelperResponse {
            protocol_version: PROTOCOL_VERSION,
            generation: request.generation,
            result,
        })
    }

    /// The framework assets and normalized declaration from this exact build.
    pub fn describe(&self) -> Description {
        let asset = |path: &str, content: &str| ResourceAsset {
            path: path.to_string(),
            content: content.to_string(),
            sha256: format!("{:x}", Sha256::digest(content.as_bytes())),
        };
        Description {
            registry: self.manifest().clone(),
            framework_release: format!("rustify-docs@{}", env!("CARGO_PKG_VERSION")),
            runtime_protocol: rustify_ui::runtime::PROTOCOL,
            island_protocol: PROTOCOL_VERSION,
            resources: vec![
                asset("dom-loader.js", rustify_ui::runtime::DOM_LOADER_JS),
                asset("dom-runtime.css", rustify_ui::runtime::DOM_RUNTIME_CSS),
                asset("islands.js", include_str!("../../assets/islands.js")),
                asset("sdk-candidates.json", rustify_components::CANDIDATES_JSON),
            ],
        }
    }

    /// Renders a pure-content builtin through the same typed component dispatch.
    pub fn render_component_static(
        &self,
        name: &str,
        props: serde_json::Value,
        children_html: String,
    ) -> Result<String, SiteError> {
        let prepared = generate(|| self.prepare_component(name, props))?;
        if prepared.placeholder.is_some() {
            return Err(SiteError::new(
                "client_only_static",
                "client-only components need an explicit interactive island",
            ));
        }
        let environment = SiteEnvironment::default();
        let seed = format!("static-{name}");
        let html = generate(|| {
            rustify_ui::id::provide(&seed);
            provide_environment(&environment)?;
            rustify_components::geometry::provide_geometry_styles(Default::default());
            Ok((prepared.render)(children_view(children_html)).to_html())
        })?;
        Ok(html)
    }

    /// Renders one target; any diagnostic prevents that target publishing HTML.
    pub fn render(&self, request: &RenderRequest, environment: &SiteEnvironment) -> RenderResult {
        let nested = Arc::new(Mutex::new(Vec::<RenderedDemo>::new()));
        let mut effective_props = None;
        let mut client_only = false;
        let generated = match &request.target {
            RenderTarget::Component {
                name,
                props,
                children_html,
            } => generate(|| self.prepare_component(name, props.clone())).and_then(|prepared| {
                effective_props = Some(prepared.props.clone());
                client_only = prepared.placeholder.is_some();
                self.render_island(request, environment, Some(prepared), children_html.clone())
            }),
            RenderTarget::Demo { .. } => {
                self.render_island(request, environment, None, String::new())
            }
            RenderTarget::Page { name, route } => self.render_surface(
                request,
                environment,
                route,
                Arc::clone(&nested),
                |context| self.surface_view(name, context),
            ),
            RenderTarget::Slot { slot, route } => self.render_surface(
                request,
                environment,
                route,
                Arc::clone(&nested),
                |context| self.slot_view(*slot, context),
            ),
        };
        let demos = nested
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        let (html, geometry_css, mut diagnostics) = match generated {
            Ok((html, css)) => (Some(html), css, Vec::new()),
            Err(error) => (None, String::new(), vec![error]),
        };
        for demo in &demos {
            diagnostics.extend(demo.diagnostics.iter().cloned());
        }
        RenderResult {
            key: request.key.clone(),
            html: diagnostics.is_empty().then_some(html).flatten(),
            geometry_css,
            props: effective_props,
            client_only,
            diagnostics,
            demos,
        }
    }

    fn render_island(
        &self,
        request: &RenderRequest,
        environment: &SiteEnvironment,
        prepared: Option<PreparedComponent>,
        children_html: String,
    ) -> Result<(String, String), SiteError> {
        let styles = rustify_components::geometry::GeometryStyles::default();
        let html = generate(|| {
            let mut render_error = None;
            let view = rustify_ui::ssr_scope(
                rustify_ui::MountConfig {
                    scope: request.seed.clone(),
                    base: environment.base.clone(),
                    url_owner: false,
                },
                || {
                    rustify_components::geometry::provide_geometry_styles(styles.clone());
                    let environment_result = provide_environment(environment);
                    let body = environment_result.and_then(|_| match prepared {
                        Some(prepared) => Ok(match prepared.placeholder {
                            Some(placeholder) => placeholder_view(placeholder),
                            None => (prepared.render)(children_view(children_html)),
                        }),
                        None => match &request.target {
                            RenderTarget::Demo { name } => self.demo_view(name),
                            _ => Err(SiteError::new(
                                "invalid_target",
                                "expected a component or demo island",
                            )),
                        },
                    });
                    match body {
                        Ok(body) => body,
                        Err(error) => {
                            render_error = Some(error);
                            ().into_any()
                        }
                    }
                },
            );
            let html = view.to_html();
            match render_error {
                Some(error) => Err(error),
                None => Ok(html),
            }
        })?;
        Ok((html, styles.css()))
    }

    fn render_surface(
        &self,
        request: &RenderRequest,
        environment: &SiteEnvironment,
        route: &str,
        nested: Arc<Mutex<Vec<RenderedDemo>>>,
        render: impl FnOnce(RenderContext) -> Result<AnyView, SiteError>,
    ) -> Result<(String, String), SiteError> {
        let sequence = Arc::new(AtomicUsize::new(0));
        let registry = self.clone();
        let parent_key = request.key.clone();
        let parent_seed = request.seed.clone();
        let demo_environment = environment.clone();
        let context = RenderContext::new(
            route.to_string(),
            environment,
            Arc::new(move |name| {
                let index = sequence.fetch_add(1, Ordering::Relaxed);
                let key = format!("{parent_key}-demo-{index}");
                let seed = format!("{parent_seed}-demo-{index}");
                let result = registry.render(
                    &RenderRequest {
                        key: key.clone(),
                        seed: seed.clone(),
                        target: RenderTarget::Demo {
                            name: name.to_string(),
                        },
                    },
                    &demo_environment,
                );
                nested
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .push(RenderedDemo {
                        key: key.clone(),
                        seed,
                        name: name.to_string(),
                        html: result.html,
                        geometry_css: result.geometry_css,
                        diagnostics: result.diagnostics,
                    });
                view! { <span data-rustify-demo-placeholder=key /> }.into_any()
            }),
        );
        let styles = rustify_components::geometry::GeometryStyles::default();
        let html = generate(|| {
            rustify_ui::id::provide(&request.seed);
            provide_environment(environment)?;
            rustify_components::geometry::provide_geometry_styles(styles.clone());
            Ok(render(context)?.to_html())
        })?;
        Ok((html, styles.css()))
    }
}

struct StaticExecutor;

impl CustomExecutor for StaticExecutor {
    fn spawn(&self, _future: any_spawner::PinnedFuture<()>) {}
    fn spawn_local(&self, _future: PinnedLocalFuture<()>) {}
    fn poll_local(&self) {}
}

static GENERATION_LOCK: Mutex<()> = Mutex::new(());

thread_local! {
    static GENERATION_DEPTH: Cell<usize> = const { Cell::new(0) };
}

struct GenerationGuard {
    // Suppression must drop before the lock releases: upstream's flag is global.
    _suppression: Option<leptos::server::SuppressResourceLoad>,
    _lock: Option<MutexGuard<'static, ()>>,
}

impl GenerationGuard {
    fn enter() -> Self {
        let outermost = GENERATION_DEPTH.with(|depth| depth.get() == 0);
        let lock = outermost.then(|| {
            GENERATION_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
        });
        let suppression = outermost.then(leptos::server::SuppressResourceLoad::new);
        GENERATION_DEPTH.with(|depth| depth.set(depth.get() + 1));
        Self {
            _suppression: suppression,
            _lock: lock,
        }
    }
}

impl Drop for GenerationGuard {
    fn drop(&mut self) {
        GENERATION_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

fn generate<T>(render: impl FnOnce() -> Result<T, SiteError>) -> Result<T, SiteError> {
    let _generation = GenerationGuard::enter();
    let _ = Executor::init_local_custom_executor(StaticExecutor);
    let context = Arc::new(ObservedContext::new());
    let issues = rustify_ui::StaticRenderIssues::default();
    let previous = Owner::current();
    let owner = Owner::new_root(Some(context.clone()));
    let result = catch_unwind(AssertUnwindSafe(|| {
        owner.with(|| {
            rustify_ui::provide_static_render_issues(issues.clone());
            render()
        })
    }));
    owner.unset_with_forced_cleanup();
    if let Some(previous) = previous {
        previous.set();
    }
    let value = match result {
        Ok(result) => result?,
        Err(payload) => {
            let message = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| {
                    payload
                        .downcast_ref::<&str>()
                        .map(|message| (*message).to_string())
                })
                .unwrap_or_else(|| "view panicked during static generation".to_string());
            return Err(SiteError::new("view_panicked", message));
        }
    };
    if context.needs_shared_data() {
        return Err(SiteError::new(
            "unsupported_initial_data",
            "initial Resource/Suspense/Transition/ErrorBoundary needs shared asynchronous data or errors; defer that work until browser attachment or register an explicit client_only placeholder",
        ));
    }
    let messages = issues.messages();
    if !messages.is_empty() {
        return Err(SiteError::new(
            "unsupported_initial_state",
            messages.join("; "),
        ));
    }
    Ok(value)
}

#[derive(Debug)]
struct ObservedContext {
    inner: SsrSharedContext,
    asynchronous: AtomicUsize,
    errors: AtomicUsize,
    incomplete: AtomicUsize,
}

impl ObservedContext {
    fn new() -> Self {
        Self {
            inner: SsrSharedContext::new(),
            asynchronous: AtomicUsize::new(0),
            errors: AtomicUsize::new(0),
            incomplete: AtomicUsize::new(0),
        }
    }

    fn needs_shared_data(&self) -> bool {
        self.asynchronous.load(Ordering::Relaxed) > 0
            || self.errors.load(Ordering::Relaxed) > 0
            || self.incomplete.load(Ordering::Relaxed) > 0
    }
}

impl SharedContext for ObservedContext {
    fn is_browser(&self) -> bool {
        false
    }
    fn next_id(&self) -> SerializedDataId {
        self.inner.next_id()
    }
    fn write_async(&self, id: SerializedDataId, future: PinnedFuture<String>) {
        self.asynchronous.fetch_add(1, Ordering::Relaxed);
        self.inner.write_async(id, future);
    }
    fn read_data(&self, id: &SerializedDataId) -> Option<String> {
        self.inner.read_data(id)
    }
    fn await_data(&self, id: &SerializedDataId) -> Option<String> {
        self.inner.await_data(id)
    }
    fn pending_data(&self) -> Option<PinnedStream<String>> {
        self.inner.pending_data()
    }
    fn during_hydration(&self) -> bool {
        false
    }
    fn hydration_complete(&self) {
        self.inner.hydration_complete();
    }
    fn get_is_hydrating(&self) -> bool {
        self.inner.get_is_hydrating()
    }
    fn set_is_hydrating(&self, value: bool) {
        self.inner.set_is_hydrating(value);
    }
    fn take_errors(&self) -> Vec<(SerializedDataId, ErrorId, Error)> {
        self.inner.take_errors()
    }
    fn errors(&self, id: &SerializedDataId) -> Vec<(ErrorId, Error)> {
        self.inner.errors(id)
    }
    fn seal_errors(&self, id: &SerializedDataId) {
        self.inner.seal_errors(id);
    }
    fn register_error(&self, boundary: SerializedDataId, id: ErrorId, error: Error) {
        self.errors.fetch_add(1, Ordering::Relaxed);
        self.inner.register_error(boundary, id, error);
    }
    fn defer_stream(&self, future: PinnedFuture<()>) {
        self.asynchronous.fetch_add(1, Ordering::Relaxed);
        self.inner.defer_stream(future);
    }
    fn await_deferred(&self) -> Option<PinnedFuture<()>> {
        self.inner.await_deferred()
    }
    fn set_incomplete_chunk(&self, id: SerializedDataId) {
        self.incomplete.fetch_add(1, Ordering::Relaxed);
        self.inner.set_incomplete_chunk(id);
    }
    fn get_incomplete_chunk(&self, id: &SerializedDataId) -> bool {
        self.inner.get_incomplete_chunk(id)
    }
}
