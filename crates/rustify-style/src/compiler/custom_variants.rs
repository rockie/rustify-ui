//! CSS-native custom variant registration and structural slot substitution.

use super::{ast, ast::AstNode, value, CompileError, Limits};
use crate::text::Text;
use std::collections::BTreeMap;

pub(super) struct RawSlot<'a> {
    pub identity: std::sync::Arc<()>,
    pub substituted_identity: std::sync::Arc<()>,
    pub body: super::public_api::PublicAst,
    pub current: &'a mut super::origin::current::Current,
    pub allowance: &'a mut super::origin::current::Allowance,
    pub mutations: &'a mut Vec<super::public_api::InputMutation>,
}

impl RawSlot<'_> {
    fn substitute(&mut self, limits: &Limits) -> Result<(), CompileError> {
        use super::{
            origin::current::Action,
            public_api::{
                design_system::public_json, InputMutation, PublicValue, PublicValue as Value,
            },
        };
        use std::sync::Arc;
        let body = self.body.clone();
        self.current.walk(&body, self.allowance, limits.ast_depth, &mut |node, cursor| {
            if node["kind"] != "at-rule" {
                return Ok(Action::Continue);
            }
            if node["name"] == "@slot" {
                let values = cursor.values(&body).to_vec();
                let target = Value::Array(cursor.array().clone());
                let start = cursor.index();
                if let Some(captured) = cursor.replace_with(&values, 4 * std::mem::size_of::<InputMutation>())? {
                    self.mutations.push(InputMutation::Splice { target, start, delete_count: 1, values: captured.anchor });
                }
            } else if node["name"] == "@property" || node["name"] == "@keyframes" {
                let inner = public_json!({"kind":"at-rule","name":node["name"],"params":node["params"],"nodes":node["nodes"]});
                for (field, value) in [("kind", Value::from("at-root")), ("nodes", public_json!([inner]))] {
                    let captured = cursor.assign(node, field, &value, 4 * std::mem::size_of::<InputMutation>() + field.len() * 2)?;
                    self.mutations.push(InputMutation::Set { target: node.clone(), field: field.into(), value: captured.anchor });
                }
                return Ok(Action::Skip);
            }
            Ok(Action::Continue)
        })?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct Definition {
    name: String,
    nodes: Vec<AstNode>,
    substitute: bool,
    compounds: u8,
    raw: Option<super::public_api::PublicValue>,
}

#[derive(Clone, Debug, Default)]
pub struct CustomVariants {
    definitions: Vec<Definition>,
    roots: BTreeMap<String, usize>,
    owned_bytes: usize,
}

#[derive(Debug, Default)]
pub struct CustomVariantOutput {
    pub nodes: Vec<AstNode>,
    pub globals: Vec<AstNode>,
}

fn syntax(message: impl Into<Text>) -> CompileError {
    CompileError::Syntax {
        offset: 0,
        message: message.into(),
    }
}

fn limit(resource: &str, maximum: usize) -> CompileError {
    CompileError::Limit {
        resource: resource.into(),
        limit: maximum,
    }
}

impl CustomVariants {
    pub(super) fn install(&mut self, mut definition: Definition, body: Vec<AstNode>) {
        if definition.substitute {
            definition.compounds = compounds_for_nodes(&body);
            definition.nodes = body;
        }
        if let Some(&index) = self.roots.get(&definition.name) {
            self.definitions[index] = definition;
        } else {
            self.roots
                .insert(definition.name.clone(), self.definitions.len());
            self.definitions.push(definition);
        }
    }

    pub(super) fn install_raw(
        &mut self,
        mut definition: Definition,
        node: super::public_api::PublicValue,
        body: Vec<AstNode>,
    ) {
        if definition.substitute {
            definition.raw = Some(node);
        }
        self.install(definition, body);
    }

    pub(super) fn finish(&mut self, limits: &Limits) -> Result<(), CompileError> {
        self.owned_bytes = self.measure_owned_bytes();
        if self.owned_bytes > limits.cache_bytes {
            return Err(limit("cache_bytes", limits.cache_bytes));
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn extract(nodes: &mut Vec<AstNode>, limits: &Limits) -> Result<Self, CompileError> {
        fn collect(
            nodes: &mut Vec<AstNode>,
            nested: bool,
            custom: &mut CustomVariants,
            limits: &Limits,
        ) -> Result<(), CompileError> {
            let mut retained = Vec::new();
            for mut node in std::mem::take(nodes) {
                if let AstNode::AtRule {
                    name,
                    params,
                    nodes: children,
                    ..
                } = &mut node
                {
                    if name == "@theme" {
                        retained.push(node);
                        continue;
                    }
                    let legacy = name == "@variant"
                        && !nested
                        && children
                            .as_ref()
                            .is_none_or(|nodes| nodes.is_empty() || has_slot(nodes));
                    if name == "@custom-variant" || legacy {
                        if nested {
                            return Err(syntax("`@custom-variant` cannot be nested."));
                        }
                        let body = children.take().unwrap_or_default();
                        let definition = prepare(params, !body.is_empty(), limits)?;
                        custom.install(definition, body);
                        continue;
                    }
                }
                match &mut node {
                    AstNode::Context { nodes, .. } => collect(nodes, nested, custom, limits)?,
                    _ => {
                        if let Some(children) = node.children_mut() {
                            collect(children, true, custom, limits)?;
                        }
                    }
                }
                retained.push(node);
            }
            *nodes = retained;
            Ok(())
        }

        ast::validate_depth(nodes, limits.ast_depth)?;
        let mut custom = Self::default();
        collect(nodes, false, &mut custom, limits)?;
        custom.finish(limits)?;
        Ok(custom)
    }

    pub fn contains_root(&self, root: &str) -> bool {
        self.roots.contains_key(root)
    }

    pub fn contains_root_text(&self, root: &Text) -> bool {
        registry_name(root).is_some_and(|root| self.contains_root(&root))
    }

    pub fn compounds_text(&self, root: &Text) -> Option<u8> {
        self.compounds(&registry_name(root)?)
    }

    pub fn root_names(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.definitions
            .iter()
            .map(|definition| definition.name.as_str())
    }

    pub fn owned_bytes(&self) -> usize {
        self.owned_bytes
    }

    pub fn compounds(&self, root: &str) -> Option<u8> {
        self.roots
            .get(root)
            .map(|&index| self.definitions[index].compounds)
    }

    #[cfg(test)]
    pub fn apply(
        &self,
        root: &str,
        nodes: &[AstNode],
        limits: &Limits,
    ) -> Result<Option<CustomVariantOutput>, CompileError> {
        self.apply_current(root, nodes, limits, None)
    }

    pub(super) fn apply_text_current(
        &self,
        root: &Text,
        nodes: &[AstNode],
        limits: &Limits,
        raw_slot: Option<&mut RawSlot<'_>>,
    ) -> Result<Option<CustomVariantOutput>, CompileError> {
        match registry_name(root) {
            Some(root) => self.apply_current(&root, nodes, limits, raw_slot),
            None => Ok(None),
        }
    }

    fn apply_current(
        &self,
        root: &str,
        nodes: &[AstNode],
        limits: &Limits,
        raw_slot: Option<&mut RawSlot<'_>>,
    ) -> Result<Option<CustomVariantOutput>, CompileError> {
        let Some(&index) = self.roots.get(root) else {
            return Ok(None);
        };
        let definition = &self.definitions[index];
        let mut budget = Budget {
            bytes: 0,
            slot_depth: 0,
            limits,
        };
        let live;
        let template = if let (Some(raw), Some(slot)) = (&definition.raw, raw_slot.as_ref()) {
            live = super::public_api::variant_body_current(raw, slot.current, limits)?;
            &live
        } else {
            &definition.nodes
        };
        let template = ast::clone_fresh_checked(template, None, limits.cache_bytes)?;
        let output = instantiate(
            &template,
            nodes,
            definition.substitute,
            true,
            0,
            &mut budget,
            raw_slot,
        )?;
        ast::validate_depth(&output, limits.ast_depth)?;
        Ok(Some(CustomVariantOutput {
            nodes: output,
            globals: Vec::new(),
        }))
    }

    fn measure_owned_bytes(&self) -> usize {
        let map_nodes = self.roots.len().div_ceil(5);
        self.definitions
            .capacity()
            .saturating_mul(std::mem::size_of::<Definition>())
            .saturating_add(
                self.definitions
                    .iter()
                    .map(|definition| {
                        definition
                            .name
                            .capacity()
                            .saturating_add(nodes_owned_bytes(&definition.nodes))
                    })
                    .fold(0usize, usize::saturating_add),
            )
            .saturating_add(map_nodes.saturating_mul(
                11 * std::mem::size_of::<(String, usize)>() + 16 * std::mem::size_of::<usize>(),
            ))
            .saturating_add(
                self.roots
                    .keys()
                    .map(String::capacity)
                    .fold(0usize, usize::saturating_add),
            )
    }
}

#[cfg(test)]
fn has_slot(nodes: &[AstNode]) -> bool {
    nodes.iter().any(|node| {
        matches!(node, AstNode::AtRule { name, .. } if name == "@slot")
            || node.children().is_some_and(has_slot)
    })
}

fn registry_name(root: &Text) -> Option<String> {
    root.units().iter().all(|unit| *unit <= 0x7f).then(|| {
        root.units()
            .iter()
            .map(|unit| char::from(*unit as u8))
            .collect()
    })
}

fn join_text(values: &[Text], separator: &str) -> Text {
    let mut output = Text::default();
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(separator);
        }
        output.push_text(value);
    }
    output
}

pub(super) fn prepare(
    params: &Text,
    has_body: bool,
    limits: &Limits,
) -> Result<Definition, CompileError> {
    let parts = value::segment_text(params, 32, limits.ast_depth)?;
    let name = parts.first().cloned().unwrap_or_default();
    let identifier = name.strip_prefix("@").unwrap_or_else(|| name.clone());
    if !identifier
        .units()
        .first()
        .is_some_and(|unit| matches!(*unit, 97..=122 | 48..=57))
        || !identifier
            .units()
            .iter()
            .all(|unit| matches!(*unit, 65..=90 | 97..=122 | 48..=57 | 95 | 45))
        || identifier
            .units()
            .last()
            .is_some_and(|unit| matches!(*unit, 95 | 45))
    {
        return Err(syntax(value::concat_text(&[&"`@custom-variant ".into(), &name, &"` defines an invalid variant name. Variants should only contain alphanumeric, dashes, or underscore characters and start with a lowercase letter or number.".into()])));
    }
    // The grammar above proves the registered key is ASCII; selectors remain original units.
    let registered_name: String = name
        .units()
        .iter()
        .map(|unit| char::from(*unit as u8))
        .collect();
    let selector = parts.get(1).filter(|selector| !selector.is_empty());
    if has_body && selector.is_some() {
        return Err(syntax(value::concat_text(&[
            &"`@custom-variant ".into(),
            &name,
            &"` cannot have both a selector and a body.".into(),
        ])));
    }
    if has_body {
        return Ok(Definition {
            name: registered_name,
            nodes: Vec::new(),
            substitute: true,
            compounds: 0,
            raw: None,
        });
    }
    let selector = selector.ok_or_else(|| {
        syntax(value::concat_text(&[
            &"`@custom-variant ".into(),
            &name,
            &"` has no selector or body.".into(),
        ]))
    })?;
    // The fixed grammar slices one UTF-16 code unit from each end, without validating parentheses.
    let inner = if selector.units().len() < 2 {
        Text::default()
    } else {
        Text::from_units(selector.units()[1..selector.units().len() - 1].to_vec())
    };
    let selectors = value::segment_text(&inner, 44, limits.ast_depth)?;
    if selectors
        .iter()
        .any(|selector| selector.trim_js().is_empty())
    {
        return Err(syntax(value::concat_text(&[
            &"`@custom-variant ".into(),
            &name,
            &" (".into(),
            &join_text(&selectors, ","),
            &")` selector is invalid.".into(),
        ])));
    }
    let mut styles = Vec::new();
    let mut at_rules = Vec::new();
    let mut compounds = 0;
    let mut never = false;
    for selector in selectors {
        let selector = selector.trim_js();
        match selector_compounds(&selector) {
            Some(bits) => compounds |= bits,
            None => never = true,
        }
        if selector.starts_with("@") {
            at_rules.push(selector);
        } else {
            styles.push(selector);
        }
    }
    let slot = || {
        vec![AstNode::AtRule {
            origin: None,
            source: None,
            destination: None,
            name: "@slot".into(),
            params: Text::default(),
            nodes: None,
        }]
    };
    let mut nodes = Vec::new();
    if !styles.is_empty() {
        nodes.push(AstNode::Rule {
            origin: None,
            source: None,
            destination: None,
            selector: join_text(&styles, ", "),
            nodes: slot(),
        });
    }
    for selector in at_rules {
        // parseAtRule searches for its separator starting after the five-unit @page prefix.
        let split = selector
            .units()
            .iter()
            .enumerate()
            .find(|(index, unit)| *index >= 5 && matches!(**unit, 32 | 40))
            .map(|(index, _)| index);
        let (name, params) = split.map_or_else(
            || (selector.clone(), Text::default()),
            |index| {
                (
                    Text::from_units(selector.units()[..index].to_vec()),
                    Text::from_units(selector.units()[index..].to_vec()),
                )
            },
        );
        nodes.push(AstNode::AtRule {
            origin: None,
            source: None,
            destination: None,
            name: name.trim_js(),
            params: params.trim_js(),
            nodes: Some(slot()),
        });
    }
    Ok(Definition {
        name: registered_name,
        nodes,
        substitute: false,
        compounds: if never { 0 } else { compounds },
        raw: None,
    })
}

fn selector_compounds(selector: &crate::text::Text) -> Option<u8> {
    if selector.starts_with("@") {
        ["@media", "@supports", "@container"]
            .iter()
            .any(|prefix| selector.starts_with(prefix))
            .then_some(1)
    } else if selector.contains("::") {
        None
    } else {
        Some(2)
    }
}

fn compounds_for_nodes(nodes: &[AstNode]) -> u8 {
    fn visit(nodes: &[AstNode], compounds: &mut u8) -> bool {
        for node in nodes {
            let bits = match node {
                AstNode::Rule { selector, .. } => selector_compounds(selector),
                AstNode::AtRule { name, .. } if name != "@slot" => selector_compounds(name),
                _ => Some(0),
            };
            let Some(bits) = bits else {
                return false;
            };
            *compounds |= bits;
            if node
                .children()
                .is_some_and(|nodes| !visit(nodes, compounds))
            {
                return false;
            }
        }
        true
    }
    let mut compounds = 0;
    if visit(nodes, &mut compounds) {
        compounds
    } else {
        0
    }
}

struct Budget<'a> {
    bytes: usize,
    slot_depth: usize,
    limits: &'a Limits,
}

impl Budget<'_> {
    fn add(&mut self, bytes: usize) -> Result<(), CompileError> {
        self.bytes = self.bytes.saturating_add(bytes);
        if self.bytes > self.limits.css_bytes {
            return Err(limit("css_bytes", self.limits.css_bytes));
        }
        Ok(())
    }

    fn clone_nodes(
        &mut self,
        nodes: &[AstNode],
        depth: usize,
    ) -> Result<Vec<AstNode>, CompileError> {
        for node in nodes {
            self.count_node(node, depth)?;
        }
        Ok(nodes.to_vec())
    }

    fn count_node(&mut self, node: &AstNode, depth: usize) -> Result<(), CompileError> {
        self.add(node_bytes(node, depth))?;
        if let Some(children) = node.children() {
            let child_depth = match node {
                AstNode::Context { .. } => depth,
                AstNode::AtRoot { .. } => 0,
                _ => depth.saturating_add(1),
            };
            if child_depth > self.limits.ast_depth {
                return Err(limit("ast_depth", self.limits.ast_depth));
            }
            for child in children {
                self.count_node(child, child_depth)?;
            }
        }
        Ok(())
    }
}

fn node_bytes(node: &AstNode, depth: usize) -> usize {
    let indentation = depth.saturating_mul(2);
    match node {
        AstNode::Context { .. } | AstNode::AtRoot { .. } => 0,
        AstNode::Declaration {
            property,
            value,
            important,
            ..
        } => indentation
            .saturating_add(property.byte_len())
            .saturating_add(value.byte_len())
            .saturating_add(if *important { 15 } else { 4 }),
        AstNode::Comment { value, .. } => indentation.saturating_add(value.len()).saturating_add(5),
        AstNode::Rule { selector, .. } => indentation
            .saturating_mul(2)
            .saturating_add(selector.len())
            .saturating_add(6),
        AstNode::AtRule {
            name,
            params,
            nodes,
            ..
        } => indentation
            .saturating_mul(if nodes.is_some() { 2 } else { 1 })
            .saturating_add(name.len())
            .saturating_add(params.len())
            .saturating_add(usize::from(!params.is_empty()))
            .saturating_add(if nodes.is_some() { 6 } else { 2 }),
    }
}

fn instantiate(
    template: &[AstNode],
    slot: &[AstNode],
    substitute: bool,
    authored: bool,
    depth: usize,
    budget: &mut Budget<'_>,
    mut raw_slot: Option<&mut RawSlot<'_>>,
) -> Result<Vec<AstNode>, CompileError> {
    if depth > budget.limits.ast_depth {
        return Err(limit("ast_depth", budget.limits.ast_depth));
    }
    let mut output = Vec::new();
    for node in template {
        match node {
            AstNode::Declaration { identity, .. }
                if raw_slot
                    .as_ref()
                    .is_some_and(|slot| std::sync::Arc::ptr_eq(identity, &slot.identity)) =>
            {
                raw_slot.as_mut().unwrap().substitute(budget.limits)?;
                let mut replacement = node.clone();
                if let AstNode::Declaration { identity, .. } = &mut replacement {
                    *identity = raw_slot.as_ref().unwrap().substituted_identity.clone();
                }
                output.push(replacement);
            }
            AstNode::AtRule { name, .. } if name == "@slot" => {
                if substitute {
                    budget.slot_depth = budget.slot_depth.saturating_add(1);
                    if budget.slot_depth > budget.limits.ast_depth {
                        return Err(limit("ast_depth", budget.limits.ast_depth));
                    }
                    let replacement = instantiate(
                        slot,
                        slot,
                        true,
                        false,
                        depth,
                        budget,
                        raw_slot.as_deref_mut(),
                    )?;
                    budget.slot_depth -= 1;
                    output.extend(replacement);
                } else {
                    output.extend(budget.clone_nodes(slot, depth)?);
                }
            }
            AstNode::AtRule { name, .. }
                if substitute && (name == "@property" || name == "@keyframes") =>
            {
                budget.count_node(node, 0)?;
                let AstNode::AtRule {
                    name,
                    params,
                    nodes,
                    ..
                } = node
                else {
                    unreachable!()
                };
                use super::public_api::{design_system::public_json, PublicValue};
                use std::sync::Arc;
                // atRule() creates an inner resource without src/dst. An exact
                // generated origin preserves that absence during projection.
                let inner = public_json!({"kind":"at-rule","name":name,"params":params,"nodes":public_json!([])});
                output.push(AstNode::AtRoot {
                    origin: node.origin().cloned(),
                    nodes: vec![AstNode::AtRule {
                        origin: Some(super::ast::NodeOrigin::generated(inner)),
                        source: None,
                        destination: None,
                        name: name.clone(),
                        params: params.clone(),
                        nodes: nodes.clone(),
                    }],
                });
            }
            AstNode::AtRoot { .. } => {
                budget.count_node(node, 0)?;
                if authored {
                    output.extend(ast::clone_fresh_checked(
                        std::slice::from_ref(node),
                        None,
                        budget.limits.cache_bytes,
                    )?);
                } else {
                    output.push(node.clone());
                }
            }
            AstNode::Context {
                origin,
                base,
                source_base,
                reference,
                nodes,
            } => {
                let nodes = instantiate(
                    nodes,
                    slot,
                    substitute,
                    authored,
                    depth,
                    budget,
                    raw_slot.as_deref_mut(),
                )?;
                output.push(AstNode::Context {
                    origin: origin.clone(),
                    base: base.clone(),
                    source_base: source_base.clone(),
                    reference: *reference,
                    nodes,
                });
            }
            _ => {
                budget.add(node_bytes(node, depth))?;
                output.push(match node {
                    AstNode::Rule {
                        selector, nodes, ..
                    } => AstNode::Rule {
                        origin: node.origin().cloned(),
                        source: node.source().cloned(),
                        destination: node.destination().cloned(),
                        selector: selector.clone(),
                        nodes: instantiate(
                            nodes,
                            slot,
                            substitute,
                            authored,
                            depth.saturating_add(1),
                            budget,
                            raw_slot.as_deref_mut(),
                        )?,
                    },
                    AstNode::AtRule {
                        name,
                        params,
                        nodes,
                        ..
                    } => AstNode::AtRule {
                        origin: node.origin().cloned(),
                        source: node.source().cloned(),
                        destination: node.destination().cloned(),
                        name: name.clone(),
                        params: params.clone(),
                        nodes: nodes
                            .as_ref()
                            .map(|nodes| {
                                instantiate(
                                    nodes,
                                    slot,
                                    substitute,
                                    authored,
                                    depth.saturating_add(1),
                                    budget,
                                    raw_slot.as_deref_mut(),
                                )
                            })
                            .transpose()?,
                    },
                    AstNode::Declaration { .. } if authored => {
                        let mut fresh = node.clone();
                        if let AstNode::Declaration { identity, .. } = &mut fresh {
                            *identity = std::sync::Arc::new(());
                        }
                        fresh
                    }
                    _ => node.clone(),
                });
            }
        }
    }
    Ok(output)
}

fn nodes_owned_bytes(nodes: &Vec<AstNode>) -> usize {
    nodes
        .capacity()
        .saturating_mul(std::mem::size_of::<AstNode>())
        .saturating_add(
            nodes
                .iter()
                .map(|node| match node {
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
                        .saturating_add(nodes_owned_bytes(nodes)),
                    AstNode::Rule {
                        selector, nodes, ..
                    } => selector
                        .owned_bytes()
                        .saturating_add(nodes_owned_bytes(nodes)),
                    AstNode::AtRoot { origin: _, nodes } => nodes_owned_bytes(nodes),
                    AstNode::AtRule {
                        name,
                        params,
                        nodes,
                        ..
                    } => name
                        .owned_bytes()
                        .saturating_add(params.owned_bytes())
                        .saturating_add(nodes.as_ref().map_or(0, nodes_owned_bytes)),
                    AstNode::Declaration {
                        property, value, ..
                    } => property
                        .owned_bytes()
                        .saturating_add(value.owned_bytes())
                        .saturating_add(super::ast::DECLARATION_IDENTITY_BYTES),
                    AstNode::Comment { value, .. } => value.owned_bytes(),
                })
                .fold(0usize, usize::saturating_add),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::parser;

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
    fn selector_registration_keeps_raw_units_and_slices_one_unit_per_end() {
        for raw in [
            &[0xd800][..],
            &[0xdc00],
            &[0xd83d, 0xde00],
            &[0xfffd],
            &[0xe000],
        ] {
            let params = Text::from_units(
                "unit (&:"
                    .encode_utf16()
                    .chain(raw.iter().copied())
                    .chain(")".encode_utf16())
                    .collect(),
            );
            let definition = prepare(&params, false, &limits()).unwrap();
            let AstNode::Rule { selector, .. } = &definition.nodes[0] else {
                panic!("expected rule");
            };
            let expected: Vec<u16> = "&:".encode_utf16().chain(raw.iter().copied()).collect();
            assert_eq!(selector.units(), expected);
        }
        let params = Text::from_units(
            "unit "
                .encode_utf16()
                .chain([0xd83d, 0xde00])
                .chain("&:x".encode_utf16())
                .chain([0xd83d, 0xde00])
                .collect(),
        );
        let definition = prepare(&params, false, &limits()).unwrap();
        let AstNode::Rule { selector, .. } = &definition.nodes[0] else {
            panic!("expected rule");
        };
        assert_eq!(selector.units(), [0xde00, 38, 58, 120, 0xd83d]);
    }

    #[test]
    fn invalid_registration_name_message_retains_lone_units() {
        let params = Text::from_units(vec![120, 0xd800, 32, 40, 38, 41]);
        let error = prepare(&params, false, &limits()).unwrap_err();
        let CompileError::Syntax { message, .. } = error else {
            panic!("expected syntax error");
        };
        let expected: Vec<u16> = "`@custom-variant x".encode_utf16().chain([0xd800]).chain("` defines an invalid variant name. Variants should only contain alphanumeric, dashes, or underscore characters and start with a lowercase letter or number.".encode_utf16()).collect();
        assert_eq!(message.units(), expected);
    }

    #[test]
    fn overrides_keep_registration_order_and_selector_branches() {
        let limits = limits();
        let mut nodes = parser::parse("@custom-variant alpha (&:hover); @custom-variant beta (&:focus); @custom-variant alpha (@media screen, &:active, &:focus);", limits.ast_depth).unwrap();
        let custom = CustomVariants::extract(&mut nodes, &limits).unwrap();
        let output = custom
            .apply("alpha", &[AstNode::declaration("width", "13px")], &limits)
            .unwrap()
            .unwrap();
        assert_eq!(
            (
                custom.root_names().collect::<Vec<_>>(),
                custom.compounds("alpha"),
                ast::to_css(&output.nodes)
            ),
            (
                vec!["alpha", "beta"],
                Some(3),
                "&:active, &:focus {\n  width: 13px;\n}\n@media screen {\n  width: 13px;\n}\n"
                    .into()
            )
        );
    }

    #[test]
    fn deep_slots_duplicate_inputs_and_hoist_resources_without_expanding_resource_slots() {
        let limits = limits();
        let mut nodes = parser::parse("@custom-variant sample { @media screen { &:hover { @slot; @slot; } @property --probe { syntax: \"*\"; @slot; } } }", limits.ast_depth).unwrap();
        let custom = CustomVariants::extract(&mut nodes, &limits).unwrap();
        let output = custom
            .apply("sample", &[AstNode::declaration("width", "13px")], &limits)
            .unwrap()
            .unwrap();
        let media = output.nodes[0].children().unwrap();
        let AstNode::AtRoot {
            origin: _,
            nodes: resources,
        } = &media[1]
        else {
            panic!("expected resource in its original position");
        };
        assert_eq!(
            (
                ast::to_css(&media[..1]),
                ast::to_css(resources),
                output.globals.is_empty(),
                custom.compounds("sample")
            ),
            (
                "&:hover {\n  width: 13px;\n  width: 13px;\n}\n".into(),
                "@property --probe {\n  syntax: \"*\";\n  @slot ;\n}\n".into(),
                true,
                Some(0)
            )
        );
    }

    #[test]
    fn root_legacy_application_is_retained_and_deep_slot_legacy_registers() {
        let limits = limits();
        let mut nodes = parser::parse(
            "@variant hover { color: red; } @variant sample { @media screen { @slot; } }",
            limits.ast_depth,
        )
        .unwrap();
        let custom = CustomVariants::extract(&mut nodes, &limits).unwrap();
        assert_eq!(
            (custom.root_names().collect::<Vec<_>>(), ast::to_css(&nodes)),
            (
                vec!["sample"],
                "@variant hover {\n  color: red;\n}\n".into()
            )
        );
    }

    #[test]
    fn replacement_budget_is_checked_before_repeated_slot_clones() {
        let mut limits = limits();
        let mut nodes = parser::parse(
            "@custom-variant sample { @slot; @slot; @slot; }",
            limits.ast_depth,
        )
        .unwrap();
        let custom = CustomVariants::extract(&mut nodes, &limits).unwrap();
        limits.css_bytes = 20;
        assert_eq!(
            custom
                .apply("sample", &[AstNode::declaration("width", "13px")], &limits)
                .unwrap_err(),
            limit("css_bytes", 20)
        );
    }

    #[test]
    fn reference_context_is_transparent_for_registration() {
        let limits = limits();
        let mut nodes = vec![AstNode::Context {
            origin: None,
            base: "/app".into(),
            source_base: None,
            reference: true,
            nodes: parser::parse("@custom-variant sample (&:hover);", limits.ast_depth).unwrap(),
        }];
        let custom = CustomVariants::extract(&mut nodes, &limits).unwrap();
        let output = custom
            .apply("sample", &[AstNode::declaration("width", "13px")], &limits)
            .unwrap()
            .unwrap();
        assert_eq!(ast::to_css(&output.nodes), "&:hover {\n  width: 13px;\n}\n");
    }

    #[test]
    fn template_slots_do_not_add_structural_depth() {
        let mut limits = limits();
        let mut nodes = parser::parse(
            "@custom-variant sample { &:hover { @slot; } }",
            limits.ast_depth,
        )
        .unwrap();
        let custom = CustomVariants::extract(&mut nodes, &limits).unwrap();
        limits.ast_depth = 1;
        let output = custom
            .apply("sample", &[AstNode::declaration("width", "13px")], &limits)
            .unwrap()
            .unwrap();
        assert_eq!(ast::to_css(&output.nodes), "&:hover {\n  width: 13px;\n}\n");
    }

    #[test]
    fn repeated_slots_share_input_identity_while_authored_declarations_are_fresh() {
        let limits = limits();
        let mut nodes = parser::parse(
            "@custom-variant sample { color:red; @slot; @slot; }",
            limits.ast_depth,
        )
        .unwrap();
        let custom = CustomVariants::extract(&mut nodes, &limits).unwrap();
        let slot = vec![AstNode::declaration("width", "13px")];
        let first = custom.apply("sample", &slot, &limits).unwrap().unwrap();
        let second = custom.apply("sample", &slot, &limits).unwrap().unwrap();
        let identity = |node: &AstNode| match node {
            AstNode::Declaration { identity, .. } => identity.clone(),
            _ => panic!("expected declaration"),
        };
        assert!(
            std::sync::Arc::ptr_eq(&identity(&first.nodes[1]), &identity(&first.nodes[2]))
                && std::sync::Arc::ptr_eq(&identity(&slot[0]), &identity(&first.nodes[1]))
                && !std::sync::Arc::ptr_eq(&identity(&first.nodes[0]), &identity(&second.nodes[0]))
        );
    }

    #[test]
    fn at_root_contents_are_counted_and_copied_without_expanding_slots() {
        let mut limits = limits();
        let mut nodes =
            parser::parse("@custom-variant sample { @slot; }", limits.ast_depth).unwrap();
        let custom = CustomVariants::extract(&mut nodes, &limits).unwrap();
        let slot = vec![AstNode::AtRoot {
            origin: None,
            nodes: vec![
                AstNode::AtRule {
                    origin: None,
                    source: None,
                    destination: None,
                    name: "@slot".into(),
                    params: crate::text::Text::default(),
                    nodes: None,
                },
                AstNode::declaration("width", "13px"),
            ],
        }];
        let output = custom.apply("sample", &slot, &limits).unwrap().unwrap();
        limits.css_bytes = 5;
        assert!(
            output.nodes == slot
                && output.globals.is_empty()
                && custom.apply("sample", &slot, &limits).unwrap_err() == limit("css_bytes", 5)
        );
    }
}
