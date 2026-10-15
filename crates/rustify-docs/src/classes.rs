//! The classes a rendered page carries: the stylesheet's candidates.
//!
//! Only `class` attributes are read, never text, so code and prose that
//! happen to look like utilities compile to nothing. Highlighting classes
//! (`hl-…`) are styled by their own palette and are left out as well.

use std::collections::BTreeSet;

use crate::highlight::CLASS_PREFIX;

/// Classes the framework's scripts switch at run time; no rendered page has
/// them yet, so they are added to every build's candidates.
pub const SCRIPT_STATE_CLASSES: &[&str] = &[
    "rd-search-result",
    "rd-search-title",
    "rd-search-badge",
    "rd-search-source",
    "rd-search-path",
    "rd-search-snippet",
];

/// Adds every class in `html`'s `class` attributes to `into`.
///
/// `html` is markup this crate wrote or validated, in which a `<` in text is
/// always escaped; a raw-text element's contents (`script`, `style`) are
/// skipped, and comments are passed over whole.
pub fn collect(html: &str, into: &mut BTreeSet<String>) {
    let bytes = html.as_bytes();
    let mut i = 0;
    while let Some(offset) = html[i..].find('<') {
        i += offset + 1;
        if html[i..].starts_with("!--") {
            i = html[i..].find("-->").map_or(html.len(), |end| i + end + 3);
            continue;
        }
        let name_end = html[i..]
            .find(|c: char| c.is_ascii_whitespace() || c == '>' || c == '/')
            .map_or(html.len(), |end| i + end);
        let name = html[i..name_end].to_ascii_lowercase();
        i = name_end;
        // Attributes, up to the end of the tag.
        loop {
            while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b'/') {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] == b'>' {
                i += 1;
                break;
            }
            let attr_end = html[i..]
                .find(|c: char| c.is_ascii_whitespace() || c == '=' || c == '>' || c == '/')
                .map_or(html.len(), |end| i + end);
            let attr = &html[i..attr_end];
            i = attr_end;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            let mut value = "";
            if i < bytes.len() && bytes[i] == b'=' {
                i += 1;
                while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                if i < bytes.len() && (bytes[i] == b'"' || bytes[i] == b'\'') {
                    let quote = bytes[i] as char;
                    let end = html[i + 1..]
                        .find(quote)
                        .map_or(html.len(), |end| i + 1 + end);
                    value = &html[i + 1..end];
                    i = (end + 1).min(html.len());
                } else {
                    let end = html[i..]
                        .find(|c: char| c.is_ascii_whitespace() || c == '>')
                        .map_or(html.len(), |end| i + end);
                    value = &html[i..end];
                    i = end;
                }
            }
            if attr.eq_ignore_ascii_case("class") {
                for class in unescape(value).split_ascii_whitespace() {
                    if !class.starts_with(CLASS_PREFIX) {
                        into.insert(class.to_owned());
                    }
                }
            }
        }
        if matches!(name.as_str(), "script" | "style" | "template") {
            let close = format!("</{name}");
            i = find_ignoring_case(&html[i..], &close).map_or(html.len(), |end| i + end);
        }
    }
}

/// Where `needle` (ASCII) first occurs in `haystack`, ignoring ASCII case.
fn find_ignoring_case(haystack: &str, needle: &str) -> Option<usize> {
    let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());
    haystack
        .windows(needle.len())
        .position(|window| window.eq_ignore_ascii_case(needle))
}

/// Class names as the browser reads the attribute, including numeric and
/// named character references in author HTML.
fn unescape(class: &str) -> String {
    if !class.contains('&') {
        return class.to_owned();
    }
    let html = format!("<i class=\"{}\">", class.replace('"', "&quot;"));
    html5gum::Tokenizer::new(html.as_str())
        .find_map(|token| match token {
            Ok(html5gum::Token::StartTag(tag)) => tag
                .attributes
                .get(b"class".as_slice())
                .map(|value| String::from_utf8_lossy(value).into_owned()),
            _ => None,
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(html: &str) -> Vec<String> {
        let mut set = BTreeSet::new();
        collect(html, &mut set);
        set.into_iter().collect()
    }

    #[test]
    fn attributes_not_text() {
        assert_eq!(
            classes("<div class=\"flex  gap-2\" data-x='class=\"bogus\"'><p>class=\"text-red-500\" flex-col</p><span class='hl-keyword hl-rust mt-1'>x</span></div>"),
            ["flex", "gap-2", "mt-1"]
        );
    }

    #[test]
    fn comments_raw_text_and_templates_are_skipped() {
        assert_eq!(
            classes("<!-- <b class=\"no\"> --><template class=\"rd-code-raw\">&lt;i class=\"no2\"&gt; <b class=\"no3\"></template><i class=\"yes\"></i>"),
            ["rd-code-raw", "yes"]
        );
    }

    #[test]
    fn unquoted_self_closing_and_escaped_values() {
        assert_eq!(
            classes("<img class=w-full src=x><br/><input class=\"a\"/><b class=\"content-[&quot;x&quot;]\">"),
            ["a", "content-[\"x\"]", "w-full"]
        );
    }

    #[test]
    fn numeric_references_decode_before_class_names_are_split() {
        assert_eq!(
            classes("<div class=\"before:content-['&#117;rl(foo)'] h&#45;10&#32;w-10\"></div>"),
            ["before:content-['url(foo)']", "h-10", "w-10"]
        );
    }
}
