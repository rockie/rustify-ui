//! In-memory extraction using the fixed Tailwind Oxide state machines.

mod discovery;
mod rules;
mod source;
pub mod sources;
mod tree;

pub use source::{scan_sources, ChangedContent, Scanner, SourceScanResult};
pub use sources::{GlobEntry, PublicSourceEntry, SourceEntry, Sources};
#[cfg(all(feature = "scanner-fs", not(target_arch = "wasm32")))]
pub use tree::NativeSourceTree;
pub use tree::{SourceTree, VirtualEntry, VirtualSourceTree};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScanError {
    InvalidPattern(String),
    InvalidUtf8,
    MissingSourceInformation(String),
}
impl std::fmt::Display for ScanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for ScanError {}

#[cfg(all(test, feature = "scanner-fs", not(target_arch = "wasm32")))]
mod upstream_native_tests;

#[cfg(test)]
#[path = "../../../tests/utility/scanner/source-contracts.rs"]
mod source_contracts;

/// Unique, lexically sorted candidate class names and CSS variables.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScanResult {
    /// Valid-looking candidates; CSS validity is checked by the compiler.
    pub candidates: Vec<String>,
    /// CSS variable names, including their leading `--`.
    pub variables: Vec<String>,
}

/// Extract candidates and variables from UTF-8 content using a file extension hint.
///
/// Extensions follow upstream spelling, without a leading dot. Like upstream
/// raw content, CSS text uses full extraction. Source programs are not evaluated
/// to predict dynamic classes.
pub fn scan(content: &str, extension: &str) -> ScanResult {
    use crate::extractor::{Extracted, Extractor};
    use std::collections::BTreeSet;

    let content = pre_process_input(content.as_bytes(), extension);
    let mut candidates = BTreeSet::new();
    let mut variables = BTreeSet::new();
    // Upstream Scanner splits preprocessed content by lines before extraction.
    for line in content.split(|byte| *byte == b'\n') {
        let mut extractor = Extractor::new(line);
        let extracted = extractor.extract();
        for item in extracted {
            let (bytes, values) = match item {
                Extracted::Candidate(bytes) => (bytes, &mut candidates),
                Extracted::CssVariable(bytes) => (bytes, &mut variables),
            };
            // The public boundary accepts UTF-8, and preprocessing preserves its bytes.
            if let Ok(value) = std::str::from_utf8(bytes) {
                values.insert(value.to_owned());
            }
        }
    }
    ScanResult {
        candidates: candidates.into_iter().collect(),
        variables: variables.into_iter().collect(),
    }
}

pub(crate) fn pre_process_input(content: &[u8], extension: &str) -> Vec<u8> {
    use crate::extractor::pre_processors::*;
    match extension {
        "clj" | "cljs" | "cljc" => Clojure.process(content),
        "heex" | "eex" | "ex" | "exs" => Elixir.process(content),
        "cshtml" | "razor" => Razor.process(content),
        "haml" => Haml.process(content),
        "json" => Json.process(content),
        "pug" => Pug.process(content),
        "rb" | "erb" => Ruby.process(content),
        "slim" | "slang" => Slim.process(content),
        "svelte" | "rs" => Svelte.process(content),
        "vue" => Vue.process(content),
        _ => content.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Deserialize)]
    struct Fixture {
        name: String,
        content: String,
        extension: String,
        candidates: Vec<String>,
        variables: Vec<String>,
    }

    #[test]
    fn fixed_upstream_corpus_matches() {
        let fixtures: Vec<Fixture> =
            serde_json::from_str(include_str!("../../../tests/utility/scanner/fixtures.json"))
                .unwrap();
        for fixture in fixtures {
            assert_eq!(
                scan(&fixture.content, &fixture.extension),
                ScanResult {
                    candidates: fixture.candidates,
                    variables: fixture.variables
                },
                "fixture {}",
                fixture.name
            );
        }
    }

    #[test]
    fn long_input_is_deduplicated() {
        let source = "w-[137px] hover:bg-[#123456] ".repeat(20_000);
        assert_eq!(
            scan(&source, "txt").candidates,
            ["hover:bg-[#123456]", "w-[137px]"]
        );
    }

    #[test]
    fn candidates_are_unique_and_sorted() {
        assert_eq!(
            scan("p-4 flex p-4 hover:bg-red-500", "html").candidates,
            ["flex", "hover:bg-red-500", "p-4"]
        );
    }

    #[test]
    fn css_raw_content_uses_the_full_extractor() {
        let fixtures: Vec<Fixture> =
            serde_json::from_str(include_str!("../../../tests/utility/scanner/fixtures.json"))
                .unwrap();
        let fixture = fixtures
            .into_iter()
            .find(|case| case.name == "css-raw-content")
            .unwrap();
        assert_eq!(
            scan(&fixture.content, "css"),
            ScanResult {
                candidates: fixture.candidates,
                variables: fixture.variables
            }
        );
    }
}
