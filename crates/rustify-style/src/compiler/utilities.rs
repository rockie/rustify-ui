//! Rust utility emitters construct both CSS declarations and renderer values.

use super::{
    ast::AstNode,
    candidate::{Candidate, CandidateKind},
    program::{GpuCondition, GpuDeclaration, GpuProgram, GpuValue},
    registry,
    theme::Theme,
    CandidateDiagnostic, DiagnosticKind,
};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct CompiledCandidate {
    pub node: AstNode,
    pub program: GpuProgram,
    pub used_variables: BTreeSet<crate::text::Text>,
    pub property_order: Vec<usize>,
    pub declaration_count: usize,
    pub variant_order: Vec<super::variants::SortKey>,
    pub globals: Vec<AstNode>,
}

#[derive(Debug)]
pub(super) enum FamilyFailure {
    Candidate(DiagnosticKind),
    Resource(super::CompileError),
}

impl From<DiagnosticKind> for FamilyFailure {
    fn from(kind: DiagnosticKind) -> Self {
        Self::Candidate(kind)
    }
}
impl From<super::CompileError> for FamilyFailure {
    fn from(error: super::CompileError) -> Self {
        Self::Resource(error)
    }
}

fn diagnostic(
    candidate: &Candidate,
    failure: impl Into<FamilyFailure>,
    family: impl Into<crate::text::Text>,
) -> CompileFailure {
    match failure.into() {
        FamilyFailure::Candidate(kind) => CompileFailure::Candidate(CandidateDiagnostic {
            candidate: candidate.raw.clone(),
            kind,
            family: Some(family.into()),
        }),
        FamilyFailure::Resource(error) => CompileFailure::Resource(error),
    }
}

pub(super) enum CompileFailure {
    Candidate(CandidateDiagnostic),
    Resource(super::CompileError),
}

fn function_failure(
    error: super::CompileError,
    candidate: &Candidate,
    family: &str,
) -> CompileFailure {
    match error {
        super::CompileError::Syntax { .. } | super::CompileError::RangeError { .. } => {
            diagnostic(candidate, DiagnosticKind::InvalidValue, family)
        }
        _ => CompileFailure::Resource(error),
    }
}

fn variant_failure(error: super::variants::Failure, candidate: &Candidate) -> CompileFailure {
    match error {
        super::variants::Failure::Candidate(kind, root) => diagnostic(candidate, kind, &root),
        super::variants::Failure::Resource(error) => CompileFailure::Resource(error),
    }
}

// Internal utility roots are registered ASCII names; opaque candidate payloads
// never use this boundary. Public objects with unknown roots simply do not match.
fn registry_root(root: &crate::text::Text) -> Option<String> {
    root.units()
        .iter()
        .all(|unit| *unit < 128)
        .then(|| root.to_utf8().ok())
        .flatten()
}

pub fn compile(
    candidate: &Candidate,
    theme: &Theme,
    limits: &super::Limits,
) -> Result<CompiledCandidate, CompileFailure> {
    let max_depth = limits.ast_depth;
    let mut used = BTreeSet::new();
    let mut declarations: Vec<(crate::text::Text, crate::text::Text)> = Vec::new();
    let mut globals = Vec::new();
    let mut static_output = None;
    let mut structural_nodes = None;
    let mut nested_structural = false;
    let mut functional_order = None;
    let mut color_selector = None;
    let family;
    let root_scalar;
    match &candidate.kind {
        CandidateKind::Static { root } => {
            root_scalar = registry_root(root)
                .ok_or_else(|| diagnostic(candidate, DiagnosticKind::UnknownUtility, root))?;
            let root = root_scalar.as_str();
            family = root;
            if let Some(output) = super::motion::compile(root, None, None, theme, &mut used)
                .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                declarations = output.declarations;
                globals = output.globals;
            } else if let Some(output) = super::grid::compile(root, None, None, theme, &mut used)
                .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                declarations = output.declarations;
            } else if let Some(mut output) =
                super::structural_spacing::compile(root, None, None, theme, &mut used)
                    .map_err(|kind| diagnostic(candidate, kind, family))?
            {
                functional_order = Some((space_sort(root), output.declaration_count));
                for node in &output.nodes {
                    if let AstNode::Declaration {
                        property, value, ..
                    } = node
                    {
                        declarations.push((property.clone(), value.clone()));
                    }
                }
                if candidate.important {
                    mark_important(&mut output.nodes);
                }
                nested_structural = output.nodes.iter().any(|node| node.children().is_some());
                structural_nodes = Some(output.nodes);
                globals = output.globals;
            } else {
                let output =
                    super::static_utilities::compile(root, candidate.important, theme, &mut used)
                        .map_err(CompileFailure::Resource)?
                        .ok_or_else(|| {
                            diagnostic(candidate, DiagnosticKind::PendingFamily, root)
                        })?;
                for node in &output.nodes {
                    if let AstNode::Declaration {
                        property, value, ..
                    } = node
                    {
                        declarations.push((property.clone(), value.clone()));
                    }
                }
                globals = output.globals.clone();
                static_output = Some(output);
            }
        }
        CandidateKind::ArbitraryProperty {
            property,
            value,
            modifier,
        } => {
            family = "arbitrary_property";
            let value = super::colors::as_color(value.clone(), modifier.as_ref(), theme, &mut used)
                .map_err(|error| diagnostic(candidate, error, family))?
                .ok_or_else(|| diagnostic(candidate, DiagnosticKind::InvalidValue, family))?;
            declarations.push((property.clone(), value));
        }
        CandidateKind::Functional {
            root,
            value,
            modifier,
        } => {
            root_scalar = registry_root(root)
                .ok_or_else(|| diagnostic(candidate, DiagnosticKind::UnknownUtility, root))?;
            let root = root_scalar.as_str();
            family = root;
            if let Some(output) =
                super::motion::compile(root, value.as_ref(), modifier.as_ref(), theme, &mut used)
                    .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                declarations = output.declarations;
                globals = output.globals;
            } else if let Some(output) =
                super::grid::compile(root, value.as_ref(), modifier.as_ref(), theme, &mut used)
                    .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                declarations = output.declarations;
            } else if let Some(output) =
                super::layout::compile(root, value.as_ref(), modifier.as_ref(), theme, &mut used)
                    .map_err(|kind| diagnostic(candidate, kind, family))?
            {
                functional_order = Some((None, output.declaration_count));
                declarations = output.declarations;
            } else if let Some(mut output) =
                super::masks::compile(root, value.as_ref(), modifier.as_ref(), theme, &mut used)
                    .map_err(|kind| diagnostic(candidate, kind, family))?
            {
                functional_order = Some((None, output.declaration_count));
                for node in &output.nodes {
                    if let AstNode::Declaration {
                        property, value, ..
                    } = node
                    {
                        declarations.push((property.clone(), value.clone()));
                    }
                }
                if candidate.important {
                    mark_important(&mut output.nodes);
                }
                nested_structural = output.nodes.iter().any(|node| node.children().is_some());
                structural_nodes = Some(output.nodes);
                globals = output.globals;
            } else if let Some(mut output) = super::miscellaneous::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
            )
            .map_err(|kind| diagnostic(candidate, kind, family))?
            {
                functional_order = Some((None, output.declaration_count));
                for node in &output.nodes {
                    if let AstNode::Declaration {
                        property, value, ..
                    } = node
                    {
                        declarations.push((property.clone(), value.clone()));
                    }
                }
                if candidate.important {
                    mark_important(&mut output.nodes);
                }
                nested_structural = output.nodes.iter().any(|node| node.children().is_some());
                structural_nodes = Some(output.nodes);
                globals = output.globals;
            } else if let Some(mut output) = super::structural_spacing::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
            )
            .map_err(|kind| diagnostic(candidate, kind, family))?
            {
                functional_order = Some((space_sort(root), output.declaration_count));
                for node in &output.nodes {
                    if let AstNode::Declaration {
                        property, value, ..
                    } = node
                    {
                        declarations.push((property.clone(), value.clone()));
                    }
                }
                if candidate.important {
                    mark_important(&mut output.nodes);
                }
                nested_structural = output.nodes.iter().any(|node| node.children().is_some());
                structural_nodes = Some(output.nodes);
                globals = output.globals;
            } else if let Some(output) = super::functional::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
            )
            .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((output.sort_property, output.declaration_count));
                declarations = output.declarations;
            } else if let Some(mut output) = super::shadows::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
                limits.css_bytes,
            )
            .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                for node in &output.nodes {
                    if let AstNode::Declaration {
                        property, value, ..
                    } = node
                    {
                        declarations.push((property.clone(), value.clone()));
                    }
                }
                if candidate.important {
                    mark_important(&mut output.nodes);
                }
                nested_structural = output.nodes.iter().any(|node| node.children().is_some());
                structural_nodes = Some(output.nodes);
                globals = output.globals;
            } else if let Some(effect) = super::effects::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
                limits.css_bytes,
            )
            .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                declarations = effect.declarations;
                globals = effect.globals;
            } else if let Some(output) =
                super::borders::compile(root, value.as_ref(), modifier.as_ref(), theme, &mut used)
                    .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((output.sort_property, output.declaration_count));
                color_selector = output.selector;
                declarations = output.declarations;
                globals = output.globals;
            } else if let Some(output) = super::transforms::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
            )
            .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                declarations = output.declarations;
                globals = output.globals;
            } else if let Some(output) = super::typography::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
            )
            .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                declarations = output.declarations;
                globals = output.globals;
            } else if let Some(mut output) = super::filters::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
                limits.css_bytes,
            )
            .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                for node in &output.nodes {
                    if let AstNode::Declaration {
                        property, value, ..
                    } = node
                    {
                        declarations.push((property.clone(), value.clone()));
                    }
                }
                if candidate.important {
                    mark_important(&mut output.nodes);
                }
                nested_structural = output.nodes.iter().any(|node| node.children().is_some());
                structural_nodes = Some(output.nodes);
                globals = output.globals;
            } else if let Some(mut output) =
                super::gradients::compile(root, value.as_ref(), modifier.as_ref(), theme, &mut used)
                    .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((output.sort_property, output.declaration_count));
                for node in &output.nodes {
                    if let AstNode::Declaration {
                        property, value, ..
                    } = node
                    {
                        declarations.push((property.clone(), value.clone()));
                    }
                }
                if candidate.important {
                    mark_important(&mut output.nodes);
                }
                nested_structural = output.nodes.iter().any(|node| node.children().is_some());
                structural_nodes = Some(output.nodes);
                globals = output.globals;
            } else if let Some(output) = super::backgrounds::compile(
                root,
                value.as_ref(),
                modifier.as_ref(),
                theme,
                &mut used,
            )
            .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((None, output.declaration_count));
                declarations = output.declarations;
            } else if let Some(output) =
                super::colors::compile(root, value.as_ref(), modifier.as_ref(), theme, &mut used)
                    .map_err(|kind| diagnostic(candidate, kind, root))?
            {
                functional_order = Some((output.sort_property, output.declaration_count));
                color_selector = output.selector;
                declarations = output.declarations;
            } else {
                return Err(diagnostic(candidate, DiagnosticKind::PendingFamily, family));
            }
        }
    }
    for (_, value) in &mut declarations {
        *value = super::functions::in_text_value(value, theme, false, max_depth, limits.css_bytes)
            .map_err(|error| function_failure(error, candidate, family))?;
    }
    let mut property_order: Vec<usize> = declarations
        .iter()
        .filter_map(|(property, value)| {
            let property = if property == "--tw-sort" {
                value
            } else {
                property
            };
            registry::PROPERTY_ORDER
                .iter()
                .position(|item| property == *item)
        })
        .collect();
    property_order.sort_unstable();
    property_order.dedup();
    let mut non_scalar = false;
    let gpu_declarations = declarations
        .iter()
        .filter_map(|(property, value)| {
            let (Ok(property), Ok(value)) = (property.to_utf8(), value.to_utf8()) else {
                non_scalar = true;
                return None;
            };
            Some(gpu_declaration(
                &property,
                &value,
                candidate.important,
                max_depth,
            ))
        })
        .collect();
    let mut count = declarations.len();
    if let Some((sort_property, declaration_count)) = functional_order {
        count = declaration_count;
        if let Some(index) = sort_property.and_then(|property| {
            registry::PROPERTY_ORDER
                .iter()
                .position(|item| *item == property)
        }) {
            property_order = vec![index];
        }
    }
    let mut nodes: Vec<_> = declarations
        .into_iter()
        .map(|(property, value)| AstNode::Declaration {
            origin: None,
            source: None,
            destination: None,
            property,
            value,
            important: candidate.important,
            identity: Default::default(),
        })
        .collect();
    if matches!(functional_order, Some((Some("size"), _))) {
        nodes.insert(
            0,
            AstNode::Declaration {
                origin: None,
                source: None,
                destination: None,
                property: "--tw-sort".into(),
                value: "size".into(),
                important: candidate.important,
                identity: Default::default(),
            },
        );
    }
    if let Some(output) = static_output {
        nodes = output.nodes;
        property_order = output.property_order;
        count = output.declaration_count;
    }
    if let Some(output) = structural_nodes {
        nodes = output;
    }
    let mut conditions = Vec::new();
    if non_scalar {
        conditions.push(GpuCondition::Unsupported {
            feature: "non-scalar UTF-16 GPU declaration".into(),
        });
    }
    if nested_structural {
        conditions.push(GpuCondition::Unsupported {
            feature: "nested functional utility branches".into(),
        });
    }
    if let Some(selector) = color_selector {
        nodes = vec![AstNode::Rule {
            origin: None,
            source: None,
            destination: None,
            selector: selector.into(),
            nodes,
        }];
        conditions.push(GpuCondition::Unsupported {
            feature: format!("color target {selector}"),
        });
    }
    if matches!(candidate.kind, CandidateKind::Static { .. })
        && nodes.iter().any(|node| node.children().is_some())
    {
        conditions.push(GpuCondition::Unsupported {
            feature: "nested static utility branches".into(),
        });
    }
    if !globals.is_empty() {
        nodes.insert(
            0,
            AstNode::AtRoot {
                origin: None,
                nodes: std::mem::take(&mut globals),
            },
        );
    }
    let output = CompiledCandidate {
        node: AstNode::Rule {
            origin: None,
            source: None,
            destination: None,
            selector: super::value::concat_text(&[
                &".".into(),
                &super::candidate::escape_text(&candidate.raw),
            ]),
            nodes,
        },
        program: GpuProgram {
            candidate: candidate.raw.clone(),
            conditions,
            declarations: gpu_declarations,
            cascade_order: 0,
        },
        used_variables: used,
        property_order,
        declaration_count: count,
        variant_order: Vec::new(),
        globals,
    };
    finalize(candidate, output, theme, limits, family)
}

fn gpu_declaration(
    property: &str,
    value: &str,
    important: bool,
    max_depth: usize,
) -> GpuDeclaration {
    GpuDeclaration {
        property: property.into(),
        value: if property.ends_with("color") || matches!(property, "fill" | "stroke") {
            GpuValue::Color {
                value: value.into(),
            }
        } else if let Some(number) = value.strip_suffix("px").filter(|number| {
            !number.is_empty()
                && number
                    .bytes()
                    .all(|byte| b"0123456789.+-eE".contains(&byte))
                && number.parse::<f64>().is_ok_and(f64::is_finite)
        }) {
            GpuValue::Pixels {
                value: number.into(),
            }
        } else if super::value::parse(value, max_depth).is_ok_and(|nodes| {
            nodes.len() > 1
                || nodes
                    .iter()
                    .any(|node| matches!(node, super::value::ValueNode::Function { .. }))
        }) {
            GpuValue::CssExpression {
                value: value.into(),
            }
        } else {
            GpuValue::Keyword {
                value: value.into(),
            }
        },
        important,
    }
}

fn finalize(
    candidate: &Candidate,
    mut output: CompiledCandidate,
    theme: &Theme,
    limits: &super::Limits,
    family: &str,
) -> Result<CompiledCandidate, CompileFailure> {
    let mut nodes = std::mem::take(output.node.children_mut().unwrap());
    for variant in &candidate.variants {
        let variant =
            super::variants::compile_variant(variant, nodes, &mut output.globals, theme, limits)
                .map_err(|error| variant_failure(error, candidate))?;
        nodes = variant.nodes;
        output.program.conditions.push(variant.condition);
        output.variant_order.push(variant.order);
    }
    output.variant_order.sort_unstable();
    output.variant_order.dedup();
    super::functions::substitute(&mut nodes, theme, limits.ast_depth, limits.css_bytes)
        .map_err(|error| function_failure(error, candidate, family))?;
    super::functions::substitute(
        &mut output.globals,
        theme,
        limits.ast_depth,
        limits.css_bytes,
    )
    .map_err(|error| function_failure(error, candidate, family))?;
    output.used_variables.clear();
    super::custom_utilities::collect_used(&nodes, &mut output.used_variables, limits.ast_depth)
        .map_err(|error| function_failure(error, candidate, family))?;
    super::custom_utilities::collect_used(
        &output.globals,
        &mut output.used_variables,
        limits.ast_depth,
    )
    .map_err(|error| function_failure(error, candidate, family))?;
    // compileAst exposes these cached children before optimization. Rendering
    // owns pruning; it shares declaration identities with this raw tree.
    *output.node.children_mut().unwrap() = nodes;
    Ok(output)
}

fn compile_custom(
    candidate: &Candidate,
    mut nodes: Vec<AstNode>,
    used: BTreeSet<crate::text::Text>,
    theme: &Theme,
    limits: &super::Limits,
) -> Result<CompiledCandidate, CompileFailure> {
    use std::collections::VecDeque;
    let mut order = BTreeSet::new();
    let mut count = 0;
    let mut sort_override = false;
    let mut queue = nodes.iter().collect::<VecDeque<_>>();
    while let Some(node) = queue.pop_front() {
        if let AstNode::Declaration {
            property, value, ..
        } = node
        {
            count += 1;
            if sort_override {
                continue;
            }
            if property == "--tw-sort" {
                if let Some(index) = registry::PROPERTY_ORDER
                    .iter()
                    .position(|item| value == *item)
                {
                    order.insert(index);
                    sort_override = true;
                    continue;
                }
            }
            if let Some(index) = registry::PROPERTY_ORDER
                .iter()
                .position(|item| property == *item)
            {
                order.insert(index);
            }
        } else if matches!(node, AstNode::Rule { .. } | AstNode::AtRule { .. }) {
            if let Some(children) = node.children() {
                queue.extend(children);
            }
        }
    }
    let mut declarations = Vec::new();
    let mut non_scalar = false;
    for node in &nodes {
        if let AstNode::Declaration {
            property,
            value,
            important,
            ..
        } = node
        {
            if property == "--tw-sort" {
                continue;
            }
            let value = super::functions::in_text_value(
                value,
                theme,
                false,
                limits.ast_depth,
                limits.css_bytes,
            )
            .map_err(|error| function_failure(error, candidate, "custom_utility"))?;
            let (Ok(property), Ok(value)) = (property.to_utf8(), value.to_utf8()) else {
                non_scalar = true;
                continue;
            };
            declarations.push(gpu_declaration(
                &property,
                &value,
                candidate.important || *important,
                limits.ast_depth,
            ));
        }
    }
    let mut conditions = Vec::new();
    if non_scalar {
        conditions.push(GpuCondition::Unsupported {
            feature: "non-scalar UTF-16 GPU declaration".into(),
        });
    }
    // Applied utilities may carry global resources without changing their target.
    if nodes
        .iter()
        .any(|node| !matches!(node, AstNode::AtRoot { .. }) && node.children().is_some())
    {
        conditions.push(GpuCondition::Unsupported {
            feature: "nested custom utility branches".into(),
        });
    }
    if candidate.important {
        mark_important(&mut nodes);
    }
    let output = CompiledCandidate {
        node: AstNode::Rule {
            origin: None,
            source: None,
            destination: None,
            selector: super::value::concat_text(&[
                &".".into(),
                &super::candidate::escape_text(&candidate.raw),
            ]),
            nodes,
        },
        program: GpuProgram {
            candidate: candidate.raw.clone(),
            conditions,
            declarations,
            cascade_order: 0,
        },
        used_variables: used,
        property_order: order.into_iter().collect(),
        declaration_count: count,
        variant_order: Vec::new(),
        globals: Vec::new(),
    };
    finalize(candidate, output, theme, limits, "custom_utility")
}

pub fn compile_all(
    candidate: &Candidate,
    theme: &Theme,
    limits: &super::Limits,
    custom: &super::custom_utilities::CustomUtilities,
) -> Result<Vec<CompiledCandidate>, CompileFailure> {
    let builtin = match &candidate.kind {
        CandidateKind::Static { root } => registry_root(root)
            .is_some_and(|root| registry::STATIC_ROOTS.binary_search(&root.as_str()).is_ok()),
        CandidateKind::Functional { root, .. } => registry_root(root).is_some_and(|root| {
            registry::FUNCTIONAL_ROOTS
                .binary_search(&root.as_str())
                .is_ok()
        }),
        CandidateKind::ArbitraryProperty { .. } => true,
    };
    let mut outputs = Vec::new();
    let mut failure = None;
    if builtin {
        match compile(candidate, theme, limits) {
            Ok(output) => outputs.push(output),
            Err(error @ CompileFailure::Resource(_)) => return Err(error),
            Err(error) => failure = Some(error),
        }
    }
    let mut used = BTreeSet::new();
    let custom = custom
        .compile(candidate, theme, &mut used, limits)
        .map_err(|error| function_failure(error, candidate, "custom_utility"))?;
    for nodes in custom.outputs {
        outputs.push(compile_custom(
            candidate,
            nodes,
            used.clone(),
            theme,
            limits,
        )?);
    }
    if outputs.is_empty() {
        return Err(failure.unwrap_or_else(|| {
            diagnostic(candidate, DiagnosticKind::InvalidValue, "custom_utility")
        }));
    }
    Ok(outputs)
}

fn space_sort(root: &str) -> Option<&'static str> {
    match root {
        "space-x" | "-space-x" | "space-x-px" | "-space-x-px" | "space-x-reverse" => {
            Some("row-gap")
        }
        "space-y" | "-space-y" | "space-y-px" | "-space-y-px" | "space-y-reverse" => {
            Some("column-gap")
        }
        _ => None,
    }
}

fn mark_important(nodes: &mut [AstNode]) {
    for node in nodes {
        if matches!(node, AstNode::AtRoot { .. }) {
            continue;
        }
        if let AstNode::Declaration { important, .. } = node {
            *important = true;
        }
        if let Some(children) = node.children_mut() {
            mark_important(children);
        }
    }
}
