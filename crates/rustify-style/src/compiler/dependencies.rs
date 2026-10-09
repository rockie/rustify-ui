//! Ordered compiler-owned resources and @property compatibility fallbacks.

use super::ast::AstNode;
use std::collections::BTreeSet;

pub fn property(name: &str, initial: Option<&str>, syntax: &str) -> AstNode {
    let mut nodes = vec![
        AstNode::declaration("syntax", format!("\"{syntax}\"")),
        AstNode::declaration("inherits", "false"),
    ];
    if let Some(value) = initial {
        nodes.push(AstNode::declaration("initial-value", value));
    }
    AstNode::AtRule {
        origin: None,
        source: None,
        destination: None,
        name: "@property".into(),
        params: name.into(),
        nodes: Some(nodes),
    }
}

pub fn content_property(pseudo: bool) -> AstNode {
    let mut node = property("--tw-content", Some("\"\""), "*");
    if pseudo {
        node.children_mut().unwrap().swap(1, 2);
    }
    node
}

pub fn box_shadow_properties() -> Vec<AstNode> {
    [
        ("--tw-shadow", Some("0 0 #0000"), "*"),
        ("--tw-shadow-color", None, "*"),
        ("--tw-shadow-alpha", Some("100%"), "<percentage>"),
        ("--tw-inset-shadow", Some("0 0 #0000"), "*"),
        ("--tw-inset-shadow-color", None, "*"),
        ("--tw-inset-shadow-alpha", Some("100%"), "<percentage>"),
        ("--tw-ring-color", None, "*"),
        ("--tw-ring-shadow", Some("0 0 #0000"), "*"),
        ("--tw-inset-ring-color", None, "*"),
        ("--tw-inset-ring-shadow", Some("0 0 #0000"), "*"),
        ("--tw-ring-inset", None, "*"),
        ("--tw-ring-offset-width", Some("0px"), "<length>"),
        ("--tw-ring-offset-color", Some("#fff"), "*"),
        ("--tw-ring-offset-shadow", Some("0 0 #0000"), "*"),
    ]
    .into_iter()
    .map(|(name, initial, syntax)| property(name, initial, syntax))
    .collect()
}

pub fn hoist_with_polyfills(
    mut stylesheet: Vec<AstNode>,
    resources: Vec<AstNode>,
    polyfills: super::public_api::Polyfills,
) -> Vec<AstNode> {
    fn collect_property(
        resource: &AstNode,
        seen: &mut BTreeSet<crate::text::Text>,
        root_fallbacks: &mut Vec<AstNode>,
        universal_fallbacks: &mut Vec<AstNode>,
    ) -> bool {
        let AstNode::AtRule {
            name,
            params,
            nodes: Some(nodes),
            ..
        } = resource
        else {
            return true;
        };
        if name != "@property" {
            return true;
        }
        if !seen.insert(params.clone()) {
            return false;
        }
        let default_initial = crate::text::Text::from("initial");
        let mut initial = &default_initial;
        let mut inherits = false;
        for node in nodes {
            if let AstNode::Declaration {
                property, value, ..
            } = node
            {
                if property == "initial-value" {
                    initial = value;
                }
                if property == "inherits" {
                    inherits = value == "true";
                }
            }
        }
        let mut fallback = AstNode::declaration(params.clone(), initial.clone());
        // Upstream assigns `fallback.src = node.src` even when it is undefined.
        fallback.set_source(Some(
            resource
                .source()
                .cloned()
                .unwrap_or_else(super::public_api::SourceLocation::undefined),
        ));
        if inherits {
            root_fallbacks.push(fallback);
        } else {
            universal_fallbacks.push(fallback);
        }
        true
    }
    fn authored_properties(
        nodes: &mut Vec<AstNode>,
        seen: &mut BTreeSet<crate::text::Text>,
        root_fallbacks: &mut Vec<AstNode>,
        universal_fallbacks: &mut Vec<AstNode>,
    ) {
        nodes.retain_mut(|node| {
            if let AstNode::Context {
                reference, nodes, ..
            } = node
            {
                if !*reference {
                    authored_properties(nodes, seen, root_fallbacks, universal_fallbacks);
                }
                return true;
            }
            collect_property(node, seen, root_fallbacks, universal_fallbacks)
        });
    }
    let mut seen = BTreeSet::new();
    let mut root_fallbacks = Vec::new();
    let mut universal_fallbacks = Vec::new();
    authored_properties(
        &mut stylesheet,
        &mut seen,
        &mut root_fallbacks,
        &mut universal_fallbacks,
    );
    for resource in resources {
        if !collect_property(
            &resource,
            &mut seen,
            &mut root_fallbacks,
            &mut universal_fallbacks,
        ) {
            continue;
        }
        stylesheet.push(resource);
    }
    if !polyfills.contains(super::public_api::Polyfills::AT_PROPERTY) {
        return stylesheet;
    }
    let mut fallbacks = Vec::new();
    if !root_fallbacks.is_empty() {
        fallbacks.push(AstNode::Rule {
            origin: None,
            source: root_fallbacks[0].source().cloned(),
            destination: None,
            selector: ":root, :host".into(),
            nodes: root_fallbacks,
        });
    }
    if !universal_fallbacks.is_empty() {
        fallbacks.push(AstNode::Rule {
            origin: None,
            source: universal_fallbacks[0].source().cloned(),
            destination: None,
            selector: "*, ::before, ::after, ::backdrop".into(),
            nodes: universal_fallbacks,
        });
    }
    if !fallbacks.is_empty() {
        let source = fallbacks[0].source().cloned();
        let index = stylesheet.iter().position(|node| !matches!(node, AstNode::Comment { .. }) && !matches!(node, AstNode::AtRule { name, .. } if name == "@charset" || name == "@import")).unwrap_or(stylesheet.len());
        stylesheet.insert(
            index,
            AstNode::AtRule {
                origin: None,
                source: source.clone(),
                destination: None,
                name: "@layer".into(),
                params: "properties".into(),
                nodes: None,
            },
        );
        stylesheet.push(AstNode::AtRule { origin: None, source: source.clone(), destination: None, name: "@layer".into(), params: "properties".into(), nodes: Some(vec![AstNode::AtRule { origin: None, source, destination: None,
            name: "@supports".into(),
            params: "((-webkit-hyphens: none) and (not (margin-trim: inline))) or ((-moz-orient: inline) and (not (color:rgb(from red r g b))))".into(),
            nodes: Some(fallbacks),
        }]) });
    }
    stylesheet
}
