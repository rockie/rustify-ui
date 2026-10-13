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

use dom_query::Document;
use html5ever::{parse_document, tendril::TendrilSink};
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

/// IDs in the document tree, in output order. Template contents and
/// Markdown buffers which were never emitted contribute nothing here.
pub(crate) fn ids(html: &str) -> Vec<String> {
    semantics(html, &[]).anchors
}

/// An emitted heading tag or raw HTML chunk, mapped back to its source.
pub(crate) struct HeadingSource {
    pub html: Range<usize>,
    pub span: Range<usize>,
    pub raw: bool,
    pub explicit: bool,
}

pub(crate) struct Semantics {
    pub anchors: Vec<String>,
    pub headings: Vec<crate::markdown::Heading>,
    pub leading_h1: Option<String>,
    /// Source start tags which actually became nodes in the document tree.
    pub tags: Vec<Range<usize>>,
    /// Document and template-fragment nodes, both checked against the policy.
    pub stored_tags: Vec<Range<usize>>,
}

/// The existing leading-H1 merge boundary, at an actual closing tag rather
/// than matching text inside an attribute.
pub(crate) fn leading_h1_end(html: &str) -> Option<usize> {
    if !html.starts_with("<h1") {
        return None;
    }
    let mut emitter = html5gum::DefaultEmitter::new_with_span();
    emitter.naively_switch_states(true);
    let mut templates = 0usize;
    for token in Tokenizer::new_with_emitter(html, emitter) {
        match token {
            Ok(Token::StartTag(tag)) if tag.name.as_slice() == b"template" && !tag.self_closing => {
                templates += 1
            }
            Ok(Token::EndTag(tag)) if tag.name.as_slice() == b"template" => {
                templates = templates.saturating_sub(1)
            }
            Ok(Token::EndTag(tag))
                if templates == 0
                    && tag.name.as_slice() == b"h1"
                    && html.get(tag.span.start..tag.span.end) == Some("</h1>") =>
            {
                return Some(tag.span.end)
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn semantics(html: &str, sources: &[HeadingSource]) -> Semantics {
    // Mark a private analysis copy, never the published HTML. The tree
    // builder then decides which source tags became document nodes,
    // including template isolation and foreign-content integration points.
    let mut annotated = String::with_capacity(html.len());
    let mut marker = "data-rd-heading-origin".to_owned();
    while html.contains(&marker) {
        marker.push('_');
    }
    let mut origins = Vec::new();
    let mut tags = Vec::new();
    let mut at = 0;
    // Tokenize candidate tags even in foreign <title>/<desc>. Only the
    // tree builder can decide whether their children are HTML headings.
    let emitter = html5gum::DefaultEmitter::new_with_span();
    for token in Tokenizer::new_with_emitter(html, emitter) {
        let Ok(Token::StartTag(tag)) = token else {
            continue;
        };
        let start = tag.span.start;
        tags.push(start..tag.span.end);
        let origin = sources.partition_point(|source| source.html.end <= start);
        let source = sources
            .get(origin)
            .filter(|source| source.html.contains(&start));
        origins.push(source.map(|source| {
            let span = if source.raw {
                let offset = source.span.start + start - source.html.start;
                offset..(offset + tag.span.end - start).min(source.span.end)
            } else {
                source.span.clone()
            };
            (span, source.explicit, start)
        }));
        let insert = start + 1 + tag.name.len();
        annotated.push_str(&html[at..insert]);
        annotated.push_str(&format!(" {marker}=\"{}\"", origins.len() - 1));
        at = insert;
    }
    annotated.push_str(&html[at..]);
    // SSR writes the article into an existing body. A fragment parser
    // ignores body tags instead of merging their attributes into that body.
    let dom = parse_document(Document::default(), Default::default()).one(format!(
        "<!doctype html><html><head></head><body><article>{annotated}</article></body></html>"
    ));
    // HTML template contents are separate fragments, not descendants.
    // Foreign elements named template retain their ordinary children.
    let mut result = Semantics {
        anchors: Vec::new(),
        headings: Vec::new(),
        leading_h1: None,
        tags: Vec::new(),
        stored_tags: Vec::new(),
    };
    let mut templates = Vec::new();
    for node in dom.root().descendants_it() {
        if let Some(fragment) = node
            .query(|node| node.as_element().and_then(|el| el.template_contents))
            .flatten()
        {
            templates.push(fragment);
        }
        let origin = node
            .attr(&marker)
            .and_then(|value| value.parse::<usize>().ok());
        if let Some(tag) = origin.and_then(|i| tags.get(i)) {
            result.tags.push(tag.clone());
        }
        if let Some(id) = node.id_attr() {
            result.anchors.push(id.to_string());
        }
        let Some(name) = node.qual_name_ref() else {
            continue;
        };
        if name.ns.as_ref() != "http://www.w3.org/1999/xhtml" {
            continue;
        }
        let level = match name.local.as_ref() {
            "h1" => 1,
            "h2" => 2,
            "h3" => 3,
            "h4" => 4,
            "h5" => 5,
            "h6" => 6,
            _ => continue,
        };
        let Some((span, explicit, start)) =
            origin.and_then(|i| origins.get(i)).and_then(Option::as_ref)
        else {
            continue;
        };
        let id = node.id_attr().map_or_else(String::new, |id| id.to_string());
        if level == 1 && *start == 0 && leading_h1_end(html).is_some() {
            result.leading_h1 = Some(id.clone());
        }
        result.headings.push(crate::markdown::Heading {
            level,
            id,
            text: heading_text(&node.text(), &marker)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
            explicit: *explicit && node.id_attr().is_some(),
            span: span.clone(),
        });
    }
    result.tags.sort_unstable_by_key(|tag| tag.start);
    result.tags.dedup();
    while let Some(fragment) = templates.pop() {
        let Some(root) = dom.tree.get(&fragment) else {
            continue;
        };
        for node in root.descendants_it() {
            if let Some(fragment) = node
                .query(|node| node.as_element().and_then(|el| el.template_contents))
                .flatten()
            {
                templates.push(fragment);
            }
            if let Some(tag) = node
                .attr(&marker)
                .and_then(|value| value.parse::<usize>().ok())
                .and_then(|i| tags.get(i))
            {
                result.stored_tags.push(tag.clone());
            }
        }
    }
    result.stored_tags.extend(result.tags.iter().cloned());
    result.stored_tags.sort_unstable_by_key(|tag| tag.start);
    result.stored_tags.dedup();
    result
}

pub(crate) fn in_document(span: &Range<usize>, tags: &[Range<usize>]) -> bool {
    let at = tags.partition_point(|tag| tag.end <= span.start);
    tags.get(at)
        .is_some_and(|tag| tag.start <= span.start && span.end <= tag.end)
}

fn heading_text(text: &str, marker: &str) -> String {
    // An annotation inside raw text (e.g. textarea) stays text in the DOM.
    // The chosen marker was absent from the input, so remove only ours.
    let prefix = format!(" {marker}=\"");
    let mut rest = text;
    let mut clean = String::with_capacity(text.len());
    while let Some((before, after)) = rest.split_once(&prefix) {
        clean.push_str(before);
        let Some((_, tail)) = after.split_once('"') else {
            break;
        };
        rest = tail;
    }
    clean.push_str(rest);
    clean
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

/// URL tokens in arbitrary utility values, excluding quoted content and
/// arbitrary selector variants. Ranges are in the decoded attribute.
pub(crate) fn class_urls(value: &str) -> Vec<(String, Range<usize>)> {
    let mut urls = Vec::new();
    let mut from = 0;
    for class in value.split_ascii_whitespace() {
        let at = from + value[from..].find(class).unwrap_or_default();
        from = at + class.len();
        if !class.contains('[') {
            continue;
        }
        if rustify_style::compiler::candidate::parse(class, crate::style::CSS_DEPTH_LIMIT)
            .map_or(true, |candidates| candidates.is_empty())
        {
            continue;
        }
        let Ok(parts) =
            rustify_style::compiler::value::segment(class, ':', crate::style::CSS_DEPTH_LIMIT)
        else {
            continue;
        };
        let Some(base) = parts.last() else { continue };
        let (Some(open), Some(close)) = (base.find('['), base.rfind(']')) else {
            continue;
        };
        if close <= open {
            continue;
        }
        let start = at + class.len() - base.len() + open + 1;
        for (url, range) in crate::style::url_ranges(&base[open + 1..close], true) {
            urls.push((url, start + range.start..start + range.end));
        }
    }
    urls
}

/// Decode an attribute while retaining the raw span for each output byte.
/// Character references use the same HTML tokenizer as the policy check.
/// This lets a URL rewrite replace `&#47;` without disturbing its class.
fn attribute_spans(raw: &str) -> Vec<Range<usize>> {
    let mut spans = Vec::new();
    let mut at = 0;
    while at < raw.len() {
        let rest = &raw[at..];
        if rest.starts_with('&') {
            let mut end = 1;
            let numeric = rest.as_bytes().get(end) == Some(&b'#');
            if numeric {
                end += 1;
                let hex = matches!(rest.as_bytes().get(end), Some(b'x' | b'X'));
                end += usize::from(hex);
                while rest.as_bytes().get(end).is_some_and(|c| {
                    if hex {
                        c.is_ascii_hexdigit()
                    } else {
                        c.is_ascii_digit()
                    }
                }) {
                    end += 1;
                }
            } else {
                while rest
                    .as_bytes()
                    .get(end)
                    .is_some_and(u8::is_ascii_alphanumeric)
                {
                    end += 1;
                }
            }
            if rest.as_bytes().get(end) == Some(&b';')
                || (!numeric && rest.as_bytes().get(end) == Some(&b'='))
            {
                end += 1;
            }
            let entity = &rest[..end];
            let input = format!("<i a=\"{entity}\">");
            let decoded = Tokenizer::new(input.as_str())
                .find_map(|token| match token {
                    Ok(Token::StartTag(tag)) => tag
                        .attributes
                        .get(b"a".as_slice())
                        .map(|value| value.to_vec()),
                    _ => None,
                })
                .unwrap_or_default();
            if decoded != entity.as_bytes() {
                spans.extend(std::iter::repeat_n(at..at + end, decoded.len()));
                at += end;
                continue;
            }
        }
        let c = rest.chars().next().unwrap_or_default();
        let len = if rest.starts_with("\r\n") {
            2
        } else {
            c.len_utf8()
        };
        let decoded_len = if c == '\r' {
            1
        } else if c == '\0' {
            '\u{fffd}'.len_utf8()
        } else {
            len
        };
        spans.extend(std::iter::repeat_n(at..at + len, decoded_len));
        at += len;
    }
    spans
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
    let (issues, mut slots) = scan(chunk);
    if chunk.to_ascii_lowercase().contains("<template") {
        let tags = semantics(chunk, &[]).tags;
        slots.retain(|slot| in_document(&slot.span, &tags));
    }
    (issues, slots)
}

/// Checks a complete rendered surface using browser tree contexts. Stored
/// template markup still obeys the policy, but supplies no document URLs.
pub fn check_rendered(html: &str) -> (Vec<Issue>, Vec<UrlSlot>) {
    let semantics = semantics(html, &[]);
    let (issues, mut slots) = scan_document(html, &semantics.stored_tags);
    slots.retain(|slot| in_document(&slot.span, &semantics.tags));
    (issues, slots)
}

pub(crate) fn scan(chunk: &str) -> (Vec<Issue>, Vec<UrlSlot>) {
    scan_tokens(chunk, true, None)
}

/// A complete rendered surface can use the tree builder's actual tokenizer
/// contexts, including foreign integration points and inert template storage.
pub(crate) fn scan_document(chunk: &str, tags: &[Range<usize>]) -> (Vec<Issue>, Vec<UrlSlot>) {
    scan_tokens(chunk, false, Some(tags))
}

fn scan_tokens(
    chunk: &str,
    naive: bool,
    tags: Option<&[Range<usize>]>,
) -> (Vec<Issue>, Vec<UrlSlot>) {
    let mut issues = Vec::new();
    let mut slots = Vec::new();
    let mut emitter = html5gum::DefaultEmitter::new_with_span();
    // Inside <script>, <style> and the like, text is text, as in a browser.
    emitter.naively_switch_states(naive);
    for token in Tokenizer::new_with_emitter(chunk, emitter) {
        let Ok(token) = token;
        let Token::StartTag(tag) = token else {
            continue;
        };
        let name = String::from_utf8_lossy(&tag.name).to_ascii_lowercase();
        let tag_span = tag.span.start..tag.span.end;
        if tags.is_some_and(|tags| !in_document(&tag_span, tags)) {
            continue;
        }
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
                let raw = &chunk[range.clone()];
                let spans = (raw != decoded).then(|| attribute_spans(raw));
                for (url, at) in class_urls(&decoded) {
                    let at = match &spans {
                        Some(spans) if !at.is_empty() => {
                            spans[at.start].start..spans[at.end - 1].end
                        }
                        _ => at,
                    };
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
                        if let Some(href) = attribute("href")
                            .filter(|href| loads && is_cross_origin(href, rel == "icon"))
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

    #[test]
    fn template_urls_do_not_become_document_references() {
        let html = "<template id='raw'><a href='missing/'>Hidden</a><img alt='' src='missing.svg'></template><a href='visible/'>Visible</a>";
        let (issues, slots) = check(html);
        assert!(issues.is_empty(), "{issues:?}");
        assert_eq!(
            slots
                .iter()
                .map(|slot| slot.value.as_str())
                .collect::<Vec<_>>(),
            ["visible/"]
        );
        // Inert storage still obeys the CSP policy if activated later.
        assert_eq!(
            rules("<template><script>x()</script></template>"),
            ["inline-script"]
        );
    }

    #[test]
    fn template_urls_follow_foreign_namespace_integration() {
        for (html, expected) in [
            ("<svg><template><image href='visible.svg'/></template></svg>", vec!["visible.svg"]),
            ("<svg><foreignObject><template><a href='hidden/'>Hidden</a></template><a href='visible/'>Visible</a></foreignObject></svg>", vec!["visible/"]),
            ("<math><mtext><template><img alt='' src='hidden.svg'></template></mtext></math><img alt='' src='visible.svg'>", vec!["visible.svg"]),
            ("<template/><img alt='' src='hidden.svg'></template><img alt='' src='visible.svg'>", vec!["visible.svg"]),
        ] {
            assert_eq!(check(html).1.iter().map(|slot| slot.value.as_str()).collect::<Vec<_>>(), expected, "{html}");
        }
    }

    #[test]
    fn body_ids_follow_document_attribute_merging_and_template_boundaries() {
        for (html, expected) in [
            (
                r#"<h1 id="heading">Heading</h1><body id="target">Text</body>"#,
                vec!["target", "heading"],
            ),
            (
                r#"<body id="target"><body id="ignored"><h2 id="visible">Visible</h2>"#,
                vec!["target", "visible"],
            ),
            (
                r#"<template id="holder"><body id="ghost"><span id="hidden"></span></template><body id="target"><h2 id="visible">Visible</h2>"#,
                vec!["target", "holder", "visible"],
            ),
        ] {
            assert_eq!(ids(html), expected, "{html}");
        }
    }

    #[test]
    fn foreign_templates_keep_their_document_ids() {
        for (html, expected) in [
            (
                r#"<svg><template/></svg><h2 id="visible">Visible</h2>"#,
                vec!["visible"],
            ),
            (
                r#"<svg><template id="holder"><circle id="circle"/></template></svg><h2 id="visible">Visible</h2>"#,
                vec!["holder", "circle", "visible"],
            ),
            (
                r#"<template id="holder"><svg><template/></svg></template><h2 id="visible">Visible</h2>"#,
                vec!["holder", "visible"],
            ),
            (
                r#"<math><template id="holder"><mi id="letter">x</mi></template></math><h2 id="visible">Visible</h2>"#,
                vec!["holder", "letter", "visible"],
            ),
        ] {
            assert_eq!(ids(html), expected, "{html}");
        }
    }

    #[test]
    fn html_templates_at_foreign_integration_points_have_separate_contents() {
        for (open, close) in [
            ("<svg><foreignObject>", "</foreignObject></svg>"),
            ("<svg><desc>", "</desc></svg>"),
            ("<svg><title>", "</title></svg>"),
            ("<math><mtext>", "</mtext></math>"),
            (
                "<math><annotation-xml encoding='TEXT/HTML'>",
                "</annotation-xml></math>",
            ),
        ] {
            let html = format!(
                r#"{open}<template id="holder"><svg><template id="hidden"><circle id="ghost"/></template></svg></template>{close}<h2 id="visible">Visible</h2>"#
            );
            assert_eq!(ids(&html), ["holder", "visible"], "{html}");
        }
        // HTML's self-closing flag cannot close a template element.
        assert_eq!(
            ids(
                r#"<template id="holder"/><span id="ghost"></span></template><h2 id="visible">Visible</h2>"#
            ),
            ["holder", "visible"]
        );
    }

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
    fn class_urls_decode_entities_and_keep_raw_rewrite_spans() {
        let html = r#"<div class="before:content-['url(text)'] [&amp;[title='url(selector)']]:block [background-image:URL(&#47;bg.svg),_url(&quot;other.svg&quot;)]"></div>"#;
        let (issues, slots) = check(html);
        assert!(issues.is_empty(), "{issues:?}");
        assert_eq!(
            slots
                .iter()
                .map(|slot| (slot.value.as_str(), &html[slot.span.clone()]))
                .collect::<Vec<_>>(),
            [("/bg.svg", "&#47;bg.svg"), ("other.svg", "other.svg")]
        );
        let html = "<div class='bg-[url(&#x2fbg.svg)]'></div>";
        let (_, slots) = check(html);
        assert_eq!(slots[0].value, "˻g.svg");
        assert_eq!(&html[slots[0].span.clone()], "&#x2fbg.svg");
    }

    #[test]
    fn numeric_entities_leave_the_following_equals_out_of_a_url_rewrite() {
        for value in [
            "url&#40=/bg.svg)",
            "url&#40;=/bg.svg)",
            "url&#x28=/bg.svg)",
            "url(&#34=/bg.svg&#34)",
            "url(&#34;=/bg.svg&#34;)",
            "url(&#x22=/bg.svg&#x22)",
        ] {
            let html = format!("<div class=\"[background-image:{value}]\"></div>");
            let (issues, slots) = check(&html);
            assert!(issues.is_empty(), "{issues:?}");
            assert_eq!(slots.len(), 1, "{html}: {slots:?}");
            assert_eq!(slots[0].value, "=/bg.svg", "{html}");
            assert_eq!(&html[slots[0].span.clone()], "=/bg.svg", "{html}");
            let mut rewritten = html;
            rewritten.replace_range(slots[0].span.clone(), "/assets/bg.svg");
            assert_eq!(check(&rewritten).1[0].value, "/assets/bg.svg");
        }
        // A named reference followed by '=' stays literal in an attribute.
        let html = "<div class=\"bg-[url(&amp=bg.svg)]\"></div>";
        assert_eq!(check(html).1[0].value, "&amp=bg.svg");
    }

    #[test]
    fn class_resources_use_the_compilers_arbitrary_value_decoding() {
        for (value, expected, raw) in [
            ("URL('/bg_one.svg')", "/bg one.svg", "/bg_one.svg"),
            ("url('/bg_one.svg')", "/bg_one.svg", "/bg_one.svg"),
            (r"URL('/bg\_one.svg')", "/bg_one.svg", r"/bg\_one.svg"),
            (r"url('/bg\_one.svg')", "/bg_one.svg", r"/bg\_one.svg"),
            ("URL('/bg%20one.svg')", "/bg%20one.svg", "/bg%20one.svg"),
            ("url('/bg%20one.svg')", "/bg%20one.svg", "/bg%20one.svg"),
        ] {
            let html = format!("<div class=\"[background-image:{value}]\"></div>");
            let (issues, slots) = check(&html);
            assert!(issues.is_empty(), "{issues:?}");
            assert_eq!(slots.len(), 1, "{html}: {slots:?}");
            assert_eq!(slots[0].value, expected, "{html}");
            assert_eq!(&html[slots[0].span.clone()], raw, "{html}");
        }
        let html =
            "<div class=\"[background-image:URL('/bg_one.svg'),_url('/bg_two.svg')]\"></div>";
        assert_eq!(
            check(html)
                .1
                .iter()
                .map(|slot| slot.value.as_str())
                .collect::<Vec<_>>(),
            ["/bg one.svg", "/bg_two.svg"]
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
    fn a_data_icon_stays_local_without_allowing_data_stylesheets_or_remote_icons() {
        assert!(
            check_document("<link rel=\"icon\" href=\"data:image/svg+xml,%3Csvg/%3E\">").is_empty()
        );
        assert!(
            !check_document("<link rel=\"stylesheet\" href=\"data:text/css,body{}\">").is_empty()
        );
        assert!(
            !check_document("<link rel=\"icon\" href=\"https://other.example/icon.svg\">")
                .is_empty()
        );
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
