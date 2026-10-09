//! The compiler's internal helpers behind the names of their fixed upstream
//! counterparts, so the upstream unit tests can execute against Rust.

use super::{
    ast::AstNode,
    public_api::{design_system::public_json, Polyfills, PublicValue, Source},
    value::ValueNode,
    CompileError, Limits,
};
use crate::text::Text;
use std::sync::Arc;

fn syntax(message: &str) -> CompileError {
    CompileError::Syntax {
        offset: 0,
        message: message.into(),
    }
}

/// Upstream splits on the first UTF-16 unit of `separator`.
pub fn segment(
    input: &Text,
    separator: &Text,
    max_depth: usize,
) -> Result<Vec<Text>, CompileError> {
    let unit = *separator
        .units()
        .first()
        .ok_or_else(|| syntax("segment separator is empty"))?;
    super::value::segment_text(input, unit, max_depth)
}

pub fn decode_arbitrary_value(input: &Text, max_depth: usize) -> Result<Text, CompileError> {
    super::value::decode_arbitrary_text(input, max_depth)
}

/// `@source inline(…)` brace expansion.
pub fn expand(pattern: &Text, limits: &Limits) -> Result<Vec<Text>, CompileError> {
    super::sources::expand(pattern, limits, 0)
}

/// Natural class-name order used for class lists and candidate sorting.
pub fn compare(left: &Text, right: &Text) -> std::cmp::Ordering {
    super::compare_classes(left, right)
}

pub fn replace_alpha(color: &Text, alpha: &Text) -> Text {
    super::shadows::replace_alpha(color, alpha)
}

/// Legacy `theme(colors.red.500)` and `fontSize.xs[1]` paths.
pub fn to_key_path(path: &Text) -> Result<Vec<Text>, CompileError> {
    super::legacy_theme::key_path_text(path)
}

pub fn escape(raw: &Text) -> Text {
    super::candidate::escape_text(raw)
}

pub fn unescape(escaped: &Text) -> Result<Text, CompileError> {
    super::legacy_theme::unescape_key_units(escaped)
}

pub fn replace_shadow_colors(
    value: &Text,
    max_bytes: usize,
    replacement: impl FnMut(&Text) -> Text,
) -> Result<Text, CompileError> {
    super::shadows::replace_shadow_colors(value, max_bytes, replacement).map_err(|failure| {
        match failure {
            super::utilities::FamilyFailure::Resource(error) => error,
            super::utilities::FamilyFailure::Candidate(_) => syntax("invalid shadow value"),
        }
    })
}

/// Parses a stylesheet into upstream's plain node objects. With `from`, each
/// node carries `src: [{ file, code }, start, end]` sharing one Source.
pub fn parse_css(
    css: &Text,
    from: Option<String>,
    max_depth: usize,
) -> Result<PublicValue, CompileError> {
    let source = from.map(|file| {
        Arc::new(Source {
            file: Some(file),
            code: css.clone(),
        })
    });
    let shared = source
        .as_ref()
        .map(|source| public_json!({"file": source.file.clone(), "code": source.code.clone()}));
    let nodes = super::parser::parse_with_source(css, max_depth, source)?;
    Ok(PublicValue::from(
        nodes
            .iter()
            .map(|node| css_node(node, shared.as_ref()))
            .collect::<Vec<_>>(),
    ))
}

fn css_node(node: &AstNode, source: Option<&PublicValue>) -> PublicValue {
    let children = |nodes: &[AstNode]| {
        PublicValue::from(
            nodes
                .iter()
                .map(|node| css_node(node, source))
                .collect::<Vec<_>>(),
        )
    };
    let mut value = match node {
        AstNode::Rule {
            selector, nodes, ..
        } => public_json!({"kind": "rule", "selector": selector.clone(), "nodes": children(nodes)}),
        AstNode::AtRule {
            name,
            params,
            nodes,
            ..
        } => public_json!({
            "kind": "at-rule",
            "name": name.clone(),
            "params": params.clone(),
            "nodes": children(nodes.as_deref().unwrap_or_default()),
        }),
        AstNode::Declaration {
            property,
            value,
            important,
            ..
        } => public_json!({
            "kind": "declaration",
            "property": property.clone(),
            "value": value.clone(),
            "important": *important,
        }),
        AstNode::Comment { value, .. } => public_json!({"kind": "comment", "value": value.clone()}),
        AstNode::Context { nodes, .. } | AstNode::AtRoot { nodes, .. } => {
            public_json!({"kind": "at-root", "nodes": children(nodes)})
        }
    };
    if let (Some(shared), Some(location)) = (source, node.source()) {
        value["src"] = public_json!([shared.clone(), location.start, location.end]);
    }
    value
}

/// Parses a CSS value into upstream's `word`/`separator`/`function` nodes.
pub fn parse_value(input: &Text, max_depth: usize) -> Result<PublicValue, CompileError> {
    fn convert(node: &ValueNode) -> PublicValue {
        match node {
            ValueNode::Word { value } => public_json!({"kind": "word", "value": value.clone()}),
            ValueNode::Separator { value } => {
                public_json!({"kind": "separator", "value": value.clone()})
            }
            ValueNode::Function { name, nodes } => public_json!({
                "kind": "function",
                "value": name.clone(),
                "nodes": PublicValue::from(nodes.iter().map(convert).collect::<Vec<_>>()),
            }),
        }
    }
    Ok(PublicValue::from(
        super::value::parse_text(input, max_depth)?
            .iter()
            .map(convert)
            .collect::<Vec<_>>(),
    ))
}

/// Prints upstream value nodes; `function` nodes name the function in `value`.
pub fn value_to_css(nodes: &PublicValue, max_depth: usize) -> Result<Text, CompileError> {
    fn read(
        nodes: &PublicValue,
        depth: usize,
        max_depth: usize,
    ) -> Result<Vec<ValueNode>, CompileError> {
        super::check_limit(depth, max_depth, "ast_depth")?;
        let PublicValue::Array(nodes) = nodes else {
            return Err(syntax("value nodes must be an array"));
        };
        nodes
            .iter()
            .map(|node| {
                let text = |field: &str| {
                    node.get(field)
                        .and_then(PublicValue::as_text)
                        .cloned()
                        .ok_or_else(|| syntax("value node field must be a string"))
                };
                let kind = text("kind")?;
                Ok(if kind == "function" {
                    ValueNode::Function {
                        name: text("value")?,
                        nodes: read(
                            node.get("nodes").unwrap_or(&PublicValue::Null),
                            depth + 1,
                            max_depth,
                        )?,
                    }
                } else if kind == "separator" {
                    ValueNode::Separator {
                        value: text("value")?,
                    }
                } else {
                    ValueNode::Word {
                        value: text("value")?,
                    }
                })
            })
            .collect()
    }
    Ok(super::value::to_css_text(&read(nodes, 0, max_depth)?))
}

/// Typed nodes from upstream's plain node objects. `src` is not needed to print.
pub fn typed_nodes(nodes: &PublicValue, max_depth: usize) -> Result<Vec<AstNode>, CompileError> {
    fn read(
        nodes: &PublicValue,
        depth: usize,
        max_depth: usize,
    ) -> Result<Vec<AstNode>, CompileError> {
        super::check_limit(depth, max_depth, "ast_depth")?;
        let PublicValue::Array(nodes) = nodes else {
            return Err(syntax("AST nodes must be an array"));
        };
        nodes
            .iter()
            .map(|node| {
                let text = |field: &str| node.get(field).and_then(PublicValue::as_text).cloned();
                let required = |field: &str| {
                    text(field).ok_or_else(|| syntax("AST node field must be a string"))
                };
                let children = || {
                    read(
                        node.get("nodes")
                            .unwrap_or(&PublicValue::Array(Default::default())),
                        depth + 1,
                        max_depth,
                    )
                };
                let kind = required("kind")?;
                Ok(if kind == "rule" {
                    AstNode::Rule {
                        origin: None,
                        source: None,
                        destination: None,
                        selector: required("selector")?,
                        nodes: children()?,
                    }
                } else if kind == "at-rule" {
                    AstNode::AtRule {
                        origin: None,
                        source: None,
                        destination: None,
                        name: required("name")?,
                        params: required("params")?,
                        nodes: Some(children()?),
                    }
                } else if kind == "declaration" {
                    AstNode::Declaration {
                        origin: None,
                        source: None,
                        destination: None,
                        property: required("property")?,
                        value: text("value").unwrap_or_default(),
                        important: node.get("important") == Some(&PublicValue::Bool(true)),
                        identity: Arc::new(()),
                    }
                } else if kind == "comment" {
                    AstNode::Comment {
                        origin: None,
                        source: None,
                        destination: None,
                        value: required("value")?,
                    }
                } else if kind == "context" {
                    let context = node.get("context");
                    let field = |name: &str| context.and_then(|context| context.get(name));
                    AstNode::Context {
                        origin: None,
                        base: field("base")
                            .and_then(PublicValue::as_text)
                            .cloned()
                            .unwrap_or_default(),
                        source_base: None,
                        reference: field("reference") == Some(&PublicValue::Bool(true)),
                        nodes: children()?,
                    }
                } else if kind == "at-root" {
                    AstNode::AtRoot {
                        origin: None,
                        nodes: children()?,
                    }
                } else {
                    return Err(syntax("unknown AST node kind"));
                })
            })
            .collect()
    }
    read(nodes, 0, max_depth)
}

/// The compiler's printer, with each printed node's generated UTF-16 span in
/// print order, as upstream `toCss(ast, true)` records in `dst`.
pub fn to_css(nodes: &[AstNode]) -> (Text, Vec<(usize, usize)>) {
    (
        super::ast::to_css(nodes),
        super::public_api::destinations(nodes)
            .into_iter()
            .map(|(_, start, end)| (start, end))
            .collect(),
    )
}

/// The compiler's output passes over an AST without theme variables.
pub fn optimize_ast(
    mut nodes: Vec<AstNode>,
    polyfills: Polyfills,
    max_depth: usize,
) -> Result<Vec<AstNode>, CompileError> {
    let mut globals = Vec::new();
    super::ast::hoist_at_roots(&mut nodes, &mut globals);
    super::ast::optimize_rules(&mut nodes);
    super::ast::optimize_rules(&mut globals);
    if polyfills.contains(Polyfills::COLOR_MIX) {
        let theme = super::theme::Theme::default();
        super::polyfills::color_mix(&mut nodes, &theme, max_depth)?;
        super::polyfills::color_mix(&mut globals, &theme, max_depth)?;
    }
    Ok(super::dependencies::hoist_with_polyfills(
        nodes, globals, polyfills,
    ))
}

/// The output of an optimized tree as upstream plain node objects.
pub fn public_nodes(nodes: &[AstNode]) -> PublicValue {
    PublicValue::from(
        nodes
            .iter()
            .map(|node| css_node(node, None))
            .collect::<Vec<_>>(),
    )
}

/// Registries assembled by upstream unit tests through `Utilities` and
/// `Variants` registration calls, described as plain data:
/// `{ prefix, utilities: { static: [root], functional: [root] },
///    variants: [{ name, kind, compounds?, compoundsWith? }] }`.
/// Omitted `compounds` and `compoundsWith` take upstream's registration defaults.
pub struct TestRegistry {
    prefix: Option<String>,
    statics: std::collections::BTreeSet<Text>,
    functionals: Vec<String>,
    variants: std::collections::BTreeMap<Text, (super::variants::VariantKind, u8, u8)>,
}

/// Upstream `Compounds.StyleRules`, the default for every registration.
const STYLE_RULES: u8 = 2;

pub fn test_registry(description: &PublicValue) -> Result<TestRegistry, CompileError> {
    use super::variants::VariantKind;
    let roots = |kind: &str| -> Result<Vec<Text>, CompileError> {
        match description
            .get("utilities")
            .and_then(|utilities| utilities.get(kind))
        {
            None => Ok(Vec::new()),
            Some(PublicValue::Array(roots)) => roots
                .iter()
                .map(|root| {
                    root.as_text()
                        .cloned()
                        .ok_or_else(|| syntax("utility roots must be strings"))
                })
                .collect(),
            Some(_) => Err(syntax("utility roots must be an array")),
        }
    };
    let mut functionals = roots("functional")?
        .iter()
        .map(|root| {
            root.to_utf8()
                .map_err(|_| syntax("functional roots must be valid UTF-16"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    functionals.sort_unstable_by(|left, right| right.cmp(left));
    functionals.dedup();
    let mut variants = std::collections::BTreeMap::new();
    if let Some(PublicValue::Array(entries)) = description.get("variants") {
        for entry in entries.iter() {
            let field = |name: &str| entry.get(name).and_then(PublicValue::as_text).cloned();
            let flags = |name: &str, default: u8| -> Result<u8, CompileError> {
                match entry.get(name) {
                    None | Some(PublicValue::Null) => Ok(default),
                    Some(value) if value.get("undefined").is_some() => Ok(default),
                    Some(PublicValue::Number(value)) => value
                        .as_f64()
                        .filter(|value| value.fract() == 0.0 && (0.0..=255.0).contains(value))
                        .map(|value| value as u8)
                        .ok_or_else(|| syntax("compounds must be a small integer")),
                    Some(_) => Err(syntax("compounds must be a small integer")),
                }
            };
            let name = field("name").ok_or_else(|| syntax("variant name must be a string"))?;
            let kind = field("kind").ok_or_else(|| syntax("variant kind must be a string"))?;
            let kind = if kind == "static" {
                VariantKind::Static
            } else if kind == "functional" {
                VariantKind::Functional
            } else if kind == "compound" {
                VariantKind::Compound
            } else {
                return Err(syntax("unknown variant kind"));
            };
            let compounds_with = if kind == VariantKind::Compound {
                flags("compoundsWith", 0)?
            } else {
                0
            };
            variants.insert(
                name,
                (kind, flags("compounds", STYLE_RULES)?, compounds_with),
            );
        }
    }
    Ok(TestRegistry {
        prefix: description
            .get("prefix")
            .and_then(PublicValue::as_text)
            .map(|prefix| {
                prefix
                    .to_utf8()
                    .map_err(|_| syntax("prefix must be valid UTF-16"))
            })
            .transpose()?,
        statics: roots("static")?.into_iter().collect(),
        functionals,
        variants,
    })
}

impl super::candidate::Registry for TestRegistry {
    fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }
    fn has_static_utility(&self, root: &Text) -> bool {
        self.statics.contains(root)
    }
    fn functional_roots(&self) -> impl Iterator<Item = &str> + '_ {
        self.functionals.iter().map(String::as_str)
    }
    fn variant_kind(&self, root: &Text) -> Option<super::variants::VariantKind> {
        self.variants.get(root).map(|(kind, ..)| *kind)
    }
    fn variant_compounds(&self, variant: &super::candidate::Variant) -> u8 {
        match variant {
            super::candidate::Variant::Registered { root, .. } => self
                .variants
                .get(root)
                .map_or(0, |(_, compounds, _)| *compounds),
            super::candidate::Variant::Arbitrary { selector, .. } => {
                super::variants::selector_compounds(selector)
            }
        }
    }
    fn compounds_with(&self, root: &Text) -> u8 {
        self.variants.get(root).map_or(0, |(_, _, with)| *with)
    }
}

/// Upstream `designSystem.parseCandidate` against an injected registry.
pub fn parse_candidate(
    raw: &Text,
    registry: &TestRegistry,
    max_depth: usize,
) -> Result<PublicValue, CompileError> {
    let mut output = Vec::new();
    for candidate in super::candidate::parse_with(raw, max_depth, registry)? {
        let parts = super::value::segment_text(raw, 58, max_depth)?;
        let start = usize::from(registry.prefix.is_some());
        let variants = parts[start..parts.len() - 1]
            .iter()
            .rev()
            .map(|part| parse_variant(part, registry, max_depth))
            .collect::<Result<Vec<_>, _>>()?;
        output.push(super::public_api::design_system::public_candidate(
            candidate, variants, max_depth,
        )?);
    }
    Ok(PublicValue::from(output))
}

/// Upstream `designSystem.parseVariant` against an injected registry.
pub fn parse_variant(
    raw: &Text,
    registry: &TestRegistry,
    max_depth: usize,
) -> Result<PublicValue, CompileError> {
    super::public_api::design_system::typed(&super::candidate::parse_variant_object_with(
        raw, max_depth, registry,
    )?)
}

/// Upstream `compoundsForSelectors`: style rules, conditional at-rules, or never.
pub fn compounds_for_selectors(selectors: &[Text]) -> u8 {
    let mut compounds = 0;
    for selector in selectors {
        let flags = super::variants::selector_compounds(selector);
        if flags == 0 {
            return 0;
        }
        compounds |= flags;
    }
    compounds
}
