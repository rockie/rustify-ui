//! Heading ids that match the anchors GitHub gives the same Markdown, so a
//! `guide.md#some-heading` link works on both.
//!
//! The text is NFKC-normalized and lowercased; letters and digits of every
//! script stay (Chinese included), as do `-` and `_`; each whitespace
//! character becomes `-`; all other punctuation and symbols go. Nothing is
//! collapsed or trimmed, which is GitHub's behaviour too. A heading that
//! leaves nothing gets `section`.

use std::collections::{HashMap, HashSet};

use unicode_normalization::UnicodeNormalization;

/// The id a heading's text gives before de-duplication.
pub fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    for c in text.nfkc().flat_map(char::to_lowercase) {
        if c.is_alphanumeric() || c == '-' || c == '_' {
            slug.push(c);
        } else if c.is_whitespace() {
            slug.push('-');
        }
    }
    if slug.is_empty() {
        slug.push_str("section");
    }
    slug
}

/// Hands out a page's heading ids in document order.
///
/// Explicit `{#id}` ids are reserved up front, so an automatic id never
/// takes one even when the explicit heading comes later; a repeated
/// automatic id gets `-1`, `-2`, … like GitHub's.
#[derive(Default)]
pub struct Slugger {
    taken: HashSet<String>,
    counts: HashMap<String, usize>,
}

impl Slugger {
    pub fn new<'a>(explicit: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            taken: explicit.into_iter().map(str::to_owned).collect(),
            counts: HashMap::new(),
        }
    }

    /// The next free id for a heading with this text.
    pub fn next(&mut self, text: &str) -> String {
        let base = slugify(text);
        loop {
            let count = self.counts.entry(base.clone()).or_insert(0);
            let candidate = if *count == 0 {
                base.clone()
            } else {
                format!("{base}-{count}")
            };
            *count += 1;
            if self.taken.insert(candidate.clone()) {
                return candidate;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_compatible_ids() {
        for (text, id) in [
            (
                "A DOM application without GPU regions",
                "a-dom-application-without-gpu-regions",
            ),
            (
                "Using Tailwind v4 in your app",
                "using-tailwind-v4-in-your-app",
            ),
            ("`mount()` & friends!", "mount--friends"),
            ("snake_case and kebab-case", "snake_case-and-kebab-case"),
            ("中文 标题：快速开始", "中文-标题快速开始"),
            ("Ｆｕｌｌ　Ｗｉｄｔｈ", "full-width"),
            ("Ünïcödé Straße", "ünïcödé-straße"),
            ("?!", "section"),
            ("", "section"),
        ] {
            assert_eq!(slugify(text), id, "{text:?}");
        }
    }

    #[test]
    fn duplicates_and_explicit_ids() {
        let mut slugger = Slugger::new(["setup", "intro-1"]);
        assert_eq!(slugger.next("Intro"), "intro");
        assert_eq!(slugger.next("Intro"), "intro-2");
        assert_eq!(slugger.next("Setup"), "setup-1");
        assert_eq!(slugger.next("Intro"), "intro-3");
    }
}
