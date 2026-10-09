//! Native and in-memory project sources used by the cn build pipeline.
use super::super::CnError;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};
use std::path::{Component, Path, PathBuf};
use std::rc::Rc;

#[derive(Clone)]
enum VirtualEntry {
    File(Vec<u8>),
    Directory,
    Symlink(String),
    Unreadable(bool),
}
/// Mutable virtual project files, including symbolic links and unreadable paths.
pub struct VirtualSources {
    entries: RefCell<BTreeMap<String, VirtualEntry>>,
}
impl Default for VirtualSources {
    fn default() -> Self {
        Self {
            entries: RefCell::new(BTreeMap::from([("/".into(), VirtualEntry::Directory)])),
        }
    }
}
pub type HostResolve = Rc<dyn Fn(&str, &str) -> Result<String, String>>;
pub type HostRead = Rc<dyn Fn(&str) -> Result<String, String>>;
pub type HostWrite = Rc<dyn Fn(&str, &str) -> Result<(), String>>;
pub type HostKind = Rc<dyn Fn(&str) -> Result<Kind, String>>;
pub type HostChildren = Rc<dyn Fn(&str) -> Result<Vec<(String, Kind)>, String>>;
/// Concrete synchronous Node filesystem ports. Discovery stays in Rust.
pub struct HostSources {
    pub cwd: Rc<dyn Fn() -> Result<String, String>>,
    pub resolve: HostResolve,
    pub relative: HostResolve,
    pub dirname: HostRead,
    pub read_text: HostRead,
    pub write_text: HostWrite,
    pub realpath: HostRead,
    pub kind: HostKind,
    pub children: HostChildren,
}
/// Build sources use native, virtual, or explicit host filesystem primitives.
pub enum BuildSourceTree {
    Native,
    Virtual(VirtualSources),
    Host(HostSources),
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    File,
    Directory,
    Symlink,
    Other,
}

pub(crate) fn resolve(base: &str, path: &str) -> String {
    let path = Path::new(path);
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        Path::new(base).join(path)
    };
    let mut normalized = PathBuf::new();
    for part in joined.components() {
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            _ => normalized.push(part.as_os_str()),
        }
    }
    normalized.to_string_lossy().into_owned()
}
pub(crate) fn parent(path: &str) -> &str {
    Path::new(path)
        .parent()
        .and_then(Path::to_str)
        .unwrap_or("")
}
fn error(path: &str, operation: &str, code: &str) -> String {
    let description = match code {
        "ENOENT" => "no such file or directory",
        "EACCES" => "permission denied",
        "EISDIR" => "illegal operation on a directory",
        "ENOTDIR" => "not a directory",
        "ELOOP" => "too many symbolic links encountered",
        _ => "input/output error",
    };
    format!("{code}: {description}, {operation} '{path}'")
}
fn native_error(path: &str, operation: &str, err: std::io::Error) -> String {
    let code = match err.kind() {
        std::io::ErrorKind::NotFound => "ENOENT",
        std::io::ErrorKind::PermissionDenied => "EACCES",
        std::io::ErrorKind::IsADirectory => "EISDIR",
        std::io::ErrorKind::NotADirectory => "ENOTDIR",
        _ => return err.to_string(),
    };
    error(path, operation, code)
}
impl BuildSourceTree {
    pub fn resolve_path(&self, base: &str, path: &str) -> Result<String, CnError> {
        match self {
            Self::Host(host) => (host.resolve)(base, path).map_err(CnError::new),
            _ => Ok(resolve(base, path)),
        }
    }
    pub fn parent_path(&self, path: &str) -> Result<String, CnError> {
        match self {
            Self::Host(host) => (host.dirname)(path).map_err(CnError::new),
            _ => Ok(parent(path).into()),
        }
    }
    pub fn relative_path(&self, base: &str, path: &str) -> Result<String, CnError> {
        match self {
            Self::Host(host) => (host.relative)(base, path).map_err(CnError::new),
            _ => {
                let abs = resolve(base, path);
                let prefix = if base.ends_with('/') {
                    base.to_owned()
                } else {
                    format!("{base}/")
                };
                Ok(if abs == base {
                    String::new()
                } else {
                    abs.strip_prefix(&prefix).unwrap_or("..").into()
                })
            }
        }
    }
    /// Construct a virtual tree from UTF-8 files. Parent directories are created.
    pub fn virtual_files<'a>(files: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
        let tree = Self::Virtual(VirtualSources::default());
        for (path, text) in files {
            tree.put_file(path, text.as_bytes());
        }
        tree
    }
    /// Add or replace a virtual file. Native writes use `write_text`.
    pub fn put_file(&self, path: &str, bytes: impl AsRef<[u8]>) {
        if let Self::Virtual(tree) = self {
            tree.insert(path, VirtualEntry::File(bytes.as_ref().to_vec()));
        }
    }
    pub fn put_directory(&self, path: &str) {
        if let Self::Virtual(tree) = self {
            tree.insert(path, VirtualEntry::Directory);
        }
    }
    pub fn put_symlink(&self, path: &str, target: &str) {
        if let Self::Virtual(tree) = self {
            tree.insert(path, VirtualEntry::Symlink(target.into()));
        }
    }
    pub fn put_unreadable(&self, path: &str, directory: bool) {
        if let Self::Virtual(tree) = self {
            tree.insert(path, VirtualEntry::Unreadable(directory));
        }
    }
    pub fn remove(&self, path: &str) {
        if let Self::Virtual(tree) = self {
            let path = resolve("/", path);
            tree.entries
                .borrow_mut()
                .retain(|key, _| key != &path && !key.starts_with(&(path.clone() + "/")));
        }
    }
    pub fn cwd(&self, cwd: &str) -> Result<String, CnError> {
        let base = match self {
            Self::Native => std::env::current_dir()
                .map_err(|err| CnError::new(err.to_string()))?
                .to_string_lossy()
                .into_owned(),
            Self::Virtual(_) => "/".into(),
            Self::Host(host) => (host.cwd)().map_err(CnError::new)?,
        };
        self.resolve_path(&base, cwd)
    }
    /// Read with Node-compatible lossy UTF-8 decoding and error messages.
    pub fn read_text(&self, path: &str) -> Result<String, String> {
        let bytes = match self {
            Self::Host(host) => return (host.read_text)(path),
            Self::Native => std::fs::read(path).map_err(|err| native_error(path, "open", err))?,
            Self::Virtual(tree) => {
                let real = tree
                    .realpath(path)
                    .map_err(|err| err.replace(", realpath '", ", open '"))?;
                match tree.entries.borrow().get(&real) {
                    Some(VirtualEntry::File(bytes)) => bytes.clone(),
                    Some(VirtualEntry::Unreadable(_)) => return Err(error(path, "open", "EACCES")),
                    _ => return Err(error(path, "read", "EISDIR")),
                }
            }
        };
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
    /// Write an output, creating its parent directories. Returns host IO errors.
    pub fn write_text(&self, path: &str, source: &str) -> Result<(), String> {
        match self {
            Self::Host(host) => (host.write_text)(path, source),
            Self::Native => {
                std::fs::create_dir_all(parent(path))
                    .map_err(|err| native_error(parent(path), "mkdir", err))?;
                std::fs::write(path, source).map_err(|err| native_error(path, "open", err))
            }
            Self::Virtual(tree) => {
                // Existing output symlinks are followed like writeFileSync.
                let target = tree
                    .resolve_links(path)
                    .map_err(|err| err.replace(", realpath '", ", open '"))?;
                if matches!(
                    tree.entries.borrow().get(&target),
                    Some(VirtualEntry::Unreadable(_))
                ) {
                    return Err(error(path, "open", "EACCES"));
                }
                if matches!(
                    tree.entries.borrow().get(&target),
                    Some(VirtualEntry::Directory)
                ) {
                    return Err(error(path, "open", "EISDIR"));
                }
                let mut ancestor = parent(&target);
                while !ancestor.is_empty() {
                    match tree.entries.borrow().get(ancestor) {
                        Some(VirtualEntry::Unreadable(_)) => {
                            return Err(error(ancestor, "mkdir", "EACCES"));
                        }
                        Some(VirtualEntry::File(_)) => {
                            return Err(error(ancestor, "mkdir", "ENOTDIR"));
                        }
                        _ => {}
                    }
                    ancestor = parent(ancestor);
                }
                tree.insert(&target, VirtualEntry::File(source.as_bytes().to_vec()));
                Ok(())
            }
        }
    }
    pub fn realpath(&self, path: &str) -> Result<String, String> {
        match self {
            Self::Native => std::fs::canonicalize(path)
                .map(|p| p.to_string_lossy().into_owned())
                .map_err(|err| native_error(path, "realpath", err)),
            Self::Virtual(tree) => tree.realpath(path),
            Self::Host(host) => (host.realpath)(path),
        }
    }
    pub fn kind(&self, path: &str) -> Result<Kind, String> {
        match self {
            Self::Host(host) => (host.kind)(path),
            Self::Native => std::fs::metadata(path)
                .map(|st| {
                    if st.is_dir() {
                        Kind::Directory
                    } else if st.is_file() {
                        Kind::File
                    } else {
                        Kind::Other
                    }
                })
                .map_err(|err| native_error(path, "stat", err)),
            Self::Virtual(tree) => {
                let path = tree.realpath(path)?;
                tree.entries
                    .borrow()
                    .get(&path)
                    .map(|entry| match entry {
                        VirtualEntry::Directory | VirtualEntry::Unreadable(true) => Kind::Directory,
                        _ => Kind::File,
                    })
                    .ok_or_else(|| error(&path, "stat", "ENOENT"))
            }
        }
    }
    pub fn children(&self, path: &str) -> Result<Vec<(String, Kind)>, String> {
        let mut children = match self {
            Self::Host(host) => return (host.children)(path),
            Self::Native => std::fs::read_dir(path)
                .map_err(|err| native_error(path, "scandir", err))?
                .map(|item| {
                    let item = item.map_err(|err| native_error(path, "scandir", err))?;
                    let kind = item
                        .file_type()
                        .map_err(|err| native_error(path, "stat", err))?;
                    Ok((
                        item.file_name().to_string_lossy().into_owned(),
                        if kind.is_symlink() {
                            Kind::Symlink
                        } else if kind.is_dir() {
                            Kind::Directory
                        } else if kind.is_file() {
                            Kind::File
                        } else {
                            Kind::Other
                        },
                    ))
                })
                .collect::<Result<Vec<_>, String>>()?,
            Self::Virtual(tree) => {
                let real = tree.realpath(path)?;
                let entries = tree.entries.borrow();
                if matches!(entries.get(&real), Some(VirtualEntry::Unreadable(_))) {
                    return Err(error(path, "scandir", "EACCES"));
                }
                if !matches!(entries.get(&real), Some(VirtualEntry::Directory)) {
                    return Err(error(path, "scandir", "ENOTDIR"));
                }
                entries
                    .iter()
                    .filter(|(key, _)| key.as_str() != real && parent(key) == real)
                    .map(|(key, value)| {
                        (
                            Path::new(key)
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .into_owned(),
                            match value {
                                VirtualEntry::Directory | VirtualEntry::Unreadable(true) => {
                                    Kind::Directory
                                }
                                VirtualEntry::Symlink(_) => Kind::Symlink,
                                _ => Kind::File,
                            },
                        )
                    })
                    .collect()
            }
        };
        children.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(children)
    }
}
impl VirtualSources {
    fn insert(&self, path: &str, entry: VirtualEntry) {
        let path = resolve("/", path);
        let mut entries = self.entries.borrow_mut();
        let mut dir = parent(&path);
        while !dir.is_empty() {
            entries
                .entry(dir.to_owned())
                .or_insert(VirtualEntry::Directory);
            dir = parent(dir);
        }
        entries.insert(path, entry);
    }
    fn resolve_links(&self, path: &str) -> Result<String, String> {
        let mut current = resolve("/", path);
        let entries = self.entries.borrow();
        let mut visited = HashSet::new();
        let mut followed_links = 0;
        loop {
            if !visited.insert(current.clone()) {
                return Err(error(path, "realpath", "ELOOP"));
            }
            let mut prefix = String::new();
            let mut rewritten = None;
            for part in current.split('/').filter(|p| !p.is_empty()) {
                prefix.push('/');
                prefix.push_str(part);
                if matches!(entries.get(&prefix), Some(VirtualEntry::Unreadable(true)))
                    && prefix != current
                {
                    return Err(error(path, "realpath", "EACCES"));
                }
                if let Some(VirtualEntry::Symlink(target)) = entries.get(&prefix) {
                    // Bound growing symlink rewrites that never repeat a whole
                    // path; forty follows is the conventional POSIX limit.
                    followed_links += 1;
                    if followed_links > 40 {
                        return Err(error(path, "realpath", "ELOOP"));
                    }
                    let target = resolve(parent(&prefix), target);
                    rewritten = Some(resolve(
                        &target,
                        current[prefix.len()..].trim_start_matches('/'),
                    ));
                    break;
                }
            }
            match rewritten {
                Some(path) => current = path,
                None => break,
            }
        }
        Ok(current)
    }
    fn realpath(&self, path: &str) -> Result<String, String> {
        let current = self.resolve_links(path)?;
        if self.entries.borrow().contains_key(&current) {
            Ok(current)
        } else {
            Err(error(path, "realpath", "ENOENT"))
        }
    }
}
