//! A page's YAML frontmatter: split off with its offsets, then read with
//! every key known.

use crate::diagnostic_text::code_span;
use std::ops::Range;

use serde::Deserialize;

use crate::diagnostic::Issue;

/// Where the frontmatter is in a file, if there is one: the YAML between
/// the fences, and where the Markdown after the closing fence starts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Split {
    pub yaml: Range<usize>,
    pub body: usize,
}

/// Finds `---` frontmatter at the very start of `text`. A file that opens a
/// fence and never closes it is an error at the opening fence, rather than a
/// page whose whole text is metadata.
pub fn split(text: &str) -> Result<Option<Split>, Issue> {
    let first = match text.find('\n') {
        Some(end) => &text[..end],
        None => text,
    };
    if first.trim_end_matches('\r') != "---" {
        return Ok(None);
    }
    let unclosed = || {
        Issue::error(
            0..first.len(),
            "frontmatter-unclosed",
            "the frontmatter opened here is never closed",
        )
        .suggest("end it with a line holding only `---`")
    };
    // A fence with no line after it has nothing to close it.
    if first.len() == text.len() {
        return Err(unclosed());
    }
    let start = first.len() + 1;
    let mut offset = start;
    for line in text[start..].split_inclusive('\n') {
        let bare = line.trim_end_matches(['\n', '\r']);
        if bare == "---" || bare == "..." {
            return Ok(Some(Split {
                yaml: start..offset,
                body: offset + line.len(),
            }));
        }
        offset += line.len();
    }
    Err(unclosed())
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Sidebar {
    pub label: Option<String>,
    pub order: Option<i64>,
    pub group: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Toc {
    /// The heading levels listed, as `[2, 3]`.
    pub levels: Option<[u8; 2]>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub text: String,
    pub link: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Feature {
    pub title: String,
    pub details: String,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    #[default]
    Doc,
    Home,
    /// A paid-traffic page: the home page's hero without the site's
    /// navigation, under the policy of a named `[landing.profiles]` entry.
    Landing,
}

impl Layout {
    /// Whether the page opens with a hero: title, tagline, actions and
    /// features, its body after them, outside the sidebar.
    pub fn has_hero(self) -> bool {
        matches!(self, Self::Home | Self::Landing)
    }
}

/// What a page says about itself.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Front {
    pub title: Option<String>,
    pub description: Option<String>,
    pub lang: Option<String>,
    #[serde(default)]
    pub sidebar: Sidebar,
    #[serde(default)]
    pub draft: bool,
    /// Whether search engines may index the page. Independent of `search`
    /// and `ai`, which control the two generated discovery indexes.
    pub index: Option<bool>,
    pub search: Option<bool>,
    pub ai: Option<bool>,
    #[serde(default)]
    pub toc: Toc,
    #[serde(default)]
    pub layout: Layout,
    pub tagline: Option<String>,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub features: Vec<Feature>,
    /// The `[landing.profiles]` entry a landing page loads under.
    pub landing: Option<String>,
}

impl Front {
    /// Whether search engines may index the page; landing pages default to
    /// no, other pages to yes.
    pub fn indexed(&self) -> bool {
        self.index.unwrap_or(self.layout != Layout::Landing)
    }

    /// Whether the page is in the site's search index.
    pub fn searchable(&self) -> bool {
        self.search.unwrap_or(self.layout != Layout::Landing)
    }

    /// Whether the page is in `llms.txt` and the other AI indexes.
    pub fn discoverable(&self) -> bool {
        self.ai.unwrap_or(self.layout != Layout::Landing)
    }
}

/// Reads the frontmatter of `text` (a whole file). Errors are located in the
/// file; a file without frontmatter has the default.
pub fn parse(text: &str) -> Result<(Front, usize), Issue> {
    let Some(split) = split(text)? else {
        return Ok((Front::default(), 0));
    };
    let yaml = &text[split.yaml.clone()];
    if yaml.trim().is_empty() {
        return Ok((Front::default(), split.body));
    }
    match serde_yaml_ng::from_str::<Front>(yaml) {
        Ok(front) => Ok((front, split.body)),
        Err(error) => {
            let at = error.location().map_or(split.yaml.start, |location| {
                split.yaml.start + location.index()
            });
            let message = error.to_string();
            // serde_yaml_ng appends " at line N column M"; the diagnostic
            // carries the position itself.
            let message = message
                .split(" at line ")
                .next()
                .unwrap_or(&message)
                .to_owned();
            Err(Issue::error(at..at, "frontmatter", message))
        }
    }
}

/// Ask YAML's deserializer to locate the language scalar itself, retaining
/// the other successfully parsed fields and avoiding a search for its value.
pub(crate) fn language_issue(text: &str) -> Option<Issue> {
    fn language<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
        struct Tag;
        impl serde::de::Visitor<'_> for Tag {
            type Value = String;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a language tag")
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<String, E> {
                if crate::config::valid_lang(value) {
                    Ok(value.to_owned())
                } else {
                    Err(E::custom(format!(
                        "{} is not a language tag",
                        code_span(value)
                    )))
                }
            }
        }
        deserializer.deserialize_string(Tag)
    }
    #[derive(Deserialize)]
    struct Language {
        #[serde(rename = "lang", deserialize_with = "language")]
        _lang: String,
    }
    let split = split(text).ok()??;
    let yaml = &text[split.yaml.clone()];
    let error = serde_yaml_ng::from_str::<Language>(yaml).err()?;
    let location = error.location()?;
    let line_start: usize = yaml
        .split_inclusive('\n')
        .take(location.line() - 1)
        .map(str::len)
        .sum();
    let column = yaml[line_start..]
        .char_indices()
        .nth(location.column() - 1)
        .map_or(0, |(at, _)| at);
    let at = split.yaml.start + line_start + column;
    let message = error.to_string();
    // Only remove YAML's actual location suffix; the author value may
    // itself contain " at line ".
    let suffix = format!(" at line {} column {}", location.line(), location.column());
    let message = message.strip_suffix(&suffix).unwrap_or(&message);
    Some(
        Issue::error(at..at, "lang", message)
            .suggest("use a language tag such as `en`, `zh-CN` or `fr`"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_opening_fence_with_nothing_after_it_is_unclosed() {
        for text in ["---", "---\r", "---\n", "---\ntitle: A\n"] {
            let issue = split(text).expect_err(text);
            assert_eq!(issue.rule, "frontmatter-unclosed", "{text:?}");
            assert_eq!(issue.span.start, 0, "{text:?}");
        }
    }

    #[test]
    fn crlf_frontmatter_closes() {
        let text = "---\r\ntitle: A\r\n---\r\n# A\r\n";
        let split = split(text).unwrap().unwrap();
        assert_eq!(&text[split.yaml.clone()], "title: A\r\n");
        assert_eq!(&text[split.body..], "# A\r\n");
        assert_eq!(parse(text).unwrap().0.title.as_deref(), Some("A"));
        assert_eq!(split.body, parse(text).unwrap().1);
    }

    #[test]
    fn landing_layout_and_profile() {
        let (front, _) = parse("---\nlayout: landing\nlanding: ads\n---\n").unwrap();
        assert_eq!(front.layout, Layout::Landing);
        assert_eq!(front.landing.as_deref(), Some("ads"));
        assert!(front.layout.has_hero() && Layout::Home.has_hero() && !Layout::Doc.has_hero());
        assert!(!front.indexed() && !front.searchable() && !front.discoverable());
        let (front, _) =
            parse("---\nlayout: landing\nindex: true\nsearch: true\nai: true\n---\n").unwrap();
        assert!(front.indexed() && front.searchable() && front.discoverable());
        for layout in ["doc", "home"] {
            let (front, _) = parse(&format!("---\nlayout: {layout}\n---\n")).unwrap();
            assert!(front.indexed() && front.searchable() && front.discoverable());
            let (front, _) = parse(&format!(
                "---\nlayout: {layout}\nindex: false\nsearch: false\nai: false\n---\n"
            ))
            .unwrap();
            assert!(!front.indexed() && !front.searchable() && !front.discoverable());
        }
        assert_eq!(
            parse("---\nlayout: lander\n---\n").unwrap_err().rule,
            "frontmatter"
        );
        assert_eq!(
            parse("---\nlanding: [ads]\n---\n").unwrap_err().rule,
            "frontmatter"
        );
    }

    #[test]
    fn no_fence_no_frontmatter() {
        assert_eq!(split("# A\n").unwrap(), None);
        assert_eq!(split("").unwrap(), None);
        assert_eq!(split("--").unwrap(), None);
    }
}
