//! The content graph: one analysis of a project's configuration, Markdown
//! and files, from which every command takes what it needs.
//!
//! It owns the site's addresses (a file's route, a heading's anchor, a
//! resource's location), resolves every link and image against them, and
//! collects every problem it finds into one sorted list, so an author sees
//! all of them in one run. It reads files and nothing else: it writes no
//! output and makes no network request.

use crate::diagnostic_text::{code_span, literal};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::config::{self, Config, GroupSource};
use crate::diagnostic::{Issue, LineIndex, Severity};
use crate::frontmatter::{self, Front, Layout};
use crate::highlight::{Highlighter, GRAMMAR_VERSION};
use crate::labels;
use crate::markdown::{self, Fragment, IncludeFiles, RefKind, WRITER_VERSION};
use crate::shell::{NavGroup, NavItem};
use crate::site::Slot;

/// A located problem, in the shape `check --format json` prints.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Diagnostic {
    /// Project-relative, with `/`; `None` for a problem with no file.
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub severity: Severity,
    pub rule: String,
    pub message: String,
    pub suggestion: Option<String>,
}

impl Diagnostic {
    pub fn located(file: &str, text: &str, issue: Issue) -> Self {
        let (line, column) = LineIndex::new(text).position(issue.span.start);
        Self {
            file: Some(file.to_owned()),
            line: Some(line),
            column: Some(column),
            severity: issue.severity,
            rule: issue.rule.to_owned(),
            message: issue.message,
            suggestion: issue.suggestion,
        }
    }

    pub fn general(severity: Severity, rule: &str, message: impl Into<String>) -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            severity,
            rule: rule.to_owned(),
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn in_file(file: &str, severity: Severity, rule: &str, message: impl Into<String>) -> Self {
        Self {
            file: Some(file.to_owned()),
            ..Self::general(severity, rule, message)
        }
    }

    fn rendered(file: &str, issue: Issue) -> Self {
        Self {
            suggestion: issue.suggestion,
            ..Self::in_file(file, issue.severity, issue.rule, issue.message)
        }
    }
}

/// Whether drafts are part of the site being analysed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// `build` and `check`: drafts are left out, and linking to one is an
    /// error.
    Production,
    /// `dev`: drafts are shown and marked; linking to one is a warning.
    Dev,
}

/// A project: its root and validated configuration.
#[derive(Clone, Debug)]
pub struct Project {
    pub root: PathBuf,
    /// The configuration's file name in `root`: `rustify-docs.toml`, unless
    /// the project was loaded from another.
    pub config_file: String,
    pub config: Config,
    pub config_text: String,
    /// The configuration's original read, including its symbolic-link target.
    pub inputs: InputReceipts,
}

impl Project {
    /// Reads `rustify-docs.toml` in `root`.
    pub fn load(root: &Path) -> Result<(Self, Vec<Diagnostic>), Vec<Diagnostic>> {
        Self::load_file(&root.join(config::FILE_NAME))
    }

    /// Reads the configuration at `path`; the project is the directory that
    /// holds it.
    pub fn load_file(path: &Path) -> Result<(Self, Vec<Diagnostic>), Vec<Diagnostic>> {
        let root = match path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
            _ => PathBuf::from("."),
        };
        let name = path
            .file_name()
            .map_or_else(|| config::FILE_NAME.into(), |name| name.to_string_lossy());
        let name = name.as_ref();
        let read = || -> std::io::Result<(PathBuf, String)> {
            let real = std::fs::canonicalize(path)?;
            let text = std::fs::read_to_string(&real)?;
            Ok((real, text))
        };
        let (real, text) = read().map_err(|error| {
            vec![Diagnostic::in_file(
                name,
                Severity::Error,
                "config-missing",
                format!("cannot read {}: {error}", literal(name)),
            )]
        })?;
        let mut inputs = InputReceipts::default();
        inputs.record(name, real, Some(Arc::from(text.as_bytes())), false);
        let locate = |issues: Vec<Issue>| -> Vec<Diagnostic> {
            issues
                .into_iter()
                .map(|issue| Diagnostic::located(name, &text, issue))
                .collect()
        };
        match config::parse(&text) {
            Ok((config, warnings)) => Ok((
                Self {
                    root,
                    config_file: name.to_owned(),
                    config,
                    config_text: text.clone(),
                    inputs,
                },
                locate(warnings),
            )),
            Err(errors) => Err(locate(errors)),
        }
    }

    pub fn content_root(&self) -> PathBuf {
        self.root.join(&self.config.content_dir)
    }

    pub fn assets_root(&self) -> PathBuf {
        self.root.join(&self.config.assets_dir)
    }
}

/// Files whose names say they hold secrets or repository internals. They are
/// never published, wherever they sit.
pub fn is_private(path: &str) -> bool {
    path.split('/').any(|part| {
        part == ".git"
            || part == ".rustify-docs"
            || part == ".env"
            || part.starts_with(".env.")
            || part.ends_with(".env")
            || part == ".DS_Store"
    })
}

/// The project's inputs, read once: Markdown by content-relative path, and
/// other files discovered in content when selection is recursive, the assets,
/// and the explicitly configured theme document.
#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub markdown: BTreeMap<String, Arc<str>>,
    /// Project messages are inputs only, never public resources.
    pub messages: BTreeMap<String, Arc<str>>,
    /// Project-relative bytes; the theme document is input only, not a resource.
    pub files: BTreeMap<String, Arc<[u8]>>,
    /// Where the symbolic links in those directories really lead, relative
    /// to the project: what `dev` watches besides the directories.
    pub linked: BTreeSet<String>,
    /// Exact original reads; separate from the files eligible for publication.
    pub inputs: InputReceipts,
    /// Original scan findings, so a stable invalid input is not mistaken for a change.
    pub diagnostics: Vec<Diagnostic>,
}

/// Original reads and existence checks used by one rendering round.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputReceipts {
    files: BTreeMap<String, InputRead>,
    changed: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InputRead {
    real: PathBuf,
    bytes: Option<Arc<[u8]>>,
    directory: bool,
}

impl InputReceipts {
    /// Combines original reads without replacing the first bytes or binding.
    #[doc(hidden)]
    pub fn extend(&mut self, other: Self) {
        for (alias, input) in other.files {
            self.record(&alias, input.real, input.bytes, input.directory);
        }
        self.changed.extend(other.changed);
    }

    /// Actual byte dependencies, retaining their alias and original real path.
    #[doc(hidden)]
    pub fn byte_paths(&self) -> impl Iterator<Item = (&str, &Path)> {
        self.files.iter().filter_map(|(alias, read)| {
            read.bytes
                .is_some()
                .then_some((alias.as_str(), read.real.as_path()))
        })
    }

    /// Existence dependencies; the last value says whether the target is a directory.
    #[doc(hidden)]
    pub fn existence_paths(&self) -> impl Iterator<Item = (&str, &Path, bool)> {
        self.files.iter().filter_map(|(alias, read)| {
            read.bytes
                .is_none()
                .then_some((alias.as_str(), read.real.as_path(), read.directory))
        })
    }

    fn record(&mut self, alias: &str, real: PathBuf, bytes: Option<Arc<[u8]>>, directory: bool) {
        match self.files.entry(alias.to_owned()) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(InputRead {
                    real,
                    bytes,
                    directory,
                });
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                let first = entry.get_mut();
                if first.real != real || first.directory != directory {
                    self.changed.insert(alias.to_owned());
                }
                if let Some(bytes) = bytes {
                    match &first.bytes {
                        Some(original) if original != &bytes => {
                            self.changed.insert(alias.to_owned());
                        }
                        None => first.bytes = Some(bytes),
                        _ => {}
                    }
                }
            }
        }
    }

    fn bytes(&self, alias: &str) -> Option<Arc<[u8]>> {
        self.files.get(alias)?.bytes.clone()
    }
}

/// Why a file the site names may not be read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refusal {
    Missing,
    /// It is reached through a symbolic link that leads out of the project.
    Outside,
    /// It, or what a link to it leads to, is a private file.
    Private,
    /// A configuration input, not a publishable resource.
    ThemeDocument,
    Messages,
}

/// The project's root as the file system resolves it, symbolic links and
/// all; what every real path is compared with.
fn real_root(project: &Project) -> PathBuf {
    std::fs::canonicalize(&project.root).unwrap_or_else(|_| project.root.clone())
}

/// Where `path` really leads, relative to `root` (a real root): refused
/// when that is outside the project, or private.
fn real_source(root: &Path, path: &Path) -> Result<String, Refusal> {
    let real = std::fs::canonicalize(path).map_err(|_| Refusal::Missing)?;
    let relative = real.strip_prefix(root).map_err(|_| Refusal::Outside)?;
    let relative = relative.to_string_lossy().replace('\\', "/");
    if is_private(&relative) {
        return Err(Refusal::Private);
    }
    Ok(relative)
}

/// Resolve a dependency even when its last file is missing. An editor may
/// unlink a symlink's target before writing it back; its next save must
/// still matter to the watcher. This path never authorizes a read.
fn watched_source(root: &Path, path: &Path) -> Option<String> {
    let mut path = path.to_path_buf();
    let mut tail = PathBuf::new();
    let mut seen = BTreeSet::new();
    while seen.insert(path.clone()) {
        if let Ok(real) = std::fs::canonicalize(&path) {
            let full = real.join(&tail);
            let relative = full.strip_prefix(root).ok()?.to_string_lossy();
            return join("", &relative).filter(|path| !is_private(path));
        }
        if let Ok(target) = std::fs::read_link(&path) {
            path = if target.is_absolute() {
                target
            } else {
                path.parent()?.join(target)
            };
        } else {
            tail = PathBuf::from(path.file_name()?).join(tail);
            path.pop();
        }
    }
    None
}

fn theme_paths(project: &Project, root: &Path) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    if let Some((path, _)) = &project.config.theme_document {
        paths.insert(path.clone());
        paths.extend(watched_source(root, &project.root.join(path)));
    }
    paths
}

fn message_paths(project: &Project, root: &Path) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    for (path, _) in project
        .config
        .locales
        .iter()
        .filter_map(|locale| locale.messages.as_ref())
    {
        paths.insert(path.clone());
        paths.extend(watched_source(root, &project.root.join(path)));
    }
    paths
}

/// Message aliases and their existing or prospective targets within the project.
/// These paths only select watch events; they never authorize file reads.
#[doc(hidden)]
pub fn message_dependencies(project: &Project) -> BTreeSet<String> {
    message_paths(project, &real_root(project))
}

fn matches_theme(project: &Project, root: &Path, theme: &BTreeSet<String>, path: &str) -> bool {
    theme.contains(path)
        || (!theme.is_empty()
            && watched_source(root, &project.root.join(path))
                .is_some_and(|real| theme.contains(&real)))
}

/// Whether a file is the configured theme document, including symbolic aliases.
/// The JSON is a build input and must not be copied as a public asset.
pub fn is_theme_document(project: &Project, path: &str) -> bool {
    if project.config.theme_document.is_none() {
        return false;
    }
    let root = real_root(project);
    matches_theme(project, &root, &theme_paths(project, &root), path)
}

/// The bytes of the project file `path` (project-relative) names, read
/// only when neither the name nor what it really is says it is private and
/// it stays inside the project.
fn read_source(
    project: &Project,
    root: &Path,
    path: &str,
    inputs: &mut InputReceipts,
) -> Result<Arc<[u8]>, Refusal> {
    if is_private(path) {
        return Err(Refusal::Private);
    }
    let real = root.join(real_source(root, &project.root.join(path))?);
    if !std::fs::metadata(&real).is_ok_and(|meta| meta.is_file()) {
        return Err(Refusal::Missing);
    }
    let bytes: Arc<[u8]> = Arc::from(std::fs::read(&real).map_err(|_| Refusal::Missing)?);
    inputs.record(path, real, Some(Arc::clone(&bytes)), false);
    Ok(bytes)
}

impl Refusal {
    /// The error for a reference to a refused file, made by `locate` where
    /// the reference is written: `what` is how the message names it,
    /// `missing` the rule when the file does not exist.
    fn issue(
        self,
        locate: &dyn Fn(&'static str, String) -> Issue,
        what: &str,
        missing: &'static str,
    ) -> Issue {
        match self {
            Self::Missing => locate(missing, format!("{what} does not exist")),
            Self::Outside => locate(
                "symlink-outside",
                format!("{what} leads out of the project through a symbolic link; it is not read"),
            )
            .suggest("copy the file into the project"),
            Self::Private => locate(
                "private-file",
                format!("{what} is a private file, which is never published"),
            )
            .suggest("reference a file that holds only what the site may show"),
            Self::ThemeDocument => locate(
                "theme-document-resource",
                format!(
                    "{what} is the theme configuration input and cannot be published as a resource"
                ),
            ),
            Self::Messages => locate(
                "i18n-messages",
                format!("{what} is a messages input and cannot be published as a resource"),
            ),
        }
    }

    /// The error for a link in the content or asset directory that leads
    /// somewhere refused.
    fn walk_problem(self, file: &str, what: &str) -> Option<Diagnostic> {
        match self {
            Self::Missing | Self::ThemeDocument | Self::Messages => None,
            Self::Outside => Some(Diagnostic::in_file(
                file,
                Severity::Error,
                "symlink-outside",
                format!("this {what} points outside the project; it is not read"),
            )),
            Self::Private => Some(Diagnostic::in_file(
                file,
                Severity::Error,
                "private-file",
                format!("this {what} leads to a private file; it is not read"),
            )),
        }
    }
}

fn walk(
    dir: &Path,
    relative: &str,
    root: &Path,
    out: &mut Vec<(String, PathBuf)>,
    linked: &mut BTreeSet<String>,
    inputs: &mut InputReceipts,
    problems: &mut Vec<Diagnostic>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        let child = if relative.is_empty() {
            name.clone()
        } else {
            format!("{relative}/{name}")
        };
        // Never read, so never published, whatever else names them.
        if is_private(&child) {
            continue;
        }
        if entry.file_type().is_ok_and(|kind| kind.is_symlink()) {
            // A missing target still needs to wake dev when it is restored.
            // Watching it does not authorize the read below.
            if let Some(target) = watched_source(root, &path) {
                linked.insert(target);
            }
            // A link is followed only while it stays inside the project and
            // leads to nothing private.
            match real_source(root, &path) {
                Ok(_) => {}
                Err(refusal) => {
                    problems.extend(refusal.walk_problem(&child, "symbolic link"));
                    continue;
                }
            }
        }
        let Ok(meta) = std::fs::metadata(&path) else {
            continue;
        };
        if meta.is_dir() {
            if let Ok(real) = real_source(root, &path) {
                inputs.record(&child, root.join(real), None, true);
            }
            walk(&path, &child, root, out, linked, inputs, problems);
        } else if meta.is_file() {
            out.push((child, path));
        }
    }
}

/// Reads the content and asset directories and the configured theme document.
pub fn snapshot(project: &Project) -> (Snapshot, Vec<Diagnostic>) {
    let mut snapshot = Snapshot::default();
    let mut problems = Vec::new();
    let config = &project.config;
    let root = real_root(project);
    let theme = theme_paths(project, &root);
    let messages = message_paths(project, &root);
    let mut read_messages = BTreeSet::new();
    for (path, span) in config
        .locales
        .iter()
        .filter_map(|locale| locale.messages.as_ref())
    {
        if !read_messages.insert(path) {
            continue;
        }
        if let Some(real) =
            watched_source(&root, &project.root.join(path)).filter(|real| real != path)
        {
            snapshot.linked.insert(real);
        }
        match read_source(project, &root, path, &mut snapshot.inputs) {
            Ok(bytes) => match std::str::from_utf8(&bytes) {
                Ok(text) => {
                    snapshot.messages.insert(path.clone(), Arc::from(text));
                }
                Err(_) => problems.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::error(
                        span.clone(),
                        "i18n-messages",
                        format!("messages {} must be UTF-8", code_span(path)),
                    ),
                )),
            },
            Err(refusal) => {
                let mut issue = refusal.issue(
                    &|rule, message| Issue::error(span.clone(), rule, message),
                    &format!("messages {}", code_span(path)),
                    "i18n-messages",
                );
                issue.rule = "i18n-messages";
                problems.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    issue,
                ));
            }
        }
    }
    for (dir, is_content) in [(&config.content_dir, true), (&config.assets_dir, false)] {
        // The directory itself, or one on its path, may be a link too.
        let full = project.root.join(dir);
        if is_content {
            if let Some(selected) = &config.content_include {
                // Retain the directory identity without walking its descendants.
                if let Ok(real) = real_source(&root, &full) {
                    let real = root.join(real);
                    if let Ok(metadata) = std::fs::metadata(&real) {
                        snapshot.inputs.record(dir, real, None, metadata.is_dir());
                    }
                }
                for (file, span) in selected {
                    let relative = format!("{dir}/{file}");
                    if matches_theme(project, &root, &messages, &relative) {
                        problems.push(Diagnostic::in_file(
                            &relative,
                            Severity::Error,
                            "i18n-messages",
                            "a messages input cannot be selected as public content",
                        ));
                        continue;
                    }
                    if let Some(real) = watched_source(&root, &project.root.join(&relative))
                        .filter(|real| real != &relative)
                    {
                        snapshot.linked.insert(real);
                    }
                    match read_source(project, &root, &relative, &mut snapshot.inputs) {
                        Ok(bytes) if matches_theme(project, &root, &theme, &relative) => {
                            snapshot.files.insert(relative, bytes);
                        }
                        Ok(bytes) => match std::str::from_utf8(&bytes) {
                            Ok(text) => {
                                snapshot.markdown.insert(file.clone(), Arc::from(text));
                            }
                            Err(_) => problems.push(Diagnostic::in_file(
                                &relative,
                                Severity::Error,
                                "not-utf8",
                                "Markdown must be UTF-8",
                            )),
                        },
                        Err(refusal) => problems.push(Diagnostic::located(
                            &project.config_file,
                            &project.config_text,
                            refusal.issue(
                                &|rule, message| Issue::error(span.clone(), rule, message),
                                &format!("content.include {}", code_span(file)),
                                "content-include-missing",
                            ),
                        )),
                    }
                }
                continue;
            }
        }
        if std::fs::symlink_metadata(&full).is_err() {
            continue;
        }
        if let Some(target) = watched_source(&root, &full).filter(|target| target != dir) {
            snapshot.linked.insert(target);
        }
        match real_source(&root, &full) {
            Ok(real) => {
                snapshot.inputs.record(dir, root.join(real), None, true);
            }
            Err(refusal) => {
                problems.extend(refusal.walk_problem(dir, "directory"));
                continue;
            }
        }
        let mut files = Vec::new();
        walk(
            &full,
            dir,
            &root,
            &mut files,
            &mut snapshot.linked,
            &mut snapshot.inputs,
            &mut problems,
        );
        for (relative, _) in files {
            if matches_theme(project, &root, &messages, &relative) {
                problems.push(Diagnostic::in_file(
                    &relative,
                    Severity::Error,
                    "i18n-messages",
                    "a messages input cannot be published from the content or assets directory",
                ));
                continue;
            }
            let Ok(bytes) = read_source(project, &root, &relative, &mut snapshot.inputs) else {
                problems.push(Diagnostic::in_file(
                    &relative,
                    Severity::Error,
                    "unreadable",
                    "this file cannot be read",
                ));
                continue;
            };
            if matches_theme(project, &root, &theme, &relative) {
                snapshot.files.insert(relative, bytes);
                continue;
            }
            let content_relative = relative
                .strip_prefix(&format!("{}/", config.content_dir))
                .map(str::to_owned);
            match content_relative {
                Some(path)
                    if path.ends_with(".md")
                        && dir == &config.content_dir
                        && (is_content || config.content_include.is_none()) =>
                {
                    match std::str::from_utf8(&bytes) {
                        Ok(text) => {
                            snapshot.markdown.insert(path, Arc::from(text));
                        }
                        Err(_) => problems.push(Diagnostic::in_file(
                            &relative,
                            Severity::Error,
                            "not-utf8",
                            "Markdown must be UTF-8",
                        )),
                    }
                }
                _ => {
                    snapshot.files.insert(relative, bytes);
                }
            }
        }
    }
    if let Some((path, span)) = &config.theme_document {
        if let Some(real) =
            watched_source(&root, &project.root.join(path)).filter(|real| real != path)
        {
            snapshot.linked.insert(real);
        }
        if !snapshot.files.contains_key(path) {
            match read_source(project, &root, path, &mut snapshot.inputs) {
                Ok(bytes) => {
                    snapshot.files.insert(path.clone(), bytes);
                }
                Err(refusal) => problems.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    refusal.issue(
                        &|rule, message| Issue::error(span.clone(), rule, message),
                        &format!("theme.document {}", code_span(path)),
                        "theme-document-missing",
                    ),
                )),
            }
        }
    }
    snapshot.diagnostics = problems.clone();
    (snapshot, problems)
}

/// Rechecks the files read for this rendering round before serving its output.
pub fn validate_inputs(
    project: &Project,
    before: &Snapshot,
    analysis: &Analysis,
) -> Vec<Diagnostic> {
    let (after, problems) = snapshot(project);
    let mut changed = BTreeSet::new();
    if problems != before.diagnostics
        || after.markdown != before.markdown
        || after.files != before.files
        || after.linked != before.linked
    {
        changed.insert(project.config.content_dir.clone());
    }
    if !project.inputs.files.contains_key(&project.config_file)
        && !std::fs::read(project.root.join(&project.config_file))
            .is_ok_and(|bytes| bytes == project.config_text.as_bytes())
    {
        changed.insert(project.config_file.clone());
    }
    let mut targets = BTreeMap::new();
    let mut reads = BTreeMap::<PathBuf, Option<Arc<[u8]>>>::new();
    for (alias, current) in &after.inputs.files {
        targets.insert(
            alias.clone(),
            Some((
                current.real.clone(),
                current.directory,
                current.bytes.is_some(),
            )),
        );
        if let Some(bytes) = &current.bytes {
            reads
                .entry(current.real.clone())
                .or_insert_with(|| Some(Arc::clone(bytes)));
        }
    }
    for inputs in [&project.inputs, &before.inputs, &analysis.inputs] {
        changed.extend(inputs.changed.iter().cloned());
        for (alias, original) in &inputs.files {
            let target = targets.entry(alias.clone()).or_insert_with(|| {
                let real = std::fs::canonicalize(project.root.join(alias)).ok()?;
                let metadata = std::fs::metadata(&real).ok()?;
                Some((real, metadata.is_dir(), metadata.is_file()))
            });
            let Some((real, directory, file)) = target.as_ref() else {
                changed.insert(alias.clone());
                continue;
            };
            if real != &original.real || directory != &original.directory {
                changed.insert(alias.clone());
                continue;
            }
            if let Some(bytes) = &original.bytes {
                if !file {
                    changed.insert(alias.clone());
                    continue;
                }
                let current = reads
                    .entry(real.clone())
                    .or_insert_with(|| std::fs::read(real).ok().map(Arc::from));
                if current.as_ref() != Some(bytes) {
                    changed.insert(alias.clone());
                }
            }
        }
    }
    changed.into_iter().map(|path| {
        Diagnostic::in_file(
            &path,
            Severity::Error,
            "render-generation-changed",
            "An input changed after the original rendering read; discard this generation and rebuild from current inputs",
        )
    }).collect()
}

/// Everything a command reads besides the configuration: the files, and the
/// dates the repository's history gives them. A project with no readable
/// history builds without dates, with a warning saying why.
pub fn read_inputs(project: &Project) -> (Snapshot, crate::history::Dates, Vec<Diagnostic>) {
    let (snapshot, mut diagnostics) = snapshot(project);
    let dates = match crate::history::read(&project.root, &project.config.content_dir) {
        Ok(dates) => dates,
        Err(reason) => {
            diagnostics.push(
                Diagnostic::general(
                    Severity::Warning,
                    "no-git-history",
                    format!("pages show no last-updated date: {reason}"),
                )
                .with_suggestion("build inside a git repository with the content committed"),
            );
            crate::history::Dates::new()
        }
    };
    (snapshot, dates, diagnostics)
}

/// A content-relative Markdown path's route: `index.md` is its directory's,
/// `a/b.md` is `/a/b/`.
pub fn route_for(content_path: &str) -> String {
    let stem = content_path.strip_suffix(".md").unwrap_or(content_path);
    let stem = if stem == "index" {
        ""
    } else {
        stem.strip_suffix("/index").unwrap_or(stem)
    };
    if stem.is_empty() {
        "/".into()
    } else {
        format!("/{stem}/")
    }
}

/// Adds a configured language root to an already canonical logical route.
pub fn locale_route(config: &Config, lang: &str, logical_route: &str) -> String {
    match config
        .locales
        .iter()
        .find(|locale| locale.lang.eq_ignore_ascii_case(lang))
    {
        Some(locale) if !locale.prefix.is_empty() => format!("/{}{logical_route}", locale.prefix),
        _ => logical_route.to_owned(),
    }
}

fn markdown_locale<'a>(config: &'a Config, path: &'a str) -> (&'a str, &'a str) {
    for locale in &config.locales {
        if !locale.prefix.is_empty() {
            if let Some(path) = path.strip_prefix(&format!("{}/", locale.prefix)) {
                return (&locale.lang, path);
            }
        }
    }
    let default_lang = config
        .locales
        .iter()
        .find(|locale| locale.lang.eq_ignore_ascii_case(&config.site.lang))
        .map_or(config.site.lang.as_str(), |locale| locale.lang.as_str());
    (default_lang, path)
}

fn reserved_logical_route(config: &Config, route: &str) -> bool {
    route == "/404/"
        || reserved_output(config, route.trim_matches('/'))
        || config.locales.iter().any(|locale| {
            !locale.prefix.is_empty() && route.starts_with(&format!("/{}/", locale.prefix))
        })
}

fn reserved_output(config: &Config, output: &str) -> bool {
    let local = config
        .locales
        .iter()
        .find_map(|locale| {
            (!locale.prefix.is_empty())
                .then(|| output.strip_prefix(&format!("{}/", locale.prefix)))
                .flatten()
        })
        .unwrap_or(output);
    crate::generation::SITE_FILES
        .iter()
        .any(|file| local == *file || local.starts_with(&format!("{file}/")))
        || crate::generation::GENERATED_PREFIXES
            .iter()
            .any(|prefix| local.starts_with(prefix))
}

/// A fragment's cache key: everything that decides its bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FragmentKey([u8; 32]);

impl FragmentKey {
    pub fn of(
        body: &str,
        offset: usize,
        source_lang: &str,
        ui_lang: &str,
        labels_digest: &str,
        includes: &IncludeFiles,
    ) -> Self {
        let mut hash = Sha256::new();
        hash.update(WRITER_VERSION.to_le_bytes());
        hash.update(GRAMMAR_VERSION.as_bytes());
        hash.update([0]);
        hash.update(markdown::options().bits().to_le_bytes());
        for part in [source_lang, ui_lang, labels_digest] {
            hash.update((part.len() as u64).to_le_bytes());
            hash.update(part.as_bytes());
        }
        hash.update((offset as u64).to_le_bytes());
        hash.update(body.as_bytes());
        for (path, contents) in &includes.0 {
            hash.update(path.as_bytes());
            hash.update([0]);
            match contents {
                Ok(text) => hash.update(text.as_bytes()),
                Err(message) => hash.update(message.as_bytes()),
            }
            hash.update([0]);
        }
        Self(hash.finalize().into())
    }

    fn html(html: &str) -> Self {
        let mut hash = Sha256::new();
        hash.update(b"rendered-html\0");
        hash.update(WRITER_VERSION.to_le_bytes());
        hash.update(html.as_bytes());
        Self(hash.finalize().into())
    }
}

/// Fragments by key. Entries an analysis no longer uses are dropped, so the
/// cache tracks the site rather than its history. A clone shares the
/// fragments, so a generation can work on one and be thrown away whole.
#[derive(Clone, Default)]
pub struct FragmentCache {
    entries: HashMap<FragmentKey, Arc<Fragment>>,
    pub hits: usize,
    pub misses: usize,
}

impl FragmentCache {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn html(
        &mut self,
        html: &str,
        live: &mut std::collections::HashSet<FragmentKey>,
    ) -> Arc<Fragment> {
        let key = FragmentKey::html(html);
        live.insert(key);
        if let Some(fragment) = self.entries.get(&key) {
            self.hits += 1;
            return fragment.clone();
        }
        self.misses += 1;
        let fragment = Arc::new(markdown::render_html(html));
        self.entries.insert(key, fragment.clone());
        fragment
    }
}

/// Where a URL in a page points, once resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// Leave the URL as written: external, an in-page anchor, a scheme.
    Keep,
    /// A page, with an optional anchor.
    Page { page: usize, anchor: Option<String> },
    /// A form's target: the page it submits to, with the query it submits
    /// and the anchor it names. A form keeps its query: it is part of what
    /// the reader sends.
    Form {
        page: usize,
        query: Option<String>,
        anchor: Option<String>,
    },
    /// A file the site publishes, by project-relative path.
    Resource(String),
    /// A file in the repository, outside the content: its web address.
    Repository(String),
    /// A `srcset`: each candidate's published file, or its URL as written
    /// (a `data:` URL), with its descriptor.
    Srcset(Vec<SrcsetCandidate>),
}

/// One image a `srcset` offers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SrcsetCandidate {
    /// The project-relative file it resolved to, if it is one.
    pub file: Option<String>,
    /// The URL as written.
    pub url: String,
    pub descriptor: String,
}

/// Whether the page came from a content file or a registered Rust renderer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageOrigin {
    Markdown,
    Registered(String),
}

/// A registered page's metadata and the HTML its renderer produced.
#[derive(Clone, Debug)]
pub struct RegisteredPage {
    pub name: String,
    pub route: String,
    pub lang: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub draft: Option<bool>,
    pub index: Option<bool>,
    pub search: Option<bool>,
    pub ai: Option<bool>,
    pub html: String,
    /// Project-relative Rust source, for diagnostics without invented spans.
    pub file: String,
}

/// Rendered additions to the Markdown site. Slot keys are canonical routes;
/// `/404/` names the generated not-found page rather than a registered page.
#[derive(Clone, Debug, Default)]
pub struct Surfaces {
    pub pages: Vec<RegisteredPage>,
    /// Final routes rendered in their target environment; declarations stay real.
    pub rendered_pages: BTreeMap<String, String>,
    pub slots: BTreeMap<String, BTreeMap<Slot, String>>,
    /// Complete source candidates, including classes not present in initial HTML.
    pub candidates: BTreeSet<String>,
}

#[derive(Clone, Debug)]
pub struct AnalysedFragment {
    pub fragment: Arc<Fragment>,
    pub targets: Vec<Target>,
}

/// One page of the site.
#[derive(Clone, Debug)]
pub struct Page {
    pub origin: PageOrigin,
    /// Content-relative for Markdown; the canonical route for registered pages.
    pub path: String,
    /// Project-relative, like `content/guide/setup.md`.
    pub file: String,
    pub route: String,
    pub front: Front,
    pub title: String,
    pub description: Option<String>,
    pub lang: String,
    pub locale: PageLocale,
    pub draft: bool,
    pub fragment: Arc<Fragment>,
    /// For each of the fragment's references, where it points.
    pub targets: Vec<Target>,
    /// For each of the frontmatter's `actions`, where its link points.
    pub action_targets: Vec<Target>,
}

/// The logical address and actual content behind a language-specific page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageLocale {
    pub logical_route: String,
    pub ui_lang: String,
    pub source_lang: String,
    pub source_route: String,
    pub is_fallback: bool,
}

/// Navigation and final page identities within one configured language.
#[derive(Clone, Debug)]
pub struct LocaleView {
    pub root_route: String,
    pub pages: BTreeMap<String, usize>,
    pub nav: Vec<NavGroup>,
    pub top: Vec<Target>,
    pub social: Vec<Target>,
}

/// The analysed site.
pub struct Analysis {
    pub mode: Mode,
    /// Markdown pages in path order, then registered pages in declaration order.
    pub pages: Vec<Page>,
    pub routes: BTreeMap<String, usize>,
    pub locales: BTreeMap<String, LocaleView>,
    pub catalogs: BTreeMap<String, Arc<labels::Catalog>>,
    pub slots: BTreeMap<String, BTreeMap<Slot, AnalysedFragment>>,
    pub nav: Vec<NavGroup>,
    /// Where each of the configuration's `nav.top` and `nav.social` links
    /// points, in their order.
    pub top: Vec<Target>,
    pub social: Vec<Target>,
    /// Where each landing profile's footer links point, by profile name.
    pub landing_footers: BTreeMap<String, Vec<Target>>,
    /// Project-relative files the pages reference, to be published.
    pub resources: BTreeSet<String>,
    /// URL values from source candidates, resolved without changing selectors.
    pub candidate_urls: BTreeMap<String, Target>,
    /// Resource bytes absent from the directory snapshot, read once when referenced.
    pub outside: BTreeMap<String, Arc<[u8]>>,
    /// Resource and include aliases and their real project-relative
    /// dependencies, found or not: what else `dev` watches.
    pub referenced: BTreeSet<String>,
    /// Project-relative files the pages include source from, read or not:
    /// what else `dev` watches.
    pub includes: BTreeSet<String>,
    /// Original reads beyond the directory snapshot, including include-only files.
    pub inputs: InputReceipts,
    pub diagnostics: Vec<Diagnostic>,
}

impl Analysis {
    pub fn errors(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count()
    }

    pub fn warnings(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .count()
    }

    /// The pages a production build publishes.
    pub fn published(&self) -> impl Iterator<Item = &Page> {
        self.pages
            .iter()
            .filter(move |page| self.mode == Mode::Dev || !page.draft)
    }
}

/// Reads an include's file, if it is one the project may expose.
fn read_include(sources: &mut Sources, path: &str) -> Result<String, String> {
    let bytes = sources.read(path).map_err(|refusal| {
        match refusal {
            Refusal::Missing => "the file does not exist",
            Refusal::Outside => "the file is outside the project",
            Refusal::Private => "this file is private and is never included",
            Refusal::ThemeDocument => "the file is the site's theme configuration input",
            Refusal::Messages => "the file is a messages configuration input",
        }
        .to_owned()
    })?;
    std::str::from_utf8(&bytes)
        .map(str::to_owned)
        .map_err(|_| "the file is not readable UTF-8 text".into())
}

/// The files analysis reads besides the snapshot: those outside the content
/// and asset directories that pages and the configuration name, each read
/// once.
struct Sources<'a> {
    project: &'a Project,
    snapshot: &'a Snapshot,
    root: PathBuf,
    theme: BTreeSet<String>,
    messages: BTreeSet<String>,
    outside: BTreeMap<String, Arc<[u8]>>,
    inputs: InputReceipts,
    /// Every project file a resource reference named, found or not.
    named: BTreeSet<String>,
}

impl Sources<'_> {
    fn track(&mut self, path: &str) {
        if self.named.insert(path.to_owned()) {
            if let Some(real) = watched_source(&self.root, &self.project.root.join(path)) {
                self.named.insert(real);
            }
        }
    }

    /// Whether the project file `path` names may be published; the first
    /// time a file outside the snapshot is named, it is read.
    fn get(&mut self, path: &str) -> Result<(), Refusal> {
        self.track(path);
        if is_private(path) {
            return Err(Refusal::Private);
        }
        if self.theme_file(path) {
            return Err(Refusal::ThemeDocument);
        }
        if self.message_file(path) {
            return Err(Refusal::Messages);
        }
        if self.snapshot.files.contains_key(path) || self.outside.contains_key(path) {
            return Ok(());
        }
        let bytes = self.read(path)?;
        self.outside.insert(path.to_owned(), bytes);
        Ok(())
    }

    fn read(&mut self, path: &str) -> Result<Arc<[u8]>, Refusal> {
        if is_private(path) {
            return Err(Refusal::Private);
        }
        if self.message_file(path) {
            return Err(Refusal::Messages);
        }
        // A fragment cache hit still belongs to this round's original read.
        // Reuse bytes from the snapshot or the first read of a shared include.
        if let Some(bytes) = self
            .snapshot
            .inputs
            .bytes(path)
            .or_else(|| self.snapshot.files.get(path).cloned())
            .or_else(|| self.inputs.bytes(path))
        {
            return Ok(bytes);
        }
        read_source(self.project, &self.root, path, &mut self.inputs)
    }

    fn exists(&mut self, path: &str) -> Result<(), Refusal> {
        self.track(path);
        if is_private(path) {
            return Err(Refusal::Private);
        }
        if self.message_file(path) {
            return Err(Refusal::Messages);
        }
        let real = self
            .root
            .join(real_source(&self.root, &self.project.root.join(path))?);
        let metadata = std::fs::metadata(&real).map_err(|_| Refusal::Missing)?;
        self.inputs.record(path, real, None, metadata.is_dir());
        Ok(())
    }

    fn theme_file(&self, path: &str) -> bool {
        matches_theme(self.project, &self.root, &self.theme, path)
    }

    fn message_file(&self, path: &str) -> bool {
        matches_theme(self.project, &self.root, &self.messages, path)
    }

    fn refuse_theme(
        &self,
        target: Target,
        found: &mut Vec<Issue>,
        issue: &dyn Fn(&'static str, String) -> Issue,
    ) -> Target {
        if let Target::Resource(path) = &target {
            if self.theme_file(path) {
                found.push(Refusal::ThemeDocument.issue(issue, &code_span(path), "broken-link"));
                return Target::Keep;
            }
            if self.message_file(path) {
                found.push(Refusal::Messages.issue(issue, &code_span(path), "i18n-messages"));
                return Target::Keep;
            }
        }
        target
    }
}

/// Splits `url` into its path and its `#fragment`.
fn split_fragment(url: &str) -> (&str, Option<&str>) {
    match url.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment)),
        None => (url, None),
    }
}

pub(crate) fn has_scheme(url: &str) -> bool {
    match url.split_once(':') {
        Some((scheme, _)) => {
            !scheme.is_empty()
                && scheme
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic())
                && scheme
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
        }
        None => false,
    }
}

/// Joins a relative URL path onto a directory, both `/`-separated and
/// relative; `None` when it climbs out of the root.
pub(crate) fn join(dir: &str, relative: &str) -> Option<String> {
    let mut parts: Vec<&str> = dir.split('/').filter(|part| !part.is_empty()).collect();
    for part in relative.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    Some(parts.join("/"))
}

pub(crate) fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    let hex = |byte: u8| (byte as char).to_digit(16);
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(high), Some(low)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                out.push((high * 16 + low) as u8);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Analyses the project: every page rendered (from `cache` where it can be),
/// every reference resolved, every problem collected.
pub fn analyze(
    project: &Project,
    snapshot: &Snapshot,
    mode: Mode,
    cache: &mut FragmentCache,
    highlighter: &Highlighter,
    workers: usize,
) -> Analysis {
    analyze_with_surfaces(
        project,
        snapshot,
        mode,
        cache,
        highlighter,
        workers,
        &Surfaces::default(),
    )
}

// Derive missing versions from real declarations before resolving references.
fn project_locales(
    project: &Project,
    mode: Mode,
    pages: &mut Vec<Page>,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, LocaleView> {
    let config = &project.config;
    if config.locales.is_empty() {
        return BTreeMap::new();
    }
    let defaults: BTreeMap<String, usize> = pages
        .iter()
        .enumerate()
        .filter(|(_, page)| page.lang.eq_ignore_ascii_case(&config.site.lang))
        .map(|(i, page)| (page.locale.logical_route.clone(), i))
        .collect();
    let real: BTreeMap<(String, String), usize> = pages
        .iter()
        .enumerate()
        .map(|(i, page)| {
            (
                (
                    page.lang.to_ascii_lowercase(),
                    page.locale.logical_route.clone(),
                ),
                i,
            )
        })
        .collect();
    for page in pages.iter() {
        if !defaults.contains_key(&page.locale.logical_route) {
            diagnostics.push(
                Diagnostic::in_file(
                    &page.file,
                    Severity::Error,
                    "i18n-orphan",
                    format!(
                        "translation {} has no default-language page at {}",
                        code_span(&page.lang),
                        code_span(&page.locale.logical_route)
                    ),
                )
                .with_suggestion("create the default-language page first"),
            );
        }
    }
    let mut locales = BTreeMap::new();
    for locale in &config.locales {
        let mut view = LocaleView {
            root_route: locale_route(config, &locale.lang, "/"),
            pages: BTreeMap::new(),
            nav: Vec::new(),
            top: Vec::new(),
            social: Vec::new(),
        };
        for (logical, &default) in &defaults {
            let found = real
                .get(&(locale.lang.to_ascii_lowercase(), logical.clone()))
                .copied();
            if mode == Mode::Production && pages[default].draft {
                if let Some(found) = found {
                    pages[found].draft = true;
                }
                continue;
            }
            let index = match found {
                Some(i) if mode == Mode::Dev || !pages[i].draft => i,
                other => {
                    let mut fallback = pages[default].clone();
                    fallback.route = locale_route(config, &locale.lang, logical);
                    fallback.lang = locale.lang.clone();
                    fallback.locale.ui_lang = locale.lang.clone();
                    fallback.locale.is_fallback = true;
                    match other {
                        Some(i) => {
                            pages[i] = fallback;
                            i
                        }
                        None => {
                            pages.push(fallback);
                            pages.len() - 1
                        }
                    }
                }
            };
            if !locale.lang.eq_ignore_ascii_case(&config.site.lang)
                && !pages[default].draft
                && (pages[index].locale.is_fallback || pages[index].front.draft)
            {
                let source = &pages[default];
                let expected = match source.origin {
                    PageOrigin::Markdown => code_span(&format!(
                        "{}/{}/{}",
                        config.content_dir, locale.prefix, source.path
                    )),
                    PageOrigin::Registered(_) => format!(
                        "a registered page with route {} and lang {}",
                        code_span(logical),
                        code_span(&locale.lang)
                    ),
                };
                diagnostics.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::warning(
                        locale.lang_span.clone(),
                        "i18n-missing-page",
                        format!(
                            "language {} has no published translation of {}; expected {}",
                            code_span(&locale.lang),
                            code_span(logical),
                            expected
                        ),
                    )
                    .suggest(format!(
                        "create the {} version of {}",
                        code_span(&locale.lang),
                        code_span(logical)
                    )),
                ));
            }
            view.pages.insert(logical.clone(), index);
        }
        locales.insert(locale.lang.clone(), view);
    }
    locales
}

pub(crate) fn resolve_catalogs(
    project: &Project,
    snapshot: &Snapshot,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, Arc<labels::Catalog>> {
    let config = &project.config;
    if config.locales.is_empty() {
        return BTreeMap::from([(
            config.site.lang.clone(),
            Arc::new(labels::resolve(&config.site.lang, &BTreeMap::new())),
        )]);
    }
    let mut catalogs = BTreeMap::new();
    for locale in &config.locales {
        let (overrides, valid) = match &locale.messages {
            None => (BTreeMap::new(), true),
            Some((path, _)) => match snapshot.messages.get(path) {
                None => (BTreeMap::new(), false),
                Some(text) => match labels::parse_messages(text) {
                    Ok(messages) => (messages, true),
                    Err(issues) => {
                        diagnostics.extend(
                            issues
                                .into_iter()
                                .map(|issue| Diagnostic::located(path, text, issue)),
                        );
                        (BTreeMap::new(), false)
                    }
                },
            },
        };
        let catalog = labels::resolve(&locale.lang, &overrides);
        if valid {
            let span = locale
                .messages
                .as_ref()
                .map_or(&locale.lang_span, |(_, span)| span);
            for key in catalog.missing_keys() {
                diagnostics.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::warning(
                        span.clone(),
                        "i18n-missing-label",
                        format!(
                            "language {} has no message {}; English is used",
                            code_span(&locale.lang),
                            code_span(key)
                        ),
                    )
                    .suggest(format!(
                        "add {} to this language's messages file",
                        code_span(key)
                    )),
                ));
            }
        }
        catalogs.insert(locale.lang.clone(), Arc::new(catalog));
    }
    catalogs
}

fn catalog_digest(
    project: &Project,
    snapshot: &Snapshot,
    lang: &str,
    catalog: &labels::Catalog,
) -> String {
    let mut hash = Sha256::new();
    hash.update(catalog.digest().as_bytes());
    if let Some(text) = project
        .config
        .locales
        .iter()
        .find(|locale| locale.lang == lang)
        .and_then(|locale| locale.messages.as_ref())
        .and_then(|(path, _)| snapshot.messages.get(path))
    {
        hash.update(text.as_bytes());
    }
    format!("{:x}", hash.finalize())
}

/// Analyses Markdown and registered HTML in the same content graph.
pub fn analyze_with_surfaces(
    project: &Project,
    snapshot: &Snapshot,
    mode: Mode,
    cache: &mut FragmentCache,
    highlighter: &Highlighter,
    workers: usize,
    surfaces: &Surfaces,
) -> Analysis {
    let config = &project.config;
    let content_dir = &config.content_dir;
    let mut diagnostics = Vec::new();
    let mut catalogs = resolve_catalogs(project, snapshot, &mut diagnostics);
    let mut catalog_digests: BTreeMap<String, String> = catalogs
        .iter()
        .map(|(lang, catalog)| {
            (
                lang.clone(),
                catalog_digest(project, snapshot, lang, catalog),
            )
        })
        .collect();
    let file_of = |path: &str| format!("{content_dir}/{path}");
    let root = real_root(project);
    let mut sources = Sources {
        project,
        snapshot,
        theme: theme_paths(project, &root),
        messages: message_paths(project, &root),
        root,
        outside: BTreeMap::new(),
        inputs: InputReceipts::default(),
        named: BTreeSet::new(),
    };
    if let Some((path, _)) = &config.theme_document {
        sources.track(path);
    }
    for (path, _) in config
        .locales
        .iter()
        .filter_map(|locale| locale.messages.as_ref())
    {
        sources.track(path);
    }

    // Pages: frontmatter, route, includes.
    struct Draft {
        path: String,
        route: String,
        locale: PageLocale,
        front: Front,
        body_offset: usize,
        includes: IncludeFiles,
        key: FragmentKey,
        lang: String,
    }
    let mut drafts = Vec::new();
    let mut routes: BTreeMap<String, usize> = BTreeMap::new();
    let mut included = BTreeSet::new();
    for (path, text) in &snapshot.markdown {
        let file = file_of(path);
        let (front, body_offset) = match frontmatter::parse(text) {
            Ok(parsed) => parsed,
            Err(issue) => {
                let body = frontmatter::split(text)
                    .ok()
                    .flatten()
                    .map_or(0, |split| split.body);
                diagnostics.push(Diagnostic::located(&file, text, issue));
                (Front::default(), body)
            }
        };
        let (declared_lang, logical_path) = markdown_locale(config, path);
        let logical_route = route_for(logical_path);
        let route = locale_route(config, declared_lang, &logical_route);
        if reserved_logical_route(config, &logical_route) {
            diagnostics.push(
                Diagnostic::in_file(
                    &file,
                    Severity::Error,
                    "reserved-route",
                    format!(
                        "{} is reserved for generated or language-specific output",
                        code_span(&logical_route)
                    ),
                )
                .with_suggestion("rename the file"),
            );
            continue;
        }
        if let Some(&other) = routes.get(&route) {
            let other: &Draft = &drafts[other];
            diagnostics.push(
                Diagnostic::in_file(
                    &file,
                    Severity::Error,
                    "route-conflict",
                    format!(
                        "this page and {} both become {}",
                        code_span(&file_of(&other.path)),
                        literal(&route)
                    ),
                )
                .with_suggestion("rename or move one of them"),
            );
            continue;
        }
        let mut lang = match &front.lang {
            Some(lang) if config::valid_lang(lang) => lang.clone(),
            Some(lang) => {
                diagnostics.push(frontmatter::language_issue(text).map_or_else(
                    || {
                        Diagnostic::in_file(
                            &file,
                            Severity::Error,
                            "lang",
                            format!("{} is not a language tag", code_span(lang)),
                        )
                        .with_suggestion("use a language tag such as `en`, `zh-CN` or `fr`")
                    },
                    |issue| Diagnostic::located(&file, text, issue),
                ));
                config.site.lang.clone()
            }
            None => declared_lang.to_owned(),
        };
        if !config.locales.is_empty() {
            if front
                .lang
                .as_ref()
                .is_some_and(|lang| !lang.eq_ignore_ascii_case(declared_lang))
            {
                let span = frontmatter::split(text)
                    .ok()
                    .flatten()
                    .map_or(0..0, |split| split.yaml);
                let at = front
                    .lang
                    .as_ref()
                    .and_then(|lang| text[span.clone()].find(lang))
                    .map_or(span.start, |at| span.start + at);
                diagnostics.push(Diagnostic::located(
                    &file,
                    text,
                    Issue::error(
                        at..at,
                        "i18n-content-lang",
                        format!(
                            "frontmatter.lang must match this directory's language {}",
                            code_span(declared_lang)
                        ),
                    ),
                ));
            }
            lang = declared_lang.to_owned();
        }
        let locale = PageLocale {
            logical_route,
            ui_lang: lang.clone(),
            source_lang: lang.clone(),
            source_route: route.clone(),
            is_fallback: false,
        };
        let body = &text[body_offset..];
        let mut includes = IncludeFiles::default();
        for include in markdown::include_paths(body) {
            sources.track(&include);
            let read = read_include(&mut sources, &include);
            included.insert(include.clone());
            includes.0.insert(include, read);
        }
        let catalog = catalogs
            .entry(lang.clone())
            .or_insert_with(|| Arc::new(labels::resolve(&lang, &BTreeMap::new())));
        let digest = catalog_digests
            .entry(lang.clone())
            .or_insert_with(|| catalog.digest().to_owned());
        let key = FragmentKey::of(body, body_offset, &lang, &lang, digest, &includes);
        routes.insert(route.clone(), drafts.len());
        drafts.push(Draft {
            path: path.clone(),
            route,
            locale,
            front,
            body_offset,
            includes,
            key,
            lang,
        });
    }

    // Fragments, from the cache or rendered in parallel.
    let missing: Vec<usize> = {
        let mut seen = std::collections::HashSet::new();
        (0..drafts.len())
            .filter(|&i| !cache.entries.contains_key(&drafts[i].key) && seen.insert(drafts[i].key))
            .collect()
    };
    let rendered = crate::generation::parallel_map(&missing, workers, |&i| {
        let draft = &drafts[i];
        let text = &snapshot.markdown[&draft.path];
        let context = markdown::Context {
            highlighter,
            catalog: &catalogs[&draft.lang],
            source_lang: &draft.lang,
            includes: &draft.includes,
        };
        Arc::new(markdown::render(
            &text[draft.body_offset..],
            draft.body_offset,
            &context,
        ))
    });
    cache.misses += missing.len();
    cache.hits += drafts.len() - missing.len();
    for (i, fragment) in missing.into_iter().zip(rendered) {
        cache.entries.insert(drafts[i].key, fragment);
    }
    let mut live: std::collections::HashSet<FragmentKey> =
        drafts.iter().map(|draft| draft.key).collect();

    let mut pages: Vec<Page> = Vec::with_capacity(drafts.len());
    for draft in drafts {
        let mut fragment = cache.entries[&draft.key].clone();
        let file = file_of(&draft.path);
        let text = &snapshot.markdown[&draft.path];
        for issue in &fragment.issues {
            diagnostics.push(Diagnostic::located(&file, text, issue.clone()));
        }
        let title = match (&draft.front.title, &fragment.title) {
            (Some(title), _) => title.clone(),
            (None, Some(title)) => title.clone(),
            (None, None) => {
                diagnostics.push(
                    Diagnostic::in_file(
                        &file,
                        Severity::Error,
                        "missing-title",
                        "this page has no title",
                    )
                    .with_suggestion(
                        "add `title:` to its frontmatter or start it with a `# heading`",
                    ),
                );
                draft.path.clone()
            }
        };
        let home = draft.front.layout.has_hero();
        let merged = home && fragment.leading_h1.is_some();
        if merged {
            if let Some(end) = crate::html_check::leading_h1_end(&fragment.html) {
                let mut projected = String::from("<h1");
                if let Some(id) = fragment.leading_h1.as_deref().filter(|id| !id.is_empty()) {
                    projected.push_str(" id=\"");
                    crate::escape::escape_html_into(&mut projected, id);
                    projected.push('"');
                }
                projected.push_str("></h1>");
                projected.push_str(&fragment.html[end..]);
                // Layout changes which author nodes ship. Keep the cached
                // source fragment reusable for ordinary content pages.
                let anchors = crate::html_check::ids(&projected);
                Arc::make_mut(&mut fragment).anchors = anchors;
            }
        }
        let generated_h1 = home || fragment.h1_count == 0;
        if fragment.h1_count + usize::from(generated_h1) - usize::from(merged) > 1 {
            let second = fragment
                .headings
                .iter()
                .filter(|h| h.level == 1)
                .nth(usize::from(!home || merged))
                .map_or(0, |h| h.span.start);
            diagnostics.push(Diagnostic::located(
                &file,
                text,
                Issue::error(
                    second..second,
                    "multiple-h1",
                    "a page has one top-level heading; this is the second",
                )
                .suggest(if home && !merged {
                    "make it an H2 (`##`) heading, or start the body with `# Title` to merge it into the home title"
                } else {
                    "make it a `##` heading"
                }),
            ));
        }
        let mut previous = if fragment.h1_count > 0 || draft.front.title.is_some() {
            1
        } else {
            0
        };
        if home && !draft.front.features.is_empty() {
            previous = 2;
        }
        for heading in fragment.headings.iter().skip(usize::from(merged)) {
            if previous > 0 && heading.level > previous + 1 {
                diagnostics.push(Diagnostic::located(
                    &file,
                    text,
                    Issue::warning(
                        heading.span.clone(),
                        "heading-skip",
                        format!(
                            "this level-{} heading follows a level-{previous} one",
                            heading.level
                        ),
                    )
                    .suggest(format!("use a level-{} heading", previous + 1)),
                ));
            }
            previous = heading.level;
        }
        let description = draft
            .front
            .description
            .clone()
            .or_else(|| fragment.summary.clone());
        if !draft.front.layout.has_hero() {
            match (&draft.front.description, &fragment.summary) {
                (Some(_), _) => {}
                (None, Some(_)) => diagnostics.push(
                    Diagnostic::in_file(
                        &file,
                        Severity::Warning,
                        "missing-description",
                        "no `description:`; the first paragraph is used",
                    )
                    .with_suggestion("add `description:` to the frontmatter"),
                ),
                (None, None) => diagnostics.push(
                    Diagnostic::in_file(
                        &file,
                        Severity::Error,
                        "missing-description",
                        "no `description:` and no paragraph to take one from",
                    )
                    .with_suggestion("add `description:` to the frontmatter"),
                ),
            }
        }
        // Only the configuration names what a landing page may load; a page
        // just picks one of its profiles.
        if let Some(name) = &draft.front.landing {
            let yaml = frontmatter::split(text)
                .ok()
                .flatten()
                .map_or(0..0, |split| split.yaml);
            let at = text[yaml.clone()]
                .find("landing:")
                .and_then(|key| {
                    let after = yaml.start + key + "landing:".len();
                    text[after..yaml.end]
                        .find(name.as_str())
                        .map(|at| after + at)
                })
                .unwrap_or(yaml.start);
            let span = at..at + name.len();
            if draft.front.layout != Layout::Landing {
                diagnostics.push(Diagnostic::located(
                    &file,
                    text,
                    Issue::error(
                        span,
                        "landing-without-layout",
                        "`landing:` selects a profile only for a `layout: landing` page",
                    )
                    .suggest("add `layout: landing`, or remove `landing:`"),
                ));
            } else if !config.landing.contains_key(name) {
                let names: Vec<String> = config.landing.keys().map(|name| literal(name)).collect();
                diagnostics.push(Diagnostic::located(
                    &file,
                    text,
                    Issue::error(
                        span,
                        "landing-profile",
                        format!(
                            "landing profile {} is not declared in {}",
                            code_span(name),
                            literal(&project.config_file)
                        ),
                    )
                    .suggest(if names.is_empty() {
                        format!("declare it as `[landing.profiles.{name}]`")
                    } else {
                        format!("use one of: {}", names.join(", "))
                    }),
                ));
            }
        }
        pages.push(Page {
            origin: PageOrigin::Markdown,
            route: draft.route,
            file,
            path: draft.path,
            title,
            description,
            lang: draft.lang,
            locale: draft.locale,
            draft: draft.front.draft,
            front: draft.front,
            fragment,
            targets: Vec::new(),
            action_targets: Vec::new(),
        });
    }

    let mut routes: BTreeMap<String, usize> = pages
        .iter()
        .enumerate()
        .map(|(i, page)| (page.route.clone(), i))
        .collect();
    for registered in &surfaces.pages {
        let logical_route = &registered.route;
        if config::normalize_base(logical_route).as_ref() != Ok(logical_route) {
            diagnostics.push(Diagnostic::in_file(
                &registered.file,
                Severity::Error,
                "invalid-route",
                format!("registered route {} needs a canonical path like `/custom/`, without query or fragment", code_span(logical_route)),
            ));
            continue;
        }
        let lang = registered.lang.as_deref().unwrap_or(&config.site.lang);
        let lang = if let Some(locale) = config
            .locales
            .iter()
            .find(|locale| locale.lang.eq_ignore_ascii_case(lang))
        {
            &locale.lang
        } else if lang.eq_ignore_ascii_case(&config.site.lang) {
            &config.site.lang
        } else {
            diagnostics.push(Diagnostic::in_file(
                &registered.file,
                Severity::Error,
                "i18n-config",
                format!(
                    "registered page {} declares unconfigured language {}",
                    code_span(&registered.name),
                    code_span(lang)
                ),
            ));
            continue;
        };
        let route = locale_route(config, lang, logical_route);
        let output = crate::generation::page_path(logical_route);
        let first = logical_route
            .trim_matches('/')
            .split('/')
            .next()
            .unwrap_or_default();
        if reserved_logical_route(config, logical_route)
            || is_private(logical_route)
            || crate::generation::SITE_FILES.contains(&first)
            || crate::generation::GENERATED_PREFIXES
                .iter()
                .any(|prefix| output.starts_with(prefix))
        {
            diagnostics.push(Diagnostic::in_file(
                &registered.file,
                Severity::Error,
                "reserved-route",
                format!(
                    "registered route {} is reserved for generated or private output",
                    code_span(logical_route)
                ),
            ));
            continue;
        }
        if let Some(&other) = routes.get(&route) {
            diagnostics.push(Diagnostic::in_file(
                &registered.file,
                Severity::Error,
                "route-conflict",
                format!(
                    "registered page {} and {} both become {}",
                    code_span(&registered.name),
                    code_span(&pages[other].file),
                    literal(&route)
                ),
            ));
            continue;
        }
        let fragment = cache.html(&registered.html, &mut live);
        routes.insert(route.clone(), pages.len());
        pages.push(Page {
            origin: PageOrigin::Registered(registered.name.clone()),
            path: logical_route.clone(),
            file: registered.file.clone(),
            route: route.clone(),
            front: Front {
                title: Some(registered.title.clone()),
                description: registered.description.clone(),
                lang: Some(lang.clone()),
                draft: registered.draft.unwrap_or(false),
                index: registered.index,
                search: registered.search,
                ai: registered.ai,
                ..Front::default()
            },
            title: registered.title.clone(),
            description: registered.description.clone(),
            lang: lang.clone(),
            locale: PageLocale {
                logical_route: logical_route.clone(),
                ui_lang: lang.clone(),
                source_lang: lang.clone(),
                source_route: route.clone(),
                is_fallback: false,
            },
            draft: registered.draft.unwrap_or(false),
            fragment,
            targets: Vec::new(),
            action_targets: Vec::new(),
        });
    }
    let mut locales = project_locales(project, mode, &mut pages, &mut diagnostics);
    let routes: BTreeMap<String, usize> = pages
        .iter()
        .enumerate()
        .map(|(i, page)| (page.route.clone(), i))
        .collect();
    for page in &mut pages {
        if page.locale.is_fallback && page.origin == PageOrigin::Markdown {
            let text = &snapshot.markdown[&page.path];
            let offset = frontmatter::split(text)
                .ok()
                .flatten()
                .map_or(0, |split| split.body);
            let body = &text[offset..];
            let mut includes = IncludeFiles::default();
            for path in markdown::include_paths(body) {
                includes
                    .0
                    .insert(path.clone(), read_include(&mut sources, &path));
            }
            let key = FragmentKey::of(
                body,
                offset,
                &page.locale.source_lang,
                &page.lang,
                &catalog_digests[&page.lang],
                &includes,
            );
            live.insert(key);
            let fragment = if let Some(fragment) = cache.entries.get(&key) {
                cache.hits += 1;
                fragment.clone()
            } else {
                cache.misses += 1;
                let fragment = Arc::new(markdown::render(
                    body,
                    offset,
                    &markdown::Context {
                        highlighter,
                        catalog: &catalogs[&page.lang],
                        source_lang: &page.locale.source_lang,
                        includes: &includes,
                    },
                ));
                cache.entries.insert(key, fragment.clone());
                fragment
            };
            let anchors = page.fragment.anchors.clone();
            page.fragment = fragment;
            if page.fragment.anchors != anchors {
                Arc::make_mut(&mut page.fragment).anchors = anchors;
            }
        }
        if !matches!(page.origin, PageOrigin::Registered(_)) {
            continue;
        }
        if let Some(html) = surfaces.rendered_pages.get(&page.route) {
            page.fragment = cache.html(html, &mut live);
        }
        diagnostics.extend(
            page.fragment
                .issues
                .iter()
                .cloned()
                .map(|issue| Diagnostic::rendered(&page.file, issue)),
        );
        if page.fragment.h1_count > 1 {
            diagnostics.push(Diagnostic::in_file(
                &page.file,
                Severity::Error,
                "multiple-h1",
                "a page has one top-level heading; the registered HTML contains more than one",
            ));
        }
        let mut previous = 1;
        for heading in &page.fragment.headings {
            if heading.level > previous + 1 {
                diagnostics.push(Diagnostic::in_file(
                    &page.file,
                    Severity::Warning,
                    "heading-skip",
                    format!(
                        "this level-{} heading follows a level-{previous} one",
                        heading.level
                    ),
                ));
            }
            previous = heading.level;
        }
    }
    if !pages.iter().any(|page| mode == Mode::Dev || !page.draft) {
        let (message, suggestion) = if !project.content_root().is_dir() {
            (
                format!(
                    "the content directory {} does not exist, so the site has no page",
                    code_span(content_dir)
                ),
                format!(
                    "create it with an `index.md`, or set content.dir in {}",
                    literal(&project.config_file)
                ),
            )
        } else if pages.is_empty() {
            (
                format!(
                    "{} holds no page, so the site would have only its not-found page",
                    code_span(content_dir)
                ),
                format!(
                    "add a Markdown page, such as {}",
                    code_span(&format!("{content_dir}/index.md"))
                ),
            )
        } else {
            (
                format!(
                    "every page in {} is a draft, so a production build would publish none",
                    code_span(content_dir)
                ),
                "set `draft: false` on the pages that are ready".to_owned(),
            )
        };
        diagnostics.push(
            Diagnostic::in_file(content_dir, Severity::Error, "no-pages", message)
                .with_suggestion(suggestion),
        );
    }
    let by_path: HashMap<String, usize> = pages
        .iter()
        .enumerate()
        .filter(|(_, page)| page.origin == PageOrigin::Markdown && !page.locale.is_fallback)
        .map(|(i, page)| (page.path.clone(), i))
        .collect();

    let slot_file = config
        .rust
        .as_ref()
        .map_or(project.config_file.as_str(), |rust| rust.registry.as_str());
    let mut slots = BTreeMap::new();
    for (route, rendered) in &surfaces.slots {
        let not_found = route == "/404/"
            || config
                .locales
                .iter()
                .any(|locale| locale_route(config, &locale.lang, "/404/") == *route);
        if !not_found && !routes.contains_key(route) {
            diagnostics.push(Diagnostic::in_file(
                slot_file,
                Severity::Error,
                "slot-route",
                format!("slots name {}, which is not a page route", code_span(route)),
            ));
            continue;
        }
        let mut analysed = BTreeMap::new();
        for (&slot, html) in rendered {
            let fragment = cache.html(html, &mut live);
            diagnostics.extend(
                fragment
                    .issues
                    .iter()
                    .cloned()
                    .map(|issue| Diagnostic::rendered(slot_file, issue)),
            );
            analysed.insert(
                slot,
                AnalysedFragment {
                    fragment,
                    targets: Vec::new(),
                },
            );
        }
        slots.insert(route.clone(), analysed);
    }
    cache.entries.retain(|key, _| live.contains(key));
    let mut anchors: BTreeMap<String, BTreeSet<String>> = pages
        .iter()
        .map(|page| {
            (
                page.route.clone(),
                page.fragment.anchors.iter().cloned().collect(),
            )
        })
        .collect();
    for (route, rendered) in &slots {
        let known = anchors.entry(route.clone()).or_default();
        for slot in rendered.values() {
            known.extend(slot.fragment.anchors.iter().cloned());
        }
    }

    // A group's tabs agree within a UI language; translated controls can differ.
    let mut groups: BTreeMap<(&str, &str), (BTreeSet<&str>, &str)> = BTreeMap::new();
    for page in &pages {
        for group in &page.fragment.code_groups {
            let labels: BTreeSet<&str> = group.labels.iter().map(String::as_str).collect();
            let key = if config.locales.is_empty() {
                ("", group.name.as_str())
            } else {
                (page.lang.as_str(), group.name.as_str())
            };
            match groups.get(&key) {
                None => {
                    groups.insert(key, (labels, &page.file));
                }
                Some((known, first)) if *known != labels => {
                    let text = &snapshot.markdown[&page.path];
                    diagnostics.push(Diagnostic::located(
                        &page.file,
                        text,
                        Issue::error(
                            group.span.clone(),
                            "code-group-conflict",
                            format!("code group {} has other tabs in {}; groups of one name switch together", code_span(&group.name), literal(first)),
                        )
                        .suggest("give the groups the same tabs, or different names"),
                    ));
                }
                Some(_) => {}
            }
        }
    }

    // References.
    let mut resources = BTreeSet::new();
    let page_count = pages.len();
    for i in 0..page_count {
        let page = &pages[i];
        let graph = LinkGraph {
            project,
            snapshot,
            pages: &pages,
            by_path: &by_path,
            routes: &routes,
            anchors: &anchors,
            mode,
        };
        let rendered = page.origin != PageOrigin::Markdown;
        let dir = if rendered {
            ""
        } else {
            page.path.rsplit_once('/').map_or("", |(dir, _)| dir)
        };
        let landing = (page.front.layout == Layout::Landing).then(|| {
            page.front
                .landing
                .as_deref()
                .and_then(|name| config.landing.get(name))
        });
        let (targets, mut found) = resolve_references(
            &graph,
            &mut sources,
            &page.route,
            dir,
            rendered,
            &page.fragment,
            landing,
        );
        collect_resources(&targets, &mut resources);
        // A home page's actions are links too, written in its frontmatter.
        let mut action_targets = Vec::with_capacity(page.front.actions.len());
        match &page.origin {
            PageOrigin::Markdown => {
                let text = &snapshot.markdown[&page.path];
                let yaml = frontmatter::split(text)
                    .ok()
                    .flatten()
                    .map_or(0..0, |split| split.yaml);
                let mut cursor = yaml.start;
                for action in &page.front.actions {
                    let link = action.link.trim();
                    let at = text[cursor..yaml.end]
                        .find(link)
                        .map_or(yaml.start, |at| cursor + at);
                    cursor = cursor.max(at + link.len()).min(yaml.end);
                    let issue = |rule: &'static str, message: String| {
                        Issue::error(at..at + link.len(), rule, message)
                    };
                    let target = resolve_link(
                        &graph,
                        &mut sources,
                        Some(&page.route),
                        dir,
                        link,
                        &mut found,
                        &issue,
                    );
                    action_targets.push(sources.refuse_theme(target, &mut found, &issue));
                }
                diagnostics.extend(
                    found
                        .into_iter()
                        .map(|issue| Diagnostic::located(&page.file, text, issue)),
                );
            }
            PageOrigin::Registered(_) => diagnostics.extend(
                found
                    .into_iter()
                    .map(|issue| Diagnostic::rendered(&page.file, issue)),
            ),
        }
        // A file only an action names is published all the same.
        for target in &action_targets {
            if let Target::Resource(path) = target {
                resources.insert(path.clone());
            }
        }
        pages[i].targets = targets;
        pages[i].action_targets = action_targets;
    }

    let graph = LinkGraph {
        project,
        snapshot,
        pages: &pages,
        by_path: &by_path,
        routes: &routes,
        anchors: &anchors,
        mode,
    };
    for (route, rendered) in &mut slots {
        for slot in rendered.values_mut() {
            let (targets, found) =
                resolve_references(&graph, &mut sources, route, "", true, &slot.fragment, None);
            collect_resources(&targets, &mut resources);
            diagnostics.extend(
                found
                    .into_iter()
                    .map(|issue| Diagnostic::rendered(slot_file, issue)),
            );
            slot.targets = targets;
        }
    }

    // The configuration's header links resolve from the content root.
    let mut config_links = |links: &[config::NavLink], from: Option<&str>| -> Vec<Target> {
        let mut found = Vec::new();
        let targets = links
            .iter()
            .map(|link| {
                let issue = |rule: &'static str, message: String| {
                    Issue::error(link.span.clone(), rule, message)
                };
                let target = resolve_link(
                    &graph,
                    &mut sources,
                    from,
                    "",
                    link.link.trim(),
                    &mut found,
                    &issue,
                );
                sources.refuse_theme(target, &mut found, &issue)
            })
            .collect();
        for issue in found {
            diagnostics.push(Diagnostic::located(
                &project.config_file,
                &project.config_text,
                issue,
            ));
        }
        targets
    };
    let (mut top, mut social) = if config.locales.is_empty() {
        (
            config_links(&config.nav.top, None),
            config_links(&config.nav.social, None),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    for locale in &config.locales {
        let Some(view) = locales.get_mut(&locale.lang) else {
            continue;
        };
        let nav = locale.nav.as_ref().unwrap_or(&config.nav);
        view.top = config_links(&nav.top, Some(&view.root_route));
        view.social = config_links(&nav.social, Some(&view.root_route));
        collect_resources(&view.top, &mut resources);
        collect_resources(&view.social, &mut resources);
    }
    let mut landing_footers = BTreeMap::new();
    for (name, profile) in &config.landing {
        let targets = config_links(&profile.footer, None);
        collect_resources(&targets, &mut resources);
        landing_footers.insert(name.clone(), targets);
    }
    for target in top.iter().chain(&social) {
        if let Target::Resource(path) = target {
            resources.insert(path.clone());
        }
    }

    // Source candidates also style states which initial HTML does not contain.
    // Resolve their URL values once; their original selectors stay intact.
    let mut candidate_urls = BTreeMap::new();
    for candidate in &surfaces.candidates {
        for (url, span) in crate::html_check::class_urls(candidate) {
            if candidate_urls.contains_key(&url) {
                continue;
            }
            let mut found = Vec::new();
            let issue = |rule: &'static str, message: String| {
                Issue::error(
                    span.clone(),
                    rule,
                    format!("source candidate {}: {message}", code_span(candidate)),
                )
            };
            let target = if url.starts_with('#') {
                // A source literal has no owning document until it is emitted.
                Target::Keep
            } else {
                resolve_resource(
                    &mut sources,
                    "",
                    true,
                    &url,
                    true,
                    false,
                    &mut found,
                    &issue,
                )
            };
            collect_resources(std::slice::from_ref(&target), &mut resources);
            diagnostics.extend(found.into_iter().map(|issue| Diagnostic {
                suggestion: issue.suggestion,
                ..Diagnostic::general(issue.severity, issue.rule, issue.message)
            }));
            candidate_urls.insert(url, target);
        }
    }

    // Navigation.
    validate_sidebar_groups(project, snapshot, &pages, &mut diagnostics);
    let mut nav = if config.locales.is_empty() {
        navigation(project, &pages, mode, &by_path, &routes, &mut diagnostics)
    } else {
        Vec::new()
    };
    if !config.locales.is_empty() {
        // Select and rank logical pages once per navigation definition, using
        // each target version's title while retaining the default path identity.
        let defaults: Vec<_> = pages
            .iter()
            .filter(|page| page.lang.eq_ignore_ascii_case(&config.site.lang))
            .collect();
        for locale in &config.locales {
            let Some(view) = locales.get_mut(&locale.lang) else {
                continue;
            };
            let mut localized = project.clone();
            localized.config.nav = locale.nav.clone().unwrap_or_else(|| config.nav.clone());
            let projected: Vec<_> = defaults
                .iter()
                .map(|source| {
                    let target = view
                        .pages
                        .get(&source.locale.logical_route)
                        .map(|&i| &pages[i]);
                    let mut page = target.unwrap_or(source).clone();
                    if locale.nav.is_none() {
                        page.front.sidebar.group = source.front.sidebar.group.clone();
                    } else if page.locale.is_fallback {
                        page.front.sidebar.group = None;
                    }
                    page.path = source.path.clone();
                    page.origin = source.origin.clone();
                    page.route = source.locale.logical_route.clone();
                    page
                })
                .collect();
            let paths = projected
                .iter()
                .enumerate()
                .filter(|(_, page)| page.origin == PageOrigin::Markdown)
                .map(|(i, page)| (page.path.clone(), i))
                .collect();
            let logical_routes = projected
                .iter()
                .enumerate()
                .map(|(i, page)| (page.route.clone(), i))
                .collect();
            view.nav = navigation(
                &localized,
                &projected,
                mode,
                &paths,
                &logical_routes,
                &mut diagnostics,
            );
            for group in &mut view.nav {
                group.lang = if locale.nav.is_some() {
                    locale.lang.clone()
                } else {
                    config.site.lang.clone()
                };
                group.prefix = locale_route(config, &locale.lang, &group.prefix);
                for item in &mut group.items {
                    item.route = locale_route(config, &locale.lang, &item.route);
                }
            }
        }
        let default = config
            .locales
            .iter()
            .find(|locale| locale.lang.eq_ignore_ascii_case(&config.site.lang));
        if let Some(locale) = default {
            nav = locales[&locale.lang].nav.clone();
            top = locales[&locale.lang].top.clone();
            social = locales[&locale.lang].social.clone();
        }
    }

    // The logo, the link-preview image and the brand colours.
    let logos = config.logo.iter().flat_map(|logo| {
        [&logo.light, &logo.dark]
            .into_iter()
            .map(|path| (path, &logo.span, "theme.logo", "logo-missing"))
    });
    let image = config
        .site
        .image
        .iter()
        .map(|(path, span)| (path, span, "site.image", "image-missing"));
    for (path, span, key, missing) in logos.chain(image) {
        match sources.get(path) {
            Ok(()) => {
                resources.insert(path.clone());
            }
            Err(refusal) => diagnostics.push(Diagnostic::located(
                &project.config_file,
                &project.config_text,
                refusal.issue(
                    &|rule, message| Issue::error(span.clone(), rule, message),
                    &format!("{key} {}", code_span(path)),
                    missing,
                ),
            )),
        }
    }
    // A landing profile's own scripts are published like the logo.
    for profile in config.landing.values() {
        for script in &profile.scripts {
            let crate::landing::ScriptSrc::Local(path) = &script.src else {
                continue;
            };
            match sources.get(path) {
                Ok(()) => {
                    resources.insert(path.clone());
                }
                Err(refusal) => diagnostics.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    refusal.issue(
                        &|rule, message| Issue::error(script.span.clone(), rule, message),
                        &format!("landing script {}", code_span(path)),
                        "landing-script-missing",
                    ),
                )),
            }
        }
    }
    let selected: BTreeSet<&str> = pages
        .iter()
        .filter_map(|page| page.front.landing.as_deref())
        .collect();
    for (name, profile) in &config.landing {
        if !selected.contains(name.as_str()) {
            diagnostics.push(Diagnostic::located(
                &project.config_file,
                &project.config_text,
                Issue::warning(
                    profile.span.clone(),
                    "landing-profile-unused",
                    format!("no page selects landing profile {}", code_span(name)),
                )
                .suggest(format!(
                    "add `landing: {name}` to a `layout: landing` page, or remove the profile"
                )),
            ));
        }
    }
    if config.site.url.is_none() {
        diagnostics.push(
            Diagnostic::in_file(
                &project.config_file,
                Severity::Warning,
                "missing-site-url",
                "site.url is not set, so pages have no canonical address or preview image and there is no sitemap",
            )
            .with_suggestion("add `url = \"https://…\"` under [site], the address the site is published at"),
        );
    }

    // A file in the asset directory is published at its own path, which
    // must not be one the build writes.
    let assets_prefix = format!("{}/", config.assets_dir);
    let page_files: BTreeSet<String> = pages
        .iter()
        .map(|page| crate::generation::page_path(&page.route))
        .collect();
    for path in snapshot.files.keys() {
        let Some(output) = path.strip_prefix(&assets_prefix) else {
            continue;
        };
        if is_private(path) || sources.theme_file(path) || sources.message_file(path) {
            continue;
        }
        let taken = reserved_output(config, output)
            || page_files.contains(output)
            || crate::generation::SITE_FILES.contains(&output)
            || config.locales.iter().any(|locale| {
                !locale.prefix.is_empty() && output == format!("{}/404.html", locale.prefix)
            })
            || crate::generation::GENERATED_PREFIXES
                .iter()
                .any(|prefix| output.starts_with(prefix));
        if taken {
            diagnostics.push(
                Diagnostic::in_file(
                    path,
                    Severity::Error,
                    "output-conflict",
                    format!(
                        "this file would be published at {}, which the build writes itself",
                        code_span(output)
                    ),
                )
                .with_suggestion("rename or move it"),
            );
        }
    }

    if let Some(primary) = &config.primary {
        for (color, background, span, mode_name) in [
            (&primary.light, "#f9fafb", &primary.spans.0, "light"),
            (&primary.dark, "#0c111d", &primary.spans.1, "dark"),
        ] {
            let link = crate::color::contrast(color, background).unwrap_or(0.0);
            let foreground = crate::style::foreground_on(color);
            let button = crate::color::contrast(foreground, color).unwrap_or(0.0);
            if link < 4.5 {
                diagnostics.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::warning(span.clone(), "brand-contrast", format!("{} as link text on the {mode_name} background is {link:.2}:1; text needs 4.5:1", literal(color)))
                        .suggest(format!("pick a {} {mode_name} primary", if mode_name == "light" { "darker" } else { "lighter" })),
                ));
            }
            if button < 4.5 {
                diagnostics.push(Diagnostic::located(
                    &project.config_file,
                    &project.config_text,
                    Issue::warning(
                        span.clone(),
                        "brand-contrast",
                        format!(
                            "no button label reaches 4.5:1 on {} ({button:.2}:1 at best)",
                            literal(color)
                        ),
                    ),
                ));
            }
        }
    }

    diagnostics.sort();
    diagnostics.dedup();
    Analysis {
        mode,
        pages,
        routes,
        locales,
        catalogs,
        slots,
        nav,
        top,
        social,
        landing_footers,
        resources,
        candidate_urls,
        outside: sources.outside,
        referenced: sources.named,
        includes: included,
        inputs: sources.inputs,
        diagnostics,
    }
}

#[doc(hidden)]
pub struct MarkdownIntroContent {
    pub html: String,
    pub inputs: InputReceipts,
    pub referenced: BTreeSet<String>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Renders the original H1-to-first-H2 introduction through the shared writer.
/// The fragment's links are resolved at its source, before the enclosing page.
#[doc(hidden)]
pub fn markdown_intro(
    project: &Project,
    snapshot: &Snapshot,
    analysis: &Analysis,
    path: &str,
    highlighter: &Highlighter,
) -> Result<MarkdownIntroContent, Vec<Diagnostic>> {
    markdown_intro_with_context(
        project,
        snapshot,
        analysis,
        path,
        highlighter,
        (&project.config.site.lang, &project.config.site.lang, "/"),
    )
}

/// The same source introduction inside a specific target-language surface.
#[doc(hidden)]
pub fn markdown_intro_with_context(
    project: &Project,
    snapshot: &Snapshot,
    analysis: &Analysis,
    path: &str,
    highlighter: &Highlighter,
    context: (&str, &str, &str),
) -> Result<MarkdownIntroContent, Vec<Diagnostic>> {
    let (ui_lang, source_lang, route) = context;
    let path = config::relative_path(path).map_err(|message| {
        vec![Diagnostic::general(
            Severity::Error,
            "markdown-intro",
            message,
        )]
    })?;
    let root = real_root(project);
    let mut sources = Sources {
        project,
        snapshot,
        theme: theme_paths(project, &root),
        messages: message_paths(project, &root),
        root,
        outside: BTreeMap::new(),
        inputs: InputReceipts::default(),
        named: BTreeSet::new(),
    };
    sources.track(&path);
    if sources.theme_file(&path) {
        return Err(vec![Diagnostic::in_file(
            &path,
            Severity::Error,
            "markdown-intro",
            "the theme document is not reading content",
        )]);
    }
    let text = read_include(&mut sources, &path).map_err(|message| {
        vec![Diagnostic::in_file(
            &path,
            Severity::Error,
            "markdown-intro",
            message,
        )]
    })?;
    let range = markdown::intro_range(&text)
        .map_err(|issue| vec![Diagnostic::located(&path, &text, issue)])?;
    let body = &text[range.clone()];
    let mut includes = IncludeFiles::default();
    for include in markdown::include_paths(body) {
        sources.track(&include);
        includes
            .0
            .insert(include.clone(), read_include(&mut sources, &include));
    }
    let catalog = analysis
        .catalogs
        .iter()
        .find(|(lang, _)| lang.eq_ignore_ascii_case(ui_lang))
        .map(|(_, catalog)| catalog.clone())
        .unwrap_or_else(|| Arc::new(labels::resolve(ui_lang, &BTreeMap::new())));
    let fragment = markdown::render(
        body,
        range.start,
        &markdown::Context {
            highlighter,
            catalog: &catalog,
            source_lang,
            includes: &includes,
        },
    );
    let by_path = analysis
        .pages
        .iter()
        .enumerate()
        .filter(|(_, page)| page.origin == PageOrigin::Markdown && !page.locale.is_fallback)
        .map(|(index, page)| (page.path.clone(), index))
        .collect();
    let anchors = analysis
        .pages
        .iter()
        .map(|page| {
            (
                page.route.clone(),
                page.fragment.anchors.iter().cloned().collect(),
            )
        })
        .collect();
    let graph = LinkGraph {
        project,
        snapshot,
        pages: &analysis.pages,
        by_path: &by_path,
        routes: &analysis.routes,
        anchors: &anchors,
        mode: analysis.mode,
    };
    let source_dir = path.rsplit_once('/').map_or("", |(dir, _)| dir);
    let mut found = fragment.issues.clone();
    let mut targets = Vec::with_capacity(fragment.references.len());
    for reference in &fragment.references {
        let issue = |rule: &'static str, message: String| {
            Issue::error(reference.span.clone(), rule, message)
        };
        let url = reference.destination.trim();
        let target = match reference.kind {
            RefKind::Link => {
                resolve_source_link(&graph, &mut sources, &path, url, &mut found, &issue)
            }
            RefKind::Frame => frame_target(None, url, &mut found, &issue),
            RefKind::Form => form_target(
                None,
                url,
                &mut found,
                &issue,
                reference.span.clone(),
                |_| Target::Keep,
            ),
            RefKind::Image | RefKind::Media | RefKind::Srcset | RefKind::ClassImage => {
                resolve_resource(
                    &mut sources,
                    source_dir,
                    true,
                    url,
                    reference.kind != RefKind::Media,
                    reference.kind == RefKind::Srcset,
                    &mut found,
                    &issue,
                )
            }
        };
        let target = if let Target::Page { page, anchor } = target {
            let page = graph.projected_page(page, Some(route), url);
            if let Some(anchor) = &anchor {
                if !graph
                    .anchors
                    .get(&graph.pages[page].route)
                    .is_some_and(|known| known.contains(anchor))
                {
                    found.push(issue(
                        "broken-anchor",
                        format!(
                            "{} has no heading {}",
                            literal(&graph.pages[page].file),
                            code_span(anchor)
                        ),
                    ));
                }
            }
            Target::Page { page, anchor }
        } else {
            target
        };
        targets.push(sources.refuse_theme(target, &mut found, &issue));
    }
    Ok(MarkdownIntroContent {
        html: crate::generation::source_fragment(project, analysis, &fragment, &targets),
        inputs: sources.inputs,
        referenced: sources.named,
        diagnostics: found
            .into_iter()
            .map(|issue| Diagnostic::located(&path, &text, issue))
            .collect(),
    })
}

fn resolve_source_link(
    graph: &LinkGraph,
    sources: &mut Sources,
    source: &str,
    url: &str,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
) -> Target {
    if url.is_empty() || has_scheme(url) || url.starts_with('/') {
        return resolve_link(graph, sources, None, "", url, found, issue);
    }
    let (path, fragment) = split_fragment(url);
    let fragment = fragment.map(percent_decode);
    let dir = source.rsplit_once('/').map_or("", |(dir, _)| dir);
    let file = if path.is_empty() {
        Some(source.to_string())
    } else {
        join(
            dir,
            &percent_decode(path.split('?').next().unwrap_or_default()),
        )
    };
    let Some(file) = file else {
        found.push(issue(
            "broken-link",
            format!("{} leaves the project", code_span(path)),
        ));
        return Target::Keep;
    };
    if let Some(page) = file
        .strip_prefix(&format!("{}/", graph.project.config.content_dir))
        .and_then(|path| graph.by_path.get(path))
        .map(|&index| &graph.pages[index])
    {
        let mut route = page.route.clone();
        if let Some(fragment) = &fragment {
            route.push('#');
            route.push_str(&crate::escape::encode_fragment(fragment));
        }
        return resolve_link(graph, sources, None, "", &route, found, issue);
    }
    repository_link(
        graph.project,
        sources,
        &file,
        fragment.as_deref(),
        found,
        issue,
    )
}

struct LinkGraph<'a> {
    project: &'a Project,
    snapshot: &'a Snapshot,
    pages: &'a [Page],
    by_path: &'a HashMap<String, usize>,
    routes: &'a BTreeMap<String, usize>,
    anchors: &'a BTreeMap<String, BTreeSet<String>>,
    mode: Mode,
}

impl LinkGraph<'_> {
    fn projected_page(&self, target: usize, from: Option<&str>, path: &str) -> usize {
        let config = &self.project.config;
        if config.locales.is_empty() {
            return target;
        }
        let source = from
            .and_then(|route| self.routes.get(route))
            .map(|&i| &self.pages[i]);
        let explicit = if path.starts_with('/') {
            let path = without_base(path, &config.site.base);
            config.locales.iter().any(|locale| {
                !locale.prefix.is_empty() && path.starts_with(&format!("/{}/", locale.prefix))
            })
        } else {
            !self.pages[target]
                .locale
                .source_lang
                .eq_ignore_ascii_case(&config.site.lang)
                && source.is_none_or(|page| {
                    !page
                        .locale
                        .source_lang
                        .eq_ignore_ascii_case(&self.pages[target].locale.source_lang)
                })
        };
        if explicit {
            return target;
        }
        let lang = source
            .map(|page| page.lang.as_str())
            .or_else(|| {
                from.and_then(|route| {
                    config
                        .locales
                        .iter()
                        .find(|locale| {
                            !locale.prefix.is_empty()
                                && route.starts_with(&format!("/{}/", locale.prefix))
                        })
                        .map(|locale| locale.lang.as_str())
                })
            })
            .unwrap_or(&config.site.lang);
        let route = locale_route(config, lang, &self.pages[target].locale.logical_route);
        self.routes.get(&route).copied().unwrap_or(target)
    }

    /// Reports an anchor the page does not have, as a link to it would.
    fn check_anchor(
        &self,
        route: &str,
        anchor: Option<&str>,
        found: &mut Vec<Issue>,
        issue: &dyn Fn(&'static str, String) -> Issue,
    ) {
        let Some(anchor) = anchor.filter(|anchor| !anchor.is_empty()) else {
            return;
        };
        if self
            .anchors
            .get(route)
            .is_some_and(|known| known.contains(anchor))
        {
            return;
        }
        let file = self
            .routes
            .get(route)
            .map_or(route, |&i| self.pages[i].file.as_str());
        found.push(
            issue(
                "broken-anchor",
                format!(
                    "{} has no heading {}",
                    literal(file),
                    code_span(&format!("#{anchor}"))
                ),
            )
            .suggest("link to an existing heading's id; ids are the heading text in lower case with spaces as `-`"),
        );
    }
}

fn collect_resources(targets: &[Target], resources: &mut BTreeSet<String>) {
    for target in targets {
        match target {
            Target::Resource(path) => {
                resources.insert(path.clone());
            }
            Target::Srcset(candidates) => {
                resources.extend(candidates.iter().filter_map(|c| c.file.clone()))
            }
            _ => {}
        }
    }
}

/// Resolves a page's or surface's references. `landing` is `None` off
/// landing pages, and a landing page's profile, if it has one, on them.
fn resolve_references(
    graph: &LinkGraph,
    sources: &mut Sources,
    route: &str,
    dir: &str,
    rendered: bool,
    fragment: &Fragment,
    landing: Option<Option<&crate::landing::Profile>>,
) -> (Vec<Target>, Vec<Issue>) {
    let mut targets = Vec::with_capacity(fragment.references.len());
    let mut found = Vec::new();
    for reference in &fragment.references {
        let issue = |rule: &'static str, message: String| {
            Issue::error(reference.span.clone(), rule, message)
        };
        let url = reference.destination.trim();
        let target = match reference.kind {
            RefKind::ClassImage | RefKind::Image
                if url.starts_with('#') && (rendered || reference.kind == RefKind::ClassImage) =>
            {
                let anchor = percent_decode(&url[1..]);
                if !graph
                    .anchors
                    .get(route)
                    .is_some_and(|known| known.contains(&anchor))
                {
                    found.push(issue(
                        "broken-anchor",
                        format!("{} has no id {}", literal(route), code_span(url)),
                    ));
                }
                Target::Keep
            }
            RefKind::Link if rendered => {
                resolve_html_link(graph, sources, route, url, &mut found, &issue)
            }
            RefKind::Link => {
                resolve_link(graph, sources, Some(route), dir, url, &mut found, &issue)
            }
            RefKind::Image | RefKind::Media | RefKind::Srcset | RefKind::ClassImage => {
                resolve_resource(
                    sources,
                    dir,
                    rendered,
                    url,
                    reference.kind != RefKind::Media,
                    reference.kind == RefKind::Srcset,
                    &mut found,
                    &issue,
                )
            }
            RefKind::Frame => frame_target(landing, url, &mut found, &issue),
            RefKind::Form => {
                let span = reference.span.clone();
                form_target(landing, url, &mut found, &issue, span, |found| {
                    // A target on the site resolves like the page's links,
                    // and keeps the query it submits.
                    form_page_target(graph, sources, route, dir, rendered, url, found, &issue)
                })
            }
        };
        targets.push(sources.refuse_theme(target, &mut found, &issue));
    }
    (targets, found)
}

/// What an `iframe` may embed: nothing, except on a landing page whose
/// profile lists the frame's origin in `sources.frame`.
fn frame_target(
    landing: Option<Option<&crate::landing::Profile>>,
    url: &str,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
) -> Target {
    match landing
        .flatten()
        .filter(|profile| !profile.sources.frame.is_empty())
    {
        None => found.push(
            issue(
                "embedded-frame",
                "an <iframe> is allowed only on a `layout: landing` page whose profile lists its origin in `sources.frame`".into(),
            )
            .suggest("move it to a landing page, or link to the page instead"),
        ),
        Some(profile) if !crate::landing::allows(&profile.sources.frame, url) => found.push(
            issue(
                "frame-origin",
                format!(
                    "{} is not an http(s) address on an origin landing profile {} lists in `sources.frame`",
                    code_span(url),
                    code_span(&profile.name)
                ),
            )
            .suggest(format!(
                "list its origin, like `https://forms.example.com`, under [landing.profiles.{}.sources] frame",
                profile.name
            )),
        ),
        Some(_) => {}
    }
    Target::Keep
}

/// Where a form may submit. Off landing pages every form is blocked, as it
/// always was, so it is only a warning; on one, the profile must list the
/// target's origin in `sources.form`, and a target on the site must exist.
fn form_target(
    landing: Option<Option<&crate::landing::Profile>>,
    url: &str,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
    span: std::ops::Range<usize>,
    on_site: impl FnOnce(&mut Vec<Issue>) -> Target,
) -> Target {
    let Some(profile) = landing else {
        found.push(
            Issue::warning(
                span,
                "form-blocked",
                format!(
                    "a form submits to {}, but the page's policy blocks every form submission (`form-action 'none'`)",
                    code_span(url)
                ),
            )
            .suggest("put the form on a `layout: landing` page whose profile lists its target in `sources.form`"),
        );
        return Target::Keep;
    };
    let Some(profile) = profile.filter(|profile| !profile.sources.form.is_empty()) else {
        found.push(
            issue(
                "form-origin",
                match profile {
                    Some(profile) => format!(
                        "landing profile {} lists no form target, so the page cannot submit a form",
                        code_span(&profile.name)
                    ),
                    None => "this landing page selects no profile with form targets, so it cannot submit a form".into(),
                },
            )
            .suggest("list the form's origin under `sources.form` of the page's landing profile"),
        );
        return Target::Keep;
    };
    if url.is_empty() || url.starts_with('#') {
        return Target::Keep;
    }
    if crate::html_check::is_cross_origin(url, false) {
        if !crate::landing::allows(&profile.sources.form, url) {
            found.push(
                issue(
                    "form-origin",
                    format!(
                        "{} is not on an origin landing profile {} lists in `sources.form`",
                        code_span(url),
                        code_span(&profile.name)
                    ),
                )
                .suggest(format!(
                    "list its origin, like `https://leads.example.com`, under [landing.profiles.{}.sources] form",
                    profile.name
                )),
            );
        }
        return Target::Keep;
    }
    on_site(found)
}

/// Where a form on the site submits: the page its path names, with the
/// query it submits. The query is not part of the name, so the path
/// resolves as a link on the page does; a target with only a query is the
/// page it is written on.
#[allow(clippy::too_many_arguments)]
fn form_page_target(
    graph: &LinkGraph,
    sources: &mut Sources,
    route: &str,
    dir: &str,
    rendered: bool,
    url: &str,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
) -> Target {
    let (before, fragment) = split_fragment(url);
    let (path, query) = match before.split_once('?') {
        Some((path, query)) => (path, Some(query.to_owned())),
        None => (before, None),
    };
    let anchor = fragment.map(percent_decode);
    if path.is_empty() {
        let Some(&page) = graph.routes.get(route) else {
            return Target::Keep;
        };
        graph.check_anchor(route, anchor.as_deref(), found, issue);
        return Target::Form {
            page,
            query,
            anchor,
        };
    }
    let path = match fragment {
        Some(fragment) => format!("{path}#{fragment}"),
        None => path.to_owned(),
    };
    let target = if rendered {
        resolve_html_link(graph, sources, route, &path, found, issue)
    } else {
        resolve_link(graph, sources, Some(route), dir, &path, found, issue)
    };
    match target {
        Target::Page { page, anchor } => Target::Form {
            page,
            query,
            anchor,
        },
        target => target,
    }
}

/// Removes only an exact configured deployment prefix from a browser URL.
fn without_base<'a>(path: &'a str, base: &str) -> &'a str {
    if base != "/" && path.starts_with(base) {
        &path[base.len() - 1..]
    } else {
        path
    }
}

fn resolve_html_link(
    graph: &LinkGraph,
    sources: &mut Sources,
    from: &str,
    url: &str,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
) -> Target {
    if url.is_empty() || has_scheme(url) || url.starts_with('/') || url.starts_with('#') {
        return resolve_link(graph, sources, Some(from), "", url, found, issue);
    }
    let (path, fragment) = split_fragment(url);
    let decoded = percent_decode(path.split('?').next().unwrap_or_default());
    let Some(file) = join("", &decoded) else {
        found.push(issue(
            "broken-link",
            format!("{} leaves the project", code_span(path)),
        ));
        return Target::Keep;
    };
    if excluded_markdown(graph.project, &file) {
        let fragment = fragment.map(percent_decode);
        return repository_link(
            graph.project,
            sources,
            &file,
            fragment.as_deref(),
            found,
            issue,
        );
    }
    let route = format!("/{file}/");
    let target = graph.routes.get(&route).copied().or_else(|| {
        file.strip_prefix(&format!("{}/", graph.project.config.content_dir))
            .and_then(|path| graph.by_path.get(path).copied())
    });
    if let Some(target) = target {
        let mut url = graph.pages[target].route.clone();
        if let Some(fragment) = fragment {
            url.push('#');
            url.push_str(fragment);
        }
        return resolve_link(graph, sources, Some(from), "", &url, found, issue);
    }
    match sources.get(&file) {
        Ok(()) => Target::Resource(file),
        Err(refusal) => {
            found.push(refusal.issue(issue, &code_span(path), "broken-link"));
            Target::Keep
        }
    }
}

fn resolve_link(
    graph: &LinkGraph,
    sources: &mut Sources,
    from: Option<&str>,
    dir: &str,
    url: &str,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
) -> Target {
    let LinkGraph {
        project,
        snapshot,
        pages,
        by_path,
        routes,
        mode,
        ..
    } = graph;
    if url.is_empty() {
        found.push(issue("empty-link", "this link has no destination".into()));
        return Target::Keep;
    }
    if has_scheme(url) || url.starts_with("//") {
        let scheme = url.split(':').next().unwrap_or("").to_ascii_lowercase();
        if matches!(scheme.as_str(), "javascript" | "vbscript" | "data") {
            found.push(issue(
                "script-url",
                format!("a {} link is not allowed", code_span(&format!("{scheme}:"))),
            ));
        }
        return Target::Keep;
    }
    let (path, fragment) = split_fragment(url);
    let fragment = fragment.map(percent_decode);
    let decoded = percent_decode(path.split('?').next().unwrap_or_default());
    let to_page = |target: usize, found: &mut Vec<Issue>| -> Target {
        let target = graph.projected_page(target, from, &decoded);
        if pages[target].draft
            && !from
                .and_then(|route| routes.get(route))
                .is_some_and(|&i| pages[i].draft)
        {
            let message = format!(
                "{} is a draft and is not published",
                literal(&pages[target].file)
            );
            match mode {
                Mode::Production => found.push(issue("draft-link", message)),
                Mode::Dev => found.push(Issue {
                    severity: Severity::Warning,
                    ..issue(
                        "draft-link",
                        format!("{message}; a production build stops here"),
                    )
                }),
            }
        }
        graph.check_anchor(&pages[target].route, fragment.as_deref(), found, issue);
        Target::Page {
            page: target,
            anchor: fragment.clone(),
        }
    };
    if path.is_empty() {
        match from {
            Some(from) => graph.check_anchor(from, fragment.as_deref(), found, issue),
            None => found.push(issue(
                "broken-link",
                format!("{} names no page; link to a page or a URL", code_span(url)),
            )),
        }
        return Target::Keep;
    }
    // URL separators are syntax before decoding: `%3F` belongs to a name.
    if let Some(absolute) = without_base(&decoded, &project.config.site.base).strip_prefix('/') {
        // A site route, or a file in the asset directory.
        let route = if absolute.is_empty() || absolute.ends_with('/') {
            format!("/{absolute}")
        } else {
            format!("/{absolute}/")
        };
        if let Some(&target) = routes.get(&route) {
            return to_page(target, found);
        }
        let asset = format!("{}/{absolute}", project.config.assets_dir);
        if snapshot.files.contains_key(&asset) {
            return Target::Resource(asset);
        }
        found.push(issue(
            "broken-link",
            format!("no page or file at {}", code_span(path)),
        ));
        return Target::Keep;
    }
    match join(dir, &decoded) {
        Some(joined) => {
            let as_markdown = if joined.ends_with(".md") {
                Some(joined.clone())
            } else if decoded.ends_with('/') || joined.is_empty() {
                Some(if joined.is_empty() {
                    "index.md".to_owned()
                } else {
                    format!("{joined}/index.md")
                })
            } else {
                None
            };
            if let Some(markdown) = as_markdown {
                let target = by_path.get(markdown.as_str()).or_else(|| {
                    (!project.config.locales.is_empty())
                        .then(|| routes.get(&route_for(&markdown)))
                        .flatten()
                });
                return match target {
                    Some(&target) => to_page(target, found),
                    None => {
                        let file = format!("{}/{markdown}", project.config.content_dir);
                        if joined.ends_with(".md") && excluded_markdown(project, &file) {
                            return repository_link(
                                project,
                                sources,
                                &file,
                                fragment.as_deref(),
                                found,
                                issue,
                            );
                        }
                        let route = if joined.is_empty() {
                            "/".into()
                        } else {
                            format!("/{joined}/")
                        };
                        if let Some(&target) = routes
                            .get(&route)
                            .filter(|&&i| matches!(pages[i].origin, PageOrigin::Registered(_)))
                        {
                            return to_page(target, found);
                        }
                        if excluded_markdown(project, &file) {
                            return repository_link(
                                project,
                                sources,
                                &file,
                                fragment.as_deref(),
                                found,
                                issue,
                            );
                        }
                        found.push(issue(
                            "broken-link",
                            format!(
                                "there is no page {}",
                                code_span(&format!("{0}/{markdown}", project.config.content_dir))
                            ),
                        ));
                        Target::Keep
                    }
                };
            }
            let file = format!("{}/{joined}", project.config.content_dir);
            if snapshot.files.contains_key(&file) && !is_private(&file) {
                return Target::Resource(file);
            }
            if project.config.content_include.is_some() {
                return match sources.get(&file) {
                    Ok(()) => Target::Resource(file),
                    Err(refusal) => {
                        found.push(refusal.issue(issue, &code_span(&file), "broken-link"));
                        Target::Keep
                    }
                };
            }
            found.push(issue(
                "broken-link",
                format!("there is no file {}", code_span(&file)),
            ));
            Target::Keep
        }
        None => {
            // Outside the content: a file in the repository.
            let project_dir = format!("{}/{dir}", project.config.content_dir);
            let Some(file) = join(&project_dir, &decoded) else {
                found.push(issue(
                    "broken-link",
                    format!("{} leaves the project", code_span(path)),
                ));
                return Target::Keep;
            };
            if sources.message_file(&file) {
                found.push(Refusal::Messages.issue(issue, &code_span(&file), "i18n-messages"));
                return Target::Keep;
            }
            if sources.exists(&file).is_err() {
                found.push(issue(
                    "broken-link",
                    format!("there is no file {}", code_span(&file)),
                ));
                return Target::Keep;
            }
            match &project.config.repo {
                Some(repo) => {
                    let in_repo = if repo.path.is_empty() {
                        file
                    } else {
                        format!("{}/{file}", repo.path)
                    };
                    let kind = if project.root.join(&in_repo).is_dir() {
                        "tree"
                    } else {
                        "blob"
                    };
                    let mut url = format!("{}/{kind}/{}/{in_repo}", repo.url, repo.reference);
                    if let Some(fragment) = &fragment {
                        url.push('#');
                        url.push_str(fragment);
                    }
                    Target::Repository(url)
                }
                None => {
                    found.push(
                        issue("repo-link", format!("{} is outside the content and the site has no [repo] to link it to", code_span(&file)))
                            .suggest("add [repo] url and ref to rustify-docs.toml, or link to a page"),
                    );
                    Target::Keep
                }
            }
        }
    }
}

fn excluded_markdown(project: &Project, file: &str) -> bool {
    let Some(selected) = &project.config.content_include else {
        return false;
    };
    let Some(relative) = file.strip_prefix(&format!("{}/", project.config.content_dir)) else {
        return false;
    };
    relative.ends_with(".md") && !selected.iter().any(|(file, _)| file == relative)
}

/// Resolves a normalized project-relative repository file without reading its body.
fn repository_link(
    project: &Project,
    sources: &mut Sources,
    file: &str,
    fragment: Option<&str>,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
) -> Target {
    if let Err(refusal) = sources.exists(file) {
        found.push(refusal.issue(issue, &code_span(file), "broken-link"));
        return Target::Keep;
    }
    let Some(repo) = &project.config.repo else {
        found.push(
            issue(
                "repo-link",
                format!(
                    "{} is not a published page and the site has no [repo] to link it to",
                    code_span(file)
                ),
            )
            .suggest("add [repo] url and ref to rustify-docs.toml, or link to a published page"),
        );
        return Target::Keep;
    };
    let in_repo = if repo.path.is_empty() {
        file.to_owned()
    } else {
        format!("{}/{file}", repo.path)
    };
    let kind = if sources
        .inputs
        .files
        .get(file)
        .is_some_and(|read| read.directory)
    {
        "tree"
    } else {
        "blob"
    };
    let mut url = format!("{}/{kind}/{}/{in_repo}", repo.url, repo.reference);
    if let Some(fragment) = fragment {
        url.push('#');
        url.push_str(fragment);
    }
    Target::Repository(url)
}

#[allow(clippy::too_many_arguments)]
fn resolve_resource(
    sources: &mut Sources,
    dir: &str,
    rendered: bool,
    url: &str,
    image: bool,
    srcset: bool,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
) -> Target {
    let candidates: Vec<(String, String)> = if srcset {
        crate::html_check::parse_srcset(url)
    } else {
        vec![(url.to_owned(), String::new())]
    };
    let mut resolved = Vec::with_capacity(candidates.len());
    for (candidate, descriptor) in candidates {
        let keep = |resolved: &mut Vec<SrcsetCandidate>| {
            resolved.push(SrcsetCandidate {
                file: None,
                url: candidate.clone(),
                descriptor: descriptor.clone(),
            })
        };
        if crate::html_check::is_cross_origin(&candidate, image) {
            found.push(
                issue(
                    "cross-origin-resource",
                    format!(
                        "{} is loaded from another origin, which the site's policy blocks",
                        code_span(&candidate)
                    ),
                )
                .suggest("download it into the asset directory and reference it from there"),
            );
            keep(&mut resolved);
            continue;
        }
        if candidate.to_ascii_lowercase().starts_with("data:") {
            keep(&mut resolved);
            continue;
        }
        let (path, _) = split_fragment(&candidate);
        let path = percent_decode(path.split('?').next().unwrap_or(""));
        let config = &sources.project.config;
        let path = without_base(&path, &config.site.base);
        let file = match path.strip_prefix('/') {
            Some(absolute) => Some(format!("{}/{absolute}", config.assets_dir)),
            None if rendered => join(dir, path),
            None => join(&format!("{}/{dir}", config.content_dir), path),
        };
        let refusal = match &file {
            Some(file) => sources.get(file).err(),
            None => Some(Refusal::Missing),
        };
        match (file, refusal) {
            (Some(file), None) => {
                resolved.push(SrcsetCandidate {
                    file: Some(file),
                    url: candidate.clone(),
                    descriptor: descriptor.clone(),
                });
            }
            (_, refusal) => {
                let missing = if image {
                    "missing-image"
                } else {
                    "missing-media"
                };
                let mut problem = refusal.unwrap_or(Refusal::Missing).issue(
                    issue,
                    &code_span(&candidate),
                    missing,
                );
                if problem.rule == missing {
                    problem = problem.suggest("check the path; it is relative to this page, or to the asset directory when it starts with `/`");
                }
                found.push(problem);
                keep(&mut resolved);
            }
        }
    }
    if srcset {
        return Target::Srcset(resolved);
    }
    match resolved.pop() {
        Some(SrcsetCandidate {
            file: Some(file), ..
        }) => Target::Resource(file),
        _ => Target::Keep,
    }
}

fn validate_sidebar_groups(
    project: &Project,
    snapshot: &Snapshot,
    pages: &[Page],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let config = &project.config;
    for page in pages.iter().filter(|page| !page.locale.is_fallback) {
        if page.origin != PageOrigin::Markdown {
            continue;
        }
        let nav = config
            .locales
            .iter()
            .find(|locale| locale.lang == page.lang)
            .and_then(|locale| locale.nav.as_ref())
            .unwrap_or(&config.nav);
        if nav.groups.is_empty() {
            continue;
        }
        let Some(label) = page
            .front
            .sidebar
            .group
            .as_deref()
            .filter(|label| !nav.groups.iter().any(|group| group.label == *label))
        else {
            continue;
        };
        let text = &snapshot.markdown[&page.path];
        let yaml = frontmatter::split(text)
            .ok()
            .flatten()
            .map_or(0..0, |split| split.yaml);
        let at = text[yaml.clone()]
            .find(label)
            .map_or(yaml.start, |at| yaml.start + at);
        let names: Vec<String> = nav
            .groups
            .iter()
            .map(|group| literal(&group.label))
            .collect();
        diagnostics.push(Diagnostic::located(
            &page.file,
            text,
            Issue::error(
                at..at + label.len(),
                "nav-unknown-group",
                format!(
                    "sidebar.group {} is not a nav group in {}",
                    code_span(label),
                    literal(&project.config_file)
                ),
            )
            .suggest(format!("use one of: {}", names.join(", "))),
        ));
    }
}

fn navigation(
    project: &Project,
    pages: &[Page],
    mode: Mode,
    by_path: &HashMap<String, usize>,
    routes: &BTreeMap<String, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<NavGroup> {
    let config = &project.config;
    let shown = |page: &Page| mode == Mode::Dev || !page.draft;
    let listable = |page: &Page| {
        shown(page)
            && !page.front.layout.has_hero()
            && (page.origin == PageOrigin::Markdown || page.route != "/")
    };
    fn chosen(page: &Page) -> Option<&str> {
        page.front.sidebar.group.as_deref()
    }
    // Within a group, pages go by `sidebar.order`, then by path.
    let ranked = |mut found: Vec<usize>| -> Vec<usize> {
        found.sort_by_key(|&i| {
            (
                pages[i].front.sidebar.order.unwrap_or(i64::MAX),
                pages[i].path.as_str(),
            )
        });
        found
    };
    let item = |i: usize| {
        let page = &pages[i];
        NavItem {
            lang: page.locale.source_lang.clone(),
            title: page
                .front
                .sidebar
                .label
                .clone()
                .unwrap_or_else(|| page.title.clone()),
            route: page.route.clone(),
            draft: page.draft,
        }
    };
    let mut listed: BTreeSet<usize> = BTreeSet::new();
    let mut groups = Vec::new();
    if config.nav.groups.is_empty() {
        // One group named after the site, then one per `sidebar.group`, in
        // the order their first pages come.
        let mut labels: Vec<(&str, Vec<usize>)> = vec![(config.site.name.as_str(), Vec::new())];
        for (i, page) in pages.iter().enumerate().filter(|(_, page)| listable(page)) {
            let label = chosen(page).unwrap_or(&config.site.name);
            match labels.iter_mut().find(|(known, _)| *known == label) {
                Some((_, found)) => found.push(i),
                None => labels.push((label, vec![i])),
            }
        }
        for (label, found) in labels {
            if !found.is_empty() {
                groups.push(NavGroup {
                    lang: config.site.lang.clone(),
                    label: label.to_owned(),
                    prefix: "/".into(),
                    items: ranked(found).into_iter().map(item).collect(),
                });
            }
        }
        return groups;
    }

    let known = |label: &str| config.nav.groups.iter().any(|group| group.label == label);
    // Explicit file and route lists keep their pages in the declared group.
    let explicit: BTreeSet<usize> = config
        .nav
        .groups
        .iter()
        .filter_map(|group| match &group.source {
            GroupSource::Items(list) => Some((list, false)),
            GroupSource::Routes(list) => Some((list, true)),
            GroupSource::Dir { .. } => None,
        })
        .flat_map(|(list, registered)| {
            list.iter().filter_map(move |(path, _)| {
                if registered {
                    routes.get(path).copied()
                } else {
                    by_path.get(path).copied()
                }
            })
        })
        .collect();
    let mut selected_routes = BTreeSet::new();
    for group in &config.nav.groups {
        let joins = |i: usize| {
            listable(&pages[i])
                && !explicit.contains(&i)
                && chosen(&pages[i]) == Some(group.label.as_str())
        };
        let found: Vec<usize> = match &group.source {
            GroupSource::Dir { dir, span } => {
                let prefix = if dir.is_empty() {
                    String::new()
                } else {
                    format!("{dir}/")
                };
                let found: Vec<usize> = (0..pages.len())
                    .filter(|&i| {
                        let page = &pages[i];
                        let moved =
                            chosen(page).is_some_and(|label| label != group.label && known(label));
                        page.origin == PageOrigin::Markdown
                            && (joins(i)
                                || (page.path.starts_with(&prefix) && listable(page) && !moved))
                    })
                    .collect();
                if found.is_empty() {
                    diagnostics.push(Diagnostic::located(
                        &project.config_file,
                        &project.config_text,
                        Issue::warning(
                            span.clone(),
                            "nav-empty",
                            format!(
                                "nav group {} lists no pages under {}",
                                code_span(&group.label),
                                code_span(dir)
                            ),
                        ),
                    ));
                }
                ranked(found)
            }
            GroupSource::Items(list) => {
                let mut found = Vec::new();
                for (path, span) in list {
                    match by_path.get(path.as_str()) {
                        Some(&i) if shown(&pages[i]) => found.push(i),
                        Some(_) => {}
                        None => diagnostics.push(Diagnostic::located(
                            &project.config_file,
                            &project.config_text,
                            Issue::error(
                                span.clone(),
                                "nav-missing-page",
                                format!(
                                    "nav lists {}, which is not a page in {}",
                                    code_span(path),
                                    literal(&config.content_dir)
                                ),
                            ),
                        )),
                    }
                }
                found.extend(ranked((0..pages.len()).filter(|&i| joins(i)).collect()));
                found
            }
            GroupSource::Routes(list) => {
                let mut found = Vec::new();
                for (route, span) in list {
                    let problem = if !selected_routes.insert(route) {
                        Some((
                            "nav-duplicate-route",
                            format!("nav.routes lists {} more than once", code_span(route)),
                        ))
                    } else {
                        match routes.get(route) {
                            Some(&i) if pages[i].origin == PageOrigin::Markdown => Some(("nav-route-origin", format!("nav.routes lists {}, which is a Markdown page; use dir or items", code_span(route)))),
                            Some(&i) => {
                                if listable(&pages[i]) { found.push(i); }
                                None
                            }
                            None => Some(("nav-missing-page", format!("nav.routes lists {}, which is not a registered page", code_span(route)))),
                        }
                    };
                    if let Some((rule, message)) = problem {
                        diagnostics.push(Diagnostic::located(
                            &project.config_file,
                            &project.config_text,
                            Issue::error(span.clone(), rule, message),
                        ));
                    }
                }
                found
            }
        };
        listed.extend(found.iter().copied());
        groups.push(NavGroup {
            lang: config.site.lang.clone(),
            label: group.label.clone(),
            prefix: group.scope.clone(),
            items: found.into_iter().map(item).collect(),
        });
    }
    for (i, page) in pages.iter().enumerate() {
        if !listed.contains(&i) && listable(page) && !page.draft {
            diagnostics.push(Diagnostic::in_file(
                &page.file,
                Severity::Warning,
                "not-in-nav",
                "this page is in no navigation group, so readers can only reach it by a link",
            ));
        }
    }
    groups
}

impl Diagnostic {
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A project in a fresh temporary directory, removed when dropped.
    pub(crate) struct Fixture {
        pub root: PathBuf,
    }

    impl Fixture {
        pub(crate) fn new(files: &[(&str, &str)]) -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let root =
                std::env::temp_dir().join(format!("rustify-docs-test-{}-{n}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            for (path, text) in files {
                let path = root.join(path);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, text).unwrap();
            }
            Self { root }
        }

        pub(crate) fn analyze(&self, mode: Mode) -> (Project, Snapshot, Analysis) {
            self.analyze_with_surfaces(mode, &Surfaces::default())
        }

        fn analyze_with_surfaces(
            &self,
            mode: Mode,
            surfaces: &Surfaces,
        ) -> (Project, Snapshot, Analysis) {
            let (project, mut warnings) =
                Project::load(&self.root).unwrap_or_else(|errors| panic!("{errors:#?}"));
            let (snapshot, problems) = snapshot(&project);
            warnings.extend(problems);
            let mut analysis = analyze_with_surfaces(
                &project,
                &snapshot,
                mode,
                &mut FragmentCache::default(),
                &Highlighter::new(),
                2,
                surfaces,
            );
            analysis.diagnostics.extend(warnings);
            (project, snapshot, analysis)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    pub(crate) const CONFIG: &str =
        "schema_version = 1\n[site]\nname = \"Docs\"\nurl = \"https://example.com/\"\n";

    #[cfg(unix)]
    #[test]
    fn render_guard_distinguishes_existing_scan_errors_from_changed_inputs() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", "# Home\n\nA readable home page.\n"),
            ("static/safe.txt", "safe\n"),
        ]);
        let outside = Fixture::new(&[("outside.txt", "secret\n")]);
        let link = fixture.root.join("static/outside.txt");
        std::os::unix::fs::symlink(outside.root.join("outside.txt"), &link).unwrap();
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        assert!(analysis
            .diagnostics
            .iter()
            .any(|problem| problem.rule == "symlink-outside"));
        assert!(analysis.errors() > 0, "the original input remains invalid");
        assert!(validate_inputs(&project, &before, &analysis).is_empty());

        std::fs::remove_file(&link).unwrap();
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        std::os::unix::fs::symlink(outside.root.join("outside.txt"), &link).unwrap();
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    #[test]
    fn render_guard_checks_the_original_outside_resource_read() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                "# Home\n\n![External](../images/sample.svg)\n",
            ),
            ("images/sample.svg", "<svg>old</svg>"),
        ]);
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        assert!(analysis.outside.contains_key("images/sample.svg"));
        assert!(validate_inputs(&project, &before, &analysis).is_empty());
        std::fs::write(fixture.root.join("images/sample.svg"), "<svg>new</svg>").unwrap();
        let (after, _) = snapshot(&project);
        assert_eq!(
            before.files, after.files,
            "the directory snapshot misses this resource"
        );
        let problems = validate_inputs(&project, &before, &analysis);
        assert!(problems
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    #[test]
    fn render_guard_checks_include_only_bytes_from_the_original_analysis() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                "# Home\n\n<!-- rustify:include path=\"extras/snippet.txt\" -->\n",
            ),
            ("extras/snippet.txt", "original include\r\n"),
        ]);
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        assert!(analysis.includes.contains("extras/snippet.txt"));
        assert!(!analysis.outside.contains_key("extras/snippet.txt"));
        assert!(analysis.pages[0].fragment.html.contains("original include"));
        assert!(validate_inputs(&project, &before, &analysis).is_empty());
        std::fs::write(fixture.root.join("extras/snippet.txt"), "new include\r\n").unwrap();
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    #[test]
    fn render_guard_checks_configuration_even_for_content_only_projects() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", "# Home\n"),
        ]);
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        assert!(project.config.rust.is_none());
        std::fs::write(
            fixture.root.join("rustify-docs.toml"),
            CONFIG.replace("Docs", "New name"),
        )
        .unwrap();
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    #[cfg(unix)]
    #[test]
    fn render_guard_detects_swapped_symlink_targets_even_when_snapshot_sets_match() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                "# Home\n\n![First](/first.svg) ![Second](/second.svg)\n",
            ),
            ("images/a.svg", "<svg/>"),
            ("images/b.svg", "<svg/>"),
        ]);
        std::fs::create_dir_all(fixture.root.join("static")).unwrap();
        let first = fixture.root.join("static/first.svg");
        let second = fixture.root.join("static/second.svg");
        std::os::unix::fs::symlink("../images/a.svg", &first).unwrap();
        std::os::unix::fs::symlink("../images/b.svg", &second).unwrap();
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        assert!(validate_inputs(&project, &before, &analysis).is_empty());
        std::fs::remove_file(&first).unwrap();
        std::fs::remove_file(&second).unwrap();
        std::os::unix::fs::symlink("../images/b.svg", &first).unwrap();
        std::os::unix::fs::symlink("../images/a.svg", &second).unwrap();
        let (after, _) = snapshot(&project);
        assert_eq!(before.files, after.files);
        assert_eq!(before.linked, after.linked);
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    #[test]
    fn render_guard_keeps_repository_links_as_existence_only_inputs() {
        let configuration =
            format!("{CONFIG}\n[repo]\nurl = \"https://github.com/owner/repo\"\nref = \"main\"\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &configuration),
            (
                "content/index.md",
                "# Home\n\n[Unselected source](../unselected/notes.md)\n",
            ),
            ("unselected/notes.md", "excluded original body"),
        ]);
        // Even invalid UTF-8 must remain a valid repository link: no body is read.
        std::fs::write(fixture.root.join("unselected/notes.md"), [0xff, 0xfe]).unwrap();
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        assert!(errors(&analysis).is_empty(), "{:#?}", analysis.diagnostics);
        assert!(analysis.inputs.files["unselected/notes.md"].bytes.is_none());
        assert!(!analysis.outside.contains_key("unselected/notes.md"));
        assert!(!analysis.resources.contains("unselected/notes.md"));
        assert!(!analysis.pages[0]
            .fragment
            .html
            .contains("excluded original body"));
        assert!(validate_inputs(&project, &before, &analysis).is_empty());
        std::fs::write(fixture.root.join("unselected/notes.md"), [0xfd]).unwrap();
        assert!(validate_inputs(&project, &before, &analysis).is_empty());
        std::fs::remove_file(fixture.root.join("unselected/notes.md")).unwrap();
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    #[test]
    fn render_guard_binds_a_fragment_cache_hit_to_this_rounds_include_read() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                "# Home\n\n<!-- rustify:include path=\"extras/snippet.txt\" -->\n",
            ),
            ("extras/snippet.txt", "cached original\r\n"),
        ]);
        let (project, _) = Project::load(&fixture.root).unwrap();
        let (before, _) = snapshot(&project);
        let highlighter = Highlighter::new();
        let mut cache = FragmentCache::default();
        analyze(&project, &before, Mode::Dev, &mut cache, &highlighter, 1);
        let hits = cache.hits;
        let analysis = analyze(&project, &before, Mode::Dev, &mut cache, &highlighter, 1);
        assert!(cache.hits > hits);
        assert_eq!(
            analysis.inputs.files["extras/snippet.txt"].bytes.as_deref(),
            Some(b"cached original\r\n".as_slice())
        );
        std::fs::write(fixture.root.join("extras/snippet.txt"), "new original\r\n").unwrap();
        assert!(analysis.pages[0].fragment.html.contains("cached original"));
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    #[test]
    fn render_guard_uses_snapshot_bytes_when_an_include_is_already_captured() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                "# Home\n\n<!-- rustify:include path=\"static/example.txt\" -->\n",
            ),
            ("static/example.txt", "snapshot original"),
        ]);
        let (project, _) = Project::load(&fixture.root).unwrap();
        let (before, _) = snapshot(&project);
        std::fs::write(fixture.root.join("static/example.txt"), "later content").unwrap();
        let analysis = analyze(
            &project,
            &before,
            Mode::Dev,
            &mut FragmentCache::default(),
            &Highlighter::new(),
            1,
        );
        assert!(analysis.pages[0]
            .fragment
            .html
            .contains("snapshot original"));
        assert!(!analysis.pages[0].fragment.html.contains("later content"));
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    #[cfg(unix)]
    #[test]
    fn render_guard_detects_config_link_retargeting_with_identical_bytes() {
        let fixture = Fixture::new(&[
            ("first.toml", CONFIG),
            ("second.toml", CONFIG),
            ("content/index.md", "# Home\n"),
        ]);
        let configuration = fixture.root.join("rustify-docs.toml");
        std::os::unix::fs::symlink("first.toml", &configuration).unwrap();
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        std::fs::remove_file(&configuration).unwrap();
        std::os::unix::fs::symlink("second.toml", &configuration).unwrap();
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(
                |problem| problem.file.as_deref() == Some("rustify-docs.toml")
                    && problem.rule == "render-generation-changed"
            ));
    }

    #[cfg(unix)]
    #[test]
    fn render_guard_detects_empty_directory_links_even_when_target_sets_match() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", "# Home\n"),
        ]);
        for directory in ["static", "empty-a", "empty-b"] {
            std::fs::create_dir_all(fixture.root.join(directory)).unwrap();
        }
        let first = fixture.root.join("static/first");
        let second = fixture.root.join("static/second");
        std::os::unix::fs::symlink("../empty-a", &first).unwrap();
        std::os::unix::fs::symlink("../empty-b", &second).unwrap();
        let (project, before, analysis) = fixture.analyze(Mode::Dev);
        std::fs::remove_file(&first).unwrap();
        std::fs::remove_file(&second).unwrap();
        std::os::unix::fs::symlink("../empty-b", &first).unwrap();
        std::os::unix::fs::symlink("../empty-a", &second).unwrap();
        let (after, _) = snapshot(&project);
        assert_eq!(before.files, after.files);
        assert_eq!(before.linked, after.linked);
        assert!(validate_inputs(&project, &before, &analysis)
            .iter()
            .any(|problem| problem.rule == "render-generation-changed"));
    }

    fn found(analysis: &Analysis) -> Vec<(String, String, usize)> {
        analysis
            .diagnostics
            .iter()
            .map(|d| {
                (
                    d.rule.clone(),
                    d.file.clone().unwrap_or_default(),
                    d.line.unwrap_or(0),
                )
            })
            .collect()
    }

    fn errors(analysis: &Analysis) -> Vec<(String, String, usize)> {
        analysis
            .diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .map(|d| {
                (
                    d.rule.clone(),
                    d.file.clone().unwrap_or_default(),
                    d.line.unwrap_or(0),
                )
            })
            .collect()
    }

    const FRONT: &str = "---\ndescription: d\n---\n";

    fn registered(name: &str, route: &str, html: &str) -> RegisteredPage {
        RegisteredPage {
            draft: None,
            index: None,
            search: None,
            ai: None,
            lang: None,
            name: name.into(),
            route: route.into(),
            title: name.into(),
            description: None,
            html: html.into(),
            file: "src/pages.rs".into(),
        }
    }

    #[test]
    fn source_candidate_urls_resolve_without_changing_the_original_class() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("static/custom.svg", "<svg/>"),
        ]);
        let candidate = "bg-[url(static/custom.svg)]";
        let html = format!("<div class='{candidate}'>Custom</div>");
        let surfaces = Surfaces {
            pages: vec![registered("Custom", "/custom/", &html)],
            candidates: BTreeSet::from([candidate.into()]),
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        assert_eq!(
            analysis.candidate_urls.get("static/custom.svg"),
            Some(&Target::Resource("static/custom.svg".into()))
        );
        assert_eq!(analysis.pages[0].fragment.html, html);
        assert_eq!(surfaces.candidates, BTreeSet::from([candidate.into()]));
    }

    #[test]
    fn source_candidate_hidden_images_enter_the_resource_graph_without_hidden_dom() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("media/hidden.svg", "<svg/>"),
        ]);
        let surfaces = Surfaces {
            pages: vec![registered("Home", "/", "<h1>Home</h1>")],
            candidates: BTreeSet::from([
                "hover:bg-[url(media/hidden.svg)]".into(),
                "mask-[url(#future-id)]".into(),
                "bg-[url(data:image/png;base64,AA)]".into(),
                "before:content-['url(no-fetch.svg)']".into(),
            ]),
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        assert_eq!(
            analysis.candidate_urls,
            BTreeMap::from([
                (
                    "media/hidden.svg".into(),
                    Target::Resource("media/hidden.svg".into())
                ),
                ("#future-id".into(), Target::Keep),
                ("data:image/png;base64,AA".into(), Target::Keep),
            ])
        );
        assert_eq!(
            analysis.resources,
            BTreeSet::from(["media/hidden.svg".into()])
        );
        assert_eq!(analysis.outside["media/hidden.svg"].as_ref(), b"<svg/>");
        assert!(analysis.referenced.contains("media/hidden.svg"));
        assert!(!analysis.referenced.contains("no-fetch.svg"));
        assert_eq!(analysis.pages[0].fragment.html, "<h1>Home</h1>");
        assert!(analysis.pages[0].fragment.references.is_empty());
    }

    #[test]
    fn source_candidate_missing_private_and_remote_images_have_general_diagnostics() {
        let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), (".env", "secret")]);
        let surfaces = Surfaces {
            pages: vec![registered("Home", "/", "<h1>Home</h1>")],
            candidates: BTreeSet::from([
                "bg-[url(media/missing.svg)]".into(),
                "bg-[url(.env)]".into(),
                "bg-[url(https://cdn.example/remote.svg)]".into(),
            ]),
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert_eq!(analysis.errors(), 3, "{:?}", analysis.diagnostics);
        for (candidate, rule) in [
            ("bg-[url(media/missing.svg)]", "missing-image"),
            ("bg-[url(.env)]", "private-file"),
            (
                "bg-[url(https://cdn.example/remote.svg)]",
                "cross-origin-resource",
            ),
        ] {
            let diagnostic = analysis
                .diagnostics
                .iter()
                .find(|d| d.rule == rule)
                .unwrap();
            assert!(diagnostic.message.contains(candidate), "{diagnostic:?}");
            assert_eq!(
                (&diagnostic.file, diagnostic.line, diagnostic.column),
                (&None, None, None)
            );
        }
        assert!(analysis.resources.is_empty());
        assert!(!analysis.outside.contains_key(".env"));
    }

    #[test]
    fn source_candidate_theme_document_is_refused_even_when_not_in_the_initial_view() {
        let config = format!("{CONFIG}[theme]\ndocument='theme.json'\n");
        let fixture = Fixture::new(&[("rustify-docs.toml", &config), ("theme.json", "{}")]);
        let candidate = "bg-[url(theme.json)]";
        let surfaces = Surfaces {
            pages: vec![registered("Home", "/", "<h1>Home</h1>")],
            candidates: BTreeSet::from([candidate.into()]),
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        let diagnostic = analysis
            .diagnostics
            .iter()
            .find(|d| d.rule == "theme-document-resource")
            .unwrap_or_else(|| panic!("{:?}", analysis.diagnostics));
        assert!(diagnostic.message.contains(candidate));
        assert_eq!(
            (&diagnostic.file, diagnostic.line, diagnostic.column),
            (&None, None, None)
        );
        assert!(!analysis.resources.contains("theme.json"));
        assert!(!analysis.outside.contains_key("theme.json"));
    }

    #[test]
    fn registered_surfaces_can_form_a_site_without_markdown_or_description() {
        let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG)]);
        let html = "<!><h1 id=\"home\">Rendered Home</h1><template id=\"holder\"><h1 id=\"ghost\">Hidden</h1></template>";
        let surfaces = Surfaces {
            pages: vec![registered("Home", "/", html)],
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        assert_eq!(analysis.warnings(), 0, "{:?}", analysis.diagnostics);
        let page = &analysis.pages[analysis.routes["/"]];
        assert_eq!(page.origin, PageOrigin::Registered("Home".into()));
        assert_eq!(
            (&page.path, &page.file, &page.title),
            (&"/".into(), &"src/pages.rs".into(), &"Home".into())
        );
        assert_eq!(page.front.layout, Layout::Doc);
        assert_eq!(page.description, None);
        assert_eq!(page.fragment.html, html);
        assert_eq!(page.fragment.anchors, ["home", "holder"]);
        assert_eq!(page.fragment.h1_count, 1);
        assert!(analysis.nav.is_empty());
    }

    #[test]
    fn registered_surfaces_share_routes_slot_anchors_and_resources_with_markdown() {
        let config = "schema_version=1\n[site]\nname='Docs'\nbase='/docs/'\nurl='https://example.com/docs/'\n";
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", config),
            ("content/index.md", "---\ndescription: d\n---\n# Home\n\n[Custom](/custom/#custom) [Footer](/docs/custom/#footer) [Relative](custom/#custom)\n"),
            ("content/guide.md", "---\ndescription: d\n---\n# Guide\n\n## Section\n"),
            ("static/logo.svg", "<svg/>"),
            ("downloads/spec.pdf", "pdf"),
        ]);
        let surfaces = Surfaces {
            pages: vec![registered("Custom", "/custom/", "<!><h1 id='custom'>Custom</h1><a href='/docs/guide/#section'>Guide</a><a href='#footer'>Footer</a><img alt='' src='/docs/logo.svg'><a href='downloads/spec.pdf'>PDF</a>")],
            slots: BTreeMap::from([("/custom/".into(), BTreeMap::from([(Slot::Footer, "<footer id='footer'><a href='#custom'>Top</a><img alt='' src='static/logo.svg'></footer>".into())]))]),
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        let custom = analysis.routes["/custom/"];
        let guide = analysis.routes["/guide/"];
        assert_eq!(
            analysis.pages[analysis.routes["/"]].targets,
            [
                Target::Page {
                    page: custom,
                    anchor: Some("custom".into())
                },
                Target::Page {
                    page: custom,
                    anchor: Some("footer".into())
                },
                Target::Page {
                    page: custom,
                    anchor: Some("custom".into())
                },
            ]
        );
        assert_eq!(
            analysis.pages[custom].targets,
            [
                Target::Page {
                    page: guide,
                    anchor: Some("section".into())
                },
                Target::Keep,
                Target::Resource("static/logo.svg".into()),
                Target::Resource("downloads/spec.pdf".into()),
            ]
        );
        assert_eq!(
            analysis.slots["/custom/"][&Slot::Footer].targets,
            [Target::Keep, Target::Resource("static/logo.svg".into())]
        );
        assert_eq!(
            analysis.resources,
            BTreeSet::from(["downloads/spec.pdf".into(), "static/logo.svg".into()])
        );
        assert!(analysis.referenced.contains("downloads/spec.pdf"));
        assert!(analysis.pages[custom].fragment.html.starts_with("<!>"));
    }

    #[test]
    fn registered_surfaces_reject_duplicate_invalid_and_reserved_routes() {
        for (route, rule) in [
            ("/", "route-conflict"),
            ("/guide/", "route-conflict"),
            ("/404/", "reserved-route"),
            ("/robots.txt/", "reserved-route"),
            ("/assets/files/generated/", "reserved-route"),
            ("/.rustify-docs/cache/", "reserved-route"),
            ("/../custom/", "invalid-route"),
            ("/custom?x/", "invalid-route"),
            ("/custom", "invalid-route"),
        ] {
            let fixture = Fixture::new(&[
                ("rustify-docs.toml", CONFIG),
                ("content/index.md", &format!("{FRONT}# Home\n")),
                ("content/guide.md", &format!("{FRONT}# Guide\n")),
            ]);
            let surfaces = Surfaces {
                pages: vec![registered("Custom", route, "<h1>Custom</h1>")],
                ..Surfaces::default()
            };
            let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
            assert!(
                analysis.diagnostics.iter().any(|d| d.rule == rule),
                "{route}: {:?}",
                analysis.diagnostics
            );
        }
        let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG)]);
        let surfaces = Surfaces {
            pages: vec![
                registered("First", "/custom/", ""),
                registered("Second", "/custom/", ""),
            ],
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert!(analysis
            .diagnostics
            .iter()
            .any(|d| d.rule == "route-conflict"));
    }

    #[test]
    fn registered_surfaces_routes_navigation_keeps_declared_order() {
        let config = format!("{CONFIG}[[nav.groups]]\nlabel='Custom'\nroutes=['/b/', '/a/']\n[[nav.groups]]\nlabel='Guides'\ndir=''\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            ("content/guide.md", &format!("{FRONT}# Guide\n")),
        ]);
        let surfaces = Surfaces {
            pages: vec![registered("A", "/a/", ""), registered("B", "/b/", "")],
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        let routes: Vec<Vec<&str>> = analysis
            .nav
            .iter()
            .map(|group| group.items.iter().map(|item| item.route.as_str()).collect())
            .collect();
        assert_eq!(routes, [vec!["/b/", "/a/"], vec!["/guide/"]]);
        assert_eq!(analysis.nav[0].items[0].title, "B");
    }

    #[test]
    fn registered_surfaces_routes_navigation_rejects_unknown_duplicates_and_markdown() {
        for (routes, rule) in [
            ("['/missing/']", "nav-missing-page"),
            ("['/custom/', '/custom/']", "nav-duplicate-route"),
            ("['/guide/']", "nav-route-origin"),
        ] {
            let config = format!("{CONFIG}[[nav.groups]]\nlabel='Custom'\nroutes={routes}\n");
            let fixture = Fixture::new(&[
                ("rustify-docs.toml", &config),
                ("content/guide.md", &format!("{FRONT}# Guide\n")),
            ]);
            let surfaces = Surfaces {
                pages: vec![registered("Custom", "/custom/", "")],
                ..Surfaces::default()
            };
            let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
            let diagnostic = analysis
                .diagnostics
                .iter()
                .find(|d| d.rule == rule)
                .unwrap_or_else(|| panic!("{routes}: {:?}", analysis.diagnostics));
            assert_eq!(diagnostic.file.as_deref(), Some("rustify-docs.toml"));
            assert_eq!(diagnostic.line, Some(7));
        }
    }

    #[test]
    fn registered_surfaces_keep_raw_policy_and_private_resource_checks() {
        let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), (".env", "secret")]);
        let surfaces = Surfaces { pages: vec![registered("Unsafe", "/unsafe/", "<h1>Unsafe</h1><script>alert(1)</script><div style='color:red' onclick='x()'></div><img alt='' src='https://cdn.example/x.svg'><img alt='' src='.env'>")], ..Surfaces::default() };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        for rule in [
            "inline-script",
            "inline-style",
            "event-handler",
            "cross-origin-resource",
            "private-file",
        ] {
            let diagnostic = analysis
                .diagnostics
                .iter()
                .find(|d| d.rule == rule)
                .unwrap_or_else(|| panic!("{rule}: {:?}", analysis.diagnostics));
            assert_eq!(diagnostic.file.as_deref(), Some("src/pages.rs"));
            assert_eq!((diagnostic.line, diagnostic.column), (None, None));
        }
        assert!(!analysis.resources.contains(".env"));
    }

    #[test]
    fn registered_surfaces_slots_check_routes_but_keep_not_found_content() {
        let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG)]);
        let surfaces = Surfaces {
            pages: vec![registered("Home", "/", "<h1 id='home'>Home</h1>")],
            slots: BTreeMap::from([
                (
                    "/404/".into(),
                    BTreeMap::from([(
                        Slot::NotFound,
                        "<div id='lost'><a href='#lost'>Lost</a><a href='/#home'>Home</a></div>"
                            .into(),
                    )]),
                ),
                (
                    "/missing/".into(),
                    BTreeMap::from([(Slot::Footer, "<p>Missing</p>".into())]),
                ),
            ]),
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert_eq!(analysis.errors(), 1, "{:?}", analysis.diagnostics);
        assert!(analysis.diagnostics.iter().any(|d| d.rule == "slot-route"));
        assert!(!analysis.routes.contains_key("/404/"));
        assert_eq!(
            analysis.slots["/404/"][&Slot::NotFound].fragment.anchors,
            ["lost"]
        );
        assert_eq!(
            analysis.slots["/404/"][&Slot::NotFound].targets,
            [
                Target::Keep,
                Target::Page {
                    page: analysis.routes["/"],
                    anchor: Some("home".into())
                }
            ]
        );
    }

    #[test]
    fn registered_surfaces_reuse_raw_cache_but_recheck_page_and_slot_relationships() {
        let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG)]);
        let (project, _) = Project::load(&fixture.root).unwrap();
        let (snapshot, _) = snapshot(&project);
        let mut cache = FragmentCache::default();
        let highlighter = Highlighter::new();
        let mut surfaces = Surfaces {
            pages: vec![registered(
                "Custom",
                "/custom/",
                "<h1 id='title'>Title</h1><a href='#footer'>Footer</a>",
            )],
            slots: BTreeMap::from([(
                "/custom/".into(),
                BTreeMap::from([(Slot::Footer, "<div id='footer'>Footer</div>".into())]),
            )]),
            ..Surfaces::default()
        };
        let first = analyze_with_surfaces(
            &project,
            &snapshot,
            Mode::Production,
            &mut cache,
            &highlighter,
            1,
            &surfaces,
        );
        assert_eq!(first.errors(), 0, "{:?}", first.diagnostics);
        let second = analyze_with_surfaces(
            &project,
            &snapshot,
            Mode::Production,
            &mut cache,
            &highlighter,
            1,
            &surfaces,
        );
        assert!(Arc::ptr_eq(
            &first.pages[0].fragment,
            &second.pages[0].fragment
        ));
        assert_eq!((cache.hits, cache.misses, cache.len()), (2, 2, 2));
        surfaces.slots.clear();
        let third = analyze_with_surfaces(
            &project,
            &snapshot,
            Mode::Production,
            &mut cache,
            &highlighter,
            1,
            &surfaces,
        );
        assert!(third.diagnostics.iter().any(|d| d.rule == "broken-anchor"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn registered_surfaces_svg_uses_resolve_as_document_anchors() {
        let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG)]);
        let surfaces = Surfaces {
            pages: vec![registered(
                "Icons",
                "/icons/",
                "<svg><symbol id='icon'><path d='M0 0'/></symbol><use href='#icon'/></svg>",
            )],
            ..Surfaces::default()
        };
        let (_, _, analysis) = fixture.analyze_with_surfaces(Mode::Production, &surfaces);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        assert_eq!(analysis.pages[0].targets, [Target::Keep]);
        assert!(analysis.resources.is_empty());
    }

    #[test]
    fn theme_document_snapshot_reads_only_the_configured_file_and_tracks_it() {
        let config = format!("{CONFIG}[theme]\ndocument='themes/document.md'\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            ("content/index.md", &format!("{FRONT}# Home\n")),
            ("themes/document.md", "{}"),
            ("themes/other.md", "unrelated"),
        ]);
        let (_, snapshot, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            snapshot
                .files
                .get("themes/document.md")
                .map(|bytes| bytes.as_ref()),
            Some(b"{}".as_slice())
        );
        assert_eq!(snapshot.markdown.len(), 1);
        assert!(!snapshot.files.contains_key("themes/other.md"));
        assert!(analysis.referenced.contains("themes/document.md"));
        assert!(!analysis.resources.contains("themes/document.md"));
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    }

    #[test]
    fn theme_document_cannot_be_published_as_an_image_or_download() {
        let config = format!("{CONFIG}[theme]\ndocument='static/theme.json'\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            (
                "content/index.md",
                &format!("{FRONT}# Home\n\n![Theme](/theme.json) [Download](/theme.json)\n"),
            ),
            ("static/theme.json", "{}"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let refused: Vec<_> = analysis
            .diagnostics
            .iter()
            .filter(|d| d.rule == "theme-document-resource")
            .collect();
        assert_eq!(refused.len(), 2, "{:?}", analysis.diagnostics);
        assert!(refused
            .iter()
            .all(|d| d.file.as_deref() == Some("content/index.md") && d.line == Some(6)));
        assert!(!analysis.resources.contains("static/theme.json"));
    }

    #[test]
    fn routes_and_their_conflicts() {
        assert_eq!(route_for("index.md"), "/");
        assert_eq!(route_for("guide/index.md"), "/guide/");
        assert_eq!(route_for("guide/setup.md"), "/guide/setup/");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/a.md", "---\ndescription: d\n---\n# A\n"),
            (
                "content/a/index.md",
                "---\ndescription: d\n---\n# A again\n",
            ),
            ("content/404.md", "---\ndescription: d\n---\n# Lost\n"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let rules: Vec<String> = errors(&analysis)
            .into_iter()
            .map(|(rule, _, _)| rule)
            .collect();
        assert_eq!(rules, ["reserved-route", "route-conflict"]);
    }

    #[test]
    fn links_resolve_to_pages_and_anchors() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", &format!("{FRONT}# Home\n\nSee [setup](guide/setup.md#install), [missing](guide/nope.md), [bad anchor](guide/setup.md#nope), [self](#home) and [route](/guide/setup/).\n")),
            ("content/guide/setup.md", &format!("{FRONT}# Setup\n\n## Install\n\nBack [home](../index.md) and [up](../).\n")),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            errors(&analysis),
            [
                ("broken-link".to_owned(), "content/index.md".to_owned(), 6),
                ("broken-anchor".to_owned(), "content/index.md".to_owned(), 6)
            ]
        );
        let home = &analysis.pages[analysis.routes["/"]];
        let setup = analysis.routes["/guide/setup/"];
        assert_eq!(
            home.targets[0],
            Target::Page {
                page: setup,
                anchor: Some("install".into())
            }
        );
        assert_eq!(home.targets[3], Target::Keep);
        assert_eq!(
            home.targets[4],
            Target::Page {
                page: setup,
                anchor: None
            }
        );
        let back = &analysis.pages[setup];
        let home = analysis.routes["/"];
        assert_eq!(
            back.targets,
            vec![
                Target::Page {
                    page: home,
                    anchor: None
                },
                Target::Page {
                    page: home,
                    anchor: None
                }
            ]
        );
    }

    #[test]
    fn a_site_without_its_url_is_warned_and_static_files_keep_clear_of_the_build() {
        let fixture = Fixture::new(&[
            (
                "rustify-docs.toml",
                "schema_version = 1\n[site]\nname = \"Docs\"\n",
            ),
            ("content/index.md", &format!("{FRONT}# Home\n")),
            ("content/guide.md", &format!("{FRONT}# Guide\n")),
            ("static/robots.txt", "User-agent: *\n"),
            ("static/guide/index.html", "<p>shadow</p>"),
            ("static/assets/files/x.png", "png"),
            ("static/assets/other.png", "png"),
            ("static/favicon.ico", "ico"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let rules: Vec<(String, String)> = analysis
            .diagnostics
            .iter()
            .map(|d| (d.rule.clone(), d.file.clone().unwrap_or_default()))
            .collect();
        assert_eq!(
            rules,
            [
                (
                    "missing-site-url".to_owned(),
                    "rustify-docs.toml".to_owned()
                ),
                (
                    "output-conflict".to_owned(),
                    "static/assets/files/x.png".to_owned()
                ),
                (
                    "output-conflict".to_owned(),
                    "static/guide/index.html".to_owned()
                ),
                ("output-conflict".to_owned(), "static/robots.txt".to_owned()),
            ]
        );
        assert!(analysis.diagnostics[0]
            .suggestion
            .as_deref()
            .unwrap()
            .contains("url = "));
    }

    #[test]
    fn home_actions_and_header_links_resolve_like_content_links() {
        let config = format!(
            "{CONFIG}[[nav.top]]\nlabel = \"Guide\"\nlink = \"guide/setup.md\"\n[[nav.top]]\nlabel = \"Lost\"\nlink = \"/nowhere/\"\n[[nav.social]]\nlabel = \"GitHub\"\nlink = \"https://github.com/example\"\n"
        );
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            (
                "content/index.md",
                "---\nlayout: home\nactions:\n  - text: Start\n    link: guide/setup.md#install\n  - text: Soon\n    link: soon.md\n  - text: Gone\n    link: guide/gone.md\n---\n# Home\n",
            ),
            ("content/guide/setup.md", &format!("{FRONT}# Setup\n\n## Install\n")),
            ("content/soon.md", "---\ndescription: d\ndraft: true\n---\n# Soon\n"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let setup = analysis.routes["/guide/setup/"];
        let home = &analysis.pages[analysis.routes["/"]];
        assert_eq!(
            home.action_targets,
            [
                Target::Page {
                    page: setup,
                    anchor: Some("install".into())
                },
                Target::Page {
                    page: analysis.routes["/soon/"],
                    anchor: None
                },
                Target::Keep,
            ]
        );
        assert_eq!(
            analysis.top,
            [
                Target::Page {
                    page: setup,
                    anchor: None
                },
                Target::Keep
            ]
        );
        assert_eq!(analysis.social, [Target::Keep]);
        let located: Vec<(String, String, usize, usize)> = analysis
            .diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .map(|d| {
                (
                    d.rule.clone(),
                    d.file.clone().unwrap_or_default(),
                    d.line.unwrap_or(0),
                    d.column.unwrap_or(0),
                )
            })
            .collect();
        assert_eq!(
            located,
            [
                (
                    "draft-link".to_owned(),
                    "content/index.md".to_owned(),
                    7,
                    11
                ),
                (
                    "broken-link".to_owned(),
                    "content/index.md".to_owned(),
                    9,
                    11
                ),
                (
                    "broken-link".to_owned(),
                    "rustify-docs.toml".to_owned(),
                    10,
                    8
                ),
            ]
        );
    }

    #[test]
    fn drafts_are_left_out_and_links_to_them_depend_on_the_mode() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                &format!("{FRONT}# Home\n\n[Soon](soon.md)\n"),
            ),
            (
                "content/soon.md",
                "---\ndescription: d\ndraft: true\n---\n# Soon\n",
            ),
        ]);
        let (_, _, production) = fixture.analyze(Mode::Production);
        assert_eq!(
            errors(&production),
            [("draft-link".to_owned(), "content/index.md".to_owned(), 6)]
        );
        assert_eq!(production.published().count(), 1);
        assert!(production
            .nav
            .iter()
            .all(|group| group.items.iter().all(|item| !item.draft)));
        let (_, _, dev) = fixture.analyze(Mode::Dev);
        assert!(errors(&dev).is_empty());
        assert!(dev
            .diagnostics
            .iter()
            .any(|d| d.rule == "draft-link" && d.severity == Severity::Warning));
        assert_eq!(dev.published().count(), 2);
        assert!(dev.nav[0].items.iter().any(|item| item.draft));
    }

    #[test]
    fn files_outside_the_content_need_the_repository() {
        let files = [
            ("content/index.md", "---\ndescription: d\n---\n# Home\n\nSee [the source](../src/lib.rs) and [gone](../src/gone.rs).\n"),
            ("src/lib.rs", "pub fn x() {}\n"),
        ];
        let fixture = Fixture::new(&[&[("rustify-docs.toml", CONFIG)][..], &files[..]].concat());
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let rules: Vec<String> = errors(&analysis)
            .into_iter()
            .map(|(rule, _, _)| rule)
            .collect();
        assert_eq!(rules, ["repo-link", "broken-link"]);
        let config = format!(
            "{CONFIG}[repo]\nurl = \"https://github.com/o/r\"\nref = \"main\"\npath = \"site\"\n"
        );
        let fixture =
            Fixture::new(&[&[("rustify-docs.toml", config.as_str())][..], &files[..]].concat());
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(errors(&analysis).len(), 1);
        assert_eq!(
            analysis.pages[0].targets[0],
            Target::Repository("https://github.com/o/r/blob/main/site/src/lib.rs".into())
        );
    }

    #[test]
    fn images_must_exist_and_stay_local() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", "---\ndescription: d\n---\n# Home\n\n![here](pic.png) ![static](/logo.svg) ![gone](gone.png) ![far](https://cdn.example/x.png) ![inline](data:image/png;base64,AA)\n"),
            ("content/pic.png", "png"),
            ("static/logo.svg", "<svg/>"),
            ("static/.env", "SECRET=1"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let rules: Vec<String> = errors(&analysis)
            .into_iter()
            .map(|(rule, _, _)| rule)
            .collect();
        // In the order they are written on the line.
        assert_eq!(rules, ["missing-image", "cross-origin-resource"]);
        assert_eq!(
            analysis.resources.iter().collect::<Vec<_>>(),
            ["content/pic.png", "static/logo.svg"]
        );
    }

    #[test]
    fn includes_read_the_project_and_nothing_outside_it() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", "---\ndescription: d\n---\n# Home\n\n<!-- rustify:include path=\"src/a.rs\" region=\"r\" -->\n\n<!-- rustify:include path=\"../outside.rs\" -->\n\n<!-- rustify:include path=\".env\" -->\n"),
            ("src/a.rs", "// rustify-region:r\nfn r() {}\n// rustify-endregion:r\n"),
            (".env", "SECRET=1"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let rules: Vec<String> = errors(&analysis)
            .into_iter()
            .map(|(rule, _, _)| rule)
            .collect();
        assert_eq!(rules, ["include-path", "include-missing"]);
        assert_eq!(
            analysis.pages[0].fragment.code_blocks[0].text,
            "fn r() {}\n"
        );
    }

    #[test]
    fn titles_h1_and_descriptions() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/a.md", "Just text.\n"),
            ("content/b.md", "---\ntitle: B\n---\n```sh\nx\n```\n"),
            (
                "content/c.md",
                "---\ndescription: d\n---\n# One\n\n# Two\n\n#### Deep\n",
            ),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            found(&analysis),
            [
                ("missing-title".to_owned(), "content/a.md".to_owned(), 0),
                (
                    "missing-description".to_owned(),
                    "content/a.md".to_owned(),
                    0
                ),
                (
                    "missing-description".to_owned(),
                    "content/b.md".to_owned(),
                    0
                ),
                ("multiple-h1".to_owned(), "content/c.md".to_owned(), 6),
                ("heading-skip".to_owned(), "content/c.md".to_owned(), 8),
            ]
        );
        assert_eq!(analysis.pages[2].title, "One");
    }

    #[test]
    fn raw_and_markdown_headings_share_visible_document_semantics() {
        for body in [
            "# One\n\n<H1>Two</H1>\n\n<h4>Deep</h4>\n",
            "<h1>One</h1>\n\n# Two\n\n#### Deep\n",
            "<div><h1>One</h1><h1>Two</h1><h4>Deep</h4></div>\n",
            "# One\n\n<svg><foreignObject><h1>Two</h1><h4>Deep</h4></foreignObject></svg>\n",
        ] {
            let source = format!("---\ndescription: d\n---\n{body}");
            let fixture =
                Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", &source)]);
            let (_, _, analysis) = fixture.analyze(Mode::Production);
            let rules: Vec<_> = analysis
                .diagnostics
                .iter()
                .map(|d| d.rule.as_str())
                .collect();
            assert_eq!(
                rules,
                ["multiple-h1", "heading-skip"],
                "{source}: {:#?}",
                analysis.diagnostics
            );
            for problem in &analysis.diagnostics {
                assert!(problem.line.is_some() && problem.column.is_some());
                assert!(problem.suggestion.is_some());
            }
        }
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", "---\ndescription: d\n---\n# One\n\n<template><h1>Hidden</h1><h4>Hidden</h4></template>\n\n<!-- <h1>Comment</h1> -->\n\n```html\n<h1>Code</h1>\n```\n\n## Visible\n"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert!(
            analysis.diagnostics.is_empty(),
            "{:#?}",
            analysis.diagnostics
        );
    }

    #[test]
    fn home_hero_rejects_h1s_outside_the_existing_leading_rule() {
        for body in [
            "Intro.\n\n# Author\n",
            "<!-- comment -->\n\n# Author\n",
            "<div><h1>Author</h1></div>\n",
            "<H1>Author</H1>\n",
        ] {
            let source = format!("---\ntitle: Home\nlayout: home\n---\n{body}");
            let fixture =
                Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", &source)]);
            let (_, _, analysis) = fixture.analyze(Mode::Production);
            let problem = analysis
                .diagnostics
                .iter()
                .find(|d| d.rule == "multiple-h1")
                .unwrap_or_else(|| panic!("{source}: {:#?}", analysis.diagnostics));
            assert_eq!(
                problem.line,
                Some(if body.starts_with("Intro") || body.starts_with("<!--") {
                    7
                } else {
                    5
                })
            );
            assert_eq!(
                problem.column,
                Some(if body.starts_with("<div>") { 6 } else { 1 })
            );
            assert!(problem.suggestion.as_deref().unwrap().contains("H2"));
        }
        for body in [
            "",
            "# Author\n\n## Next\n",
            "<h1 id=\"author\">Author</h1>\n",
            "<template><h1>Hidden</h1></template>\n",
            "<!-- <h1>Hidden</h1> -->\n",
        ] {
            let source = format!("---\ntitle: Home\nlayout: home\n---\n{body}");
            let fixture =
                Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", &source)]);
            let (_, _, analysis) = fixture.analyze(Mode::Production);
            assert!(
                analysis.diagnostics.is_empty(),
                "{source}: {:#?}",
                analysis.diagnostics
            );
        }
    }

    #[test]
    fn home_links_cannot_target_markup_discarded_with_the_leading_h1() {
        for layout in ["home", "doc"] {
            let source = format!("---\ntitle: Home\ndescription: d\nlayout: {layout}\n---\n# Home <span id=\"inside\">title</span>\n\n[Inside](#inside)\n");
            let fixture =
                Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", &source)]);
            let (_, _, analysis) = fixture.analyze(Mode::Production);
            assert_eq!(
                analysis.errors(),
                usize::from(layout == "home"),
                "{:#?}",
                analysis.diagnostics
            );
            if layout == "home" {
                assert_eq!(analysis.diagnostics[0].rule, "broken-anchor");
            }
        }
    }

    #[test]
    fn frontmatter_errors_are_located_and_the_rest_still_analysed() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/a.md",
                "---\ntitle: A\ncolour: red\n---\n# A\n\n[x](nope.md)\n",
            ),
            ("content/b.md", "---\ntitle: B\n"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let errors = errors(&analysis);
        assert!(
            errors.contains(&("frontmatter".to_owned(), "content/a.md".to_owned(), 3)),
            "{errors:?}"
        );
        assert!(errors
            .iter()
            .any(|(rule, file, _)| rule == "broken-link" && file == "content/a.md"));
        assert!(errors
            .iter()
            .any(|(rule, file, _)| rule == "frontmatter-unclosed" && file == "content/b.md"));
    }

    #[test]
    fn navigation_from_directories_and_lists() {
        let config = format!("{CONFIG}[[nav.groups]]\nlabel = \"Guide\"\ndir = \"guide\"\n[[nav.groups]]\nlabel = \"Picked\"\nitems = [\"extra.md\", \"nope.md\"]\nscope = \"/guide/\"\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", config.as_str()),
            (
                "content/index.md",
                "---\ndescription: d\nlayout: home\n---\n# Home\n",
            ),
            (
                "content/guide/b.md",
                "---\ndescription: d\nsidebar:\n  order: 1\n---\n# B\n",
            ),
            (
                "content/guide/a.md",
                "---\ndescription: d\nsidebar:\n  order: 2\n  label: Alpha\n---\n# A\n",
            ),
            ("content/extra.md", "---\ndescription: d\n---\n# Extra\n"),
            ("content/lonely.md", "---\ndescription: d\n---\n# Lonely\n"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let titles: Vec<Vec<&str>> = analysis
            .nav
            .iter()
            .map(|g| g.items.iter().map(|i| i.title.as_str()).collect())
            .collect();
        assert_eq!(titles, [vec!["B", "Alpha"], vec!["Extra"]]);
        assert_eq!(analysis.nav[1].prefix, "/guide/");
        let rules: Vec<(String, String)> = found(&analysis)
            .into_iter()
            .map(|(rule, file, _)| (rule, file))
            .collect();
        assert_eq!(
            rules,
            [
                ("not-in-nav".to_owned(), "content/lonely.md".to_owned()),
                (
                    "nav-missing-page".to_owned(),
                    "rustify-docs.toml".to_owned()
                ),
            ]
        );
    }

    #[test]
    fn sidebar_group_moves_a_page_and_names_groups_without_configuration() {
        let config = format!("{CONFIG}[[nav.groups]]\nlabel = \"Guide\"\ndir = \"guide\"\n[[nav.groups]]\nlabel = \"Reference\"\nitems = [\"api.md\"]\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", config.as_str()),
            ("content/guide/a.md", "---\ndescription: d\n---\n# A\n"),
            (
                "content/guide/moved.md",
                "---\ndescription: d\nsidebar:\n  group: Reference\n  order: 1\n---\n# Moved\n",
            ),
            ("content/api.md", "---\ndescription: d\n---\n# API\n"),
            (
                "content/joined.md",
                "---\ndescription: d\nsidebar:\n  group: Guide\n  order: 0\n---\n# Joined\n",
            ),
            (
                "content/typo.md",
                "---\ndescription: d\nsidebar:\n  group: Refrence\n---\n# Typo\n",
            ),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let titles: Vec<Vec<&str>> = analysis
            .nav
            .iter()
            .map(|g| g.items.iter().map(|i| i.title.as_str()).collect())
            .collect();
        assert_eq!(titles, [vec!["Joined", "A"], vec!["API", "Moved"]]);
        let located: Vec<(String, String, usize, usize)> = analysis
            .diagnostics
            .iter()
            .map(|d| {
                (
                    d.rule.clone(),
                    d.file.clone().unwrap_or_default(),
                    d.line.unwrap_or(0),
                    d.column.unwrap_or(0),
                )
            })
            .collect();
        assert_eq!(
            located,
            [
                ("not-in-nav".to_owned(), "content/typo.md".to_owned(), 0, 0),
                (
                    "nav-unknown-group".to_owned(),
                    "content/typo.md".to_owned(),
                    4,
                    10
                ),
            ]
        );

        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/z.md",
                "---\ndescription: d\nsidebar:\n  order: 1\n---\n# Z\n",
            ),
            ("content/a.md", "---\ndescription: d\n---\n# A\n"),
            (
                "content/more/b.md",
                "---\ndescription: d\nsidebar:\n  group: More\n---\n# B\n",
            ),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let groups: Vec<(&str, Vec<&str>)> = analysis
            .nav
            .iter()
            .map(|g| {
                (
                    g.label.as_str(),
                    g.items.iter().map(|i| i.title.as_str()).collect(),
                )
            })
            .collect();
        assert_eq!(groups, [("Docs", vec!["Z", "A"]), ("More", vec!["B"])]);
        assert!(
            analysis.diagnostics.is_empty(),
            "{:#?}",
            analysis.diagnostics
        );
    }

    #[test]
    fn code_groups_of_one_name_share_their_tabs_across_the_site() {
        let group = |labels: [&str; 2]| {
            format!("---\ndescription: d\n---\n# P\n\n<!-- rustify:code-group name=\"install\" -->\n\n```sh title=\"{}\"\na\n```\n\n```sh title=\"{}\"\nb\n```\n\n<!-- rustify:end-code-group -->\n", labels[0], labels[1])
        };
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/a.md", &group(["Cargo", "Manifest"])),
            ("content/b.md", &group(["Manifest", "Cargo"])),
            ("content/c.md", &group(["Cargo", "Npm"])),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            errors(&analysis),
            [(
                "code-group-conflict".to_owned(),
                "content/c.md".to_owned(),
                6
            )]
        );
    }

    #[test]
    fn logo_and_brand_contrast() {
        let config = format!("{CONFIG}[theme]\nlogo = \"static/logo.svg\"\nprimary = {{ light = \"#ffd400\", dark = \"#1b1b6b\" }}\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", config.as_str()),
            ("content/index.md", &format!("{FRONT}# H\n")),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let rules: Vec<(String, Severity)> = analysis
            .diagnostics
            .iter()
            .map(|d| (d.rule.clone(), d.severity))
            .collect();
        assert_eq!(
            rules,
            [
                ("logo-missing".to_owned(), Severity::Error),
                ("brand-contrast".to_owned(), Severity::Warning),
                ("brand-contrast".to_owned(), Severity::Warning),
            ]
        );
    }

    /// A directory beside a fixture, outside its project; removed when
    /// dropped.
    #[cfg(unix)]
    struct Outside(PathBuf);

    #[cfg(unix)]
    impl Outside {
        fn new(fixture: &Fixture, files: &[(&str, &str)]) -> Self {
            let dir = fixture.root.with_extension("outside");
            let _ = std::fs::remove_dir_all(&dir);
            for (path, text) in files {
                let path = dir.join(path);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, text).unwrap();
            }
            Self(dir)
        }
    }

    #[cfg(unix)]
    impl Drop for Outside {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[cfg(unix)]
    fn link(target: &Path, at: &Path) {
        std::fs::create_dir_all(at.parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(target, at).unwrap();
    }

    /// Whether any input the analysis read, or would publish, holds `marker`.
    fn holds(snapshot: &Snapshot, analysis: &Analysis, marker: &str) -> bool {
        let found = |bytes: &[u8]| bytes.windows(marker.len()).any(|w| w == marker.as_bytes());
        snapshot.files.values().any(|bytes| found(bytes))
            || snapshot
                .markdown
                .values()
                .any(|text| found(text.as_bytes()))
            || analysis.outside.values().any(|bytes| found(bytes))
            || analysis
                .pages
                .iter()
                .any(|page| found(page.fragment.html.as_bytes()))
    }

    #[cfg(unix)]
    #[test]
    fn nothing_is_read_through_a_link_out_of_the_project() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                "---\ndescription: d\n---\n# Home\n\n![far](../images/far.svg) ![near](../brand/near.svg) ![shared](shared/near.svg)\n",
            ),
            ("brand/near.svg", "<svg/>"),
        ]);
        let outside = Outside::new(
            &fixture,
            &[
                ("static/x.txt", "OUTSIDE"),
                ("far.svg", "<svg>OUTSIDE</svg>"),
            ],
        );
        // The asset directory itself, and a directory on an image's path.
        link(&outside.0.join("static"), &fixture.root.join("static"));
        link(&outside.0, &fixture.root.join("images"));
        // A link that stays inside the project is followed.
        link(
            &fixture.root.join("brand"),
            &fixture.root.join("content/shared"),
        );
        let (_, snapshot, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            errors(&analysis),
            [
                (
                    "symlink-outside".to_owned(),
                    "content/index.md".to_owned(),
                    6
                ),
                ("symlink-outside".to_owned(), "static".to_owned(), 0),
            ]
        );
        assert!(!holds(&snapshot, &analysis, "OUTSIDE"));
        assert_eq!(
            analysis.resources.iter().collect::<Vec<_>>(),
            ["brand/near.svg", "content/shared/near.svg"]
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_private_file_stays_private_under_any_name() {
        let config = format!("{CONFIG}[theme]\nlogo = \".env\"\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", config.as_str()),
            (
                "content/index.md",
                "---\ndescription: d\n---\n# Home\n\n![alias](alias.svg)\n\n<!-- rustify:include path=\"notes.txt\" -->\n",
            ),
            (".env", "SECRET=PRIVATE"),
            ("static/.DS_Store", "PRIVATE"),
        ]);
        link(
            &fixture.root.join(".env"),
            &fixture.root.join("static/public.txt"),
        );
        link(
            &fixture.root.join(".env"),
            &fixture.root.join("content/alias.svg"),
        );
        link(&fixture.root.join(".env"), &fixture.root.join("notes.txt"));
        let (_, snapshot, analysis) = fixture.analyze(Mode::Production);
        let mut rules: Vec<(String, String)> = errors(&analysis)
            .into_iter()
            .map(|(rule, file, _)| (file, rule))
            .collect();
        rules.sort();
        assert_eq!(
            rules,
            [
                ("content/alias.svg".to_owned(), "private-file".to_owned()),
                ("content/index.md".to_owned(), "include-missing".to_owned()),
                ("content/index.md".to_owned(), "private-file".to_owned()),
                ("rustify-docs.toml".to_owned(), "private-file".to_owned()),
                ("static/public.txt".to_owned(), "private-file".to_owned()),
            ]
        );
        assert!(!holds(&snapshot, &analysis, "PRIVATE"));
        assert!(analysis.resources.is_empty(), "{:?}", analysis.resources);
    }

    #[test]
    fn private_markdown_is_never_a_page() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", &format!("{FRONT}# Home\n")),
            ("content/.env.md", &format!("{FRONT}# PRIVATE\n")),
            ("content/.env/notes.md", &format!("{FRONT}# PRIVATE\n")),
            ("content/guide/local.env", "PRIVATE"),
        ]);
        let (_, snapshot, analysis) = fixture.analyze(Mode::Production);
        assert!(errors(&analysis).is_empty(), "{:?}", analysis.diagnostics);
        assert_eq!(
            analysis
                .pages
                .iter()
                .map(|p| p.route.as_str())
                .collect::<Vec<_>>(),
            ["/"]
        );
        assert!(!holds(&snapshot, &analysis, "PRIVATE"));
        assert!(analysis
            .nav
            .iter()
            .all(|group| group.items.iter().all(|item| item.route == "/")));
    }

    #[test]
    fn files_only_actions_and_header_links_name_are_published() {
        let config = format!(
            "{CONFIG}[[nav.top]]\nlabel = \"Spec\"\nlink = \"/spec.pdf\"\n[[nav.social]]\nlabel = \"Feed\"\nlink = \"feed.xml\"\n"
        );
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", config.as_str()),
            (
                "content/index.md",
                "---\nlayout: home\nactions:\n  - text: Manual\n    link: manual.pdf#page=2\n---\n# Home\n",
            ),
            ("content/manual.pdf", "%PDF"),
            ("content/feed.xml", "<feed/>"),
            ("static/spec.pdf", "%PDF"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert!(errors(&analysis).is_empty(), "{:?}", analysis.diagnostics);
        assert_eq!(
            analysis.resources.iter().collect::<Vec<_>>(),
            ["content/feed.xml", "content/manual.pdf", "static/spec.pdf"]
        );
    }

    #[test]
    fn a_site_with_nothing_to_publish_is_an_error() {
        let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), ("static/logo.svg", "<svg/>")]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            errors(&analysis),
            [("no-pages".to_owned(), "content".to_owned(), 0)]
        );
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/soon.md",
                "---\ndescription: d\ndraft: true\n---\n# Soon\n",
            ),
        ]);
        let (_, _, production) = fixture.analyze(Mode::Production);
        assert_eq!(
            errors(&production),
            [("no-pages".to_owned(), "content".to_owned(), 0)]
        );
        // dev shows drafts, so a draft is something to show.
        let (_, _, dev) = fixture.analyze(Mode::Dev);
        assert!(errors(&dev).is_empty(), "{:?}", dev.diagnostics);
    }

    #[test]
    fn footnote_anchors_are_the_ids_the_page_has() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                &format!("{FRONT}# Home\n\nA claim[^n] and [its note](#fn-n), [back](#fnref-n), [there](other.md#fn-o), [lost](#fn-never-defined), [gone](other.md#fnref-x).\n\n[^n]: The note.\n"),
            ),
            (
                "content/other.md",
                &format!("{FRONT}# Other\n\nTwice[^o] over[^o].\n\n[^o]: Its note.\n"),
            ),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            errors(&analysis),
            [
                ("broken-anchor".to_owned(), "content/index.md".to_owned(), 6),
                ("broken-anchor".to_owned(), "content/index.md".to_owned(), 6),
            ]
        );
        let other = &analysis.pages[analysis.routes["/other/"]];
        assert_eq!(
            other.fragment.anchors,
            ["other", "fnref-o", "fnref-o-2", "fn-o"]
        );
    }

    #[test]
    fn links_cannot_target_references_in_an_unrendered_footnote() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", &format!("{FRONT}# Home\n\n[Absent](#fnref-b) and [there](other.md#fnref-b).\n\n[^unused]: Never shown, with a note[^b].\n\n[^b]: Its note.\n")),
            ("content/other.md", &format!("{FRONT}# Other\n\n[^unused]: Never shown, with a note[^b].\n\n[^b]: Its note.\n")),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            errors(&analysis)
                .iter()
                .filter(|(rule, _, _)| rule == "broken-anchor")
                .count(),
            2,
            "{:?}",
            analysis.diagnostics
        );
    }

    #[test]
    fn links_only_target_ids_in_the_document_not_template_contents() {
        let body = format!("{FRONT}# Home\n\n<template id=\"holder\"><span id=\"ghost\">Hidden</span><template id=\"nested\"><b id=\"deep\">Hidden</b></template></template>\n\n<p id=\"visible\">Visible</p>\n\n[Holder](#holder) [Visible](#visible) [Other holder](other.md#holder) [Other visible](other.md#visible)\n\n[Ghost](#ghost) [Nested](#nested) [Deep](other.md#deep) [Other ghost](other.md#ghost)\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", &body),
            ("content/other.md", &body),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let errors = errors(&analysis);
        assert_eq!(errors.len(), 8, "{errors:?}");
        assert!(errors.iter().all(|error| error.0 == "broken-anchor"));
        for page in &analysis.pages {
            assert_eq!(page.fragment.anchors, ["home", "holder", "visible"]);
        }
    }

    #[test]
    fn body_anchors_resolve_on_the_same_and_other_pages() {
        let body = format!("{FRONT}# Body anchor\n\n<body id=\"target\">\n\nText\n\n</body>\n\n[Target](#target) [Other target](other.md#target)\n");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", &body),
            ("content/other.md", &body),
        ]);
        for mode in [Mode::Production, Mode::Dev] {
            let (_, _, analysis) = fixture.analyze(mode);
            assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
            for page in &analysis.pages {
                assert_eq!(page.fragment.anchors, ["target", "body-anchor"]);
            }
        }
    }

    #[test]
    fn foreign_template_links_match_document_ids_on_the_same_and_other_pages() {
        for (markup, targets) in [
            (r#"<svg><template/></svg>"#, vec![]),
            (
                r#"<svg><template id="svg-holder"><circle id="circle" r="10"/></template></svg>"#,
                vec!["svg-holder", "circle"],
            ),
            (
                r#"<template id="holder"><svg><template/></svg></template>"#,
                vec!["holder"],
            ),
            (
                r#"<svg><foreignObject><template id="holder"><span id="ghost">Hidden</span><svg><template/></svg></template></foreignObject></svg>"#,
                vec!["holder"],
            ),
        ] {
            let mut body = format!("{FRONT}# Home\n\n{markup}\n\n## Visible\n\n[Visible](#visible) [Other visible](other.md#visible)\n");
            for id in &targets {
                body.push_str(&format!("\n[{id}](#{id}) [Other {id}](other.md#{id})\n"));
            }
            let fixture = Fixture::new(&[
                ("rustify-docs.toml", CONFIG),
                ("content/index.md", &body),
                ("content/other.md", &body),
            ]);
            for mode in [Mode::Production, Mode::Dev] {
                let (_, _, analysis) = fixture.analyze(mode);
                assert_eq!(analysis.errors(), 0, "{markup}: {:?}", analysis.diagnostics);
                let mut expected = vec!["home"];
                expected.extend(targets.iter().copied());
                expected.push("visible");
                for page in &analysis.pages {
                    assert_eq!(page.fragment.anchors, expected, "{markup}");
                }
            }
        }
    }

    #[test]
    fn html_integration_templates_reject_same_and_cross_page_ghost_links() {
        for (open, close) in [
            ("<svg><foreignObject>", "</foreignObject></svg>"),
            ("<svg><desc>", "</desc></svg>"),
            ("<svg><title>", "</title></svg>"),
            ("<math><mtext>", "</mtext></math>"),
            (
                "<math><annotation-xml encoding='application/xhtml+xml'>",
                "</annotation-xml></math>",
            ),
        ] {
            let body = format!("{FRONT}# Home\n\n{open}<template id=\"holder\"><svg><template id=\"nested\"><circle id=\"ghost\"/></template></svg></template>{close}\n\n## Visible\n\n[Holder](#holder) [Visible](#visible)\n\n[Ghost](#ghost) [Other ghost](other.md#ghost)\n");
            let fixture = Fixture::new(&[
                ("rustify-docs.toml", CONFIG),
                ("content/index.md", &body),
                ("content/other.md", &body),
            ]);
            let (_, _, analysis) = fixture.analyze(Mode::Production);
            let errors = errors(&analysis);
            assert_eq!(errors.len(), 4, "{open}: {errors:?}");
            assert!(errors.iter().all(|error| error.0 == "broken-anchor"));
            for page in &analysis.pages {
                assert_eq!(
                    page.fragment.anchors,
                    ["home", "holder", "visible"],
                    "{open}"
                );
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn snapshot_links_keep_safe_recovery_targets_when_missing() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                &format!("{FRONT}# Home\n\n[Download](manual.pdf) [Static](/manual.pdf)\n"),
            ),
            ("brand/real.pdf", "original attachment"),
        ]);
        for alias in ["content/manual.pdf", "static/manual.pdf"] {
            link(
                &fixture.root.join("brand/real.pdf"),
                &fixture.root.join(alias),
            );
        }
        let (project, initial, analysis) = fixture.analyze(Mode::Dev);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        assert!(initial.linked.contains("brand/real.pdf"));
        std::fs::remove_file(fixture.root.join("brand/real.pdf")).unwrap();
        let (_, missing, failed) = fixture.analyze(Mode::Dev);
        assert_eq!(failed.errors(), 2, "{:?}", failed.diagnostics);
        assert!(
            missing.linked.contains("brand/real.pdf"),
            "{:?}",
            missing.linked
        );
        assert!(!missing.files.contains_key("content/manual.pdf"));
        std::fs::write(fixture.root.join("brand/real.pdf"), "restored attachment").unwrap();
        let (_, restored, analysis) = fixture.analyze(Mode::Dev);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        assert_eq!(
            &*restored.files["content/manual.pdf"],
            b"restored attachment"
        );

        // A dependency is not permission to read a private or external target.
        link(
            &fixture.root.join(".env"),
            &fixture.root.join("content/private.pdf"),
        );
        let outside = Outside::new(&fixture, &[]);
        std::fs::create_dir_all(&outside.0).unwrap();
        link(
            &outside.0.join("missing.pdf"),
            &fixture.root.join("static/outside.pdf"),
        );
        let (missing, _) = snapshot(&project);
        assert_eq!(
            missing.linked,
            BTreeSet::from(["brand/real.pdf".to_owned()])
        );
    }

    #[cfg(unix)]
    #[test]
    fn dependencies_include_real_resource_brand_and_include_targets_when_missing() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", "schema_version = 1\n[site]\nname = 'D'\nurl = 'https://example.com/'\n[theme]\nlogo = 'images/logo.svg'\n"),
            ("content/index.md", &format!("{FRONT}# Home\n\n![Mark](../images/mark.svg)\n\n<!-- rustify:include path=\"src/example.rs\" region=\"main\" lang=\"rust\" -->\n")),
            ("brand/real.svg", "<svg xmlns='http://www.w3.org/2000/svg'/>") ,
            ("author/real.rs", "// rustify-region:main\nfn main() {}\n// rustify-endregion:main\n"),
        ]);
        for alias in ["images/mark.svg", "images/logo.svg"] {
            link(
                &fixture.root.join("brand/real.svg"),
                &fixture.root.join(alias),
            );
        }
        link(
            &fixture.root.join("author/real.rs"),
            &fixture.root.join("src/example.rs"),
        );
        let (_, _, analysis) = fixture.analyze(Mode::Dev);
        assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
        let expected = [
            "images/mark.svg",
            "images/logo.svg",
            "brand/real.svg",
            "src/example.rs",
            "author/real.rs",
        ];
        for path in expected {
            assert!(
                analysis.referenced.contains(path),
                "{path}: {:?}",
                analysis.referenced
            );
        }
        std::fs::remove_file(fixture.root.join("brand/real.svg")).unwrap();
        std::fs::remove_file(fixture.root.join("author/real.rs")).unwrap();
        let (_, _, failed) = fixture.analyze(Mode::Dev);
        assert!(failed.errors() > 0);
        for path in expected {
            assert!(
                failed.referenced.contains(path),
                "missing {path}: {:?}",
                failed.referenced
            );
        }
    }

    #[test]
    fn page_languages() {
        let fixture = Fixture::new(&[
            (
                "rustify-docs.toml",
                "schema_version = 1\n[site]\nname = \"D\"\nlang = \"zh-CN\"\n",
            ),
            ("content/a.md", "---\ndescription: d\n---\n# 甲\n"),
            ("content/b.md", "---\ndescription: d\nlang: en\n---\n# B\n"),
            (
                "content/c.md",
                "---\ndescription: d\nlang: english\n---\n# C\n",
            ),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let langs: Vec<&str> = analysis.pages.iter().map(|p| p.lang.as_str()).collect();
        assert_eq!(langs, ["zh-CN", "en", "zh-CN"]);
        assert_eq!(
            errors(&analysis),
            [("lang".to_owned(), "content/c.md".to_owned(), 3)]
        );
    }

    const LANDING: &str = "[landing.profiles.ads]\ncarry = [\"gclid\"]\n[[landing.profiles.ads.scripts]]\nsrc = \"static/landing/init.js\"\n[[landing.profiles.ads.footer]]\nlabel = \"Privacy\"\nlink = \"privacy.md\"\n";

    /// A diagnostic's line and column.
    type Position = (Option<usize>, Option<usize>);

    /// Line and column of `needle`'s first occurrence in `text`.
    fn position(text: &str, needle: &str) -> Position {
        let (line, column) = LineIndex::new(text).position(text.find(needle).unwrap());
        (Some(line), Some(column))
    }

    #[test]
    fn landing_pages_take_the_hero_and_stay_out_of_the_sidebar() {
        let config = format!("{CONFIG}{LANDING}");
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            ("content/index.md", &format!("{FRONT}# Home\n")),
            ("content/privacy.md", &format!("{FRONT}# Privacy\n")),
            // No description and no paragraph to take one from.
            (
                "content/lp.md",
                "---\nlayout: landing\nlanding: ads\n---\n# Try it\n\n## Why\n",
            ),
            ("static/landing/init.js", "window.ready = true;\n"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert!(
            analysis.diagnostics.is_empty(),
            "{:#?}",
            analysis.diagnostics
        );
        let lp = &analysis.pages[analysis.routes["/lp/"]];
        assert_eq!(
            (lp.title.as_str(), lp.description.as_deref()),
            ("Try it", None)
        );
        let listed: Vec<&str> = analysis
            .nav
            .iter()
            .flat_map(|group| &group.items)
            .map(|item| item.route.as_str())
            .collect();
        assert!(
            listed.contains(&"/privacy/") && !listed.contains(&"/lp/"),
            "{listed:?}"
        );
        assert_eq!(
            analysis.landing_footers["ads"],
            [Target::Page {
                page: analysis.routes["/privacy/"],
                anchor: None
            }]
        );
        assert!(analysis.resources.contains("static/landing/init.js"));

        // The hero's title rule: an H1 after other content is a second one.
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            ("content/privacy.md", &format!("{FRONT}# Privacy\n")),
            (
                "content/lp.md",
                "---\ntitle: Try it\nlayout: landing\nlanding: ads\n---\nIntro.\n\n# Author\n",
            ),
            ("static/landing/init.js", ""),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let problem = analysis
            .diagnostics
            .iter()
            .find(|d| d.rule == "multiple-h1")
            .unwrap_or_else(|| panic!("{:#?}", analysis.diagnostics));
        assert!(problem.suggestion.as_deref().unwrap().contains("H2"));
    }

    #[test]
    fn landing_profile_selection_is_checked_where_it_is_written() {
        let config = format!("{CONFIG}{LANDING}[landing.profiles.plain]\ncarry = [\"gclid\"]\n");
        let wrong_layout = "---\ndescription: d\nlanding: ads\n---\n# A\n";
        let undeclared = "---\nlayout: landing\ntitle: landing\nlanding: nope\n---\n# B\n";
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            ("content/privacy.md", &format!("{FRONT}# Privacy\n")),
            ("content/a.md", wrong_layout),
            ("content/b.md", undeclared),
            ("static/landing/init.js", ""),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let found: Vec<(&str, Severity, &str, Position)> = analysis
            .diagnostics
            .iter()
            .map(|d| {
                (
                    d.rule.as_str(),
                    d.severity,
                    d.file.as_deref().unwrap_or(""),
                    (d.line, d.column),
                )
            })
            .collect();
        assert_eq!(
            found,
            [
                (
                    "landing-without-layout",
                    Severity::Error,
                    "content/a.md",
                    position(wrong_layout, "ads")
                ),
                (
                    "landing-profile",
                    Severity::Error,
                    "content/b.md",
                    position(undeclared, "nope")
                ),
                (
                    "landing-profile-unused",
                    Severity::Warning,
                    "rustify-docs.toml",
                    position(&config, "[landing.profiles.plain]")
                ),
            ]
        );
        let undeclared = analysis
            .diagnostics
            .iter()
            .find(|d| d.rule == "landing-profile")
            .unwrap();
        assert_eq!(
            undeclared.suggestion.as_deref(),
            Some("use one of: ads, plain")
        );
    }

    #[test]
    fn landing_profile_files_and_footer_links_must_exist() {
        let config = format!(
            "{CONFIG}[landing.profiles.ads]\n[[landing.profiles.ads.scripts]]\nsrc = \"static/landing/gone.js\"\n[[landing.profiles.ads.scripts]]\nsrc = \".env.js\"\n[[landing.profiles.ads.footer]]\nlabel = \"Gone\"\nlink = \"gone.md\"\n"
        );
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            (
                "content/lp.md",
                "---\nlayout: landing\nlanding: ads\n---\n# Try it\n",
            ),
            (".env.js", "secret\n"),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let mut found: Vec<(&str, Position)> = analysis
            .diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .map(|d| {
                assert_eq!(d.file.as_deref(), Some("rustify-docs.toml"), "{d:?}");
                (d.rule.as_str(), (d.line, d.column))
            })
            .collect();
        found.sort();
        assert_eq!(
            found,
            [
                ("broken-link", position(&config, "\"gone.md\"")),
                (
                    "landing-script-missing",
                    position(&config, "\"static/landing/gone.js\"")
                ),
                ("private-file", position(&config, "\".env.js\"")),
            ]
        );
        assert!(!analysis.resources.iter().any(|path| path.ends_with(".js")));
    }

    /// Each file's diagnostics as (rule, severity), in order.
    fn rules_by_file(analysis: &Analysis) -> BTreeMap<String, Vec<(String, Severity)>> {
        let mut found: BTreeMap<String, Vec<(String, Severity)>> = BTreeMap::new();
        for d in &analysis.diagnostics {
            found
                .entry(d.file.clone().unwrap_or_default())
                .or_default()
                .push((d.rule.clone(), d.severity));
        }
        found
    }

    #[test]
    fn landing_frames_and_forms_follow_the_pages_profile() {
        let config = format!(
            "{CONFIG}[landing.profiles.ads.sources]\nframe = [\"https://forms.example\"]\nform = [\"https://leads.example\"]\n[landing.profiles.plain]\ncarry = [\"gclid\"]\n"
        );
        let body = "<iframe title=\"F\" src=\"https://forms.example/f\"></iframe>\n\n<form method=\"post\" action=\"https://leads.example/lead\"><button formaction=\"thanks.md\">Send</button></form>\n";
        let landing = |profile: &str, body: &str| {
            format!("---\ntitle: Offer\nlayout: landing\n{profile}---\n{body}")
        };
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", &config),
            ("content/lp.md", &landing("landing: ads\n", body)),
            ("content/thanks.md", &landing("landing: ads\n", "Thanks.\n")),
            ("content/plain.md", &landing("landing: plain\n", body)),
            ("content/none.md", &landing("", body)),
            (
                "content/bad.md",
                &landing(
                    "landing: ads\n",
                    "<iframe title=\"F\" src=\"https://other.example/f\"></iframe>\n\n<form action=\"https://other.example/lead\"><button formaction=\"missing.md\">x</button></form>\n",
                ),
            ),
            (
                "content/relative.md",
                &landing("landing: ads\n", "<iframe title=\"F\" src=\"/embed/\"></iframe>\n\n<form action=\"\"></form>\n"),
            ),
            (
                "content/doc.md",
                &format!("{FRONT}# Doc\n\n<iframe title=\"F\" src=\"https://forms.example/f\"></iframe>\n"),
            ),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        let error = |rule: &str| (rule.to_owned(), Severity::Error);
        let expected = BTreeMap::from([
            (
                "content/bad.md".to_owned(),
                vec![
                    error("frame-origin"),
                    error("form-origin"),
                    error("broken-link"),
                ],
            ),
            ("content/doc.md".to_owned(), vec![error("embedded-frame")]),
            // One `form-origin` for the form and its button: the same
            // problem at the same place.
            (
                "content/none.md".to_owned(),
                vec![error("embedded-frame"), error("form-origin")],
            ),
            (
                "content/plain.md".to_owned(),
                vec![error("embedded-frame"), error("form-origin")],
            ),
            (
                "content/relative.md".to_owned(),
                vec![error("frame-origin")],
            ),
        ]);
        let mut found = rules_by_file(&analysis);
        for rules in found.values_mut() {
            rules.sort_by_key(|(rule, _)| {
                [
                    "frame-origin",
                    "embedded-frame",
                    "form-origin",
                    "broken-link",
                ]
                .iter()
                .position(|known| known == rule)
            });
        }
        assert_eq!(found, expected);
        let lp = &analysis.pages[analysis.routes["/lp/"]];
        assert_eq!(
            lp.targets,
            [
                Target::Keep,
                Target::Keep,
                Target::Form {
                    page: analysis.routes["/thanks/"],
                    query: None,
                    anchor: None
                }
            ]
        );
    }

    #[test]
    fn form_on_an_ordinary_page_is_only_a_warning() {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            (
                "content/index.md",
                &format!("{FRONT}# Home\n\n<form action=\"/api/lead\" method=\"post\"><button formaction=\"/api/x\">Send</button></form>\n"),
            ),
        ]);
        let (_, _, analysis) = fixture.analyze(Mode::Production);
        assert_eq!(
            rules_by_file(&analysis),
            BTreeMap::from([(
                "content/index.md".to_owned(),
                vec![
                    ("form-blocked".to_owned(), Severity::Warning),
                    ("form-blocked".to_owned(), Severity::Warning)
                ]
            )])
        );
        assert_eq!(analysis.pages[0].targets, [Target::Keep, Target::Keep]);
    }
}
