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
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
            content_length = value.trim().parse().unwrap_or(0);
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
    match resolve(&config.root, &config.base, target) {
        Some(file) => {
            let body = std::fs::read(&file)?;
            write_response(
                &mut stream,
                200,
                mime_type(&file),
                &body,
                config,
                method == "HEAD",
            )
        }
        None => write_response(&mut stream, 404, "text/plain", b"not found", config, false),
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    mime: &str,
    body: &[u8],
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
mod tests {
    use super::*;

    fn fixture() -> PathBuf {
        let root = std::env::temp_dir().join(format!("xtask-serve-{}", std::process::id()));
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("index.html"), "<html>").unwrap();
        std::fs::write(root.join("sub").join("a.js"), "1").unwrap();
        root
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
        let root = fixture();
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
