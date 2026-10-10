//! The content graph: one analysis of a project's configuration, Markdown
//! and files, from which every command takes what it needs.
//!
//! It owns the site's addresses (a file's route, a heading's anchor, a
//! resource's location), resolves every link and image against them, and
//! collects every problem it finds into one sorted list, so an author sees
//! all of them in one run. It reads files and nothing else: it writes no
//! output and makes no network request.

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
    pub config: Config,
    pub config_text: String,
}

impl Project {
    /// Reads `rustify-docs.toml` in `root`.
    pub fn load(root: &Path) -> Result<(Self, Vec<Diagnostic>), Vec<Diagnostic>> {
        let path = root.join(config::FILE_NAME);
        let text = std::fs::read_to_string(&path).map_err(|error| {
            vec![Diagnostic::in_file(
                config::FILE_NAME,
                Severity::Error,
                "config-missing",
                format!("cannot read {}: {error}", config::FILE_NAME),
            )]
        })?;
        let locate = |issues: Vec<Issue>| -> Vec<Diagnostic> {
            issues
                .into_iter()
                .map(|issue| Diagnostic::located(config::FILE_NAME, &text, issue))
                .collect()
        };
        match config::parse(&text) {
            Ok((config, warnings)) => Ok((
                Self {
                    root: root.to_path_buf(),
                    config,
                    config_text: text.clone(),
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
/// every other file in the content and asset directories.
#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub markdown: BTreeMap<String, Arc<str>>,
    /// Project-relative path to bytes, for the files a page may reference.
    pub files: BTreeMap<String, Arc<[u8]>>,
}

fn walk(
    dir: &Path,
    relative: &str,
    root: &Path,
    out: &mut Vec<(String, PathBuf)>,
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
        if name == ".git" || name == ".rustify-docs" {
            continue;
        }
        let Ok(meta) = std::fs::metadata(&path) else {
            continue;
        };
        if entry.file_type().is_ok_and(|kind| kind.is_symlink()) {
            // A link is followed only while it stays inside the project.
            let inside = std::fs::canonicalize(&path)
                .ok()
                .zip(std::fs::canonicalize(root).ok())
                .is_some_and(|(target, root)| target.starts_with(root));
            if !inside {
                problems.push(Diagnostic::in_file(
                    &child,
                    Severity::Error,
                    "symlink-outside",
                    "this symbolic link points outside the project; it is not read",
                ));
                continue;
            }
        }
        if meta.is_dir() {
            walk(&path, &child, root, out, problems);
        } else if meta.is_file() {
            out.push((child, path));
        }
    }
}

/// Reads the content and asset directories.
pub fn snapshot(project: &Project) -> (Snapshot, Vec<Diagnostic>) {
    let mut snapshot = Snapshot::default();
    let mut problems = Vec::new();
    let config = &project.config;
    for dir in [&config.content_dir, &config.assets_dir] {
        let mut files = Vec::new();
        walk(
            &project.root.join(dir),
            dir,
            &project.root,
            &mut files,
            &mut problems,
        );
        for (relative, path) in files {
            let Ok(bytes) = std::fs::read(&path) else {
                problems.push(Diagnostic::in_file(
                    &relative,
                    Severity::Error,
                    "unreadable",
                    "this file cannot be read",
                ));
                continue;
            };
            let content_relative = relative
                .strip_prefix(&format!("{}/", config.content_dir))
                .map(str::to_owned);
            match content_relative {
                Some(path) if path.ends_with(".md") && dir == &config.content_dir => {
                    match String::from_utf8(bytes) {
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
                    snapshot.files.insert(relative, Arc::from(bytes));
                }
            }
        }
    }
    (snapshot, problems)
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

/// A fragment's cache key: everything that decides its bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FragmentKey([u8; 32]);

impl FragmentKey {
    pub fn of(body: &str, offset: usize, lang: &str, includes: &IncludeFiles) -> Self {
        let (labels, foreign) = labels::for_lang(lang);
        let mut hash = Sha256::new();
        hash.update(WRITER_VERSION.to_le_bytes());
        hash.update(GRAMMAR_VERSION.as_bytes());
        hash.update([0]);
        hash.update(markdown::options().bits().to_le_bytes());
        hash.update(labels.lang.as_bytes());
        hash.update([u8::from(foreign)]);
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
}

/// Fragments by key. Entries an analysis no longer uses are dropped, so the
/// cache tracks the site rather than its history.
#[derive(Default)]
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
}

/// Where a URL in a page points, once resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// Leave the URL as written: external, an in-page anchor, a scheme.
    Keep,
    /// A page, with an optional anchor.
    Page { page: usize, anchor: Option<String> },
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

/// One page of the site.
#[derive(Clone, Debug)]
pub struct Page {
    /// Content-relative, like `guide/setup.md`.
    pub path: String,
    /// Project-relative, like `content/guide/setup.md`.
    pub file: String,
    pub route: String,
    pub front: Front,
    pub title: String,
    pub description: Option<String>,
    pub lang: String,
    pub draft: bool,
    pub fragment: Arc<Fragment>,
    /// For each of the fragment's references, where it points.
    pub targets: Vec<Target>,
    /// For each of the frontmatter's `actions`, where its link points.
    pub action_targets: Vec<Target>,
}

/// The analysed site.
pub struct Analysis {
    pub mode: Mode,
    /// Every page, drafts included, in path order.
    pub pages: Vec<Page>,
    pub routes: BTreeMap<String, usize>,
    pub nav: Vec<NavGroup>,
    /// Where each of the configuration's `nav.top` and `nav.social` links
    /// points, in their order.
    pub top: Vec<Target>,
    pub social: Vec<Target>,
    /// Project-relative files the pages reference, to be published.
    pub resources: BTreeSet<String>,
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
fn read_include(project: &Project, path: &str) -> Result<String, String> {
    if is_private(path) {
        return Err("this file is private and is never included".into());
    }
    let full = project.root.join(path);
    let canonical =
        std::fs::canonicalize(&full).map_err(|_| "the file does not exist".to_owned())?;
    let root = std::fs::canonicalize(&project.root).map_err(|e| e.to_string())?;
    if !canonical.starts_with(&root) {
        return Err("the file is outside the project".into());
    }
    std::fs::read_to_string(&canonical).map_err(|_| "the file is not readable UTF-8 text".into())
}

/// Splits `url` into its path and its `#fragment`.
fn split_fragment(url: &str) -> (&str, Option<&str>) {
    match url.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment)),
        None => (url, None),
    }
}

fn has_scheme(url: &str) -> bool {
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
fn join(dir: &str, relative: &str) -> Option<String> {
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

fn percent_decode(text: &str) -> String {
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
    let config = &project.config;
    let content_dir = &config.content_dir;
    let mut diagnostics = Vec::new();
    let file_of = |path: &str| format!("{content_dir}/{path}");

    // Pages: frontmatter, route, includes.
    struct Draft {
        path: String,
        front: Front,
        body_offset: usize,
        includes: IncludeFiles,
        key: FragmentKey,
        lang: String,
    }
    let mut drafts = Vec::new();
    let mut routes: BTreeMap<String, usize> = BTreeMap::new();
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
        let route = route_for(path);
        if route == "/404/" {
            diagnostics.push(
                Diagnostic::in_file(
                    &file,
                    Severity::Error,
                    "reserved-route",
                    "`/404/` is reserved for the generated not-found page",
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
                        "this page and `{}` both become {route}",
                        file_of(&other.path)
                    ),
                )
                .with_suggestion("rename or move one of them"),
            );
            continue;
        }
        let lang = match &front.lang {
            Some(lang) if config::valid_lang(lang) => lang.clone(),
            Some(lang) => {
                diagnostics.push(Diagnostic::in_file(
                    &file,
                    Severity::Error,
                    "lang",
                    format!("`{lang}` is not a language tag"),
                ));
                config.site.lang.clone()
            }
            None => config.site.lang.clone(),
        };
        let body = &text[body_offset..];
        let mut includes = IncludeFiles::default();
        for include in markdown::include_paths(body) {
            let read = read_include(project, &include);
            includes.0.insert(include, read);
        }
        let key = FragmentKey::of(body, body_offset, &lang, &includes);
        routes.insert(route, drafts.len());
        drafts.push(Draft {
            path: path.clone(),
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
        let (labels, foreign) = labels::for_lang(&draft.lang);
        let context = markdown::Context {
            highlighter,
            labels,
            foreign,
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
    let live: std::collections::HashSet<FragmentKey> =
        drafts.iter().map(|draft| draft.key).collect();
    cache.entries.retain(|key, _| live.contains(key));

    let mut pages: Vec<Page> = Vec::with_capacity(drafts.len());
    for draft in drafts {
        let fragment = cache.entries[&draft.key].clone();
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
        if fragment.h1_count > 1 {
            let second = fragment
                .headings
                .iter()
                .filter(|h| h.level == 1)
                .nth(1)
                .map_or(0, |h| h.span.start);
            diagnostics.push(Diagnostic::located(
                &file,
                text,
                Issue::error(
                    second..second,
                    "multiple-h1",
                    "a page has one top-level heading; this is the second",
                )
                .suggest("make it a `##` heading"),
            ));
        }
        let mut previous = if fragment.h1_count > 0 || draft.front.title.is_some() {
            1
        } else {
            0
        };
        for heading in &fragment.headings {
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
        if draft.front.layout != Layout::Home {
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
        pages.push(Page {
            route: route_for(&draft.path),
            file,
            path: draft.path,
            title,
            description,
            lang: draft.lang,
            draft: draft.front.draft,
            front: draft.front,
            fragment,
            targets: Vec::new(),
            action_targets: Vec::new(),
        });
    }
    let routes: BTreeMap<String, usize> = pages
        .iter()
        .enumerate()
        .map(|(i, page)| (page.route.clone(), i))
        .collect();
    let by_path: HashMap<String, usize> = pages
        .iter()
        .enumerate()
        .map(|(i, page)| (page.path.clone(), i))
        .collect();

    // Code groups: one name, one set of tabs, across the site.
    let mut groups: BTreeMap<&str, (BTreeSet<&str>, &str)> = BTreeMap::new();
    for page in &pages {
        for group in &page.fragment.code_groups {
            let labels: BTreeSet<&str> = group.labels.iter().map(String::as_str).collect();
            match groups.get(group.name.as_str()) {
                None => {
                    groups.insert(&group.name, (labels, &page.file));
                }
                Some((known, first)) if *known != labels => {
                    let text = &snapshot.markdown[&page.path];
                    diagnostics.push(Diagnostic::located(
                        &page.file,
                        text,
                        Issue::error(
                            group.span.clone(),
                            "code-group-conflict",
                            format!("code group `{}` has other tabs in {first}; groups of one name switch together", group.name),
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
        let text = snapshot.markdown[&page.path].clone();
        let dir = page.path.rsplit_once('/').map_or("", |(dir, _)| dir);
        let mut targets = Vec::with_capacity(page.fragment.references.len());
        let mut found = Vec::new();
        for reference in &page.fragment.references {
            let issue = |rule: &'static str, message: String| {
                Issue::error(reference.span.clone(), rule, message)
            };
            let url = reference.destination.trim();
            let target = match reference.kind {
                RefKind::Link => resolve_link(
                    project,
                    snapshot,
                    &pages,
                    &by_path,
                    &routes,
                    Some(i),
                    dir,
                    url,
                    mode,
                    &mut found,
                    &issue,
                ),
                RefKind::Image | RefKind::Media | RefKind::Srcset => {
                    let image = reference.kind != RefKind::Media;
                    resolve_resource(
                        project,
                        snapshot,
                        dir,
                        url,
                        image,
                        reference.kind == RefKind::Srcset,
                        &mut found,
                        &issue,
                    )
                }
            };
            match &target {
                Target::Resource(path) => {
                    resources.insert(path.clone());
                }
                Target::Srcset(candidates) => {
                    resources.extend(candidates.iter().filter_map(|c| c.file.clone()));
                }
                _ => {}
            }
            targets.push(target);
        }
        // A home page's actions are links too, written in its frontmatter.
        let yaml = frontmatter::split(&text)
            .ok()
            .flatten()
            .map_or(0..0, |split| split.yaml);
        let mut cursor = yaml.start;
        let mut action_targets = Vec::with_capacity(page.front.actions.len());
        for action in &page.front.actions {
            let link = action.link.trim();
            let at = text[cursor..yaml.end]
                .find(link)
                .map_or(yaml.start, |at| cursor + at);
            cursor = cursor.max(at + link.len()).min(yaml.end);
            let issue = |rule: &'static str, message: String| {
                Issue::error(at..at + link.len(), rule, message)
            };
            action_targets.push(resolve_link(
                project,
                snapshot,
                &pages,
                &by_path,
                &routes,
                Some(i),
                dir,
                link,
                mode,
                &mut found,
                &issue,
            ));
        }
        for issue in found {
            diagnostics.push(Diagnostic::located(&pages[i].file, &text, issue));
        }
        pages[i].targets = targets;
        pages[i].action_targets = action_targets;
    }

    // The configuration's header links resolve from the content root.
    let mut config_links = |links: &[config::NavLink]| -> Vec<Target> {
        let mut found = Vec::new();
        let targets = links
            .iter()
            .map(|link| {
                let issue = |rule: &'static str, message: String| {
                    Issue::error(link.span.clone(), rule, message)
                };
                resolve_link(
                    project,
                    snapshot,
                    &pages,
                    &by_path,
                    &routes,
                    None,
                    "",
                    link.link.trim(),
                    mode,
                    &mut found,
                    &issue,
                )
            })
            .collect();
        for issue in found {
            diagnostics.push(Diagnostic::located(
                config::FILE_NAME,
                &project.config_text,
                issue,
            ));
        }
        targets
    };
    let top = config_links(&config.nav.top);
    let social = config_links(&config.nav.social);

    // Navigation.
    let nav = navigation(project, snapshot, &pages, mode, &by_path, &mut diagnostics);

    // The logo and the brand colours.
    if let Some(logo) = &config.logo {
        for path in [&logo.light, &logo.dark] {
            if snapshot.files.contains_key(path) || project.root.join(path).is_file() {
                resources.insert(path.clone());
            } else {
                diagnostics.push(Diagnostic::located(
                    config::FILE_NAME,
                    &project.config_text,
                    Issue::error(
                        logo.span.clone(),
                        "logo-missing",
                        format!("theme.logo `{path}` does not exist"),
                    ),
                ));
            }
        }
    }
    if let Some((path, span)) = &config.site.image {
        if snapshot.files.contains_key(path) || project.root.join(path).is_file() {
            resources.insert(path.clone());
        } else {
            diagnostics.push(Diagnostic::located(
                config::FILE_NAME,
                &project.config_text,
                Issue::error(
                    span.clone(),
                    "image-missing",
                    format!("site.image `{path}` does not exist"),
                ),
            ));
        }
    }
    if config.site.url.is_none() {
        diagnostics.push(
            Diagnostic::in_file(
                config::FILE_NAME,
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
        if is_private(path) {
            continue;
        }
        let taken = page_files.contains(output)
            || crate::generation::SITE_FILES.contains(&output)
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
                        "this file would be published at `{output}`, which the build writes itself"
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
                    config::FILE_NAME,
                    &project.config_text,
                    Issue::warning(span.clone(), "brand-contrast", format!("{color} as link text on the {mode_name} background is {link:.2}:1; text needs 4.5:1"))
                        .suggest(format!("pick a {} {mode_name} primary", if mode_name == "light" { "darker" } else { "lighter" })),
                ));
            }
            if button < 4.5 {
                diagnostics.push(Diagnostic::located(
                    config::FILE_NAME,
                    &project.config_text,
                    Issue::warning(
                        span.clone(),
                        "brand-contrast",
                        format!("no button label reaches 4.5:1 on {color} ({button:.2}:1 at best)"),
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
        nav,
        top,
        social,
        resources,
        diagnostics,
    }
}

#[allow(clippy::too_many_arguments)]
fn resolve_link(
    project: &Project,
    snapshot: &Snapshot,
    pages: &[Page],
    by_path: &HashMap<String, usize>,
    routes: &BTreeMap<String, usize>,
    from: Option<usize>,
    dir: &str,
    url: &str,
    mode: Mode,
    found: &mut Vec<Issue>,
    issue: &dyn Fn(&'static str, String) -> Issue,
) -> Target {
    if url.is_empty() {
        found.push(issue("empty-link", "this link has no destination".into()));
        return Target::Keep;
    }
    if has_scheme(url) || url.starts_with("//") {
        let scheme = url.split(':').next().unwrap_or("").to_ascii_lowercase();
        if matches!(scheme.as_str(), "javascript" | "vbscript" | "data") {
            found.push(issue(
                "script-url",
                format!("a `{scheme}:` link is not allowed"),
            ));
        }
        return Target::Keep;
    }
    let (path, fragment) = split_fragment(url);
    let fragment = fragment.map(percent_decode);
    let check_anchor = |page: usize, found: &mut Vec<Issue>| {
        if let Some(anchor) = fragment.as_deref().filter(|anchor| !anchor.is_empty()) {
            let known = pages[page]
                .fragment
                .headings
                .iter()
                .any(|heading| heading.id == anchor)
                || anchor.starts_with("fn-")
                || anchor.starts_with("fnref-");
            if !known {
                found.push(
                    issue("broken-anchor", format!("{} has no heading `#{anchor}`", pages[page].file))
                        .suggest("link to an existing heading's id; ids are the heading text in lower case with spaces as `-`"),
                );
            }
        }
    };
    let to_page = |target: usize, found: &mut Vec<Issue>| -> Target {
        if pages[target].draft && !from.is_some_and(|from| pages[from].draft) {
            let message = format!("{} is a draft and is not published", pages[target].file);
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
        check_anchor(target, found);
        Target::Page {
            page: target,
            anchor: fragment.clone(),
        }
    };
    if path.is_empty() {
        match from {
            Some(from) => check_anchor(from, found),
            None => found.push(issue(
                "broken-link",
                format!("`{url}` names no page; link to a page or a URL"),
            )),
        }
        return Target::Keep;
    }
    let decoded = percent_decode(path);
    if let Some(absolute) = decoded.strip_prefix('/') {
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
        found.push(issue("broken-link", format!("no page or file at `{path}`")));
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
                return match by_path.get(markdown.as_str()) {
                    Some(&target) => to_page(target, found),
                    None => {
                        found.push(issue(
                            "broken-link",
                            format!(
                                "there is no page `{}/{markdown}`",
                                project.config.content_dir
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
            found.push(issue("broken-link", format!("there is no file `{file}`")));
            Target::Keep
        }
        None => {
            // Outside the content: a file in the repository.
            let project_dir = format!("{}/{dir}", project.config.content_dir);
            let Some(file) = join(&project_dir, &decoded) else {
                found.push(issue("broken-link", format!("`{path}` leaves the project")));
                return Target::Keep;
            };
            if !project.root.join(&file).exists() || is_private(&file) {
                found.push(issue("broken-link", format!("there is no file `{file}`")));
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
                        issue("repo-link", format!("`{file}` is outside the content and the site has no [repo] to link it to"))
                            .suggest("add [repo] url and ref to rustify-docs.toml, or link to a page"),
                    );
                    Target::Keep
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn resolve_resource(
    project: &Project,
    snapshot: &Snapshot,
    dir: &str,
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
                issue("cross-origin-resource", format!("`{candidate}` is loaded from another origin, which the site's policy blocks"))
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
        let file = match path.strip_prefix('/') {
            Some(absolute) => Some(format!("{}/{absolute}", project.config.assets_dir)),
            None => join(&format!("{}/{dir}", project.config.content_dir), &path),
        };
        match file {
            Some(file)
                if !is_private(&file)
                    && (snapshot.files.contains_key(&file)
                        || project.root.join(&file).is_file()) =>
            {
                resolved.push(SrcsetCandidate {
                    file: Some(file),
                    url: candidate.clone(),
                    descriptor: descriptor.clone(),
                });
            }
            _ => {
                found.push(
                    issue(if image { "missing-image" } else { "missing-media" }, format!("`{candidate}` does not exist"))
                        .suggest("check the path; it is relative to this page, or to the asset directory when it starts with `/`"),
                );
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

fn navigation(
    project: &Project,
    snapshot: &Snapshot,
    pages: &[Page],
    mode: Mode,
    by_path: &HashMap<String, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<NavGroup> {
    let config = &project.config;
    let shown = |page: &Page| mode == Mode::Dev || !page.draft;
    let listable = |page: &Page| shown(page) && page.front.layout != Layout::Home;
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
                    label: label.to_owned(),
                    prefix: "/".into(),
                    items: ranked(found).into_iter().map(item).collect(),
                });
            }
        }
        return groups;
    }

    // With groups configured, `sidebar.group` names one of them.
    let known = |label: &str| config.nav.groups.iter().any(|group| group.label == label);
    for page in pages {
        let Some(label) = chosen(page).filter(|label| !known(label)) else {
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
        let names: Vec<&str> = config
            .nav
            .groups
            .iter()
            .map(|group| group.label.as_str())
            .collect();
        diagnostics.push(Diagnostic::located(
            &page.file,
            text,
            Issue::error(
                at..at + label.len(),
                "nav-unknown-group",
                format!(
                    "sidebar.group `{label}` is not a nav group in {}",
                    config::FILE_NAME
                ),
            )
            .suggest(format!("use one of: {}", names.join(", "))),
        ));
    }
    // A group's `items` is an explicit list; a page in one is not moved.
    let explicit: BTreeSet<usize> = config
        .nav
        .groups
        .iter()
        .filter_map(|group| match &group.source {
            GroupSource::Items(list) => Some(list),
            GroupSource::Dir { .. } => None,
        })
        .flatten()
        .filter_map(|(path, _)| by_path.get(path.as_str()).copied())
        .collect();
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
                        joins(i) || (page.path.starts_with(&prefix) && listable(page) && !moved)
                    })
                    .collect();
                if found.is_empty() {
                    diagnostics.push(Diagnostic::located(
                        config::FILE_NAME,
                        &project.config_text,
                        Issue::warning(
                            span.clone(),
                            "nav-empty",
                            format!("nav group `{}` lists no pages under `{dir}`", group.label),
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
                            config::FILE_NAME,
                            &project.config_text,
                            Issue::error(
                                span.clone(),
                                "nav-missing-page",
                                format!(
                                    "nav lists `{path}`, which is not a page in {}",
                                    config.content_dir
                                ),
                            ),
                        )),
                    }
                }
                found.extend(ranked((0..pages.len()).filter(|&i| joins(i)).collect()));
                found
            }
        };
        listed.extend(found.iter().copied());
        groups.push(NavGroup {
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
            let (project, mut warnings) =
                Project::load(&self.root).unwrap_or_else(|errors| panic!("{errors:#?}"));
            let (snapshot, problems) = snapshot(&project);
            warnings.extend(problems);
            let mut analysis = analyze(
                &project,
                &snapshot,
                mode,
                &mut FragmentCache::default(),
                &Highlighter::new(),
                2,
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
            [("lang".to_owned(), "content/c.md".to_owned(), 0)]
        );
    }
}
