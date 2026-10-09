//! Optional source inputs. Source discovery stays in Rust and never loads external resources.
use super::{Error, Session};
use rustify_style::{
    compiler::CompileError,
    protocol::{SourceContent, SourceSnapshot, VirtualSourceEntry},
    scanner::{self, ScanError, ScanResult, SourceTree, VirtualSourceTree},
    text::Text,
};
use std::{collections::BTreeSet, path::Path};

impl From<ScanError> for Error {
    fn from(error: ScanError) -> Self {
        Self::Scan {
            message: error.to_string(),
        }
    }
}

fn check(amount: usize, limit: usize, resource: &str) -> Result<(), Error> {
    if amount > limit {
        Err(CompileError::Limit {
            resource: resource.into(),
            limit,
        }
        .into())
    } else {
        Ok(())
    }
}

impl Session {
    fn check_scan_input(&self, input: &str) -> Result<(), Error> {
        if self.pending.is_some() {
            return Err(CompileError::PendingPrepared.into());
        }
        check(input.len(), self.limits.queued_bytes, "queued_bytes")
    }

    pub(super) fn prepare_content(&mut self, input: &str) -> Result<String, Error> {
        self.check_scan_input(input)?;
        let contents: Vec<SourceContent> = serde_json::from_str(input)?;
        check(
            contents.iter().fold(0usize, |size, source| {
                size.saturating_add(source.content.len())
                    .saturating_add(source.extension.len())
            }),
            self.limits.input_bytes,
            "input_bytes",
        )?;
        let mut candidates = BTreeSet::new();
        for source in contents {
            let result = scanner::scan(&source.content, &source.extension);
            self.merge_scan(&mut candidates, result)?;
        }
        self.prepare_candidates(&candidates.into_iter().collect::<Vec<_>>())
    }

    pub(super) fn prepare_sources(&mut self, input: &str) -> Result<String, Error> {
        self.check_scan_input(input)?;
        let snapshot: SourceSnapshot = serde_json::from_str(input)?;
        let mut bytes = snapshot.root.len().saturating_add(snapshot.base.len());
        let mut storage = std::mem::size_of::<VirtualSourceTree>();
        check(
            Path::new(&snapshot.root).components().count(),
            self.limits.ast_depth,
            "source_depth",
        )?;
        for ancestor in Path::new(&snapshot.root).ancestors() {
            storage = storage
                .saturating_add(ancestor.as_os_str().len())
                .saturating_add(std::mem::size_of::<scanner::VirtualEntry>())
                .saturating_add(std::mem::size_of::<std::path::PathBuf>())
                .saturating_add(8 * std::mem::size_of::<usize>());
        }
        // Parent directory insertion owns every path prefix. Charge them before
        // constructing the tree, including duplicates as a conservative bound.
        for entry in &snapshot.entries {
            let (path, extra) = match entry {
                VirtualSourceEntry::File { path, content, .. } => (path, content.len()),
                VirtualSourceEntry::Symlink { path, target } => (path, target.len()),
                VirtualSourceEntry::Directory { path }
                | VirtualSourceEntry::Unavailable { path } => (path, 0),
            };
            if let VirtualSourceEntry::Symlink { target, .. } = entry {
                check(
                    Path::new(target).components().count(),
                    self.limits.ast_depth,
                    "source_depth",
                )?;
            }
            bytes = bytes.saturating_add(path.len()).saturating_add(extra);
            check(
                Path::new(path).components().count(),
                self.limits.ast_depth,
                "source_depth",
            )?;
            for ancestor in Path::new(path).ancestors() {
                storage = storage
                    .saturating_add(ancestor.as_os_str().len())
                    .saturating_add(snapshot.root.len())
                    .saturating_add(std::mem::size_of::<scanner::VirtualEntry>())
                    .saturating_add(std::mem::size_of::<std::path::PathBuf>())
                    .saturating_add(8 * std::mem::size_of::<usize>());
            }
            storage = storage.saturating_add(extra);
        }
        check(bytes, self.limits.asset_bytes, "asset_bytes")?;
        check(
            storage.saturating_add(self.committed.metrics.cache_bytes),
            self.limits.cache_bytes,
            "cache_bytes",
        )?;
        let mut tree = VirtualSourceTree::new(snapshot.root);
        for entry in snapshot.entries {
            match entry {
                VirtualSourceEntry::Directory { path } => tree.insert_directory(path),
                VirtualSourceEntry::File {
                    path,
                    content,
                    revision,
                } => tree.insert_file(path, content.into_bytes(), revision as u128),
                VirtualSourceEntry::Symlink { path, target } => tree.insert_symlink(path, target),
                VirtualSourceEntry::Unavailable { path } => tree.mark_unavailable(path),
            }
        }
        let mut scanner = self
            .compiler
            .scanner(&snapshot.base, SourceTree::Virtual(tree))?;
        let result = scanner.scan()?;
        let mut candidates = BTreeSet::new();
        self.merge_scan(
            &mut candidates,
            ScanResult {
                candidates: result.candidates,
                variables: result.variables,
            },
        )?;
        // No source tree is retained after extraction. Candidate cache publication
        // remains controlled by the existing preparation/installation token.
        drop(scanner);
        self.prepare_candidates(&candidates.into_iter().collect::<Vec<_>>())
    }

    fn merge_scan(&self, candidates: &mut BTreeSet<Text>, result: ScanResult) -> Result<(), Error> {
        for candidate in result.candidates.into_iter().chain(result.variables) {
            // Scanner candidates come from its scalar UTF-8 schema; merge once into the
            // same canonical Text input used by explicit candidate JSON.
            candidates.insert(Text::from(candidate));
            check(
                candidates.len(),
                self.limits.candidate_count,
                "candidate_count",
            )?;
        }
        check(
            candidates
                .iter()
                .fold(0usize, |size, value| size.saturating_add(value.len())),
            self.limits.queued_bytes,
            "queued_bytes",
        )
    }
}
