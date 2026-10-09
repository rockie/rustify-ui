use super::{
    rules::{self, Match, Rules},
    sources::{split_pattern, GlobEntry, SourceEntry, Sources},
    ScanError, SourceTree,
};
use fast_glob::glob_match;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

pub(crate) fn explicit_rules(sources: &Sources) -> Result<Vec<Rules>, ScanError> {
    let mut result = rules::defaults()?;
    let mut groups: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
    for source in sources.iter() {
        let base = source.base().to_path_buf();
        match source {
            SourceEntry::Auto { .. } => {}
            SourceEntry::Pattern { pattern, .. } if !pattern.contains("**") => {
                groups
                    .entry(base)
                    .or_default()
                    .insert(format!("!{}", pinned(pattern)));
            }
            SourceEntry::Pattern { pattern, .. } => {
                if let Some(extension) = Path::new(pattern).extension() {
                    groups
                        .entry(base)
                        .or_default()
                        .insert(format!("!*.{}", extension.to_string_lossy()));
                }
            }
            SourceEntry::Ignored { pattern, .. } => {
                groups.entry(base).or_default().insert(pinned(pattern));
            }
            SourceEntry::External { .. } => {
                let group = groups.entry(base).or_default();
                group.insert("!/**/*".into());
                group.insert(rules::binary_glob());
            }
        }
    }
    for (base, patterns) in groups {
        result.push(Rules::new(base, patterns, false, true)?);
    }
    Ok(result)
}
fn pinned(pattern: &str) -> String {
    if pattern.starts_with('/') {
        pattern.into()
    } else {
        format!("/{pattern}")
    }
}

pub(crate) fn source_matches(sources: &Sources, path: &Path) -> bool {
    sources.iter().any(|source| match source {
        SourceEntry::Auto { base } | SourceEntry::External { base } => path.starts_with(base),
        SourceEntry::Pattern { base, pattern } => path.strip_prefix(base).is_ok_and(|relative| {
            glob_match(
                pinned(pattern),
                format!("/{}", relative.to_string_lossy()).as_bytes(),
            )
        }),
        _ => false,
    })
}

#[derive(Default)]
struct DirectoryRules {
    ignore: Option<Rules>,
    gitignore: Option<Rules>,
    exclude: Option<Rules>,
    has_git: bool,
}
struct IgnoreContext<'a> {
    tree: &'a SourceTree,
    explicit: &'a [Rules],
    directories: BTreeMap<PathBuf, DirectoryRules>,
    require_git: bool,
}
impl<'a> IgnoreContext<'a> {
    fn new(
        tree: &'a SourceTree,
        explicit: &'a [Rules],
        first_root: &Path,
    ) -> Result<Self, ScanError> {
        let mut require_git = false;
        for parent in first_root.ancestors() {
            if tree.metadata(&parent.join(".git"))?.is_some() {
                require_git = true;
                break;
            }
        }
        Ok(Self {
            tree,
            explicit,
            directories: BTreeMap::new(),
            require_git,
        })
    }
    fn load(&mut self, path: &Path) -> Result<(), ScanError> {
        if self.directories.contains_key(path) {
            return Ok(());
        }
        let git = path.join(".git");
        let has_git = self.tree.metadata(&git)?.is_some();
        let mut git_directory = git.clone();
        if let Some(bytes) = self.tree.read(&git)? {
            if let Ok(text) = std::str::from_utf8(&bytes) {
                if let Some(target) = text.trim().strip_prefix("gitdir: ") {
                    git_directory = path.join(target);
                }
            }
        }
        if let Some(bytes) = self.tree.read(&git_directory.join("commondir"))? {
            if let Ok(target) = std::str::from_utf8(&bytes) {
                git_directory = git_directory.join(target.trim());
            }
        }
        let rules = DirectoryRules {
            ignore: rules::ignore_file(self.tree, path, &path.join(".ignore"))?,
            gitignore: rules::ignore_file(self.tree, path, &path.join(".gitignore"))?,
            exclude: if has_git {
                rules::ignore_file(self.tree, path, &git_directory.join("info/exclude"))?
            } else {
                None
            },
            has_git,
        };
        self.directories.insert(path.to_path_buf(), rules);
        Ok(())
    }
    fn ignored(&mut self, path: &Path, is_dir: bool) -> Result<bool, ScanError> {
        for rules in self.explicit.iter().rev() {
            match rules.matched(path, is_dir) {
                Match::Ignore => return Ok(true),
                Match::Whitelist => return Ok(false),
                Match::None => {}
            }
        }
        let ancestors: Vec<_> = path
            .parent()
            .into_iter()
            .flat_map(Path::ancestors)
            .map(Path::to_path_buf)
            .collect();
        for ancestor in &ancestors {
            self.load(ancestor)?;
        }
        let any_git =
            !self.require_git || ancestors.iter().any(|path| self.directories[path].has_git);
        let mut seen_git = false;
        let mut ignore = Match::None;
        let mut gitignore = Match::None;
        let mut exclude = Match::None;
        for ancestor in &ancestors {
            let rules = &self.directories[ancestor];
            if ignore == Match::None {
                if let Some(rule) = &rules.ignore {
                    ignore = rule.matched(path, is_dir);
                }
            }
            if any_git && !seen_git {
                if gitignore == Match::None {
                    if let Some(rule) = &rules.gitignore {
                        gitignore = rule.matched(path, is_dir);
                    }
                }
                if exclude == Match::None {
                    if let Some(rule) = &rules.exclude {
                        exclude = rule.matched(path, is_dir);
                    }
                }
            }
            seen_git |= rules.has_git;
        }
        Ok([ignore, gitignore, exclude]
            .into_iter()
            .find(|matched| *matched != Match::None)
            == Some(Match::Ignore))
    }
}

pub(crate) struct Discovery {
    pub files: Vec<PathBuf>,
    pub dirs: Vec<PathBuf>,
}
pub(crate) fn discover(
    tree: &SourceTree,
    sources: &Sources,
    explicit: &[Rules],
    mtimes: &mut BTreeMap<PathBuf, u128>,
    mut wanted: Option<BTreeSet<PathBuf>>,
) -> Result<Discovery, ScanError> {
    let roots: Vec<_> = sources
        .iter()
        .filter(|source| !matches!(source, SourceEntry::Ignored { .. }))
        .map(|source| source.base().to_path_buf())
        .collect();
    let mut result = Discovery {
        files: vec![],
        dirs: vec![],
    };
    let Some(first) = roots.first() else {
        return Ok(result);
    };
    let mut context = IgnoreContext::new(tree, explicit, first)?;
    let mut unique = BTreeSet::new();
    for root in roots {
        if unique.insert(root.clone()) {
            walk(
                tree,
                &root,
                true,
                &mut context,
                sources,
                mtimes,
                &mut result,
                &mut vec![],
                &mut wanted,
            )?;
        }
    }
    Ok(result)
}
#[allow(clippy::too_many_arguments)]
fn walk(
    tree: &SourceTree,
    path: &Path,
    root: bool,
    context: &mut IgnoreContext<'_>,
    sources: &Sources,
    mtimes: &mut BTreeMap<PathBuf, u128>,
    result: &mut Discovery,
    ancestors: &mut Vec<PathBuf>,
    wanted: &mut Option<BTreeSet<PathBuf>>,
) -> Result<(), ScanError> {
    if wanted.as_ref().is_some_and(BTreeSet::is_empty) {
        return Ok(());
    }
    let Some(metadata) = tree.metadata(path)? else {
        return Ok(());
    };
    if !root && context.ignored(path, metadata.is_dir)? {
        return Ok(());
    }
    if metadata.is_dir {
        let Some(canonical) = tree.canonicalize(path)? else {
            return Ok(());
        };
        if ancestors.contains(&canonical) {
            return Ok(());
        }
        result.dirs.push(path.to_path_buf());
        ancestors.push(canonical);
        let mut children = tree.children(path, true)?;
        children.sort();
        for child in children {
            walk(
                tree, &child, false, context, sources, mtimes, result, ancestors, wanted,
            )?;
        }
        ancestors.pop();
    } else if metadata.is_file && source_matches(sources, path) {
        if let Some(current) = metadata.revision {
            if mtimes.insert(path.to_path_buf(), current) == Some(current) {
                return Ok(());
            }
        }
        result.files.push(path.to_path_buf());
        if let Some(wanted) = wanted {
            wanted.remove(path);
        }
    }
    Ok(())
}

pub(crate) fn resolve_globs(
    tree: &SourceTree,
    base: &Path,
    dirs: &[PathBuf],
    extensions: &BTreeSet<String>,
) -> Result<Vec<GlobEntry>, ScanError> {
    let allowed: BTreeSet<_> = dirs.iter().cloned().collect();
    let mut extensions = extensions.clone();
    extensions.extend(
        include_str!("fixtures/template-extensions.txt")
            .trim()
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_owned),
    );
    let extension_list = extensions.into_iter().collect::<Vec<_>>().join(",");
    let mut shallow = BTreeSet::new();
    let mut deep = BTreeSet::new();
    let paths = all_directories(tree, base)?;
    let mut skipped: Vec<PathBuf> = vec![];
    for path in &paths {
        if skipped.iter().any(|prefix| path.starts_with(prefix)) {
            continue;
        }
        if !allowed.contains(path) {
            let mut parent = path.parent();
            while let Some(path) = parent {
                if path == base {
                    break;
                }
                shallow.insert(path.to_path_buf());
                parent = path.parent();
            }
            skipped.push(path.clone());
        }
    }
    skipped.clear();
    for path in &paths {
        if path == base || skipped.iter().any(|prefix| path.starts_with(prefix)) {
            continue;
        }
        if rules::ignored_dirs().any(|dir| path.file_name().is_some_and(|name| name == dir)) {
            skipped.push(path.clone());
            continue;
        }
        if !allowed.contains(path) || shallow.contains(path) {
            continue;
        }
        if path == &base.join("public") {
            skipped.push(path.clone());
            continue;
        }
        deep.insert(path.clone());
        skipped.push(path.clone());
    }
    let mut globs = vec![GlobEntry {
        base: base.to_string_lossy().into_owned(),
        pattern: "*".into(),
    }];
    globs.extend(shallow.into_iter().map(|path| GlobEntry {
        base: path.to_string_lossy().into_owned(),
        pattern: format!("*/*.{{{extension_list}}}"),
    }));
    globs.extend(deep.into_iter().map(|path| GlobEntry {
        base: path.to_string_lossy().into_owned(),
        pattern: format!("**/*.{{{extension_list}}}"),
    }));
    Ok(globs)
}
fn all_directories(tree: &SourceTree, base: &Path) -> Result<Vec<PathBuf>, ScanError> {
    let mut result = vec![];
    if tree.metadata(base)?.is_none_or(|metadata| !metadata.is_dir) {
        return Ok(result);
    }
    result.push(base.to_path_buf());
    let mut children = tree.children(base, false)?;
    children.sort();
    for child in children {
        if tree
            .metadata(&child)?
            .is_some_and(|metadata| metadata.is_dir)
        {
            result.extend(all_directories(tree, &child)?);
        }
    }
    Ok(result)
}

pub(crate) fn optimize_globs(
    tree: &SourceTree,
    entries: Vec<GlobEntry>,
) -> Result<Vec<GlobEntry>, ScanError> {
    let mut grouped: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in entries {
        let (static_part, dynamic_part) = split_pattern(&entry.pattern);
        let path = Path::new(&entry.base).join(static_part.unwrap_or_default());
        let Some(base) = tree.canonicalize(&path)? else {
            continue;
        };
        let is_dir = tree
            .metadata(&base)?
            .is_some_and(|metadata| metadata.is_dir);
        let pattern =
            dynamic_part.unwrap_or_else(|| if is_dir { "**/*".into() } else { String::new() });
        if pattern.is_empty() && !is_dir {
            if let (Some(parent), Some(name)) = (base.parent(), base.file_name()) {
                grouped
                    .entry(parent.to_string_lossy().into_owned())
                    .or_default()
                    .insert(name.to_string_lossy().into_owned());
            }
        }
        grouped
            .entry(base.to_string_lossy().into_owned())
            .or_default()
            .insert(pattern);
    }
    Ok(grouped
        .into_iter()
        .map(|(base, patterns)| GlobEntry {
            base,
            pattern: if patterns.len() == 1 {
                patterns.into_iter().next().unwrap_or_default()
            } else {
                format!("{{{}}}", patterns.into_iter().collect::<Vec<_>>().join(","))
            },
        })
        .collect())
}
