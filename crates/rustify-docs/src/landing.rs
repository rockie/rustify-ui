//! Landing pages: the named profiles in `rustify-docs.toml` that let a
//! `layout: landing` page load third-party scripts, send measurements,
//! embed frames and post forms, and the one place their sources become a
//! page's policy.
//!
//! Every other page keeps [`crate::shell::CSP`] byte for byte. A profile
//! only widens the directives it lists, and only with origins in the narrow
//! `https://host[:port]` form, so the policy a page carries can be read off
//! the configuration. The checks here mirror CSP's host-source matching for
//! that form; the browser stays the final judge.

use std::ops::Range;

use serde::Deserialize;

use crate::config::NavLink;
use crate::diagnostic_text::code_span;

/// How a profile's script loads; neither blocks parsing.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Load {
    Async,
    #[default]
    Defer,
}

impl Load {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Async => "async",
            Self::Defer => "defer",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScriptSrc {
    /// A project-relative file, published under its content fingerprint.
    Local(String),
    /// An absolute address, written into the page as given.
    External(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Script {
    pub src: ScriptSrc,
    pub load: Load,
    pub span: Range<usize>,
}

/// Origins a profile adds to each directive, normalized and in the order
/// they were written.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Sources {
    pub script: Vec<String>,
    pub connect: Vec<String>,
    pub img: Vec<String>,
    pub frame: Vec<String>,
    pub form: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub scripts: Vec<Script>,
    pub sources: Sources,
    /// Query parameter names the theme script copies into marked elements.
    pub carry: Vec<String>,
    pub footer: Vec<NavLink>,
    /// Where the profile is declared.
    pub span: Range<usize>,
}

/// A profile name: what a page's `landing:` writes.
pub fn valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    name.len() <= 32
        && chars
            .next()
            .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// A parameter name a profile may carry.
pub fn valid_carry(name: &str) -> bool {
    (1..=64).contains(&name.len())
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
}

fn default_port(scheme: &str) -> u16 {
    if scheme == "http" {
        80
    } else {
        443
    }
}

/// A host the browser treats as an IP address: a bracketed IPv6 literal, or
/// one whose last label is a number (WHATWG's IPv4 parser takes it).
fn ip_literal(host: &str) -> bool {
    let last = host.rsplit('.').next().unwrap_or(host);
    host.starts_with('[')
        || (!last.is_empty() && last.chars().all(|c| c.is_ascii_digit()))
        || last
            .strip_prefix("0x")
            .is_some_and(|hex| hex.chars().all(|c| c.is_ascii_hexdigit()))
}

/// A lower-case ASCII domain of letters, digits and `-`; `127.0.0.1` is the
/// one address CSP's host matching accepts.
fn valid_host(host: &str) -> bool {
    if host == "127.0.0.1" {
        return true;
    }
    !ip_literal(host)
        && host.split('.').all(|label| {
            !label.is_empty()
                && label
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        })
}

fn loopback(host: &str) -> bool {
    matches!(host, "127.0.0.1" | "localhost")
}

/// Splits `scheme://host[:port]`, the authority of an http(s) URL, without
/// judging the host: the scheme lower-cased, the host as written and the
/// port when one is written. `None` for anything else.
fn authority(url: &str) -> Option<(String, &str, Option<&str>)> {
    let (scheme, rest) = url.split_once("://")?;
    let scheme = scheme.to_ascii_lowercase();
    if scheme != "https" && scheme != "http" {
        return None;
    }
    let end = rest.find(['/', '?', '#', '\\']).unwrap_or(rest.len());
    let authority = &rest[..end];
    // The host after any user information, as a URL parser reads it.
    let authority = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host)| host);
    if authority.starts_with('[') {
        return Some((scheme, authority, None));
    }
    Some(match authority.split_once(':') {
        Some((host, port)) => (scheme, host, Some(port)),
        None => (scheme, authority, None),
    })
}

fn port(text: &str) -> Option<u16> {
    (!text.is_empty() && text.len() <= 5 && text.chars().all(|c| c.is_ascii_digit()))
        .then(|| text.parse().ok())
        .flatten()
        .filter(|&port| port > 0)
}

/// `scheme://host[:port]` lower-cased, the scheme's default port dropped.
fn origin(scheme: &str, host: &str, port: Option<u16>) -> String {
    match port {
        Some(port) if port != default_port(scheme) => format!("{scheme}://{host}:{port}"),
        _ => format!("{scheme}://{host}"),
    }
}

const SOURCE_FORM: &str =
    "write an origin like `https://www.example.com`, `https://*.example.com` or `https://example.com:8443`";

/// A profile source: `https://host[:port]` or `https://*.suffix[:port]`,
/// with `http://` only for `127.0.0.1` and `localhost`. Returns the origin
/// normalized: lower case, without the scheme's default port.
pub fn parse_source(text: &str) -> Result<String, String> {
    let fail = |why: &str| Err(format!("{} {why}; {SOURCE_FORM}", code_span(text)));
    if text.is_empty() || text.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return fail("is not an origin");
    }
    if text.starts_with('\'') || text.ends_with(':') && !text.contains("//") {
        return fail("is a keyword or scheme, which a profile cannot add");
    }
    let Some((scheme, host, written_port)) = authority(text) else {
        return fail("is not an http(s) origin");
    };
    let rest = &text[text.find("://").map_or(0, |i| i + 3)..];
    if rest.contains(['/', '?', '#', '\\']) {
        return fail("has a path, query or fragment");
    }
    if rest.contains('@') {
        return fail("has user information");
    }
    let host = host.to_ascii_lowercase();
    let port = match written_port {
        None => None,
        Some(text) => match port(text) {
            Some(port) => Some(port),
            None => return fail("has a port that is not a number from 1 to 65535"),
        },
    };
    let (wildcard, domain) = match host.strip_prefix("*.") {
        Some(suffix) => (true, suffix),
        None => (false, host.as_str()),
    };
    if !domain.is_ascii() {
        return fail("has a non-ASCII host; write its punycode (`xn--…`) form");
    }
    if domain.starts_with('[') || (ip_literal(domain) && domain != "127.0.0.1") {
        return fail("is an IP address; CSP host matching takes only domain names and `127.0.0.1`");
    }
    if !valid_host(domain) || (wildcard && (!domain.contains('.') || domain == "127.0.0.1")) {
        return fail("does not have a valid host");
    }
    if scheme == "http" && (wildcard || !loopback(domain)) {
        return fail("uses `http://`, which only `127.0.0.1` and `localhost` may");
    }
    Ok(origin(&scheme, &host, port))
}

/// A profile script's `src`: an absolute https address (or http on a
/// loopback host), whose origin joins `script-src`, or else a
/// project-relative `.js` file.
pub fn parse_script_src(text: &str) -> Result<ScriptSrc, String> {
    if text.is_empty() || text.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(format!(
            "script src {} is empty or has spaces",
            code_span(text)
        ));
    }
    if let Some((scheme, host, port)) = authority(text) {
        let written = &text[..text.find("://").map_or(0, |i| i + 3)];
        let rest = &text[written.len()..];
        let authority = &rest[..rest.find(['/', '?', '#', '\\']).unwrap_or(rest.len())];
        if authority.contains('@') {
            return Err(format!(
                "script src {} has user information",
                code_span(text)
            ));
        }
        let origin = match port {
            Some(port) => format!("{scheme}://{host}:{port}"),
            None => format!("{scheme}://{host}"),
        };
        return match parse_source(&origin) {
            Ok(_) if !host.starts_with('*') => Ok(ScriptSrc::External(text.to_owned())),
            Ok(_) => Err(format!(
                "script src {} has a wildcard host",
                code_span(text)
            )),
            Err(message) => Err(format!("script src: {message}")),
        };
    }
    if crate::content::has_scheme(text) {
        return Err(format!(
            "script src {} is neither an http(s) address nor a project file",
            code_span(text)
        ));
    }
    let path =
        crate::config::relative_path(text).map_err(|message| format!("script src: {message}"))?;
    if !path.ends_with(".js") || path.rsplit('/').next() == Some(".js") {
        return Err(format!(
            "script src {} is not a `.js` file in the project",
            code_span(text)
        ));
    }
    Ok(ScriptSrc::Local(path))
}

/// The origin of a validated external script address.
fn script_origin(src: &str) -> Option<String> {
    let (scheme, host, port) = authority(src)?;
    let port = match port {
        Some(text) => Some(self::port(text)?),
        None => None,
    };
    Some(origin(&scheme, &host.to_ascii_lowercase(), port))
}

fn push_unique(list: &mut Vec<String>, value: &str) {
    if !list.iter().any(|known| known == value) {
        list.push(value.to_owned());
    }
}

fn directive(name: &str, base: &str, sources: &[String]) -> String {
    let mut text = format!("{name} {base}");
    for source in sources {
        text.push(' ');
        text.push_str(source);
    }
    text
}

/// The origins a profile adds to `script-src`: its `sources.script`, then
/// the origin of each external script not already among them.
pub fn script_sources(profile: &Profile) -> Vec<String> {
    let mut script = Vec::new();
    for source in &profile.sources.script {
        push_unique(&mut script, source);
    }
    for entry in &profile.scripts {
        if let ScriptSrc::External(src) = &entry.src {
            if let Some(origin) = script_origin(src) {
                push_unique(&mut script, &origin);
            }
        }
    }
    script
}

/// The policy a page is served under, as a header. Without a profile it is
/// [`crate::shell::CSP`] itself.
pub fn policy(profile: Option<&Profile>) -> String {
    let empty = Sources::default();
    let sources = profile.map_or(&empty, |profile| &profile.sources);
    let script = profile.map(script_sources).unwrap_or_default();
    let mut directives = vec![
        "default-src 'none'".to_owned(),
        directive("script-src", "'self' 'wasm-unsafe-eval'", &script),
        "style-src 'self'".to_owned(),
        directive("img-src", "'self' data:", &sources.img),
        "font-src 'self'".to_owned(),
        directive("connect-src", "'self'", &sources.connect),
        "media-src 'self'".to_owned(),
        "worker-src 'self'".to_owned(),
    ];
    if let Some((first, rest)) = sources.frame.split_first() {
        directives.push(directive("frame-src", first, rest));
    }
    directives.push("object-src 'none'".to_owned());
    directives.push("base-uri 'none'".to_owned());
    directives.push(if sources.form.is_empty() {
        "form-action 'none'".to_owned()
    } else {
        directive("form-action", "'self'", &sources.form)
    });
    directives.push("frame-ancestors 'none'".to_owned());
    directives.join("; ")
}

/// [`policy`] without the one directive a `meta` element cannot deliver.
pub fn meta_policy(profile: Option<&Profile>) -> String {
    policy(profile)
        .split("; ")
        .filter(|directive| !directive.starts_with("frame-ancestors"))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Whether one of `sources` (as [`parse_source`] returns them) lets the
/// browser load `url`: the same scheme, the host itself or under a wildcard
/// suffix, and the same port, the scheme's default when none is written.
/// IP addresses other than `127.0.0.1` never match.
pub fn allows(sources: &[String], url: &str) -> bool {
    let Some((scheme, host, port)) = authority(url.trim()) else {
        return false;
    };
    let host = host.to_ascii_lowercase();
    if host.is_empty() || (ip_literal(&host) && host != "127.0.0.1") {
        return false;
    }
    let port = match port {
        None => default_port(&scheme),
        Some(text) => match self::port(text) {
            Some(port) => port,
            None => return false,
        },
    };
    sources.iter().any(|source| {
        let Some((source_scheme, source_host, source_port)) = authority(source) else {
            return false;
        };
        let source_port = source_port
            .and_then(self::port)
            .unwrap_or(default_port(&source_scheme));
        let host_matches = match source_host.strip_prefix("*.") {
            Some(suffix) => host
                .strip_suffix(suffix)
                .is_some_and(|label| label.ends_with('.') && label.len() > 1),
            None => host == source_host,
        };
        source_scheme == scheme && host_matches && source_port == port
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile(sources: Sources, scripts: &[(&str, Load)]) -> Profile {
        Profile {
            name: "ads".into(),
            scripts: scripts
                .iter()
                .map(|(src, load)| Script {
                    src: parse_script_src(src).unwrap(),
                    load: *load,
                    span: 0..0,
                })
                .collect(),
            sources,
            carry: Vec::new(),
            footer: Vec::new(),
            span: 0..0,
        }
    }

    #[test]
    fn no_profile_is_the_strict_policy() {
        assert_eq!(policy(None), crate::shell::CSP);
        assert_eq!(meta_policy(None), crate::shell::meta_csp());
        let empty = profile(Sources::default(), &[]);
        assert_eq!(policy(Some(&empty)), crate::shell::CSP);
    }

    #[test]
    fn a_profile_widens_only_the_directives_it_lists() {
        let ads = profile(
            Sources {
                script: vec!["https://www.googletagmanager.com".into()],
                connect: vec!["https://*.google-analytics.com".into()],
                img: vec!["https://www.google.com".into()],
                frame: vec![
                    "https://www.googletagmanager.com".into(),
                    "https://jinshuju.net".into(),
                ],
                form: vec!["https://leads.example.com".into()],
            },
            &[
                (
                    "https://www.googletagmanager.com/gtag/js?id=AW-1",
                    Load::Async,
                ),
                ("http://127.0.0.1:4211/tag.js", Load::Async),
                ("static/landing/init.js", Load::Defer),
            ],
        );
        assert_eq!(
            policy(Some(&ads)),
            "default-src 'none'; script-src 'self' 'wasm-unsafe-eval' https://www.googletagmanager.com http://127.0.0.1:4211; style-src 'self'; img-src 'self' data: https://www.google.com; font-src 'self'; connect-src 'self' https://*.google-analytics.com; media-src 'self'; worker-src 'self'; frame-src https://www.googletagmanager.com https://jinshuju.net; object-src 'none'; base-uri 'none'; form-action 'self' https://leads.example.com; frame-ancestors 'none'"
        );
        assert_eq!(
            meta_policy(Some(&ads)),
            policy(Some(&ads))
                .strip_suffix("; frame-ancestors 'none'")
                .unwrap()
        );
        let scripts_only = profile(
            Sources::default(),
            &[("https://hm.baidu.com/hm.js?x", Load::Async)],
        );
        assert_eq!(
            policy(Some(&scripts_only)),
            crate::shell::CSP.replace(
                "'wasm-unsafe-eval'",
                "'wasm-unsafe-eval' https://hm.baidu.com"
            )
        );
    }

    #[test]
    fn sources_accept_only_narrow_origins() {
        for (text, normalized) in [
            ("https://www.example.com", "https://www.example.com"),
            ("HTTPS://WWW.Example.COM", "https://www.example.com"),
            ("https://example.com:443", "https://example.com"),
            ("https://example.com:8443", "https://example.com:8443"),
            (
                "https://*.google-analytics.com",
                "https://*.google-analytics.com",
            ),
            ("https://xn--fiqs8s.example", "https://xn--fiqs8s.example"),
            ("http://127.0.0.1:4211", "http://127.0.0.1:4211"),
            ("http://localhost:80", "http://localhost"),
            ("https://127.0.0.1", "https://127.0.0.1"),
        ] {
            assert_eq!(parse_source(text).as_deref(), Ok(normalized), "{text}");
        }
        for text in [
            "",
            "*",
            "https:",
            "https://",
            "'self'",
            "'unsafe-inline'",
            "'unsafe-eval'",
            "'nonce-abc'",
            "'sha256-abc='",
            "data:",
            "blob:",
            "*://example.com",
            "ftp://example.com",
            "example.com",
            "//example.com",
            "https://example.com/",
            "https://example.com/path",
            "https://example.com?x",
            "https://example.com#x",
            "https://user@example.com",
            "https://example.com:*",
            "https://example.com:0",
            "https://example.com:65536",
            "https://example.com:x",
            "https://*",
            "https://*.com",
            "https://a.*.example.com",
            "https://exa mple.com",
            "https://example.com\n; script-src *",
            "https://例子.com",
            "https://exa_mple.com",
            "https://example..com",
            "http://example.com",
            "http://*.localhost",
            "http://[::1]:4211",
            "https://[::1]",
            "https://10.0.0.1",
            "https://8.8.8.8:443",
            "https://0x7f.1",
            "https://2130706433",
        ] {
            assert!(parse_source(text).is_err(), "{text:?}");
        }
    }

    #[test]
    fn script_sources_are_addresses_or_project_files() {
        assert_eq!(
            parse_script_src("https://www.googletagmanager.com/gtag/js?id=G-1"),
            Ok(ScriptSrc::External(
                "https://www.googletagmanager.com/gtag/js?id=G-1".into()
            ))
        );
        assert_eq!(
            parse_script_src("http://127.0.0.1:4211/tag.js"),
            Ok(ScriptSrc::External("http://127.0.0.1:4211/tag.js".into()))
        );
        assert_eq!(
            parse_script_src("static/./landing/init.js"),
            Ok(ScriptSrc::Local("static/landing/init.js".into()))
        );
        for text in [
            "",
            "static/init.js extra",
            "http://example.com/tag.js",
            "http://[::1]:4211/tag.js",
            "https://[::1]/tag.js",
            "https://10.0.0.1/tag.js",
            "https://user@example.com/tag.js",
            "https://*.example.com/tag.js",
            "https://例子.com/tag.js",
            "javascript:alert(1)",
            "data:text/javascript,1",
            "../outside.js",
            "/static/init.js",
            "static/init.txt",
            "static/",
        ] {
            assert!(parse_script_src(text).is_err(), "{text:?}");
        }
    }

    #[test]
    fn names_and_carried_parameters() {
        for name in ["ads", "a", "baidu-2", "0", &"a".repeat(32)] {
            assert!(valid_name(name), "{name}");
        }
        for name in ["", "-ads", "Ads", "ads_2", "ads.x", &"a".repeat(33)] {
            assert!(!valid_name(name), "{name}");
        }
        for name in ["gclid", "utm_source", "bd_vid", "a.b-c", &"x".repeat(64)] {
            assert!(valid_carry(name), "{name}");
        }
        for name in ["", "a b", "a=b", "a&b", "名", &"x".repeat(65)] {
            assert!(!valid_carry(name), "{name}");
        }
    }

    #[test]
    fn matching_follows_host_source_rules() {
        let sources: Vec<String> = [
            "https://www.example.com",
            "https://*.cdn.example",
            "https://api.example.com:8443",
            "http://127.0.0.1:4211",
        ]
        .map(String::from)
        .into();
        for url in [
            "https://www.example.com",
            "https://www.example.com/x?y#z",
            "https://WWW.EXAMPLE.COM:443/x",
            "https://a.cdn.example/x",
            "https://a.b.cdn.example/x",
            "https://api.example.com:8443/lead",
            "http://127.0.0.1:4211/tag.js",
        ] {
            assert!(allows(&sources, url), "{url}");
        }
        for url in [
            "http://www.example.com",
            "https://www.example.com:8443",
            "https://example.com",
            "https://evil-www.example.com",
            "https://cdn.example/x",
            "https://acdn.example/x",
            "https://api.example.com/lead",
            "http://127.0.0.1:4212/tag.js",
            "http://127.0.0.1/tag.js",
            "http://localhost:4211/tag.js",
            "http://[::1]:4211/tag.js",
            "https://10.0.0.1/x",
            "//www.example.com/x",
            "/lead",
            "lead",
            "javascript:alert(1)",
            "https://www.example.com:x/",
        ] {
            assert!(!allows(&sources, url), "{url}");
        }
        assert!(allows(&sources, "https://evil.test@www.example.com/x"));
        assert!(!allows(&sources, "https://www.example.com@evil.test/x"));
        assert!(!allows(
            &["http://[::1]:4211".to_owned()],
            "http://[::1]:4211/"
        ));
    }
}
