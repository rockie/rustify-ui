//! CSS color-mix fallbacks follow the fixed upstream AST optimization order.

use super::{
    ast::AstNode,
    theme::Theme,
    value::{self, ValueNode},
    CompileError,
};
use std::{collections::BTreeSet, sync::Arc};

fn inline_variables(
    nodes: &mut [ValueNode],
    theme: &Theme,
    depth: usize,
    required: &mut bool,
    unresolved: &mut bool,
    current: &mut bool,
) -> Result<(), CompileError> {
    for node in nodes {
        if matches!(node, ValueNode::Word { value } if value.eq_ignore_ascii_case("currentcolor")) {
            *current = true;
            *required = true;
        }
        if matches!(node, ValueNode::Function { name, .. } if name == "var") {
            let mut candidate = node.clone();
            let mut seen = BTreeSet::new();
            let mut replacement = None;
            let mut resolved = true;
            while let ValueNode::Function { name, nodes } = candidate {
                if name != "var" {
                    break;
                }
                let Some(ValueNode::Word { value: variable }) = nodes.first() else {
                    break;
                };
                *required = true;
                if !seen.insert(variable.clone()) {
                    *unresolved = true;
                    resolved = false;
                    break;
                }
                let Some(token) = theme.tokens.get(variable) else {
                    *unresolved = true;
                    resolved = false;
                    break;
                };
                if token.value.is_empty() {
                    *unresolved = true;
                    resolved = false;
                    break;
                }
                if token.value.eq_ignore_ascii_case("currentcolor") {
                    *current = true;
                    resolved = false;
                    break;
                }
                replacement = Some(token.value.clone());
                if token.value.starts_with("var(") {
                    let parsed = value::parse_text(&token.value, depth)?;
                    if let Some(next) = parsed.into_iter().next() {
                        candidate = next;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if resolved {
                if let Some(value) = replacement {
                    *node = ValueNode::Word { value };
                }
            }
        }
        if let ValueNode::Function { nodes, .. } = node {
            inline_variables(nodes, theme, depth, required, unresolved, current)?;
        }
    }
    Ok(())
}

fn fallback_values(
    nodes: &mut [ValueNode],
    theme: &Theme,
    depth: usize,
    required: &mut bool,
) -> Result<(), CompileError> {
    for node in nodes {
        let mut replaced = false;
        if let ValueNode::Function { name, nodes } = node {
            if name == "color-mix" {
                let mut unresolved = false;
                let mut current = false;
                inline_variables(nodes, theme, depth, required, &mut unresolved, &mut current)?;
                if unresolved || current {
                    if let Some(index) = nodes.iter().position(|node| matches!(node, ValueNode::Separator { value } if value.contains(","))) {
                        if let Some(first) = nodes.get(index + 1) { *node = first.clone(); replaced = true; }
                    }
                } else if *required {
                    if let Some(ValueNode::Word { value }) = nodes.get_mut(2) {
                        if ["oklab", "oklch", "lab", "lch"]
                            .iter()
                            .any(|name| value == *name)
                        {
                            *value = "srgb".into();
                        }
                    }
                }
            }
        }
        if replaced {
            fallback_values(std::slice::from_mut(node), theme, depth, required)?;
        } else if let ValueNode::Function { nodes, .. } = node {
            fallback_values(nodes, theme, depth, required)?;
        }
    }
    Ok(())
}

pub fn color_mix(
    nodes: &mut Vec<AstNode>,
    theme: &Theme,
    depth_limit: usize,
) -> Result<(), CompileError> {
    color_mix_inner(nodes, theme, depth_limit, true)
}

fn color_mix_inner(
    nodes: &mut Vec<AstNode>,
    theme: &Theme,
    depth_limit: usize,
    root: bool,
) -> Result<(), CompileError> {
    let mut output = Vec::new();
    let mut declarations = BTreeSet::new();
    for mut node in std::mem::take(nodes) {
        if let Some(children) = node.children_mut() {
            color_mix_inner(children, theme, depth_limit, false)?;
        }
        if let AstNode::Declaration {
            property,
            value,
            important,
            identity,
            ..
        } = &node
        {
            if !root
                && value.contains("color-mix(")
                && declarations.insert(Arc::as_ptr(identity) as usize)
            {
                let mut ast = value::parse_text(value, depth_limit)?;
                let mut required = false;
                fallback_values(&mut ast, theme, depth_limit, &mut required)?;
                if required {
                    output.push(AstNode::Declaration {
                        origin: None,
                        source: node.source().cloned(),
                        destination: node.destination().cloned(),
                        property: property.clone(),
                        value: value::to_css_text(&ast),
                        important: *important,
                        identity: Arc::new(()),
                    });
                    output.push(AstNode::AtRule {
                        origin: None,
                        source: node.source().cloned(),
                        destination: node.destination().cloned(),
                        name: "@supports".into(),
                        params: "(color: color-mix(in lab, red, red))".into(),
                        nodes: Some(vec![node]),
                    });
                    continue;
                }
            }
        }
        output.push(node);
    }
    *nodes = output;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::parser;

    #[test]
    fn shared_color_mix_declarations_get_one_fallback_per_parent() {
        let mut theme_nodes = parser::parse("@theme { --color-brand: #123456; }", 32).unwrap();
        let theme = Theme::extract(&mut theme_nodes, 32).unwrap();
        let raw = "color-mix(in oklab, var(--color-brand) 50%, transparent)";
        let shared = AstNode::declaration("background-color", raw);
        let distinct = AstNode::declaration("background-color", raw);
        let mut nodes = vec![
            AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: "@media".into(),
                params: "screen".into(),
                nodes: Some(vec![shared.clone(), shared.clone(), distinct]),
            },
            AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: "@media".into(),
                params: "print".into(),
                nodes: Some(vec![shared.clone()]),
            },
        ];
        color_mix(&mut nodes, &theme, 32).unwrap();
        let identity = |node: &AstNode| match node {
            AstNode::Declaration { identity, .. } => identity.clone(),
            _ => panic!("expected declaration"),
        };
        let screen = nodes[0].children().unwrap();
        let print = nodes[1].children().unwrap();
        assert!(
            screen.len() == 5
                && print.len() == 2
                && matches!(&screen[0], AstNode::Declaration { value, .. } if value == "color-mix(in srgb, #123456 50%, transparent)")
                && matches!(&screen[2], AstNode::Declaration { value, .. } if value == raw)
                && Arc::ptr_eq(
                    &identity(&shared),
                    &identity(&screen[1].children().unwrap()[0])
                )
                && Arc::ptr_eq(&identity(&shared), &identity(&screen[2]))
                && Arc::ptr_eq(
                    &identity(&shared),
                    &identity(&print[1].children().unwrap()[0])
                )
                && !Arc::ptr_eq(&identity(&shared), &identity(&screen[0]))
        );
    }
}
