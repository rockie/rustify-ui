//! What the CLI's integration tests share: throwaway projects, copies of the
//! repository's fixtures, a stand-in toolchain that records what runs, and
//! snapshots of directory trees.

#![allow(dead_code)]

pub mod embedded;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};

pub const BIN: &str = env!("CARGO_BIN_EXE_rustify-docs");

pub fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/docs-ssg/fixtures")
        .join(name)
}

/// Keep development paths, or project the exact pair emitted by the tested CLI.
/// The helper writes only the copied author's manifest and lock, never its views.
pub fn pair_component_project(project: &Path) {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new("python3")
        .arg(repository.join("scripts/docs-pages-prepare.py"))
        .arg("--project")
        .arg(project.join("Cargo.toml"))
        .args(["--cli", BIN])
        .output()
        .expect("the repository's Python 3.12 pair helper runs");
    succeeded(&output);
    let pair: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        pair["framework_revision"],
        include_str!("../../assets/framework-revision.txt").trim(),
        "the tested CLI and its Git source C must have the same paired R"
    );
}

/// A fresh temporary directory, removed when dropped.
pub struct Temp(pub PathBuf);

impl Temp {
    pub fn new(label: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "rustify-docs-cli-{label}-{}-{n}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        // Resolved, so paths the CLI prints compare equal.
        Self(std::fs::canonicalize(&root).unwrap())
    }

    pub fn path(&self, relative: &str) -> PathBuf {
        self.0.join(relative)
    }

    pub fn write(&self, relative: &str, text: &str) {
        let path = self.path(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    pub fn read(&self, relative: &str) -> String {
        std::fs::read_to_string(self.path(relative))
            .unwrap_or_else(|error| panic!("{relative}: {error}"))
    }

    /// Replaces `from` with `to` in a file, asserting it was there.
    pub fn edit(&self, relative: &str, from: &str, to: &str) {
        let text = self.read(relative);
        assert!(text.contains(from), "{from:?} not in {relative}");
        self.write(relative, &text.replacen(from, to, 1));
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A copy of a fixture in a fresh temporary directory, under `site/`.
pub fn copy_fixture(name: &str) -> Temp {
    let temp = Temp::new(name);
    copy_tree(&fixture(name), &temp.path("site"));
    temp
}

pub fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// Every file under `root`, by `/`-separated relative path, with its bytes.
pub fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut files = BTreeMap::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                let relative = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                files.insert(relative, std::fs::read(&path).unwrap());
            }
        }
    }
    files
}

pub fn text_of(files: &BTreeMap<String, Vec<u8>>, path: &str) -> String {
    String::from_utf8(
        files
            .get(path)
            .unwrap_or_else(|| panic!("{path} not in {:?}", files.keys().collect::<Vec<_>>()))
            .clone(),
    )
    .unwrap()
}

pub fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Runs the CLI in `dir` with `args`.
pub fn run_in(dir: &Path, args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the rustify-docs binary runs")
}

pub fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

pub fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[track_caller]
pub fn succeeded(output: &Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "stdout:\n{}\nstderr:\n{}",
        stdout(output),
        stderr(output)
    );
}

/// A directory of stand-in tools for `PATH`. Each records its arguments in
/// `log`; `rustc --version` answers like a stable toolchain when `rustc` is
/// included, and every other tool fails, so a test sees anything the CLI
/// tried to compile with.
pub struct Toolchain {
    pub bin: PathBuf,
    pub log: PathBuf,
}

impl Toolchain {
    pub fn new(temp: &Temp, with_rustc: bool) -> Self {
        use std::os::unix::fs::PermissionsExt;
        let bin = temp.path("toolchain-bin");
        let log = temp.path("toolchain.log");
        std::fs::create_dir_all(&bin).unwrap();
        std::fs::write(&log, "").unwrap();
        let mut tools = vec![
            "cargo",
            "rustup",
            "cc",
            "clang",
            "ld",
            "wasm-bindgen",
            "mbx",
        ];
        if with_rustc {
            tools.push("rustc");
        }
        for tool in tools {
            let answer = if tool == "rustc" {
                "if [ \"$1\" = \"--version\" ]; then echo 'rustc 1.99.0 (b940084d7 2026-09-28)'; exit 0; fi\nexit 1\n"
            } else {
                "exit 1\n"
            };
            let script = format!(
                "#!/bin/sh\necho \"{tool} $*\" >> '{}'\n{answer}",
                log.display()
            );
            let path = bin.join(tool);
            std::fs::write(&path, script).unwrap();
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        Self { bin, log }
    }

    /// The CLI in `dir` with only these tools on `PATH`.
    pub fn run_in(&self, dir: &Path, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .current_dir(dir)
            .env_clear()
            .env("PATH", &self.bin)
            .env("HOME", dir)
            .output()
            .expect("the rustify-docs binary runs")
    }

    pub fn calls(&self) -> Vec<String> {
        std::fs::read_to_string(&self.log)
            .unwrap()
            .lines()
            .map(str::to_owned)
            .collect()
    }
}

/// A running `preview`, stopped when dropped.
pub struct Preview {
    pub child: Child,
    /// `http://127.0.0.1:<port>/<base>`.
    pub url: String,
}

impl Preview {
    /// Starts `preview` in `dir` with `args` and waits for its address.
    /// `Err` holds what it printed when it exited instead.
    pub fn start(dir: &Path, args: &[&str]) -> Result<Self, String> {
        use std::io::{BufRead, BufReader};
        let mut child = Command::new(BIN)
            .arg("preview")
            .args(["--port", "0"])
            .args(args)
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut line = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        match line.strip_prefix("preview: ") {
            Some(url) => Ok(Self {
                child,
                url: url.trim().to_owned(),
            }),
            None => {
                let output = child.wait_with_output().unwrap();
                Err(format!("{line}{}", stderr(&output)))
            }
        }
    }
}

impl Drop for Preview {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A minimal HTTP/1.1 GET: the status code, the headers (lower-cased
/// names) and the body.
pub fn get(url: &str) -> (u16, BTreeMap<String, String>, Vec<u8>) {
    use std::io::{Read, Write};
    let rest = url.strip_prefix("http://").expect("an http URL");
    let (host, path) = match rest.find('/') {
        Some(cut) => (&rest[..cut], &rest[cut..]),
        None => (rest, "/"),
    };
    let mut stream = std::net::TcpStream::connect(host).unwrap();
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();
    let split = response
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("a header block");
    let head = String::from_utf8_lossy(&response[..split]).into_owned();
    let body = response[split + 4..].to_vec();
    let mut lines = head.lines();
    let status = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap();
    let headers = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.trim().to_ascii_lowercase(), value.trim().to_owned()))
        .collect();
    (status, headers, body)
}

/// A running `dev`, stopped when dropped.
pub struct Dev {
    pub child: Child,
    /// `http://127.0.0.1:<port>/<base>`.
    pub url: String,
    /// What it wrote to stderr, as it goes.
    pub log: std::sync::Arc<std::sync::Mutex<String>>,
}

impl Dev {
    /// Starts `dev --port 0` in `dir` with `args` and `env`, and waits for
    /// its address. `Err` holds what it printed when it exited instead.
    pub fn start(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> Result<Self, String> {
        Self::start_with(dir, &[&["--port", "0"], args].concat(), env)
    }

    /// As `start`, without choosing the port.
    pub fn start_with(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> Result<Self, String> {
        let dev = Self::start_pending(dir, args, env)?;
        dev.events("/", 0).until(|state| state["building"] == false);
        Ok(dev)
    }

    /// Starts at the printed address, before the initial build has finished.
    pub fn start_pending(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> Result<Self, String> {
        use std::io::{BufRead, BufReader, Read};
        let mut command = Command::new(BIN);
        command
            .arg("dev")
            .args(args)
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (name, value) in env {
            command.env(name, value);
        }
        let mut child = command.spawn().unwrap();
        let log = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let mut stderr = child.stderr.take().unwrap();
        let sink = log.clone();
        std::thread::spawn(move || {
            let mut buffer = [0u8; 4096];
            while let Ok(n) = stderr.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                sink.lock()
                    .unwrap()
                    .push_str(&String::from_utf8_lossy(&buffer[..n]));
            }
        });
        let mut line = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        match line.strip_prefix("dev: ") {
            Some(url) => Ok(Self {
                child,
                url: url.trim().to_owned(),
                log,
            }),
            None => {
                let _ = child.wait();
                std::thread::sleep(std::time::Duration::from_millis(100));
                let text = log.lock().unwrap().clone();
                Err(format!("{line}{text}"))
            }
        }
    }

    /// `http://127.0.0.1:<port>`.
    pub fn origin(&self) -> String {
        let rest = self.url.strip_prefix("http://").unwrap();
        format!("http://{}", &rest[..rest.find('/').unwrap()])
    }

    pub fn log(&self) -> String {
        self.log.lock().unwrap().clone()
    }

    /// The event stream a page at `path` (a copy of build `good`) opens.
    pub fn events(&self, path: &str, good: u64) -> Events {
        Events::open(&self.origin(), path, good)
    }
}

impl Drop for Dev {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// `dev`'s server-sent events, read as a page would.
pub struct Events {
    reader: std::io::BufReader<std::net::TcpStream>,
}

impl Events {
    pub fn open(origin: &str, path: &str, good: u64) -> Self {
        use std::io::{BufRead, Write};
        let host = origin.strip_prefix("http://").unwrap();
        let mut stream = std::net::TcpStream::connect(host).unwrap();
        let encoded: String = path
            .bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' => {
                    (b as char).to_string()
                }
                _ => format!("%{b:02X}"),
            })
            .collect();
        write!(
            stream,
            "GET /__rustify-docs/events?path={encoded}&good={good} HTTP/1.1\r\nHost: {host}\r\n\r\n"
        )
        .unwrap();
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(20)))
            .unwrap();
        let mut reader = std::io::BufReader::new(stream);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if line == "\r\n" || line.is_empty() {
                break;
            }
        }
        Self { reader }
    }

    /// The next state the server sends.
    pub fn next(&mut self) -> serde_json::Value {
        use std::io::BufRead;
        loop {
            let mut line = String::new();
            let read = self.reader.read_line(&mut line).expect("an event in time");
            assert!(read > 0, "the event stream ended");
            if let Some(data) = line.strip_prefix("data: ") {
                return serde_json::from_str(data).unwrap();
            }
        }
    }

    /// The first state that satisfies `wanted`.
    pub fn until(&mut self, wanted: impl Fn(&serde_json::Value) -> bool) -> serde_json::Value {
        loop {
            let state = self.next();
            if wanted(&state) {
                return state;
            }
        }
    }
}

/// The body of a GET as text.
pub fn get_text(url: &str) -> (u16, String) {
    let (status, _, body) = get(url);
    (status, String::from_utf8_lossy(&body).into_owned())
}
