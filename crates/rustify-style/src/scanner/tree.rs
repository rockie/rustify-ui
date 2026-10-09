use super::ScanError;
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug)]
pub enum VirtualEntry {
    /// A known source path whose metadata/content was not supplied.
    Unavailable,
    Directory,
    File {
        content: Vec<u8>,
        revision: u128,
    },
    Symlink {
        target: PathBuf,
    },
}

/// A complete snapshot of files, directories and symlinks rooted at `root`.
/// Parent ignore files and repository markers must be included when applicable.
#[derive(Clone, Debug)]
pub struct VirtualSourceTree {
    root: PathBuf,
    entries: BTreeMap<PathBuf, VirtualEntry>,
}

impl VirtualSourceTree {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = normalize(&root.into());
        let mut tree = Self {
            root: root.clone(),
            entries: BTreeMap::new(),
        };
        tree.insert_directory(root);
        tree
    }
    pub fn insert_directory(&mut self, path: impl AsRef<Path>) {
        let path = self.absolute(path.as_ref());
        for ancestor in path.ancestors() {
            self.entries
                .entry(ancestor.to_path_buf())
                .or_insert(VirtualEntry::Directory);
        }
    }
    pub fn insert_file(&mut self, path: impl AsRef<Path>, content: Vec<u8>, revision: u128) {
        let path = self.absolute(path.as_ref());
        if let Some(parent) = path.parent() {
            self.insert_directory(parent);
        }
        self.entries
            .insert(path, VirtualEntry::File { content, revision });
    }
    pub fn insert_symlink(&mut self, path: impl AsRef<Path>, target: impl Into<PathBuf>) {
        let path = self.absolute(path.as_ref());
        if let Some(parent) = path.parent() {
            self.insert_directory(parent);
        }
        self.entries.insert(
            path,
            VirtualEntry::Symlink {
                target: target.into(),
            },
        );
    }
    /// Mark missing information explicitly instead of treating it as an absent path.
    pub fn mark_unavailable(&mut self, path: impl AsRef<Path>) {
        let path = self.absolute(path.as_ref());
        if let Some(parent) = path.parent() {
            self.insert_directory(parent);
        }
        self.entries.insert(path, VirtualEntry::Unavailable);
    }
    pub fn remove(&mut self, path: impl AsRef<Path>) {
        let path = self.absolute(path.as_ref());
        self.entries.retain(|key, _| !key.starts_with(&path));
    }
    fn absolute(&self, path: &Path) -> PathBuf {
        normalize(&if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        })
    }
    fn canonicalize(&self, path: &Path) -> Result<Option<PathBuf>, ScanError> {
        let mut path = self.absolute(path);
        for _ in 0..40 {
            let mut replaced = false;
            let mut current = PathBuf::new();
            for component in path.components() {
                current.push(component.as_os_str());
                if matches!(self.entries.get(&current), Some(VirtualEntry::Unavailable)) {
                    return Err(ScanError::MissingSourceInformation(
                        current.to_string_lossy().into_owned(),
                    ));
                }
                if let Some(VirtualEntry::Symlink { target }) = self.entries.get(&current) {
                    let base = current.parent().unwrap_or(Path::new("/"));
                    let target = if target.is_absolute() {
                        target.clone()
                    } else {
                        base.join(target)
                    };
                    path = normalize(
                        &target.join(path.strip_prefix(&current).unwrap_or(Path::new(""))),
                    );
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                return Ok(self.entries.contains_key(&path).then_some(path));
            }
        }
        Ok(None)
    }
}

#[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
#[derive(Clone, Debug, Default)]
pub struct NativeSourceTree;

/// Concrete source storage; source selection is shared by both variants.
#[derive(Clone, Debug)]
pub enum SourceTree {
    Virtual(VirtualSourceTree),
    #[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
    Native(NativeSourceTree),
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Metadata {
    pub is_dir: bool,
    pub is_file: bool,
    pub revision: Option<u128>,
}

impl SourceTree {
    pub(crate) fn canonicalize(&self, path: &Path) -> Result<Option<PathBuf>, ScanError> {
        match self {
            Self::Virtual(tree) => tree.canonicalize(path),
            #[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
            Self::Native(_) => Ok(std::fs::canonicalize(path).ok()),
        }
    }
    pub(crate) fn metadata(&self, path: &Path) -> Result<Option<Metadata>, ScanError> {
        match self {
            Self::Virtual(tree) => {
                let Some(path) = tree.canonicalize(path)? else {
                    return Ok(None);
                };
                Ok(tree.entries.get(&path).map(|entry| match entry {
                    VirtualEntry::Directory => Metadata {
                        is_dir: true,
                        is_file: false,
                        revision: None,
                    },
                    VirtualEntry::File { revision, .. } => Metadata {
                        is_dir: false,
                        is_file: true,
                        revision: Some(*revision),
                    },
                    VirtualEntry::Symlink { .. } | VirtualEntry::Unavailable => unreachable!(),
                }))
            }
            #[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
            Self::Native(_) => Ok(std::fs::metadata(path).ok().map(|metadata| Metadata {
                is_dir: metadata.is_dir(),
                is_file: metadata.is_file(),
                revision: metadata
                    .modified()
                    .ok()
                    .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|time| time.as_nanos()),
            })),
        }
    }
    pub(crate) fn read(&self, path: &Path) -> Result<Option<Vec<u8>>, ScanError> {
        match self {
            Self::Virtual(tree) => {
                let Some(path) = tree.canonicalize(path)? else {
                    return Ok(None);
                };
                Ok(match tree.entries.get(&path) {
                    Some(VirtualEntry::File { content, .. }) => Some(content.clone()),
                    _ => None,
                })
            }
            #[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
            Self::Native(_) => Ok(std::fs::read(path).ok()),
        }
    }
    pub(crate) fn children(
        &self,
        path: &Path,
        follow_links: bool,
    ) -> Result<Vec<PathBuf>, ScanError> {
        match self {
            Self::Virtual(tree) => {
                let Some(actual) = tree.canonicalize(path)? else {
                    return Ok(vec![]);
                };
                if !follow_links && actual != tree.absolute(path) {
                    return Ok(vec![]);
                }
                Ok(tree
                    .entries
                    .keys()
                    .filter(|key| key.parent() == Some(actual.as_path()))
                    .filter_map(|key| key.file_name().map(|name| path.join(name)))
                    .collect())
            }
            #[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
            Self::Native(_) => {
                if !follow_links
                    && std::fs::symlink_metadata(path)
                        .is_ok_and(|metadata| metadata.file_type().is_symlink())
                {
                    return Ok(vec![]);
                }
                Ok(std::fs::read_dir(path)
                    .ok()
                    .into_iter()
                    .flatten()
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .collect())
            }
        }
    }
    pub fn virtual_mut(&mut self) -> Option<&mut VirtualSourceTree> {
        match self {
            Self::Virtual(tree) => Some(tree),
            #[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
            Self::Native(_) => None,
        }
    }
}

pub(crate) fn normalize(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                result.pop();
            }
            _ => result.push(component.as_os_str()),
        }
    }
    result
}
