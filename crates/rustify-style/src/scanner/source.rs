use super::{
    discovery,
    rules::Rules,
    sources::{self, SourceEntry, Sources},
    GlobEntry, PublicSourceEntry, ScanError, ScanResult, SourceTree,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum ChangedContent {
    File(PathBuf, String),
    Content(String, String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceScanResult {
    pub candidates: Vec<String>,
    pub variables: Vec<String>,
    pub files: Vec<String>,
    pub globs: Vec<GlobEntry>,
    pub normalized_sources: Vec<GlobEntry>,
}

/// Stateful source scanning. Candidates accumulate, while changed content returns only new names.
#[derive(Clone, Debug)]
pub struct Scanner {
    tree: SourceTree,
    sources: Sources,
    explicit: Vec<Rules>,
    mtimes: BTreeMap<PathBuf, u128>,
    candidates: BTreeSet<String>,
    variables: BTreeSet<String>,
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
    extensions: BTreeSet<String>,
    globs: Option<Vec<GlobEntry>>,
    pending: Vec<(PathBuf, String, bool)>,
}
impl Scanner {
    pub fn new(sources: Vec<PublicSourceEntry>, tree: SourceTree) -> Result<Self, ScanError> {
        let sources = sources::normalize_sources(sources, &tree)?;
        let explicit = discovery::explicit_rules(&sources)?;
        Ok(Self {
            tree,
            sources,
            explicit,
            mtimes: BTreeMap::new(),
            candidates: BTreeSet::new(),
            variables: BTreeSet::new(),
            files: vec![],
            dirs: vec![],
            extensions: BTreeSet::new(),
            globs: None,
            pending: vec![],
        })
    }
    pub fn tree_mut(&mut self) -> &mut SourceTree {
        &mut self.tree
    }
    pub fn scan(&mut self) -> Result<SourceScanResult, ScanError> {
        self.discover()?;
        self.extract_pending()?;
        let files = self
            .files
            .iter()
            .filter_map(|path| path.to_str().map(str::to_owned))
            .collect();
        let globs = self.get_globs()?;
        Ok(SourceScanResult {
            candidates: self.candidates.iter().cloned().collect(),
            variables: self.variables.iter().cloned().collect(),
            files,
            globs,
            normalized_sources: self.get_normalized_sources(),
        })
    }
    fn discover(&mut self) -> Result<(), ScanError> {
        let discovery = discovery::discover(
            &self.tree,
            &self.sources,
            &self.explicit,
            &mut self.mtimes,
            None,
        )?;
        self.dirs.extend(discovery.dirs);
        for file in discovery.files {
            let extension = file
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or_default()
                .to_owned();
            self.pending
                .push((file.clone(), extension.clone(), extension == "css"));
            self.extensions.insert(extension);
            self.files.push(file);
        }
        Ok(())
    }
    fn commit(&mut self, results: impl IntoIterator<Item = ScanResult>) -> ScanResult {
        let mut candidates = BTreeSet::new();
        let mut variables = BTreeSet::new();
        for result in results {
            candidates.extend(result.candidates);
            variables.extend(result.variables);
        }
        candidates.retain(|candidate| !self.candidates.contains(candidate));
        variables.retain(|variable| !self.variables.contains(variable));
        self.candidates.extend(candidates.iter().cloned());
        self.variables.extend(variables.iter().cloned());
        ScanResult {
            candidates: candidates.into_iter().collect(),
            variables: variables.into_iter().collect(),
        }
    }
    fn extract_pending(&mut self) -> Result<ScanResult, ScanError> {
        let mut results = vec![];
        for (file, extension, css_only) in &self.pending {
            if let Some(bytes) = self.tree.read(file)? {
                results.push(extract(&bytes, extension, *css_only)?);
            }
        }
        self.pending.clear();
        Ok(self.commit(results))
    }
    pub fn scan_content(&mut self, changed: Vec<ChangedContent>) -> Result<ScanResult, ScanError> {
        let mut raw = vec![];
        let mut known = vec![];
        let mut unknown = BTreeMap::new();
        for item in changed {
            match item {
                ChangedContent::Content(content, extension) => {
                    raw.push(super::scan(&content, &extension))
                }
                ChangedContent::File(path, extension) => {
                    if let Some(path) = self.tree.canonicalize(&path)? {
                        if self.files.contains(&path) {
                            known.push((path, extension));
                        } else {
                            unknown.insert(path, extension);
                        }
                    }
                }
            }
        }
        if !unknown.is_empty() {
            let found = discovery::discover(
                &self.tree,
                &self.sources,
                &self.explicit,
                &mut self.mtimes,
                Some(unknown.keys().cloned().collect()),
            )?;
            for file in found.files {
                if let Some(extension) = unknown.remove(&file) {
                    self.files.push(file.clone());
                    known.push((file, extension));
                }
            }
        }
        for (file, extension) in known {
            if let Some(bytes) = self.tree.read(&file)? {
                raw.push(extract(&bytes, &extension, false)?);
            }
        }
        let queued = self.extract_pending()?;
        let new = self.commit(raw);
        let mut candidates = queued.candidates;
        candidates.extend(new.candidates);
        candidates.sort_unstable();
        let mut variables = queued.variables;
        variables.extend(new.variables);
        variables.sort_unstable();
        Ok(ScanResult {
            candidates,
            variables,
        })
    }
    pub fn get_files(&mut self) -> Result<Vec<String>, ScanError> {
        self.discover()?;
        let mut files = self
            .files
            .iter()
            .filter_map(|path| path.to_str().map(str::to_owned))
            .collect::<Vec<_>>();
        files.sort();
        Ok(files)
    }
    pub fn get_globs(&mut self) -> Result<Vec<GlobEntry>, ScanError> {
        if let Some(globs) = &self.globs {
            return Ok(globs.clone());
        }
        self.discover()?;
        let mut globs = vec![];
        for source in self.sources.iter() {
            match source {
                SourceEntry::Auto { base } | SourceEntry::External { base } => globs.extend(
                    discovery::resolve_globs(&self.tree, base, &self.dirs, &self.extensions)?,
                ),
                SourceEntry::Pattern { .. } => globs.push(source.glob()),
                _ => {}
            }
        }
        let globs = discovery::optimize_globs(&self.tree, globs)?;
        self.globs = Some(globs.clone());
        Ok(globs)
    }
    pub fn get_normalized_sources(&self) -> Vec<GlobEntry> {
        self.sources
            .iter()
            .filter(|source| !matches!(source, SourceEntry::Ignored { .. }))
            .map(SourceEntry::glob)
            .collect()
    }
    pub fn get_candidates_with_positions(
        &self,
        changed: ChangedContent,
    ) -> Result<Vec<(String, usize)>, ScanError> {
        use crate::extractor::{Extracted, Extractor};
        use bstr::ByteSlice;
        let (bytes, extension) = match changed {
            ChangedContent::Content(content, extension) => (content.into_bytes(), extension),
            ChangedContent::File(path, extension) => {
                (self.tree.read(&path)?.unwrap_or_default(), extension)
            }
        };
        let _ = std::str::from_utf8(&bytes).map_err(|_| ScanError::InvalidUtf8)?;
        let original = super::pre_process_input(&bytes, &extension);
        let replaced = original.replace("-[]", "XYZ");
        let offset = replaced.as_ptr() as usize;
        let mut result = vec![];
        for item in Extractor::new(&replaced).extract() {
            if let Extracted::Candidate(bytes) = item {
                let index = bytes.as_ptr() as usize - offset;
                if let Ok(candidate) = std::str::from_utf8(&original[index..index + bytes.len()]) {
                    result.push((candidate.to_owned(), index));
                }
            }
        }
        Ok(result)
    }
}

pub fn scan_sources(
    tree: SourceTree,
    sources: Vec<PublicSourceEntry>,
) -> Result<SourceScanResult, ScanError> {
    Scanner::new(sources, tree)?.scan()
}

fn extract(bytes: &[u8], extension: &str, css_only: bool) -> Result<ScanResult, ScanError> {
    use crate::extractor::{Extracted, Extractor};
    let content = std::str::from_utf8(bytes).map_err(|_| ScanError::InvalidUtf8)?;
    if !css_only {
        return Ok(super::scan(content, extension));
    }
    let processed = super::pre_process_input(bytes, extension);
    let mut variables = BTreeSet::new();
    for line in processed.split(|byte| *byte == b'\n') {
        for item in Extractor::new(line).extract_variables_from_css() {
            if let Extracted::CssVariable(bytes) = item {
                if let Ok(variable) = std::str::from_utf8(bytes) {
                    variables.insert(variable.to_owned());
                }
            }
        }
    }
    Ok(ScanResult {
        candidates: vec![],
        variables: variables.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::VirtualSourceTree;
    #[test]
    fn pending_source_and_raw_changes_return_sorted_delta() {
        let mut tree = VirtualSourceTree::new("/project");
        tree.insert_file("index.html", b"p-9".to_vec(), 1);
        let mut scanner = Scanner::new(
            vec![PublicSourceEntry {
                base: "/project".into(),
                pattern: "**/*".into(),
                negated: false,
            }],
            SourceTree::Virtual(tree),
        )
        .unwrap();
        scanner.get_files().unwrap();
        assert_eq!(
            scanner
                .scan_content(vec![ChangedContent::Content("flex".into(), "html".into())])
                .unwrap()
                .candidates,
            ["flex", "p-9"]
        );
    }
    #[test]
    fn missing_virtual_source_information_is_reported() {
        let mut tree = VirtualSourceTree::new("/project");
        tree.mark_unavailable("src");
        let result = Scanner::new(
            vec![PublicSourceEntry {
                base: "/project".into(),
                pattern: "src/**/*".into(),
                negated: false,
            }],
            SourceTree::Virtual(tree),
        );
        assert!(matches!(
            result,
            Err(ScanError::MissingSourceInformation(_))
        ));
    }
    #[test]
    fn invalid_source_utf8_returns_a_diagnostic() {
        let mut tree = VirtualSourceTree::new("/project");
        tree.insert_file("broken.vue", vec![255], 1);
        let mut scanner = Scanner::new(
            vec![PublicSourceEntry {
                base: "/project".into(),
                pattern: "**/*".into(),
                negated: false,
            }],
            SourceTree::Virtual(tree),
        )
        .unwrap();
        assert_eq!(scanner.scan(), Err(ScanError::InvalidUtf8));
        assert_eq!(scanner.scan(), Err(ScanError::InvalidUtf8));
    }
}
