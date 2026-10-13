//! `rustify-docs dev`: the site served from memory while its author writes.
//!
//! One builder owns every generation. The watcher only says that something
//! the site reads has changed; saves that land during a build wait for the
//! next round, which reads every input afresh and renders a complete
//! generation, its fragments from the cache wherever their content is
//! unchanged. A generation that succeeds replaces the one served, whole. One
//! that fails, or panics, changes nothing served: its problems go to the
//! terminal and to every open page, which keeps showing the last good
//! version. Each round runs on a thread of its own, so a panic takes that
//! round's reactive owners and cache updates with it and the next round
//! starts from what was last committed.
//!
//! Open pages follow over a server-sent event stream. Each connection says
//! which page it is and which build it is a copy of, and is told whether the
//! newest build changed that page; only then does the page reload.
//!
//! Nothing is written: not into the project, not into its output directory,
//! whose lock is never taken, so `build` can run alongside. The server
//! listens on the loopback interface only.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::time::{Duration, Instant};

use rustify_docs::content::{self, Diagnostic, FragmentCache, Mode, Project};
use rustify_docs::diagnostic::Severity;
use rustify_docs::generation;
use rustify_docs::highlight::Highlighter;
use rustify_docs::history::Dates;
use rustify_docs::shell::{CSP, DEV_PREFIX, DEV_SCRIPT_URL};

use crate::report;
use crate::FAULT_VARIABLE;

/// A generation's files by output path.
type FileMap = BTreeMap<String, Arc<[u8]>>;
type Files = Arc<FileMap>;

/// How long the builder waits for a burst of saves to settle, and the most
/// it waits before building anyway.
const SETTLE: Duration = Duration::from_millis(25);
const SETTLE_AT_MOST: Duration = Duration::from_millis(250);

/// How often an idle event stream is written to, so a closed tab is noticed.
const KEEP_ALIVE: Duration = Duration::from_secs(15);

pub struct Options {
    pub config: PathBuf,
    /// An explicit port; without one, the configuration's `dev.port`, or a
    /// free port when that one is taken.
    pub port: Option<u16>,
}

/// The newest generation that built.
struct Good {
    revision: u64,
    files: Files,
    base: String,
}

/// What every connection reads.
struct State {
    sequence: u64,
    building: bool,
    phase: &'static str,
    compile: serde_json::Value,
    /// Moves on whenever what an open page should know changes: a build
    /// that changed something, or one that failed (even with the same
    /// problems as before, so a page can show the save was seen).
    revision: u64,
    good: Option<Good>,
    /// The newest build failed; `diagnostics` are its problems.
    failed: bool,
    diagnostics: Arc<Vec<Diagnostic>>,
    /// For each output path, the revision that last changed or removed it.
    changed_at: HashMap<String, u64>,
    /// What is served while nothing has built, and its base.
    shell: Option<(Files, String)>,
}

struct Shared {
    /// Tells this run of `dev` from an earlier one on the same port.
    session: String,
    state: Mutex<State>,
    changed: Condvar,
}

/// Which changed paths matter: the configuration, the content and asset
/// directories and what their links lead to, and the other files the site
/// reads (included source, referenced images, the logo); never the output.
struct Filter {
    config: PathBuf,
    dirs: Vec<PathBuf>,
    files: BTreeSet<PathBuf>,
    existence: BTreeMap<PathBuf, Option<(PathBuf, bool)>>,
    ancestors: BTreeMap<PathBuf, Option<DirectoryIdentity>>,
    ignored: Vec<PathBuf>,
    rust_dirs: BTreeSet<PathBuf>,
    rust_files: BTreeSet<PathBuf>,
    build_dirs: BTreeSet<PathBuf>,
}

/// What one round read beyond the content and asset directories: the
/// project-relative files it named, found or not, and where the links in
/// those directories lead.
#[derive(Default)]
struct Read<'a> {
    files: Vec<&'a BTreeSet<String>>,
    linked: Option<&'a BTreeSet<String>>,
    inputs: Vec<&'a content::InputReceipts>,
}

#[derive(PartialEq, Eq)]
struct DirectoryIdentity {
    real: PathBuf,
    directory: bool,
    #[cfg(unix)]
    node: (u64, u64),
    #[cfg(not(unix))]
    created: Option<std::time::SystemTime>,
}

fn target_identity(path: &Path) -> Option<(PathBuf, bool)> {
    let real = std::fs::canonicalize(path).ok()?;
    let metadata = std::fs::metadata(&real).ok()?;
    Some((real, metadata.is_dir()))
}

fn directory_identity(path: &Path) -> Option<DirectoryIdentity> {
    let real = std::fs::canonicalize(path).ok()?;
    let metadata = std::fs::metadata(&real).ok()?;
    #[cfg(unix)]
    use std::os::unix::fs::MetadataExt;
    Some(DirectoryIdentity {
        real,
        directory: metadata.is_dir(),
        #[cfg(unix)]
        node: (metadata.dev(), metadata.ino()),
        #[cfg(not(unix))]
        created: metadata.created().ok(),
    })
}

fn prospective_file(path: &Path) -> Option<PathBuf> {
    for parent in path.ancestors() {
        if let Ok(real) = std::fs::canonicalize(parent) {
            return if parent == path {
                Some(real)
            } else {
                Some(real.join(path.strip_prefix(parent).ok()?))
            };
        }
    }
    None
}

impl Filter {
    fn of(root: &Path, project: &Project, read: Read) -> Self {
        let config = &project.config;
        let out = root.join(&config.out_dir);
        let mut dirs = vec![root.join(&config.assets_dir)];
        // A link may lead to a file or to a directory; either way, what is
        // under it. Explicit content selection does not authorize a tree.
        if config.content_include.is_none() {
            dirs.insert(0, root.join(&config.content_dir));
            dirs.extend(
                read.linked
                    .into_iter()
                    .flatten()
                    .map(|path| root.join(path)),
            );
        }
        let mut files: BTreeSet<_> = read
            .files
            .into_iter()
            .flatten()
            .map(|path| root.join(path))
            .collect();
        if let Some((path, _)) = &config.theme_document {
            files.insert(root.join(path));
        }
        let mut existence = BTreeMap::new();
        let mut ancestors = BTreeMap::new();
        if let Some(selected) = &config.content_include {
            let assets = root.join(&config.assets_dir);
            let mut bytes = BTreeSet::new();
            files.extend(
                read.linked
                    .into_iter()
                    .flatten()
                    .map(|path| root.join(path)),
            );
            for inputs in read
                .inputs
                .iter()
                .copied()
                .chain(std::iter::once(&project.inputs))
            {
                for (alias, real) in inputs.byte_paths() {
                    bytes.insert(root.join(alias));
                    bytes.insert(real.to_path_buf());
                }
                for (alias, real, directory) in inputs.existence_paths() {
                    let alias = root.join(alias);
                    let original = Some((real.to_path_buf(), directory));
                    existence
                        .entry(alias.clone())
                        .or_insert_with(|| original.clone());
                    existence.entry(real.to_path_buf()).or_insert(original);
                    if directory && alias.starts_with(&assets) {
                        dirs.push(real.to_path_buf());
                    }
                }
            }
            // `referenced` includes repository targets. They depend on identity,
            // while actual reads depend on bytes even if another receipt is Exists.
            files.retain(|path| !existence.contains_key(path));
            files.extend(bytes);
            for (path, _) in selected {
                let alias = root.join(&config.content_dir).join(path);
                if let Some(real) = prospective_file(&alias) {
                    files.insert(real);
                }
                files.insert(alias);
            }
            existence.retain(|path, _| !files.contains(path));
            let real_root = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
            for path in files.iter().chain(existence.keys()) {
                for parent in path
                    .ancestors()
                    .skip(1)
                    .filter(|parent| parent.starts_with(root) || parent.starts_with(&real_root))
                {
                    ancestors
                        .entry(parent.to_path_buf())
                        .or_insert_with(|| directory_identity(parent));
                }
            }
        }
        Self {
            config: root.join(&project.config_file),
            dirs,
            files,
            existence,
            ancestors,
            ignored: vec![out],
            rust_dirs: BTreeSet::new(),
            rust_files: BTreeSet::new(),
            build_dirs: BTreeSet::new(),
        }
    }

    fn relevant(&self, path: &Path) -> bool {
        let private = path.components().any(|part| {
            matches!(part, Component::Normal(name) if name == ".git" || name == ".rustify-docs" || name == "target")
        });
        if private || self.ignored.iter().any(|dir| path.starts_with(dir)) {
            return false;
        }
        path == self.config
            || self.dirs.iter().any(|dir| path.starts_with(dir))
            || self.files.contains(path)
            || self.existence.iter().any(|(dependency, original)| {
                dependency.starts_with(path)
                    && target_identity(dependency).as_ref() != original.as_ref()
            })
            || self.ancestors.iter().any(|(parent, original)| {
                parent.starts_with(path) && directory_identity(parent).as_ref() != original.as_ref()
            })
            || self.rust_files.contains(path)
            || self.rust_dirs.contains(path)
            || self
                .build_dirs
                .iter()
                .any(|directory| path.starts_with(directory))
            || (path.extension().is_some_and(|extension| extension == "rs")
                && self
                    .rust_dirs
                    .iter()
                    .any(|directory| path.starts_with(directory)))
    }
}

/// Whether a watcher event calls for a round. Every round reads all of its
/// inputs again, so an overflow (events the system dropped) or a watcher
/// error needs nothing more than one.
fn wants_round(event: &notify::Result<notify::Event>, filter: &Filter) -> bool {
    match event {
        Err(_) => true,
        Ok(event) if event.need_rescan() => true,
        Ok(event) => event.paths.iter().any(|path| filter.relevant(path)),
    }
}

/// What one round produced.
enum Outcome {
    Built {
        files: FileMap,
        base: String,
        diagnostics: Vec<Diagnostic>,
    },
    Failed {
        diagnostics: Vec<Diagnostic>,
        /// The page to show while nothing has built, for the configuration
        /// this round read, when it read one.
        shell: Option<(FileMap, String)>,
    },
}

/// The builder's own state between rounds.
struct Builder {
    root: PathBuf,
    /// The configuration file, in `root`.
    config: PathBuf,
    highlighter: Highlighter,
    /// The fragments of the last round that finished; a round works on a
    /// copy and its copy replaces this only when it finishes.
    cache: FragmentCache,
    dates: Dates,
    workers: usize,
    fault: Option<String>,
    filter: Arc<RwLock<Filter>>,
    rust: Option<crate::rust_site::Prepared>,
    observer: Arc<dyn Fn(crate::rust_site::Progress) + Send + Sync>,
}

fn sorted(mut diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

fn has_errors(diagnostics: &[Diagnostic]) -> bool {
    diagnostics.iter().any(|d| d.severity == Severity::Error)
}

fn stale(outcome: &Outcome) -> bool {
    match outcome {
        Outcome::Failed { diagnostics, .. } => diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.rule.as_str(),
                "rust-generation-changed" | "render-generation-changed"
            )
        }),
        _ => false,
    }
}

fn panic_message(panic: &(dyn std::any::Any + Send)) -> String {
    panic
        .downcast_ref::<&str>()
        .map(|text| text.to_string())
        .or_else(|| panic.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "a panic without a message".to_owned())
}

impl Builder {
    /// One round, on a thread of its own.
    fn round(&mut self) -> Outcome {
        let mut cache = self.cache.clone();
        let mut rust = self.rust.clone();
        let result = std::thread::scope(|scope| {
            let worker = std::thread::Builder::new()
                .name("rustify-docs-build".into())
                .stack_size(8 << 20)
                .spawn_scoped(scope, || self.build(&mut cache, &mut rust));
            match worker {
                Ok(worker) => worker.join().map_err(|panic| panic_message(&*panic)),
                Err(error) => Err(format!("no thread to build on: {error}")),
            }
        });
        match result {
            Ok((outcome, filter)) => {
                self.cache = cache;
                self.rust = rust;
                if let Some(filter) = filter {
                    if let Ok(mut current) = self.filter.write() {
                        let mut filter = filter;
                        filter.rust_dirs.extend(current.rust_dirs.iter().cloned());
                        filter.rust_files.extend(current.rust_files.iter().cloned());
                        filter.build_dirs.extend(current.build_dirs.iter().cloned());
                        filter.ignored.extend(current.ignored.iter().cloned());
                        *current = filter;
                    }
                }
                outcome
            }
            Err(message) => Outcome::Failed {
                diagnostics: vec![Diagnostic::general(
                    Severity::Error,
                    "internal",
                    format!(
                        "this build stopped inside rustify-docs ({message}); nothing it made is served"
                    ),
                )
                .with_suggestion(
                    "save again to retry; if it happens every time, report it with the file that triggers it",
                )],
                shell: None,
            },
        }
    }

    fn build(
        &self,
        cache: &mut FragmentCache,
        rust: &mut Option<crate::rust_site::Prepared>,
    ) -> (Outcome, Option<Filter>) {
        let (project, mut diagnostics) = match Project::load_file(&self.config) {
            Ok(loaded) => loaded,
            Err(diagnostics) => {
                return (
                    Outcome::Failed {
                        diagnostics: sorted(diagnostics),
                        shell: None,
                    },
                    None,
                )
            }
        };
        let shell = || {
            generation::dev_shell(&project)
                .ok()
                .map(|shell| (shell.files, project.config.site.base.clone()))
        };
        let (snapshot, problems) = content::snapshot(&project);
        diagnostics.extend(problems);
        if let Some(marker) = &self.fault {
            let texts: Vec<&str> = snapshot.markdown.values().map(|text| &**text).collect();
            generation::parallel_map(&texts, self.workers, |text| {
                if text.contains(marker.as_str()) {
                    panic!("{FAULT_VARIABLE}: a page contains {marker}");
                }
            });
        }
        let analysis = content::analyze(
            &project,
            &snapshot,
            Mode::Dev,
            cache,
            &self.highlighter,
            self.workers,
        );
        let crate::rendering::Build {
            analysis,
            extensions,
            diagnostics: problems,
            guard,
            theme,
            ..
        } = crate::rendering::build_cached(
            &project,
            &snapshot,
            analysis,
            cache,
            &self.highlighter,
            crate::rust_site::BuildMode::Development,
            rust.as_ref(),
            &*self.observer,
        );
        diagnostics.extend(problems);
        if let Some(prepared) = &guard {
            *rust = Some(prepared.clone());
        }
        let filter = Filter::of(
            &self.root,
            &project,
            Read {
                files: vec![&analysis.includes, &analysis.referenced],
                linked: Some(&snapshot.linked),
                inputs: vec![&snapshot.inputs, &analysis.inputs],
            },
        );
        diagnostics.extend(analysis.diagnostics.iter().cloned());
        if has_errors(&diagnostics) {
            let outcome = Outcome::Failed {
                diagnostics: sorted(diagnostics),
                shell: shell(),
            };
            return (outcome, Some(filter));
        }
        let generated = generation::generate_extended(
            &project,
            &snapshot,
            &analysis,
            &self.dates,
            self.workers,
            &extensions,
        );
        if let Some(guard) = &guard {
            if let Err(problems) = guard.validate_current() {
                diagnostics.extend(problems);
            }
        }
        if let Some(theme) = &theme {
            diagnostics.extend(crate::theme::validate_snapshot(&project, &snapshot, theme));
        }
        diagnostics.extend(content::validate_inputs(&project, &snapshot, &analysis));
        if has_errors(&diagnostics) {
            return (
                Outcome::Failed {
                    diagnostics: sorted(diagnostics),
                    shell: shell(),
                },
                Some(filter),
            );
        }
        let outcome = match generated {
            Ok(generation) => Outcome::Built {
                files: generation.files,
                base: project.config.site.base.clone(),
                diagnostics: sorted(diagnostics),
            },
            Err(problems) => {
                diagnostics.extend(problems);
                Outcome::Failed {
                    diagnostics: sorted(diagnostics),
                    shell: shell(),
                }
            }
        };
        (outcome, Some(filter))
    }
}

/// The output paths whose bytes differ between two generations, removals
/// included.
fn changed_paths(
    before: Option<&BTreeMap<String, Arc<[u8]>>>,
    after: &BTreeMap<String, Arc<[u8]>>,
) -> Vec<String> {
    let Some(before) = before else {
        return after.keys().cloned().collect();
    };
    let mut changed: Vec<String> = after
        .iter()
        .filter(|(path, bytes)| {
            before
                .get(*path)
                .is_none_or(|old| !Arc::ptr_eq(old, bytes) && old[..] != bytes[..])
        })
        .map(|(path, _)| path.clone())
        .collect();
    changed.extend(
        before
            .keys()
            .filter(|path| !after.contains_key(*path))
            .cloned(),
    );
    changed
}

/// What a round changed, for the terminal.
enum Committed {
    /// Nothing an open page would notice.
    Unchanged,
    Built {
        revision: u64,
        pages: usize,
    },
    Failed {
        revision: u64,
        good: Option<u64>,
    },
}

fn commit(shared: &Shared, outcome: Outcome) -> Committed {
    let mut state = shared
        .state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let committed = match outcome {
        Outcome::Built {
            files,
            base,
            diagnostics,
        } => {
            let changed = changed_paths(state.good.as_ref().map(|good| &*good.files), &files);
            let same_base = state.good.as_ref().is_some_and(|good| good.base == base);
            if changed.is_empty() && same_base && !state.failed && *state.diagnostics == diagnostics
            {
                state.building = false;
                state.sequence += 1;
                shared.changed.notify_all();
                return Committed::Unchanged;
            }
            state.revision += 1;
            let revision = state.revision;
            let pages = changed
                .iter()
                .filter(|path| path.ends_with(".html"))
                .count();
            for path in changed {
                state.changed_at.insert(path, revision);
            }
            state.good = Some(Good {
                revision,
                files: Arc::new(files),
                base,
            });
            state.failed = false;
            state.diagnostics = Arc::new(diagnostics);
            Committed::Built { revision, pages }
        }
        Outcome::Failed { diagnostics, shell } => {
            state.revision += 1;
            state.failed = true;
            state.diagnostics = Arc::new(diagnostics);
            if let Some((files, base)) = shell {
                state.shell = Some((Arc::new(files), base));
            }
            Committed::Failed {
                revision: state.revision,
                good: state.good.as_ref().map(|good| good.revision),
            }
        }
    };
    state.building = false;
    state.sequence += 1;
    shared.changed.notify_all();
    committed
}

/// Tells the author what a round did, listing its problems when they are
/// not the ones listed last time.
fn print(
    shared: &Shared,
    committed: &Committed,
    elapsed: Duration,
    printed: &mut Arc<Vec<Diagnostic>>,
) {
    let diagnostics = {
        let state = shared
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.diagnostics.clone()
    };
    if matches!(committed, Committed::Unchanged) {
        return;
    }
    if *diagnostics != **printed {
        for diagnostic in diagnostics.iter() {
            eprint!("{}", report::text(diagnostic));
        }
        *printed = diagnostics.clone();
    }
    let summary = report::Summary::of(&diagnostics, false);
    let ms = elapsed.as_millis();
    match committed {
        Committed::Built { revision, pages } => eprintln!(
            "dev: revision {revision}: {pages} page(s) changed, {} warning(s), {ms} ms",
            summary.warnings
        ),
        Committed::Failed { revision, good } => eprintln!(
            "dev: revision {revision} failed: {} error(s), {} warning(s), {ms} ms; {}",
            summary.errors,
            summary.warnings,
            match good {
                Some(good) => format!("still serving revision {good}"),
                None => "nothing has built yet".to_owned(),
            }
        ),
        Committed::Unchanged => {}
    }
}

/// The output path a page address names, whether or not it exists: what an
/// open page compares its build against.
fn page_key(base: &str, path: &str) -> String {
    let Some(path) = crate::preview::decode(path) else {
        return "404.html".into();
    };
    let Some(rest) = path.strip_prefix(base) else {
        return "404.html".into();
    };
    if rest.is_empty() || rest.ends_with('/') {
        format!("{rest}index.html")
    } else {
        format!("{rest}/index.html")
    }
}

/// Whether the page at `path`, a copy of build `seen`, is out of date.
fn needs_reload(state: &State, path: &str, seen: u64) -> bool {
    let Some(good) = &state.good else {
        return false;
    };
    let key = page_key(&good.base, path);
    let mut at = state.changed_at.get(&key).copied().unwrap_or(0);
    if !good.files.contains_key(&key) {
        at = at.max(state.changed_at.get("404.html").copied().unwrap_or(0));
    }
    at > seen
}

fn state_json(session: &str, state: &State, reload: bool) -> String {
    let summary = report::Summary::of(&state.diagnostics, false);
    serde_json::json!({
        "session": session,
        "revision": state.revision,
        "sequence":state.sequence,
        "building":state.building,
        "phase":state.phase,
        "compile":state.compile,
        "good": state.good.as_ref().map(|good| good.revision),
        "ok": !state.failed,
        "reload": reload,
        "errors": summary.errors,
        "warnings": summary.warnings,
        "diagnostics": state.diagnostics.iter().map(report::diagnostic_json).collect::<Vec<_>>(),
    })
    .to_string()
}

fn query_value(query: &str, name: &str) -> Option<String> {
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key == name).then(|| crate::preview::decode(&value.replace('+', " ")).unwrap_or_default())
    })
}

/// One open page's event stream: the current state at once, then every
/// change, each telling the page whether to reload.
fn events(mut stream: TcpStream, shared: &Shared, query: &str) -> std::io::Result<()> {
    let path = query_value(query, "path").unwrap_or_else(|| "/".into());
    let mut seen_build: u64 = query_value(query, "good")
        .and_then(|good| good.parse().ok())
        .unwrap_or(0);
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    stream.write_all(
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream; charset=utf-8\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nContent-Security-Policy: {CSP}\r\nConnection: keep-alive\r\n\r\nretry: 500\n\n"
        )
        .as_bytes(),
    )?;
    stream.flush()?;
    let mut seen: Option<u64> = None;
    loop {
        let message = {
            let state = shared
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let (state, _) = shared
                .changed
                .wait_timeout_while(state, KEEP_ALIVE, |state| Some(state.sequence) == seen)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if Some(state.sequence) == seen {
                None
            } else {
                seen = Some(state.sequence);
                let reload = !state.building && needs_reload(&state, &path, seen_build);
                if !reload {
                    // The page is the same in the newest build.
                    if let Some(good) = &state.good {
                        seen_build = good.revision;
                    }
                }
                Some(state_json(&shared.session, &state, reload))
            }
        };
        match message {
            Some(json) => stream.write_all(format!("event: state\ndata: {json}\n\n").as_bytes())?,
            None => stream.write_all(b": idle\n\n")?,
        }
        stream.flush()?;
    }
}

fn respond(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
    head_only: bool,
    extra: &[(&str, String)],
) -> std::io::Result<()> {
    let mut head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nContent-Security-Policy: {CSP}\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\nConnection: close\r\n",
        body.len()
    );
    for (name, value) in extra {
        head.push_str(&format!("{name}: {value}\r\n"));
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes())?;
    if !head_only {
        stream.write_all(body)?;
    }
    stream.flush()
}

/// What to answer for a request path, from a generation in memory.
#[derive(Debug, PartialEq, Eq)]
enum Answer {
    File(String),
    Redirect(String),
    NotFound,
    BadRequest,
}

fn resolve(files: &BTreeMap<String, Arc<[u8]>>, base: &str, target: &str) -> Answer {
    let raw_path = target.split(['?', '#']).next().unwrap_or("");
    let Some(path) = crate::preview::decode(raw_path) else {
        return Answer::BadRequest;
    };
    if !path.starts_with('/') {
        return Answer::BadRequest;
    }
    if format!("{path}/") == base {
        return Answer::Redirect(format!("{raw_path}/"));
    }
    let Some(rest) = path.strip_prefix(base) else {
        return Answer::NotFound;
    };
    if rest
        .split('/')
        .any(|segment| segment == ".." || segment == ".")
    {
        return Answer::NotFound;
    }
    if rest.is_empty() || rest.ends_with('/') {
        let key = format!("{rest}index.html");
        return if files.contains_key(&key) {
            Answer::File(key)
        } else {
            Answer::NotFound
        };
    }
    if files.contains_key(rest) {
        Answer::File(rest.to_owned())
    } else if files.contains_key(&format!("{rest}/index.html")) {
        Answer::Redirect(format!("{raw_path}/"))
    } else {
        Answer::NotFound
    }
}

/// A page as served: its dev script told which run of `dev` and which
/// build it came from.
fn stamped(html: &[u8], session: &str, build: u64) -> Vec<u8> {
    let text = String::from_utf8_lossy(html);
    let from = format!("src=\"{DEV_SCRIPT_URL}\"");
    let to = format!("src=\"{DEV_SCRIPT_URL}?session={session}&amp;good={build}\"");
    text.replacen(&from, &to, 1).into_bytes()
}

fn handle(mut stream: TcpStream, shared: &Shared) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
    }
    let mut parts = request_line.split_whitespace();
    let (method, target) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let head_only = method == "HEAD";
    if method != "GET" && !head_only {
        return respond(
            &mut stream,
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            b"405 method not allowed\n",
            false,
            &[("Allow", "GET, HEAD".into())],
        );
    }
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    if let Some(name) = path.strip_prefix(DEV_PREFIX) {
        return match name {
            "dev.js" => respond(
                &mut stream,
                "200 OK",
                "text/javascript; charset=utf-8",
                generation::DEV_SCRIPT.as_bytes(),
                head_only,
                &[],
            ),
            "dev.css" => respond(
                &mut stream,
                "200 OK",
                "text/css; charset=utf-8",
                generation::DEV_STYLE.as_bytes(),
                head_only,
                &[],
            ),
            "events" if !head_only => events(stream, shared, query),
            _ => respond(
                &mut stream,
                "404 Not Found",
                "text/plain; charset=utf-8",
                b"404 not found\n",
                head_only,
                &[],
            ),
        };
    }
    let (files, base, build, built, building) = {
        let state = shared
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match (&state.good, &state.shell) {
            (Some(good), _) => (
                good.files.clone(),
                good.base.clone(),
                good.revision,
                true,
                state.building,
            ),
            (None, Some((files, base))) => (files.clone(), base.clone(), 0, false, state.building),
            (None, None) => {
                drop(state);
                return respond(
                    &mut stream,
                    "503 Service Unavailable",
                    "text/plain; charset=utf-8",
                    b"rustify-docs dev: the site does not build yet; the terminal lists the problems\n",
                    head_only,
                    &[],
                );
            }
        }
    };
    let html = |bytes: &[u8]| stamped(bytes, &shared.session, build);
    let page = |status: &str, key: &str, stream: &mut TcpStream| match files.get(key) {
        Some(bytes) => respond(
            stream,
            status,
            "text/html; charset=utf-8",
            &html(bytes),
            head_only,
            &[],
        ),
        None => respond(
            stream,
            status,
            "text/plain; charset=utf-8",
            b"404 not found\n",
            head_only,
            &[],
        ),
    };
    match resolve(&files, &base, target) {
        Answer::File(key) if built || key != generation::DEV_SHELL_PAGE => {
            let bytes = &files[&key];
            let content_type = crate::preview::mime(Path::new(&key));
            if key.ends_with(".html") {
                respond(
                    &mut stream,
                    "200 OK",
                    content_type,
                    &html(bytes),
                    head_only,
                    &[],
                )
            } else {
                respond(&mut stream, "200 OK", content_type, bytes, head_only, &[])
            }
        }
        Answer::Redirect(location) => respond(
            &mut stream,
            "301 Moved Permanently",
            "text/plain; charset=utf-8",
            b"moved\n",
            head_only,
            &[("Location", location)],
        ),
        Answer::BadRequest => respond(
            &mut stream,
            "400 Bad Request",
            "text/plain; charset=utf-8",
            b"400 bad request\n",
            head_only,
            &[],
        ),
        // While nothing has built, every page is the shell saying so.
        _ if !built => page(
            if building {
                "200 OK"
            } else {
                "500 Internal Server Error"
            },
            generation::DEV_SHELL_PAGE,
            &mut stream,
        ),
        _ => page("404 Not Found", "404.html", &mut stream),
    }
}

fn session_id() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    format!("{:x}-{:x}", std::process::id(), nanos)
}

fn phase(shared: &Shared, name: &'static str) {
    let mut state = shared
        .state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.building = true;
    state.phase = name;
    state.sequence += 1;
    shared.changed.notify_all();
    eprintln!("dev: {name}");
}

fn watch_parent(path: &Path) -> Option<PathBuf> {
    let mut directory = if path.is_dir() { path } else { path.parent()? };
    while !directory.is_dir() {
        directory = directory.parent()?;
    }
    std::fs::canonicalize(directory).ok()
}

fn fail(message: impl std::fmt::Display) -> ExitCode {
    eprintln!("dev failed: {message}");
    ExitCode::from(1)
}

pub fn run(options: Options) -> ExitCode {
    if !crate::UNWINDS {
        return fail(format!(
            "this rustify-docs was built to abort on a panic, so one failed build would end the session; install it with the docs-cli profile:\n  {}",
            crate::install()
        ));
    }
    // Rust first, before anything is watched or listened on.
    let rustc = match crate::new::doctor() {
        Ok(version) => version,
        Err(message) => return fail(message),
    };
    let root = crate::project_root(&options.config);
    let root = match std::fs::canonicalize(&root) {
        Ok(root) => root,
        Err(error) => return fail(format!("{}: {error}", root.display())),
    };
    let config = match options.config.file_name() {
        Some(name) => root.join(name),
        None => root.join(rustify_docs::config::FILE_NAME),
    };
    let project = match Project::load_file(&config) {
        Ok((project, _)) => project,
        Err(diagnostics) => {
            for diagnostic in &diagnostics {
                eprint!("{}", report::text(diagnostic));
            }
            return fail(format!(
                "{} has errors; fix them and run dev again",
                config.display()
            ));
        }
    };
    let rustc = if project.config.rust.is_some() {
        match crate::new::doctor_components() {
            Ok(version) => version,
            Err(message) => return fail(message),
        }
    } else {
        rustc
    };
    eprintln!("{rustc}");
    let dates = match rustify_docs::history::read(&root, &project.config.content_dir) {
        Ok(dates) => dates,
        Err(reason) => {
            eprintln!("dev: pages show no last-updated date: {reason}");
            Dates::new()
        }
    };
    let fault = std::env::var(FAULT_VARIABLE)
        .ok()
        .and_then(|value| value.strip_prefix("panic:").map(str::to_owned));

    let filter = Arc::new(RwLock::new(Filter::of(&root, &project, Read::default())));
    let (changes, rounds) = mpsc::channel::<()>();
    let reschedule = changes.clone();
    let watch_filter = filter.clone();
    let mut watcher =
        match notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
            let relevant = match watch_filter.read() {
                Ok(filter) => wants_round(&event, &filter),
                Err(_) => true,
            };
            if relevant {
                let _ = changes.send(());
            }
        }) {
            Ok(watcher) => watcher,
            Err(error) => return fail(format!("cannot watch {}: {error}", root.display())),
        };
    if let Err(error) =
        notify::Watcher::watch(&mut watcher, &root, notify::RecursiveMode::Recursive)
    {
        return fail(format!("cannot watch {}: {error}", root.display()));
    }
    let watcher = Arc::new(Mutex::new(watcher));
    let watched = Arc::new(Mutex::new(BTreeSet::from([root.clone()])));

    let shared = Arc::new(Shared {
        session: session_id(),
        state: Mutex::new(State {
            sequence: 0,
            building: true,
            phase: "render",
            compile: serde_json::Value::Null,
            revision: 0,
            good: None,
            failed: false,
            diagnostics: Arc::new(Vec::new()),
            changed_at: HashMap::new(),
            shell: generation::dev_shell_with_status(&project, true)
                .ok()
                .map(|shell| (Arc::new(shell.files), project.config.site.base.clone())),
        }),
        changed: Condvar::new(),
    });
    let progress_state = shared.clone();
    let progress_filter = filter.clone();
    let progress_watcher = watcher.clone();
    let observer = Arc::new(move |event| {
        use crate::rust_site::Progress;
        match event {
            Progress::Watch(plan) => {
                {
                    let mut filter = progress_filter
                        .write()
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                    filter.rust_dirs.extend(plan.rust_dirs.iter().cloned());
                    filter.build_dirs.extend(plan.build_dirs.iter().cloned());
                    filter.rust_files.extend(plan.files.iter().cloned());
                    for path in &plan.ignored {
                        if !filter.ignored.contains(path) {
                            filter.ignored.push(path.clone());
                        }
                    }
                }
                let paths = plan
                    .rust_dirs
                    .iter()
                    .chain(&plan.build_dirs)
                    .chain(&plan.files);
                let mut watched = watched
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                let mut watcher = progress_watcher
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                for path in paths.filter_map(|path| watch_parent(path)) {
                    if watched.iter().any(|directory| path.starts_with(directory)) {
                        continue;
                    }
                    match notify::Watcher::watch(
                        &mut *watcher,
                        &path,
                        notify::RecursiveMode::Recursive,
                    ) {
                        Ok(()) => {
                            watched.insert(path);
                        }
                        Err(error) => eprintln!("dev: cannot watch {}: {error}", path.display()),
                    }
                }
            }
            Progress::Host => phase(&progress_state, "host"),
            Progress::Wasm => phase(&progress_state, "wasm"),
            Progress::Render { reused } => {
                phase(&progress_state, "render");
                if reused {
                    eprintln!("dev: reusing the compiled Rust generation");
                }
            }
            Progress::Compiled {
                generation,
                timings,
            } => {
                let mut state = progress_state
                    .state
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                state.compile = serde_json::json!({"generation":generation,"completed_at_ms":timings.host.completed_at_ms.max(timings.wasm.completed_at_ms), "host":crate::rendering::timing(&timings.host),"wasm":crate::rendering::timing(&timings.wasm)});
                state.sequence += 1;
                progress_state.changed.notify_all();
            }
        }
    });
    let mut builder = Builder {
        root: root.clone(),
        config,
        highlighter: Highlighter::new(),
        cache: FragmentCache::default(),
        dates,
        workers: generation::default_workers(),
        fault,
        filter,
        rust: None,
        observer,
    };
    let mut printed = Arc::new(Vec::new());

    let host = "127.0.0.1";
    let listener = match options.port {
        Some(port) => TcpListener::bind((host, port))
            .map_err(|error| format!("cannot listen on {host}:{port}: {error}")),
        None => crate::preview::bind(host, None, project.config.dev_port),
    };
    let listener = match listener {
        Ok(listener) => listener,
        Err(message) => return fail(message),
    };
    let address = match listener.local_addr() {
        Ok(address) => address,
        Err(error) => return fail(error),
    };
    let base = {
        let state = shared
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match (&state.good, &state.shell) {
            (Some(good), _) => good.base.clone(),
            (_, Some((_, base))) => base.clone(),
            _ => project.config.site.base.clone(),
        }
    };
    if options.port.is_none() && address.port() != project.config.dev_port {
        eprintln!(
            "dev: port {} is in use; listening on {} instead",
            project.config.dev_port,
            address.port()
        );
    }
    println!("dev: http://{address}{base}");
    eprintln!(
        "watching {}; drafts are shown, source and published output are unchanged, stop with Ctrl-C",
        root.display()
    );
    let _ = std::io::stdout().flush();

    let server = shared.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let shared = server.clone();
            std::thread::spawn(move || {
                let _ = handle(stream, &shared);
            });
        }
    });

    let started = Instant::now();
    let outcome = builder.round();
    if stale(&outcome) {
        eprintln!("dev: inputs changed; discarding the candidate and rebuilding");
        let _ = reschedule.send(());
    } else {
        let committed = commit(&shared, outcome);
        print(&shared, &committed, started.elapsed(), &mut printed);
    }

    // The builder: one round per burst of saves; saves during a round make
    // the next one.
    while rounds.recv().is_ok() {
        let first = Instant::now();
        loop {
            match rounds.recv_timeout(SETTLE) {
                Ok(()) if first.elapsed() < SETTLE_AT_MOST => continue,
                Ok(()) | Err(mpsc::RecvTimeoutError::Timeout) => break,
                Err(mpsc::RecvTimeoutError::Disconnected) => return ExitCode::SUCCESS,
            }
        }
        let started = Instant::now();
        phase(&shared, "render");
        let outcome = builder.round();
        if stale(&outcome) {
            eprintln!("dev: inputs changed; discarding the candidate and rebuilding");
            let _ = reschedule.send(());
            continue;
        }
        let committed = commit(&shared, outcome);
        print(&shared, &committed, started.elapsed(), &mut printed);
    }
    drop(watcher);
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FilterFixture(PathBuf);

    impl FilterFixture {
        fn new(selected: &str) -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let root = std::env::temp_dir().join(format!(
                "rustify-docs-filter-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            ));
            std::fs::create_dir_all(&root).unwrap();
            let fixture = Self(std::fs::canonicalize(root).unwrap());
            fixture.write("rustify-docs.toml", &format!(
                "schema_version=1\n[site]\nname='Docs'\nurl='https://example.com/'\n[content]\ninclude=['{selected}']\n[repo]\nurl='https://github.com/owner/repo'\nref='main'\n"
            ));
            fixture
        }

        fn write(&self, path: &str, text: &str) {
            let path = self.0.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }

        fn project(&self) -> Project {
            Project::load(&self.0).unwrap().0
        }

        fn filter(&self) -> Filter {
            let project = self.project();
            let (snapshot, problems) = content::snapshot(&project);
            assert!(problems.is_empty(), "{problems:?}");
            let analysis = content::analyze(
                &project,
                &snapshot,
                Mode::Dev,
                &mut FragmentCache::default(),
                &Highlighter::new(),
                1,
            );
            assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
            Filter::of(
                &self.0,
                &project,
                Read {
                    files: vec![&analysis.includes, &analysis.referenced],
                    linked: Some(&snapshot.linked),
                    inputs: vec![&snapshot.inputs, &analysis.inputs],
                },
            )
        }
    }

    impl Drop for FilterFixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn include_filter_watches_actual_reads_and_ignores_unselected_body_events() {
        let fixture = FilterFixture::new("index.md");
        fixture.write("content/index.md", "# Home\n\n![Image](images/icon.svg) [Source](excluded.md)\n\n<!-- rustify:include path=\"README.md\" -->\n");
        fixture.write("content/images/icon.svg", "<svg width='1' height='1'/>");
        fixture.write("content/excluded.md", "excluded body");
        fixture.write("content/validation/evidence.md", "evidence");
        fixture.write("content/unused.bin", "unreferenced");
        fixture.write("README.md", "read once as an explicit include");
        let filter = fixture.filter();
        for path in [
            "content/index.md",
            "content/images/icon.svg",
            "README.md",
            "static/new.svg",
        ] {
            assert!(filter.relevant(&fixture.0.join(path)), "{path}");
        }
        for path in [
            "content/excluded.md",
            "content/validation/evidence.md",
            "content/unused.bin",
            "content",
            "content/validation",
        ] {
            assert!(!filter.relevant(&fixture.0.join(path)), "{path}");
        }
    }

    #[test]
    fn include_filter_repo_targets_ignore_atomic_body_saves_but_detect_deletion() {
        let fixture = FilterFixture::new("index.md");
        fixture.write("content/index.md", "# Home\n\n[Source](excluded.md)\n");
        fixture.write("content/excluded.md", "old excluded body");
        let filter = fixture.filter();
        let excluded = fixture.0.join("content/excluded.md");
        assert!(!filter.files.contains(&excluded));
        fixture.write("replacement.md", "new excluded body");
        std::fs::rename(fixture.0.join("replacement.md"), &excluded).unwrap();
        assert!(!filter.relevant(&excluded));
        assert!(!filter.relevant(&fixture.0.join("content")));
        std::fs::remove_file(&excluded).unwrap();
        assert!(filter.relevant(&excluded));
    }

    #[test]
    fn include_filter_initial_and_failed_selection_recovers_missing_ancestors() {
        let fixture = FilterFixture::new("guide/nested/index.md");
        let project = fixture.project();
        let initial = Filter::of(&fixture.0, &project, Read::default());
        let content = fixture.0.join("content");
        std::fs::create_dir(&content).unwrap();
        assert!(initial.relevant(&content));
        let (snapshot, _) = content::snapshot(&project);
        let failed = Filter::of(
            &fixture.0,
            &project,
            Read {
                inputs: vec![&snapshot.inputs],
                linked: Some(&snapshot.linked),
                ..Read::default()
            },
        );
        let nested = content.join("guide/nested");
        std::fs::create_dir_all(&nested).unwrap();
        assert!(failed.relevant(&nested));
        assert!(failed.relevant(&content.join("guide")));
        assert!(failed.relevant(&nested.join("index.md")));
        assert!(!failed.relevant(&content.join("validation/unselected.md")));
    }

    #[cfg(unix)]
    #[test]
    fn include_filter_follows_selected_real_paths_and_detects_retarget_and_replacement() {
        let fixture = FilterFixture::new("index.md");
        fixture.write("first/index.md", "# Home\n\n[Source](excluded.md)\n");
        fixture.write("first/excluded.md", "excluded body");
        fixture.write("second/index.md", "# Home\n\nThe replacement home.\n");
        let content = fixture.0.join("content");
        std::os::unix::fs::symlink("first", &content).unwrap();
        let filter = fixture.filter();
        assert!(filter.relevant(&fixture.0.join("first/index.md")));
        assert!(!filter.relevant(&fixture.0.join("first/excluded.md")));
        assert!(!filter.relevant(&fixture.0.join("first/validation/new.md")));
        std::fs::remove_file(&content).unwrap();
        std::os::unix::fs::symlink("second", &content).unwrap();
        assert!(filter.relevant(&content));

        let filter = fixture.filter();
        std::fs::rename(fixture.0.join("second"), fixture.0.join("previous")).unwrap();
        std::fs::create_dir(fixture.0.join("second")).unwrap();
        assert!(filter.relevant(&fixture.0.join("second")));

        let initial = Filter::of(&fixture.0, &fixture.project(), Read::default());
        assert!(initial.relevant(&fixture.0.join("second/index.md")));
    }

    fn files(entries: &[(&str, &str)]) -> BTreeMap<String, Arc<[u8]>> {
        entries
            .iter()
            .map(|(path, text)| (path.to_string(), Arc::from(text.as_bytes())))
            .collect()
    }

    #[test]
    fn requests_map_onto_the_generation() {
        let site = files(&[
            ("index.html", "home"),
            ("guide/index.html", "guide"),
            ("assets/site.css", "css"),
            ("404.html", "missing"),
        ]);
        assert_eq!(
            resolve(&site, "/docs/", "/docs/"),
            Answer::File("index.html".into())
        );
        assert_eq!(
            resolve(&site, "/docs/", "/docs/guide/?x"),
            Answer::File("guide/index.html".into())
        );
        assert_eq!(
            resolve(&site, "/docs/", "/docs/guide"),
            Answer::Redirect("/docs/guide/".into())
        );
        assert_eq!(
            resolve(&site, "/docs/", "/docs"),
            Answer::Redirect("/docs/".into())
        );
        assert_eq!(
            resolve(&site, "/docs/", "/docs/assets/site.css"),
            Answer::File("assets/site.css".into())
        );
        assert_eq!(resolve(&site, "/docs/", "/guide/"), Answer::NotFound);
        assert_eq!(resolve(&site, "/docs/", "/docs/no/"), Answer::NotFound);
        assert_eq!(resolve(&site, "/docs/", "/docs/%2e%2e/x"), Answer::NotFound);
        assert_eq!(resolve(&site, "/docs/", "/docs/%ff"), Answer::BadRequest);
    }

    #[test]
    fn a_page_reloads_only_when_its_own_build_output_changed() {
        let mut state = State {
            sequence: 0,
            building: false,
            phase: "render",
            compile: serde_json::Value::Null,
            revision: 0,
            good: None,
            failed: false,
            diagnostics: Arc::new(Vec::new()),
            changed_at: HashMap::new(),
            shell: None,
        };
        // Nothing has built: nothing to reload to.
        assert!(!needs_reload(&state, "/", 0));
        let one = files(&[
            ("index.html", "a"),
            ("guide/index.html", "b"),
            ("404.html", "x"),
        ]);
        for path in changed_paths(None, &one) {
            state.changed_at.insert(path, 1);
        }
        state.good = Some(Good {
            revision: 1,
            files: Arc::new(one.clone()),
            base: "/".into(),
        });
        // The shell (build 0) gives way to the first build, a missing page
        // to the not-found page.
        assert!(needs_reload(&state, "/guide/", 0));
        assert!(needs_reload(&state, "/nothing/", 0));
        assert!(!needs_reload(&state, "/guide/", 1));
        let two = files(&[
            ("index.html", "a"),
            ("guide/index.html", "B"),
            ("404.html", "x"),
        ]);
        assert_eq!(changed_paths(Some(&one), &two), vec!["guide/index.html"]);
        state.changed_at.insert("guide/index.html".into(), 2);
        assert!(needs_reload(&state, "/guide/", 1));
        assert!(!needs_reload(&state, "/", 1));
        // A removed page counts as changed for whoever was reading it.
        let three = files(&[("index.html", "a"), ("404.html", "x")]);
        assert_eq!(changed_paths(Some(&two), &three), vec!["guide/index.html"]);
        state.changed_at.insert("guide/index.html".into(), 3);
        assert!(needs_reload(&state, "/guide/", 2));
        assert!(!needs_reload(&state, "/%E4%B8%AD/", 3));
    }

    #[test]
    fn page_keys_ignore_existence() {
        assert_eq!(page_key("/docs/", "/docs/"), "index.html");
        assert_eq!(page_key("/docs/", "/docs/a/b/"), "a/b/index.html");
        assert_eq!(page_key("/docs/", "/docs/a"), "a/index.html");
        assert_eq!(page_key("/docs/", "/elsewhere/"), "404.html");
        assert_eq!(page_key("/", "/%E4%B8%AD%E6%96%87/"), "中文/index.html");
    }

    #[test]
    fn served_pages_say_which_run_and_build_they_are() {
        let html = format!("<script src=\"{DEV_SCRIPT_URL}\" defer=\"\"></script>");
        let out = String::from_utf8(stamped(html.as_bytes(), "s1", 7)).unwrap();
        assert_eq!(
            out,
            format!("<script src=\"{DEV_SCRIPT_URL}?session=s1&amp;good=7\" defer=\"\"></script>")
        );
    }

    #[test]
    fn the_filter_watches_inputs_and_never_the_output() {
        let root = Path::new("/p");
        let filter = Filter {
            config: root.join("rustify-docs.toml"),
            dirs: vec![root.join("content"), root.join("static")],
            files: [root.join("src/example.rs")].into_iter().collect(),
            existence: BTreeMap::new(),
            ancestors: BTreeMap::new(),
            ignored: vec![root.join("dist")],
            rust_dirs: BTreeSet::new(),
            rust_files: BTreeSet::new(),
            build_dirs: BTreeSet::new(),
        };
        assert!(filter.relevant(&root.join("rustify-docs.toml")));
        assert!(filter.relevant(&root.join("content/guide/a.md")));
        assert!(filter.relevant(&root.join("content")));
        assert!(filter.relevant(&root.join("static/logo.svg")));
        assert!(filter.relevant(&root.join("src/example.rs")));
        assert!(!filter.relevant(&root.join("src/other.rs")));
        assert!(!filter.relevant(&root.join("dist/index.html")));
        assert!(!filter.relevant(&root.join("content/.git/HEAD")));
        assert!(!filter.relevant(&root.join(".rustify-docs/dist/lock")));
        assert!(!filter.relevant(&root.join("README.md")));
    }

    #[test]
    fn the_filter_follows_what_a_round_read() {
        let (project, _) = Project::load_file(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/docs-ssg/fixtures/content/rustify-docs.toml"
        )))
        .unwrap();
        let root = Path::new("/p");
        let includes: BTreeSet<String> = ["src/example.rs".to_owned()].into();
        let referenced: BTreeSet<String> =
            ["images/mark.svg".to_owned(), "brand/logo.svg".to_owned()].into();
        let linked: BTreeSet<String> = ["shared".to_owned()].into();
        let filter = Filter::of(
            root,
            &project,
            Read {
                files: vec![&includes, &referenced],
                linked: Some(&linked),
                ..Read::default()
            },
        );
        for path in [
            "rustify-docs.toml",
            "src/example.rs",
            "images/mark.svg",
            "brand/logo.svg",
            "shared/a.svg",
        ] {
            assert!(filter.relevant(&root.join(path)), "{path}");
        }
        for path in [
            "images/other.svg",
            "brand",
            "dist/index.html",
            "custom.toml",
        ] {
            assert!(!filter.relevant(&root.join(path)), "{path}");
        }
    }

    #[test]
    fn dropped_events_and_watcher_errors_read_everything_again() {
        use notify::event::{EventKind, Flag, ModifyKind};
        let root = Path::new("/p");
        let filter = Filter {
            config: root.join("rustify-docs.toml"),
            dirs: vec![root.join("content")],
            files: BTreeSet::new(),
            existence: BTreeMap::new(),
            ancestors: BTreeMap::new(),
            ignored: vec![root.join("dist")],
            rust_dirs: BTreeSet::new(),
            rust_files: BTreeSet::new(),
            build_dirs: BTreeSet::new(),
        };
        let event = |path: &str| {
            notify::Event::new(EventKind::Modify(ModifyKind::Any)).add_path(root.join(path))
        };
        assert!(wants_round(&Ok(event("content/a.md")), &filter));
        assert!(!wants_round(&Ok(event("dist/a.html")), &filter));
        assert!(wants_round(
            &Ok(event("dist/a.html").set_flag(Flag::Rescan)),
            &filter
        ));
        assert!(wants_round(
            &Err(notify::Error::generic("overflow")),
            &filter
        ));
    }

    #[test]
    fn query_values_decode() {
        assert_eq!(
            query_value("path=%2Fdocs%2Fa%20b%2F&good=3", "path").as_deref(),
            Some("/docs/a b/")
        );
        assert_eq!(query_value("path=%2F&good=3", "good").as_deref(), Some("3"));
        assert_eq!(query_value("path=%2F", "good"), None);
    }
}
