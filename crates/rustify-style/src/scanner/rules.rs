// Adapted from the fixed Tailwind ignore fork's gitignore parsing/matching core.
// This module accepts supplied bytes and does not read files or environment configuration.
use super::{ScanError, SourceTree};
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Match {
    None,
    Ignore,
    Whitelist,
}

#[derive(Clone, Debug)]
pub(crate) struct Rules {
    pub base: PathBuf,
    set: GlobSet,
    kinds: Vec<(bool, bool)>,
    only_files: bool,
}

impl Rules {
    pub fn new(
        base: PathBuf,
        lines: impl IntoIterator<Item = String>,
        only_files: bool,
        strict: bool,
    ) -> Result<Self, ScanError> {
        let mut builder = GlobSetBuilder::new();
        let mut kinds = vec![];
        for original in lines {
            let mut line = original.as_str();
            if line.starts_with('#') {
                continue;
            }
            if !line.ends_with("\\ ") {
                line = line.trim_end();
            }
            if line.is_empty() {
                continue;
            }
            let mut whitelist = false;
            let mut absolute = false;
            if line.starts_with("\\!") || line.starts_with("\\#") {
                line = &line[1..];
                absolute = line.starts_with('/');
            } else {
                if line.starts_with('!') {
                    whitelist = true;
                    line = &line[1..];
                }
                if line.starts_with('/') {
                    absolute = true;
                    line = &line[1..];
                }
            }
            let only_dir = line.ends_with('/');
            if only_dir {
                line = &line[..line.len() - 1];
                if line.ends_with('\\') {
                    line = &line[..line.len() - 1];
                }
            }
            let mut actual = line.to_owned();
            if !absolute && !line.contains('/') && !actual.starts_with("**/") && actual != "**" {
                actual = format!("**/{actual}");
            }
            if actual.ends_with("/**") {
                actual.push_str("/*");
            }
            match GlobBuilder::new(&actual)
                .literal_separator(true)
                .backslash_escape(true)
                .build()
            {
                Ok(glob) => {
                    builder.add(glob);
                    kinds.push((whitelist, only_dir));
                }
                Err(error) if strict => return Err(ScanError::InvalidPattern(error.to_string())),
                Err(_) => {}
            }
        }
        let set = builder
            .build()
            .map_err(|error| ScanError::InvalidPattern(error.to_string()))?;
        Ok(Self {
            base,
            set,
            kinds,
            only_files,
        })
    }
    pub fn matched(&self, path: &Path, is_dir: bool) -> Match {
        if self.only_files && is_dir {
            return Match::None;
        }
        if !path.starts_with(&self.base) {
            return Match::None;
        }
        let relative = path.strip_prefix(&self.base).unwrap_or(path);
        for index in self.set.matches(relative).into_iter().rev() {
            let (whitelist, only_dir) = self.kinds[index];
            if !only_dir || is_dir {
                return if whitelist {
                    Match::Whitelist
                } else {
                    Match::Ignore
                };
            }
        }
        Match::None
    }
}

pub(crate) fn defaults() -> Result<Vec<Rules>, ScanError> {
    let group = |fixture: &str, suffix: &str| {
        format!(
            "{{{}}}{suffix}",
            fixture.trim().lines().collect::<Vec<_>>().join(",")
        )
    };
    Ok(vec![
        Rules::new(
            PathBuf::new(),
            [
                group(include_str!("fixtures/ignored-content-dirs.txt"), "/"),
                format!(
                    "*.{}",
                    group(include_str!("fixtures/ignored-extensions.txt"), "")
                ),
                group(include_str!("fixtures/ignored-files.txt"), ""),
            ],
            false,
            true,
        )?,
        Rules::new(PathBuf::new(), [binary_glob()], true, true)?,
    ])
}

pub(crate) fn binary_glob() -> String {
    format!(
        "*.{{{}}}",
        include_str!("fixtures/binary-extensions.txt")
            .trim()
            .lines()
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn ignored_dirs() -> impl Iterator<Item = &'static str> {
    include_str!("fixtures/ignored-content-dirs.txt")
        .trim()
        .lines()
}

pub(crate) fn ignore_file(
    tree: &SourceTree,
    base: &Path,
    path: &Path,
) -> Result<Option<Rules>, ScanError> {
    let Some(bytes) = tree.read(path)? else {
        return Ok(None);
    };
    // The upstream line reader retains valid rules and stops at a non-UTF-8 line.
    let lines = bytes
        .split(|byte| *byte == b'\n')
        .map_while(|line| std::str::from_utf8(line).ok())
        .map(str::to_owned);
    Ok(Some(Rules::new(base.to_path_buf(), lines, false, false)?))
}
