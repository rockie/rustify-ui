//! Static preview server for built examples.
//!
//! Serves one app directory under a base path with the release Content
//! Security Policy and no cross-origin isolation, so a preview exercises the
//! same browser contract as an ordinary static deployment.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CspMode {
    /// The release policy: same-origin scripts plus wasm execution.
    Strict,
    /// The release policy without `'wasm-unsafe-eval'`: wasm must fail to start.
    NoWasm,
    /// No policy header at all.
    Off,
}

impl CspMode {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "strict" => Ok(Self::Strict),
            "no-wasm" => Ok(Self::NoWasm),
            "off" => Ok(Self::Off),
            other => Err(format!(
                "unknown csp mode {other}; use strict, no-wasm or off"
            )),
        }
    }

    pub fn header_value(self) -> Option<&'static str> {
        match self {
            Self::Strict => Some(
                "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; \
                 img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; \
                 worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; \
                 frame-ancestors 'none'",
            ),
            Self::NoWasm => Some(
                "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' data:; \
                 font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; \
                 object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'",
            ),
            Self::Off => None,
        }
    }
}

pub struct ServeConfig {
    pub root: PathBuf,
    pub base: String,
    pub port: u16,
    pub csp: CspMode,
    /// Answer a path that names no file with `index.html`, so a deep link
    /// opened cold reaches the application rather than a 404. Only for paths
    /// that look like a route: anything with a file extension stays a 404,
    /// because a missing script answered with HTML is a deployment fault that
    /// hides itself.
    pub spa: bool,
    /// What this server is currently doing wrong on purpose. Set at start with
    /// `--fault`, and changed at run time through `<base>__fault/<spec>`, so
    /// one server can exercise several deployment failures without a restart.
    pub fault: std::sync::Mutex<Option<Fault>>,
}

/// A deployment that is broken in one specific way.
///
/// Each of these is something a real deployment does: a file that was not
/// copied, a file that arrived damaged, and a set of assets from two different
/// builds. They are injected at the server rather than by editing the build,
/// so what is under test is the running product and not a doctored copy of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The named file answers 404.
    Missing(String),
    /// The named file answers with its bytes damaged in the middle.
    Corrupt(String),
    /// The named file answers with half its bytes and a truncated length.
    Truncated(String),
    /// The message bridge claims a schema hash the wasm does not have, which
    /// is what a half-updated deployment looks like from the browser.
    StaleBridge,
}

impl Fault {
    pub fn parse(spec: &str) -> Result<Self, String> {
        match spec.split_once(':') {
            Some(("missing", path)) => Ok(Self::Missing(path.to_string())),
            Some(("corrupt", path)) => Ok(Self::Corrupt(path.to_string())),
            Some(("truncated", path)) => Ok(Self::Truncated(path.to_string())),
            _ if spec == "stale-bridge" => Ok(Self::StaleBridge),
            _ if spec == "none" => Err("none".to_string()),
            other => Err(format!(
                "unknown fault {other:?}; use missing:<path>, corrupt:<path>, truncated:<path> or stale-bridge"
            )),
        }
    }

    /// The file this fault concerns, if it names one.
    fn target(&self) -> Option<&str> {
        match self {
            Self::Missing(path) | Self::Corrupt(path) | Self::Truncated(path) => Some(path),
            Self::StaleBridge => Some("rustify_makepad/message_bridge.js"),
        }
    }
}

/// Normalizes a user supplied base path to the `/segment/.../` form.
pub fn normalize_base(base: &str) -> String {
    let trimmed = base.trim().trim_matches('/');
    if trimmed.is_empty() {
        "/".to_string()
    } else {
        format!("/{trimmed}/")
    }
}

/// Maps a request path to a file inside the root, or `None` when the path is
/// outside the base, escapes the root, or names nothing.
pub fn resolve(root: &Path, base: &str, request_path: &str) -> Option<PathBuf> {
    let path = request_path.split(['?', '#']).next().unwrap_or("");
    let rest = path
        .strip_prefix(base)
        .or_else(|| (path.len() + 1 == base.len() && base.starts_with(path)).then_some(""))?;
    let mut file = root.to_path_buf();
    for segment in rest.split('/') {
        match segment {
            "" | "." => {}
            ".." => return None,
            other => file.push(other),
        }
    }
    if file.is_dir() {
        file.push("index.html");
    }
    file.is_file().then_some(file)
}

pub fn mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("wasm") => "application/wasm",
        Some("json") => "application/json",
        Some("ttf") => "font/ttf",
        Some("otf") => "font/otf",
        Some("woff2") => "font/woff2",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("md") => "text/markdown; charset=utf-8",
        Some("bin") => "application/octet-stream",
        _ => "application/octet-stream",
    }
}

pub fn serve(config: ServeConfig) -> Result<(), String> {
    let listener = TcpListener::bind(("127.0.0.1", config.port))
        .map_err(|e| format!("cannot bind 127.0.0.1:{}: {e}", config.port))?;
    let addr = listener.local_addr().map_err(|e| e.to_string())?;
    println!(
        "serving {} at http://{}{}",
        config.root.display(),
        addr,
        config.base
    );
    println!("csp: {:?}", config.csp);
    let config = std::sync::Arc::new(config);
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("accept failed: {error}");
                continue;
            }
        };
        let config = config.clone();
        std::thread::spawn(move || {
            if let Err(error) = handle(stream, &config) {
                eprintln!("request failed: {error}");
            }
        });
    }
    Ok(())
}

fn handle(mut stream: TcpStream, config: &ServeConfig) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    let mut content_length = 0usize;
    let mut if_none_match = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        let lower = line.to_ascii_lowercase();
        if let Some(value) = lower.strip_prefix("content-length:") {
            content_length = value.trim().parse().unwrap_or(0);
        }
        if lower.starts_with("if-none-match:") {
            if_none_match = line
                .split_once(':')
                .map(|(_, value)| value.trim().to_string())
                .unwrap_or_default();
        }
    }
    if content_length > 0 {
        let mut body = vec![0u8; content_length];
        reader.read_exact(&mut body)?;
    }

    if method != "GET" && method != "HEAD" {
        return write_response(
            &mut stream,
            405,
            "text/plain",
            b"method not allowed",
            config,
            false,
        );
    }
    let path_only = target.split(['?', '#']).next().unwrap_or("/");
    if path_only.len() + 1 == config.base.len() && config.base.starts_with(path_only) {
        let location = format!("Location: {}\r\n", config.base);
        let head = format!("HTTP/1.1 301 Moved Permanently\r\n{location}Content-Length: 0\r\nConnection: close\r\n\r\n");
        return stream.write_all(head.as_bytes());
    }
    // The fault switch. A page reloads after setting one, so the fault applies
    // to the whole document rather than to whatever happened to be in flight.
    if let Some(spec) = path_only.strip_prefix(&format!("{}__fault/", config.base)) {
        let next = match Fault::parse(spec) {
            Ok(fault) => Some(fault),
            Err(reason) if reason == "none" => None,
            Err(reason) => {
                return write_response(
                    &mut stream,
                    404,
                    "text/plain",
                    reason.as_bytes(),
                    config,
                    false,
                )
            }
        };
        let reply = format!("{next:?}");
        *config.fault.lock().unwrap() = next;
        return write_response(
            &mut stream,
            200,
            "text/plain",
            reply.as_bytes(),
            config,
            false,
        );
    }
    match resolve(&config.root, &config.base, target) {
        Some(file) => {
            let relative = file
                .strip_prefix(&config.root)
                .map(|rest| rest.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            let fault = config
                .fault
                .lock()
                .unwrap()
                .clone()
                .filter(|fault| fault.target() == Some(relative.as_str()));
            match fault {
                Some(Fault::Missing(_)) => {
                    return write_response(
                        &mut stream,
                        404,
                        "text/plain",
                        b"not found",
                        config,
                        false,
                    )
                }
                Some(fault) => {
                    let body = damage(&fault, std::fs::read(&file)?);
                    return write_response(
                        &mut stream,
                        200,
                        mime_type(&file),
                        &body,
                        config,
                        method == "HEAD",
                    );
                }
                None => {}
            }
            // A validator, so a browser that already has this file can be
            // told to keep it. Without one, `Cache-Control: no-cache` means
            // every reload re-downloads every byte - which made a "hot start"
            // impossible to measure here, because there was no such thing.
            //
            // Size and modification time rather than a hash of the content:
            // this serves an eleven-megabyte module, and hashing it on every
            // request would trade the download for a read.
            let tag = etag(&file);
            if let Some(tag) = &tag {
                if etag_matches(&if_none_match, tag) {
                    return write_not_modified(&mut stream, tag, config);
                }
            }
            let body = std::fs::read(&file)?;
            write_file_response(
                &mut stream,
                mime_type(&file),
                &body,
                tag.as_deref(),
                config,
                method == "HEAD",
            )
        }
        None => {
            if config.spa && looks_like_a_route(target) {
                let index = config.root.join("index.html");
                if let Ok(body) = std::fs::read(&index) {
                    return write_response(
                        &mut stream,
                        200,
                        "text/html; charset=utf-8",
                        &body,
                        config,
                        method == "HEAD",
                    );
                }
            }
            write_response(&mut stream, 404, "text/plain", b"not found", config, false)
        }
    }
}

/// Whether a path that names no file should be answered with the application.
///
/// A route has no file extension in its last segment. A request for
/// `app.js` that is missing must stay a 404: answering it with HTML would turn
/// a deployment that lost a file into a page that fails somewhere else, much
/// later, for a reason nobody can see.
fn looks_like_a_route(target: &str) -> bool {
    let path = target.split(['?', '#']).next().unwrap_or(target);
    !path
        .rsplit('/')
        .next()
        .is_some_and(|segment| segment.contains('.'))
}

/// The bytes a broken deployment would serve.
pub fn damage(fault: &Fault, mut body: Vec<u8>) -> Vec<u8> {
    match fault {
        Fault::Missing(_) => Vec::new(),
        Fault::Corrupt(_) => {
            // In the middle, so a reader that checks a header still gets past
            // it and fails on the content, which is the harder case.
            if !body.is_empty() {
                let middle = body.len() / 2;
                body[middle] ^= 0xff;
            }
            body
        }
        Fault::Truncated(_) => {
            body.truncate(body.len() / 2);
            body
        }
        Fault::StaleBridge => {
            // The hash the page checks against the wasm's own. One digit is
            // enough: what is being tested is that the two are compared.
            let text = String::from_utf8_lossy(&body).to_string();
            const PREFIX: &str = "export const SCHEMA_HASH = \"";
            match text.find(PREFIX) {
                Some(start) => {
                    let value = start + PREFIX.len();
                    // One digit in front of the hash: the two no longer agree,
                    // which is the whole of what a half-updated deployment
                    // looks like from the browser.
                    format!("{}1{}", &text[..value], &text[value..]).into_bytes()
                }
                None => body,
            }
        }
    }
}

/// What this file is right now, as a value a browser can send back.
///
/// `None` when the file's metadata cannot be read, which is not an error here:
/// the response simply goes out without a validator and is never revalidated.
fn etag(file: &Path) -> Option<String> {
    let meta = std::fs::metadata(file).ok()?;
    let modified = meta
        .modified()
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?;
    Some(format!(
        "\"{}-{}-{}\"",
        meta.len(),
        modified.as_secs(),
        modified.subsec_nanos()
    ))
}

/// Whether an `If-None-Match` header names this version.
///
/// A list, because that is what the header is, and `*` because a browser is
/// allowed to send it. A weak comparison would need the `W/` prefix stripped;
/// nothing here ever sends a weak tag, so a mismatch is a mismatch.
fn etag_matches(header: &str, tag: &str) -> bool {
    header
        .split(',')
        .map(str::trim)
        .any(|candidate| candidate == "*" || candidate == tag)
}

fn write_not_modified(
    stream: &mut TcpStream,
    tag: &str,
    config: &ServeConfig,
) -> std::io::Result<()> {
    let mut head = format!(
        "HTTP/1.1 304 Not Modified\r\nETag: {tag}\r\nCache-Control: no-cache\r\n\
         X-Content-Type-Options: nosniff\r\nConnection: close\r\n"
    );
    if let Some(csp) = config.csp.header_value() {
        head.push_str("Content-Security-Policy: ");
        head.push_str(csp);
        head.push_str("\r\n");
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes())?;
    stream.flush()
}

/// A file, with its validator when it has one. Everything else about the
/// response is what `write_response` sends.
fn write_file_response(
    stream: &mut TcpStream,
    mime: &str,
    body: &[u8],
    tag: Option<&str>,
    config: &ServeConfig,
    head_only: bool,
) -> std::io::Result<()> {
    write_response_with(stream, 200, mime, body, tag, config, head_only)
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    mime: &str,
    body: &[u8],
    config: &ServeConfig,
    head_only: bool,
) -> std::io::Result<()> {
    write_response_with(stream, status, mime, body, None, config, head_only)
}

fn write_response_with(
    stream: &mut TcpStream,
    status: u16,
    mime: &str,
    body: &[u8],
    tag: Option<&str>,
    config: &ServeConfig,
    head_only: bool,
) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Error",
    };
    let mut head = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {mime}\r\nContent-Length: {}\r\n\
         Cache-Control: no-cache\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n",
        body.len()
    );
    if let Some(tag) = tag {
        head.push_str("ETag: ");
        head.push_str(tag);
        head.push_str("\r\n");
    }
    if let Some(csp) = config.csp.header_value() {
        head.push_str("Content-Security-Policy: ");
        head.push_str(csp);
        head.push_str("\r\n");
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes())?;
    if !head_only {
        stream.write_all(body)?;
    }
    stream.flush()
}

#[cfg(test)]
mod fault_tests {
    use super::*;

    #[test]
    fn a_fault_names_the_file_it_breaks() {
        assert_eq!(
            Fault::parse("missing:app.js"),
            Ok(Fault::Missing("app.js".to_string()))
        );
        assert_eq!(
            Fault::parse("corrupt:fonts/a.ttf"),
            Ok(Fault::Corrupt("fonts/a.ttf".to_string()))
        );
        assert_eq!(
            Fault::parse("truncated:fonts/a.ttf"),
            Ok(Fault::Truncated("fonts/a.ttf".to_string()))
        );
        assert_eq!(Fault::parse("stale-bridge"), Ok(Fault::StaleBridge));
        assert_eq!(Fault::parse("none"), Err("none".to_string()));
        assert!(Fault::parse("explode:app.js")
            .unwrap_err()
            .contains("unknown fault"));
    }

    #[test]
    fn each_kind_of_damage_is_the_one_it_says() {
        let body = b"0123456789".to_vec();
        assert!(damage(&Fault::Missing("x".into()), body.clone()).is_empty());
        assert_eq!(damage(&Fault::Truncated("x".into()), body.clone()).len(), 5);
        let corrupt = damage(&Fault::Corrupt("x".into()), body.clone());
        assert_eq!(corrupt.len(), body.len());
        assert_ne!(corrupt, body);
        // The ends survive, so a reader that only checks a header is fooled.
        assert_eq!(corrupt[0], body[0]);
        assert_eq!(corrupt[body.len() - 1], body[body.len() - 1]);
    }

    #[test]
    fn a_stale_bridge_claims_a_hash_the_wasm_does_not_have() {
        let module =
            b"// generated\nexport const SCHEMA_HASH = \"4242\";\nexport function x(){}".to_vec();
        let stale = damage(&Fault::StaleBridge, module.clone());
        let text = String::from_utf8(stale).expect("still javascript");
        assert!(
            text.contains("export const SCHEMA_HASH = \"14242\";"),
            "{text}"
        );
        // Still a module: what fails is the comparison, not the parse.
        assert!(text.contains("export function x(){}"), "{text}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tree of its own for the test that asked, named after it: two tests
    /// sharing one directory is two tests where one can delete the other's
    /// files halfway through, and they run at the same time.
    fn fixture(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("xtask-serve-{}-{name}", std::process::id()));
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("index.html"), "<html>").unwrap();
        std::fs::write(root.join("sub").join("a.js"), "1").unwrap();
        root
    }

    #[test]
    fn a_route_is_answered_with_the_application_and_a_missing_file_is_not() {
        // The distinction the `--spa` fallback turns on. A deep link has to
        // reach the application; a script that did not deploy has to stay a
        // 404, or the failure hides itself somewhere much later.
        assert!(looks_like_a_route("/tools/demo/objects/42"));
        assert!(looks_like_a_route("/objects"));
        assert!(looks_like_a_route("/objects/42?tab=notes"));
        assert!(!looks_like_a_route("/app.js"));
        assert!(!looks_like_a_route("/tools/demo/rustify.css"));
        assert!(!looks_like_a_route("/vendor/nouislider/nouislider.min.css"));
    }

    #[test]
    fn a_revalidation_is_answered_only_for_the_version_the_browser_has() {
        // The whole point of the validator is that a stale one does not match.
        // Getting this wrong does not fail loudly: it serves an old build to a
        // browser that asked whether its copy was still good.
        let root = fixture("revalidation");
        let file = root.join("sub").join("a.js");
        let tag = etag(&file).expect("a file that exists has a validator");
        assert!(etag_matches(&tag, &tag));
        assert!(etag_matches("*", &tag));
        assert!(etag_matches(&format!("\"other\", {tag}"), &tag));
        assert!(!etag_matches("\"other\"", &tag));
        assert!(!etag_matches("", &tag));

        // A file that changed has a different validator, so the browser is
        // told to take the new bytes.
        std::fs::write(&file, "12").unwrap();
        let after = etag(&file).expect("still a file");
        assert_ne!(tag, after);
        assert!(!etag_matches(&tag, &after));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn base_paths_are_normalized_to_slash_wrapped_form() {
        assert_eq!(normalize_base(""), "/");
        assert_eq!(normalize_base("/"), "/");
        assert_eq!(normalize_base("tools/demo"), "/tools/demo/");
        assert_eq!(normalize_base("/tools/demo/"), "/tools/demo/");
    }

    #[test]
    fn requests_resolve_inside_the_base_and_never_escape_the_root() {
        let root = fixture("resolve");
        assert_eq!(resolve(&root, "/", "/"), Some(root.join("index.html")));
        assert_eq!(
            resolve(&root, "/", "/sub/a.js?x=1"),
            Some(root.join("sub").join("a.js"))
        );
        assert_eq!(
            resolve(&root, "/tools/demo/", "/tools/demo/sub/a.js"),
            Some(root.join("sub").join("a.js"))
        );
        assert_eq!(
            resolve(&root, "/tools/demo/", "/tools/demo"),
            Some(root.join("index.html"))
        );
        assert_eq!(resolve(&root, "/tools/demo/", "/sub/a.js"), None);
        assert_eq!(resolve(&root, "/", "/../Cargo.toml"), None);
        assert_eq!(resolve(&root, "/", "/missing.js"), None);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn strict_policy_allows_wasm_but_no_script_eval() {
        let strict = CspMode::Strict.header_value().unwrap();
        assert!(strict.contains("'wasm-unsafe-eval'"));
        assert!(!strict.contains("'unsafe-eval'"));
        assert!(!strict.contains("'unsafe-inline'"));
        assert!(!CspMode::NoWasm
            .header_value()
            .unwrap()
            .contains("wasm-unsafe-eval"));
        assert!(CspMode::Off.header_value().is_none());
    }
}
