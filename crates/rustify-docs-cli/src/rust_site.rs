//! The native Cargo, helper and bindgen boundary for one Rust-site generation.

use crate::rust_source::{DemoSource, Resolver};
use rustify_docs::content::{Diagnostic, Project};
use rustify_docs::diagnostic::{Issue, Severity};
use rustify_docs::diagnostic_text::{code_span, literal};
use rustify_docs::site::{
    Description, HelperOperation, HelperRequest, HelperResponse, HelperResult, RenderRequest,
    RenderResult, SiteEnvironment, PROTOCOL_VERSION,
};
use rustify_docs_syntax::{Manifest, SiteDeclaration};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::visit::Visit;

const BINDGEN_VERSION: &str = "0.2.128";
const FRAMEWORKS: [&str; 3] = ["rustify-docs", "rustify-ui", "rustify-components"];
const RUNTIME_ASSETS: [&str; 4] = [
    "dom-loader.js",
    "dom-runtime.css",
    "islands.js",
    "sdk-candidates.json",
];
const PAIR_ADVICE: &str = "Use the paired CLI and framework revisions together; restore both Cargo.toml/Cargo.lock and the paired CLI to roll back.";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BuildMode {
    Development,
    Release,
}

impl BuildMode {
    fn profiles(self) -> (&'static str, &'static str) {
        match self {
            Self::Development => ("docs-host", "docs-wasm"),
            Self::Release => ("docs-host-release", "release"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct CompileTiming {
    pub elapsed_ms: u64,
    pub completed_at_ms: u64,
    pub fresh_artifacts: usize,
    pub compiled_artifacts: usize,
    pub profile: String,
    pub target: Option<String>,
    pub compiler: String,
    pub flags: Vec<String>,
    pub success: bool,
}

#[derive(Clone, Debug)]
pub(super) struct BuildTimings {
    pub host: CompileTiming,
    pub wasm: CompileTiming,
}

#[derive(Clone, Debug, Default)]
pub(super) struct WatchPlan {
    pub rust_dirs: BTreeSet<PathBuf>,
    pub build_dirs: BTreeSet<PathBuf>,
    pub files: BTreeSet<PathBuf>,
    pub ignored: Vec<PathBuf>,
}

pub(super) enum Progress {
    Watch(WatchPlan),
    Host,
    Wasm,
    Render {
        reused: bool,
    },
    Compiled {
        generation: Option<String>,
        timings: BuildTimings,
    },
}

pub(super) type Observer<'a> = &'a (dyn Fn(Progress) + Sync);

#[derive(Clone)]
pub(super) struct Prepared {
    pub manifest: Manifest,
    pub generation: String,
    pub resources: BTreeMap<String, Arc<[u8]>>,
    pub asset_prefix: String,
    pub candidates: Vec<String>,
    pub sources: BTreeMap<String, DemoSource>,
    pub timings: BuildTimings,
    pub framework_release: String,
    helper: PathBuf,
    project_root: PathBuf,
    inputs: InputLayout,
    snapshot: InputSnapshot,
    compile_snapshot: InputSnapshot,
    rust: rustify_docs::config::RustConfig,
}

impl Prepared {
    /// Reuse only a proven compiled pair, then bind it to this round's reads.
    pub(super) fn reuse(&self, project: &Project) -> Result<Option<Self>, String> {
        if project.config.rust.as_ref() != Some(&self.rust)
            || fs::canonicalize(&project.root).ok().as_ref() != Some(&self.project_root)
            || std::env::var_os("RUSTIFY_DOCS_CARGO")
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| OsString::from("cargo"))
                != OsString::from(&self.timings.host.compiler)
        {
            return Ok(None);
        }
        let mut inputs = self.inputs.clone();
        for path in &inputs.render_files {
            if !inputs.compile_files.contains(path) {
                inputs.explicit.remove(path);
            }
        }
        inputs.render_files = BTreeSet::from([self.project_root.join(&project.config_file)]);
        if let Some((path, _)) = &project.config.theme_document {
            inputs.render_files.insert(self.project_root.join(path));
        }
        inputs.explicit.extend(inputs.render_files.iter().cloned());
        let out = self.project_root.join(&project.config.out_dir);
        if !inputs.excluded.contains(&out) {
            inputs.excluded.push(out);
        }
        let snapshot = inputs.capture()?;
        if inputs.compiled(&snapshot).digest != self.compile_snapshot.digest {
            return Ok(None);
        }
        let mut reused = self.clone();
        // This identity belongs to the compiled Rust/source pair. A content
        // save gets a fresh receipt without changing unrelated island markup.
        reused.inputs = inputs;
        reused.snapshot = snapshot;
        Ok(Some(reused))
    }

    pub(super) fn watch_plan(&self) -> WatchPlan {
        self.inputs.watch_plan()
    }

    pub(super) fn matches_input(&self, relative: &str, bytes: &[u8]) -> bool {
        fs::canonicalize(self.project_root.join(relative))
            .ok()
            .and_then(|path| self.snapshot.files.get(&path))
            .is_some_and(|captured| captured.as_ref() == bytes)
    }

    pub(super) fn render(
        &self,
        environment: SiteEnvironment,
        items: Vec<RenderRequest>,
    ) -> Result<Vec<RenderResult>, Vec<Diagnostic>> {
        self.validate_current()?;
        let request = HelperRequest {
            protocol_version: PROTOCOL_VERSION,
            generation: self.generation.clone(),
            operation: HelperOperation::Render {
                environment,
                items: items.clone(),
            },
        };
        let response = invoke_helper(&self.helper, &self.project_root, &request)
            .map_err(|error| vec![failure("rust-helper", error)])?;
        let results = match response.result {
            HelperResult::Render { items: results } => {
                validate_render_keys(&items, &results)
                    .map_err(|error| vec![failure("rust-helper-protocol", error)])?;
                validate_demo_names(&results, &self.manifest)
                    .map_err(|error| vec![failure("rust-helper-protocol", error)])?;
                results
            }
            _ => {
                return Err(vec![failure(
                    "rust-helper-protocol",
                    "helper returned describe data for a render request",
                )])
            }
        };
        self.validate_current()?;
        Ok(results)
    }

    /// Recheck the compiled inputs after composition, before publication.
    pub(super) fn validate_current(&self) -> Result<(), Vec<Diagnostic>> {
        self.check_inputs()
    }

    fn check_inputs(&self) -> Result<(), Vec<Diagnostic>> {
        let current = self
            .inputs
            .capture()
            .map_err(|error| vec![failure("rust-inputs", error)])?;
        if current.digest != self.snapshot.digest {
            return Err(vec![input_changes(
                &self.snapshot,
                &current,
                &self.project_root,
            )]);
        }
        Ok(())
    }
}

pub(super) fn prepare_checked_with(
    project: &Project,
    mode: BuildMode,
    preflight: impl FnOnce(&Manifest) -> Vec<Diagnostic>,
    observer: Observer<'_>,
) -> (Option<Prepared>, Vec<Diagnostic>) {
    let Some(rust) = &project.config.rust else {
        return (None, Vec::new());
    };
    let mut diagnostics = Vec::new();
    let project_root = match fs::canonicalize(&project.root) {
        Ok(root) => root,
        Err(error) => {
            return (
                None,
                vec![failure(
                    "rust-inputs",
                    format!("cannot open the author project: {error}"),
                )],
            )
        }
    };
    let mut initial_watch = WatchPlan {
        rust_dirs: BTreeSet::from([project_root
            .join(&rust.manifest)
            .parent()
            .unwrap_or(&project_root)
            .to_path_buf()]),
        files: BTreeSet::from([
            project_root.join(&rust.manifest),
            project_root.join(&rust.registry),
        ]),
        ignored: vec![
            project_root.join(&project.config.out_dir),
            project_root.join(".rustify-docs"),
        ],
        ..Default::default()
    };
    if let Ok(text) = fs::read_to_string(project_root.join(&rust.manifest)) {
        if let Ok(value) = toml::from_str::<toml::Value>(&text) {
            let manifest_root = project_root
                .join(&rust.manifest)
                .parent()
                .unwrap_or(&project_root)
                .to_path_buf();
            declared_dependency_paths(&value, &manifest_root, &mut initial_watch.rust_dirs);
        }
    }
    initial_watch.files.extend(
        initial_watch
            .rust_dirs
            .iter()
            .map(|root| root.join("Cargo.toml")),
    );
    observer(Progress::Watch(initial_watch));
    let manifest_path = match project_path(&project_root, &rust.manifest) {
        Ok(path) => path,
        Err(error) => {
            return (
                None,
                vec![Diagnostic::in_file(
                    &rust.manifest,
                    Severity::Error,
                    "rust-manifest",
                    error,
                )],
            )
        }
    };
    let registry_path = project_path(&project_root, &rust.registry).ok();
    let cache = project_root.join(".rustify-docs");
    let target = cache.join("target");
    let compiler = std::env::var_os("RUSTIFY_DOCS_CARGO")
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| OsString::from("cargo"));
    // Keep the SSR and hydrate graphs isolated, including target-only edges.
    let graphs: Vec<_> = [false, true]
        .into_iter()
        .map(|wasm| {
            match cargo_metadata(&compiler, &manifest_path, &target, &project_root, wasm)
                .and_then(|value| Metadata::read(value, &manifest_path))
            {
                Ok(metadata) => {
                    if let Err(error) =
                        validate_framework(&metadata, crate::EXPECTED_FRAMEWORK_REV.trim())
                    {
                        diagnostics.push(failure("rust-framework-pair", error));
                    }
                    Some(metadata)
                }
                Err(error) => {
                    diagnostics.push(failure("rust-metadata", error));
                    None
                }
            }
        })
        .collect();
    let metadata = graphs[0].as_ref();
    let hydrate_metadata = graphs[1].as_ref();
    let input_graphs: Vec<_> = graphs.iter().filter_map(Option::as_ref).collect();
    let mut inputs = match InputLayout::new(
        project,
        &project_root,
        &manifest_path,
        &target,
        &input_graphs,
        mode,
        &compiler,
    ) {
        Ok(inputs) => inputs,
        Err(error) => return (None, vec![failure("rust-inputs", error)]),
    };
    let mut snapshot = match inputs.capture() {
        Ok(snapshot) => Some(snapshot),
        Err(error) => {
            diagnostics.push(failure("rust-inputs", error));
            None
        }
    };
    let registry_text = registry_path.as_ref().and_then(|path| {
        snapshot
            .as_ref()
            .and_then(|snapshot| snapshot.files.get(path))
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
    });
    let declaration = match registry_text {
        Some(text) => match rustify_docs_syntax::parse_registry(text) {
            Ok(declaration) => Some(declaration),
            Err(errors) => {
                diagnostics.extend(errors.into_iter().map(|error| {
                    Diagnostic::located(
                        &rust.registry,
                        text,
                        Issue::error(
                            error.span().byte_range(),
                            "rust-registry",
                            error.to_string(),
                        ),
                    )
                }));
                None
            }
        },
        None => {
            diagnostics.push(Diagnostic::in_file(
                &rust.registry,
                Severity::Error,
                "rust-registry",
                "cannot read the configured UTF-8 site! registry inside the project",
            ));
            None
        }
    };
    if let Some(declaration) = &declaration {
        diagnostics.extend(preflight(&declaration.manifest()));
    }
    let (sources, source_files) = match (&declaration, metadata, &snapshot) {
        (Some(declaration), Some(metadata), Some(snapshot)) => demo_sources(
            declaration,
            metadata,
            &project_root,
            &snapshot.files,
            &rust.registry,
            registry_text.unwrap_or(""),
            &mut diagnostics,
        ),
        _ => (BTreeMap::new(), BTreeMap::new()),
    };
    inputs.explicit.extend(source_files.keys().cloned());
    inputs.compile_files.extend(source_files.keys().cloned());
    observer(Progress::Watch(inputs.watch_plan()));
    if let Some(snapshot) = &mut snapshot {
        snapshot.files.extend(source_files);
        snapshot.digest = inputs.digest(&snapshot.files, &snapshot.environment, &snapshot.bindings);
        match fs::canonicalize(project_root.join(&project.config_file)) {
            Ok(config) => match snapshot.files.get(&config) {
                Some(bytes) if bytes.as_ref() == project.config_text.as_bytes() => {}
                Some(_) => diagnostics.push(generation_changed()),
                None => diagnostics.push(failure(
                    "rust-inputs",
                    "the project configuration is missing from the captured build inputs",
                )),
            },
            Err(error) => diagnostics.push(failure(
                "rust-inputs",
                format!("cannot verify the project configuration: {error}"),
            )),
        }
        match inputs.capture() {
            Ok(current) if current.digest == snapshot.digest => {}
            Ok(current) => diagnostics.push(input_changes(snapshot, &current, &project_root)),
            Err(error) => diagnostics.push(failure("rust-inputs", error)),
        }
    }

    // Both targets contribute diagnostics even when the independent cold
    // registry or source checks have already failed.
    let (host_profile, wasm_profile) = mode.profiles();
    observer(Progress::Host);
    let host = cargo_build(
        &compiler,
        &manifest_path,
        &target,
        &project_root,
        host_profile,
        false,
        metadata,
    );
    // The immutable host artifact can describe itself while Cargo builds the
    // wasm target. Both compilers remain serial; nothing from this worker is
    // used until both builds and the captured-input checks have succeeded.
    let (wasm, description) = std::thread::scope(|scope| {
        let description = match (&host.artifact, &snapshot) {
            (Some(artifact), Some(snapshot))
                if host.timing.success
                    && !diagnostics
                        .iter()
                        .chain(&host.diagnostics)
                        .any(|diagnostic| diagnostic.severity == Severity::Error) =>
            {
                Some(
                    std::thread::Builder::new()
                        .name("rustify-docs-describe".into())
                        .spawn_scoped(scope, || -> Result<_, String> {
                            let helper = immutable_helper(&cache, artifact)?;
                            let response = invoke_helper(
                                &helper,
                                &project_root,
                                &HelperRequest {
                                    protocol_version: PROTOCOL_VERSION,
                                    generation: snapshot.digest.clone(),
                                    operation: HelperOperation::Describe,
                                },
                            )?;
                            let HelperResult::Describe { description } = response.result else {
                                return Err(
                                    "helper returned render data for a describe request".into()
                                );
                            };
                            Ok((helper, description))
                        }),
                )
            }
            _ => None,
        };
        observer(Progress::Wasm);
        let wasm = cargo_build(
            &compiler,
            &manifest_path,
            &target,
            &project_root,
            wasm_profile,
            true,
            hydrate_metadata,
        );
        let description = description.map(|worker| {
            worker
                .map_err(|error| format!("cannot prepare docs-render on a thread: {error}"))?
                .join()
                .map_err(|_| "docs-render preparation thread failed".to_string())?
        });
        (wasm, description)
    });
    observer(Progress::Compiled {
        generation: snapshot.as_ref().map(|snapshot| snapshot.digest.clone()),
        timings: BuildTimings {
            host: host.timing.clone(),
            wasm: wasm.timing.clone(),
        },
    });
    diagnostics.extend(host.diagnostics);
    diagnostics.extend(wasm.diagnostics);
    if let Err(error) = inputs.discover_build_inputs() {
        diagnostics.push(failure("rust-inputs", error));
    }
    observer(Progress::Watch(inputs.watch_plan()));
    if let Some(snapshot) = &snapshot {
        match inputs.capture() {
            Ok(after) if after.digest == snapshot.digest => {}
            Ok(after) => diagnostics.push(input_changes(snapshot, &after, &project_root)),
            Err(error) => diagnostics.push(failure("rust-inputs", error)),
        }
    }
    if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
    {
        return (None, diagnostics);
    }
    let (Some(declaration), Some(metadata), Some(snapshot), Some(description), Some(wasm_path)) =
        (declaration, metadata, snapshot, description, wasm.artifact)
    else {
        diagnostics.push(failure(
            "rust-artifact",
            "Cargo succeeded without the expected docs-render executable and cdylib wasm artifacts",
        ));
        return (None, diagnostics);
    };
    let generation = snapshot.digest.clone();
    let result = (|| -> Result<Prepared, String> {
        let (helper, description) = description?;
        let manifest = declaration.manifest();
        let runtime = validate_description(&description, &manifest, &metadata.framework_release)?;
        let mut candidates: BTreeSet<String> =
            serde_json::from_slice::<Vec<String>>(&runtime["sdk-candidates.json"])
                .map_err(|error| format!("invalid SDK candidate resource: {error}"))?
                .into_iter()
                .collect();
        for path in host.sources.union(&wasm.sources) {
            if let Some(bytes) = snapshot.files.get(path) {
                let text = std::str::from_utf8(bytes)
                    .map_err(|error| format!("Rust class source is not UTF-8: {error}"))?;
                candidates.extend(product_class_candidates(text).map_err(|error| {
                    format!(
                        "cannot scan compiled Rust class source {}: {error}",
                        literal(&path.display().to_string())
                    )
                })?);
            }
        }
        let mut files = bindgen_files(&cache, &wasm_path, &runtime)?;
        for (path, bytes) in runtime {
            // Candidate metadata is consumed by the compiler, not a reader
            // asset. The whole reader runtime and every snippet stay together.
            if path == "sdk-candidates.json" {
                continue;
            }
            if files.insert(path.clone(), bytes).is_some() {
                return Err(format!(
                    "framework runtime resource conflicts with generated binding {}",
                    code_span(&path)
                ));
            }
        }
        let group = resource_digest(&files);
        let asset_prefix = format!("assets/islands/{group}/");
        let resources = files
            .into_iter()
            .map(|(path, bytes)| (format!("{asset_prefix}{path}"), bytes))
            .collect();
        let compile_snapshot = inputs.compiled(&snapshot);
        Ok(Prepared {
            manifest,
            generation,
            resources,
            asset_prefix,
            candidates: candidates.into_iter().collect(),
            sources,
            timings: BuildTimings {
                host: host.timing,
                wasm: wasm.timing,
            },
            framework_release: metadata.framework_release.clone(),
            helper,
            project_root,
            inputs,
            snapshot,
            compile_snapshot,
            rust: rust.clone(),
        })
    })();
    match result {
        Ok(prepared) => match prepared.validate_current() {
            Ok(()) => (Some(prepared), diagnostics),
            Err(problems) => {
                diagnostics.extend(problems);
                (None, diagnostics)
            }
        },
        Err(error) => {
            diagnostics.push(failure("rust-prepare", error));
            (None, diagnostics)
        }
    }
}

struct Metadata {
    value: Value,
    package_id: String,
    library_name: String,
    library_source: PathBuf,
    framework_release: String,
    local_roots: BTreeSet<PathBuf>,
}

impl Metadata {
    fn read(mut value: Value, manifest: &Path) -> Result<Self, String> {
        let packages = value
            .get("packages")
            .and_then(Value::as_array)
            .ok_or("Cargo metadata has no packages")?;
        let package = packages.iter().find(|package| {
            package.get("manifest_path").and_then(Value::as_str)
                .and_then(|path| fs::canonicalize(path).ok()).as_deref() == Some(manifest)
        }).ok_or("the configured Cargo.toml must identify an author package, not a virtual workspace")?;
        let targets = package
            .get("targets")
            .and_then(Value::as_array)
            .ok_or("the author package has no Cargo targets")?;
        if !targets.iter().any(|target| {
            target.get("name").and_then(Value::as_str) == Some("docs-render")
                && has_string(target.get("kind"), "bin")
        }) {
            return Err("the author package needs a docs-render host binary; use the complete component-site template".to_string());
        }
        let library = targets.iter().find(|target| has_string(target.get("crate_types"), "cdylib"))
            .ok_or("the author package needs a cdylib library target; use the complete component-site template")?;
        let library_name = required_string(library, "name")?.to_string();
        let library_source = PathBuf::from(required_string(library, "src_path")?);
        let package_id = required_string(package, "id")?.to_string();
        let direct = value
            .pointer("/resolve/nodes")
            .and_then(Value::as_array)
            .and_then(|nodes| {
                nodes.iter().find(|node| {
                    node.get("id").and_then(Value::as_str) == Some(package_id.as_str())
                })
            })
            .and_then(|node| node.get("dependencies"))
            .and_then(Value::as_array)
            .ok_or("Cargo metadata has no resolved author dependency graph")?;
        let docs = packages
            .iter()
            .find(|package| {
                package.get("name").and_then(Value::as_str) == Some("rustify-docs")
                    && direct
                        .iter()
                        .any(|id| id.as_str() == package.get("id").and_then(Value::as_str))
            })
            .ok_or("the author package must depend on rustify-docs")?;
        let framework_release = format!("rustify-docs@{}", required_string(docs, "version")?);
        // Cargo's packages list can include target-inactive packages. Only
        // traverse edges from the selected author in this filtered resolve graph.
        let nodes = value
            .pointer("/resolve/nodes")
            .and_then(Value::as_array)
            .ok_or("Cargo metadata has no resolved author dependency graph")?;
        let nodes: BTreeMap<_, _> = nodes
            .iter()
            .filter_map(|node| node.get("id").and_then(Value::as_str).map(|id| (id, node)))
            .collect();
        let mut active = BTreeSet::new();
        let mut pending = vec![package_id.as_str()];
        while let Some(id) = pending.pop() {
            if !active.insert(id.to_string()) {
                continue;
            }
            if let Some(dependencies) = nodes
                .get(id)
                .and_then(|node| node.get("dependencies"))
                .and_then(Value::as_array)
            {
                pending.extend(dependencies.iter().filter_map(Value::as_str));
            }
        }
        if let Some(packages) = value.get_mut("packages").and_then(Value::as_array_mut) {
            packages.retain(|package| {
                package
                    .get("id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| active.contains(id))
            });
        }
        if let Some(nodes) = value
            .pointer_mut("/resolve/nodes")
            .and_then(Value::as_array_mut)
        {
            nodes.retain(|node| {
                node.get("id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| active.contains(id))
            });
        }
        let packages = value
            .get("packages")
            .and_then(Value::as_array)
            .ok_or("Cargo metadata has no packages")?;
        let mut local_roots = BTreeSet::new();
        for package in packages
            .iter()
            .filter(|package| package.get("source").is_none_or(Value::is_null))
        {
            let manifest = fs::canonicalize(required_string(package, "manifest_path")?)
                .map_err(|error| format!("cannot open a local dependency manifest: {error}"))?;
            if let Some(root) = manifest.parent() {
                local_roots.insert(root.to_path_buf());
            }
        }
        Ok(Self {
            value,
            package_id,
            library_name,
            library_source,
            framework_release,
            local_roots,
        })
    }
}

fn validate_framework(metadata: &Metadata, expected: &str) -> Result<(), String> {
    let packages = metadata
        .value
        .get("packages")
        .and_then(Value::as_array)
        .ok_or("Cargo metadata has no packages")?;
    let direct = metadata
        .value
        .pointer("/resolve/nodes")
        .and_then(Value::as_array)
        .and_then(|nodes| {
            nodes.iter().find(|node| {
                node.get("id").and_then(Value::as_str) == Some(metadata.package_id.as_str())
            })
        })
        .and_then(|node| node.get("dependencies"))
        .and_then(Value::as_array)
        .ok_or("Cargo metadata has no resolved author dependency graph")?;
    let mut framework = Vec::new();
    for name in FRAMEWORKS {
        let matches: Vec<_> = packages
            .iter()
            .filter(|package| {
                package.get("name").and_then(Value::as_str) == Some(name)
                    && direct
                        .iter()
                        .any(|id| id.as_str() == package.get("id").and_then(Value::as_str))
            })
            .collect();
        let [package] = matches.as_slice() else {
            return Err(format!(
                "Cargo.toml/Cargo.lock must resolve one direct {} dependency. {PAIR_ADVICE}",
                code_span(name)
            ));
        };
        framework.push(*package);
    }
    let source = framework[0].get("source").and_then(Value::as_str);
    if framework
        .iter()
        .any(|package| package.get("source").and_then(Value::as_str) != source)
    {
        return Err(format!("the three framework packages have different sources/revisions in Cargo.lock. {PAIR_ADVICE}"));
    }
    match source {
        Some(source) if source.starts_with("git+") => {
            let resolved = source.rsplit_once('#').map(|(_, revision)| revision).unwrap_or("");
            let requested = source.split_once('?').and_then(|(_, query)| query.split('#').next())
                .and_then(|query| query.split('&').find_map(|part| part.strip_prefix("rev="))).unwrap_or("");
            if expected.is_empty() || resolved != expected || requested != expected {
                return Err(format!("Cargo.toml/Cargo.lock framework revision does not match this CLI's explicit expected revision {}. {PAIR_ADVICE}", code_span(expected)));
            }
        }
        None if expected.is_empty() => {
            let repository = Path::new(env!("CARGO_MANIFEST_DIR")).parent().and_then(Path::parent)
                .ok_or("cannot identify the compiled development CLI repository")?;
            for package in framework {
                let name = required_string(package, "name")?;
                let expected = fs::canonicalize(repository.join("crates").join(name).join("Cargo.toml"))
                    .map_err(|error| format!("cannot verify the compiled CLI's framework path: {error}"))?;
                let actual = fs::canonicalize(required_string(package, "manifest_path")?)
                    .map_err(|error| format!("cannot verify a framework manifest: {error}"))?;
                if actual != expected { return Err(format!("local framework paths must match the exact repository compiled into this development CLI. {PAIR_ADVICE}")); }
            }
        }
        _ => return Err(format!("framework dependencies need the explicit git revision paired with this CLI; local paths are only supported by its own development fixture. {PAIR_ADVICE}")),
    }
    let bindings: Vec<_> = packages
        .iter()
        .filter(|package| package.get("name").and_then(Value::as_str) == Some("wasm-bindgen"))
        .collect();
    if bindings.is_empty()
        || bindings
            .iter()
            .any(|package| package.get("version").and_then(Value::as_str) != Some(BINDGEN_VERSION))
    {
        return Err(format!("Cargo.lock must resolve wasm-bindgen exactly {BINDGEN_VERSION} for the paired framework. {PAIR_ADVICE}"));
    }
    Ok(())
}

type CapturedFiles = BTreeMap<PathBuf, Arc<[u8]>>;

fn demo_sources(
    declaration: &SiteDeclaration,
    metadata: &Metadata,
    root: &Path,
    files: &CapturedFiles,
    registry_path: &str,
    registry_text: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> (BTreeMap<String, DemoSource>, CapturedFiles) {
    let mut sources = BTreeMap::new();
    let mut resolver = match Resolver::from_snapshot(root, &metadata.library_source, files) {
        Ok(resolver) => resolver,
        Err(error) => {
            diagnostics.push(failure("rust-demo-source", error));
            return (sources, BTreeMap::new());
        }
    };
    for demo in &declaration.demos {
        let path = &demo.function.path;
        let parts: Vec<_> = path
            .segments
            .iter()
            .map(|part| part.ident.to_string())
            .collect();
        let prefix = parts.first().map(String::as_str).unwrap_or("");
        let rooted = (prefix == "crate" || prefix == metadata.library_name)
            && demo.function.qself.is_none()
            && path
                .segments
                .iter()
                .all(|part| matches!(part.arguments, syn::PathArguments::None));
        let source = if !rooted {
            Err("reference the direct site source with crate::module::function; self:: and bare paths cannot identify the declaration's lexical module".to_string())
        } else {
            let symbol = std::iter::once("crate".to_string())
                .chain(parts.into_iter().skip(1))
                .collect::<Vec<_>>()
                .join("::");
            resolver.resolve(&symbol)
        };
        match source {
            Ok(source) => {
                sources.insert(demo.name.unraw().to_string(), source);
            }
            Err(error) => diagnostics.push(Diagnostic::located(
                registry_path,
                registry_text,
                Issue::error(demo.function.span().byte_range(), "rust-demo-source", error),
            )),
        }
    }
    (sources, resolver.captured_files())
}

/// Keep recovery watches even when metadata cannot load a missing dependency.
fn declared_dependency_paths(value: &toml::Value, root: &Path, paths: &mut BTreeSet<PathBuf>) {
    if let Some(table) = value.as_table() {
        for (key, value) in table {
            if matches!(
                key.as_str(),
                "dependencies" | "build-dependencies" | "dev-dependencies"
            ) {
                if let Some(dependencies) = value.as_table() {
                    for dependency in dependencies.values() {
                        if let Some(path) = dependency.get("path").and_then(toml::Value::as_str) {
                            let mut normalized = PathBuf::new();
                            for component in root.join(path).components() {
                                match component {
                                    Component::ParentDir => {
                                        normalized.pop();
                                    }
                                    Component::CurDir => {}
                                    component => normalized.push(component.as_os_str()),
                                }
                            }
                            paths.insert(normalized);
                        }
                    }
                }
            } else if value.is_table() {
                declared_dependency_paths(value, root, paths);
            }
        }
    }
}

fn cargo_metadata(
    compiler: &OsString,
    manifest: &Path,
    target: &Path,
    root: &Path,
    wasm: bool,
) -> Result<Value, String> {
    let platform = if wasm {
        "wasm32-unknown-unknown".to_string()
    } else {
        let output =
            Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc")))
                .current_dir(root)
                .arg("-vV")
                .output()
                .map_err(|error| format!("cannot identify the host Rust target: {error}"))?;
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .find_map(|line| line.strip_prefix("host: "))
            .map(str::to_owned)
            .ok_or("rustc did not report the host target")?
    };
    let feature = if wasm { "hydrate" } else { "ssr" };
    let mut command = Command::new(compiler);
    command
        .current_dir(root)
        .env("CARGO_TARGET_DIR", target)
        .env("CARGO_TERM_COLOR", "never")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .args([
            "metadata",
            "--locked",
            "--format-version",
            "1",
            "--no-default-features",
            "--features",
            feature,
            "--filter-platform",
            &platform,
            "--manifest-path",
        ])
        .arg(manifest);
    command.arg("--offline");
    let mut output = command.output().map_err(|error| {
        format!("cannot run Cargo metadata: {error}; install/activate Rust and Cargo, then retry")
    })?;
    if !output.status.success()
        && !matches!(
            std::env::var("CARGO_NET_OFFLINE").as_deref(),
            Ok("true" | "1")
        )
    {
        // The locked dependency graph may not have been fetched yet. This is
        // normal Cargo resolution, without installing any toolchain or service.
        let mut online = Command::new(compiler);
        online
            .current_dir(root)
            .env("CARGO_TARGET_DIR", target)
            .env("CARGO_TERM_COLOR", "never")
            .env("RUSTUP_AUTO_INSTALL", "0")
            .args([
                "metadata",
                "--locked",
                "--format-version",
                "1",
                "--no-default-features",
                "--features",
                feature,
                "--filter-platform",
                &platform,
                "--manifest-path",
            ])
            .arg(manifest);
        output = online
            .output()
            .map_err(|error| format!("cannot run Cargo metadata: {error}"))?;
    }
    if !output.status.success() {
        return Err(format!(
            "Cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("invalid Cargo metadata JSON: {error}"))
}

struct Compilation {
    artifact: Option<PathBuf>,
    sources: BTreeSet<PathBuf>,
    diagnostics: Vec<Diagnostic>,
    timing: CompileTiming,
}

fn cargo_build(
    compiler: &OsString,
    manifest: &Path,
    target: &Path,
    root: &Path,
    profile: &str,
    wasm: bool,
    metadata: Option<&Metadata>,
) -> Compilation {
    let began = Instant::now();
    let mut command = Command::new(compiler);
    command
        .current_dir(root)
        .env("CARGO_TERM_COLOR", "never")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .args([
            "build",
            "--locked",
            "--message-format",
            "json",
            "--no-default-features",
            "--profile",
            profile,
            "--features",
            if wasm { "hydrate" } else { "ssr" },
            "--manifest-path",
        ])
        .arg(manifest)
        .arg("--target-dir")
        .arg(target);
    if wasm {
        command.args(["--lib", "--target", "wasm32-unknown-unknown"]);
    } else {
        command.args(["--bin", "docs-render"]);
    }
    let output = command.output();
    let mut timing = CompileTiming {
        elapsed_ms: began.elapsed().as_millis() as u64,
        completed_at_ms: epoch_ms(),
        profile: profile.to_string(),
        target: wasm.then(|| "wasm32-unknown-unknown".to_string()),
        compiler: compiler.to_string_lossy().into_owned(),
        flags: command
            .get_args()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect(),
        ..Default::default()
    };
    let mut diagnostics = Vec::new();
    let mut artifact = None;
    let mut sources = BTreeSet::new();
    match output {
        Err(error) => diagnostics.push(failure("rust-cargo", format!("cannot run the {} compiler: {error}; install/activate Rust and the wasm32-unknown-unknown target, then retry", if wasm { "wasm" } else { "host" }))),
        Ok(output) => {
            timing.success = output.status.success();
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                let Ok(message) = serde_json::from_str::<Value>(line) else { continue; };
                match message.get("reason").and_then(Value::as_str) {
                    Some("compiler-message") => { if let Some(message) = message.get("message") { diagnostics.push(compiler_diagnostic(message, root)); } }
                    Some("compiler-artifact") => {
                        if message.get("fresh").and_then(Value::as_bool) == Some(true) { timing.fresh_artifacts += 1; } else { timing.compiled_artifacts += 1; }
                        let source = message.pointer("/target/src_path").and_then(Value::as_str).map(Path::new);
                        let local_root = source.and_then(|source| metadata.and_then(|metadata| metadata.local_roots.iter().filter(|root| source.starts_with(root)).max_by_key(|root| root.components().count())));
                        if let Some(local_root) = local_root {
                            match compiled_sources(&message, local_root) {
                                Ok(files) => sources.extend(files),
                                Err(error) => diagnostics.push(failure("rust-source-inputs", error)),
                            }
                        }
                        let owned = metadata.is_some_and(|metadata| message.get("package_id").and_then(Value::as_str) == Some(metadata.package_id.as_str()));
                        if owned && wasm && message.pointer("/target/name").and_then(Value::as_str) == metadata.map(|metadata| metadata.library_name.as_str()) {
                            artifact = message.get("filenames").and_then(Value::as_array).and_then(|files| files.iter().filter_map(Value::as_str).find(|path| path.ends_with(".wasm"))).map(PathBuf::from);
                        } else if owned && !wasm && message.pointer("/target/name").and_then(Value::as_str) == Some("docs-render") {
                            artifact = message.get("executable").and_then(Value::as_str).map(PathBuf::from);
                        }
                    }
                    _ => {}
                }
            }
            if !output.status.success() && !diagnostics.iter().any(|diagnostic| diagnostic.severity == Severity::Error) {
                diagnostics.push(failure("rust-cargo", format!("{} Cargo build failed: {}", if wasm { "wasm" } else { "host" }, String::from_utf8_lossy(&output.stderr).trim())));
            }
            let logs = target.parent().unwrap_or(target).join("logs");
            let name = if wasm { "wasm" } else { "host" };
            // The next input capture needs these package_id/OUT_DIR receipts.
            // A missing receipt must not silently drop build-script inputs.
            if let Err(error) = fs::create_dir_all(&logs).and_then(|()| {
                fs::write(logs.join(format!("{profile}-{name}.stdout")), &output.stdout)
            }) {
                diagnostics.push(failure("rust-inputs", format!("cannot save Cargo build-input identities: {error}")));
            }
            let _ = fs::write(logs.join(format!("{profile}-{name}.stderr")), &output.stderr);
            let _ = record_compile_history(&logs, &timing, &output.stdout, &output.stderr);
        }
    }
    Compilation {
        artifact,
        sources,
        diagnostics,
        timing,
    }
}

fn record_compile_history(
    logs: &Path,
    timing: &CompileTiming,
    stdout: &[u8],
    stderr: &[u8],
) -> std::io::Result<()> {
    let target = timing.target.as_deref().unwrap_or("host");
    let directory = logs.join("history").join(format!(
        "{}-{}-{target}-{}-{}",
        timing.completed_at_ms,
        timing.profile,
        std::process::id(),
        next_temporary(),
    ));
    fs::create_dir_all(&directory)?;
    fs::write(directory.join("stdout"), stdout)?;
    fs::write(directory.join("stderr"), stderr)?;
    let timing = json!({
        "elapsed_ms":timing.elapsed_ms,"completed_at_ms":timing.completed_at_ms,
        "fresh_artifacts":timing.fresh_artifacts,"compiled_artifacts":timing.compiled_artifacts,
        "profile":timing.profile,"target":timing.target,"compiler":timing.compiler,
        "flags":timing.flags,"success":timing.success,
    });
    let bytes = serde_json::to_vec(&timing).map_err(std::io::Error::other)?;
    fs::write(directory.join("timing.json"), bytes)
}

/// Use only dep-info belonging to artifacts Cargo reported in this build;
/// unrelated examples, tests and stale target files are not class sources.
fn compiled_sources(message: &Value, package_root: &Path) -> Result<BTreeSet<PathBuf>, String> {
    let mut dep_info = BTreeSet::new();
    for filename in message
        .get("filenames")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        let filename = Path::new(filename);
        dep_info.insert(filename.with_extension("d"));
        if has_string(message.pointer("/target/kind"), "custom-build") {
            if let Some(directory) = filename.parent() {
                for entry in fs::read_dir(directory).map_err(|error| error.to_string())? {
                    let path = entry.map_err(|error| error.to_string())?.path();
                    if path.extension().is_some_and(|extension| extension == "d") {
                        dep_info.insert(path);
                    }
                }
            }
        }
        if matches!(
            filename
                .extension()
                .and_then(|extension| extension.to_str()),
            Some("rlib" | "rmeta" | "dylib" | "so" | "dll")
        ) {
            if let Some(stem) = filename
                .file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|stem| stem.strip_prefix("lib"))
            {
                dep_info.insert(filename.with_file_name(stem).with_extension("d"));
            }
        }
    }
    let mut found = false;
    let mut sources = BTreeSet::new();
    for path in dep_info.into_iter().filter(|path| path.is_file()) {
        found = true;
        let text = fs::read_to_string(&path).map_err(|error| {
            format!(
                "cannot read compiled Rust dep-info {}: {error}",
                literal(&path.display().to_string())
            )
        })?;
        for source in dep_info_paths(&text) {
            let path = package_root.join(source);
            if path.extension().is_some_and(|extension| extension == "rs") {
                if let Ok(path) = fs::canonicalize(path) {
                    sources.insert(path);
                }
            }
        }
    }
    if !found {
        return Err(format!("Cargo did not leave dep-info for compiled local target {}; class scanning needs its actual source set", code_span(message.pointer("/target/name").and_then(Value::as_str).unwrap_or("unknown"))));
    }
    Ok(sources)
}

fn product_class_candidates(text: &str) -> Result<Vec<String>, String> {
    let syntax = syn::parse_file(text).map_err(|error| error.to_string())?;
    let offset = usize::from(text.starts_with('\u{feff}')) * '\u{feff}'.len_utf8()
        + syntax.shebang.as_ref().map_or(0, String::len);
    let mut tests = TestItems::default();
    tests.visit_file(&syntax);
    let mut product = text.as_bytes().to_vec();
    for range in tests.ranges {
        let range = range.start + offset..range.end + offset;
        let bytes = product
            .get_mut(range)
            .ok_or("test-only source span is outside the captured Rust file")?;
        for byte in bytes {
            if !matches!(*byte, b'\r' | b'\n') {
                *byte = b' ';
            }
        }
    }
    let product = std::str::from_utf8(&product).map_err(|error| error.to_string())?;
    Ok(rustify_style::scanner::scan(product, "rs").candidates)
}

#[derive(Default)]
struct TestItems {
    ranges: Vec<std::ops::Range<usize>>,
}

impl TestItems {
    fn exclude(&mut self, attrs: &[syn::Attribute], span: proc_macro2::Span) -> bool {
        let test_only = attrs.iter().any(|attribute| {
            attribute.path().is_ident("test")
                || (attribute.path().is_ident("cfg")
                    && attribute
                        .parse_args::<syn::Meta>()
                        .ok()
                        .and_then(|meta| non_test_cfg(&meta))
                        == Some(false))
        });
        if test_only {
            self.ranges.push(span.byte_range());
        }
        test_only
    }
}

impl<'ast> Visit<'ast> for TestItems {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        let attrs = match item {
            syn::Item::Const(item) => &item.attrs,
            syn::Item::Enum(item) => &item.attrs,
            syn::Item::ExternCrate(item) => &item.attrs,
            syn::Item::Fn(item) => &item.attrs,
            syn::Item::ForeignMod(item) => &item.attrs,
            syn::Item::Impl(item) => &item.attrs,
            syn::Item::Macro(item) => &item.attrs,
            syn::Item::Mod(item) => &item.attrs,
            syn::Item::Static(item) => &item.attrs,
            syn::Item::Struct(item) => &item.attrs,
            syn::Item::Trait(item) => &item.attrs,
            syn::Item::TraitAlias(item) => &item.attrs,
            syn::Item::Type(item) => &item.attrs,
            syn::Item::Union(item) => &item.attrs,
            syn::Item::Use(item) => &item.attrs,
            _ => return,
        };
        if !self.exclude(attrs, item.span()) {
            syn::visit::visit_item(self, item);
        }
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        let attrs = match item {
            syn::ImplItem::Const(item) => &item.attrs,
            syn::ImplItem::Fn(item) => &item.attrs,
            syn::ImplItem::Type(item) => &item.attrs,
            syn::ImplItem::Macro(item) => &item.attrs,
            _ => return,
        };
        if !self.exclude(attrs, item.span()) {
            syn::visit::visit_impl_item(self, item);
        }
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        let attrs = match item {
            syn::TraitItem::Const(item) => &item.attrs,
            syn::TraitItem::Fn(item) => &item.attrs,
            syn::TraitItem::Type(item) => &item.attrs,
            syn::TraitItem::Macro(item) => &item.attrs,
            _ => return,
        };
        if !self.exclude(attrs, item.span()) {
            syn::visit::visit_trait_item(self, item);
        }
    }
}

/// Unknown feature/target predicates stay eligible. Only a predicate that is
/// certainly false without `test` removes a source contribution.
fn non_test_cfg(meta: &syn::Meta) -> Option<bool> {
    if meta.path().is_ident("test") {
        return Some(false);
    }
    let syn::Meta::List(list) = meta else {
        return None;
    };
    let items = list
        .parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .ok()?;
    let values: Vec<_> = items.iter().map(non_test_cfg).collect();
    if list.path.is_ident("all") {
        if values.contains(&Some(false)) {
            Some(false)
        } else if values.iter().all(|value| *value == Some(true)) {
            Some(true)
        } else {
            None
        }
    } else if list.path.is_ident("any") {
        if values.contains(&Some(true)) {
            Some(true)
        } else if values.iter().all(|value| *value == Some(false)) {
            Some(false)
        } else {
            None
        }
    } else if list.path.is_ident("not") && values.len() == 1 {
        values[0].map(|value| !value)
    } else {
        None
    }
}

fn compiler_diagnostic(message: &Value, root: &Path) -> Diagnostic {
    let level = message
        .get("level")
        .and_then(Value::as_str)
        .unwrap_or("error");
    let severity = if matches!(level, "error" | "failure-note" | "ice") {
        Severity::Error
    } else {
        Severity::Warning
    };
    let mut diagnostic = Diagnostic::general(
        severity,
        "rust-compile",
        message
            .get("rendered")
            .and_then(Value::as_str)
            .or_else(|| message.get("message").and_then(Value::as_str))
            .unwrap_or("Rust compiler diagnostic"),
    );
    if let Some(span) = message
        .get("spans")
        .and_then(Value::as_array)
        .and_then(|spans| {
            spans
                .iter()
                .find(|span| span.get("is_primary").and_then(Value::as_bool) == Some(true))
        })
    {
        if let Some(file) = span.get("file_name").and_then(Value::as_str) {
            let path = Path::new(file);
            let path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                root.join(path)
            };
            diagnostic.file = Some(
                path.strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        diagnostic.line = span
            .get("line_start")
            .and_then(Value::as_u64)
            .map(|value| value as usize);
        diagnostic.column = span
            .get("column_start")
            .and_then(Value::as_u64)
            .map(|value| value as usize);
    }
    let help: Vec<_> = message
        .get("children")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|child| {
            matches!(
                child.get("level").and_then(Value::as_str),
                Some("help" | "note")
            )
        })
        .filter_map(|child| child.get("message").and_then(Value::as_str))
        .collect();
    if !help.is_empty() {
        diagnostic.suggestion = Some(help.join("\n"));
    }
    diagnostic
}

fn invoke_helper(
    helper: &Path,
    root: &Path,
    request: &HelperRequest,
) -> Result<HelperResponse, String> {
    let bytes = serde_json::to_vec(request)
        .map_err(|error| format!("cannot serialize helper request: {error}"))?;
    let mut child = Command::new(helper)
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot start docs-render: {error}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or("docs-render stdout was not piped")?;
    let stderr = child
        .stderr
        .take()
        .ok_or("docs-render stderr was not piped")?;
    let read = |mut stream: Box<dyn Read + Send>| {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            stream.read_to_end(&mut bytes).map(|_| bytes)
        })
    };
    let out = read(Box::new(stdout));
    let err = read(Box::new(stderr));
    let input = child
        .stdin
        .take()
        .ok_or("docs-render stdin was not piped")?;
    let write = std::thread::spawn(move || {
        let mut input = input;
        input.write_all(&bytes)
    });
    let began = Instant::now();
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("cannot observe docs-render: {error}"))?
        {
            break status;
        }
        if began.elapsed() >= Duration::from_secs(30) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("docs-render exceeded the 30-second helper deadline; use a synchronous deterministic initial view".to_string());
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let stdout = out
        .join()
        .map_err(|_| "docs-render stdout reader failed")?
        .map_err(|error| error.to_string())?;
    let stderr = err
        .join()
        .map_err(|_| "docs-render stderr reader failed")?
        .map_err(|error| error.to_string())?;
    let write = write
        .join()
        .map_err(|_| "docs-render stdin writer failed")?
        .map_err(|error| error.to_string());
    if !stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&stderr));
    }
    if !status.success() {
        return Err(format!(
            "docs-render exited with {status}: {}",
            String::from_utf8_lossy(&stderr).trim()
        ));
    }
    write?;
    let response: HelperResponse = serde_json::from_slice(&stdout).map_err(|error| {
        format!("docs-render stdout must contain exactly one complete JSON response: {error}")
    })?;
    if response.protocol_version != PROTOCOL_VERSION || response.generation != request.generation {
        return Err("docs-render protocol version or generation does not match the request; discard the entire generation".to_string());
    }
    if !matches!(
        (&request.operation, &response.result),
        (HelperOperation::Describe, HelperResult::Describe { .. })
            | (HelperOperation::Render { .. }, HelperResult::Render { .. })
    ) {
        return Err("docs-render returned a different operation than requested; discard the entire generation".to_string());
    }
    Ok(response)
}

fn validate_render_keys(
    requests: &[RenderRequest],
    results: &[RenderResult],
) -> Result<(), String> {
    let mut wanted = BTreeSet::new();
    let mut seeds = BTreeSet::new();
    for request in requests {
        if request.key.is_empty()
            || request.seed.is_empty()
            || !wanted.insert(request.key.as_str())
            || !seeds.insert(request.seed.as_str())
        {
            return Err("render request keys and seeds must be nonempty and unique".to_string());
        }
    }
    let mut received = BTreeSet::new();
    for result in results {
        if !wanted.contains(result.key.as_str()) || !received.insert(result.key.as_str()) {
            return Err("docs-render returned an unexpected or duplicate render key; discard the entire generation".to_string());
        }
        if result.html.is_none() && result.diagnostics.is_empty() {
            return Err("docs-render returned neither HTML nor a diagnostic for a render key; discard the entire generation".to_string());
        }
    }
    if received != wanted {
        return Err(
            "docs-render omitted a requested render key; discard the entire generation".to_string(),
        );
    }
    for result in results {
        for intro in &result.markdown_intros {
            if intro.key.is_empty() || !received.insert(intro.key.as_str()) {
                return Err("docs-render returned an empty or duplicate Markdown introduction key; discard the entire generation".to_string());
            }
        }
        for demo in &result.demos {
            if demo.key.is_empty()
                || demo.seed.is_empty()
                || !received.insert(demo.key.as_str())
                || !seeds.insert(demo.seed.as_str())
            {
                return Err("docs-render returned an empty or duplicate nested demo key/seed; discard the entire generation".to_string());
            }
            if demo.html.is_none() && demo.diagnostics.is_empty() {
                return Err("docs-render returned neither HTML nor a diagnostic for a nested demo; discard the entire generation".to_string());
            }
        }
    }
    Ok(())
}

fn validate_description(
    description: &Description,
    manifest: &Manifest,
    release: &str,
) -> Result<BTreeMap<String, Arc<[u8]>>, String> {
    let cold = serde_json::to_value(manifest).map_err(|error| error.to_string())?;
    if let Some(difference) = manifest_mismatch(&cold, &description.registry, "registry") {
        return Err(format!("compiled site registry differs from the cold declaration at {difference}; discard the entire generation"));
    }
    if description.framework_release != release
        || description.runtime_protocol != 1
        || description.island_protocol != PROTOCOL_VERSION
    {
        return Err(format!("compiled framework release or runtime/island protocol does not match Cargo metadata. {PAIR_ADVICE}"));
    }
    let fonts: Vec<_> = crate::theme::font_catalog()
        .iter()
        .map(|font| rustify_docs::site::FontResource {
            family: font.family.to_owned(),
            resource_path: font.resource_path.to_owned(),
            sha256: font.sha256.to_owned(),
        })
        .collect();
    if description.fonts != fonts {
        return Err(format!("compiled SDK font catalogue does not match this CLI's bundled font identities. {PAIR_ADVICE}"));
    }
    let mut resources = BTreeMap::new();
    for resource in &description.resources {
        if !RUNTIME_ASSETS.contains(&resource.path.as_str()) || !safe_resource_path(&resource.path)
        {
            return Err(
                "docs-render exported an unexpected or unsafe framework resource path".to_string(),
            );
        }
        if digest(resource.content.as_bytes()) != resource.sha256 {
            return Err(format!(
                "framework resource {} does not match its SHA-256",
                code_span(&resource.path)
            ));
        }
        if resources
            .insert(
                resource.path.clone(),
                Arc::from(resource.content.as_bytes()),
            )
            .is_some()
        {
            return Err("docs-render exported duplicate framework resource paths".to_string());
        }
    }
    if resources.len() != RUNTIME_ASSETS.len() {
        return Err("docs-render omitted required framework resources".to_string());
    }
    Ok(resources)
}

fn validate_demo_names(results: &[RenderResult], manifest: &Manifest) -> Result<(), String> {
    for demo in results.iter().flat_map(|result| &result.demos) {
        if demo.html.is_some()
            && !manifest
                .demos
                .iter()
                .any(|declared| declared.name == demo.name)
        {
            return Err(format!("docs-render returned HTML for undeclared nested demo {}; discard the entire generation", code_span(&demo.name)));
        }
    }
    Ok(())
}

fn manifest_mismatch(cold: &Value, compiled: &Value, path: &str) -> Option<String> {
    if cold == compiled {
        return None;
    }
    match (cold, compiled) {
        (Value::Object(cold), Value::Object(compiled)) => {
            for (key, expected) in cold {
                let path = format!("{path}.{key}");
                let Some(received) = compiled.get(key) else {
                    return Some(format!("{}: missing compiled field", literal(&path)));
                };
                if let Some(difference) = manifest_mismatch(expected, received, &path) {
                    return Some(difference);
                }
            }
            compiled
                .keys()
                .find(|key| !cold.contains_key(*key))
                .map(|key| {
                    format!(
                        "{}: unexpected compiled field",
                        literal(&format!("{path}.{key}"))
                    )
                })
        }
        (Value::Array(cold), Value::Array(compiled)) if cold.len() == compiled.len() => cold
            .iter()
            .zip(compiled)
            .enumerate()
            .find_map(|(index, (expected, received))| {
                manifest_mismatch(expected, received, &format!("{path}[{index}]"))
            }),
        _ => {
            let bounded = |value: &Value| {
                let text = value.to_string();
                let mut bounded: String = text.chars().take(160).collect();
                if bounded.len() < text.len() {
                    bounded.push('…');
                }
                bounded
            };
            Some(format!(
                "{}: cold {}, compiled {}",
                literal(path),
                literal(&bounded(cold)),
                literal(&bounded(compiled))
            ))
        }
    }
}

#[derive(Clone)]
struct InputLayout {
    roots: BTreeSet<PathBuf>,
    explicit: BTreeSet<PathBuf>,
    excluded: Vec<PathBuf>,
    context: String,
    target: PathBuf,
    profiles: (String, String),
    package_roots: BTreeMap<String, PathBuf>,
    immutable_roots: Vec<PathBuf>,
    build_env: BTreeSet<String>,
    render_files: BTreeSet<PathBuf>,
    compile_files: BTreeSet<PathBuf>,
    compile_dirs: BTreeSet<PathBuf>,
    default_compile_dirs: BTreeSet<PathBuf>,
}

#[derive(Clone)]
struct InputSnapshot {
    files: BTreeMap<PathBuf, Arc<[u8]>>,
    environment: BTreeMap<String, Option<OsString>>,
    digest: String,
    bindings: BTreeMap<PathBuf, PathBuf>,
}

impl InputLayout {
    fn new(
        project: &Project,
        root: &Path,
        manifest: &Path,
        target: &Path,
        metadata: &[&Metadata],
        mode: BuildMode,
        compiler: &OsString,
    ) -> Result<Self, String> {
        let mut roots = BTreeSet::from([manifest
            .parent()
            .ok_or("Cargo.toml has no parent directory")?
            .to_path_buf()]);
        for metadata in metadata {
            roots.extend(metadata.local_roots.iter().cloned());
        }
        let mut explicit = BTreeSet::from([manifest.to_path_buf()]);
        for package in &roots {
            for parent in package.ancestors() {
                for name in [
                    "Cargo.toml",
                    "Cargo.lock",
                    ".cargo/config.toml",
                    ".cargo/config",
                    "rust-toolchain.toml",
                    "rust-toolchain",
                ] {
                    let path = parent.join(name);
                    if path.is_file() {
                        explicit.insert(path);
                    }
                }
            }
        }
        if let Some(rust) = &project.config.rust {
            explicit.insert(root.join(&rust.registry));
        }
        let compile_files = explicit.clone();
        let mut render_files = BTreeSet::from([root.join(&project.config_file)]);
        if let Some((document, _)) = &project.config.theme_document {
            render_files.insert(root.join(document));
        }
        explicit.extend(render_files.iter().cloned());
        let flags: Vec<_> = [
            "RUSTFLAGS",
            "CARGO_ENCODED_RUSTFLAGS",
            "RUSTC",
            "RUSTC_WRAPPER",
            "RUSTC_WORKSPACE_WRAPPER",
        ]
        .into_iter()
        .map(|key| {
            (
                key,
                std::env::var_os(key).map(|value| value.to_string_lossy().into_owned()),
            )
        })
        .collect();
        let context = serde_json::to_string(&json!({"cli": env!("CARGO_PKG_VERSION"), "expected": crate::EXPECTED_FRAMEWORK_REV.trim(),
            "protocol": PROTOCOL_VERSION, "bindgen": BINDGEN_VERSION, "profiles": mode.profiles(), "compiler": compiler.to_string_lossy(), "flags": flags,
            "metadata": metadata.iter().map(|metadata| &metadata.value).collect::<Vec<_>>()})).map_err(|error| error.to_string())?;
        let mut package_roots = BTreeMap::new();
        let mut immutable_roots = Vec::new();
        for metadata in metadata {
            for package in metadata
                .value
                .get("packages")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let id = required_string(package, "id")?;
                let package_manifest = Path::new(required_string(package, "manifest_path")?);
                let package_root = package_manifest
                    .parent()
                    .ok_or("dependency manifest has no parent directory")?
                    .to_path_buf();
                if package.get("source").is_none_or(Value::is_null) {
                    package_roots.insert(id.to_string(), package_root);
                } else {
                    immutable_roots.push(package_root);
                }
            }
        }
        let mut inputs = Self {
            roots,
            explicit,
            excluded: vec![
                root.join(&project.config.out_dir),
                root.join(".rustify-docs"),
                target.to_path_buf(),
            ],
            context,
            target: target.to_path_buf(),
            profiles: (mode.profiles().0.to_string(), mode.profiles().1.to_string()),
            package_roots,
            immutable_roots,
            build_env: [
                "RUSTFLAGS",
                "CARGO_ENCODED_RUSTFLAGS",
                "RUSTC",
                "RUSTC_WRAPPER",
                "RUSTC_WORKSPACE_WRAPPER",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            render_files,
            compile_files,
            compile_dirs: BTreeSet::new(),
            default_compile_dirs: BTreeSet::new(),
        };
        inputs.discover_build_inputs()?;
        Ok(inputs)
    }

    fn capture(&self) -> Result<InputSnapshot, String> {
        let mut files = BTreeMap::new();
        let mut visited = BTreeSet::new();
        let mut bindings = BTreeMap::new();
        for root in &self.roots {
            self.read_tree(root, &mut files, &mut visited, &mut bindings)?;
        }
        for path in &self.explicit {
            self.read_file(path, &mut files, &mut bindings)?;
        }
        let environment = self
            .build_env
            .iter()
            .map(|key| (key.clone(), std::env::var_os(key)))
            .collect();
        Ok(InputSnapshot {
            digest: self.digest(&files, &environment, &bindings),
            files,
            environment,
            bindings,
        })
    }

    fn digest(
        &self,
        files: &BTreeMap<PathBuf, Arc<[u8]>>,
        environment: &BTreeMap<String, Option<OsString>>,
        bindings: &BTreeMap<PathBuf, PathBuf>,
    ) -> String {
        let mut hash = Sha256::new();
        hash.update(self.context.as_bytes());
        for (path, bytes) in files {
            hash.update(path.to_string_lossy().as_bytes());
            hash.update([0]);
            hash.update(digest(bytes).as_bytes());
            hash.update([0]);
        }
        for (key, value) in environment {
            hash.update(key.as_bytes());
            hash.update([0]);
            match value {
                Some(value) => {
                    hash.update([1]);
                    hash.update(value.to_string_lossy().as_bytes());
                }
                None => hash.update([0]),
            }
            hash.update([0]);
        }
        for (alias, real) in bindings {
            hash.update(alias.to_string_lossy().as_bytes());
            hash.update([0]);
            hash.update(real.to_string_lossy().as_bytes());
            hash.update([0]);
        }
        format!("{:x}", hash.finalize())
    }

    fn compiled(&self, snapshot: &InputSnapshot) -> InputSnapshot {
        let explicit: BTreeSet<_> = self
            .compile_files
            .iter()
            .filter_map(|path| fs::canonicalize(path).ok())
            .collect();
        let directories: Vec<_> = self
            .compile_dirs
            .union(&self.default_compile_dirs)
            .filter_map(|path| fs::canonicalize(path).ok())
            .collect();
        let files: BTreeMap<_, _> = snapshot
            .files
            .iter()
            .filter(|(path, _)| {
                path.extension().is_some_and(|extension| extension == "rs")
                    || explicit.contains(*path)
                    || directories
                        .iter()
                        .any(|directory| path.starts_with(directory))
            })
            .map(|(path, bytes)| (path.clone(), bytes.clone()))
            .collect();
        let compiled_paths: BTreeSet<_> = files
            .keys()
            .flat_map(|path| path.ancestors().map(Path::to_path_buf))
            .collect();
        let bindings = snapshot
            .bindings
            .iter()
            .filter(|(alias, real)| {
                alias.extension().is_some_and(|extension| extension == "rs")
                    || self.compile_files.contains(*alias)
                    || compiled_paths.contains(*real)
            })
            .map(|(alias, real)| (alias.clone(), real.clone()))
            .collect();
        InputSnapshot {
            digest: self.digest(&files, &snapshot.environment, &bindings),
            files,
            environment: snapshot.environment.clone(),
            bindings,
        }
    }

    fn watch_plan(&self) -> WatchPlan {
        WatchPlan {
            rust_dirs: self.roots.clone(),
            build_dirs: self
                .compile_dirs
                .union(&self.default_compile_dirs)
                .cloned()
                .collect(),
            files: self.compile_files.clone(),
            ignored: self.excluded.clone(),
        }
    }

    fn discover_build_inputs(&mut self) -> Result<(), String> {
        self.default_compile_dirs.clear();
        let logs = self.target.parent().unwrap_or(&self.target).join("logs");
        let profiles = [
            (
                self.target.join(&self.profiles.0),
                format!("{}-host.stdout", self.profiles.0),
            ),
            (
                self.target
                    .join("wasm32-unknown-unknown")
                    .join(&self.profiles.1),
                format!("{}-wasm.stdout", self.profiles.1),
            ),
        ];
        for (profile, log) in profiles {
            let deps = profile.join("deps");
            if deps.is_dir() {
                for entry in fs::read_dir(&deps).map_err(|error| error.to_string())? {
                    let path = entry.map_err(|error| error.to_string())?.path();
                    if path.extension().is_none_or(|extension| extension != "d") {
                        continue;
                    }
                    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
                    for line in text.lines() {
                        if let Some(dependency) = line.strip_prefix("# env-dep:") {
                            let key = dependency
                                .split_once('=')
                                .map_or(dependency, |(key, _)| key);
                            self.build_env.insert(key.to_string());
                        }
                    }
                    for dependency in dep_info_paths(&text) {
                        let path = PathBuf::from(dependency);
                        if path.is_absolute() {
                            self.track_build_input(path);
                        }
                    }
                }
            }
            let messages = match fs::read_to_string(logs.join(log)) {
                Ok(messages) => messages,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => {
                    return Err(format!("cannot read Cargo build-input identities: {error}"))
                }
            };
            for line in messages.lines() {
                let Ok(message) = serde_json::from_str::<Value>(line) else {
                    continue;
                };
                if message.get("reason").and_then(Value::as_str) != Some("build-script-executed") {
                    continue;
                }
                let id = required_string(&message, "package_id")?;
                let Some(root) = self.package_roots.get(id).cloned() else {
                    continue;
                };
                let out_dir = Path::new(required_string(&message, "out_dir")?);
                let output = out_dir
                    .parent()
                    .ok_or("build script out_dir has no parent")?
                    .join("output");
                if !output.is_file() {
                    continue;
                }
                // OUT_DIR hashes are opaque; Cargo's complete package identity
                // binds each relative rerun path to its own manifest root.
                let text = fs::read_to_string(output).map_err(|error| error.to_string())?;
                if !text.lines().any(|line| {
                    line.starts_with("cargo:rerun-if-") || line.starts_with("cargo::rerun-if-")
                }) {
                    self.default_compile_dirs.insert(root.clone());
                }
                for line in text.lines() {
                    if let Some(path) = line
                        .strip_prefix("cargo:rerun-if-changed=")
                        .or_else(|| line.strip_prefix("cargo::rerun-if-changed="))
                    {
                        self.track_build_input(root.join(path));
                    }
                    if let Some(key) = line
                        .strip_prefix("cargo:rerun-if-env-changed=")
                        .or_else(|| line.strip_prefix("cargo::rerun-if-env-changed="))
                    {
                        self.build_env.insert(key.to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn track_build_input(&mut self, path: PathBuf) {
        if path.starts_with(&self.target)
            || self
                .immutable_roots
                .iter()
                .any(|root| path.starts_with(root))
        {
            return;
        }
        // Cargo replaces stale dep-info after an edit. A deleted old input
        // must not make the private cache permanently block its replacement.
        if !path.exists() {
            return;
        }
        if path.is_dir() {
            self.compile_dirs.insert(path.clone());
            self.roots.insert(path);
        } else {
            self.compile_files.insert(path.clone());
            self.explicit.insert(path);
        }
    }

    fn read_tree(
        &self,
        root: &Path,
        files: &mut BTreeMap<PathBuf, Arc<[u8]>>,
        visited: &mut BTreeSet<PathBuf>,
        bindings: &mut BTreeMap<PathBuf, PathBuf>,
    ) -> Result<(), String> {
        let canonical = fs::canonicalize(root)
            .map_err(|error| format!("Rust input directory is unreadable: {error}"))?;
        bindings.insert(root.to_path_buf(), canonical.clone());
        if self
            .excluded
            .iter()
            .any(|excluded| canonical.starts_with(excluded))
        {
            return Ok(());
        }
        if !visited.insert(canonical) {
            return Ok(());
        }
        let mut entries: Vec<_> = fs::read_dir(root)
            .map_err(|error| format!("cannot read Rust build inputs: {error}"))?
            .collect::<Result<_, _>>()
            .map_err(|error| error.to_string())?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            if self
                .excluded
                .iter()
                .any(|excluded| path.starts_with(excluded))
                || matches!(
                    entry.file_name().to_str(),
                    Some("target" | ".git" | ".rustify-docs" | "node_modules")
                )
            {
                continue;
            }
            let kind = entry.file_type().map_err(|error| error.to_string())?;
            if kind.is_dir() || (kind.is_symlink() && path.is_dir()) {
                self.read_tree(&path, files, visited, bindings)?;
            } else if kind.is_file() || kind.is_symlink() {
                self.read_file(&path, files, bindings)?;
            }
        }
        Ok(())
    }

    fn read_file(
        &self,
        path: &Path,
        files: &mut BTreeMap<PathBuf, Arc<[u8]>>,
        bindings: &mut BTreeMap<PathBuf, PathBuf>,
    ) -> Result<(), String> {
        let canonical = fs::canonicalize(path)
            .map_err(|error| format!("Rust build input is unreadable: {error}"))?;
        bindings.insert(path.to_path_buf(), canonical.clone());
        if files.contains_key(&canonical) {
            return Ok(());
        }
        let bytes = fs::read(&canonical)
            .map_err(|error| format!("Rust build input is unreadable: {error}"))?;
        files.insert(canonical, Arc::from(bytes));
        Ok(())
    }
}

fn dep_info_paths(text: &str) -> Vec<String> {
    let joined = text.replace("\\\r\n", "").replace("\\\n", "");
    let Some((_, values)) = joined.lines().next().and_then(|line| line.split_once(':')) else {
        return Vec::new();
    };
    let mut paths = Vec::new();
    let mut current = String::new();
    let mut escaped = false;
    for character in values.chars() {
        if escaped {
            current.push(character);
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character.is_whitespace() {
            if !current.is_empty() {
                paths.push(std::mem::take(&mut current));
            }
        } else {
            current.push(character);
        }
    }
    if !current.is_empty() {
        paths.push(current);
    }
    paths
}

fn immutable_helper(cache: &Path, source: &Path) -> Result<PathBuf, String> {
    let bytes =
        fs::read(source).map_err(|error| format!("cannot read compiled docs-render: {error}"))?;
    let directory = cache.join("helpers").join(digest(&bytes));
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let destination = directory.join("docs-render");
    if destination.exists() {
        if fs::read(&destination).map_err(|error| error.to_string())? != bytes {
            return Err(
                "cached docs-render is corrupt; remove the private helper cache and rebuild"
                    .to_string(),
            );
        }
    } else {
        fs::copy(source, &destination)
            .map_err(|error| format!("cannot preserve the compiled docs-render: {error}"))?;
    }
    Ok(destination)
}

fn bindgen_files(
    cache: &Path,
    wasm: &Path,
    runtime: &BTreeMap<String, Arc<[u8]>>,
) -> Result<BTreeMap<String, Arc<[u8]>>, String> {
    let bytes = fs::read(wasm).map_err(|error| format!("cannot read compiled wasm: {error}"))?;
    let mut key = Sha256::new();
    key.update(BINDGEN_VERSION);
    key.update(&bytes);
    key.update(resource_digest(runtime));
    let key = format!("{:x}", key.finalize());
    let directory = cache.join("bindings").join(&key);
    let index = directory.join("cache.json");
    if index.is_file() {
        let expected: Value =
            serde_json::from_slice(&fs::read(&index).map_err(|error| error.to_string())?)
                .map_err(|error| format!("invalid binding cache index: {error}"))?;
        if expected.get("input").and_then(Value::as_str) != Some(key.as_str()) {
            return Err("binding cache belongs to a different wasm generation".to_string());
        }
        let files = read_resources(&directory, true)?;
        let actual: BTreeMap<_, _> = files
            .iter()
            .map(|(path, bytes)| (path.clone(), digest(bytes)))
            .collect();
        if expected.get("files")
            != Some(&serde_json::to_value(actual).map_err(|error| error.to_string())?)
        {
            return Err("binding cache resource hashes changed; remove the private binding cache and rebuild".to_string());
        }
        if !files.contains_key("app.js") || !files.contains_key("app_bg.wasm") {
            return Err(
                "binding cache is missing the complete app.js/app_bg.wasm pair".to_string(),
            );
        }
        return Ok(files);
    }
    let staging = cache.join("bindings").join(format!(
        ".staging-{}-{}",
        std::process::id(),
        next_temporary()
    ));
    fs::create_dir_all(&staging).map_err(|error| error.to_string())?;
    let generated = wasm_bindgen_cli_support::Bindgen::new().input_bytes("app", bytes)
        .web(true).map_err(|error| format!("wasm-bindgen web target: {error}"))?
        .debug(false).typescript(false).out_name("app").generate(&staging)
        .map_err(|error| format!("wasm-bindgen {BINDGEN_VERSION}: {error:#}; verify the paired Cargo.lock and wasm-bindgen version"));
    if let Err(error) = generated {
        let _ = fs::remove_dir_all(&staging);
        return Err(error);
    }
    let files = read_resources(&staging, false)?;
    if !files.contains_key("app.js") || !files.contains_key("app_bg.wasm") {
        return Err(
            "wasm-bindgen did not generate the complete app.js/app_bg.wasm pair".to_string(),
        );
    }
    let hashes: BTreeMap<_, _> = files
        .iter()
        .map(|(path, bytes)| (path.clone(), digest(bytes)))
        .collect();
    fs::write(
        staging.join("cache.json"),
        serde_json::to_vec(&json!({"input":key,"files":hashes}))
            .map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    if directory.exists() {
        fs::remove_dir_all(&directory).map_err(|error| error.to_string())?;
    }
    fs::rename(&staging, &directory).map_err(|error| error.to_string())?;
    Ok(files)
}

fn read_resources(
    directory: &Path,
    skip_index: bool,
) -> Result<BTreeMap<String, Arc<[u8]>>, String> {
    fn visit(
        base: &Path,
        directory: &Path,
        skip_index: bool,
        files: &mut BTreeMap<String, Arc<[u8]>>,
    ) -> Result<(), String> {
        for entry in fs::read_dir(directory).map_err(|error| error.to_string())? {
            let entry = entry.map_err(|error| error.to_string())?;
            let path = entry.path();
            let kind = entry.file_type().map_err(|error| error.to_string())?;
            if kind.is_symlink() {
                return Err("binding resources must not contain symlinks".to_string());
            }
            if kind.is_dir() {
                visit(base, &path, skip_index, files)?;
                continue;
            }
            let relative = path
                .strip_prefix(base)
                .map_err(|error| error.to_string())?
                .to_str()
                .ok_or("binding resource paths must be UTF-8")?;
            if skip_index && relative == "cache.json" {
                continue;
            }
            if !safe_resource_path(relative) {
                return Err("unsafe generated binding resource path".to_string());
            }
            files.insert(
                relative.to_string(),
                Arc::from(fs::read(&path).map_err(|error| error.to_string())?),
            );
        }
        Ok(())
    }
    let mut files = BTreeMap::new();
    visit(directory, directory, skip_index, &mut files)?;
    Ok(files)
}

fn safe_resource_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', ':', '?', '#'])
        && !path.chars().any(char::is_control)
        && Path::new(path)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

fn resource_digest(resources: &BTreeMap<String, Arc<[u8]>>) -> String {
    let mut hash = Sha256::new();
    for (path, bytes) in resources {
        hash.update((path.len() as u64).to_le_bytes());
        hash.update(path.as_bytes());
        hash.update((bytes.len() as u64).to_le_bytes());
        hash.update(bytes);
    }
    format!("{:x}", hash.finalize())
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |time| time.as_millis() as u64)
}
fn next_temporary() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}
fn has_string(value: Option<&Value>, expected: &str) -> bool {
    value
        .and_then(Value::as_array)
        .is_some_and(|values| values.iter().any(|value| value.as_str() == Some(expected)))
}
fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("Cargo metadata is missing {}", code_span(field)))
}
fn project_path(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let path = fs::canonicalize(root.join(relative))
        .map_err(|error| format!("cannot read configured Rust input: {error}"))?;
    if !path.starts_with(root) || !path.is_file() {
        return Err("configured Rust input must be a file within the project boundary".to_string());
    }
    Ok(path)
}
fn failure(rule: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic::general(Severity::Error, rule, message)
}
fn generation_changed() -> Diagnostic {
    failure("rust-generation-changed", "Rust build inputs changed during this generation; discard it and rebuild from the current inputs")
}

fn input_changes(before: &InputSnapshot, after: &InputSnapshot, root: &Path) -> Diagnostic {
    let paths: BTreeSet<_> = before.files.keys().chain(after.files.keys()).collect();
    let mut examples = Vec::new();
    let mut file_count = 0;
    for path in paths {
        let old = before.files.get(path);
        let new = after.files.get(path);
        if old == new {
            continue;
        }
        file_count += 1;
        if examples.len() < 10 {
            let kind = match (old, new) {
                (None, Some(_)) => "added",
                (Some(_), None) => "removed",
                _ => "changed",
            };
            let display = path.strip_prefix(root).unwrap_or(path).to_string_lossy();
            examples.push(format!("{kind} file {}", code_span(&display)));
        }
    }
    let keys: BTreeSet<_> = before
        .environment
        .keys()
        .chain(after.environment.keys())
        .collect();
    let mut environment_count = 0;
    for key in keys {
        let old = before.environment.get(key);
        let new = after.environment.get(key);
        if old == new {
            continue;
        }
        environment_count += 1;
        if examples.len() < 10 {
            let kind = match (old, new) {
                (None, Some(_)) => "added",
                (Some(_), None) => "removed",
                _ => "changed",
            };
            examples.push(format!(
                "{kind} build environment dependency {}",
                code_span(key)
            ));
        }
    }
    let details = if examples.is_empty() {
        "no captured file bytes or tracked environment values changed; the build context digest differs".to_string()
    } else {
        format!("{file_count} file inputs and {environment_count} environment dependencies differ; first {}: {}", examples.len(), examples.join(", "))
    };
    failure("rust-generation-changed", format!("Rust build inputs changed during this generation; discard it and rebuild from the current inputs. {details}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_docs::site::{RenderTarget, ResourceAsset};

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "rustify-rust-site-{}-{}",
                std::process::id(),
                next_temporary()
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        #[cfg(unix)]
        fn helper(&self, stdout: &str, exit: u8) -> PathBuf {
            use std::os::unix::fs::PermissionsExt;
            let path = self.0.join(format!("helper-{}", next_temporary()));
            let quote = |text: &str| format!("'{}'", text.replace('\'', "'\\''"));
            fs::write(&path, format!("#!/bin/sh\ncat >/dev/null\nprintf '%s' {}\nprintf '%s' 'author stderr' >&2\nexit {exit}\n", quote(stdout))).unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
            path
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn description() -> Description {
        Description {
            registry: serde_json::to_value(Manifest::default()).unwrap(),
            framework_release: "rustify-docs@0.1.0".to_string(),
            runtime_protocol: 1,
            island_protocol: PROTOCOL_VERSION,
            fonts: crate::theme::font_catalog()
                .iter()
                .map(|font| rustify_docs::site::FontResource {
                    family: font.family.to_owned(),
                    resource_path: font.resource_path.to_owned(),
                    sha256: font.sha256.to_owned(),
                })
                .collect(),
            resources: RUNTIME_ASSETS
                .iter()
                .map(|path| {
                    let content = if *path == "sdk-candidates.json" {
                        "[\"rounded-md\"]".to_string()
                    } else {
                        format!("exported {path}")
                    };
                    ResourceAsset {
                        path: (*path).to_string(),
                        sha256: digest(content.as_bytes()),
                        content,
                    }
                })
                .collect(),
        }
    }

    fn request() -> HelperRequest {
        HelperRequest {
            protocol_version: PROTOCOL_VERSION,
            generation: "same-generation".to_string(),
            operation: HelperOperation::Describe,
        }
    }

    #[cfg(unix)]
    #[test]
    fn real_helper_process_reads_one_request_and_returns_one_complete_response() {
        let fixture = Fixture::new();
        let response = HelperResponse {
            protocol_version: PROTOCOL_VERSION,
            generation: request().generation,
            result: HelperResult::Describe {
                description: description(),
            },
        };
        let helper = fixture.helper(&serde_json::to_string(&response).unwrap(), 0);

        assert_eq!(
            invoke_helper(&helper, &fixture.0, &request()).unwrap(),
            response
        );
    }

    #[cfg(unix)]
    #[test]
    fn helper_noise_extra_json_bad_generation_and_nonzero_exit_reject_the_batch() {
        let fixture = Fixture::new();
        let valid = HelperResponse {
            protocol_version: PROTOCOL_VERSION,
            generation: request().generation,
            result: HelperResult::Describe {
                description: description(),
            },
        };
        let json = serde_json::to_string(&valid).unwrap();
        for (stdout, exit) in [
            (format!("log before JSON\n{json}"), 0),
            (format!("{json}{json}"), 0),
            (json.replace("same-generation", "other-generation"), 0),
            (json.clone(), 7),
            ("{".to_string(), 0),
        ] {
            let helper = fixture.helper(&stdout, exit);
            assert!(
                invoke_helper(&helper, &fixture.0, &request()).is_err(),
                "{stdout}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_real_helper_cannot_answer_describe_with_render_results() {
        let fixture = Fixture::new();
        let response = HelperResponse {
            protocol_version: PROTOCOL_VERSION,
            generation: request().generation,
            result: HelperResult::Render { items: Vec::new() },
        };
        let helper = fixture.helper(&serde_json::to_string(&response).unwrap(), 0);
        assert!(invoke_helper(&helper, &fixture.0, &request()).is_err());
    }

    #[test]
    fn describe_cold_manifest_protocol_missing_resource_and_hash_mismatch_are_errors() {
        let manifest = Manifest::default();
        let release = "rustify-docs@0.1.0";
        assert!(validate_description(&description(), &manifest, release).is_ok());
        let mut wrong = description();
        wrong.registry = json!({"components":[]});
        assert!(validate_description(&wrong, &manifest, release).is_err());
        let mut wrong = description();
        wrong.runtime_protocol = 2;
        assert!(validate_description(&wrong, &manifest, release).is_err());
        let mut wrong = description();
        wrong.resources.pop();
        assert!(validate_description(&wrong, &manifest, release).is_err());
        let mut wrong = description();
        wrong.resources[0].content.push('x');
        assert!(validate_description(&wrong, &manifest, release).is_err());
        let mut wrong = description();
        wrong.resources.push(wrong.resources[0].clone());
        assert!(validate_description(&wrong, &manifest, release).is_err());
    }

    #[test]
    fn a_manifest_identity_error_names_the_exact_field_without_relaxing_equality() {
        let manifest =
            rustify_docs_syntax::parse_registry("site! { demos { Demo => crate::demo } }")
                .unwrap()
                .manifest();
        let mut description = description();
        description.registry = serde_json::to_value(&manifest).unwrap();
        description.registry["demos"][0]["function"] = json!("crate :: other");

        let error =
            validate_description(&description, &manifest, "rustify-docs@0.1.0").unwrap_err();

        assert!(error.contains("registry.demos[0].function"), "{error}");
        assert!(error.contains("crate :: other"), "{error}");
    }

    #[test]
    fn render_keys_must_match_exactly_and_remain_unique() {
        let requests: Vec<_> = ["first", "second"]
            .into_iter()
            .map(|key| RenderRequest {
                key: key.to_string(),
                seed: format!("seed-{key}"),
                target: RenderTarget::Demo {
                    name: "Demo".to_string(),
                },
            })
            .collect();
        let result = |key: &str| RenderResult {
            key: key.to_string(),
            html: Some("<div></div>".to_string()),
            geometry_css: String::new(),
            props: None,
            client_only: false,
            diagnostics: Vec::new(),
            demos: Vec::new(),
            markdown_intros: Vec::new(),
        };
        assert!(validate_render_keys(&requests, &[result("second"), result("first")]).is_ok());
        for results in [
            vec![result("first")],
            vec![result("first"), result("first")],
            vec![result("first"), result("outside")],
        ] {
            assert!(validate_render_keys(&requests, &results).is_err());
        }
    }

    #[test]
    fn render_results_require_html_or_a_diagnostic_and_unique_nested_keys() {
        let requests = vec![RenderRequest {
            key: "page".to_string(),
            seed: "page-seed".to_string(),
            target: RenderTarget::Page {
                name: "Page".to_string(),
                route: "/".to_string(),
            },
        }];
        let mut result = RenderResult {
            key: "page".to_string(),
            html: None,
            geometry_css: String::new(),
            props: None,
            client_only: false,
            diagnostics: Vec::new(),
            demos: Vec::new(),
            markdown_intros: Vec::new(),
        };
        assert!(validate_render_keys(&requests, &[result.clone()]).is_err());
        result.html = Some(String::new());
        result.demos.push(rustify_docs::site::RenderedDemo {
            key: "page-demo-0".to_string(),
            seed: "page-seed-demo-0".to_string(),
            name: "Demo".to_string(),
            html: Some(String::new()),
            geometry_css: String::new(),
            diagnostics: Vec::new(),
        });
        assert!(validate_render_keys(&requests, &[result.clone()]).is_ok());
        result.demos.push(result.demos[0].clone());
        assert!(validate_render_keys(&requests, &[result]).is_err());
    }

    #[test]
    fn successful_nested_demos_must_appear_in_the_cold_declaration() {
        let manifest =
            rustify_docs_syntax::parse_registry("site! { demos { Known => crate::demo } }")
                .unwrap()
                .manifest();
        let mut result = RenderResult {
            key: "page".to_string(),
            html: Some(String::new()),
            geometry_css: String::new(),
            props: None,
            client_only: false,
            diagnostics: Vec::new(),
            markdown_intros: Vec::new(),
            demos: vec![rustify_docs::site::RenderedDemo {
                key: "nested".to_string(),
                seed: "nested-seed".to_string(),
                name: "Known".to_string(),
                html: Some(String::new()),
                geometry_css: String::new(),
                diagnostics: Vec::new(),
            }],
        };
        assert!(validate_demo_names(&[result.clone()], &manifest).is_ok());
        result.demos[0].name = "Unknown".to_string();
        assert!(validate_demo_names(&[result], &manifest).is_err());
    }

    #[test]
    fn compiler_json_keeps_source_location_and_children_help() {
        let root = Path::new("/author");
        let diagnostic = compiler_diagnostic(
            &json!({"level":"error","message":"wrong type","rendered":"error: wrong type\nsource excerpt", "spans":[{"file_name":"site/src/demo.rs","is_primary":true,"line_start":12,"column_start":7}], "children":[{"level":"help","message":"use a number"}]}),
            root,
        );
        assert_eq!(diagnostic.file.as_deref(), Some("site/src/demo.rs"));
        assert_eq!((diagnostic.line, diagnostic.column), (Some(12), Some(7)));
        assert_eq!(diagnostic.message, "error: wrong type\nsource excerpt");
        assert_eq!(diagnostic.suggestion.as_deref(), Some("use a number"));
    }

    #[test]
    fn cargo_dep_info_preserves_escaped_spaces_and_continued_lines() {
        assert_eq!(dep_info_paths("/target/app: /site/src/lib.rs /my\\ site/data.txt \\\n /site/build.rs\n\n/site/src/lib.rs:\n"), ["/site/src/lib.rs", "/my site/data.txt", "/site/build.rs"]);
    }

    #[test]
    fn compiled_source_set_does_not_include_unbuilt_examples_or_test_files() {
        let fixture = Fixture::new();
        let compiled = fixture.0.join("counter.rs");
        let inactive = fixture.0.join("test_fixture.rs");
        fs::write(
            &compiled,
            "pub fn counter() -> &'static str { \"w-[137.25px]\" }",
        )
        .unwrap();
        fs::write(
            &inactive,
            "const POISON: &str = \"[background:url(https://example.com?q=bad)]\";",
        )
        .unwrap();
        fs::write(
            fixture.0.join("counter-123.d"),
            format!("artifact: {}\n", compiled.display()),
        )
        .unwrap();
        fs::write(
            fixture.0.join("stale-example.d"),
            format!("artifact: {}\n", inactive.display()),
        )
        .unwrap();
        let message = json!({"target":{"name":"counter"},"filenames":[fixture.0.join("libcounter-123.rlib"), fixture.0.join("libcounter-123.rmeta")]});

        let sources = compiled_sources(&message, &fixture.0).unwrap();

        assert_eq!(
            sources,
            BTreeSet::from([fs::canonicalize(&compiled).unwrap()])
        );
        assert!(!sources.contains(&fs::canonicalize(inactive).unwrap()));
        assert!(
            product_class_candidates(&fs::read_to_string(compiled).unwrap())
                .unwrap()
                .contains(&"w-[137.25px]".to_string())
        );
    }

    #[test]
    fn a_build_script_source_set_follows_its_compiler_artifact_alias() {
        let fixture = Fixture::new();
        let directory = fixture.0.join("build/author-123");
        fs::create_dir_all(&directory).unwrap();
        let source = fixture.0.join("build.rs");
        fs::write(&source, "fn main() {}\n").unwrap();
        fs::write(
            directory.join("build_script_build-123.d"),
            format!("artifact: {}\n", source.display()),
        )
        .unwrap();
        let message = json!({"target":{"name":"build-script-build","kind":["custom-build"]},"filenames":[directory.join("build-script-build")]});

        assert_eq!(
            compiled_sources(&message, &fixture.0).unwrap(),
            BTreeSet::from([fs::canonicalize(source).unwrap()])
        );
    }

    #[test]
    fn product_classes_keep_comments_and_future_feature_branches_but_exclude_tests() {
        let text = concat!(
            "\u{feff}//! 产品 résumé\r\n",
            "// Future hidden state: opacity-0\r\n",
            "pub fn counter() -> &'static str { \"w-[137.25px]\" }\r\n",
            "#[cfg(not(test))] const LIVE: &str = \"border-teal-600\";\r\n",
            "#[cfg(any(test, feature = \"future\"))] const FUTURE: &str = \"md:gap-5\";\r\n",
            "#[cfg(test)] mod tests { const POISON: &str = \"bg-fuchsia-700\"; }\r\n",
            "#[cfg(all(test, feature = \"future\"))] const ONLY_TEST: &str = \"text-orange-800\";\r\n",
            "#[test] fn test_counter() { let _ = \"p-[999px]\"; }\r\n",
            "struct Counter; impl Counter { #[cfg(test)] fn test_method() { let _ = \"lg:grid-cols-7\"; } }\r\n",
        );
        let candidates = product_class_candidates(text).unwrap();
        for class in ["opacity-0", "w-[137.25px]", "border-teal-600", "md:gap-5"] {
            assert!(
                candidates.contains(&class.to_string()),
                "{class}: {candidates:?}"
            );
        }
        for class in [
            "bg-fuchsia-700",
            "text-orange-800",
            "p-[999px]",
            "lg:grid-cols-7",
        ] {
            assert!(
                !candidates.contains(&class.to_string()),
                "{class}: {candidates:?}"
            );
        }
    }

    #[test]
    fn the_actual_extractor_test_url_cannot_enter_product_candidates() {
        let source = include_str!(
            "../../rustify-style/upstream/oxide/extractor/arbitrary_property_machine.rs"
        );
        let poisoned = |candidate: &String| candidate.contains("https://example.com?q=");
        assert!(rustify_style::scanner::scan(source, "rs")
            .candidates
            .iter()
            .any(poisoned));
        assert!(!product_class_candidates(source)
            .unwrap()
            .iter()
            .any(poisoned));
    }

    #[test]
    fn every_actual_compile_keeps_history_even_when_profile_and_epoch_match() {
        let fixture = Fixture::new();
        let timing = CompileTiming {
            elapsed_ms: 120_000,
            completed_at_ms: 1_000,
            compiled_artifacts: 7,
            profile: "docs-host".to_string(),
            compiler: "mbx".to_string(),
            flags: vec!["build".to_string(), "--locked".to_string()],
            success: true,
            ..Default::default()
        };
        record_compile_history(&fixture.0, &timing, b"cold Cargo JSON", b"cold stderr").unwrap();
        let warm = CompileTiming {
            elapsed_ms: 210,
            fresh_artifacts: 7,
            compiled_artifacts: 0,
            ..timing
        };
        record_compile_history(&fixture.0, &warm, b"warm Cargo JSON", b"warm stderr").unwrap();
        let histories: Vec<_> = fs::read_dir(fixture.0.join("history"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        assert_eq!(histories.len(), 2);
        let mut elapsed = BTreeMap::new();
        for directory in histories {
            let timing: Value =
                serde_json::from_slice(&fs::read(directory.join("timing.json")).unwrap()).unwrap();
            elapsed.insert(
                timing["elapsed_ms"].as_u64().unwrap(),
                fs::read(directory.join("stdout")).unwrap(),
            );
            assert_eq!(timing["profile"], "docs-host");
            assert_eq!(timing["flags"], json!(["build", "--locked"]));
            assert!(!fs::read(directory.join("stderr")).unwrap().is_empty());
        }
        assert_eq!(elapsed[&120_000], b"cold Cargo JSON");
        assert_eq!(elapsed[&210], b"warm Cargo JSON");
    }

    fn cache_project(root: &Path) -> Project {
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/lib.rs"), "pub fn value() {}\n").unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}\n").unwrap();
        fs::write(
            root.join("rustify-docs.toml"),
            "schema_version = 1\n[site]\nname = 'Cache regression'\n[rust]\nmanifest = 'Cargo.toml'\nregistry = 'src/lib.rs'\n",
        )
        .unwrap();
        Project::load(root).unwrap().0
    }

    #[test]
    fn input_layout_tracks_hydrate_only_manifest_and_target_graphs() {
        let fixture = Fixture::new();
        let site = fixture.0.join("site");
        let project = cache_project(&site);
        let site = fs::canonicalize(&site).unwrap();
        for name in ["rustify-docs", "palette", "native", "web"] {
            let root = fixture.0.join(name);
            fs::create_dir_all(root.join("src")).unwrap();
            fs::write(root.join("src/lib.rs"), "pub fn value() {}\n").unwrap();
            fs::write(root.join("Cargo.toml"), format!(
                "[package]\nname = '{name}'\nversion = '0.1.0'\nedition = '2021'\n[features]\nalternate = []\n"
            )).unwrap();
        }
        let manifest = site.join("Cargo.toml");
        fs::write(
            &manifest,
            r#"[package]
name = "cache-site"
version = "0.1.0"
edition = "2021"
[workspace]
[lib]
crate-type = ["cdylib", "rlib"]
[[bin]]
name = "docs-render"
path = "src/main.rs"
[features]
ssr = []
hydrate = ["dep:palette"]
[dependencies]
rustify-docs = { path = "../rustify-docs" }
palette = { path = "../palette", optional = true }
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
native = { path = "../native" }
[target.'cfg(target_arch = "wasm32")'.dependencies]
web = { path = "../web" }
"#,
        )
        .unwrap();
        let compiler = OsString::from("mbx");
        let lock = Command::new(&compiler)
            .args(["generate-lockfile", "--offline", "--manifest-path"])
            .arg(&manifest)
            .output()
            .unwrap();
        assert!(
            lock.status.success(),
            "{}",
            String::from_utf8_lossy(&lock.stderr)
        );
        let target = site.join(".rustify-docs/target");
        let host = Metadata::read(
            cargo_metadata(&compiler, &manifest, &target, &site, false).unwrap(),
            &manifest,
        )
        .unwrap();
        let wasm = Metadata::read(
            cargo_metadata(&compiler, &manifest, &target, &site, true).unwrap(),
            &manifest,
        )
        .unwrap();
        assert!(!host
            .local_roots
            .contains(&fs::canonicalize(fixture.0.join("palette")).unwrap()));
        assert!(
            wasm.local_roots
                .contains(&fs::canonicalize(fixture.0.join("palette")).unwrap()),
            "hydrate-only package is missing"
        );
        assert!(host
            .local_roots
            .contains(&fs::canonicalize(fixture.0.join("native")).unwrap()));
        assert!(!host
            .local_roots
            .contains(&fs::canonicalize(fixture.0.join("web")).unwrap()));
        assert!(wasm
            .local_roots
            .contains(&fs::canonicalize(fixture.0.join("web")).unwrap()));
        assert!(!wasm
            .local_roots
            .contains(&fs::canonicalize(fixture.0.join("native")).unwrap()));
        for (metadata, feature) in [(&host, "ssr"), (&wasm, "hydrate")] {
            let author = metadata
                .value
                .pointer("/resolve/nodes")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .find(|node| node["id"] == metadata.package_id)
                .unwrap();
            assert_eq!(
                author["features"],
                json!([feature]),
                "the two rendering features must never be unified"
            );
        }
        let layout = InputLayout::new(
            &project,
            &site,
            &manifest,
            &target,
            &[&host, &wasm],
            BuildMode::Development,
            &compiler,
        )
        .unwrap();
        let palette_manifest = fs::canonicalize(fixture.0.join("palette/Cargo.toml")).unwrap();
        assert!(layout.watch_plan().files.contains(&palette_manifest));
        let before = layout.compiled(&layout.capture().unwrap()).digest;
        let text = fs::read_to_string(&palette_manifest).unwrap();
        fs::write(
            &palette_manifest,
            format!("{text}default = ['alternate']\n"),
        )
        .unwrap();
        assert_ne!(layout.compiled(&layout.capture().unwrap()).digest, before);
        let compiled = layout.compiled(&layout.capture().unwrap()).digest;
        fs::write(site.join("page.md"), "Markdown does not recompile Rust").unwrap();
        assert_eq!(layout.compiled(&layout.capture().unwrap()).digest, compiled);
    }

    #[test]
    fn discover_build_inputs_binds_duplicate_names_by_package_id_and_out_dir() {
        let fixture = Fixture::new();
        let site = fixture.0.join("site");
        let project = cache_project(&site);
        let site = fs::canonicalize(&site).unwrap();
        let manifest = site.join("Cargo.toml");
        fs::write(&manifest, "[package]\nname = 'site'\nversion = '0.1.0'\n").unwrap();
        let target = site.join(".rustify-docs/target");
        let logs = site.join(".rustify-docs/logs");
        fs::create_dir_all(&logs).unwrap();
        let mut packages = Vec::new();
        let mut roots = BTreeSet::new();
        for version in [1, 2] {
            let root = fixture.0.join(format!("payload-v{version}"));
            fs::create_dir_all(&root).unwrap();
            fs::write(
                root.join("Cargo.toml"),
                format!("[package]\nname = 'payload'\nversion = '{version}.0.0'\n"),
            )
            .unwrap();
            fs::write(root.join("payload.txt"), format!("{version}")).unwrap();
            fs::write(root.join("unrelated.txt"), "stale output must not track me").unwrap();
            let id = format!("path+file://{}#payload@{version}.0.0", root.display());
            packages.push(json!({"name":"payload", "id":id, "source":null, "manifest_path":root.join("Cargo.toml")}));
            roots.insert(root);
        }
        for (profile, name) in [
            ("docs-host", "host"),
            ("wasm32-unknown-unknown/docs-wasm", "wasm"),
        ] {
            let mut stdout = String::new();
            for (index, package) in packages.iter().enumerate() {
                if (name == "host") != (index == 0) {
                    continue;
                }
                let directory = target
                    .join(profile)
                    .join(format!("build/payload-hash{index}"));
                fs::create_dir_all(directory.join("out")).unwrap();
                fs::write(directory.join("output"), "cargo:rerun-if-changed=payload.txt\ncargo::rerun-if-env-changed=CACHE_REGRESSION_ENV\n").unwrap();
                stdout.push_str(&format!("{}\n", json!({"reason":"build-script-executed", "package_id":package["id"], "out_dir":directory.join("out")})));
            }
            let profile_name = profile.rsplit('/').next().unwrap();
            fs::write(logs.join(format!("{profile_name}-{name}.stdout")), stdout).unwrap();
            // A stale output not reported by Cargo must never be guessed by name.
            let stale = target.join(profile).join("build/payload-stale");
            fs::create_dir_all(&stale).unwrap();
            fs::write(
                stale.join("output"),
                "cargo:rerun-if-changed=unrelated.txt\n",
            )
            .unwrap();
        }
        let metadata = Metadata {
            value: json!({"packages":packages}),
            package_id: "site".into(),
            library_name: "site".into(),
            library_source: site.join("src/lib.rs"),
            framework_release: "unused".into(),
            local_roots: roots,
        };
        let layout = InputLayout::new(
            &project,
            &site,
            &manifest,
            &target,
            &[&metadata],
            BuildMode::Development,
            &OsString::from("mbx"),
        )
        .unwrap();
        for version in [1, 2] {
            let path = fixture.0.join(format!("payload-v{version}/payload.txt"));
            assert!(
                layout.watch_plan().files.contains(&path),
                "missing v{version} build input"
            );
            let before = layout.compiled(&layout.capture().unwrap()).digest;
            fs::write(&path, format!("changed {version}")).unwrap();
            assert_ne!(layout.compiled(&layout.capture().unwrap()).digest, before);
            fs::remove_file(&path).unwrap();
            assert!(layout.capture().is_err());
            assert!(
                layout.watch_plan().files.contains(&path),
                "recovery watch was lost"
            );
            fs::write(&path, format!("restored {version}")).unwrap();
            assert!(layout.capture().is_ok());
        }
        assert!(layout.build_env.contains("CACHE_REGRESSION_ENV"));
        assert!(!layout
            .compile_files
            .iter()
            .any(|path| path.ends_with("unrelated.txt")));
    }

    #[test]
    fn source_changes_invalidate_a_snapshot_but_private_outputs_do_not() {
        let fixture = Fixture::new();
        fs::write(fixture.0.join("lib.rs"), "first").unwrap();
        let cache = fixture.0.join(".rustify-docs");
        fs::create_dir_all(&cache).unwrap();
        let layout = InputLayout {
            roots: BTreeSet::from([fixture.0.clone()]),
            explicit: BTreeSet::new(),
            excluded: vec![cache.clone()],
            context: "compiler flags".to_string(),
            target: cache.join("target"),
            profiles: ("docs-host".to_string(), "docs-wasm".to_string()),
            package_roots: BTreeMap::new(),
            immutable_roots: Vec::new(),
            build_env: BTreeSet::new(),
            render_files: BTreeSet::new(),
            compile_files: BTreeSet::new(),
            compile_dirs: BTreeSet::new(),
            default_compile_dirs: BTreeSet::new(),
        };
        let first = layout.capture().unwrap();
        fs::write(cache.join("artifact"), "private output").unwrap();
        assert_eq!(layout.capture().unwrap().digest, first.digest);
        fs::write(fixture.0.join("lib.rs"), "second").unwrap();
        let after = layout.capture().unwrap();
        assert_ne!(after.digest, first.digest);
        let root = fs::canonicalize(&fixture.0).unwrap();
        let diagnostic = input_changes(&first, &after, &root);
        assert!(
            diagnostic.message.contains("changed file `lib.rs`"),
            "{}",
            diagnostic.message
        );
        assert!(diagnostic.message.contains("1 file inputs"));
        assert!(!diagnostic.message.contains("private output"));

        // Render-only reads change the round receipt, not the compiled pair.
        fs::write(fixture.0.join("page.md"), "first page").unwrap();
        fs::write(fixture.0.join("theme.json"), "first theme").unwrap();
        let first = layout.capture().unwrap();
        let compiled = layout.compiled(&first).digest;
        fs::write(fixture.0.join("page.md"), "second page").unwrap();
        fs::write(fixture.0.join("theme.json"), "second theme").unwrap();
        let after = layout.capture().unwrap();
        assert_ne!(after.digest, first.digest);
        assert_eq!(layout.compiled(&after).digest, compiled);

        // The same Markdown file is a compile input when Rust includes it.
        let mut dual = layout.clone();
        dual.compile_files.insert(fixture.0.join("page.md"));
        let compiled = dual.compiled(&after).digest;
        fs::write(fixture.0.join("page.md"), "third page").unwrap();
        assert_ne!(dual.compiled(&dual.capture().unwrap()).digest, compiled);

        // A build script with no rerun-if instructions keeps the whole package.
        let mut all = layout.clone();
        all.default_compile_dirs.insert(fixture.0.clone());
        let compiled = all.compiled(&all.capture().unwrap()).digest;
        fs::write(fixture.0.join("theme.json"), "third theme").unwrap();
        assert_ne!(all.compiled(&all.capture().unwrap()).digest, compiled);

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            fs::write(fixture.0.join("a.rs"), "first module").unwrap();
            fs::write(fixture.0.join("b.rs"), "second module").unwrap();
            let alias = fixture.0.join("module.rs");
            symlink("a.rs", &alias).unwrap();
            let before = layout.compiled(&layout.capture().unwrap()).digest;
            fs::remove_file(&alias).unwrap();
            symlink("b.rs", &alias).unwrap();
            assert_ne!(
                layout.compiled(&layout.capture().unwrap()).digest,
                before,
                "all real modules exist in both trees, but the compiled alias binding changed"
            );
        }
    }

    #[test]
    fn input_deltas_bound_examples_and_report_environment_names_without_values() {
        let files: BTreeMap<_, _> = (0..12)
            .map(|index| {
                (
                    PathBuf::from(format!("/author/{index}.rs")),
                    Arc::<[u8]>::from(b"old".as_slice()),
                )
            })
            .collect();
        let before = InputSnapshot {
            bindings: BTreeMap::new(),
            files: files.clone(),
            environment: BTreeMap::from([("AUTHOR_FLAG".to_string(), None)]),
            digest: "old".to_string(),
        };
        let mut after = InputSnapshot {
            bindings: BTreeMap::new(),
            files: files
                .into_keys()
                .map(|path| (path, Arc::<[u8]>::from(b"new".as_slice())))
                .collect(),
            environment: BTreeMap::from([(
                "AUTHOR_FLAG".to_string(),
                Some(OsString::from("private-value")),
            )]),
            digest: "new".to_string(),
        };
        let diagnostic = input_changes(&before, &after, Path::new("/author"));
        assert!(diagnostic
            .message
            .contains("12 file inputs and 1 environment dependencies differ; first 10"));
        assert_eq!(diagnostic.message.matches("changed file `").count(), 10);
        after.files = before.files.clone();
        let diagnostic = input_changes(&before, &after, Path::new("/author"));
        assert!(diagnostic.message.contains("AUTHOR_FLAG"));
        assert!(!diagnostic.message.contains("private-value"));
    }

    #[test]
    fn the_resource_group_digest_binds_paths_bytes_and_every_snippet() {
        let first = BTreeMap::from([
            ("app.js".to_string(), Arc::<[u8]>::from(b"glue".as_slice())),
            (
                "snippets/sdk/runtime.js".to_string(),
                Arc::<[u8]>::from(b"runtime".as_slice()),
            ),
        ]);
        let mut changed = first.clone();
        changed.insert(
            "snippets/sdk/runtime.js".to_string(),
            Arc::from(b"new runtime".as_slice()),
        );
        assert_ne!(resource_digest(&first), resource_digest(&changed));
        assert_eq!(resource_digest(&first).len(), 64);
    }

    fn git_metadata(revision: &str) -> Metadata {
        let source = format!("git+https://example.com/framework?rev={revision}#{revision}");
        let mut packages: Vec<_> = FRAMEWORKS
            .iter()
            .map(|name| json!({"name":name,"id":name,"version":"0.1.0","source":source}))
            .collect();
        packages.push(json!({"name":"wasm-bindgen","id":"bindings","version":BINDGEN_VERSION,"source":"registry+https://example.com"}));
        Metadata {
            value: json!({"packages":packages,"resolve":{"nodes":[{"id":"author","dependencies":FRAMEWORKS}]}}),
            package_id: "author".to_string(),
            library_name: "author".to_string(),
            library_source: PathBuf::from("/author/src/lib.rs"),
            framework_release: "rustify-docs@0.1.0".to_string(),
            local_roots: BTreeSet::new(),
        }
    }

    #[test]
    fn framework_sources_need_one_explicit_matching_revision_and_exact_bindgen() {
        assert!(validate_framework(&git_metadata("abcdef"), "abcdef").is_ok());
        assert!(validate_framework(&git_metadata("abcdef"), "").is_err());
        assert!(validate_framework(&git_metadata("abcdef"), "changed").is_err());
        let mut metadata = git_metadata("abcdef");
        metadata.value["packages"][1]["source"] =
            json!("git+https://example.com/other?rev=abcdef#abcdef");
        assert!(validate_framework(&metadata, "abcdef").is_err());
        let mut metadata = git_metadata("abcdef");
        metadata.value["packages"][3]["version"] = json!("0.2.127");
        assert!(validate_framework(&metadata, "abcdef").is_err());
    }

    #[test]
    fn registry_local_paths_cannot_silently_bind_a_root_function() {
        let fixture = Fixture::new();
        let library = fixture.0.join("lib.rs");
        let text = "pub fn demo() {}\n";
        fs::write(&library, text).unwrap();
        let mut metadata = git_metadata("abcdef");
        metadata.library_source = library.clone();
        metadata.library_name = "known_lib".to_string();
        let files = BTreeMap::from([(
            fs::canonicalize(library).unwrap(),
            Arc::<[u8]>::from(text.as_bytes()),
        )]);
        let registry = "mod nested { fn site() { site! { demos { Demo => self::demo } } } }";
        let declaration = rustify_docs_syntax::parse_registry(registry).unwrap();
        let mut diagnostics = Vec::new();
        let (sources, _) = demo_sources(
            &declaration,
            &metadata,
            &fixture.0,
            &files,
            "registry.rs",
            registry,
            &mut diagnostics,
        );
        assert!(sources.is_empty());
        assert!(diagnostics[0].message.contains("crate::module::function"));
    }

    #[test]
    fn generated_resources_reject_absolute_traversal_and_url_paths() {
        for path in [
            "../outside.js",
            "/private/file.js",
            "C:\\file.js",
            "foo/../file.js",
            "https://example.com/x.js",
            "file.js?q=1",
        ] {
            assert!(!safe_resource_path(path), "{path}");
        }
        assert!(safe_resource_path(
            "snippets/rustify-ui-123/web/dom-runtime.js"
        ));
    }
}
