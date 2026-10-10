//! What a build reports about its input, with a location an author can open.

use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Error,
    Warning,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

/// A finding about one source file, located by byte offsets into that file.
/// Turning offsets into a line and column needs the file's text, which the
/// caller has; see [`LineIndex`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Issue {
    pub span: Range<usize>,
    pub severity: Severity,
    /// A stable kebab-case name for the rule, which `--format json` reports.
    pub rule: &'static str,
    pub message: String,
    pub suggestion: Option<String>,
}

impl Issue {
    pub fn error(span: Range<usize>, rule: &'static str, message: impl Into<String>) -> Self {
        Self {
            span,
            severity: Severity::Error,
            rule,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn warning(span: Range<usize>, rule: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            ..Self::error(span, rule, message)
        }
    }

    pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

/// Byte offsets to 1-based lines and columns. Columns count characters, so
/// a position after Chinese text is where an editor puts the cursor.
pub struct LineIndex<'a> {
    text: &'a str,
    starts: Vec<usize>,
}

impl<'a> LineIndex<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut starts = vec![0];
        starts.extend(text.match_indices('\n').map(|(i, _)| i + 1));
        Self { text, starts }
    }

    pub fn position(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.text.len());
        let line = self.starts.partition_point(|&start| start <= offset) - 1;
        let start = self.starts[line];
        let column = self
            .text
            .get(start..offset)
            .map_or(offset - start, |prefix| prefix.chars().count());
        (line + 1, column + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_are_one_based_and_count_characters() {
        let index = LineIndex::new("ab\n中文x\n\nend");
        assert_eq!(index.position(0), (1, 1));
        assert_eq!(index.position(2), (1, 3));
        assert_eq!(index.position(3), (2, 1));
        assert_eq!(index.position(9), (2, 3));
        assert_eq!(index.position(11), (3, 1));
        assert_eq!(index.position(12), (4, 1));
        assert_eq!(index.position(99), (4, 4));
    }
}
