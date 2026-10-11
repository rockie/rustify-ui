//! Raw HTML in Markdown, checked against the site's policy before it ships.
//!
//! The policy is the strict CSP's, applied at build time so a violation is a
//! located error rather than a blocked request in a reader's browser: no
//! script or style written into the page, no event handler, no `javascript:`
//! URL, no embedded frame or plugin, and every resource the page loads from
//! the site itself. The HTML is tokenized as a browser tokenizes it
//! (html5gum, WHATWG), so case, entities and malformed markup decode the way
//! they would there.
//!
//! The same pass records each URL attribute's raw value range, so links and
//! resources in author HTML are resolved and rewritten like Markdown ones.

use std::ops::Range;

use html5gum::{Token, Tokenizer};

use crate::diagnostic::Issue;

/// What an attribute's URL is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UrlRole {
    /// Navigation: `a href`, `area href`.
    Link,
    /// Something the page loads: an image, media, a track, an SVG reference.
    Resource { image: bool },
    /// A `srcset`, which lists several image URLs.
    Srcset,
    /// An image a class loads through the stylesheet: the URL in a
    /// `url(…)` such as `bg-[url(/bg.svg)]`, which the class carries into
    /// the stylesheet as written.
    ClassImage,
}

/// A URL attribute in author HTML.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UrlSlot {
    /// The raw value between its quotes (or the unquoted value), as offsets
    /// into the checked chunk: what a rewrite replaces.
    pub span: Range<usize>,
    pub role: UrlRole,
    /// The value as a browser reads it, with references decoded.
    pub value: String,
    pub element: String,
    pub attribute: String,
}

/// Elements whose presence alone breaks the policy.
const FORBIDDEN: &[(&str, &str, &str)] = &[
    ("script", "inline-script", "a script written into the page"),
    ("style", "inline-style", "a <style> element"),
    ("iframe", "embedded-frame", "an <iframe>"),
    ("frame", "embedded-frame", "a <frame>"),
    ("object", "embedded-plugin", "an <object>"),
    ("embed", "embedded-plugin", "an <embed>"),
    (
        "base",
        "base-element",
        "a <base> element, which would move every relative URL",
    ),
    ("meta", "meta-element", "a <meta> element in content"),
    ("link", "link-element", "a <link> element in content"),
];

/// The URL attributes per element, and what they load.
fn url_role(element: &str, attribute: &str) -> Option<UrlRole> {
    Some(match (element, attribute) {
        ("a" | "area", "href") => UrlRole::Link,
        ("img", "src") | ("input", "src") => UrlRole::Resource { image: true },
        ("img" | "source", "srcset") => UrlRole::Srcset,
        ("source", "src") | ("video" | "audio" | "track", "src") => {
            UrlRole::Resource { image: false }
        }
        ("video", "poster") => UrlRole::Resource { image: true },
        ("image" | "use" | "feimage", "href" | "xlink:href") => UrlRole::Resource { image: true },
        (_, "background") => UrlRole::Resource { image: true },
        _ => return None,
    })
}

/// A URL a browser would run as script, whatever its case, entities or
/// embedded whitespace.
fn is_script_url(value: &str) -> bool {
    let compact: String = value
        .chars()
        .filter(|c| !c.is_ascii_whitespace() && !c.is_control())
        .flat_map(char::to_lowercase)
        .collect();
    compact.starts_with("javascript:") || compact.starts_with("vbscript:")
}

/// Whether a resource URL leaves the site: an absolute URL with a scheme
/// other than `data:` for images, or a scheme-relative one.
pub fn is_cross_origin(value: &str, image: bool) -> bool {
    let value = value.trim();
    if value.starts_with("//") {
        return true;
    }
    match value.split_once(':') {
        Some((scheme, _))
            if scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.') =>
        {
            !(image && scheme.eq_ignore_ascii_case("data"))
        }
        _ => false,
    }
}

/// A `srcset`'s candidates as a browser splits them: each URL with its
/// descriptor (`2x`, `640w`, or empty). A URL may hold commas (a `data:`
/// URL does), so the split is by whitespace first, as the HTML standard's
/// algorithm does, not by comma.
pub fn parse_srcset(value: &str) -> Vec<(String, String)> {
    let mut candidates = Vec::new();
    let mut rest = value;
    loop {
        rest = rest.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == ',');
        if rest.is_empty() {
            break;
        }
        let end = rest
            .find(|c: char| c.is_ascii_whitespace())
            .unwrap_or(rest.len());
        let url = &rest[..end];
        rest = &rest[end..];
        if url.ends_with(',') {
            candidates.push((url.trim_end_matches(',').to_owned(), String::new()));
            continue;
        }
        // The descriptor runs to the next comma outside parentheses.
        let mut depth = 0usize;
        let mut stop = rest.len();
        for (i, c) in rest.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => {
                    stop = i;
                    break;
                }
                _ => {}
            }
        }
        candidates.push((url.to_owned(), rest[..stop].trim().to_owned()));
        rest = &rest[stop..];
    }
    candidates
}

/// Whether a link opens another site in a new window without cutting that
/// window's handle on this one (`rel="noopener"`, or `noreferrer`, which
/// implies it). `attribute` looks an attribute up by its lowercase name.
fn opens_unguarded_window(attribute: impl Fn(&str) -> Option<String>) -> bool {
    let Some(target) = attribute("target") else {
        return false;
    };
    let target = target.trim().to_ascii_lowercase();
    if target.is_empty() || matches!(target.as_str(), "_self" | "_parent" | "_top") {
        return false;
    }
    let href = attribute("href").unwrap_or_default();
    if !is_cross_origin(&href, false) {
        return false;
    }
    let rel = attribute("rel").unwrap_or_default().to_ascii_lowercase();
    !rel.split_ascii_whitespace()
        .any(|rel| rel == "noopener" || rel == "noreferrer")
}

/// The URLs of the `url(…)`s in a class attribute's raw value, each with
/// its range in `value`, quotes excluded.
fn class_urls(value: &str) -> Vec<(String, Range<usize>)> {
    let mut urls = Vec::new();
    let mut from = 0;
    while let Some(at) = value[from..].find("url(") {
        let open = from + at + "url(".len();
        let (start, close) = match value[open..].chars().next() {
            Some(quote @ ('"' | '\'')) => (
                open + 1,
                value[open + 1..].find(quote).map(|end| open + 1 + end),
            ),
            _ => (open, value[open..].find(')').map(|end| open + end)),
        };
        let Some(end) = close else {
            break;
        };
        urls.push((value[start..end].to_owned(), start..end));
        from = end;
    }
    urls
}

/// The raw value range of an attribute, given the span html5gum reports for
/// it (name through value).
fn value_range(source: &str, span: Range<usize>) -> Option<Range<usize>> {
    let text = source.get(span.clone())?;
    let equals = text.find('=')?;
    let after = &text[equals + 1..];
    let lead = after.len() - after.trim_start().len();
    let start = span.start + equals + 1 + lead;
    let rest = &source[start..];
    match rest.chars().next()? {
        quote @ ('"' | '\'') => {
            let end = rest[1..].find(quote)?;
            Some(start + 1..start + 1 + end)
        }
        _ => {
            let end = rest
                .find(|c: char| c.is_ascii_whitespace() || c == '>')
                .unwrap_or(rest.len());
            Some(start..start + end)
        }
    }
}

/// Checks one chunk of raw HTML. Issue spans and slots are offsets into
/// `chunk`; the caller places them in the file and in its output.
pub fn check(chunk: &str) -> (Vec<Issue>, Vec<UrlSlot>) {
    let mut issues = Vec::new();
    let mut slots = Vec::new();
    let mut emitter = html5gum::DefaultEmitter::new_with_span();
    // Inside <script>, <style> and the like, text is text, as in a browser.
    emitter.naively_switch_states(true);
    for token in Tokenizer::new_with_emitter(chunk, emitter) {
        let Ok(token) = token;
        let Token::StartTag(tag) = token else {
            continue;
        };
        let name = String::from_utf8_lossy(&tag.name).to_ascii_lowercase();
        let tag_span = tag.span.start..tag.span.end;
        if let Some((_, rule, what)) = FORBIDDEN.iter().find(|(element, _, _)| *element == name) {
            issues.push(
                Issue::error(tag_span.clone(), rule, format!("{what} is not allowed: the site's content security policy would block it"))
                    .suggest("put behaviour in the theme's scripts and styling in classes; reference media from static/"),
            );
            continue;
        }
        let attribute = |key: &str| {
            tag.attributes
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(key.as_bytes()))
                .map(|(_, value)| String::from_utf8_lossy(&value.value).into_owned())
        };
        if matches!(name.as_str(), "a" | "area") && opens_unguarded_window(attribute) {
            issues.push(
                Issue::error(
                    tag_span.clone(),
                    "new-window-opener",
                    "this link opens another site in a new window without rel=\"noopener\"",
                )
                .suggest("add rel=\"noopener\", or drop target"),
            );
        }
        if name == "img"
            && !tag
                .attributes
                .keys()
                .any(|key| key.eq_ignore_ascii_case(b"alt"))
        {
            issues.push(
                Issue::warning(
                    tag_span.clone(),
                    "missing-alt",
                    "this image has no alt attribute",
                )
                .suggest("add alt=\"…\" describing it, or alt=\"\" if it is decoration"),
            );
        }
        for (attribute, value) in &tag.attributes {
            let attribute = String::from_utf8_lossy(attribute).to_ascii_lowercase();
            let decoded = String::from_utf8_lossy(&value.value).into_owned();
            let span = value.span.start..value.span.end;
            if attribute.starts_with("on") {
                issues.push(Issue::error(
                    span.clone(),
                    "event-handler",
                    format!("`{attribute}` runs inline script, which the policy blocks"),
                ));
                continue;
            }
            if attribute == "style" {
                issues.push(
                    Issue::error(
                        span.clone(),
                        "inline-style",
                        "a `style` attribute is blocked by the policy",
                    )
                    .suggest("use a class"),
                );
                continue;
            }
            if is_script_url(&decoded) {
                issues.push(Issue::error(
                    span.clone(),
                    "script-url",
                    format!("`{attribute}` holds a script URL"),
                ));
                continue;
            }
            // A class's `url(…)` is loaded by the stylesheet the class
            // becomes: a resource like any other, rewritten in the class.
            if attribute == "class" {
                let Some(range) = value_range(chunk, value.span.start..value.span.end) else {
                    continue;
                };
                for (url, at) in class_urls(&chunk[range.clone()]) {
                    if is_cross_origin(&url, true) {
                        issues.push(
                            Issue::error(span.clone(), "cross-origin-resource", format!("a class loads `{url}` from another origin, which the policy blocks"))
                                .suggest("download it into static/ and reference it from there"),
                        );
                    } else if !url.trim_start().to_ascii_lowercase().starts_with("data:") {
                        slots.push(UrlSlot {
                            span: range.start + at.start..range.start + at.end,
                            role: UrlRole::ClassImage,
                            value: url,
                            element: name.clone(),
                            attribute: attribute.clone(),
                        });
                    }
                }
                continue;
            }
            let Some(role) = url_role(&name, &attribute) else {
                continue;
            };
            let urls: Vec<String> = match role {
                UrlRole::Srcset => parse_srcset(&decoded)
                    .into_iter()
                    .map(|(url, _)| url)
                    .collect(),
                _ => vec![decoded.clone()],
            };
            let image = matches!(role, UrlRole::Srcset | UrlRole::Resource { image: true });
            if !matches!(role, UrlRole::Link) && urls.iter().any(|url| is_cross_origin(url, image))
            {
                issues.push(
                    Issue::error(span.clone(), "cross-origin-resource", format!("<{name} {attribute}> loads from another origin, which the policy blocks"))
                        .suggest("download it into static/ and reference it from there"),
                );
                continue;
            }
            if let Some(range) = value_range(chunk, value.span.start..value.span.end) {
                slots.push(UrlSlot {
                    span: range,
                    role,
                    value: decoded.clone(),
                    element: name.clone(),
                    attribute: attribute.clone(),
                });
            }
        }
    }
    (issues, slots)
}

/// Checks a page the build generated, the last gate before it is published:
/// the theme's scripts and stylesheet load from files on the site, and
/// nothing in the page, author content included, runs inline script, carries
/// a style, embeds a frame or loads from another origin. Returns what is
/// wrong; empty when the page may ship.
pub fn check_document(html: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let mut emitter = html5gum::DefaultEmitter::default();
    emitter.naively_switch_states(true);
    let mut in_script = false;
    for token in Tokenizer::new_with_emitter(html, emitter) {
        let Ok(token) = token;
        match token {
            Token::StartTag(tag) => {
                let name = String::from_utf8_lossy(&tag.name).to_ascii_lowercase();
                let attribute = |key: &str| {
                    tag.attributes
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case(key.as_bytes()))
                        .map(|(_, value)| String::from_utf8_lossy(&value.value).into_owned())
                };
                match name.as_str() {
                    "script" => {
                        in_script = !tag.self_closing;
                        match attribute("src") {
                            None => problems.push("an inline <script>".to_owned()),
                            Some(src) if is_cross_origin(&src, false) => {
                                problems.push(format!("a script from another origin: {src}"))
                            }
                            Some(_) => {}
                        }
                    }
                    "style" | "iframe" | "frame" | "object" | "embed" | "base" => {
                        problems.push(format!("a <{name}> element"))
                    }
                    "a" | "area" if opens_unguarded_window(attribute) => problems.push(format!(
                        "a new-window <{name}> to another site without rel=\"noopener\""
                    )),
                    "link" => {
                        let rel = attribute("rel").unwrap_or_default().to_ascii_lowercase();
                        let loads = rel.split_ascii_whitespace().any(|rel| {
                            matches!(
                                rel,
                                "stylesheet"
                                    | "preload"
                                    | "modulepreload"
                                    | "prefetch"
                                    | "icon"
                                    | "manifest"
                            )
                        });
                        if let Some(href) =
                            attribute("href").filter(|href| loads && is_cross_origin(href, false))
                        {
                            problems
                                .push(format!("a <link rel=\"{rel}\"> to another origin: {href}"));
                        }
                    }
                    _ => {}
                }
                for (key, value) in &tag.attributes {
                    let key = String::from_utf8_lossy(key).to_ascii_lowercase();
                    let value = String::from_utf8_lossy(&value.value);
                    if key.starts_with("on") || key == "style" {
                        problems.push(format!("a `{key}` attribute on <{name}>"));
                    } else if is_script_url(&value) {
                        problems.push(format!("a script URL in <{name} {key}>"));
                    } else if let Some(role) = url_role(&name, &key) {
                        let image =
                            matches!(role, UrlRole::Srcset | UrlRole::Resource { image: true });
                        let urls: Vec<String> = match role {
                            UrlRole::Link | UrlRole::ClassImage => Vec::new(),
                            UrlRole::Srcset => parse_srcset(&value)
                                .into_iter()
                                .map(|(url, _)| url)
                                .collect(),
                            UrlRole::Resource { .. } => vec![value.clone().into_owned()],
                        };
                        if let Some(url) = urls.into_iter().find(|url| is_cross_origin(url, image))
                        {
                            problems
                                .push(format!("<{name} {key}> loads from another origin: {url}"));
                        }
                    }
                }
            }
            Token::EndTag(tag) if tag.name.eq_ignore_ascii_case(b"script") => in_script = false,
            Token::String(text) if in_script && !text.value.iter().all(u8::is_ascii_whitespace) => {
                problems.push("script text inside a <script> element".to_owned())
            }
            _ => {}
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The errors, in order; warnings have a test of their own.
    fn rules(html: &str) -> Vec<&'static str> {
        check(html)
            .0
            .iter()
            .filter(|issue| issue.severity == crate::diagnostic::Severity::Error)
            .map(|issue| issue.rule)
            .collect()
    }

    #[test]
    fn an_image_without_alt_is_a_warning() {
        let (issues, _) =
            check("<img src=\"a.png\"><img src=\"b.png\" alt=\"\"><IMG SRC=c.png ALT=c>");
        let found: Vec<(&str, crate::diagnostic::Severity)> =
            issues.iter().map(|i| (i.rule, i.severity)).collect();
        assert_eq!(
            found,
            [("missing-alt", crate::diagnostic::Severity::Warning)]
        );
    }

    #[test]
    fn a_new_window_to_another_site_needs_noopener() {
        assert_eq!(
            rules("<a href=\"https://x.example\" target=\"_blank\">x</a>"),
            ["new-window-opener"]
        );
        assert_eq!(
            rules("<A HREF=//x.example TARGET=Help REL=external>x</A>"),
            ["new-window-opener"]
        );
        for fine in [
            "<a href=\"https://x.example\" target=\"_blank\" rel=\"noopener\">x</a>",
            "<a href=\"https://x.example\" target=\"_blank\" rel=\"external noreferrer\">x</a>",
            "<a href=\"https://x.example\" target=\"_self\">x</a>",
            "<a href=\"https://x.example\">x</a>",
            "<a href=\"guide.md\" target=\"_blank\">x</a>",
        ] {
            assert!(rules(fine).is_empty(), "{fine}");
        }
        assert_eq!(
            check_document("<a href=\"https://x.example\" target=_blank>x</a>"),
            ["a new-window <a> to another site without rel=\"noopener\""]
        );
    }

    #[test]
    fn script_in_every_disguise() {
        assert_eq!(rules("<script>alert(1)</script>"), ["inline-script"]);
        assert_eq!(rules("<SCRIPT src=x.js></SCRIPT>"), ["inline-script"]);
        assert_eq!(
            rules("<svg><script>alert(1)</script></svg>"),
            ["inline-script"]
        );
        assert_eq!(rules("<img src=x.png onerror=alert(1)>"), ["event-handler"]);
        assert_eq!(rules("<div OnClick=\"x()\">"), ["event-handler"]);
        assert_eq!(
            rules("<a href=\"javascript:alert(1)\">x</a>"),
            ["script-url"]
        );
        assert_eq!(
            rules("<a href=\"JaVaScRiPt:alert(1)\">x</a>"),
            ["script-url"]
        );
        assert_eq!(
            rules("<a href=\"java&#x09;script:alert(1)\">x</a>"),
            ["script-url"]
        );
        assert_eq!(
            rules("<a href=\"&#106;avascript:alert(1)\">x</a>"),
            ["script-url"]
        );
        assert_eq!(rules("<a href=\" \n javascript:x\">x</a>"), ["script-url"]);
        assert_eq!(rules("<p style=\"color:red\">"), ["inline-style"]);
        assert_eq!(rules("<style>p{}</style>"), ["inline-style"]);
        assert_eq!(
            rules("<iframe src=\"/x\"></iframe><object></object><embed>"),
            ["embedded-frame", "embedded-plugin", "embedded-plugin"]
        );
        assert_eq!(rules("<base href=\"/\">"), ["base-element"]);
    }

    #[test]
    fn malformed_markup_is_read_as_a_browser_would() {
        // An unclosed attribute quote swallows the rest of the tag; nothing
        // after it is an attribute, and nothing is invented.
        assert!(rules("<img src=\"a.png alt=x>").is_empty());
        assert_eq!(rules("<<script>x</script>"), ["inline-script"]);
        assert_eq!(rules("<p/onclick=x>"), ["event-handler"]);
        assert!(rules("<p>plain <b>text</b></p><!-- <script> in a comment -->").is_empty());
    }

    #[test]
    fn resources_must_stay_on_the_site() {
        assert_eq!(
            rules("<img src=\"https://cdn.example/x.png\">"),
            ["cross-origin-resource"]
        );
        assert_eq!(
            rules("<img src=\"//cdn.example/x.png\">"),
            ["cross-origin-resource"]
        );
        assert_eq!(
            rules("<img srcset=\"a.png 1x, https://x.example/b.png 2x\">"),
            ["cross-origin-resource"]
        );
        assert_eq!(
            rules("<video poster=\"http://x.example/p.png\"></video>"),
            ["cross-origin-resource"]
        );
        assert_eq!(
            rules("<svg><use href=\"https://x.example/s.svg#i\"/></svg>"),
            ["cross-origin-resource"]
        );
        assert_eq!(
            rules("<audio src=\"data:audio/wav;base64,AA\"></audio>"),
            ["cross-origin-resource"]
        );
        assert!(rules(
            "<img src=\"data:image/png;base64,AA\"><a href=\"https://example.com\">ok</a>"
        )
        .is_empty());
        assert!(
            rules("<img src=\"diagram.png\" alt=\"d\"><source srcset=\"/static/a.webp\">")
                .is_empty()
        );
    }

    #[test]
    fn slots_cover_the_raw_values() {
        let html = "<a href=\"guide.md#x\">g</a> <img alt='a' src=pic.png>";
        let (issues, slots) = check(html);
        assert!(issues.is_empty());
        let values: Vec<(&str, &str)> = slots
            .iter()
            .map(|slot| (&html[slot.span.clone()], slot.value.as_str()))
            .collect();
        assert_eq!(
            values,
            [("guide.md#x", "guide.md#x"), ("pic.png", "pic.png")]
        );
        assert_eq!(slots[0].role, UrlRole::Link);
        assert_eq!(slots[1].role, UrlRole::Resource { image: true });
        let (_, slots) = check("<a href=\"a&amp;b.md\">x</a>");
        assert_eq!(slots[0].value, "a&b.md");
    }

    #[test]
    fn urls_in_class_values_are_resources_too() {
        let html = "<div class=\"h-10 bg-[url(/bg.svg)] mask-[url('icons/m.svg#i')]\"></div><p class='bg-[url(\"data:image/svg+xml,%3Csvg/%3E\")]'></p>";
        let (issues, slots) = check(html);
        assert!(issues.is_empty(), "{issues:?}");
        let found: Vec<(&str, &str, UrlRole)> = slots
            .iter()
            .map(|slot| (&html[slot.span.clone()], slot.value.as_str(), slot.role))
            .collect();
        assert_eq!(
            found,
            [
                ("/bg.svg", "/bg.svg", UrlRole::ClassImage),
                ("icons/m.svg#i", "icons/m.svg#i", UrlRole::ClassImage),
            ]
        );
        assert_eq!(
            rules("<div class=\"bg-[url(https://cdn.example/x.png)]\"></div>"),
            ["cross-origin-resource"]
        );
        assert_eq!(
            rules("<div class=\"bg-[url(//cdn.example/x.png)]\"></div>"),
            ["cross-origin-resource"]
        );
    }

    #[test]
    fn a_generated_document_loads_its_own_files_and_runs_nothing_inline() {
        let ok = "<!doctype html><html><head><meta charset=\"utf-8\"><link rel=\"canonical\" href=\"https://example.com/a/\"><link rel=\"stylesheet\" href=\"/assets/site.css\"><script src=\"/assets/mode.js\"></script></head><body><a href=\"https://example.com\">x</a><img src=\"data:image/png;base64,AA\" alt=\"\"></body></html>";
        assert!(check_document(ok).is_empty(), "{:?}", check_document(ok));
        let bad = "<script>alert(1)</script><script src=\"https://cdn.example/x.js\"></script><link rel=\"stylesheet\" href=\"//cdn.example/x.css\"><p style=\"color:red\" onclick=\"x()\">t</p><a href=\"JaVaScRiPt:x\">y</a><img src=\"https://img.example/a.png\"><iframe src=\"/a\"></iframe>";
        assert_eq!(check_document(bad).len(), 9, "{:#?}", check_document(bad));
    }

    #[test]
    fn srcset_candidates_split_as_a_browser_splits_them() {
        let pairs = |value: &str| parse_srcset(value);
        assert_eq!(
            pairs("a.png 1x, b.png 2x"),
            [
                ("a.png".to_owned(), "1x".to_owned()),
                ("b.png".to_owned(), "2x".to_owned())
            ]
        );
        assert_eq!(
            pairs("data:image/png;base64,AA 1x,b.png"),
            [
                ("data:image/png;base64,AA".to_owned(), "1x".to_owned()),
                ("b.png".to_owned(), String::new())
            ]
        );
        assert_eq!(
            pairs(" a.png, b.png 640w "),
            [
                ("a.png".to_owned(), String::new()),
                ("b.png".to_owned(), "640w".to_owned())
            ]
        );
    }
}
