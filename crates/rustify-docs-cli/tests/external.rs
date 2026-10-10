//! `check --external-links` against a server on this machine that records
//! every request: each address is asked once, a refused HEAD is retried as
//! GET, redirects are followed, and an address that answers 400 or more, or
//! not at all, is an error at every place it is written. Without the flag,
//! not one request is made.

mod support;

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use support::*;

/// Requests by "METHOD /path", counted.
type Seen = Arc<Mutex<BTreeMap<String, usize>>>;

/// A server answering `/ok` 200, `/missing` 404, `/get-only` 405 to HEAD and
/// 200 to GET, and `/moved` with a redirect to `/ok`.
fn server() -> (String, Seen) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let seen: Seen = Arc::default();
    let record = seen.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            if reader.read_line(&mut line).is_err() {
                continue;
            }
            loop {
                let mut header = String::new();
                if reader.read_line(&mut header).unwrap_or(0) == 0 || header == "\r\n" {
                    break;
                }
            }
            let mut parts = line.split_whitespace();
            let (method, path) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
            *record
                .lock()
                .unwrap()
                .entry(format!("{method} {path}"))
                .or_default() += 1;
            let status = match (method, path) {
                (_, "/ok") => "200 OK",
                (_, "/missing") => "404 Not Found",
                ("HEAD", "/get-only") => "405 Method Not Allowed",
                (_, "/get-only") => "200 OK",
                (_, "/moved") => "301 Moved Permanently\r\nLocation: /ok",
                _ => "500 Internal Server Error",
            };
            let mut stream = stream;
            let _ = write!(
                stream,
                "HTTP/1.1 {status}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
        }
    });
    (origin, seen)
}

/// A port with nothing listening on it.
fn closed_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

fn site(origin: &str, closed: u16) -> Temp {
    let temp = Temp::new("external");
    temp.write(
        "site/rustify-docs.toml",
        &format!(
            "schema_version = 1\n\n[site]\nname = \"Links\"\nurl = \"https://example.com/\"\n\n[[nav.top]]\nlabel = \"Moved\"\nlink = \"{origin}/moved\"\n"
        ),
    );
    temp.write(
        "site/content/index.md",
        &format!(
            "---\ndescription: Home.\n---\n\n# Home\n\n[Working]({origin}/ok) and [missing]({origin}/missing#part).\n\nA [GET-only server]({origin}/get-only), and [one that is down](http://127.0.0.1:{closed}/).\n"
        ),
    );
    temp.write(
        "site/content/guide.md",
        &format!(
            "---\ndescription: Guide.\n---\n\n# Guide\n\nThe [same missing page]({origin}/missing) again, and [working]({origin}/ok).\n"
        ),
    );
    temp
}

fn check(temp: &Temp, extra: &[&str]) -> (Option<i32>, Value) {
    let config = temp.path("site/rustify-docs.toml");
    let mut args = vec![
        "check",
        "--config",
        config.to_str().unwrap(),
        "--format",
        "json",
    ];
    args.extend_from_slice(extra);
    let output = run_in(&temp.0, &args);
    let json: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("{error}: {}{}", stdout(&output), stderr(&output)));
    (output.status.code(), json)
}

#[test]
fn without_the_flag_check_requests_nothing() {
    let (origin, seen) = server();
    let temp = site(&origin, closed_port());
    let (code, json) = check(&temp, &[]);
    assert_eq!(code, Some(0), "{json}");
    assert!(json["summary"].get("external_addresses").is_none());
    assert!(
        seen.lock().unwrap().is_empty(),
        "{:?}",
        seen.lock().unwrap()
    );
}

#[test]
fn each_address_is_asked_once_and_failures_are_errors_where_written() {
    let (origin, seen) = server();
    let closed = closed_port();
    let temp = site(&origin, closed);
    let (code, json) = check(&temp, &["--external-links"]);
    assert_eq!(code, Some(1), "{json}");
    assert_eq!(json["summary"]["external_addresses"], 5, "{json}");
    let errors: Vec<&Value> = json["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"] == "error")
        .collect();
    let problems: Vec<(String, String, u64)> = errors
        .iter()
        .map(|d| {
            (
                d["rule"].as_str().unwrap().to_owned(),
                d["file"].as_str().unwrap().to_owned(),
                d["line"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        problems,
        vec![
            ("external-link".to_owned(), "content/guide.md".to_owned(), 7),
            ("external-link".to_owned(), "content/index.md".to_owned(), 7),
            (
                "external-link-unreachable".to_owned(),
                "content/index.md".to_owned(),
                9
            ),
        ],
        "{json}"
    );
    for d in &errors {
        assert!(
            d["column"].as_u64().is_some() && d["suggestion"].is_string(),
            "{d}"
        );
    }
    let missing = &errors[0]["message"];
    assert!(
        missing.as_str().unwrap().contains("answered 404"),
        "{missing}"
    );
    let seen = seen.lock().unwrap().clone();
    assert_eq!(
        seen,
        BTreeMap::from([
            ("GET /get-only".to_owned(), 1),
            ("HEAD /get-only".to_owned(), 1),
            ("HEAD /missing".to_owned(), 1),
            ("HEAD /moved".to_owned(), 1),
            ("HEAD /ok".to_owned(), 2),
        ]),
        "one request per address (and one more for the redirect's target)"
    );
}
