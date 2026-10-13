//! Connects one compiled Rust generation to the existing content pipeline.

use crate::rust_site::{self, BuildMode, CompileTiming, Prepared};
use rustify_docs::content::{Analysis, Diagnostic, Page, Project, Snapshot};
use rustify_docs::diagnostic::{LineIndex, Severity};
use rustify_docs::embed::Kind;
use rustify_docs::generation::{self, Extensions};
use rustify_docs::highlight::Highlighter;
use rustify_docs::markdown::Embed;
use rustify_docs::site::{
    IslandMetadata, RenderRequest, RenderTarget, SiteEnvironment, PROTOCOL_VERSION,
};
use rustify_docs_syntax::Manifest;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn build(
    project: &Project,
    snapshot: &Snapshot,
    analysis: &Analysis,
    highlighter: &Highlighter,
    mode: BuildMode,
) -> (Extensions, Vec<Diagnostic>, Value, Option<Prepared>) {
    let (prepared, mut diagnostics) = rust_site::prepare_checked(project, mode, |manifest| {
        preflight(project, snapshot, analysis, Some(manifest))
    });
    if project.config.rust.is_none() {
        diagnostics.extend(preflight(project, snapshot, analysis, None));
    }
    let Some(prepared) = prepared else {
        return (Extensions::default(), diagnostics, Value::Null, None);
    };
    let info = json!({
        "generation":prepared.generation,"framework_release":prepared.framework_release,
        "host":timing(&prepared.timings.host),"wasm":timing(&prepared.timings.wasm),
    });
    let mut extensions = Extensions {
        articles: generation::resolved_articles(project, snapshot, analysis),
        files: prepared.resources.clone(),
        candidates: prepared.candidates.iter().cloned().collect(),
        island_script: Some(format!("{}islands.js", prepared.asset_prefix)),
        ..Default::default()
    };
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
    let environment = SiteEnvironment {
        base: project.config.site.base.clone(),
        lang: project.config.site.lang.clone(),
        theme: None,
    };
    let rendered = match prepared.render(environment.clone(), requests.clone()) {
        Ok(results) => results,
        Err(problems) => {
            diagnostics.extend(problems);
            return (extensions, diagnostics, info, Some(prepared));
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
            diagnostics.push(located(
                snapshot,
                page,
                embed,
                error.property.as_deref(),
                &error.code,
                error.message,
            ));
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
    (extensions, diagnostics, info, Some(prepared))
}

fn timing(timing: &CompileTiming) -> Value {
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
    if let Some(manifest) = manifest {
        for component in &manifest.components {
            if rustify_docs::builtins::is_builtin(&component.name) {
                diagnostics.push(Diagnostic::in_file(
                    &project
                        .config
                        .rust
                        .as_ref()
                        .expect("the Rust project")
                        .registry,
                    Severity::Error,
                    "reserved-component",
                    format!(
                        "`{}` is a builtin component name; choose another registration name",
                        component.name
                    ),
                ));
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
            match declaration.kind {
                Kind::Demo if !demos.contains(declaration.name.as_str()) => diagnostics.push(located(snapshot,page,embed,None,"unregistered-demo",format!("demo `{}` is not registered in site!",declaration.name))),
                Kind::Component => match components.get(declaration.name.as_str()) {
                    None => diagnostics.push(located(snapshot,page,embed,None,"unregistered-component",format!("component `{}` is not registered; add it to site! and configure [rust]",declaration.name))),
                    Some(component) => {
                        for property in declaration.properties.keys() {
                            if !component.fields.iter().any(|field| field.name==*property) {
                                diagnostics.push(located(snapshot,page,embed,Some(property),"unknown-property",format!("component `{}` has no property `{property}`",declaration.name)));
                            }
                        }
                    }
                },
                Kind::Demo => {},
            }
        }
    }
    diagnostics
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
