//! Candidate seed from the generated utilities layer, independent of source scanning.

use rustify_style::compiler::{ast::AstNode, parser};
use std::collections::BTreeSet;

pub(super) fn candidates(css: &str) -> Result<Vec<String>, String> {
    fn has_utility_layer(nodes: &[AstNode]) -> bool {
        nodes.iter().any(|node| {
            matches!(node, AstNode::AtRule { name, params, .. } if name == "@layer" && params.trim_whitespace() == "utilities")
                || node.children().is_some_and(has_utility_layer)
        })
    }
    fn walk(
        nodes: &[AstNode],
        utilities: bool,
        output: &mut BTreeSet<String>,
    ) -> Result<(), String> {
        for node in nodes {
            match node {
                AstNode::Rule { selector, .. } if utilities => {
                    // Descendants are variant/structural selectors, not additional input classes.
                    selectors(
                        &selector
                            .to_utf8()
                            .map_err(|_| "non-scalar static seed selector")?,
                        output,
                    );
                }
                AstNode::AtRule { name, params, .. } if name == "@layer" => {
                    if let Some(nodes) = node.children() {
                        walk(nodes, params.trim_whitespace() == "utilities", output)?;
                    }
                }
                _ => {
                    if let Some(nodes) = node.children() {
                        walk(nodes, utilities, output)?;
                    }
                }
            }
        }
        Ok(())
    }
    let ast = parser::parse(css, usize::MAX).map_err(|error| error.to_string())?;
    let mut output = BTreeSet::new();
    walk(&ast, !has_utility_layer(&ast), &mut output)?;
    Ok(output.into_iter().collect())
}

fn selectors(input: &str, output: &mut BTreeSet<String>) {
    let mut chars = input.chars().peekable();
    let mut attribute = false;
    while let Some(ch) = chars.next() {
        match ch {
            '\'' | '"' => {
                while let Some(value) = chars.next() {
                    if value == '\\' {
                        chars.next();
                    } else if value == ch {
                        break;
                    }
                }
            }
            '[' => attribute = true,
            ']' => attribute = false,
            '\\' => {
                chars.next();
            }
            '.' if !attribute => {
                let mut value = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '\\' {
                        chars.next();
                        let mut hex = String::new();
                        while hex.len() < 6 && chars.peek().is_some_and(char::is_ascii_hexdigit) {
                            hex.push(chars.next().expect("peeked character"));
                        }
                        if hex.is_empty() {
                            if let Some(ch) = chars.next() {
                                value.push(ch);
                            }
                        } else {
                            if chars.peek().is_some_and(char::is_ascii_whitespace) {
                                let terminator = chars.next();
                                if terminator == Some('\r') && chars.peek() == Some(&'\n') {
                                    chars.next();
                                }
                            }
                            let scalar = u32::from_str_radix(&hex, 16).unwrap_or(0);
                            value.push(
                                char::from_u32(scalar)
                                    .filter(|ch| *ch != '\0')
                                    .unwrap_or('\u{fffd}'),
                            );
                        }
                    } else if ch == '-' || ch == '_' || ch.is_ascii_alphanumeric() || !ch.is_ascii()
                    {
                        value.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !value.is_empty() {
                    output.insert(value);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_layer_keeps_escaped_candidates_and_excludes_nested_context() {
        let css = r#".application {color:red} @layer utilities {
            .p-4 {padding:1rem}
            .hover\:bg-\[\#123456\] { &:hover {color:red} }
            .group-hover\:block { &:is(:where(.group):hover *) {display:block} }
            @media (width > 20px) { .sm\:block,.block {display:block} }
            .\32 xl\:flex {display:flex}
            .café\:block[data-dot=".fake"] {display:block}
        }"#;
        assert_eq!(
            candidates(css).unwrap(),
            [
                "2xl:flex",
                "block",
                "café:block",
                "group-hover:block",
                "hover:bg-[#123456]",
                "p-4",
                "sm:block"
            ]
        );
    }

    #[test]
    fn committed_css_seed_contains_generated_classes_only() {
        let css = include_str!("../../../crates/rustify-components/css/rustify.css");
        let seed = candidates(css).unwrap();
        assert!(seed.iter().any(|class| class == "p-4"));
        assert!(seed.iter().any(|class| class == "focus-visible:ring-[3px]"));
        assert!(!seed.iter().any(|class| class == "group"));
        assert!(!seed.iter().any(|class| class == "theme"));
    }

    #[test]
    fn unlayered_utilities_are_seeded_without_base_or_property_selectors() {
        assert_eq!(
            candidates(
                "@layer base { .base { color:red; } } @scope (.host) { .p-4 { padding:1rem; } }"
            )
            .unwrap(),
            ["p-4"]
        );
    }
}
