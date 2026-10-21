//! Writing a production build into its output directory, and nothing else.
//!
//! A build never edits its output in place. It writes the whole new site
//! into a staging directory, then swaps: the old output moves to a backup,
//! the staging directory becomes the output, and only then is the backup
//! removed. Each step is recorded before it is taken, so a build killed
//! half-way is finished or undone by the next one, and a failed build never
//! costs the site that was there. The work area is private to this tool,
//! `<output's parent>/.rustify-docs/<output name>-<hash>/`, on the output's
//! file system so the swap is two renames. It is not atomic for a server
//! reading the output during those renames; it is recoverable.
//!
//! One output has one writer: a build holds the area's lock exclusively and
//! a preview holds it shared, and neither waits for the other. A directory
//! that is not a previous build's output is never replaced.

use crate::diagnostic_text::{code_span, literal};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, TryLockError};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::generation::MANIFEST_FILE;

/// The only mode a published output has; `dev` keeps its site in memory.
pub const PRODUCTION: &str = "production";

/// What a production output says about itself: written last, read by
/// `preview` before it serves anything. It holds no time, host path or
/// environment value, so two builds of one input write the same bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: u32,
    pub mode: String,
    pub base: String,
    pub cli_version: String,
    /// The digest of the file list below.
    pub build_id: String,
    pub files: Vec<ManifestFile>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub locale_not_found: BTreeMap<String, String>,
    /// The header policy of each landing page served under another than
    /// [`crate::shell::CSP`], by its listed path; every other file is served
    /// under that one.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub page_policies: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestFile {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

impl Manifest {
    /// The manifest of `files` (which must not hold the manifest itself).
    pub fn of(files: &BTreeMap<String, Arc<[u8]>>, base: &str, cli_version: &str) -> Self {
        let mut list = Vec::with_capacity(files.len());
        let mut whole = Sha256::new();
        for (path, bytes) in files {
            let digest: String = Sha256::digest(bytes)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            whole.update(path.as_bytes());
            whole.update([0]);
            whole.update(digest.as_bytes());
            whole.update(b"\n");
            list.push(ManifestFile {
                path: path.clone(),
                sha256: digest,
                bytes: bytes.len() as u64,
            });
        }
        let build_id: String = whole.finalize()[..8]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        Self {
            schema_version: 1,
            mode: PRODUCTION.into(),
            base: base.into(),
            cli_version: cli_version.into(),
            build_id,
            files: list,
            locale_not_found: BTreeMap::new(),
            page_policies: BTreeMap::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut text = serde_json::to_string_pretty(self).expect("a manifest serializes");
        text.push('\n');
        text.into_bytes()
    }

    /// Reads the manifest of the output at `dir`.
    pub fn read(dir: &Path) -> Result<Self, String> {
        let path = dir.join(MANIFEST_FILE);
        let text = fs::read_to_string(&path).map_err(|error| match error.kind() {
            io::ErrorKind::NotFound => format!(
                "{} has no {MANIFEST_FILE}; it is not a production build",
                literal(&dir.display().to_string())
            ),
            _ => format!(
                "cannot read {}: {error}",
                literal(&path.display().to_string())
            ),
        })?;
        serde_json::from_str(&text).map_err(|error| {
            format!(
                "{} is damaged: {error}",
                literal(&path.display().to_string())
            )
        })
    }

    /// Validates and freezes this output's explicitly declared language errors.
    pub fn locale_not_found_files(&self, dir: &Path) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let damaged = |message: String| {
            format!("invalid locale_not_found: {message}; run `rustify-docs build` again")
        };
        if self.locale_not_found.is_empty() {
            return Ok(BTreeMap::new());
        }
        if self.locale_not_found.get("").map(String::as_str) != Some("404.html") {
            return Err(damaged(
                "the default entry must be an empty prefix mapped to 404.html".into(),
            ));
        }
        let root = fs::canonicalize(dir)
            .map_err(|error| damaged(format!("cannot read output: {error}")))?;
        let listed: BTreeSet<_> = self.files.iter().map(|file| file.path.as_str()).collect();
        let mut frozen = BTreeMap::new();
        for (prefix, path) in &self.locale_not_found {
            let expected = if prefix.is_empty() {
                "404.html".to_owned()
            } else {
                if !crate::config::valid_locale_prefix(prefix) {
                    return Err(damaged(format!(
                        "{} is not a safe language prefix",
                        code_span(prefix)
                    )));
                }
                format!("{prefix}/404.html")
            };
            if path != &expected || !listed.contains(path.as_str()) {
                return Err(damaged(format!(
                    "{} must name the listed file {}",
                    code_span(prefix),
                    code_span(&expected)
                )));
            }
            let real = fs::canonicalize(root.join(path))
                .map_err(|error| damaged(format!("cannot read {}: {error}", code_span(path))))?;
            if !real.starts_with(&root) || !real.is_file() {
                return Err(damaged(format!(
                    "{} is not a regular file inside the output",
                    code_span(path)
                )));
            }
            let bytes = fs::read(real)
                .map_err(|error| damaged(format!("cannot read {}: {error}", code_span(path))))?;
            frozen.insert(prefix.clone(), bytes);
        }
        Ok(frozen)
    }

    /// Validates this output's per-page policies before any is sent as a
    /// header: each names a listed page and is one printable line that
    /// starts and ends as every policy the build writes does.
    pub fn checked_page_policies(&self) -> Result<BTreeMap<String, String>, String> {
        let damaged = |message: String| {
            format!("invalid page_policies: {message}; run `rustify-docs build` again")
        };
        let listed: BTreeSet<_> = self.files.iter().map(|file| file.path.as_str()).collect();
        for (path, policy) in &self.page_policies {
            if !path.ends_with(".html") || !listed.contains(path.as_str()) {
                return Err(damaged(format!(
                    "{} is not a page the output lists",
                    code_span(path)
                )));
            }
            if !policy.bytes().all(|byte| (0x20..0x7f).contains(&byte))
                || !policy.starts_with("default-src 'none'; ")
                || !policy.ends_with("; frame-ancestors 'none'")
            {
                return Err(damaged(format!(
                    "the policy of {} is not one the build writes",
                    code_span(path)
                )));
            }
        }
        Ok(self.page_policies.clone())
    }
}

/// Adds the manifest of `files` to them.
pub fn add_manifest(
    files: &mut BTreeMap<String, Arc<[u8]>>,
    base: &str,
    cli_version: &str,
    locale_not_found: &BTreeMap<String, String>,
    page_policies: &BTreeMap<String, String>,
) {
    let mut manifest = Manifest::of(files, base, cli_version);
    manifest.locale_not_found = locale_not_found.clone();
    manifest.page_policies = page_policies.clone();
    files.insert(MANIFEST_FILE.to_owned(), Arc::from(manifest.to_bytes()));
}

/// An output directory and its private work area.
#[derive(Clone, Debug)]
pub struct Area {
    /// The output, absolute.
    pub out: PathBuf,
    /// `<out's parent>/.rustify-docs/<name>-<hash>/`.
    pub dir: PathBuf,
}

impl Area {
    /// The area of `out` (absolute; it need not exist, its parent must).
    pub fn of(out: &Path) -> Result<Self, String> {
        let name = out.file_name().ok_or_else(|| {
            format!(
                "{} is not a directory name",
                literal(&out.display().to_string())
            )
        })?;
        let parent = out.parent().ok_or_else(|| {
            format!(
                "{} has no parent directory",
                literal(&out.display().to_string())
            )
        })?;
        let parent = fs::canonicalize(parent)
            .map_err(|error| format!("{}: {error}", literal(&parent.display().to_string())))?;
        let out = parent.join(name);
        let label: String = name
            .to_string_lossy()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let id = crate::generation::fingerprint(out.as_os_str().as_encoded_bytes());
        let dir = parent.join(".rustify-docs").join(format!("{label}-{id}"));
        Ok(Self { out, dir })
    }

    fn lock_path(&self) -> PathBuf {
        self.dir.join("lock")
    }
    fn journal(&self) -> PathBuf {
        self.dir.join("transaction")
    }
    fn staging(&self) -> PathBuf {
        self.dir.join("staging")
    }
    fn backup(&self) -> PathBuf {
        self.dir.join("backup")
    }

    /// Whether a build stopped half-way and has not been recovered.
    pub fn unfinished(&self) -> bool {
        self.journal().exists()
    }
}

/// Why an output could not be locked.
#[derive(Debug)]
pub enum LockError {
    /// Another build or a preview holds it.
    Busy,
    Io(String),
}

/// A held lock on an output; dropping it releases the lock.
#[derive(Debug)]
pub struct Lock {
    _file: File,
    pub area: Area,
    exclusive: bool,
}

/// Locks `area`: exclusively for a build, shared for a preview. Never waits.
pub fn lock(area: &Area, exclusive: bool) -> Result<Lock, LockError> {
    fs::create_dir_all(&area.dir)
        .map_err(|e| LockError::Io(format!("{}: {e}", literal(&area.dir.display().to_string()))))?;
    let file = File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(area.lock_path())
        .map_err(|e| {
            LockError::Io(format!(
                "{}: {e}",
                literal(&area.lock_path().display().to_string())
            ))
        })?;
    let taken = if exclusive {
        file.try_lock()
    } else {
        file.try_lock_shared()
    };
    match taken {
        Ok(()) => Ok(Lock {
            _file: file,
            area: area.clone(),
            exclusive,
        }),
        Err(TryLockError::WouldBlock) => Err(LockError::Busy),
        Err(TryLockError::Error(error)) => Err(LockError::Io(error.to_string())),
    }
}

fn remove_tree(path: &Path) -> io::Result<()> {
    match fs::remove_dir_all(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

fn io_error(what: &str, path: &Path, error: io::Error) -> String {
    format!("{what} {}: {error}", literal(&path.display().to_string()))
}

/// What recovering an interrupted build did.
#[derive(Debug, PartialEq, Eq)]
pub enum Recovery {
    /// Nothing was interrupted.
    Clean,
    /// A build stopped before its output was swapped in; it was dropped.
    Discarded,
    /// A build stopped between the two renames; the old output is back.
    Restored,
    /// A build had swapped its output in; its clean-up was finished.
    Completed,
}

/// Finishes or undoes a build that stopped half-way. Needs the exclusive
/// lock, so it never runs while another build or a preview is active.
pub fn recover(lock: &Lock) -> Result<Recovery, String> {
    assert!(lock.exclusive, "recovery needs the exclusive lock");
    let area = &lock.area;
    let journal = area.journal();
    let staging = area.staging();
    let backup = area.backup();
    let step = match fs::read_to_string(&journal) {
        Ok(step) => step,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            // No transaction: anything left in the area is debris of one
            // that was finishing its clean-up.
            remove_tree(&staging).map_err(|e| io_error("removing", &staging, e))?;
            remove_tree(&backup).map_err(|e| io_error("removing", &backup, e))?;
            return Ok(Recovery::Clean);
        }
        Err(error) => return Err(io_error("reading", &journal, error)),
    };
    let outcome = if step.trim() == "swap" && !staging.exists() {
        // The new output was renamed into place.
        if !area.out.exists() && backup.exists() {
            fs::rename(&backup, &area.out).map_err(|e| io_error("restoring", &area.out, e))?;
            Recovery::Restored
        } else {
            remove_tree(&backup).map_err(|e| io_error("removing", &backup, e))?;
            Recovery::Completed
        }
    } else if !area.out.exists() && backup.exists() {
        // The old output was moved aside and the new one never arrived.
        fs::rename(&backup, &area.out).map_err(|e| io_error("restoring", &area.out, e))?;
        remove_tree(&staging).map_err(|e| io_error("removing", &staging, e))?;
        Recovery::Restored
    } else {
        remove_tree(&staging).map_err(|e| io_error("removing", &staging, e))?;
        remove_tree(&backup).map_err(|e| io_error("removing", &backup, e))?;
        Recovery::Discarded
    };
    fs::remove_file(&journal).map_err(|e| io_error("removing", &journal, e))?;
    Ok(outcome)
}

/// Whether `out` (absolute) may be a project's output: not the project, a
/// directory holding it, or one that overlaps its content or asset
/// directory, all of which a build would replace whole.
pub fn check_out(project: &crate::content::Project, out: &Path) -> Result<(), String> {
    let normal = |path: &Path| -> PathBuf {
        // The longest existing ancestor, resolved, with the rest appended.
        let mut existing = path.to_path_buf();
        let mut rest = Vec::new();
        while !existing.exists() {
            match (existing.file_name(), existing.parent()) {
                (Some(name), Some(parent)) => {
                    rest.push(name.to_owned());
                    existing = parent.to_path_buf();
                }
                _ => break,
            }
        }
        let mut resolved = fs::canonicalize(&existing).unwrap_or(existing);
        for name in rest.into_iter().rev() {
            resolved.push(name);
        }
        resolved
    };
    let out = normal(out);
    let root = normal(&project.root);
    if out.components().any(|part| {
        matches!(part, std::path::Component::Normal(name) if name == ".git" || name == ".rustify-docs")
    }) {
        return Err(format!(
            "the output {} is inside a repository's or this tool's own directory",
            literal(&out.display().to_string())
        ));
    }
    let overlaps = |a: &Path, b: &Path| a.starts_with(b) || b.starts_with(a);
    if root.starts_with(&out) {
        return Err(format!(
            "the output {} would hold the project; a build replaces its output whole",
            literal(&out.display().to_string())
        ));
    }
    for (dir, what) in [
        (&project.config.content_dir, "content directory"),
        (&project.config.assets_dir, "asset directory"),
    ] {
        let dir = normal(&project.root.join(dir));
        if overlaps(&out, &dir) {
            return Err(format!(
                "the output {} overlaps the {what} {}; a build replaces its output whole",
                literal(&out.display().to_string()),
                literal(&dir.display().to_string())
            ));
        }
    }
    let messages: BTreeSet<_> = project
        .config
        .locales
        .iter()
        .filter_map(|locale| locale.messages.as_ref().map(|(path, _)| path))
        .collect();
    for path in messages {
        let input = normal(&project.root.join(path));
        if overlaps(&out, &input) {
            return Err(format!(
                "the output {} overlaps the messages input {}; a build replaces its output whole",
                literal(&out.display().to_string()),
                literal(&input.display().to_string())
            ));
        }
    }
    Ok(())
}

/// Whether `out` may be replaced: it does not exist, is empty, or holds a
/// previous production build (a manifest that reads as one). A directory
/// whose owner cannot be confirmed, a damaged manifest included, is never
/// replaced; nor is a symbolic link, whose target a swap would not reach.
pub fn replaceable(out: &Path) -> Result<(), String> {
    if fs::symlink_metadata(out).is_ok_and(|meta| meta.file_type().is_symlink()) {
        return Err(format!(
            "{} is a symbolic link; give the directory it points to as the output",
            literal(&out.display().to_string())
        ));
    }
    match fs::read_dir(out) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io_error("reading", out, error)),
        Ok(mut entries) => {
            if entries.next().is_none() {
                return Ok(());
            }
            if !out.join(MANIFEST_FILE).is_file() {
                return Err(format!(
                    "{} holds files that no build wrote (it has no {MANIFEST_FILE}); it is not replaced",
                    literal(&out.display().to_string())
                ));
            }
            match Manifest::read(out) {
                Ok(manifest) if manifest.mode == PRODUCTION => Ok(()),
                Ok(manifest) => Err(format!("{} holds a {} output, not a production build; it is not replaced", literal(&out.display().to_string()), code_span(&manifest.mode))),
                Err(message) => Err(format!(
                    "{message}; whatever wrote it, the directory is not replaced: remove it and build again"
                )),
            }
        }
    }
}

/// A point in [`publish_with`] at which the old output is in a known state:
/// what a crash right there must leave recoverable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// The new site is written into the staging directory; the old output
    /// is untouched.
    Staged,
    /// The old output has been moved aside; nothing is at the output path.
    MovedAside,
    /// The new site is the output; the old one is still in the backup.
    SwappedIn,
}

impl Step {
    pub fn name(self) -> &'static str {
        match self {
            Step::Staged => "staged",
            Step::MovedAside => "moved-aside",
            Step::SwappedIn => "swapped-in",
        }
    }
}

/// Writes `files` as the output of `lock`'s area, replacing what was there
/// only once all of it is written. Call [`recover`] first.
pub fn publish(lock: &Lock, files: &BTreeMap<String, Arc<[u8]>>) -> Result<(), String> {
    publish_with(lock, files, &mut |_| {})
}

/// [`publish`], telling `at` each [`Step`] as it is reached; how the
/// recovery of a build stopped at each one is tested.
pub fn publish_with(
    lock: &Lock,
    files: &BTreeMap<String, Arc<[u8]>>,
    at: &mut dyn FnMut(Step),
) -> Result<(), String> {
    assert!(lock.exclusive, "publishing needs the exclusive lock");
    let area = &lock.area;
    replaceable(&area.out)?;
    let journal = area.journal();
    let staging = area.staging();
    let backup = area.backup();
    fs::write(&journal, "staging\n").map_err(|e| io_error("writing", &journal, e))?;
    remove_tree(&staging).map_err(|e| io_error("removing", &staging, e))?;
    let written = (|| -> Result<(), String> {
        fs::create_dir_all(&staging).map_err(|e| io_error("creating", &staging, e))?;
        for (path, bytes) in files {
            let target = staging.join(path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|e| io_error("creating", parent, e))?;
            }
            fs::write(&target, bytes).map_err(|e| io_error("writing", &target, e))?;
        }
        Ok(())
    })();
    if let Err(error) = written {
        let _ = remove_tree(&staging);
        let _ = fs::remove_file(&journal);
        return Err(error);
    }

    at(Step::Staged);
    fs::write(&journal, "swap\n").map_err(|e| io_error("writing", &journal, e))?;
    let had_output = area.out.exists();
    if had_output {
        if let Err(error) = fs::rename(&area.out, &backup) {
            let _ = remove_tree(&staging);
            let _ = fs::remove_file(&journal);
            return Err(io_error("moving aside", &area.out, error));
        }
        at(Step::MovedAside);
    }
    if let Err(error) = fs::rename(&staging, &area.out) {
        let failed = io_error("moving into place", &area.out, error);
        if had_output {
            fs::rename(&backup, &area.out).map_err(|e| {
                format!("{failed}; and the previous output could not be put back ({e}): the next build restores it")
            })?;
        }
        let _ = remove_tree(&staging);
        let _ = fs::remove_file(&journal);
        return Err(failed);
    }
    at(Step::SwappedIn);
    remove_tree(&backup).map_err(|e| io_error("removing", &backup, e))?;
    fs::remove_file(&journal).map_err(|e| io_error("removing", &journal, e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Dir(PathBuf);

    impl Dir {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let root = std::env::temp_dir()
                .join(format!("rustify-docs-publish-{}-{n}", std::process::id()));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).unwrap();
            Self(root)
        }
    }

    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn site(text: &str) -> BTreeMap<String, Arc<[u8]>> {
        let mut files = BTreeMap::new();
        files.insert("index.html".to_owned(), Arc::from(text.as_bytes()));
        files.insert("a/b.txt".to_owned(), Arc::from(&b"b"[..]));
        add_manifest(&mut files, "/", "0.1.0", &BTreeMap::new(), &BTreeMap::new());
        files
    }

    #[test]
    fn a_manifest_is_its_files_and_nothing_of_the_machine() {
        let files = site("one");
        let manifest: Manifest = serde_json::from_slice(&files[MANIFEST_FILE]).unwrap();
        assert_eq!(manifest.mode, "production");
        assert_eq!(
            manifest
                .files
                .iter()
                .map(|f| f.path.as_str())
                .collect::<Vec<_>>(),
            ["a/b.txt", "index.html"]
        );
        assert_eq!(site("one")[MANIFEST_FILE], files[MANIFEST_FILE]);
        assert_ne!(site("two")[MANIFEST_FILE], files[MANIFEST_FILE]);
    }

    #[test]
    fn locale_manifest_freezes_only_declared_error_files_and_preserves_legacy_bytes() {
        let dir = Dir::new();
        let mut files = BTreeMap::from([
            ("404.html".into(), Arc::<[u8]>::from(&b"English"[..])),
            ("zh/404.html".into(), Arc::<[u8]>::from(&b"Chinese"[..])),
            (
                "foo/404.html".into(),
                Arc::<[u8]>::from(&b"Ordinary file"[..]),
            ),
        ]);
        let mapping = BTreeMap::from([
            ("".into(), "404.html".into()),
            ("zh".into(), "zh/404.html".into()),
        ]);
        add_manifest(&mut files, "/docs/", "0.1.0", &mapping, &BTreeMap::new());
        for (path, bytes) in &files {
            let path = dir.0.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
        }
        let manifest = Manifest::read(&dir.0).unwrap();
        assert_eq!(manifest.locale_not_found, mapping);
        let frozen = manifest.locale_not_found_files(&dir.0).unwrap();
        assert_eq!(frozen.len(), 2);
        assert_eq!(frozen["zh"], b"Chinese");
        assert!(!frozen.contains_key("foo"));
        let legacy = site("legacy");
        assert!(!String::from_utf8_lossy(&legacy[MANIFEST_FILE]).contains("locale_not_found"));
        let manifest: Manifest = serde_json::from_slice(&legacy[MANIFEST_FILE]).unwrap();
        assert!(manifest.locale_not_found.is_empty());
    }

    #[test]
    fn locale_manifest_rejects_invalid_or_missing_declared_error_files() {
        let dir = Dir::new();
        let files = BTreeMap::from([("404.html".into(), Arc::<[u8]>::from(&b"English"[..]))]);
        fs::write(dir.0.join("404.html"), b"English").unwrap();
        for (prefix, path) in [
            ("zh", "zh/404.html"),
            ("..", "../404.html"),
            ("%7a%68", "%7a%68/404.html"),
            ("assets", "assets/404.html"),
            ("zh", "404.html"),
            ("zh", "/zh/404.html"),
        ] {
            let mut manifest = Manifest::of(&files, "/", "0.1.0");
            manifest.locale_not_found =
                BTreeMap::from([("".into(), "404.html".into()), (prefix.into(), path.into())]);
            assert!(
                manifest.locale_not_found_files(&dir.0).is_err(),
                "{prefix} {path}"
            );
        }
        let mut manifest = Manifest::of(&files, "/", "0.1.0");
        manifest
            .locale_not_found
            .insert("zh".into(), "zh/404.html".into());
        assert!(manifest.locale_not_found_files(&dir.0).is_err());
        manifest.locale_not_found = BTreeMap::from([("".into(), "404.html".into())]);
        fs::remove_file(dir.0.join("404.html")).unwrap();
        assert!(manifest.locale_not_found_files(&dir.0).is_err());
    }

    #[test]
    fn page_policies_are_recorded_only_when_there_are_some_and_checked_before_use() {
        let legacy = site("legacy");
        assert!(!String::from_utf8_lossy(&legacy[MANIFEST_FILE]).contains("page_policies"));
        let manifest: Manifest = serde_json::from_slice(&legacy[MANIFEST_FILE]).unwrap();
        assert!(manifest.page_policies.is_empty());
        assert_eq!(manifest.checked_page_policies(), Ok(BTreeMap::new()));

        let policy = crate::shell::CSP.replace(
            "form-action 'none'",
            "form-action 'self' https://leads.example.com",
        );
        let mut files = BTreeMap::from([
            ("lp/index.html".to_owned(), Arc::<[u8]>::from(&b"lp"[..])),
            ("assets/x.js".to_owned(), Arc::<[u8]>::from(&b"x"[..])),
        ]);
        let policies = BTreeMap::from([("lp/index.html".to_owned(), policy.clone())]);
        add_manifest(&mut files, "/", "0.1.0", &BTreeMap::new(), &policies);
        let manifest: Manifest = serde_json::from_slice(&files[MANIFEST_FILE]).unwrap();
        assert_eq!(manifest.page_policies, policies);
        assert_eq!(manifest.checked_page_policies(), Ok(policies));

        for (path, value) in [
            ("lp/index.html", format!("{policy}\r\nSet-Cookie: a=b")),
            (
                "lp/index.html",
                policy.replace("form-action", "form-action\n"),
            ),
            (
                "lp/index.html",
                policy.replace("'self' https", "'self' h\u{e9}ttps"),
            ),
            (
                "lp/index.html",
                policy.replacen("default-src 'none'; ", "", 1),
            ),
            (
                "lp/index.html",
                policy.replace("; frame-ancestors 'none'", ""),
            ),
            ("lp/index.html", String::new()),
            ("assets/x.js", policy.clone()),
            ("other/index.html", policy.clone()),
            ("../lp/index.html", policy.clone()),
        ] {
            let mut damaged = manifest.clone();
            damaged.page_policies = BTreeMap::from([(path.to_owned(), value.clone())]);
            let error = damaged.checked_page_policies().unwrap_err();
            assert!(
                error.starts_with("invalid page_policies: ")
                    && error.ends_with("run `rustify-docs build` again"),
                "{path} {value:?}: {error}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn locale_manifest_rejects_error_file_symlinks_outside_the_output() {
        let dir = Dir::new();
        let outside = Dir::new();
        fs::write(outside.0.join("error.html"), b"outside").unwrap();
        std::os::unix::fs::symlink(outside.0.join("error.html"), dir.0.join("404.html")).unwrap();
        let files = BTreeMap::from([("404.html".into(), Arc::<[u8]>::from(&b"outside"[..]))]);
        let mut manifest = Manifest::of(&files, "/", "0.1.0");
        manifest
            .locale_not_found
            .insert("".into(), "404.html".into());
        assert!(manifest.locale_not_found_files(&dir.0).is_err());
    }

    #[test]
    fn publishing_replaces_a_build_whole_and_keeps_strangers_out() {
        let dir = Dir::new();
        let area = Area::of(&dir.0.join("dist")).unwrap();
        assert!(area
            .dir
            .starts_with(fs::canonicalize(&dir.0).unwrap().join(".rustify-docs")));
        let held = lock(&area, true).unwrap();
        assert!(matches!(lock(&area, true), Err(LockError::Busy)));
        assert!(matches!(lock(&area, false), Err(LockError::Busy)));
        assert_eq!(recover(&held).unwrap(), Recovery::Clean);
        publish(&held, &site("one")).unwrap();
        assert_eq!(
            fs::read_to_string(area.out.join("index.html")).unwrap(),
            "one"
        );
        publish(&held, &site("two")).unwrap();
        assert_eq!(
            fs::read_to_string(area.out.join("index.html")).unwrap(),
            "two"
        );
        assert!(!area.staging().exists() && !area.backup().exists() && !area.unfinished());
        drop(held);
        let shared = lock(&area, false).unwrap();
        assert!(lock(&area, false).is_ok(), "previews share");
        assert!(matches!(lock(&area, true), Err(LockError::Busy)));
        drop(shared);

        let stranger = dir.0.join("src");
        fs::create_dir_all(&stranger).unwrap();
        fs::write(stranger.join("main.rs"), "fn main() {}").unwrap();
        let area = Area::of(&stranger).unwrap();
        let held = lock(&area, true).unwrap();
        assert!(publish(&held, &site("x"))
            .unwrap_err()
            .contains("no build wrote"));
        assert_eq!(
            fs::read_to_string(stranger.join("main.rs")).unwrap(),
            "fn main() {}"
        );
    }

    #[test]
    fn an_interrupted_build_is_finished_or_undone() {
        let dir = Dir::new();
        let area = Area::of(&dir.0.join("dist")).unwrap();
        let held = lock(&area, true).unwrap();
        publish(&held, &site("old")).unwrap();
        let old = || fs::read_to_string(area.out.join("index.html")).unwrap();

        // Killed while staging.
        fs::write(area.journal(), "staging\n").unwrap();
        fs::create_dir_all(area.staging()).unwrap();
        assert_eq!(recover(&held).unwrap(), Recovery::Discarded);
        assert_eq!(old(), "old");

        // Killed between the renames: the old output is moved aside.
        fs::write(area.journal(), "swap\n").unwrap();
        fs::create_dir_all(area.staging()).unwrap();
        fs::rename(&area.out, area.backup()).unwrap();
        assert_eq!(recover(&held).unwrap(), Recovery::Restored);
        assert_eq!(old(), "old");

        // Killed after the swap, before the clean-up.
        fs::write(area.journal(), "swap\n").unwrap();
        fs::create_dir_all(area.backup()).unwrap();
        assert_eq!(recover(&held).unwrap(), Recovery::Completed);
        assert!(!area.backup().exists() && !area.unfinished());
        assert_eq!(old(), "old");
    }
}
