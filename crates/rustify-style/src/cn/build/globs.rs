use super::source::{resolve, Kind};
use super::{BuildSourceTree, CnError};
use regex::Regex;
use std::collections::HashSet;

pub const DEFAULT_CONTENT: &[&str] = &["**/*.{js,jsx,ts,tsx,html,vue,svelte,astro,mdx}"];
const IGNORED: &[&str] = &[
    "node_modules",
    ".git",
    "dist",
    "build",
    ".next",
    ".nuxt",
    "out",
    "coverage",
    ".svelte-kit",
    ".astro",
    ".vercel",
    ".output",
];
fn normalize(pattern: &str) -> String {
    pattern
        .replace('\\', "/")
        .strip_prefix("./")
        .unwrap_or(&pattern.replace('\\', "/"))
        .to_owned()
}
fn is_glob(pattern: &str) -> bool {
    pattern.contains(['*', '?', '{'])
}
fn literal_prefix(pattern: &str) -> String {
    pattern
        .split('/')
        .take_while(|s| !is_glob(s))
        .collect::<Vec<_>>()
        .join("/")
}
// ECMAScript RegExp without /u consumes UTF-16 units. Map each unit to one
// scalar so Rust Regex preserves '?' and adjacent wildcard behavior.
fn regex_units(text: &str) -> String {
    text.encode_utf16()
        .filter_map(|unit| char::from_u32(0x10000 + u32::from(unit)))
        .collect()
}
fn glob_regex(pattern: &str) -> Result<Regex, CnError> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    let mut re = String::from("^");
    while i < chars.len() {
        match chars[i] {
            '*' if chars.get(i + 1) == Some(&'*') => {
                re.push_str(&format!("(?:[^{}]*)", regex_units("\n\r\u{2028}\u{2029}")));
                i += 1;
                if chars.get(i + 1) == Some(&'/') {
                    re.push_str(&format!("{}?", regex_units("/")));
                    i += 1;
                }
            }
            '*' => re.push_str(&format!("[^{}]*", regex_units("/"))),
            '?' => re.push_str(&format!("[^{}]", regex_units("/"))),
            '{' => {
                let end = (i + 1..chars.len())
                    .find(|&j| chars[j] == '}')
                    .ok_or_else(|| CnError::new(format!("unclosed {{ in glob: {pattern}")))?;
                re.push_str("(?:");
                re.push_str(
                    &chars[i + 1..end]
                        .iter()
                        .collect::<String>()
                        .split(',')
                        .map(|s| regex::escape(&regex_units(s)))
                        .collect::<Vec<_>>()
                        .join("|"),
                );
                re.push(')');
                i = end;
            }
            c => re.push_str(&regex::escape(&regex_units(&c.to_string()))),
        };
        i += 1;
    }
    re.push_str(r"\z");
    Regex::new(&re).map_err(|err| CnError::new(format!("invalid glob {pattern}: {err}")))
}
fn opt_in(patterns: &[String]) -> Vec<String> {
    patterns
        .iter()
        .map(|p| literal_prefix(p))
        .filter(|p| !p.is_empty())
        .collect()
}
fn opted(prefixes: &[String], dir: &str) -> bool {
    prefixes
        .iter()
        .any(|p| p == dir || p.starts_with(&(dir.to_owned() + "/")))
}
/// Nonfatal unreadable path count accumulated during glob traversal.
#[derive(Default, Debug)]
pub struct ScanState {
    pub skipped_unreadable: usize,
}
fn walk(
    tree: &BuildSourceTree,
    dir: &str,
    rel: &str,
    out: &mut Vec<String>,
    opt_in: &[String],
    ancestors: &mut HashSet<String>,
    state: &mut ScanState,
) {
    let Ok(real) = tree.realpath(dir) else {
        state.skipped_unreadable += 1;
        return;
    };
    if !ancestors.insert(real.clone()) {
        return;
    }
    let Ok(entries) = tree.children(dir) else {
        state.skipped_unreadable += 1;
        ancestors.remove(&real);
        return;
    };
    for (name, kind) in entries {
        let child_rel = if rel.is_empty() || rel == ".." || rel.starts_with("../") {
            name.clone()
        } else {
            format!("{rel}/{name}")
        };
        let Ok(child) = tree.resolve_path(dir, &name) else {
            state.skipped_unreadable += 1;
            continue;
        };
        let kind = if kind == Kind::Symlink {
            match tree.kind(&child) {
                Ok(kind) => kind,
                Err(_) => continue,
            }
        } else {
            kind
        };
        if kind == Kind::Directory {
            if name == ".git" {
                continue;
            }
            if (IGNORED.contains(&name.as_str()) || name.starts_with('.'))
                && !opted(opt_in, &child_rel)
            {
                continue;
            }
            walk(tree, &child, &child_rel, out, opt_in, ancestors, state);
        } else if kind == Kind::File {
            out.push(child_rel);
        }
    }
    ancestors.remove(&real);
}
/// Expand cn mini-globs. Symlinks are followed with branch-local cycle detection.
pub fn expand_globs(
    tree: &BuildSourceTree,
    patterns: &[String],
    cwd: &str,
    state: &mut ScanState,
) -> Result<Vec<String>, CnError> {
    let cwd = tree.cwd(cwd)?;
    let patterns: Vec<_> = patterns.iter().map(|p| normalize(p)).collect();
    let prefixes = opt_in(&patterns);
    let mut all_files = None;
    let mut files = Vec::new();
    let mut seen = HashSet::new();
    for pattern in &patterns {
        if !is_glob(pattern) {
            let path = tree.resolve_path(&cwd, pattern)?;
            match tree.kind(&path) {
                Ok(Kind::File) => {
                    if seen.insert(path.clone()) {
                        files.push(path);
                    }
                }
                Ok(Kind::Directory) => {
                    let mut sub = Vec::new();
                    walk(tree, &path, "", &mut sub, &[], &mut HashSet::new(), state);
                    for relative in sub {
                        let file = tree.resolve_path(&path, &relative)?;
                        if seen.insert(file.clone()) {
                            files.push(file);
                        }
                    }
                }
                _ => {}
            };
            continue;
        }
        if all_files.is_none() {
            let mut all = Vec::new();
            walk(
                tree,
                &cwd,
                "",
                &mut all,
                &prefixes,
                &mut HashSet::new(),
                state,
            );
            all_files = Some(
                all.into_iter()
                    .map(|relative| {
                        let encoded = regex_units(&relative);
                        (relative, encoded)
                    })
                    .collect::<Vec<_>>(),
            );
        }
        let regex = glob_regex(pattern)?;
        if let Some(all) = &all_files {
            for (relative, encoded) in all {
                if regex.is_match(encoded) {
                    let file = tree.resolve_path(&cwd, relative)?;
                    if seen.insert(file.clone()) {
                        files.push(file);
                    }
                }
            }
        }
    }
    Ok(files)
}
/// A content predicate requiring no filesystem access.
pub struct ContentMatcher {
    cwd: String,
    literals: Vec<String>,
    regexes: Vec<Regex>,
    opt_in: Vec<String>,
}
impl ContentMatcher {
    pub fn matches(&self, file: &str) -> bool {
        let file = resolve(&self.cwd, file);
        let prefix = if self.cwd.ends_with('/') {
            self.cwd.clone()
        } else {
            self.cwd.clone() + "/"
        };
        let Some(rel) = file.strip_prefix(&prefix) else {
            return false;
        };
        self.matches_relative(rel)
    }
    /// Apply matching after a host's native path.relative/sep normalization.
    pub fn matches_relative(&self, rel: &str) -> bool {
        if rel.is_empty() {
            return false;
        }
        if self
            .literals
            .iter()
            .any(|lit| rel == lit || rel.starts_with(&(lit.clone() + "/")))
        {
            return true;
        }
        let segments: Vec<_> = rel.split('/').collect();
        let mut path = String::new();
        for name in &segments[..segments.len() - 1] {
            if !path.is_empty() {
                path.push('/');
            }
            path.push_str(name);
            if *name == ".git" {
                return false;
            }
            if (IGNORED.contains(name) || name.starts_with('.')) && !opted(&self.opt_in, &path) {
                return false;
            }
        }
        let encoded = regex_units(rel);
        self.regexes.iter().any(|re| re.is_match(&encoded))
    }
}
/// Apply the upstream literal-path, glob, and ignored-directory matcher rules.
pub fn create_content_matcher(patterns: &[String], cwd: &str) -> Result<ContentMatcher, CnError> {
    let normalized: Vec<_> = patterns.iter().map(|p| normalize(p)).collect();
    Ok(ContentMatcher {
        cwd: cwd.into(),
        literals: normalized.iter().filter(|p| !is_glob(p)).cloned().collect(),
        regexes: normalized
            .iter()
            .filter(|p| is_glob(p))
            .map(|p| glob_regex(p))
            .collect::<Result<_, _>>()?,
        opt_in: opt_in(&normalized),
    })
}
