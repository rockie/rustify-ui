//! `rustify-docs preview`: a production output, served as a static host
//! would serve it.
//!
//! It reads nothing but the output: no rebuild, no watch, no script of its
//! own. Addresses outside the base, and addresses under it with nothing
//! behind them, get a real 404 with the site's not-found page; a directory
//! without its slash is redirected to it, as static hosts do; every response
//! carries the full content security policy as a header. While it runs it
//! holds the output's lock shared, so a build of the same output is refused
//! instead of swapping files under it.

use rustify_docs::diagnostic_text::{code_span, literal};
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rustify_docs::shell::CSP;

pub const DEFAULT_PORT: u16 = 4189;

/// What a preview serves.
pub struct Site {
    /// The output directory, resolved.
    pub dir: PathBuf,
    /// `/` or `/segments/`.
    pub base: String,
    /// The bytes of the output's `404.html`.
    pub not_found: Vec<u8>,
}

pub fn mime(path: &Path) -> &'static str {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    match extension.as_str() {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "json" | "map" => "application/json",
        "txt" => "text/plain; charset=utf-8",
        "xml" => "application/xml",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "pdf" => "application/pdf",
        "wasm" => "application/wasm",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        "wav" => "audio/wav",
        "vtt" => "text/vtt; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// Percent-decodes a request path; `None` for one that does not decode to
/// UTF-8 or holds a NUL or a backslash.
pub fn decode(path: &str) -> Option<String> {
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = |b: u8| (b as char).to_digit(16);
            let high = bytes.get(i + 1).copied().and_then(hex)?;
            let low = bytes.get(i + 2).copied().and_then(hex)?;
            out.push((high * 16 + low) as u8);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    let text = String::from_utf8(out).ok()?;
    (!text.contains(['\0', '\\'])).then_some(text)
}

/// What to answer for a request path.
#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    File(PathBuf),
    Redirect(String),
    NotFound,
    BadRequest,
}

/// Maps a request target onto the output.
pub fn resolve(site: &Site, target: &str) -> Answer {
    let raw_path = target.split(['?', '#']).next().unwrap_or("");
    let Some(path) = decode(raw_path) else {
        return Answer::BadRequest;
    };
    if !path.starts_with('/') {
        return Answer::BadRequest;
    }
    if format!("{path}/") == site.base {
        return Answer::Redirect(format!("{raw_path}/"));
    }
    let Some(rest) = path.strip_prefix(&site.base) else {
        return Answer::NotFound;
    };
    if rest
        .split('/')
        .any(|segment| segment == ".." || segment == ".")
    {
        return Answer::NotFound;
    }
    let mut file = site.dir.join(rest);
    if rest.is_empty() || rest.ends_with('/') {
        file.push("index.html");
    } else if file.is_dir() {
        return Answer::Redirect(format!("{raw_path}/"));
    }
    // Resolve links and check the file is still inside the output.
    match std::fs::canonicalize(&file) {
        Ok(real) if real.starts_with(&site.dir) && real.is_file() => Answer::File(real),
        _ => Answer::NotFound,
    }
}

fn respond(
    stream: &mut TcpStream,
    status: &str,
    headers: &[(&str, String)],
    body: &[u8],
    head_only: bool,
) -> std::io::Result<()> {
    let mut head = format!("HTTP/1.1 {status}\r\n");
    for (name, value) in headers {
        head.push_str(&format!("{name}: {value}\r\n"));
    }
    head.push_str(&format!(
        "Content-Length: {}\r\nContent-Security-Policy: {CSP}\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",
        body.len()
    ));
    stream.write_all(head.as_bytes())?;
    if !head_only {
        stream.write_all(body)?;
    }
    stream.flush()
}

fn handle(mut stream: TcpStream, site: &Site) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    // The headers are read and ignored; nothing here depends on them.
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
    }
    let mut parts = request_line.split_whitespace();
    let (method, target) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let head_only = method == "HEAD";
    if method != "GET" && !head_only {
        return respond(
            &mut stream,
            "405 Method Not Allowed",
            &[
                ("Allow", "GET, HEAD".into()),
                ("Content-Type", "text/plain; charset=utf-8".into()),
            ],
            b"405 method not allowed\n",
            false,
        );
    }
    match resolve(site, target) {
        Answer::File(path) => match std::fs::read(&path) {
            Ok(body) => respond(
                &mut stream,
                "200 OK",
                &[("Content-Type", mime(&path).into())],
                &body,
                head_only,
            ),
            Err(_) => respond(
                &mut stream,
                "404 Not Found",
                &[("Content-Type", "text/html; charset=utf-8".into())],
                &site.not_found,
                head_only,
            ),
        },
        Answer::Redirect(location) => respond(
            &mut stream,
            "301 Moved Permanently",
            &[
                ("Location", location),
                ("Content-Type", "text/plain; charset=utf-8".into()),
            ],
            b"moved\n",
            head_only,
        ),
        Answer::NotFound => respond(
            &mut stream,
            "404 Not Found",
            &[("Content-Type", "text/html; charset=utf-8".into())],
            &site.not_found,
            head_only,
        ),
        Answer::BadRequest => respond(
            &mut stream,
            "400 Bad Request",
            &[("Content-Type", "text/plain; charset=utf-8".into())],
            b"400 bad request\n",
            head_only,
        ),
    }
}

/// Binds `host:port`. Without an explicit port, a busy `default` falls back
/// to a port the system picks; an explicit busy port is an error.
pub fn bind(host: &str, port: Option<u16>, default: u16) -> Result<TcpListener, String> {
    let address = |port: u16| -> Result<SocketAddr, String> {
        let text = if host.contains(':') {
            format!("[{host}]:{port}")
        } else {
            format!("{host}:{port}")
        };
        text.parse()
            .map_err(|_| format!("{} is not an IP address to listen on", code_span(host)))
    };
    match port {
        Some(port) => TcpListener::bind(address(port)?)
            .map_err(|error| format!("cannot listen on {}:{port}: {error}", literal(host))),
        None => {
            let (default, any) = (address(default)?, address(0)?);
            TcpListener::bind(default)
                .or_else(|_| TcpListener::bind(any))
                .map_err(|error| format!("cannot listen on {}: {error}", literal(host)))
        }
    }
}

/// Serves `site` on `listener` until the process ends.
pub fn serve(listener: TcpListener, site: Site) {
    let site = Arc::new(site);
    for stream in listener.incoming().flatten() {
        let site = site.clone();
        std::thread::spawn(move || {
            let _ = handle(stream, &site);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> (Site, PathBuf) {
        let dir = std::env::temp_dir().join(format!("rustify-docs-preview-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("guide")).unwrap();
        std::fs::write(dir.join("index.html"), "home").unwrap();
        std::fs::write(dir.join("guide/index.html"), "guide").unwrap();
        std::fs::write(dir.join("a b.txt"), "space").unwrap();
        let real = std::fs::canonicalize(&dir).unwrap();
        (
            Site {
                dir: real.clone(),
                base: "/docs/".into(),
                not_found: b"404".to_vec(),
            },
            real,
        )
    }

    #[test]
    fn requests_map_onto_the_output_and_nothing_else() {
        let (site, dir) = site();
        assert_eq!(
            resolve(&site, "/docs/"),
            Answer::File(dir.join("index.html"))
        );
        assert_eq!(
            resolve(&site, "/docs/guide/?x=1"),
            Answer::File(dir.join("guide/index.html"))
        );
        assert_eq!(
            resolve(&site, "/docs/a%20b.txt"),
            Answer::File(dir.join("a b.txt"))
        );
        assert_eq!(resolve(&site, "/docs"), Answer::Redirect("/docs/".into()));
        assert_eq!(
            resolve(&site, "/docs/guide"),
            Answer::Redirect("/docs/guide/".into())
        );
        assert_eq!(resolve(&site, "/"), Answer::NotFound);
        assert_eq!(resolve(&site, "/docs/no/such/page/"), Answer::NotFound);
        assert_eq!(resolve(&site, "/docs/%2e%2e/etc/passwd"), Answer::NotFound);
        assert_eq!(resolve(&site, "/docs/..%2f..%2fetc"), Answer::NotFound);
        assert_eq!(resolve(&site, "/docs/a%5cb"), Answer::BadRequest);
        assert_eq!(resolve(&site, "/docs/%ff"), Answer::BadRequest);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
