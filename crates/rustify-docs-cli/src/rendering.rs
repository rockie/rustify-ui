//! Connects one compiled Rust generation to the existing content pipeline.

use crate::rust_site::{self, BuildMode, CompileTiming, Prepared};
use rustify_docs::content::{
    self, Analysis, Diagnostic, FragmentCache, Page, Project, RegisteredPage, Snapshot, Surfaces,
};
use rustify_docs::diagnostic::{LineIndex, Severity};
use rustify_docs::diagnostic_text::{code_span, literal};
use rustify_docs::embed::{self, Kind, Owner};
use rustify_docs::generation::{self, Extensions};
use rustify_docs::highlight::Highlighter;
use rustify_docs::markdown::Embed;
use rustify_docs::site::{
    IslandMetadata, RenderRequest, RenderTarget, SiteEnvironment, Slot, ThemeInput,
    PROTOCOL_VERSION,
};
use rustify_docs_syntax::{ComponentManifest, Manifest};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Build {
    pub analysis: Analysis,
    pub extensions: Extensions,
    pub diagnostics: Vec<Diagnostic>,
    pub info: Value,
    pub guard: Option<Prepared>,
    pub theme: Option<crate::theme::PreparedTheme>,
}

pub(super) fn build(
    project: &Project,
    snapshot: &Snapshot,
    analysis: Analysis,
    cache: &mut FragmentCache,
    highlighter: &Highlighter,
    mode: BuildMode,
) -> Build {
    build_cached(
        project,
        snapshot,
        analysis,
        cache,
        highlighter,
        mode,
        None,
        &|_| {},
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn build_cached(
    project: &Project,
    snapshot: &Snapshot,
    mut analysis: Analysis,
    cache: &mut FragmentCache,
    highlighter: &Highlighter,
    mode: BuildMode,
    cached: Option<&Prepared>,
    observer: rust_site::Observer<'_>,
) -> Build {
    let (theme, mut diagnostics) = crate::theme::prepare(project);
    let mut manifest = Manifest::default();
    let reused = cached.and_then(|cached| cached.reuse(project).ok().flatten());
    let is_reused = reused.is_some();
    let (prepared, problems) = match reused {
        Some(prepared) => {
            manifest = prepared.manifest.clone();
            observer(rust_site::Progress::Watch(prepared.watch_plan()));
            let problems = preflight(project, snapshot, &analysis, Some(&manifest));
            (Some(prepared), problems)
        }
        None => rust_site::prepare_checked_with(
            project,
            mode,
            |declaration| {
                manifest = declaration.clone();
                preflight(project, snapshot, &analysis, Some(declaration))
            },
            observer,
        ),
    };
    diagnostics.extend(problems);
    if project.config.rust.is_none() {
        diagnostics.extend(preflight(project, snapshot, &analysis, None));
    }
    let mut extensions = Extensions::default();
    if let Some(theme) = &theme {
        diagnostics.extend(crate::theme::validate_snapshot(project, snapshot, theme));
        extensions.files.extend(theme.files.clone());
        extensions.theme_utilities.clone_from(&theme.utilities);
        extensions.theme_css.clone_from(&theme.css);
    }
    let mut surfaces = declared_pages(project, &manifest);
    let Some(prepared) = prepared else {
        if !manifest.pages.is_empty() {
            analysis = content::analyze_with_surfaces(
                project,
                snapshot,
                analysis.mode,
                cache,
                highlighter,
                generation::default_workers(),
                &surfaces,
            );
        }
        return Build {
            analysis,
            extensions,
            diagnostics,
            info: Value::Null,
            guard: None,
            theme,
        };
    };
    if let Some(theme) = &theme {
        if !prepared.matches_input(&theme.input_path, &theme.input_bytes) {
            diagnostics.push(Diagnostic::in_file(
                &theme.input_path,
                Severity::Error,
                "rust-generation-changed",
                "theme.document differs from the compiled generation's input snapshot",
            ));
        }
    }
    let info = json!({
        "generation":prepared.generation,"framework_release":prepared.framework_release,
        "host":timing(&prepared.timings.host),"wasm":timing(&prepared.timings.wasm),
        "reused":is_reused,
    });
    observer(rust_site::Progress::Render { reused: is_reused });
    extensions.files.extend(prepared.resources.clone());
    extensions
        .candidates
        .extend(prepared.candidates.iter().cloned());
    surfaces
        .candidates
        .extend(prepared.candidates.iter().cloned());
    extensions.island_script = Some(format!("{}islands.js", prepared.asset_prefix));
    let environment = SiteEnvironment {
        base: project.config.site.base.clone(),
        lang: project.config.site.lang.clone(),
        theme: theme.as_ref().map(|theme| ThemeInput {
            document: theme.document.clone(),
            mode: rustify_ui::ThemeMode::Light,
        }),
    };
    let surface_render = render_surfaces(
        project,
        &analysis,
        &prepared,
        &environment,
        &mut surfaces,
        &mut extensions,
        (snapshot, highlighter),
    );
    diagnostics.extend(surface_render.diagnostics);
    if !manifest.pages.is_empty() || !manifest.slots.is_empty() || !surfaces.candidates.is_empty() {
        analysis = content::analyze_with_surfaces(
            project,
            snapshot,
            analysis.mode,
            cache,
            highlighter,
            generation::default_workers(),
            &surfaces,
        );
    }
    analysis.inputs.extend(surface_render.inputs);
    analysis.referenced.extend(surface_render.referenced);
    extensions.articles = generation::resolved_articles(project, snapshot, &analysis);
    if let Some(css) = prepared
        .resources
        .get(&format!("{}dom-runtime.css", prepared.asset_prefix))
    {
        extensions.css.push_str(&String::from_utf8_lossy(css));
    }
    let mut requests = Vec::new();
    let mut locations = BTreeMap::new();
    for page in analysis.published() {
        let article = &extensions.articles[&page.route];
        for embed in &page.fragment.embeds {
            // The preflight already refused this embed, with its repair. A
            // reused generation must not render it as well and report the
            // same refusal a second time under the helper's code.
            if !declared(&manifest, embed) {
                continue;
            }
            let key = format!(
                "island-{}-{}",
                generation::fingerprint(page.route.as_bytes()),
                embed.index
            );
            let children = match children(article, embed) {
                Ok(children) => children,
                Err(message) => {
                    diagnostics.push(located(
                        snapshot,
                        page,
                        embed,
                        None,
                        "embed-marker",
                        message,
                    ));
                    continue;
                }
            };
            let target = match embed.declaration.kind {
                Kind::Component => RenderTarget::Component {
                    name: embed.declaration.name.clone(),
                    props: embed.declaration.props.clone(),
                    children_html: children,
                },
                Kind::Demo => RenderTarget::Demo {
                    name: embed.declaration.name.clone(),
                },
            };
            locations.insert(key.clone(), (page, embed));
            requests.push(RenderRequest {
                key: key.clone(),
                seed: key,
                target,
            });
        }
    }
    let rendered = match if requests.is_empty() {
        Ok(Vec::new())
    } else {
        prepared.render(environment.clone(), requests.clone())
    } {
        Ok(results) => results,
        Err(problems) => {
            diagnostics.extend(problems);
            return Build {
                analysis,
                extensions,
                diagnostics,
                info,
                guard: Some(prepared),
                theme,
            };
        }
    };
    let mut by_key: BTreeMap<_, _> = rendered
        .into_iter()
        .map(|item| (item.key.clone(), item))
        .collect();
    for mut request in requests {
        let (page, embed) = locations[&request.key];
        let Some(result) = by_key.remove(&request.key) else {
            continue;
        };
        for error in result.diagnostics {
            let repair = props_repair(project, &manifest, embed, &error);
            let diagnostic = located(
                snapshot,
                page,
                embed,
                error.property.as_deref(),
                &error.code,
                error.message,
            );
            diagnostics.push(match repair {
                Some(repair) => diagnostic.with_suggestion(repair),
                None => diagnostic,
            });
        }
        let Some(html) = result.html else {
            continue;
        };
        if let (RenderTarget::Component { props, .. }, Some(effective)) =
            (&mut request.target, result.props)
        {
            *props = effective;
        }
        let metadata = IslandMetadata {
            protocol_version: PROTOCOL_VERSION,
            key: request.key,
            seed: request.seed,
            target: request.target,
            environment: environment.clone(),
        };
        let source = match embed.declaration.kind {
            Kind::Demo => prepared.sources.get(&embed.declaration.name).map(|source| {
                rustify_docs::demo::Source {
                    path: &source.path,
                    function: &source.function,
                    file: &source.file,
                }
            }),
            Kind::Component => None,
        };
        let (labels, foreign) = rustify_docs::labels::for_lang(&page.lang);
        let block = rustify_docs::demo::render(
            &html,
            &metadata,
            source,
            highlighter,
            labels,
            foreign,
            result.client_only,
        );
        let article = extensions
            .articles
            .get_mut(&page.route)
            .expect("the resolved page");
        match replace_embed(article, embed, &block) {
            Ok(()) => {
                extensions.island_routes.insert(page.route.clone());
                extensions.css.push_str(&result.geometry_css);
            }
            Err(message) => diagnostics.push(located(
                snapshot,
                page,
                embed,
                None,
                "embed-marker",
                message,
            )),
        }
    }
    Build {
        analysis,
        extensions,
        diagnostics,
        info,
        guard: Some(prepared),
        theme,
    }
}

fn declared_pages(project: &Project, manifest: &Manifest) -> Surfaces {
    let file = project
        .config
        .rust
        .as_ref()
        .map(|rust| rust.registry.as_str())
        .unwrap_or_default();
    Surfaces {
        pages: manifest
            .pages
            .iter()
            .map(|page| RegisteredPage {
                name: page.name.clone(),
                route: page.route.clone(),
                title: page.title.clone(),
                description: page.description.clone(),
                html: String::new(),
                file: file.to_owned(),
            })
            .collect(),
        ..Default::default()
    }
}

#[derive(Default)]
struct SurfaceRender {
    diagnostics: Vec<Diagnostic>,
    inputs: content::InputReceipts,
    referenced: BTreeSet<String>,
}

fn render_surfaces(
    project: &Project,
    analysis: &Analysis,
    prepared: &Prepared,
    environment: &SiteEnvironment,
    surfaces: &mut Surfaces,
    extensions: &mut Extensions,
    content_read: (&Snapshot, &Highlighter),
) -> SurfaceRender {
    let (snapshot, highlighter) = content_read;
    let mut requests = Vec::new();
    for page in &surfaces.pages {
        let key = format!(
            "surface-{}-page-{}",
            generation::fingerprint(page.route.as_bytes()),
            page.name
        );
        requests.push(RenderRequest {
            seed: key.clone(),
            key,
            target: RenderTarget::Page {
                name: page.name.clone(),
                route: page.route.clone(),
            },
        });
    }
    let routes: BTreeSet<_> = analysis
        .published()
        .map(|page| page.route.as_str())
        .chain(surfaces.pages.iter().map(|page| page.route.as_str()))
        .chain(["/404/"])
        .collect();
    for route in routes {
        for slot in &prepared.manifest.slots {
            if slot.slot == rustify_docs_syntax::Slot::NotFound && route != "/404/" {
                continue;
            }
            let key = format!(
                "surface-{}-slot-{}",
                generation::fingerprint(route.as_bytes()),
                slot.slot.key()
            );
            requests.push(RenderRequest {
                seed: key.clone(),
                key,
                target: RenderTarget::Slot {
                    slot: site_slot(slot.slot),
                    route: route.to_owned(),
                },
            });
        }
    }
    if requests.is_empty() {
        return SurfaceRender::default();
    }
    let results = match prepared.render(environment.clone(), requests.clone()) {
        Ok(results) => results,
        Err(problems) => {
            return SurfaceRender {
                diagnostics: problems,
                ..SurfaceRender::default()
            }
        }
    };
    let mut results: BTreeMap<_, _> = results
        .into_iter()
        .map(|result| (result.key.clone(), result))
        .collect();
    let mut diagnostics = Vec::new();
    let mut source_inputs = content::InputReceipts::default();
    let mut source_referenced = BTreeSet::new();
    let file = &project
        .config
        .rust
        .as_ref()
        .expect("a Rust project")
        .registry;
    for request in requests {
        let result = results
            .remove(&request.key)
            .expect("a validated render key");
        let route = match &request.target {
            RenderTarget::Page { route, .. } | RenderTarget::Slot { route, .. } => route,
            _ => unreachable!(),
        };
        for error in result.diagnostics {
            diagnostics.push(Diagnostic::in_file(
                file,
                Severity::Error,
                &error.code,
                format!("{}: {}", literal(route), error.message),
            ));
        }
        let Some(mut html) = result.html else {
            continue;
        };
        extensions.css.push_str(&result.geometry_css);
        for intro in result.markdown_intros {
            source_referenced.insert(intro.path.clone());
            let marker = format!(
                "<span data-rustify-markdown-placeholder=\"{}\"></span>",
                intro.key
            );
            if html.matches(&marker).count() != 1 {
                diagnostics.push(Diagnostic::in_file(
                    file,
                    Severity::Error,
                    "markdown-intro-marker",
                    format!(
                        "{}: the Markdown introduction placeholder is missing or duplicated",
                        literal(route)
                    ),
                ));
                continue;
            }
            match content::markdown_intro(project, snapshot, analysis, &intro.path, highlighter) {
                Ok(intro) => {
                    source_inputs.extend(intro.inputs);
                    source_referenced.extend(intro.referenced);
                    diagnostics.extend(intro.diagnostics);
                    html = html.replacen(&marker, &intro.html, 1);
                }
                Err(problems) => diagnostics.extend(problems),
            }
        }
        for demo in result.demos {
            let Some(demo_html) = demo.html else {
                continue;
            };
            let metadata = IslandMetadata {
                protocol_version: PROTOCOL_VERSION,
                key: demo.key.clone(),
                seed: demo.seed,
                target: RenderTarget::Demo {
                    name: demo.name.clone(),
                },
                environment: environment.clone(),
            };
            let source =
                prepared
                    .sources
                    .get(&demo.name)
                    .map(|source| rustify_docs::demo::Source {
                        path: &source.path,
                        function: &source.function,
                        file: &source.file,
                    });
            let (labels, foreign) = rustify_docs::labels::for_lang(&environment.lang);
            let block = rustify_docs::demo::render(
                &demo_html,
                &metadata,
                source,
                highlighter,
                labels,
                foreign,
                false,
            );
            let marker = format!(
                "<span data-rustify-demo-placeholder=\"{}\"></span>",
                demo.key
            );
            if html.matches(&marker).count() != 1 {
                diagnostics.push(Diagnostic::in_file(
                    file,
                    Severity::Error,
                    "embed-marker",
                    format!(
                        "{}: the nested demo placeholder {} is missing or duplicated",
                        literal(route),
                        code_span(&demo.key)
                    ),
                ));
                continue;
            }
            html = html.replacen(&marker, &block, 1);
            extensions.island_routes.insert(route.clone());
            extensions.css.push_str(&demo.geometry_css);
        }
        match request.target {
            RenderTarget::Page { name, .. } => {
                surfaces
                    .pages
                    .iter_mut()
                    .find(|page| page.name == name)
                    .expect("a declared page")
                    .html = html;
            }
            RenderTarget::Slot { slot, route } => {
                surfaces.slots.entry(route).or_default().insert(slot, html);
            }
            _ => unreachable!(),
        }
    }
    SurfaceRender {
        diagnostics,
        inputs: source_inputs,
        referenced: source_referenced,
    }
}

fn site_slot(slot: rustify_docs_syntax::Slot) -> Slot {
    match slot {
        rustify_docs_syntax::Slot::Header => Slot::Header,
        rustify_docs_syntax::Slot::Footer => Slot::Footer,
        rustify_docs_syntax::Slot::SidebarTop => Slot::SidebarTop,
        rustify_docs_syntax::Slot::SidebarBottom => Slot::SidebarBottom,
        rustify_docs_syntax::Slot::BeforeContent => Slot::BeforeContent,
        rustify_docs_syntax::Slot::AfterContent => Slot::AfterContent,
        rustify_docs_syntax::Slot::NotFound => Slot::NotFound,
    }
}

pub(super) fn timing(timing: &CompileTiming) -> Value {
    json!({"elapsed_ms":timing.elapsed_ms,"completed_at_ms":timing.completed_at_ms,
        "fresh_artifacts":timing.fresh_artifacts,"compiled_artifacts":timing.compiled_artifacts,
        "profile":timing.profile,"target":timing.target,"compiler":timing.compiler,
        "flags":timing.flags,"success":timing.success})
}

fn preflight(
    project: &Project,
    snapshot: &Snapshot,
    analysis: &Analysis,
    manifest: Option<&Manifest>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let registry = registry_file(project);
    if let Some(manifest) = manifest {
        for component in &manifest.components {
            if rustify_docs::builtins::is_builtin(&component.name) {
                diagnostics.push(
                    Diagnostic::in_file(
                        registry,
                        Severity::Error,
                        "reserved-component",
                        format!(
                            "{} is a builtin component name; choose another registration name",
                            code_span(&component.name)
                        ),
                    )
                    .with_suggestion(embed::reserved_component(&component.name)),
                );
            }
        }
    }
    let components: BTreeMap<_, _> = manifest
        .into_iter()
        .flat_map(|manifest| &manifest.components)
        .map(|component| (component.name.as_str(), component))
        .collect();
    let demos: BTreeSet<_> = manifest
        .into_iter()
        .flat_map(|manifest| &manifest.demos)
        .map(|demo| demo.name.as_str())
        .collect();
    for page in analysis.published() {
        for embed in &page.fragment.embeds {
            let declaration = &embed.declaration;
            let name = declaration.name.as_str();
            match declaration.kind {
                Kind::Demo if !demos.contains(name) => {
                    let names: Vec<&str> = demos.iter().copied().collect();
                    diagnostics.push(
                        located(
                            snapshot,
                            page,
                            embed,
                            None,
                            "unregistered-demo",
                            format!("demo {} is not registered in site!", code_span(name)),
                        )
                        .with_suggestion(embed::unregistered_demo(&names)),
                    );
                }
                Kind::Component => match components.get(name) {
                    None => {
                        // Every name a `rustify-component` fence accepts: this
                        // generation's registrations and the builtins.
                        let builtins = rustify_docs::builtins::declarations();
                        let names: Vec<&str> = components
                            .keys()
                            .copied()
                            .chain(builtins.iter().map(|(name, _)| name.as_str()))
                            .collect();
                        diagnostics.push(
                            located(
                                snapshot,
                                page,
                                embed,
                                None,
                                "unregistered-component",
                                format!("component {} is not registered; add it to site! and configure [rust]", code_span(name)),
                            )
                            .with_suggestion(embed::unregistered_component(&names)),
                        );
                    }
                    Some(component) => {
                        for property in declaration.properties.keys() {
                            if component.fields.iter().any(|field| field.name == *property) {
                                continue;
                            }
                            let repair = embed::unknown_property(
                                name,
                                &fields(component),
                                Owner::Registered { registry },
                                property,
                            );
                            diagnostics.push(
                                located(
                                    snapshot,
                                    page,
                                    embed,
                                    Some(property),
                                    "unknown-property",
                                    format!(
                                        "component {} has no property {}",
                                        code_span(name),
                                        code_span(property)
                                    ),
                                )
                                .with_suggestion(repair),
                            );
                        }
                    }
                },
                Kind::Demo => {}
            }
        }
    }
    diagnostics
}

/// Whether `embed` names a registration of this generation and only its
/// declared props: exactly the embeds the preflight accepts.
fn declared(manifest: &Manifest, embed: &Embed) -> bool {
    let declaration = &embed.declaration;
    match declaration.kind {
        Kind::Demo => manifest
            .demos
            .iter()
            .any(|demo| demo.name == declaration.name),
        Kind::Component => manifest
            .components
            .iter()
            .find(|component| component.name == declaration.name)
            .is_some_and(|component| {
                declaration
                    .properties
                    .keys()
                    .all(|property| component.fields.iter().any(|field| field.name == *property))
            }),
    }
}

fn registry_file(project: &Project) -> &str {
    project
        .config
        .rust
        .as_ref()
        .map(|rust| rust.registry.as_str())
        .unwrap_or_default()
}

fn fields(component: &ComponentManifest) -> Vec<embed::Field> {
    component
        .fields
        .iter()
        .map(|field| embed::Field {
            name: field.name.clone(),
            ty: field.ty.clone(),
            default: field.default.clone(),
        })
        .collect()
}

/// The repair for props a registered component's declared types refused.
fn props_repair(
    project: &Project,
    manifest: &Manifest,
    embed: &Embed,
    error: &rustify_docs::site::SiteError,
) -> Option<String> {
    if error.code != "invalid_props" || embed.declaration.kind != Kind::Component {
        return None;
    }
    let component = manifest
        .components
        .iter()
        .find(|component| component.name == embed.declaration.name)?;
    Some(embed::invalid_props(
        &fields(component),
        Owner::Registered {
            registry: registry_file(project),
        },
        error.property.as_deref(),
    ))
}

fn bounds(article: &str, embed: &Embed) -> Result<(usize, usize, usize, usize), String> {
    let start = embed.start_marker();
    let end = embed.end_marker();
    if article.matches(&start).count() != 1 || article.matches(&end).count() != 1 {
        return Err("an embed marker is missing or duplicated".into());
    }
    let from = article.find(&start).expect("one start marker");
    let content = from + start.len();
    let to = article[content..]
        .find(&end)
        .map(|index| content + index)
        .ok_or("the embed markers are out of order")?;
    Ok((from, content, to, to + end.len()))
}

fn children(article: &str, embed: &Embed) -> Result<String, String> {
    let (_, from, to, _) = bounds(article, embed)?;
    Ok(article[from..to].to_owned())
}

fn replace_embed(article: &mut String, embed: &Embed, html: &str) -> Result<(), String> {
    let (from, _, _, to) = bounds(article, embed)?;
    article.replace_range(from..to, html);
    Ok(())
}

fn located(
    snapshot: &Snapshot,
    page: &Page,
    embed: &Embed,
    property: Option<&str>,
    rule: &str,
    message: impl Into<String>,
) -> Diagnostic {
    let field = property.and_then(|path| path.split(['.', '[']).next());
    let span = field
        .and_then(|field| embed.declaration.properties.get(field))
        .cloned()
        .unwrap_or_else(|| embed.declaration.span.clone());
    let text = &snapshot.markdown[&page.path];
    let (line, column) = LineIndex::new(text).position(span.start);
    Diagnostic {
        file: Some(page.file.clone()),
        line: Some(line),
        column: Some(column),
        severity: Severity::Error,
        rule: rule.to_owned(),
        message: message.into(),
        suggestion: None,
    }
}
