//! CSS-native utility application and dependency ordering.

use super::value::{concat_text, split_whitespace_text};
use super::{
    ast::{self, AstNode},
    candidate::{self, CandidateKind},
    custom_utilities::CustomUtilities,
    theme::Theme,
    utilities::{self, CompileFailure, CompiledCandidate},
    CompileError, Limits,
};
use crate::text::Text;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

pub(super) struct Replacement {
    pub source: Arc<super::public_api::PublicValue>,
    pub nodes: Vec<AstNode>,
}

struct Expansion<'a> {
    bytes: usize,
    replacements: &'a mut Vec<Replacement>,
}

fn syntax(message: impl Into<Text>) -> CompileError {
    CompileError::Syntax {
        offset: 0,
        message: message.into(),
    }
}

// Stable arena indices preserve parent identity while replacing sibling rules.
struct Node {
    value: AstNode,
    children: Vec<usize>,
    definition: Option<usize>,
}

fn insert(value: AstNode, arena: &mut Vec<Node>, custom: &CustomUtilities) -> usize {
    let index = arena.len();
    let mut value = value;
    let definition = custom.definition_index(&value);
    if let Some(definition) = definition {
        let (name, nodes) = custom.definition(definition);
        value = AstNode::AtRule {
            origin: None,
            source: None,
            destination: None,
            name: "@utility".into(),
            params: name.into(),
            nodes: Some(nodes.to_vec()),
        };
    }
    let children = value.children_mut().map(std::mem::take).unwrap_or_default();
    arena.push(Node {
        value,
        children: Vec::new(),
        definition,
    });
    let children = children
        .into_iter()
        .map(|child| insert(child, arena, custom))
        .collect();
    arena[index].children = children;
    index
}

fn restore(index: usize, arena: &[Node], hide_definitions: bool) -> AstNode {
    let mut node = arena[index].value.clone();
    if let Some(children) = node.children_mut() {
        *children = arena[index]
            .children
            .iter()
            .filter(|&&child| !hide_definitions || arena[child].definition.is_none())
            .map(|&child| restore(child, arena, hide_definitions))
            .collect();
    }
    node
}

fn roots(
    params: &Text,
    theme: &Theme,
    custom: &CustomUtilities,
    limits: &Limits,
) -> Result<Vec<Text>, CompileError> {
    let mut roots = Vec::new();
    for raw in split_whitespace_text(params)
        .iter()
        .filter(|raw| !raw.is_empty())
    {
        for candidate in candidate::parse_with_utilities(raw, limits.ast_depth, theme, custom)? {
            let root = match candidate.kind {
                CandidateKind::Static { root } | CandidateKind::Functional { root, .. } => root,
                CandidateKind::ArbitraryProperty { .. } => continue,
            };
            if !roots.contains(&root) {
                roots.push(root);
            }
        }
    }
    Ok(roots)
}

fn contains_apply(index: usize, arena: &[Node]) -> bool {
    matches!(&arena[index].value, AstNode::AtRule { name, .. } if name == "@apply")
        || arena[index]
            .children
            .iter()
            .any(|&child| contains_apply(child, arena))
}

struct Graph {
    parents: Vec<usize>,
    dependencies: BTreeMap<usize, Vec<Text>>,
    definitions: BTreeMap<Text, Vec<usize>>,
}

impl Graph {
    fn parent(&mut self, index: usize) {
        if !self.dependencies.contains_key(&index) {
            self.parents.push(index);
            self.dependencies.insert(index, Vec::new());
        }
    }
    fn add(&mut self, index: usize, roots: &[Text]) {
        let dependencies = self.dependencies.entry(index).or_default();
        for root in roots {
            if !dependencies.contains(root) {
                dependencies.push(root.clone());
            }
        }
    }
    fn collect(
        &mut self,
        index: usize,
        arena: &[Node],
        path: &mut Vec<usize>,
        theme: &Theme,
        custom: &CustomUtilities,
        limits: &Limits,
    ) -> Result<(), CompileError> {
        let node = &arena[index];
        if let AstNode::AtRule { name, params, .. } = &node.value {
            if name == "@keyframes" {
                if contains_apply(index, arena) {
                    return Err(syntax("You cannot use `@apply` inside `@keyframes`."));
                }
                return Ok(());
            }
            if name == "@utility" {
                self.definitions
                    .entry(params.strip_suffix("-*").unwrap_or_else(|| params.clone()))
                    .or_default()
                    .push(index);
                if contains_apply(index, arena) {
                    self.parent(index);
                    fn collect_roots(
                        index: usize,
                        arena: &[Node],
                        theme: &Theme,
                        custom: &CustomUtilities,
                        limits: &Limits,
                    ) -> Result<Vec<Text>, CompileError> {
                        let mut output = Vec::new();
                        if let AstNode::AtRule { name, params, .. } = &arena[index].value {
                            if name == "@apply" {
                                output.extend(roots(params, theme, custom, limits)?);
                            }
                        }
                        for &child in &arena[index].children {
                            output.extend(collect_roots(child, arena, theme, custom, limits)?);
                        }
                        Ok(output)
                    }
                    self.add(index, &collect_roots(index, arena, theme, custom, limits)?);
                }
            }
            if name == "@apply" {
                if let Some(&parent) = path.last() {
                    self.parent(parent);
                    let dependencies = roots(params, theme, custom, limits)?;
                    for &parent in path.iter() {
                        if self.dependencies.contains_key(&parent) {
                            self.add(parent, &dependencies);
                        }
                    }
                }
            }
        }
        let transparent = matches!(node.value, AstNode::Context { .. });
        if !transparent {
            path.push(index);
        }
        for &child in &node.children {
            self.collect(child, arena, path, theme, custom, limits)?;
        }
        if !transparent {
            path.pop();
        }
        Ok(())
    }

    fn sort(
        &self,
        arena: &[Node],
        theme: &Theme,
        custom: &CustomUtilities,
        limits: &Limits,
    ) -> Result<Vec<usize>, CompileError> {
        let mut seen = BTreeSet::new();
        let mut active: Vec<usize> = Vec::new();
        let mut sorted = Vec::new();
        for &parent in &self.parents {
            let mut stack = vec![(parent, false)];
            while let Some((index, exit)) = stack.pop() {
                if exit {
                    active.pop();
                    seen.insert(index);
                    sorted.push(index);
                    continue;
                }
                if seen.contains(&index) {
                    continue;
                }
                if let Some(position) = active.iter().position(|&node| node == index) {
                    let next = active.get(position + 1).copied().unwrap_or(active[0]);
                    let root = match &arena[next].value {
                        AstNode::AtRule { params, .. } => {
                            params.strip_suffix("-*").unwrap_or_else(|| params.clone())
                        }
                        _ => Text::default(),
                    };
                    let mut pending = vec![index];
                    while let Some(node) = pending.pop() {
                        if let AstNode::AtRule { name, params, .. } = &arena[node].value {
                            if name == "@apply" {
                                for raw in split_whitespace_text(params)
                                    .iter()
                                    .filter(|raw| !raw.is_empty())
                                {
                                    if roots(raw, theme, custom, limits)?
                                        .iter()
                                        .any(|candidate| candidate == &root)
                                    {
                                        return Err(syntax(concat_text(&[&"You cannot `@apply` the `".into(), raw, &"` utility here because it creates a circular dependency.".into()])));
                                    }
                                }
                            }
                        }
                        pending.extend(arena[node].children.iter().rev());
                    }
                    return Err(syntax(concat_text(&[
                        &"Circular dependency detected:\n\n".into(),
                        &ast::to_css(&[restore(index, arena, false)]),
                        &"\nRelies on:\n\n".into(),
                        &ast::to_css(&[restore(next, arena, false)]),
                    ])));
                }
                active.push(index);
                stack.push((index, true));
                let edges = self
                    .dependencies
                    .get(&index)
                    .into_iter()
                    .flatten()
                    .flat_map(|root| self.definitions.get(root).into_iter().flatten())
                    .copied()
                    .collect::<Vec<_>>();
                for dependency in edges.into_iter().rev() {
                    stack.push((dependency, false));
                }
            }
        }
        Ok(sorted)
    }
}

fn compile(
    raw: &Text,
    theme: &Theme,
    custom: &CustomUtilities,
    limits: &Limits,
) -> Result<Vec<CompiledCandidate>, CompileError> {
    let mut output = Vec::new();
    if theme.blocked.contains(raw) {
        return Ok(output);
    }
    for interpretation in candidate::parse_with_utilities(raw, limits.ast_depth, theme, custom)? {
        match utilities::compile_all(&interpretation, theme, limits, custom) {
            Ok(compiled) => output.extend(compiled),
            Err(CompileFailure::Resource(error)) => return Err(error),
            Err(CompileFailure::Candidate(_)) => {}
        }
    }
    Ok(output)
}

fn invalid(
    raw: &Text,
    theme: &Theme,
    custom: &CustomUtilities,
    limits: &Limits,
) -> Result<CompileError, CompileError> {
    if let Some(prefix) = &theme.prefix {
        if !raw.starts_with(prefix) {
            return Ok(syntax(concat_text(&[
                &"Cannot apply unprefixed utility class `".into(),
                raw,
                &"`. Did you mean `".into(),
                &prefix.clone().into(),
                &":".into(),
                raw,
                &"`?".into(),
            ])));
        }
    }
    if theme.blocked.contains(raw) {
        return Ok(syntax(concat_text(&[&"Cannot apply utility class `".into(), raw, &"` because it has been explicitly disabled: https://tailwindcss.com/docs/detecting-classes-in-source-files#explicitly-excluding-classes".into()])));
    }
    let parts = super::value::segment_text(raw, 58, limits.ast_depth)?;
    if let Some((utility, variants)) = parts.split_last() {
        if !variants.is_empty() && !compile(utility, theme, custom, limits)?.is_empty() {
            let mut unknown = Vec::new();
            for variant in variants {
                if compile(
                    &concat_text(&[variant, &":[--tw-variant-check:1]".into()]),
                    theme,
                    custom,
                    limits,
                )?
                .is_empty()
                {
                    unknown.push(concat_text(&[&"`".into(), variant, &"`".into()]));
                }
            }
            if !unknown.is_empty() {
                let suffix = if unknown.len() == 1 {
                    "variant does"
                } else {
                    "variants do"
                };
                let names = match unknown.len() {
                    1 => unknown[0].clone(),
                    2 => super::shadows::join_text(&unknown, " and "),
                    _ => concat_text(&[
                        &super::shadows::join_text(&unknown[..unknown.len() - 1], ", "),
                        &", and ".into(),
                        unknown.last().unwrap_or(&Text::default()),
                    ]),
                };
                return Ok(syntax(concat_text(&[
                    &"Cannot apply utility class `".into(),
                    raw,
                    &"` because the ".into(),
                    &names,
                    &Text::from(format!(" {suffix} not exist.")),
                ])));
            }
        }
    }
    Ok(syntax(concat_text(&[
        &"Cannot apply unknown utility class `".into(),
        raw,
        &Text::from(if theme.tokens.is_empty() {
            "`. Are you using CSS modules or similar and missing `@reference`? https://tailwindcss.com/docs/functions-and-directives#reference-directive"
        } else {
            "`"
        }),
    ])))
}

fn expand(
    params: &Text,
    source: Option<&super::public_api::SourceLocation>,
    theme: &Theme,
    custom: &CustomUtilities,
    limits: &Limits,
) -> Result<Vec<AstNode>, CompileError> {
    let mut compiled = Vec::new();
    let mut seen = BTreeSet::new();
    // Upstream splits whitespace without bracket-aware segmentation, including an empty argument.
    for raw in split_whitespace_text(params) {
        if raw.is_empty() && !params.is_empty() {
            continue;
        }
        if !seen.insert(raw.clone()) {
            continue;
        }
        super::check_limit(seen.len(), limits.candidate_count, "candidate_count")?;
        let output = compile(&raw, theme, custom, limits)?;
        if output.is_empty() {
            return Err(invalid(&raw, theme, custom, limits)?);
        }
        compiled.extend(output);
    }
    super::check_limit(seen.len(), limits.candidate_count, "candidate_count")?;
    compiled.sort_by(super::compare_rules);
    let mut nodes = Vec::new();
    for output in compiled {
        let candidate_source = source.cloned().map(|mut source| {
            let raw = &output.program.candidate;
            let units = params.units();
            let mut offset = 0;
            let mut index = 0;
            while index < units.len() {
                if Text::is_js_whitespace(units[index]) {
                    index += 1;
                    continue;
                }
                let start = index;
                while index < units.len() && !Text::is_js_whitespace(units[index]) {
                    index += 1;
                }
                if &units[start..index] == raw.units() {
                    offset = start;
                }
            }
            source.start += 7 + offset;
            source.end = source.start + raw.units().len();
            source.offset_public(7 + offset, raw.units().len());
            source
        });
        let mut replacement = if let AstNode::Rule {
            nodes: children, ..
        } = output.node
        {
            ast::clone_fresh_checked(&children, None, limits.cache_bytes)?
        } else {
            ast::clone_fresh_checked(&[output.node], None, limits.cache_bytes)?
        };
        ast::replace_sources(&mut replacement, candidate_source.as_ref());
        nodes.extend(replacement);
    }
    ast::validate_depth(&nodes, limits.ast_depth)?;
    super::check_limit(ast::to_css(&nodes).len(), limits.css_bytes, "css_bytes")?;
    Ok(nodes)
}

fn replace(
    index: usize,
    arena: &mut Vec<Node>,
    theme: &Theme,
    custom: &CustomUtilities,
    limits: &Limits,
    depth: usize,
    expansion: &mut Expansion<'_>,
) -> Result<(), CompileError> {
    super::check_limit(depth, limits.ast_depth, "ast_depth")?;
    let mut children = Vec::new();
    for child in std::mem::take(&mut arena[index].children) {
        if let AstNode::AtRule { name, params, .. } = &arena[child].value {
            if name == "@apply" {
                let source = arena[child]
                    .value
                    .source()
                    .and_then(|source| source.public.clone());
                let nodes = expand(params, arena[child].value.source(), theme, custom, limits)?;
                expansion.bytes = expansion.bytes.saturating_add(ast::to_css(&nodes).len());
                super::check_limit(expansion.bytes, limits.css_bytes, "css_bytes")?;
                // Revisit replacement nodes, including @apply introduced by custom variant slots.
                let replacement = insert(
                    AstNode::Rule {
                        origin: None,
                        source: None,
                        destination: None,
                        selector: "&".into(),
                        nodes,
                    },
                    arena,
                    custom,
                );
                replace(
                    replacement,
                    arena,
                    theme,
                    custom,
                    limits,
                    depth + 1,
                    expansion,
                )?;
                if let Some(source) = source {
                    expansion.replacements.push(Replacement {
                        source,
                        nodes: arena[replacement]
                            .children
                            .iter()
                            .map(|&child| restore(child, arena, false))
                            .collect(),
                    });
                }
                children.extend(std::mem::take(&mut arena[replacement].children));
                continue;
            }
        }
        replace(child, arena, theme, custom, limits, depth, expansion)?;
        children.push(child);
    }
    arena[index].children = children;
    Ok(())
}

pub(super) fn substitute(
    nodes: &mut Vec<AstNode>,
    theme: &Theme,
    custom: &mut CustomUtilities,
    limits: &Limits,
    replacements: &mut Vec<Replacement>,
) -> Result<(), CompileError> {
    fn has_apply(nodes: &[AstNode]) -> bool {
        nodes.iter().any(|node| {
            matches!(node, AstNode::AtRule { name, .. } if name == "@apply")
                || node.children().is_some_and(has_apply)
        })
    }
    if has_apply(&theme.keyframes) {
        return Err(syntax("You cannot use `@apply` inside `@keyframes`."));
    }
    let mut arena = Vec::new();
    let root = insert(
        AstNode::Rule {
            origin: None,
            source: None,
            destination: None,
            selector: "&".into(),
            nodes: std::mem::take(nodes),
        },
        &mut arena,
        custom,
    );
    let mut graph = Graph {
        parents: Vec::new(),
        dependencies: BTreeMap::new(),
        definitions: BTreeMap::new(),
    };
    graph.collect(root, &arena, &mut Vec::new(), theme, custom, limits)?;
    let mut expansion = Expansion {
        bytes: 0,
        replacements,
    };
    for index in graph.sort(&arena, theme, custom, limits)? {
        replace(index, &mut arena, theme, custom, limits, 0, &mut expansion)?;
        if let Some(definition) = arena[index].definition {
            let children = arena[index]
                .children
                .iter()
                .map(|&child| restore(child, &arena, false))
                .collect();
            custom.replace_definition(definition, children);
        }
    }
    custom.recount(limits)?;
    *nodes = arena[root]
        .children
        .iter()
        .filter(|&&index| arena[index].definition.is_none())
        .map(|&index| restore(index, &arena, true))
        .collect();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{Compiler, CompilerInput};

    fn limits() -> Limits {
        Limits {
            input_bytes: 65536,
            asset_bytes: 65536,
            candidate_count: 1024,
            ast_depth: 32,
            css_bytes: 262144,
            queued_bytes: 65536,
            cache_bytes: 1048576,
        }
    }

    #[test]
    fn apply_errors_keep_original_non_scalar_class_and_variant_units() {
        let opaque = Text::from_units(vec![0xd800, 0xfffd, 92, 117, 100, 56, 48, 48]);
        let raw = concat_text(&[&"missing-".into(), &opaque]);
        let theme = Theme::default();
        let custom = CustomUtilities::default();
        let error = invalid(&raw, &theme, &custom, &limits()).unwrap();
        let CompileError::Syntax { message, .. } = error else {
            panic!("expected syntax error")
        };
        assert_eq!(message, concat_text(&[&"Cannot apply unknown utility class `".into(), &raw, &"`. Are you using CSS modules or similar and missing `@reference`? https://tailwindcss.com/docs/functions-and-directives#reference-directive".into()]));
        let raw = concat_text(&[&opaque, &":block".into()]);
        let error = invalid(&raw, &theme, &custom, &limits()).unwrap();
        let CompileError::Syntax { message, .. } = error else {
            panic!("expected variant error")
        };
        assert_eq!(
            message,
            concat_text(&[
                &"Cannot apply utility class `".into(),
                &raw,
                &"` because the `".into(),
                &opaque,
                &"` variant does not exist.".into()
            ])
        );
    }

    #[test]
    fn apply_last_duplicate_offset_counts_original_utf16_units() {
        use crate::compiler::public_api::{Source, SourceLocation};
        use std::sync::Arc;
        let params = Text::from("block\u{feff}block");
        let source = SourceLocation {
            public: None,
            source: Arc::new(Source {
                file: Some("input.css".into()),
                code: params.clone(),
            }),
            start: 10,
            end: 40,
        };
        let nodes = expand(
            &params,
            Some(&source),
            &Theme::default(),
            &CustomUtilities::default(),
            &limits(),
        )
        .unwrap();
        let location = nodes[0].source().unwrap();
        assert_eq!((location.start, location.end), (23, 28));
    }

    #[test]
    fn apply_non_scalar_candidate_source_end_counts_units() {
        use crate::compiler::public_api::{Source, SourceLocation};
        use std::sync::Arc;
        let params = Text::from_units(
            "[--probe:"
                .encode_utf16()
                .chain([0xd800])
                .chain("]".encode_utf16())
                .collect(),
        );
        let source = SourceLocation {
            public: None,
            source: Arc::new(Source {
                file: None,
                code: params.clone(),
            }),
            start: 10,
            end: 40,
        };
        let nodes = expand(
            &params,
            Some(&source),
            &Theme::default(),
            &CustomUtilities::default(),
            &limits(),
        )
        .unwrap();
        let location = nodes[0].source().unwrap();
        assert_eq!(
            (location.start, location.end),
            (17, 17 + params.units().len())
        );
    }

    #[test]
    fn authored_at_rules_cannot_impersonate_internal_utility_definitions() {
        let mut compiler = Compiler::new(
            CompilerInput {
                entry: "input.css".into(),
                css: "@rustify-utility 9999 { width:13px; } @tailwind utilities;".into(),
            },
            vec![],
            limits(),
        )
        .unwrap();
        let prepared = compiler.prepare_add(&[]).unwrap();
        assert_eq!(
            prepared.report().css,
            "@rustify-utility 9999 {\n  width: 13px;\n}\n"
        );
    }

    #[test]
    fn total_application_expansion_is_bounded_before_final_optimization() {
        let mut limits = limits();
        limits.css_bytes = 50;
        let result = Compiler::new(CompilerInput { entry: "input.css".into(),
            css: ".probe { @apply w-[13px]; @apply w-[13px]; @apply w-[13px]; @apply w-[13px]; @apply w-[13px]; }".into() }, vec![], limits);
        assert!(
            matches!(result, Err(CompileError::Limit { resource, limit: 50 }) if resource == "css_bytes")
        );
    }

    #[test]
    fn inline_candidates_and_exclusions_are_charged_to_cache_budget() {
        let mut limits = limits();
        limits.cache_bytes = 1;
        let result = Compiler::new(
            CompilerInput {
                entry: "input.css".into(),
                css: "@source not inline(\"{flex,block,underline}\"); @tailwind utilities;".into(),
            },
            vec![],
            limits,
        );
        assert!(
            matches!(result, Err(CompileError::Limit { resource, limit: 1 }) if resource == "cache_bytes")
        );
    }

    #[test]
    fn inline_brace_expansion_respects_count_before_emitting_candidates() {
        let mut limits = limits();
        limits.candidate_count = 10;
        let result = Compiler::new(
            CompilerInput {
                entry: "input.css".into(),
                css: "@source inline(\"w-{1..1000000}\"); @tailwind utilities;".into(),
            },
            vec![],
            limits,
        );
        assert!(
            matches!(result, Err(CompileError::Limit { resource, limit: 10 }) if resource == "candidate_count")
        );
    }

    #[test]
    fn generated_application_cycles_stop_at_the_callers_depth_limit() {
        let mut limits = limits();
        limits.ast_depth = 8;
        let result = Compiler::new(CompilerInput { entry: "input.css".into(),
            css: "@custom-variant recurse { @apply recurse:block; @slot; } .probe { @apply recurse:block; } @tailwind utilities;".into() }, vec![], limits);
        assert!(
            matches!(result, Err(CompileError::Limit { resource, limit: 8 }) if resource == "ast_depth")
        );
    }
}
