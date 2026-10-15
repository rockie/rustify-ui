//! Verbatim code fragments in diagnostic messages and repairs.
//!
//! This is Rustify's diagnostic notation, not CommonMark: spaces and line
//! endings are preserved. Separate fragments with non-backtick prose so
//! their fences cannot merge.

/// Quotes a complete code value with a fence longer than any run it contains.
pub fn code_span(value: &str) -> String {
    if value.is_empty() {
        return "``  ``".into();
    }
    let longest = value.split(|ch| ch != '`').map(str::len).max().unwrap_or(0);
    if longest == 0 {
        return format!("`{value}`");
    }
    let fence = "`".repeat(longest + 1);
    format!("{fence} {value} {fence}")
}

/// Keeps ordinary author text as written, quoting the whole value if it has ticks.
pub fn literal(value: &str) -> String {
    if value.contains('`') {
        code_span(value)
    } else {
        value.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_empty_whitespace_line_endings_and_tick_runs() {
        for (value, encoded) in [
            ("", "``  ``"),
            (" ", "` `"),
            ("  ", "`  `"),
            (" a ", "` a `"),
            ("a\nb", "`a\nb`"),
            ("a\r\nb", "`a\r\nb`"),
            ("a\\b", "`a\\b`"),
            ("Counter", "`Counter`"),
            ("a`b", "`` a`b ``"),
            ("a``b", "``` a``b ```"),
            ("`a`", "`` `a` ``"),
            ("`", "`` ` ``"),
            ("``", "``` `` ```"),
            (" `\r\n ", "``  `\r\n  ``"),
        ] {
            assert_eq!(code_span(value), encoded, "{value:?}");
            assert_eq!(
                literal(value),
                if value.contains('`') { encoded } else { value }
            );
        }
    }
}
