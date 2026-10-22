//! CSS-native custom utility registration and value/modifier substitution.

use super::value::concat_text;
use super::{
    ast::AstNode,
    candidate::{Candidate, CandidateKind, CandidateValue},
    colors::{canonical_nonnegative_text, is_color_text, is_number_text},
    theme::{self, Theme},
    value::{self, ValueNode},
    CompileError, Limits,
};
use crate::text::Text;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UtilityKind {
    Static,
    Functional,
}

pub(super) struct RegisteredDefinition {
    pub name: String,
    pub functional: bool,
    pub nodes: Vec<AstNode>,
    pub origin: Option<super::ast::NodeOrigin>,
    pub completion: CompletionMetadata,
}

pub(super) fn marker(index: usize) -> AstNode {
    AstNode::AtRoot {
        origin: None,
        nodes: vec![AstNode::Comment {
            origin: None,
            source: None,
            destination: None,
            value: format!("utility definition {index}").into(),
        }],
    }
}

#[derive(Clone, Debug)]
struct Definition {
    name: String,
    nodes: Vec<AstNode>,
    completion: CompletionMetadata,
    origin: Option<super::ast::NodeOrigin>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct CompletionValues {
    pub literals: Vec<Text>,
    pub namespaces: Vec<Text>,
    pub number: bool,
    pub integer: bool,
}

#[derive(Clone, Debug, Default)]
pub(super) struct CompletionMetadata {
    pub values: CompletionValues,
    pub modifiers: CompletionValues,
}

impl CompletionMetadata {
    pub(super) fn include_value(&mut self, input: &Text, depth: usize) -> Result<(), CompileError> {
        fn visit(
            values: &[value::ValueNode],
            spacing: bool,
            storage: &mut CompletionMetadata,
            depth: usize,
        ) -> Result<(), CompileError> {
            for value in values {
                if let value::ValueNode::Function { name, nodes } = value {
                    if name == "--value" || name == "--modifier" {
                        let target = if name == "--modifier" {
                            &mut storage.modifiers
                        } else {
                            &mut storage.values
                        };
                        for raw in value::segment_text(&value::to_css_text(nodes), 44, depth)? {
                            let raw = raw.replace_ascii_sequence("\\*", "*").trim_js();
                            if spacing {
                                target.number |= raw == "number";
                                target.integer |= raw == "integer";
                            } else if (raw.starts_with("'") && raw.ends_with("'"))
                                || (raw.starts_with("\"") && raw.ends_with("\""))
                            {
                                let literal = Text::from_units(
                                    raw.units()[1..raw.units().len() - 1].to_vec(),
                                );
                                if !target.literals.contains(&literal) {
                                    target.literals.push(literal);
                                }
                            } else if raw.starts_with("--") {
                                let end = raw.find("-*").unwrap_or_else(|| {
                                    raw.units()
                                        .iter()
                                        .position(|unit| js_whitespace(*unit))
                                        .unwrap_or(raw.units().len())
                                });
                                let namespace = Text::from_units(raw.units()[..end].to_vec());
                                if !target.namespaces.contains(&namespace) {
                                    target.namespaces.push(namespace);
                                }
                            }
                        }
                    }
                    visit(nodes, spacing || name == "--spacing", storage, depth)?;
                }
            }
            Ok(())
        }
        visit(&value::parse_text(input, depth)?, false, self, depth)
    }

    #[cfg(test)]
    fn extract(nodes: &[AstNode], depth: usize) -> Result<Self, CompileError> {
        fn collect(
            nodes: &[AstNode],
            storage: &mut CompletionMetadata,
            depth: usize,
        ) -> Result<(), CompileError> {
            for node in nodes {
                if let AstNode::Declaration { value, .. } = node {
                    storage.include_value(value, depth)?;
                }
                if let Some(nodes) = node.children() {
                    collect(nodes, storage, depth)?;
                }
            }
            Ok(())
        }
        let mut storage = Self::default();
        collect(nodes, &mut storage, depth)?;
        Ok(storage)
    }

    fn owned_bytes(&self) -> usize {
        [&self.values, &self.modifiers]
            .iter()
            .fold(0usize, |total, values| {
                total.saturating_add([&values.literals, &values.namespaces].iter().fold(
                    0usize,
                    |total, strings| {
                        strings.iter().fold(
                            total.saturating_add(
                                strings
                                    .capacity()
                                    .saturating_mul(std::mem::size_of::<Text>()),
                            ),
                            |total, value| total.saturating_add(value.owned_bytes()),
                        )
                    },
                ))
            })
    }
}

#[derive(Clone, Debug, Default)]
pub struct CustomUtilities {
    definitions: Vec<Definition>,
    static_roots: BTreeMap<String, Vec<usize>>,
    functional_roots: BTreeMap<String, Vec<usize>>,
    owned_bytes: usize,
    warnings: Vec<Text>,
}

impl CustomUtilities {
    /// A stylesheet without `@utility` rules.
    pub(super) fn empty() -> &'static Self {
        static EMPTY: CustomUtilities = CustomUtilities {
            definitions: Vec::new(),
            static_roots: BTreeMap::new(),
            functional_roots: BTreeMap::new(),
            owned_bytes: 0,
            warnings: Vec::new(),
        };
        &EMPTY
    }
}

#[derive(Debug, Default)]
pub struct CustomCompile {
    pub outputs: Vec<Vec<AstNode>>,
    pub stop: bool,
}

fn syntax(message: String) -> CompileError {
    CompileError::Syntax {
        offset: 0,
        message: message.into(),
    }
}

pub(super) fn validate_definition(
    params: &Text,
    has_body: bool,
    nested: bool,
) -> Result<(String, bool), CompileError> {
    if nested {
        return Err(syntax("`@utility` cannot be nested.".into()));
    }
    let message = |suffix: &str| {
        let mut message = crate::text::Text::from("`@utility ");
        message.push_text(params);
        message.push_str(suffix);
        CompileError::Syntax { offset: 0, message }
    };
    if !has_body {
        return Err(message(
            "` is empty. Utilities should include at least one property.",
        ));
    }
    let functional = params.ends_with("-*");
    let units = params.units();
    let root = if functional {
        &units[..units.len() - 2]
    } else {
        units
    };
    let valid = {
        let name = root.strip_prefix(&[45]).unwrap_or(root);
        name.first().is_some_and(|unit| (97..=122).contains(unit))
            && name.iter().all(|unit| {
                (48..=57).contains(unit)
                    || (65..=90).contains(unit)
                    || (97..=122).contains(unit)
                    || [47, 37, 46, 95, 45].contains(unit)
            })
    };
    if !valid {
        return Err(message(if !functional && params.ends_with("*") {
            "` defines an invalid utility name. A functional utility must end in `-*`."
        } else if !functional && params.contains("*") {
            "` defines an invalid utility name. The dynamic portion marked by `-*` must appear once at the end."
        } else {
            "` defines an invalid utility name. Utilities should be alphanumeric and start with a lowercase letter."
        }));
    }
    let name: String = params
        .units()
        .iter()
        .map(|unit| char::from(*unit as u8))
        .collect();
    Ok((name, functional))
}

impl CustomUtilities {
    pub(super) fn from_definitions(
        definitions: Vec<RegisteredDefinition>,
        warnings: Vec<Text>,
        limits: &Limits,
    ) -> Result<Self, CompileError> {
        if definitions.is_empty() {
            return Ok(Self::default());
        }
        let mut custom = Self {
            warnings,
            ..Self::default()
        };
        for RegisteredDefinition {
            name,
            functional,
            nodes,
            origin,
            completion,
        } in definitions
        {
            let root = if functional {
                &name[..name.len() - 2]
            } else {
                &name
            };
            let roots = if functional {
                &mut custom.functional_roots
            } else {
                &mut custom.static_roots
            };
            roots
                .entry(root.into())
                .or_default()
                .push(custom.definitions.len());
            custom.definitions.push(Definition {
                name,
                nodes,
                completion,
                origin,
            });
        }
        custom.owned_bytes = custom.measure_owned_bytes();
        super::check_limit(custom.owned_bytes, limits.cache_bytes, "cache_bytes")?;
        Ok(custom)
    }

    pub(super) fn contains_apply(&self) -> bool {
        self.definitions
            .iter()
            .any(|definition| super::ast::contains_at_rule(&definition.nodes, "@apply"))
    }

    #[cfg(test)]
    pub fn extract(nodes: &mut Vec<AstNode>, limits: &Limits) -> Result<Self, CompileError> {
        Self::extract_inner(nodes, limits)
    }

    #[cfg(test)]
    fn extract_inner(nodes: &mut Vec<AstNode>, limits: &Limits) -> Result<Self, CompileError> {
        fn collect(
            nodes: &mut Vec<AstNode>,
            nested: bool,
            custom: &mut CustomUtilities,
            limits: &Limits,
            edits: &mut super::origin::Edits,
        ) -> Result<(), CompileError> {
            let mut output = Vec::new();
            for mut node in std::mem::take(nodes) {
                let origin = node.origin().cloned();
                if let AstNode::AtRule {
                    name,
                    params,
                    nodes: children,
                    ..
                } = &mut node
                {
                    if name == "@utility" {
                        let (name, functional) = validate_definition(
                            params,
                            children.as_ref().is_some_and(|nodes| !nodes.is_empty()),
                            nested,
                        )?;
                        let children = children.as_mut().unwrap();
                        let root = if functional {
                            &name[..name.len() - 2]
                        } else {
                            &name
                        };
                        if functional {
                            normalize(children, limits.ast_depth, &mut custom.warnings, edits)?;
                        }
                        let completion = if functional {
                            CompletionMetadata::extract(children, limits.ast_depth)?
                        } else {
                            CompletionMetadata::default()
                        };
                        let index = custom.definitions.len();
                        let roots = if functional {
                            &mut custom.functional_roots
                        } else {
                            &mut custom.static_roots
                        };
                        roots.entry(root.into()).or_default().push(index);
                        custom.definitions.push(Definition {
                            name,
                            nodes: std::mem::take(children),
                            completion,
                            origin,
                        });
                        continue;
                    }
                }
                match &mut node {
                    AstNode::Context { nodes, .. } => {
                        collect(nodes, nested, custom, limits, edits)?
                    }
                    _ => {
                        if let Some(children) = node.children_mut() {
                            collect(children, true, custom, limits, edits)?;
                        }
                    }
                }
                output.push(node);
            }
            *nodes = output;
            Ok(())
        }
        let mut custom = Self::default();
        let mut edits = super::origin::Edits::from_nodes(nodes);
        collect(nodes, false, &mut custom, limits, &mut edits)?;
        let snapshot = edits.finish();
        snapshot.bind(nodes);
        for definition in &mut custom.definitions {
            snapshot.bind(&mut definition.nodes);
            if let Some(origin) = &mut definition.origin {
                snapshot.bind_origin(origin);
            }
        }
        custom.owned_bytes = custom.measure_owned_bytes();
        if custom.owned_bytes > limits.cache_bytes {
            return Err(CompileError::Limit {
                resource: "cache_bytes".into(),
                limit: limits.cache_bytes,
            });
        }
        Ok(custom)
    }

    pub(super) fn definition(&self, index: usize) -> (&str, &[AstNode]) {
        let definition = &self.definitions[index];
        (&definition.name, &definition.nodes)
    }

    pub(super) fn definition_index(&self, node: &AstNode) -> Option<usize> {
        let AstNode::AtRoot {
            origin: None,
            nodes,
        } = node
        else {
            return None;
        };
        let [AstNode::Comment { value, .. }] = nodes.as_slice() else {
            return None;
        };
        if !value.starts_with("utility definition ") || value.units().len() <= 19 {
            return None;
        }
        // Caller AtRoot objects have an origin and cannot select private slots.
        value.units()[19..]
            .iter()
            .try_fold(0usize, |index, unit| {
                if !(48..=57).contains(unit) {
                    return None;
                }
                index.checked_mul(10)?.checked_add(usize::from(*unit - 48))
            })
            .filter(|&index| index < self.definitions.len())
    }

    pub(super) fn completion_definitions(
        &self,
    ) -> impl Iterator<Item = (&str, &CompletionMetadata)> {
        self.definitions.iter().filter_map(|definition| {
            definition
                .name
                .strip_suffix("-*")
                .map(|root| (root, &definition.completion))
        })
    }

    pub(super) fn replace_definition(&mut self, index: usize, nodes: Vec<AstNode>) {
        self.definitions[index].nodes = nodes;
    }

    pub(super) fn recount(&mut self, limits: &Limits) -> Result<(), CompileError> {
        self.owned_bytes = self.measure_owned_bytes();
        super::check_limit(self.owned_bytes, limits.cache_bytes, "cache_bytes")
    }

    pub fn roots(&self, kind: UtilityKind) -> impl DoubleEndedIterator<Item = &str> {
        let roots = match kind {
            UtilityKind::Static => &self.static_roots,
            UtilityKind::Functional => &self.functional_roots,
        };
        roots.keys().map(String::as_str)
    }

    pub fn contains_root(&self, kind: UtilityKind, root: &str) -> bool {
        match kind {
            UtilityKind::Static => self.static_roots.contains_key(root),
            UtilityKind::Functional => self.functional_roots.contains_key(root),
        }
    }

    pub fn warnings(&self) -> &[Text] {
        &self.warnings
    }

    pub fn owned_bytes(&self) -> usize {
        self.owned_bytes
    }

    pub fn compile(
        &self,
        candidate: &Candidate,
        theme: &Theme,
        used: &mut BTreeSet<crate::text::Text>,
        limits: &Limits,
    ) -> Result<CustomCompile, CompileError> {
        let registered_root = |root: &Text| {
            root.units().iter().all(|unit| *unit < 128).then(|| {
                root.units()
                    .iter()
                    .map(|unit| char::from(*unit as u8))
                    .collect::<String>()
            })
        };
        let (indices, candidate_value, modifier) = match &candidate.kind {
            CandidateKind::Static { root } => (
                registered_root(root).and_then(|root| self.static_roots.get(&root)),
                None,
                None,
            ),
            CandidateKind::Functional {
                root,
                value,
                modifier,
            } => (
                registered_root(root).and_then(|root| self.functional_roots.get(&root)),
                value.as_ref(),
                modifier.as_ref(),
            ),
            CandidateKind::ArbitraryProperty { .. } => return Ok(CustomCompile::default()),
        };
        let Some(indices) = indices else {
            return Ok(CustomCompile::default());
        };
        let functional = matches!(candidate.kind, CandidateKind::Functional { .. });
        let modifier_value = modifier
            .map(|modifier| {
                if let Some(raw) = modifier
                    .strip_prefix("[")
                    .and_then(|raw| raw.strip_suffix("]"))
                {
                    Ok::<CandidateValue, CompileError>(CandidateValue::Arbitrary {
                        value: value::decode_arbitrary_text(&raw, limits.ast_depth)?,
                        data_type: None,
                    })
                } else if let Some(raw) = modifier
                    .strip_prefix("(")
                    .and_then(|raw| raw.strip_suffix(")"))
                {
                    Ok::<CandidateValue, CompileError>(CandidateValue::Arbitrary {
                        value: value::decode_arbitrary_text(
                            &concat_text(&[&"var(".into(), &raw, &")".into()]),
                            limits.ast_depth,
                        )?,
                        data_type: None,
                    })
                } else {
                    Ok::<CandidateValue, CompileError>(CandidateValue::Named {
                        value: modifier.clone(),
                    })
                }
            })
            .transpose()?;
        let fraction = if matches!(modifier_value, Some(CandidateValue::Named { .. })) {
            if let (Some(CandidateValue::Named { value }), Some(modifier)) =
                (candidate_value, modifier)
            {
                Some(concat_text(&[value, &"/".into(), modifier]))
            } else {
                None
            }
        } else {
            None
        };
        let mut compiled = CustomCompile::default();
        for &index in indices {
            let definition = &self.definitions[index];
            let root = definition.origin.as_ref().map(|origin| {
                if functional {
                    super::origin::CloneRoot::Definition(origin)
                } else {
                    super::origin::CloneRoot::Body(origin)
                }
            });
            let mut nodes =
                super::ast::clone_fresh_checked(&definition.nodes, root, limits.cache_bytes)?;
            if functional {
                let Some(candidate_value) = candidate_value else {
                    continue;
                };
                let mut state = Resolution::default();
                let mut edits = super::origin::Edits::from_nodes(&nodes);
                substitute(
                    &mut nodes,
                    (candidate_value, modifier_value.as_ref(), fraction.as_ref()),
                    theme,
                    limits,
                    &mut state,
                    &mut edits,
                )?;
                if state.used_value && !state.resolved_value
                    || state.used_modifier && !state.resolved_modifier
                    || state.ratio && state.resolved_modifier
                    || modifier.is_some() && !state.ratio && !state.resolved_modifier
                {
                    compiled.stop = true;
                    break;
                }
                if state.ratio {
                    remove_nonratio(&mut nodes, &mut 0, &state.nonratio);
                }
                edits.sync(&nodes);
                edits.finish().bind(&mut nodes);
            }
            collect_used(&nodes, used, limits.ast_depth)?;
            compiled.outputs.push(nodes);
        }
        Ok(compiled)
    }

    fn measure_owned_bytes(&self) -> usize {
        fn nodes_bytes(nodes: &Vec<AstNode>) -> usize {
            nodes
                .capacity()
                .saturating_mul(std::mem::size_of::<AstNode>())
                .saturating_add(
                    nodes
                        .iter()
                        .map(|node| match node {
                            AstNode::AtRoot { origin: _, nodes } => nodes_bytes(nodes),
                            AstNode::Context {
                                base,
                                source_base,
                                nodes,
                                ..
                            } => base
                                .owned_bytes()
                                .saturating_add(
                                    source_base
                                        .as_ref()
                                        .map_or(0, crate::text::Text::owned_bytes),
                                )
                                .saturating_add(nodes_bytes(nodes)),
                            AstNode::Rule {
                                selector, nodes, ..
                            } => selector.owned_bytes().saturating_add(nodes_bytes(nodes)),
                            AstNode::AtRule {
                                name,
                                params,
                                nodes,
                                ..
                            } => name
                                .owned_bytes()
                                .saturating_add(params.owned_bytes())
                                .saturating_add(nodes.as_ref().map_or(0, nodes_bytes)),
                            AstNode::Declaration {
                                property, value, ..
                            } => property
                                .owned_bytes()
                                .saturating_add(value.owned_bytes())
                                .saturating_add(super::ast::DECLARATION_IDENTITY_BYTES),
                            AstNode::Comment { value, .. } => value.owned_bytes(),
                        })
                        .sum::<usize>(),
                )
        }
        fn roots_bytes(roots: &BTreeMap<String, Vec<usize>>) -> usize {
            let slots = roots.len().div_ceil(5).saturating_mul(11);
            slots
                .saturating_mul(std::mem::size_of::<(String, Vec<usize>)>())
                .saturating_add(
                    roots
                        .len()
                        .div_ceil(5)
                        .saturating_mul(16 * std::mem::size_of::<usize>()),
                )
                .saturating_add(
                    roots
                        .iter()
                        .map(|(name, indices)| {
                            name.capacity().saturating_add(
                                indices
                                    .capacity()
                                    .saturating_mul(std::mem::size_of::<usize>()),
                            )
                        })
                        .sum::<usize>(),
                )
        }
        let mut seen = BTreeSet::new();
        let origin_bytes = self
            .definitions
            .iter()
            .filter_map(|definition| definition.origin.as_ref())
            .map(|origin| origin.owned_bytes(&mut seen))
            .fold(0usize, usize::saturating_add);
        self.definitions
            .capacity()
            .saturating_mul(std::mem::size_of::<Definition>())
            .saturating_add(origin_bytes)
            .saturating_add(
                self.definitions
                    .iter()
                    .map(|definition| {
                        definition
                            .name
                            .capacity()
                            .saturating_add(nodes_bytes(&definition.nodes))
                            .saturating_add(definition.completion.owned_bytes())
                    })
                    .sum::<usize>(),
            )
            .saturating_add(roots_bytes(&self.static_roots))
            .saturating_add(roots_bytes(&self.functional_roots))
            .saturating_add(
                self.warnings
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Text>()),
            )
            .saturating_add(self.warnings.iter().map(Text::owned_bytes).sum::<usize>())
    }
}

fn js_whitespace(unit: u16) -> bool {
    matches!(unit, 9..=13 | 32 | 160 | 5760 | 8192..=8202 | 8232 | 8233 | 8239 | 8287 | 12288 | 65279)
}

pub(super) fn normalize_value(
    css: &Text,
    depth: usize,
    warnings: &mut Vec<Text>,
) -> Result<Option<Text>, CompileError> {
    fn argument(raw: &Text) -> Text {
        let raw = raw.replace_ascii_sequence("\\*", "*");
        let units = raw.units();
        let mut output = Vec::with_capacity(units.len());
        let mut index = 0;
        while index < units.len() {
            if js_whitespace(units[index]) {
                while index < units.len() && js_whitespace(units[index]) {
                    index += 1;
                }
                if output.starts_with(&[45, 45]) && units[index..].starts_with(&[45, 45]) {
                    output.extend_from_slice(&[45, 42]);
                }
            } else {
                output.push(units[index]);
                index += 1;
            }
        }
        let mut output = Text::from_units(output);
        while output.contains("-*-*") {
            output = output.replace_ascii_sequence("-*-*", "-*");
        }
        if output.starts_with("--") && !output.contains("-*") {
            output.push_str("-*");
        }
        output
    }
    fn functions(
        nodes: &mut [ValueNode],
        depth: usize,
        warnings: &mut Vec<Text>,
    ) -> Result<(), CompileError> {
        for node in nodes {
            if let ValueNode::Function { name, nodes } = node {
                if name == "--value" || name == "--modifier" {
                    let css = value::to_css_text(nodes);
                    let mut args = Text::default();
                    for (index, arg) in value::segment_text(&css, 44, depth)?.iter().enumerate() {
                        if index > 0 {
                            args.push_str(",");
                        }
                        args.push_text(&argument(arg));
                    }
                    *nodes = value::parse_text(&args, depth)?;
                    for argument in nodes.iter() {
                        let ValueNode::Word { value: data_type } = argument else {
                            continue;
                        };
                        if data_type.starts_with("\"")
                            || data_type.starts_with("'")
                            || data_type.starts_with("--")
                            || data_type.starts_with("[") && data_type.ends_with("]")
                            || ["number", "integer", "ratio", "percentage"]
                                .iter()
                                .any(|kind| data_type == kind)
                        {
                            continue;
                        }
                        warnings.push(concat_text(&[&"Unsupported bare value data type: \"".into(), data_type, &"\".\nOnly valid data types are: \"number\", \"integer\", \"ratio\", \"percentage\".\n".into()]));
                        let function = ValueNode::Function {
                            name: name.clone(),
                            nodes: nodes.clone(),
                        };
                        let mut marked = function.clone();
                        if let ValueNode::Function { nodes, .. } = &mut marked {
                            for node in nodes {
                                if matches!(node, ValueNode::Word { value } if value == data_type) {
                                    *node = ValueNode::Word { value: "¶".into() };
                                }
                            }
                        }
                        let marked_css = value::to_css_text(&[marked]);
                        let offset = marked_css.find("¶").unwrap_or(0);
                        warnings.push(concat_text(&[
                            &"```css\n".into(),
                            &value::to_css_text(&[function]),
                            &"\n".into(),
                            &" ".repeat(offset).into(),
                            &"^".repeat(data_type.units().len()).into(),
                            &"\n```".into(),
                        ]));
                    }
                } else {
                    functions(nodes, depth, warnings)?;
                }
            }
        }
        Ok(())
    }
    if !css.contains("--value(") && !css.contains("--modifier(") {
        return Ok(None);
    }
    let mut parsed = value::parse_text(css, depth)?;
    functions(&mut parsed, depth, warnings)?;
    Ok(Some(value::to_css_text(&parsed)))
}

#[cfg(test)]
fn normalize(
    nodes: &mut [AstNode],
    depth: usize,
    warnings: &mut Vec<Text>,
    edits: &mut super::origin::Edits,
) -> Result<(), CompileError> {
    for node in nodes {
        edits.refresh(node);
        if let AstNode::Declaration { value: css, .. } = node {
            if let Some(value) = normalize_value(css, depth, warnings)? {
                edits.assign(node, "value", value)?;
            }
        }
        if let Some(children) = node.children_mut() {
            normalize(children, depth, warnings, edits)?;
        }
    }
    Ok(())
}

#[derive(Default)]
struct Resolution {
    used_value: bool,
    resolved_value: bool,
    used_modifier: bool,
    resolved_modifier: bool,
    ratio: bool,
    next_id: usize,
    nonratio: BTreeSet<usize>,
}

fn serialized_bytes(nodes: &[ValueNode]) -> usize {
    nodes
        .iter()
        .map(|node| match node {
            ValueNode::Word { value } | ValueNode::Separator { value } => value.len(),
            ValueNode::Function { name, nodes } => name
                .len()
                .saturating_add(2)
                .saturating_add(serialized_bytes(nodes)),
        })
        .fold(0usize, usize::saturating_add)
}

fn substitute(
    nodes: &mut Vec<AstNode>,
    input: (&CandidateValue, Option<&CandidateValue>, Option<&Text>),
    theme: &Theme,
    limits: &Limits,
    state: &mut Resolution,
    edits: &mut super::origin::Edits,
) -> Result<(), CompileError> {
    let (candidate_value, modifier, fraction) = input;
    fn functions(
        nodes: &mut Vec<ValueNode>,
        input: (&CandidateValue, Option<&CandidateValue>, Option<&Text>),
        theme: &Theme,
        limits: &Limits,
        state: &mut Resolution,
        nonratio: &mut bool,
    ) -> Result<bool, CompileError> {
        let (candidate_value, modifier, fraction) = input;
        let mut output = Vec::new();
        let mut output_bytes = 0usize;
        for mut node in std::mem::take(nodes) {
            if let ValueNode::Function { name, nodes } = &mut node {
                let target = if name == "--value" {
                    state.used_value = true;
                    Some((candidate_value, fraction))
                } else if name == "--modifier" {
                    let Some(modifier) = modifier else {
                        return Ok(false);
                    };
                    state.used_modifier = true;
                    Some((modifier, None))
                } else {
                    None
                };
                if let Some((target, fraction)) = target {
                    let Some((replacement, ratio)) =
                        resolve(target, fraction, nodes, theme, limits)?
                    else {
                        return Ok(false);
                    };
                    if name == "--value" {
                        state.resolved_value = true;
                        state.ratio |= ratio;
                        *nonratio |= !ratio;
                    } else {
                        state.resolved_modifier = true;
                    }
                    output_bytes = output_bytes.saturating_add(replacement.len());
                    if output_bytes > limits.css_bytes {
                        return Err(CompileError::Limit {
                            resource: "css_bytes".into(),
                            limit: limits.css_bytes,
                        });
                    }
                    output.extend(value::parse_text(&replacement, limits.ast_depth)?);
                    continue;
                }
                if !functions(nodes, input, theme, limits, state, nonratio)? {
                    return Ok(false);
                }
            }
            output_bytes =
                output_bytes.saturating_add(serialized_bytes(std::slice::from_ref(&node)));
            if output_bytes > limits.css_bytes {
                return Err(CompileError::Limit {
                    resource: "css_bytes".into(),
                    limit: limits.css_bytes,
                });
            }
            output.push(node);
        }
        *nodes = output;
        Ok(true)
    }
    let mut output = Vec::new();
    for mut node in std::mem::take(nodes) {
        edits.refresh(&mut node);
        if let AstNode::Declaration { value: css, .. } = &mut node {
            if !css.is_empty() {
                let mut parsed = value::parse_text(css, limits.ast_depth)?;
                let mut nonratio = false;
                if !functions(
                    &mut parsed,
                    (candidate_value, modifier, fraction),
                    theme,
                    limits,
                    state,
                    &mut nonratio,
                )? {
                    continue;
                }
                let replacement = value::to_css_text(&parsed);
                if replacement.len() > limits.css_bytes {
                    return Err(CompileError::Limit {
                        resource: "css_bytes".into(),
                        limit: limits.css_bytes,
                    });
                }
                edits.assign(&mut node, "value", replacement)?;
                if nonratio {
                    state.nonratio.insert(state.next_id);
                }
            }
            state.next_id += 1;
        }
        if let Some(children) = node.children_mut() {
            substitute(children, input, theme, limits, state, edits)?;
        }
        output.push(node);
    }
    *nodes = output;
    Ok(())
}

fn remove_nonratio(nodes: &mut Vec<AstNode>, id: &mut usize, removed: &BTreeSet<usize>) {
    nodes.retain_mut(|node| {
        if matches!(node, AstNode::Declaration { .. }) {
            let current = *id;
            *id += 1;
            if removed.contains(&current) {
                return false;
            }
        }
        if let Some(children) = node.children_mut() {
            remove_nonratio(children, id, removed);
        }
        true
    });
}

pub(super) fn collect_used(
    nodes: &[AstNode],
    used: &mut BTreeSet<crate::text::Text>,
    depth: usize,
) -> Result<(), CompileError> {
    for node in nodes {
        if matches!(
            node,
            AstNode::Context {
                reference: true,
                ..
            }
        ) || matches!(node, AstNode::Declaration { property, .. } if property == "--tw-sort")
        {
            continue;
        }
        if let AstNode::Declaration { value: css, .. } = node {
            used.extend(theme::used_variables(&value::parse_text(css, depth)?));
        }
        if let Some(children) = node.children() {
            collect_used(children, used, depth)?;
        }
    }
    Ok(())
}

fn resolve(
    candidate_value: &CandidateValue,
    fraction: Option<&Text>,
    args: &[ValueNode],
    theme: &Theme,
    limits: &Limits,
) -> Result<Option<(Text, bool)>, CompileError> {
    for arg in args {
        let ValueNode::Word { value: arg } = arg else {
            continue;
        };
        match candidate_value {
            CandidateValue::Named { value } => {
                let units = arg.units();
                if matches!(units.first(), Some(39 | 34))
                    && units.len() >= 2
                    && units.last() == units.first()
                    && &units[1..units.len() - 1] == value.units()
                {
                    return Ok(Some((value.clone(), false)));
                }
                if arg.starts_with("--") {
                    let namespace =
                        Text::from_units(units[..arg.find("-*").unwrap_or(units.len())].to_vec());
                    let base = [
                        concat_text(&[&namespace, &"-".into(), value]),
                        concat_text(&[&namespace, &"-".into(), &value.replace_ascii(b'.', "_")]),
                    ]
                    .into_iter()
                    .find(|key| theme.tokens.contains_key(key));
                    let Some(base) = base else { continue };
                    let key = if arg.ends_with("-*") {
                        base
                    } else if let Some(last) = arg.rfind("-*") {
                        concat_text(&[&base, &Text::from_units(units[last + 2..].to_vec())])
                    } else {
                        continue;
                    };
                    if let Some(resolved) = theme
                        .resolve_text(&key, &mut BTreeSet::new())
                        .filter(|value| !value.is_empty())
                    {
                        return Ok(Some((resolved, false)));
                    }
                    continue;
                }
                let resolved = if arg == "ratio" {
                    fraction
                } else {
                    Some(value)
                };
                let Some(resolved) = resolved else { continue };
                let valid = if arg == "number" {
                    is_number_text(resolved) && canonical_nonnegative_text(resolved, false)
                } else if arg == "integer" {
                    canonical_nonnegative_text(resolved, true)
                } else if arg == "percentage" {
                    resolved
                        .strip_suffix("%")
                        .is_some_and(|value| canonical_nonnegative_text(&value, true))
                } else if arg == "ratio" {
                    let parts = value::segment_text(resolved, 47, limits.ast_depth)?;
                    parts.len() == 2
                        && parts
                            .iter()
                            .all(|value| canonical_nonnegative_text(value, true))
                } else {
                    false
                };
                if valid {
                    return Ok(Some((resolved.clone(), arg == "ratio")));
                }
            }
            CandidateValue::Arbitrary { value, data_type } => {
                let Some(expected) = arg.strip_prefix("[").and_then(|arg| arg.strip_suffix("]"))
                else {
                    continue;
                };
                if expected == "*"
                    || data_type.as_ref().is_some_and(|hint| hint == &expected)
                    || data_type.is_none() && infer(value, &expected, limits.ast_depth)?
                {
                    return Ok(Some((value.clone(), false)));
                }
            }
        }
    }
    Ok(None)
}

fn numeric(value: &Text) -> bool {
    let units = value.units();
    let mut i = usize::from(matches!(units.first(), Some(43 | 45)));
    let start = i;
    while units.get(i).is_some_and(|unit| (48..=57).contains(unit)) {
        i += 1;
    }
    let integral = i - start;
    if units.get(i) == Some(&46) {
        i += 1;
        let start = i;
        while units.get(i).is_some_and(|unit| (48..=57).contains(unit)) {
            i += 1;
        }
        if i == start {
            return false;
        }
    } else if integral == 0 {
        return false;
    }
    if matches!(units.get(i), Some(69 | 101)) {
        i += 1;
        if matches!(units.get(i), Some(43 | 45)) {
            i += 1;
        }
        let start = i;
        while units.get(i).is_some_and(|unit| (48..=57).contains(unit)) {
            i += 1;
        }
        if i == start {
            return false;
        }
    }
    i == units.len()
}

fn math(value: &Text) -> bool {
    [
        "calc", "min", "max", "clamp", "mod", "rem", "sin", "cos", "tan", "asin", "acos", "atan",
        "atan2", "pow", "sqrt", "hypot", "log", "exp", "round",
    ]
    .iter()
    .any(|name| value.contains(&format!("{name}(")))
}

fn length(value: &Text) -> bool {
    [
        "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "rlh", "vw", "vh",
        "vmin", "vmax", "vb", "vi", "svw", "svh", "lvw", "lvh", "dvw", "dvh", "cqw", "cqh", "cqi",
        "cqb", "cqmin", "cqmax",
    ]
    .iter()
    .any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|value| numeric(&value))
    }) || math(value)
}

fn percentage(value: &Text) -> bool {
    value.strip_suffix("%").is_some_and(|value| numeric(&value)) || math(value)
}

fn infer(value: &Text, data_type: &Text, depth: usize) -> Result<bool, CompileError> {
    if value.starts_with("var(") {
        return Ok(false);
    }
    let Some(kind) = [
        "color",
        "length",
        "percentage",
        "number",
        "integer",
        "ratio",
        "url",
        "line-width",
        "image",
        "family-name",
        "generic-name",
        "absolute-size",
        "relative-size",
        "angle",
        "vector",
        "position",
        "bg-size",
    ]
    .into_iter()
    .find(|kind| data_type == kind) else {
        return Ok(false);
    };
    Ok(match kind {
        "color" => is_color_text(value),
        "length" => length(value),
        "percentage" => percentage(value),
        "number" => is_number_text(value),
        "integer" => canonical_nonnegative_text(value, true),
        "ratio" => {
            math(value) || {
                let parts = value::segment_text(value, 47, depth)?;
                // Fixed upstream's template literal interprets `\s` as `s`.
                if parts.len() != 2 {
                    false
                } else {
                    let left = parts[0].units();
                    let right = parts[1].units();
                    let end = left
                        .iter()
                        .rposition(|unit| *unit != 115)
                        .map_or(0, |index| index + 1);
                    let start = right.iter().take_while(|unit| **unit == 115).count();
                    numeric(&Text::from_units(left[..end].to_vec()))
                        && numeric(&Text::from_units(right[start..].to_vec()))
                }
            }
        }
        "url" => value.starts_with("url(") && value.ends_with(")"),
        "line-width" => value::segment_text(value, 32, depth)?.iter().all(|part| {
            length(part)
                || is_number_text(part)
                || ["thin", "medium", "thick"]
                    .iter()
                    .any(|keyword| part == keyword)
        }),
        "image" => {
            let mut count = 0;
            let mut valid = true;
            for part in value::segment_text(value, 44, depth)? {
                if part.starts_with("var(") {
                    continue;
                }
                let gradient = part
                    .strip_prefix("repeating-")
                    .unwrap_or_else(|| part.clone());
                if part.starts_with("url(") && part.ends_with(")")
                    || ["element(", "image(", "cross-fade(", "image-set("]
                        .iter()
                        .any(|prefix| part.starts_with(prefix))
                    || ["conic-gradient(", "linear-gradient(", "radial-gradient("]
                        .iter()
                        .any(|prefix| gradient.starts_with(prefix))
                {
                    count += 1;
                } else {
                    valid = false;
                    break;
                }
            }
            valid && count > 0
        }
        "family-name" => {
            let mut count = 0;
            let mut valid = true;
            for part in value::segment_text(value, 44, depth)? {
                if part
                    .units()
                    .first()
                    .is_some_and(|unit| (48..=57).contains(unit))
                {
                    valid = false;
                    break;
                }
                if !part.starts_with("var(") {
                    count += 1;
                }
            }
            valid && count > 0
        }
        "generic-name" => [
            "serif",
            "sans-serif",
            "monospace",
            "cursive",
            "fantasy",
            "system-ui",
            "ui-serif",
            "ui-sans-serif",
            "ui-monospace",
            "ui-rounded",
            "math",
            "emoji",
            "fangsong",
        ]
        .iter()
        .any(|keyword| value == keyword),
        "absolute-size" => [
            "xx-small",
            "x-small",
            "small",
            "medium",
            "large",
            "x-large",
            "xx-large",
            "xxx-large",
        ]
        .iter()
        .any(|keyword| value == keyword),
        "relative-size" => value == "larger" || value == "smaller",
        "angle" => ["deg", "rad", "grad", "turn"].iter().any(|unit| {
            value
                .strip_suffix(unit)
                .is_some_and(|value| numeric(&value))
        }),
        "vector" => {
            let parts = value
                .units()
                .split(|unit| *unit == 32)
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>();
            parts.len() == 3
                && !value.starts_with(" ")
                && !value.ends_with(" ")
                && parts
                    .iter()
                    .all(|part| numeric(&Text::from_units(part.to_vec())))
        }
        "position" => {
            let mut count = 0;
            let mut valid = true;
            for part in value::segment_text(value, 32, depth)? {
                if ["center", "top", "right", "bottom", "left"]
                    .iter()
                    .any(|keyword| &part == keyword)
                    || length(&part)
                    || percentage(&part)
                {
                    count += 1;
                } else if !part.starts_with("var(") {
                    valid = false;
                    break;
                }
            }
            valid && count > 0
        }
        "bg-size" => {
            let mut count = 0;
            let mut valid = true;
            for size in value::segment_text(value, 44, depth)? {
                if size == "cover" || size == "contain" {
                    count += 1;
                    continue;
                }
                let parts = value::segment_text(&size, 32, depth)?;
                if !matches!(parts.len(), 1 | 2) {
                    valid = false;
                    break;
                }
                if parts
                    .iter()
                    .all(|part| part == "auto" || length(part) || percentage(part))
                {
                    count += 1;
                }
            }
            valid && count > 0
        }
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::parser;
    #[cfg(rustify_repo_tests)]
    use crate::{
        compiler::{Compiler, CompilerInput},
        protocol::SourceAsset,
    };

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
    fn custom_literal_namespace_modifier_and_completion_preserve_original_units() {
        let opaque = Text::from_units(vec![0xd800, 0xfffd, 92, 117, 100, 56, 48, 48]);
        let namespace = concat_text(&[&"--opaque-".into(), &opaque]);
        let mut nodes = vec![AstNode::AtRule {
            origin: None,
            name: "@utility".into(),
            params: "probe-*".into(),
            source: None,
            destination: None,
            nodes: Some(vec![
                AstNode::declaration(
                    "--literal",
                    concat_text(&[&"--value('".into(), &opaque, &"')".into()]),
                ),
                AstNode::declaration(
                    "--theme",
                    concat_text(&[&"--value(".into(), &namespace, &"-*)".into()]),
                ),
                AstNode::declaration("--modifier", "--modifier([*])"),
            ]),
        }];
        let custom = CustomUtilities::extract(&mut nodes, &limits()).unwrap();
        let metadata = &custom.definitions[0].completion;
        assert_eq!(metadata.values.literals, vec![opaque.clone()]);
        assert_eq!(metadata.values.namespaces, vec![namespace.clone()]);
        let mut theme_nodes = vec![AstNode::AtRule {
            origin: None,
            name: "@theme".into(),
            params: "inline".into(),
            source: None,
            destination: None,
            nodes: Some(vec![AstNode::declaration(
                crate::compiler::candidate::escape_text(&concat_text(&[
                    &namespace,
                    &"-card".into(),
                ])),
                opaque.clone(),
            )]),
        }];
        let theme = Theme::extract(&mut theme_nodes, 32).unwrap();
        for (value, property) in [
            (opaque.clone(), "--literal"),
            (Text::from("card"), "--theme"),
        ] {
            let candidate = Candidate {
                raw: concat_text(&[&"probe-".into(), &value]),
                important: false,
                variants: Vec::new(),
                kind: CandidateKind::Functional {
                    root: "probe".into(),
                    value: Some(CandidateValue::Named { value }),
                    modifier: Some(concat_text(&[&"[".into(), &opaque, &"]".into()])),
                },
            };
            let result = custom
                .compile(&candidate, &theme, &mut BTreeSet::new(), &limits())
                .unwrap();
            assert_eq!(
                result.outputs,
                vec![vec![
                    AstNode::declaration(property, opaque.clone()),
                    AstNode::declaration("--modifier", opaque.clone())
                ]]
            );
        }
        let candidate = Candidate {
            raw: "unknown".into(),
            important: false,
            variants: Vec::new(),
            kind: CandidateKind::Static { root: opaque },
        };
        assert!(custom
            .compile(&candidate, &theme, &mut BTreeSet::new(), &limits())
            .unwrap()
            .outputs
            .is_empty());
    }

    #[test]
    fn custom_wildcard_and_unknown_type_warning_preserve_original_units() {
        let opaque = Text::from_units(vec![0xd800, 0xfffd]);
        let mut nodes = vec![AstNode::AtRule {
            origin: None,
            name: "@utility".into(),
            params: "probe-*".into(),
            source: None,
            destination: None,
            nodes: Some(vec![
                AstNode::declaration("--wildcard", "--value([*])"),
                AstNode::declaration(
                    "--unknown",
                    concat_text(&[&"--value(".into(), &opaque, &")".into()]),
                ),
            ]),
        }];
        let custom = CustomUtilities::extract(&mut nodes, &limits()).unwrap();
        let expected = concat_text(&[&"Unsupported bare value data type: \"".into(), &opaque, &"\".\nOnly valid data types are: \"number\", \"integer\", \"ratio\", \"percentage\".\n".into()]);
        assert_eq!(custom.warnings()[0], expected);
        let candidate = Candidate {
            raw: "probe-[opaque]".into(),
            important: false,
            variants: Vec::new(),
            kind: CandidateKind::Functional {
                root: "probe".into(),
                value: Some(CandidateValue::Arbitrary {
                    value: opaque.clone(),
                    data_type: None,
                }),
                modifier: None,
            },
        };
        let result = custom
            .compile(
                &candidate,
                &Theme::default(),
                &mut BTreeSet::new(),
                &limits(),
            )
            .unwrap();
        assert_eq!(
            result.outputs,
            vec![vec![AstNode::declaration("--wildcard", opaque)]]
        );
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn custom_utilities_match_official_css_and_configuration_errors() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/custom-utilities-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let assets = case
                .get("assets")
                .map(|assets| serde_json::from_value::<Vec<SourceAsset>>(assets.clone()).unwrap())
                .unwrap_or_default();
            let result = Compiler::new(
                CompilerInput {
                    css: case["input"].as_str().unwrap().into(),
                    entry: case["entry"].as_str().unwrap_or("").into(),
                },
                assets,
                limits(),
            );
            if let Some(expected) = case["expectedError"].as_str() {
                if !matches!(result, Err(CompileError::Syntax { ref message, .. }) if message == expected)
                {
                    failures.push(format!(
                        "{}: expected error {:?}, actual {:?}",
                        case["id"],
                        expected,
                        result.err()
                    ));
                }
                continue;
            }
            let mut compiler = match result {
                Ok(compiler) => compiler,
                Err(error) => {
                    failures.push(format!("{}: construction error {:?}", case["id"], error));
                    continue;
                }
            };
            let candidates = case["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|candidate| Text::from(candidate.as_str().unwrap()))
                .collect::<Vec<_>>();
            let prepared = match compiler.prepare_add_text(&candidates) {
                Ok(prepared) => prepared,
                Err(error) => {
                    failures.push(format!("{}: preparation error {:?}", case["id"], error));
                    continue;
                }
            };
            if prepared.report().css != case["expectedCss"].as_str().unwrap()
                || !prepared.report().unsupported.is_empty()
            {
                failures.push(format!(
                    "{}: actual {:?}, expected {:?}, pending {:?}",
                    case["id"],
                    prepared.report().css,
                    case["expectedCss"],
                    prepared.report().unsupported
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "{} mismatches; first cases: {:#?}",
            failures.len(),
            &failures[..failures.len().min(12)]
        );
    }

    #[test]
    fn ratio_prunes_nonratio_declarations_and_null_stops_later_handlers() {
        let limits = limits();
        let mut ast = parser::parse("@utility probe-* { color:red; --number:--value(number); --ratio:--value(ratio); } @utility probe-* { width:--value(integer); } @utility probe-* { color:blue; }", limits.ast_depth).unwrap();
        let custom = CustomUtilities::extract(&mut ast, &limits).unwrap();
        assert!(ast.is_empty());
        assert_eq!(
            custom.roots(UtilityKind::Functional).collect::<Vec<_>>(),
            ["probe"]
        );
        let candidate = Candidate {
            raw: "probe-2/3".into(),
            important: false,
            variants: Vec::new(),
            kind: CandidateKind::Functional {
                root: "probe".into(),
                value: Some(CandidateValue::Named { value: "2".into() }),
                modifier: Some("3".into()),
            },
        };
        let result = custom
            .compile(&candidate, &Theme::default(), &mut BTreeSet::new(), &limits)
            .unwrap();
        assert!(result.stop);
        assert_eq!(
            result.outputs,
            [vec![
                AstNode::declaration("color", "red"),
                AstNode::declaration("--ratio", "2/3")
            ]]
        );
    }

    #[test]
    fn repeated_function_expansion_respects_css_bytes_before_publication() {
        let mut limits = limits();
        limits.css_bytes = 80;
        let mut ast = parser::parse(
            "@utility probe-* { --result: --value([*]) --value([*]) --value([*]) --value([*]); }",
            limits.ast_depth,
        )
        .unwrap();
        let custom = CustomUtilities::extract(&mut ast, &limits).unwrap();
        let candidate = Candidate {
            raw: "probe-[a-long-repeated-arbitrary-value]".into(),
            important: false,
            variants: Vec::new(),
            kind: CandidateKind::Functional {
                root: "probe".into(),
                value: Some(CandidateValue::Arbitrary {
                    value: "a-long-repeated-arbitrary-value".into(),
                    data_type: None,
                }),
                modifier: None,
            },
        };
        assert!(
            matches!(custom.compile(&candidate, &Theme::default(), &mut BTreeSet::new(), &limits), Err(CompileError::Limit { resource, limit:80 }) if resource == "css_bytes")
        );
    }

    #[test]
    fn registration_accounts_owned_templates_against_cache_budget() {
        let mut limits = limits();
        limits.cache_bytes = 1;
        let mut ast = parser::parse("@utility probe { color:red; }", limits.ast_depth).unwrap();
        assert!(
            matches!(CustomUtilities::extract(&mut ast, &limits), Err(CompileError::Limit { resource, limit:1 }) if resource == "cache_bytes")
        );
    }
}
