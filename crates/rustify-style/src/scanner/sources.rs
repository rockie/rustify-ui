use super::{rules, ScanError, SourceTree};
use bexpand::Expression;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlobEntry {
    pub base: String,
    pub pattern: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicSourceEntry {
    pub base: String,
    pub pattern: String,
    pub negated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceEntry {
    Auto { base: PathBuf },
    Pattern { base: PathBuf, pattern: String },
    Ignored { base: PathBuf, pattern: String },
    External { base: PathBuf },
}
impl SourceEntry {
    pub(crate) fn base(&self) -> &Path {
        match self {
            Self::Auto { base }
            | Self::External { base }
            | Self::Pattern { base, .. }
            | Self::Ignored { base, .. } => base,
        }
    }
    pub(crate) fn glob(&self) -> GlobEntry {
        GlobEntry {
            base: self.base().to_string_lossy().into_owned(),
            pattern: match self {
                Self::Pattern { pattern, .. } | Self::Ignored { pattern, .. } => pattern.clone(),
                _ => "**/*".into(),
            },
        }
    }
}

impl PublicSourceEntry {
    pub fn optimize(&mut self, tree: &SourceTree) -> Result<(), ScanError> {
        let Some(base) = tree.canonicalize(Path::new(&self.base))? else {
            return Ok(());
        };
        self.base = base.to_string_lossy().into_owned();
        if !self.pattern.contains('*') {
            if let Some(path) = tree.canonicalize(&base.join(&self.pattern))? {
                if tree
                    .metadata(&path)?
                    .is_some_and(|metadata| metadata.is_dir)
                {
                    self.base = path.to_string_lossy().into_owned();
                    self.pattern = "**/*".into();
                } else if let (Some(parent), Some(name)) = (path.parent(), path.file_name()) {
                    self.base = parent.to_string_lossy().into_owned();
                    self.pattern = format!("/{}", name.to_string_lossy());
                }
            }
            return Ok(());
        }
        let (static_part, dynamic_part) = split_pattern(&self.pattern);
        let base = if let Some(part) = static_part {
            let Some(path) = tree.canonicalize(&base.join(part))? else {
                return Ok(());
            };
            path
        } else {
            base
        };
        self.pattern = dynamic_part.unwrap_or_else(|| {
            if tree
                .metadata(&base)
                .ok()
                .flatten()
                .is_some_and(|metadata| metadata.is_dir)
            {
                "**/*".into()
            } else {
                String::new()
            }
        });
        self.base = base.to_string_lossy().into_owned();
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Sources {
    pub(crate) entries: Vec<SourceEntry>,
}
impl Sources {
    pub fn new(entries: Vec<SourceEntry>) -> Self {
        Self { entries }
    }
    pub fn iter(&self) -> impl Iterator<Item = &SourceEntry> {
        self.entries.iter()
    }
}

pub fn normalize_sources(
    entries: Vec<PublicSourceEntry>,
    tree: &SourceTree,
) -> Result<Sources, ScanError> {
    let mut result = vec![];
    for entry in entries {
        let patterns = match Expression::try_from(entry.pattern.as_str()) {
            Ok(expression) => expression
                .into_iter()
                .filter_map(Result::ok)
                .map(|pattern| pattern.into())
                .collect::<Vec<String>>(),
            Err(_) => vec![entry.pattern.clone()],
        };
        for pattern in patterns {
            let mut entry = PublicSourceEntry {
                pattern,
                ..entry.clone()
            };
            entry.optimize(tree)?;
            let base = PathBuf::from(&entry.base);
            let auto = entry.pattern.ends_with("**/*")
                || tree
                    .metadata(&base.join(&entry.pattern))?
                    .is_some_and(|metadata| metadata.is_dir);
            let external = rules::ignored_dirs().any(|dir| {
                entry.base.contains(&format!("/{dir}/")) || entry.base.ends_with(&format!("/{dir}"))
            });
            result.push(match (entry.negated, auto, external) {
                (true, _, _) => SourceEntry::Ignored {
                    base,
                    pattern: entry.pattern,
                },
                (false, true, true) => SourceEntry::External { base },
                (false, true, false) => SourceEntry::Auto { base },
                (false, false, _) => SourceEntry::Pattern {
                    base,
                    pattern: entry.pattern,
                },
            });
        }
    }
    Ok(Sources::new(result))
}

pub(crate) fn split_pattern(pattern: &str) -> (Option<String>, Option<String>) {
    if !pattern.contains('*') {
        return (Some(pattern.into()), None);
    }
    let mut slash = None;
    for (index, character) in pattern.char_indices() {
        if character == '/' {
            slash = Some(index);
        }
        if character == '*' || character == '!' {
            break;
        }
    }
    let Some(slash) = slash else {
        return (None, Some(pattern.into()));
    };
    let static_part = &pattern[..slash];
    let dynamic_part = &pattern[slash + 1..];
    (
        (!static_part.is_empty()).then(|| static_part.into()),
        (!dynamic_part.is_empty()).then(|| dynamic_part.into()),
    )
}
