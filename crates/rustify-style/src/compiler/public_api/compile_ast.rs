//! Public AST nodes use the upstream JSON shape, including explicit undefined
//! markers. Array handles preserve the observable identity of `build` results.

use super::design_system::{literal, public_json, PublicValue, PublicValue as Value};
use super::{CompileOptions, Features, Stylesheet, StylesheetLoad, BANNER};
use crate::compiler::origin::current::{Allowance, Current};
use crate::compiler::{self, ast::AstNode, CompileError, Compiler, Limits, SourceMetadata};
use crate::text::Text;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

/// Ordered upstream AST objects. `{ "undefined": true }` represents an
/// explicitly present undefined property, distinct from an omitted property.
pub type PublicAst = Arc<Vec<Value>>;
mod setup;
pub use setup::{
    prepare_ast_setup, prepare_ast_setup_async, AstSetupFault, AstSetupJob, PreparedAstSetup,
};

/// Ordered operations on caller-owned AST objects and arrays.
/// Hosts apply these effects separately from the optimized build result.
#[derive(Clone, Debug)]
pub enum InputMutation {
    Set {
        target: Value,
        field: Text,
        value: Value,
    },
    Splice {
        target: Value,
        start: usize,
        delete_count: usize,
        values: Value,
    },
}

impl InputMutation {
    pub fn target(&self) -> &Value {
        match self {
            Self::Set { target, .. } | Self::Splice { target, .. } => target,
        }
    }
    pub fn value(&self) -> &Value {
        match self {
            Self::Set { value, .. } => value,
            Self::Splice { values, .. } => values,
        }
    }
    pub fn field(&self) -> Option<&Text> {
        match self {
            Self::Set { field, .. } => Some(field),
            Self::Splice { .. } => None,
        }
    }
}

pub struct AstCompiler {
    compiler: Compiler,
    input: PublicAst,
    compiled: Option<PublicAst>,
    banner: Value,
    license_banner: bool,
    origins: Origins,
    from: bool,
    previous_node_count: usize,
    previous_variables: usize,
    input_mutations: Vec<InputMutation>,
    import_program: Option<compiler::imports::schedule::Program>,
    setup_compatibility: Option<usize>,
}

/// Constructor outcome with admitted caller effects even when semantic setup fails.
pub enum AstSetupOutcome {
    /// Fully initialized compiler; its successful effects remain on the compiler.
    Complete(Box<AstCompiler>),
    /// Semantic setup failure and the completed effects retained for host replay.
    Failed(AstSetupFailure),
}

/// A failed constructor has no compiler to publish. Hosts preflight this reserve
/// and their crossing handles before replaying effects and returning the error.
pub struct AstSetupFailure {
    /// Original semantic error, unless an earlier host write throws during replay.
    pub error: CompileError,
    /// Completed caller operations in their recorded order.
    pub input_mutations: Vec<InputMutation>,
    /// Conservative native graph/journal reserve for the host admission check.
    pub cache_bytes: usize,
    import_program: Option<compiler::imports::schedule::Program>,
    setup_compatibility: Option<usize>,
}

#[derive(Default)]
struct Origins {
    declarations: BTreeMap<usize, (Arc<()>, Value)>,
    generated_declarations: BTreeMap<usize, (Arc<()>, Value)>,
    authored: BTreeMap<OriginKey, Vec<AuthoredOrigin>>,
    theme_declarations: BTreeMap<Text, Value>,
    source_locations: BTreeMap<(usize, usize, usize, bool), Value>,
    source_owners: BTreeMap<usize, Arc<super::Source>>,
    theme_rule: Option<Value>,
    theme_node: Option<Value>,
    utility_definitions: BTreeMap<usize, Vec<usize>>,
    utility_occurrences: BTreeMap<usize, usize>,
    utilities: Option<Value>,
    utility_node: Option<Value>,
    raw_candidates: BTreeMap<usize, (Arc<compiler::utilities::CompiledCandidate>, Value)>,
    apply_inputs: BTreeMap<usize, (Arc<Value>, PublicAst, usize)>,
    apply_nodes: BTreeMap<usize, Vec<Arc<Value>>>,
}

#[derive(Default)]
struct ImportViews {
    current: Arc<Current>,
    allowance: Allowance,
}
impl ImportViews {
    fn new(limit: usize) -> Self {
        Self {
            current: Arc::new(Current::default()),
            allowance: Allowance::new(0, limit),
        }
    }
    fn node<'a>(&'a self, value: &'a Value) -> &'a Value {
        self.current.value(value)
    }
    fn nodes<'a>(&'a self, input: &'a PublicAst) -> &'a [Value] {
        self.current.array(input)
    }
    // Imported arrays already have live host handles from earlier reactions.
    // Replay uses their anchors; a frozen storage version would detach them
    // from later operations that target the original receiver.
    fn assign(&mut self, target: &Value, field: &str, value: Value) -> Result<Value, CompileError> {
        let captured = Arc::get_mut(&mut self.current)
            .expect("registration owner is unique before lowering")
            .assign(
                target,
                field,
                &value,
                &mut self.allowance,
                4 * std::mem::size_of::<InputMutation>() + field.encode_utf16().count() * 2,
            )?;
        Ok(captured.anchor)
    }
    fn replace(
        &mut self,
        parent: &PublicAst,
        start: usize,
        delete_count: usize,
        items: &[Value],
    ) -> Result<Value, CompileError> {
        let captured = Arc::get_mut(&mut self.current)
            .expect("registration owner is unique before lowering")
            .splice(
                &Value::Array(parent.clone()),
                start,
                delete_count,
                items,
                &mut self.allowance,
                4 * std::mem::size_of::<InputMutation>(),
            )?;
        Ok(captured.anchor)
    }
}

struct ImportGraph<'a> {
    root: PublicAst,
    base: Text,
    origins: &'a mut Origins,
    views: ImportViews,
    initial_mutations: Vec<InputMutation>,
    apply_failure_mutations: Vec<InputMutation>,
    mutations: Vec<InputMutation>,
    import_program: Option<compiler::imports::schedule::Program>,
    setup_compatibility: Option<usize>,
}
impl compiler::imports::PublicSetup for ImportGraph<'_> {
    fn root(&self) -> PublicAst {
        self.root.clone()
    }
    fn parsed(&mut self, nodes: &mut [AstNode]) -> Result<PublicAst, CompileError> {
        self.origins.bind_locations(nodes);
        let values = Arc::new(self.origins.output_raw(
            nodes,
            None,
            false,
            &mut BTreeMap::new(),
            &mut BTreeMap::new(),
        )?);
        self.origins.bind_imported(nodes, &values);
        Ok(values)
    }
    fn begin(
        &mut self,
        parent: &PublicAst,
        index: usize,
    ) -> Result<(Value, InputMutation), CompileError> {
        let context =
            public_json!({"kind":"context", "context":public_json!({}), "nodes":public_json!([])});
        let captured = self
            .views
            .replace(parent, index, 1, std::slice::from_ref(&context))?;
        let effect = InputMutation::Set {
            target: Value::Array(parent.clone()),
            field: index.to_string().into(),
            value: captured[0].clone(),
        };
        Ok((context, effect))
    }
    fn scheduled(
        &mut self,
        program: compiler::imports::schedule::Program,
        prefix: Vec<InputMutation>,
    ) {
        self.initial_mutations = prefix;
        self.import_program =
            (!program.tasks.is_empty() || program.scopes[0].error.is_some()).then_some(program);
    }
    fn compatibility(&mut self) {
        self.setup_compatibility = Some(self.initial_mutations.len() + self.mutations.len());
    }
    fn failed_apply(
        &mut self,
        replacements: Vec<compiler::apply::Replacement>,
    ) -> Result<(), CompileError> {
        apply_input_mutations(
            self.origins,
            &self.root,
            &self.views,
            &mut self.apply_failure_mutations,
            replacements,
        )?;
        Ok(())
    }
    fn initial(
        &mut self,
        limits: &Limits,
    ) -> Result<compiler::registration::Initial, CompileError> {
        let compiler::registration::Registered {
            sources,
            important,
            utilities,
            variants,
            mut theme,
            theme_rule,
            keyframes,
            definitions,
            authored_variants,
        } = compiler::registration::run(
            &self.root,
            Arc::get_mut(&mut self.views.current)
                .expect("registration owner is unique before lowering"),
            &mut self.views.allowance,
            &self.base,
            limits,
            &mut self.mutations,
        )?;
        self.compatibility();
        compiler::registration::reject_javascript_configuration(
            &self.root,
            Arc::get_mut(&mut self.views.current)
                .expect("registration owner is unique before lowering"),
            &mut self.views.allowance,
            limits,
        )?;
        let (warnings, completions) = compiler::registration::normalize_definitions(
            &definitions,
            Arc::get_mut(&mut self.views.current)
                .expect("registration owner is unique before lowering"),
            &mut self.views.allowance,
            limits,
            &mut self.mutations,
        )?;
        self.origins.theme_declarations = compiler::registration::publish_theme(
            &theme,
            theme_rule.as_ref(),
            Arc::get_mut(&mut self.views.current)
                .expect("registration owner is unique before lowering"),
            &mut self.views.allowance,
            &mut self.mutations,
        )?;
        apply_registered_variants(
            &variants,
            &authored_variants,
            &mut theme,
            Arc::get_mut(&mut self.views.current)
                .expect("registration owner is unique before lowering"),
            &mut self.views.allowance,
            limits,
            &mut self.mutations,
        )?;
        let has_functions = compiler::functions::substitute_current(
            &self.root,
            &theme,
            Arc::get_mut(&mut self.views.current)
                .expect("registration owner is unique before lowering"),
            &mut self.views.allowance,
            limits,
            &mut self.mutations,
        )?;
        // Hoisted theme keyframes follow the caller tree without changing its root array.
        let keyframes = Arc::new(keyframes);
        let has_functions = compiler::functions::substitute_current(
            &keyframes,
            &theme,
            Arc::get_mut(&mut self.views.current)
                .expect("registration owner is unique before lowering"),
            &mut self.views.allowance,
            limits,
            &mut self.mutations,
        )? || has_functions;
        self.origins.reset_lowering();
        self.origins.utility_node = utilities.clone();
        self.origins.theme_rule = theme_rule
            .as_ref()
            .and_then(|node| node.get("src"))
            .cloned();
        self.origins.theme_node = theme_rule;
        self.origins.register_definitions(&definitions);
        let nodes =
            self.origins
                .convert_current(&self.root, &self.views.current, limits.ast_depth)?;
        let variants = variants
            .into_iter()
            .map(|(definition, node)| {
                let Value::Array(children) = &self.views.node(&node)["nodes"] else {
                    unreachable!("validated definition body")
                };
                Ok((
                    definition,
                    self.origins.convert_current(
                        children,
                        &self.views.current,
                        limits.ast_depth,
                    )?,
                ))
            })
            .collect::<Result<Vec<_>, CompileError>>()?;
        theme.keyframes =
            self.origins
                .convert_current(&keyframes, &self.views.current, limits.ast_depth)?;
        let definitions = self.origins.lower_definitions(
            definitions,
            completions,
            &self.views.current,
            limits.ast_depth,
        )?;
        Ok(compiler::registration::Initial {
            nodes,
            sources,
            important,
            utilities: utilities.is_some(),
            variants,
            theme,
            definitions,
            warnings,
            has_authored_variants: !authored_variants.is_empty(),
            has_functions,
        })
    }
    fn complete(
        &mut self,
        parent: &PublicAst,
        index: usize,
        original: &AstNode,
        imported: &mut AstNode,
        loaded: &PublicAst,
        context: Value,
    ) -> Result<InputMutation, CompileError> {
        fn wrappers(node: &mut AstNode, loaded: &PublicAst, source: &Value) -> Value {
            let value = match node {
                AstNode::Context { base, .. } => {
                    public_json!({"kind":"context", "context":public_json!({"base":base.clone()}), "nodes":Value::Array(loaded.clone())})
                }
                AstNode::AtRule {
                    name,
                    params,
                    nodes,
                    ..
                } => {
                    let children = nodes.as_mut().unwrap();
                    let values: Vec<_> = children
                        .iter_mut()
                        .map(|child| wrappers(child, loaded, source))
                        .collect();
                    public_json!({"kind":"at-rule", "name":name.clone(), "params":params.clone(), "nodes":values, "src":source})
                }
                _ => unreachable!("import wrapper"),
            };
            node.set_origin(Some(compiler::ast::NodeOrigin::caller(value.clone())));
            value
        }
        let source = original
            .origin()
            .and_then(|origin| origin.fields().get("src"))
            .cloned()
            .unwrap_or_else(undefined);
        let nodes = imported.children_mut().unwrap();
        let values: Vec<_> = nodes
            .iter_mut()
            .map(|node| wrappers(node, loaded, &source))
            .collect();
        let values = Value::Array(Arc::new(values));
        let captured = self.views.assign(&context, "nodes", values)?;
        let effect = InputMutation::Set {
            target: context.clone(),
            field: "nodes".into(),
            value: captured,
        };
        let current = self.views.node(&context).clone();
        imported.set_origin(Some(compiler::ast::NodeOrigin::caller(current.clone())));
        self.views.replace(parent, index, 1, &[current])?;
        Ok(effect)
    }
}

pub(in crate::compiler) fn variant_body_current(
    node: &Value,
    current: &Current,
    limits: &Limits,
) -> Result<Vec<AstNode>, CompileError> {
    let Value::Array(body) = &current.value(node)["nodes"] else {
        unreachable!("validated variant body")
    };
    Origins::default().convert_inner(body, None, 0, limits.ast_depth, Some(current), None)
}

fn apply_registered_variants(
    variants: &[(compiler::custom_variants::Definition, Value)],
    authored: &[Value],
    theme: &mut compiler::theme::Theme,
    current: &mut Current,
    allowance: &mut Allowance,
    limits: &Limits,
    mutations: &mut Vec<InputMutation>,
) -> Result<(), CompileError> {
    if authored.is_empty() {
        return Ok(());
    }
    for (definition, node) in variants {
        theme.custom_variants.install_raw(
            definition.clone(),
            node.clone(),
            variant_body_current(node, current, limits)?,
        );
    }
    theme.custom_variants.finish(limits)?;
    // Captures use the initial walk's order, including repeated aliases. New
    // wrappers are not walked again; each application reads its live body.
    for target in authored {
        let node = current.value(target);
        let params = field(node, "params")?.clone();
        let origins = Origins::default();
        let Value::Array(body) = &node["nodes"] else {
            unreachable!("validated variant body")
        };
        // The variant kernel wraps the slot; it does not own the caller's
        // body. Keep that array opaque until the generated wrappers are read.
        let mut slot = AstNode::declaration("", "");
        let identity = match &slot {
            AstNode::Declaration { identity, .. } => identity.clone(),
            _ => unreachable!(),
        };
        slot.set_origin(None);
        let body = body.clone();
        let substituted_identity = Arc::new(());
        let mut raw_slot = compiler::custom_variants::RawSlot {
            identity: identity.clone(),
            substituted_identity: substituted_identity.clone(),
            body: body.clone(),
            current,
            allowance,
            mutations,
        };
        let wrapped = compiler::variants::apply_authored_variant(
            &params,
            vec![slot],
            theme,
            limits,
            Some(&mut raw_slot),
        )?;
        let values = variant_values(
            &origins,
            &wrapped,
            &identity,
            &substituted_identity,
            &body,
            current,
        )?;
        // Object.assign(styleRule) keeps the receiver's other fields and writes
        // kind, selector, nodes in this order, even when their values agree.
        for (field, value) in [
            ("kind", Value::from("rule")),
            ("selector", Value::from("&")),
            ("nodes", values),
        ] {
            let captured = current.assign(
                target,
                field,
                &value,
                allowance,
                4 * std::mem::size_of::<InputMutation>() + field.len() * 2,
            )?;
            mutations.push(InputMutation::Set {
                target: target.clone(),
                field: field.into(),
                value: captured.anchor,
            });
        }
    }
    theme.custom_variants = compiler::custom_variants::CustomVariants::default();
    Ok(())
}

fn variant_values(
    origins: &Origins,
    nodes: &[AstNode],
    slot: &Arc<()>,
    substituted: &Arc<()>,
    body: &PublicAst,
    current: &Current,
) -> Result<Value, CompileError> {
    fn project(
        nodes: &[AstNode],
        values: Vec<Value>,
        slot: &Arc<()>,
        substituted: &Arc<()>,
        body: &PublicAst,
        current: &Current,
    ) -> Value {
        let is_slot = |node: &AstNode| matches!(node, AstNode::Declaration { identity, .. } if Arc::ptr_eq(identity, slot) || Arc::ptr_eq(identity, substituted));
        if nodes.len() == 1
            && matches!(&nodes[0], AstNode::Declaration { identity, .. } if Arc::ptr_eq(identity, slot))
        {
            return Value::Array(body.clone());
        }
        let mut output = Vec::new();
        for (node, mut value) in nodes.iter().zip(values) {
            if is_slot(node) {
                output.extend(current.array(body).iter().cloned());
            } else {
                if let Some(children) = node.children() {
                    let values = value["nodes"].as_array().unwrap().to_vec();
                    value["nodes"] = project(children, values, slot, substituted, body, current);
                }
                output.push(value);
            }
        }
        Value::Array(Arc::new(output))
    }
    // Serialize the generated tree once, then reconnect its opaque slot edges.
    let values = origins.output_raw(
        nodes,
        None,
        false,
        &mut BTreeMap::new(),
        &mut BTreeMap::new(),
    )?;
    Ok(project(nodes, values, slot, substituted, body, current))
}

pub(in crate::compiler) fn register_native(
    nodes: &[AstNode],
    base: &str,
    limits: &Limits,
) -> Result<compiler::registration::Initial, CompileError> {
    let mut origins = Origins::default();
    origins.bind_locations(nodes);
    let root = Arc::new(origins.output_raw(
        nodes,
        None,
        false,
        &mut BTreeMap::new(),
        &mut BTreeMap::new(),
    )?);
    let mut current = Current::default();
    let mut allowance = Allowance::new(0, limits.cache_bytes);
    let compiler::registration::Registered {
        sources,
        important,
        utilities,
        variants,
        mut theme,
        theme_rule,
        keyframes,
        definitions,
        authored_variants,
    } = compiler::registration::run(
        &root,
        &mut current,
        &mut allowance,
        &Text::from(base),
        limits,
        &mut Vec::new(),
    )?;
    compiler::registration::reject_javascript_configuration(
        &root,
        &mut current,
        &mut allowance,
        limits,
    )?;
    let (warnings, completions) = compiler::registration::normalize_definitions(
        &definitions,
        &mut current,
        &mut allowance,
        limits,
        &mut Vec::new(),
    )?;
    origins.theme_declarations = compiler::registration::publish_theme(
        &theme,
        theme_rule.as_ref(),
        &mut current,
        &mut allowance,
        &mut Vec::new(),
    )?;
    apply_registered_variants(
        &variants,
        &authored_variants,
        &mut theme,
        &mut current,
        &mut allowance,
        limits,
        &mut Vec::new(),
    )?;
    let has_functions = compiler::functions::substitute_current(
        &root,
        &theme,
        &mut current,
        &mut allowance,
        limits,
        &mut Vec::new(),
    )?;
    let keyframes = Arc::new(keyframes);
    let has_functions = compiler::functions::substitute_current(
        &keyframes,
        &theme,
        &mut current,
        &mut allowance,
        limits,
        &mut Vec::new(),
    )? || has_functions;
    let current = Arc::new(current);
    origins.theme_rule = theme_rule
        .as_ref()
        .and_then(|node| node.get("src"))
        .cloned();
    origins.theme_node = theme_rule;
    origins.register_definitions(&definitions);
    let mut nodes = origins.convert_current(&root, &current, limits.ast_depth)?;
    let mut variants = variants
        .into_iter()
        .map(|(definition, node)| {
            let Value::Array(children) = &current.value(&node)["nodes"] else {
                unreachable!("validated definition body")
            };
            Ok((
                definition,
                origins.convert_current(children, &current, limits.ast_depth)?,
            ))
        })
        .collect::<Result<Vec<_>, CompileError>>()?;
    theme.keyframes = origins.convert_current(&keyframes, &current, limits.ast_depth)?;
    let mut definitions =
        origins.lower_definitions(definitions, completions, &current, limits.ast_depth)?;
    // The native adapter carries source tuples through the raw pass, then
    // restores the original native owners rather than retaining caller handles.
    let locations: BTreeMap<_, _> = origins
        .source_locations
        .iter()
        .filter_map(|(&(source, start, end, _destination), value)| {
            let Value::Array(tuple) = value else {
                return None;
            };
            Some((
                Arc::as_ptr(tuple) as usize,
                super::SourceLocation {
                    source: origins.source_owners[&source].clone(),
                    start,
                    end,
                    public: None,
                },
            ))
        })
        .collect();
    fn restore(nodes: &mut [AstNode], locations: &BTreeMap<usize, super::SourceLocation>) {
        for node in nodes {
            let location = |location: Option<&super::SourceLocation>| {
                location
                    .and_then(|location| location.public.as_deref())
                    .and_then(|value| match value {
                        Value::Array(tuple) => {
                            locations.get(&(Arc::as_ptr(tuple) as usize)).cloned()
                        }
                        _ => None,
                    })
            };
            let source = location(node.source());
            let destination = location(node.destination());
            node.set_source(source);
            node.set_destination(destination);
            node.set_origin(None);
            if let Some(children) = node.children_mut() {
                restore(children, locations);
            }
        }
    }
    restore(&mut nodes, &locations);
    for (_, body) in &mut variants {
        restore(body, &locations);
    }
    restore(&mut theme.keyframes, &locations);
    for definition in &mut definitions {
        restore(&mut definition.nodes, &locations);
        definition.origin = None;
    }
    for token in theme.tokens.values_mut() {
        token.source = token
            .source
            .as_ref()
            .and_then(|source| source.public.as_deref())
            .and_then(|value| match value {
                Value::Array(tuple) => locations.get(&(Arc::as_ptr(tuple) as usize)).cloned(),
                _ => None,
            });
    }
    Ok(compiler::registration::Initial {
        nodes,
        sources,
        important,
        utilities: utilities.is_some(),
        variants,
        theme,
        definitions,
        warnings,
        has_authored_variants: !authored_variants.is_empty(),
        has_functions,
    })
}

fn caller_utilities(input: &PublicAst, reference: bool, views: &ImportViews) -> Option<Value> {
    for node in views.nodes(input) {
        let node = views.node(node);
        let reference = reference
            || node
                .get("context")
                .and_then(|context| context.get("reference"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        if !reference
            && node["kind"] == "at-rule"
            && node["name"] == "@tailwind"
            && node
                .get("params")
                .and_then(Value::as_text)
                .is_some_and(|params| params.starts_with("utilities"))
        {
            return Some(node.clone());
        }
        if let Some(found) = node.get("nodes").and_then(|value| match value {
            Value::Array(nodes) => caller_utilities(nodes, reference, views),
            _ => None,
        }) {
            return Some(found);
        }
    }
    None
}

fn mutation_bytes(mutations: &[InputMutation], seen: &mut BTreeSet<(u8, usize)>) -> usize {
    mutations
        .iter()
        .map(|mutation| {
            std::mem::size_of::<InputMutation>()
                .saturating_add(mutation.field().map_or(0, Text::owned_bytes))
                .saturating_add(super::design_system::json_bytes_seen(
                    mutation.target(),
                    seen,
                ))
                .saturating_add(super::design_system::json_bytes_seen(
                    mutation.value(),
                    seen,
                ))
        })
        .sum()
}

pub(in crate::compiler) fn caller_source(value: Value) -> super::SourceLocation {
    super::SourceLocation {
        source: Arc::new(super::Source {
            file: None,
            code: Text::default(),
        }),
        start: 0,
        end: 0,
        public: Some(Arc::new(value)),
    }
}

pub(in crate::compiler) fn invalid_theme_child(
    name: &Text,
    params: &Text,
    child: &Value,
    cursor: &compiler::origin::current::Visit<'_>,
    depth: usize,
) -> Result<CompileError, CompileError> {
    fn error_view(
        value: &Value,
        cursor: &compiler::origin::current::Visit<'_>,
        depth: usize,
        limit: usize,
    ) -> Result<Value, CompileError> {
        compiler::check_limit(depth, limit, "ast_depth")?;
        let mut value = cursor.value(value).clone();
        // toCss prints an undefined declaration literally when constructing
        // this error, before optimizeAst discards undefined declarations.
        if value["kind"] == "declaration" && value.get("value") == Some(&undefined()) {
            value["value"] = "undefined".into();
        }
        if let Some(Value::Array(children)) = value.get("nodes") {
            let children = cursor
                .values(children)
                .iter()
                .map(|child| error_view(child, cursor, depth + 1, limit))
                .collect::<Result<Vec<_>, _>>()?;
            value["nodes"] = Value::Array(Arc::new(children));
        }
        Ok(value)
    }
    let child = Origins::default().convert(
        &Arc::new(vec![error_view(child, cursor, 0, depth)?]),
        None,
        0,
        depth,
    )?;
    Ok(compiler::theme::invalid_children(name, params, &child))
}

// Prefix lengths locate original siblings after ordered constructor splices.
// A Fenwick tree avoids rescanning every preceding directive for each @apply.
struct ArrayPositions {
    tree: Vec<usize>,
    lengths: Vec<usize>,
}

impl ArrayPositions {
    fn new(length: usize) -> Self {
        Self {
            tree: (0..=length).map(usize::isolate_lowest_one).collect(),
            lengths: vec![1; length],
        }
    }
    fn prefix(&self, mut index: usize) -> usize {
        let mut result = 0;
        while index > 0 {
            result += self.tree[index];
            index &= index - 1;
        }
        result
    }
    fn original_index(&self, offset: usize) -> usize {
        let mut low = 0;
        let mut high = self.lengths.len();
        while low < high {
            let middle = low + (high - low) / 2;
            if self.prefix(middle + 1) <= offset {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        low
    }
    fn replace(&mut self, index: usize, length: usize) {
        let old = std::mem::replace(&mut self.lengths[index], length);
        let mut index = index + 1;
        while index < self.tree.len() {
            self.tree[index] = self.tree[index] - old + length;
            index += index.isolate_lowest_one();
        }
    }
}

fn apply_input_mutations(
    origins: &mut Origins,
    input: &PublicAst,
    views: &ImportViews,
    mutations: &mut Vec<InputMutation>,
    replacements: Vec<compiler::apply::Replacement>,
) -> Result<BTreeMap<usize, ArrayPositions>, CompileError> {
    origins.rebind_apply_inputs(input, views);
    let mut positions = BTreeMap::new();
    for mutation in mutations.iter() {
        if let InputMutation::Splice {
            target: Value::Array(array),
            start,
            ..
        } = mutation
        {
            let position = positions
                .entry(Arc::as_ptr(array) as usize)
                .or_insert_with(|| ArrayPositions::new(views.nodes(array).len()));
            let original = position.original_index(*start);
            position.replace(original, 0);
        }
    }
    let mut staged = BTreeMap::new();
    let mut locations = BTreeMap::new();
    for replacement in replacements {
        let Some((_, array, index)) = origins
            .apply_inputs
            .remove(&(Arc::as_ptr(&replacement.source) as usize))
        else {
            continue;
        };
        let values =
            origins.output_raw(&replacement.nodes, None, false, &mut staged, &mut locations)?;
        let position = positions
            .entry(Arc::as_ptr(&array) as usize)
            .or_insert_with(|| ArrayPositions::new(views.nodes(&array).len()));
        let start = position.prefix(index);
        position.replace(index, values.len());
        mutations.push(if values.len() == 1 {
            InputMutation::Set {
                target: Value::Array(array),
                field: start.to_string().into(),
                value: values[0].clone(),
            }
        } else {
            InputMutation::Splice {
                target: Value::Array(array),
                start,
                delete_count: 1,
                values: Value::Array(Arc::new(values)),
            }
        });
    }
    origins.apply_inputs.clear();
    origins.generated_declarations.extend(staged);
    origins.source_locations.extend(locations);
    Ok(positions)
}

fn remove_utility_inputs(
    input: &PublicAst,
    positions: &mut BTreeMap<usize, ArrayPositions>,
    mutations: &mut Vec<InputMutation>,
    views: &ImportViews,
) {
    for (index, node) in views.nodes(input).iter().enumerate() {
        let node = views.node(node);
        if node["kind"] == "at-rule" && node["name"] == "@utility" {
            let position = positions
                .entry(Arc::as_ptr(input) as usize)
                .or_insert_with(|| ArrayPositions::new(views.nodes(input).len()));
            if position.lengths[index] == 0 {
                continue;
            }
            let start = position.prefix(index);
            position.replace(index, 0);
            mutations.push(InputMutation::Splice {
                target: Value::Array(input.clone()),
                start,
                delete_count: 1,
                values: public_json!([]),
            });
        } else if node["kind"] == "context" {
            if let Some(Value::Array(children)) = node.get("nodes") {
                remove_utility_inputs(children, positions, mutations, views);
            }
        }
    }
}

struct AuthoredOrigin {
    fields: Value,
    declarations: BTreeSet<usize>,
    shape: Option<Text>,
}

fn declaration_ids(nodes: &[AstNode], ids: &mut BTreeSet<usize>) {
    for node in nodes {
        if let AstNode::Declaration { identity, .. } = node {
            ids.insert(Arc::as_ptr(identity) as usize);
        }
        if let Some(children) = node.children() {
            declaration_ids(children, ids);
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum OriginKey {
    Rule(Text),
    AtRule(Text, Text),
    Comment(Text),
}
impl OriginKey {
    fn of(node: &AstNode) -> Option<Self> {
        match node {
            AstNode::Rule { selector, .. } => Some(Self::Rule(selector.clone())),
            AstNode::AtRule { name, params, .. } => {
                Some(Self::AtRule(name.clone(), params.clone()))
            }
            AstNode::Comment { value, .. } => Some(Self::Comment(value.clone())),
            _ => None,
        }
    }
    fn owned_bytes(&self) -> usize {
        match self {
            Self::Rule(value) | Self::Comment(value) => value.owned_bytes(),
            Self::AtRule(name, params) => name.owned_bytes().saturating_add(params.owned_bytes()),
        }
    }
}

pub(in crate::compiler) fn undefined() -> Value {
    public_json!({"undefined": true})
}
fn syntax(message: &str) -> CompileError {
    CompileError::Syntax {
        offset: 0,
        message: message.into(),
    }
}
fn field<'a>(node: &'a Value, key: &str) -> Result<&'a Text, CompileError> {
    node.get(key)
        .and_then(Value::as_text)
        .ok_or_else(|| syntax("Invalid public AST string field"))
}

fn check_input_budget(
    input: &[Value],
    from: Option<&str>,
    limits: &Limits,
) -> Result<(), CompileError> {
    fn depth(nodes: &[Value], current: usize, limit: usize) -> Result<(), CompileError> {
        compiler::check_limit(current, limit, "ast_depth")?;
        for node in nodes {
            if let Some(nodes) = node.get("nodes").and_then(Value::as_array) {
                depth(nodes, current.saturating_add(1), limit)?;
            }
        }
        Ok(())
    }
    let from_bytes = from.map_or(0, str::len);
    compiler::check_limit(from_bytes, limits.input_bytes, "input_bytes")?;
    let mut memo = BTreeMap::new();
    let mut bytes = 2usize
        .saturating_add(input.len().saturating_sub(1))
        .saturating_add(from_bytes);
    compiler::check_limit(bytes, limits.input_bytes, "input_bytes")?;
    for node in input {
        bytes = bytes.saturating_add(json_size(node, &mut memo, limits.input_bytes)?);
        compiler::check_limit(bytes, limits.input_bytes, "input_bytes")?;
    }
    depth(input, 0, limits.ast_depth)
}

// Measure the expanded wire representation without materializing it. Memoizing
// storage (rather than semantic identity) preserves distinct copy-on-write views.
fn json_size(
    value: &Value,
    memo: &mut BTreeMap<(u8, usize), usize>,
    limit: usize,
) -> Result<usize, CompileError> {
    fn string_size(value: &Text) -> usize {
        char::decode_utf16(value.units().iter().copied()).fold(2usize, |size, point| {
            size.saturating_add(match point {
                Ok('"' | '\\' | '\u{08}' | '\t' | '\n' | '\u{0c}' | '\r') => 2,
                Ok(ch) if ch <= '\u{1f}' => 6,
                Ok(ch) => ch.len_utf8(),
                Err(_) => 6,
            })
        })
    }
    let id = match value {
        Value::Array(value) => Some((0, Arc::as_ptr(value) as usize)),
        Value::Object(value) => Some((1, Arc::as_ptr(value) as usize)),
        _ => None,
    };
    if let Some(size) = id.and_then(|id| memo.get(&id)) {
        return Ok(*size);
    }
    let size = match value {
        Value::Null | Value::Bool(true) => 4,
        Value::Bool(false) => 5,
        Value::Number(value) => value.to_string().len(),
        Value::String(value) => string_size(value),
        Value::Array(values) => {
            let mut size = 2usize.saturating_add(values.len().saturating_sub(1));
            for value in values.iter() {
                size = size.saturating_add(json_size(value, memo, limit)?);
                compiler::check_limit(size, limit, "input_bytes")?;
            }
            size
        }
        Value::Object(values) => {
            let mut size = 2usize.saturating_add(values.len().saturating_sub(1));
            for (key, value) in values.iter() {
                size = size
                    .saturating_add(string_size(key))
                    .saturating_add(1)
                    .saturating_add(json_size(value, memo, limit)?);
                compiler::check_limit(size, limit, "input_bytes")?;
            }
            size
        }
    };
    compiler::check_limit(size, limit, "input_bytes")?;
    if let Some(id) = id {
        memo.insert(id, size);
    }
    Ok(size)
}

fn value_owned_bytes(value: &Value) -> usize {
    value.owned_bytes()
}
fn declaration_cache_bytes(value: &Value) -> usize {
    std::mem::size_of::<(usize, Arc<()>, Value)>()
        .saturating_add(5 * std::mem::size_of::<usize>())
        .saturating_add(value_owned_bytes(value))
}
type LocationKey = (usize, usize, usize, bool);
struct OutputState<'a> {
    used: BTreeSet<(OriginKey, usize)>,
    projection: compiler::origin::Projection,
    locations: &'a mut BTreeMap<LocationKey, Value>,
}
impl<'a> OutputState<'a> {
    fn new(
        nodes: &[AstNode],
        locations: &'a mut BTreeMap<LocationKey, Value>,
        published: &BTreeMap<LocationKey, Value>,
    ) -> Self {
        fn bind(
            nodes: &[AstNode],
            projection: &mut compiler::origin::Projection,
            locations: &BTreeMap<LocationKey, Value>,
            published: &BTreeMap<LocationKey, Value>,
        ) {
            for node in nodes {
                if node.origin().is_some_and(|origin| !origin.is_caller()) {
                    for (destination, location) in
                        [(false, node.source()), (true, node.destination())]
                    {
                        if let Some(location) = location {
                            let key = (
                                Arc::as_ptr(&location.source) as usize,
                                location.start,
                                location.end,
                                destination,
                            );
                            if let (Some(original), Some(current)) = (
                                location.public.as_deref(),
                                locations.get(&key).or_else(|| published.get(&key)),
                            ) {
                                projection.bind_location(original, current);
                            }
                        }
                    }
                }
                if let Some(children) = node.children() {
                    bind(children, projection, locations, published);
                }
            }
        }
        let mut projection = compiler::origin::Projection::new(nodes);
        bind(nodes, &mut projection, locations, published);
        Self {
            used: BTreeSet::new(),
            projection,
            locations,
        }
    }
}

impl Origins {
    fn bind_locations(&mut self, nodes: &[AstNode]) {
        fn bind(origins: &mut Origins, nodes: &[AstNode], sources: &mut BTreeMap<usize, Value>) {
            for node in nodes {
                for (destination, location) in [(false, node.source()), (true, node.destination())]
                {
                    if let Some(location) = location {
                        if location.public.is_some() {
                            continue;
                        }
                        let id = Arc::as_ptr(&location.source) as usize;
                        origins
                            .source_owners
                            .entry(id)
                            .or_insert_with(|| location.source.clone());
                        let source = sources.entry(id).or_insert_with(|| public_json!({"file":location.source.file, "code":location.source.code}));
                        origins
                            .source_locations
                            .entry((id, location.start, location.end, destination))
                            .or_insert_with(|| {
                                public_json!([source.clone(), location.start, location.end])
                            });
                    }
                }
                if let Some(children) = node.children() {
                    bind(origins, children, sources);
                }
            }
        }
        bind(self, nodes, &mut BTreeMap::new());
    }

    fn location<'a>(
        &'a self,
        location: &'a super::SourceLocation,
        destination: bool,
    ) -> Option<&'a Value> {
        if let Some(public) = &location.public {
            return Some(public);
        }
        self.source_locations.get(&(
            Arc::as_ptr(&location.source) as usize,
            location.start,
            location.end,
            destination,
        ))
    }

    fn owned_bytes(&self, seen: &mut BTreeSet<(u8, usize)>) -> usize {
        let authored = self
            .authored
            .iter()
            .map(|(key, origins)| {
                key.owned_bytes().saturating_add(
                    origins
                        .iter()
                        .map(|origin| {
                            std::mem::size_of::<AuthoredOrigin>()
                                .saturating_add(super::design_system::json_bytes_seen(
                                    &origin.fields,
                                    seen,
                                ))
                                .saturating_add(
                                    origin
                                        .declarations
                                        .len()
                                        .saturating_mul(5 * std::mem::size_of::<usize>()),
                                )
                                .saturating_add(origin.shape.as_ref().map_or(0, Text::owned_bytes))
                        })
                        .sum::<usize>(),
                )
            })
            .sum::<usize>();
        let declarations = self
            .declarations
            .values()
            .chain(self.generated_declarations.values())
            .map(|(_, value)| {
                std::mem::size_of::<(usize, Arc<()>, Value)>()
                    .saturating_add(5 * std::mem::size_of::<usize>())
                    .saturating_add(super::design_system::json_bytes_seen(value, seen))
            })
            .sum::<usize>();
        authored
            .saturating_add(declarations)
            .saturating_add(
                self.source_owners
                    .values()
                    .map(|source| {
                        let entry = std::mem::size_of::<(usize, Arc<super::Source>)>()
                            + 5 * std::mem::size_of::<usize>();
                        if seen.insert((5, Arc::as_ptr(source) as usize)) {
                            entry
                                .saturating_add(
                                    std::mem::size_of::<super::Source>()
                                        + 2 * std::mem::size_of::<usize>(),
                                )
                                .saturating_add(source.code.owned_bytes())
                                .saturating_add(source.file.as_ref().map_or(0, String::capacity))
                        } else {
                            entry
                        }
                    })
                    .sum::<usize>(),
            )
            .saturating_add(
                self.source_locations
                    .values()
                    .map(|value| {
                        std::mem::size_of::<((usize, usize, usize, bool), Value)>()
                            .saturating_add(5 * std::mem::size_of::<usize>())
                            .saturating_add(super::design_system::json_bytes_seen(value, seen))
                    })
                    .sum::<usize>(),
            )
            .saturating_add(
                self.theme_declarations
                    .iter()
                    .map(|(key, value)| {
                        key.owned_bytes()
                            .saturating_add(5 * std::mem::size_of::<usize>())
                            .saturating_add(super::design_system::json_bytes_seen(value, seen))
                    })
                    .sum::<usize>(),
            )
            .saturating_add(
                self.theme_rule
                    .iter()
                    .chain(self.theme_node.iter())
                    .chain(self.utilities.iter())
                    .map(|value| {
                        std::mem::size_of::<Value>()
                            .saturating_add(super::design_system::json_bytes_seen(value, seen))
                    })
                    .sum::<usize>(),
            )
    }
    fn bind_imported(&mut self, nodes: &mut [AstNode], input: &PublicAst) {
        for (index, (node, value)) in nodes.iter_mut().zip(input.iter()).enumerate() {
            node.set_origin(Some(compiler::ast::NodeOrigin::caller(value.clone())));
            for (destination, field) in [(false, "src"), (true, "dst")] {
                let location = if destination {
                    node.destination()
                } else {
                    node.source()
                };
                if let Some(location) = location {
                    let mut location = location.clone();
                    location.public = value.get(field).cloned().map(Arc::new);
                    if destination {
                        node.set_destination(Some(location));
                    } else {
                        node.set_source(Some(location));
                    }
                }
            }
            match node {
                AstNode::Declaration { identity, .. } => {
                    self.declarations.insert(
                        Arc::as_ptr(identity) as usize,
                        (identity.clone(), value.clone()),
                    );
                }
                AstNode::AtRule { name, .. } if name == "@apply" => {
                    if node.source().is_none() {
                        node.set_source(Some(caller_source(undefined())));
                    }
                    if let Some(owner) = node.source().and_then(|source| source.public.as_ref()) {
                        self.apply_inputs.insert(
                            Arc::as_ptr(owner) as usize,
                            (owner.clone(), input.clone(), index),
                        );
                        self.apply_nodes
                            .entry(Arc::as_ptr(value.as_object().unwrap().identity()) as usize)
                            .or_default()
                            .push(owner.clone());
                    }
                }
                _ => {}
            }
            if let (Some(children), Some(Value::Array(values))) =
                (node.children_mut(), value.get("nodes"))
            {
                self.bind_imported(children, values);
            }
        }
    }

    fn rebind_apply_inputs(&mut self, input: &PublicAst, views: &ImportViews) {
        fn visit(
            origins: &mut Origins,
            input: &PublicAst,
            views: &ImportViews,
            occurrences: &mut BTreeMap<usize, usize>,
        ) {
            for (index, value) in views.nodes(input).iter().enumerate() {
                let value = views.node(value);
                if value["kind"] == "at-rule" && value["name"] == "@apply" {
                    let id = Arc::as_ptr(value.as_object().unwrap().identity()) as usize;
                    let occurrence = occurrences.entry(id).or_default();
                    if let Some(owner) = origins
                        .apply_nodes
                        .get(&id)
                        .and_then(|owners| owners.get(*occurrence))
                    {
                        if let Some((_, parent, position)) =
                            origins.apply_inputs.get_mut(&(Arc::as_ptr(owner) as usize))
                        {
                            *parent = input.clone();
                            *position = index;
                        }
                    }
                    *occurrence += 1;
                }
                if let Some(Value::Array(children)) = value.get("nodes") {
                    visit(origins, children, views, occurrences);
                }
            }
        }
        visit(self, input, views, &mut BTreeMap::new());
        self.apply_nodes.clear();
    }

    fn convert(
        &mut self,
        input: &PublicAst,
        parent: Option<&Text>,
        depth: usize,
        limit: usize,
    ) -> Result<Vec<AstNode>, CompileError> {
        self.convert_inner(input, parent, depth, limit, None, None)
    }
    fn convert_current(
        &mut self,
        input: &PublicAst,
        current: &Arc<Current>,
        limit: usize,
    ) -> Result<Vec<AstNode>, CompileError> {
        self.convert_inner(input, None, 0, limit, Some(current), Some(current))
    }
    fn register_definitions(&mut self, definitions: &[(String, bool, Value)]) {
        for (index, (_, _, node)) in definitions.iter().enumerate() {
            let identity = Arc::as_ptr(node.as_object().unwrap().identity()) as usize;
            self.utility_definitions
                .entry(identity)
                .or_default()
                .push(index);
        }
    }
    fn lower_definitions(
        &mut self,
        definitions: Vec<(String, bool, Value)>,
        completions: Vec<compiler::custom_utilities::CompletionMetadata>,
        current: &Arc<Current>,
        limit: usize,
    ) -> Result<Vec<compiler::custom_utilities::RegisteredDefinition>, CompileError> {
        let definitions = definitions
            .into_iter()
            .zip(completions)
            .map(|((name, functional, node), completion)| {
                let node = current.value(&node);
                let Value::Array(children) = &node["nodes"] else {
                    unreachable!("validated definition body")
                };
                Ok(compiler::custom_utilities::RegisteredDefinition {
                    name,
                    functional,
                    completion,
                    nodes: self.convert_current(children, current, limit)?,
                    origin: Some(compiler::ast::NodeOrigin::caller_with_current(
                        node.clone(),
                        current.clone(),
                    )),
                })
            })
            .collect();
        self.utility_definitions.clear();
        self.utility_occurrences.clear();
        definitions
    }
    fn reset_lowering(&mut self) {
        self.declarations.clear();
        self.authored.clear();
        self.apply_inputs.clear();
        self.apply_nodes.clear();
        self.utilities = None;
        self.theme_rule = None;
    }
    fn convert_inner(
        &mut self,
        input: &PublicAst,
        parent: Option<&Text>,
        depth: usize,
        limit: usize,
        current: Option<&Current>,
        snapshot: Option<&Arc<Current>>,
    ) -> Result<Vec<AstNode>, CompileError> {
        compiler::check_limit(depth, limit, "ast_depth")?;
        let mut output = Vec::new();
        let values = current.map_or(input.as_slice(), |current| current.array(input).as_slice());
        for (index, original) in values.iter().enumerate() {
            let node = current.map_or(original, |current| current.value(original));
            let children = |origins: &mut Self, parent| {
                let Some(Value::Array(nodes)) = node.get("nodes") else {
                    return Err(syntax("Invalid public AST nodes"));
                };
                origins.convert_inner(nodes, parent, depth + 1, limit, current, snapshot)
            };
            let mut core = match literal(field(node, "kind")?) {
                "rule" => {
                    let selector = field(node, "selector")?;
                    if self
                        .theme_node
                        .as_ref()
                        .is_some_and(|theme| compiler::registration::same_object(theme, node))
                    {
                        AstNode::AtRule {
                            origin: None,
                            source: None,
                            destination: None,
                            name: "@rustify-theme-anchor".into(),
                            params: Text::default(),
                            nodes: Some(children(self, Some(selector))?),
                        }
                    } else {
                        AstNode::Rule {
                            origin: None,
                            source: None,
                            destination: None,
                            selector: selector.into(),
                            nodes: children(self, Some(selector))?,
                        }
                    }
                }
                "at-rule" => {
                    let name = field(node, "name")?;
                    if current.is_some() && name == "@utility" {
                        let identity = Arc::as_ptr(node.as_object().unwrap().identity()) as usize;
                        if let Some(indices) = self.utility_definitions.get(&identity) {
                            let occurrence = self.utility_occurrences.entry(identity).or_default();
                            let index = indices[*occurrence];
                            *occurrence += 1;
                            output.push(compiler::custom_utilities::marker(index));
                            continue;
                        }
                    }
                    if name == "@tailwind" && field(node, "params")?.starts_with("utilities") {
                        self.utilities = node.get("src").cloned();
                    }
                    let nodes = children(self, parent)?;
                    let source = (name == "@theme"
                        || name == "@apply"
                        || name == "@tailwind" && field(node, "params")?.starts_with("utilities"))
                    .then(|| caller_source(node.get("src").cloned().unwrap_or_else(undefined)));
                    if name == "@apply" {
                        if let Some(owner) =
                            source.as_ref().and_then(|source| source.public.as_ref())
                        {
                            self.apply_inputs.insert(
                                Arc::as_ptr(owner) as usize,
                                (owner.clone(), input.clone(), index),
                            );
                            self.apply_nodes
                                .entry(Arc::as_ptr(node.as_object().unwrap().identity()) as usize)
                                .or_default()
                                .push(owner.clone());
                        }
                    }
                    AstNode::AtRule {
                        origin: None,
                        // The typed span tracks relative candidate offsets; the
                        // public tuple retains the caller's full Source graph.
                        source,
                        destination: None,
                        name: name.into(),
                        params: if current.is_some()
                            && name == "@tailwind"
                            && field(node, "params")?.starts_with("utilities")
                        {
                            "utilities".into()
                        } else {
                            field(node, "params")?.into()
                        },
                        nodes: (name == "@theme" || !nodes.is_empty()).then_some(nodes),
                    }
                }
                "declaration" => {
                    // Undefined declarations are discarded by optimizeAst. The
                    // untouched public input remains available for Features.None.
                    if node.get("value") == Some(&undefined()) {
                        continue;
                    }
                    let important = node
                        .get("important")
                        .and_then(Value::as_bool)
                        .ok_or_else(|| syntax("Invalid public AST important flag"))?;
                    let identity = Arc::new(());
                    self.declarations.insert(
                        Arc::as_ptr(&identity) as usize,
                        (identity.clone(), node.clone()),
                    );
                    AstNode::Declaration {
                        origin: None,
                        source: node.get("src").cloned().map(caller_source),
                        destination: None,
                        property: field(node, "property")?.into(),
                        value: field(node, "value")?.into(),
                        important,
                        identity,
                    }
                }
                "comment" => AstNode::Comment {
                    origin: None,
                    source: None,
                    destination: None,
                    value: field(node, "value")?.into(),
                },
                "context" => {
                    let context = node
                        .get("context")
                        .and_then(Value::as_object)
                        .ok_or_else(|| syntax("Invalid public AST context"))?;
                    AstNode::Context {
                        origin: None,
                        base: context
                            .get(&Text::from("base"))
                            .and_then(Value::as_text)
                            .cloned()
                            .unwrap_or_default(),
                        source_base: context
                            .get(&Text::from("sourceBase"))
                            .and_then(Value::as_text)
                            .cloned(),
                        reference: context
                            .get(&Text::from("reference"))
                            .and_then(Value::as_bool)
                            .unwrap_or(false),
                        nodes: children(self, parent)?,
                    }
                }
                "at-root" => AstNode::AtRoot {
                    origin: None,
                    nodes: children(self, parent)?,
                },
                _ => return Err(syntax("Invalid public AST kind")),
            };
            core.set_origin(Some(match snapshot {
                Some(current) => {
                    compiler::ast::NodeOrigin::caller_with_current(node.clone(), (*current).clone())
                }
                None => compiler::ast::NodeOrigin::caller(node.clone()),
            }));
            if core.source().is_none() {
                core.set_source(node.get("src").cloned().map(caller_source));
            }
            core.set_destination(node.get("dst").cloned().map(caller_source));
            if current.is_some()
                && matches!(&core, AstNode::AtRule { name, params, .. } if name == "@tailwind" && params.starts_with("utilities"))
            {
                core.set_destination(None);
            }
            if let Some(key) = OriginKey::of(&core) {
                self.authored.entry(key).or_default().push(AuthoredOrigin {
                    fields: node.clone(),
                    declarations: BTreeSet::new(),
                    shape: None,
                });
            }
            output.push(core);
        }
        Ok(output)
    }

    // Initialization may replace @apply children. Bind each authored occurrence
    // to the declaration identities that survive those transformations, before
    // render prunes empty/reference rules or flattens wrapper contexts.
    fn bind_sources(&mut self, nodes: &[AstNode]) {
        fn bind(
            origins: &mut Origins,
            nodes: &[AstNode],
            occurrences: &mut BTreeMap<OriginKey, usize>,
        ) {
            for node in nodes {
                if let Some(children) = node.children() {
                    bind(origins, children, occurrences);
                }
                if let Some(key) = OriginKey::of(node) {
                    let index = occurrences.entry(key.clone()).or_default();
                    if let Some(origin) = origins
                        .authored
                        .get_mut(&key)
                        .and_then(|entries| entries.get_mut(*index))
                    {
                        declaration_ids(std::slice::from_ref(node), &mut origin.declarations);
                        origin.shape = Some(compiler::ast::to_css(std::slice::from_ref(node)));
                    }
                    *index += 1;
                }
            }
        }
        bind(self, nodes, &mut BTreeMap::new());
    }

    fn output(
        &self,
        nodes: &[AstNode],
        inherited: Option<&Value>,
        from: bool,
        staged: &mut BTreeMap<usize, (Arc<()>, Value)>,
        locations: &mut BTreeMap<LocationKey, Value>,
    ) -> Result<Vec<Value>, CompileError> {
        self.output_inner(
            nodes,
            inherited,
            from,
            true,
            &mut OutputState::new(nodes, locations, &self.source_locations),
            staged,
        )
    }

    fn output_raw(
        &self,
        nodes: &[AstNode],
        inherited: Option<&Value>,
        from: bool,
        staged: &mut BTreeMap<usize, (Arc<()>, Value)>,
        locations: &mut BTreeMap<LocationKey, Value>,
    ) -> Result<Vec<Value>, CompileError> {
        self.output_inner(
            nodes,
            inherited,
            from,
            false,
            &mut OutputState::new(nodes, locations, &self.source_locations),
            staged,
        )
    }

    fn output_inner(
        &self,
        nodes: &[AstNode],
        inherited: Option<&Value>,
        from: bool,
        optimized: bool,
        state: &mut OutputState<'_>,
        staged: &mut BTreeMap<usize, (Arc<()>, Value)>,
    ) -> Result<Vec<Value>, CompileError> {
        let mut result = Vec::new();
        for node in nodes {
            let exact = node.origin().map(|origin| state.projection.fields(origin));
            let original = exact.as_ref().or_else(|| match node {
                AstNode::Declaration { identity, .. } => self
                    .declarations
                    .get(&(Arc::as_ptr(identity) as usize))
                    .map(|(_, value)| value),
                _ => OriginKey::of(node).and_then(|key| {
                    let entries = self.authored.get(&key)?;
                    let mut declarations = BTreeSet::new();
                    declaration_ids(std::slice::from_ref(node), &mut declarations);
                    let shape = compiler::ast::to_css(std::slice::from_ref(node));
                    let (index, origin) = entries
                        .iter()
                        .enumerate()
                        .filter(|(index, origin)| {
                            !state.used.contains(&(key.clone(), *index))
                                && (origin
                                    .declarations
                                    .iter()
                                    .any(|id| declarations.contains(id))
                                    || origin.shape.as_ref() == Some(&shape))
                        })
                        .max_by_key(|(index, origin)| {
                            (
                                origin.shape.as_ref() == Some(&shape),
                                origin.declarations.intersection(&declarations).count(),
                                std::cmp::Reverse(*index),
                            )
                        })?;
                    state.used.insert((key, index));
                    Some(&origin.fields)
                }),
            });
            let source = original
                .and_then(|original| original.get("src"))
                .or_else(|| if original.is_none() { inherited } else { None });
            let mut value = match node {
                AstNode::Rule {
                    selector, nodes, ..
                } => {
                    let theme = selector == ":root, :host" && self.theme_rule.is_some();
                    let mut value = original.cloned().unwrap_or_else(|| public_json!({"kind":"rule", "selector":selector, "nodes":public_json!([])}));
                    value["kind"] = public_json!("rule");
                    value["selector"] = public_json!(selector);
                    value["nodes"] = public_json!(
                        self.output_inner(nodes, source, from, optimized, state, staged)?
                    );
                    if theme {
                        value["src"] = self.theme_rule.clone().unwrap_or_else(undefined);
                        for (node, declaration) in nodes.iter().zip(
                            value["nodes"]
                                .as_array_mut()
                                .ok_or_else(|| syntax("Invalid theme output"))?,
                        ) {
                            if let Some(cached) = declaration["property"]
                                .as_text()
                                .and_then(|property| self.theme_declarations.get(property))
                            {
                                let mut stable = cached.clone();
                                stable["value"] = declaration["value"].clone();
                                *declaration = stable;
                                if let AstNode::Declaration { identity, .. } = node {
                                    staged.remove(&(Arc::as_ptr(identity) as usize));
                                }
                            }
                        }
                    }
                    value
                }
                AstNode::AtRule {
                    name,
                    params,
                    nodes,
                    ..
                } => {
                    let mut value = original.cloned().unwrap_or_else(|| public_json!({"kind":"at-rule", "name":name, "params":params, "nodes":public_json!([])}));
                    value["name"] = public_json!(name);
                    value["params"] = public_json!(params);
                    value["nodes"] = public_json!(self.output_inner(
                        nodes.as_deref().unwrap_or_default(),
                        source,
                        from,
                        optimized,
                        state,
                        staged
                    )?);
                    value
                }
                AstNode::Declaration {
                    property,
                    value,
                    important,
                    identity,
                    ..
                } => {
                    let mut output = original.cloned().or_else(|| self.generated_declarations.get(&(Arc::as_ptr(identity) as usize)).map(|(_, value)| value.clone())).or_else(|| staged.get(&(Arc::as_ptr(identity) as usize)).map(|(_, value)| value.clone()))
                        .unwrap_or_else(|| public_json!({"kind":"declaration", "property":property, "value":value, "important":important}));
                    output["property"] = public_json!(property);
                    output["value"] = public_json!(value);
                    output["important"] = public_json!(important);
                    output
                }
                AstNode::Comment { value, .. } => original
                    .cloned()
                    .unwrap_or_else(|| public_json!({"kind":"comment", "value":value})),
                AstNode::AtRoot { origin: _, nodes } => {
                    let mut value = original.cloned().unwrap_or_else(
                        || public_json!({"kind":"at-root", "nodes":public_json!([])}),
                    );
                    value["kind"] = public_json!("at-root");
                    value["nodes"] = public_json!(
                        self.output_inner(nodes, source, from, optimized, state, staged)?
                    );
                    // At-root has no typed span. @apply stamps its whole cloned
                    // subtree with one candidate tuple, including this wrapper.
                    fn public_source(nodes: &[AstNode]) -> Option<&Value> {
                        nodes.iter().find_map(|node| {
                            node.source()
                                .and_then(|source| source.public.as_deref())
                                .or_else(|| node.children().and_then(public_source))
                        })
                    }
                    if let Some(source) = public_source(nodes) {
                        value["src"] = source.clone();
                    }
                    value
                }
                AstNode::Context {
                    base,
                    source_base,
                    reference,
                    nodes,
                    ..
                } => {
                    let mut context = public_json!({});
                    if !base.is_empty() {
                        context["base"] = public_json!(base);
                    }
                    if *reference {
                        context["reference"] = public_json!(true);
                    }
                    if let Some(source_base) = source_base {
                        context["sourceBase"] = public_json!(source_base);
                    }
                    let mut value = original.cloned().unwrap_or_else(|| public_json!({"kind":"context", "context":context, "nodes":public_json!([])}));
                    value["nodes"] = public_json!(
                        self.output_inner(nodes, source, from, optimized, state, staged)?
                    );
                    value
                }
            };
            if let Some(source) = source {
                value["src"] = source.clone();
            } else if from
                && (original.is_none() || node.origin().is_some_and(|origin| !origin.is_caller()))
                && value.get("src").is_none()
                && !matches!(node, AstNode::Comment { .. })
            {
                value["src"] = self.utilities.clone().unwrap_or_else(undefined);
            }
            if let Some(dst) = original.and_then(|original| original.get("dst")) {
                value["dst"] = state.projection.location(node, dst);
            }
            for (destination, location) in [(false, node.source()), (true, node.destination())] {
                let Some(location) = location else { continue };
                let Some(public) = self.location(location, destination) else {
                    continue;
                };
                let key = (
                    Arc::as_ptr(&location.source) as usize,
                    location.start,
                    location.end,
                    destination,
                );
                let projected = state
                    .locations
                    .get(&key)
                    .or_else(|| self.source_locations.get(&key))
                    .cloned()
                    .unwrap_or_else(|| state.projection.location(node, public));
                if node.origin().is_some_and(|origin| !origin.is_caller())
                    && !self.source_locations.contains_key(&key)
                {
                    state.locations.insert(key, projected.clone());
                }
                value[if destination { "dst" } else { "src" }] = projected;
            }
            if let AstNode::Declaration { identity, .. } = node {
                if original.is_none() || node.origin().is_some_and(|origin| !origin.is_caller()) {
                    staged.insert(
                        Arc::as_ptr(identity) as usize,
                        (identity.clone(), value.clone()),
                    );
                }
            }
            if !optimized {
                if let Some(original) = original.and_then(|value| value.get("nodes")) {
                    if value.get("nodes") == Some(original) {
                        value["nodes"] = original.clone();
                    }
                }
            }
            if optimized && matches!(node, AstNode::Rule { .. } | AstNode::AtRule { .. }) {
                if let Value::Object(fields) = value {
                    value = Value::Object(Arc::new(
                        fields
                            .iter()
                            .map(|(key, value)| (key.clone(), value.clone()))
                            .collect(),
                    ));
                }
            }
            result.push(value);
        }
        Ok(result)
    }
}

pub fn compile_ast(
    input: PublicAst,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<AstCompiler, CompileError> {
    match compile_ast_setup(input, options, assets, limits)? {
        AstSetupOutcome::Complete(compiler) => Ok(*compiler),
        AstSetupOutcome::Failed(failure) => Err(failure.error),
    }
}

/// Constructs an AST compiler or returns its retained semantic failure effects.
/// Admission/limit failures return `Err` and do not authorize any host writes.
pub fn compile_ast_setup(
    input: PublicAst,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<AstSetupOutcome, CompileError> {
    compiler::validate_limits(&limits)?;
    check_input_budget(&input, options.from.as_deref(), &limits)?;
    let from = options.from.as_ref().is_some_and(|from| !from.is_empty());
    let mut origins = Origins {
        utility_node: caller_utilities(&input, false, &ImportViews::default()),
        ..Origins::default()
    };
    let core = origins.convert(&input, None, 0, limits.ast_depth)?;
    let has_loader = !assets.is_empty();
    let mut import_graph = ImportGraph {
        root: input.clone(),
        base: options.base.as_deref().unwrap_or("").into(),
        origins: &mut origins,
        views: ImportViews::new(limits.cache_bytes),
        initial_mutations: Vec::new(),
        apply_failure_mutations: Vec::new(),
        mutations: Vec::new(),
        import_program: None,
        setup_compatibility: None,
    };
    let result = Compiler::new_with_ast(
        core,
        &assets,
        limits.clone(),
        options,
        None,
        Some(&mut import_graph),
    )
    .map_err(|error| {
        if !has_loader && matches!(error, CompileError::MissingAsset { .. }) {
            syntax("No `loadStylesheet` function provided to `compile`")
        } else {
            error
        }
    });
    let ImportGraph {
        views: import_views,
        initial_mutations: mut import_mutations,
        apply_failure_mutations,
        mutations,
        import_program,
        setup_compatibility,
        ..
    } = import_graph;
    import_mutations.extend(mutations);
    let mut compiler = match result {
        Ok(compiler) => compiler,
        Err(error) => {
            if matches!(error, CompileError::Limit { .. }) {
                return Err(error);
            }
            import_mutations.extend(apply_failure_mutations);
            let mut seen = BTreeSet::new();
            let cache_bytes =
                super::design_system::json_bytes_seen(&Value::Array(input.clone()), &mut seen)
                    .saturating_add(origins.owned_bytes(&mut seen))
                    .saturating_add(mutation_bytes(&import_mutations, &mut seen))
                    .saturating_add(
                        import_program
                            .as_ref()
                            .map_or(0, compiler::imports::schedule::Program::owned_bytes),
                    )
                    .saturating_add(
                        import_program
                            .as_ref()
                            .map_or(0, |program| program.graph_bytes(&mut seen)),
                    );
            compiler::check_limit(cache_bytes, limits.cache_bytes, "cache_bytes")?;
            return Ok(AstSetupOutcome::Failed(AstSetupFailure {
                error,
                input_mutations: import_mutations,
                cache_bytes,
                import_program,
                setup_compatibility,
            }));
        }
    };
    origins.bind_sources(&compiler.input_ast);
    origins.bind_locations(&compiler.input_ast);
    if !compiler.features().contains(Features::UTILITIES) {
        origins.utility_node = None;
    }
    if let Some(node) = &origins.utility_node {
        origins.utilities = node.get("src").cloned();
    }
    let utility_mutations = origins
        .utility_node
        .as_ref()
        .map(|node| {
            vec![
                InputMutation::Set {
                    target: node.clone(),
                    field: "kind".into(),
                    value: public_json!("context"),
                },
                InputMutation::Set {
                    target: node.clone(),
                    field: "context".into(),
                    value: public_json!({}),
                },
            ]
        })
        .unwrap_or_default();
    let mut input_mutations = Vec::new();
    let mut positions = apply_input_mutations(
        &mut origins,
        &input,
        &import_views,
        &mut input_mutations,
        std::mem::take(&mut compiler.apply_replacements),
    )?;
    input_mutations.extend(utility_mutations);
    remove_utility_inputs(&input, &mut positions, &mut input_mutations, &import_views);
    input_mutations.splice(0..0, import_mutations);
    let banner = public_json!({"kind":"comment", "value": "! tailwindcss v4.1.13 | MIT License | https://tailwindcss.com "});
    let mut seen = BTreeSet::new();
    let retained_bytes =
        super::design_system::json_bytes_seen(&Value::Array(input.clone()), &mut seen)
            .saturating_add(origins.owned_bytes(&mut seen))
            .saturating_add(mutation_bytes(&input_mutations, &mut seen))
            .saturating_add(banner.owned_bytes())
            .saturating_add(
                import_program
                    .as_ref()
                    .map_or(0, compiler::imports::schedule::Program::owned_bytes),
            )
            .saturating_add(
                import_program
                    .as_ref()
                    .map_or(0, |program| program.graph_bytes(&mut seen)),
            );
    compiler.sources_cache_bytes = compiler.sources_cache_bytes.saturating_add(retained_bytes);
    compiler.committed.metrics.cache_bytes = compiler
        .committed
        .metrics
        .cache_bytes
        .saturating_add(retained_bytes);
    compiler::check_limit(
        compiler.committed.metrics.cache_bytes,
        limits.cache_bytes,
        "cache_bytes",
    )?;
    Ok(AstSetupOutcome::Complete(Box::new(AstCompiler {
        compiler,
        input,
        compiled: None,
        banner,
        license_banner: true,
        origins,
        from,
        previous_node_count: 0,
        previous_variables: 0,
        input_mutations,
        import_program,
        setup_compatibility,
    })))
}

impl AstCompiler {
    /// Upstream omits the license comment in its own test environment.
    pub fn set_license_banner(&mut self, emit: bool) {
        self.license_banner = emit;
    }
    pub fn input_mutations(&self) -> &[InputMutation] {
        &self.input_mutations
    }
    pub fn retained_cache_bytes(&self) -> usize {
        self.compiler.committed.metrics.cache_bytes
    }
    /// A host crossing error keeps admitted candidates and counters, while the
    /// next no-op build reuses the last successfully published AST, as upstream.
    pub fn restore_output_after_crossing_error(&mut self, previous: Option<PublicAst>) {
        self.compiled = previous;
    }
    pub fn features(&self) -> Features {
        self.compiler.features()
    }
    pub fn sources(&self) -> &SourceMetadata {
        self.compiler.source_metadata()
    }
    pub fn stylesheet_loads(&self) -> &[StylesheetLoad] {
        self.compiler.stylesheet_loads()
    }
    pub fn warnings(&self) -> &[Text] {
        self.compiler.warnings()
    }

    pub fn build(&mut self, candidates: &[String]) -> Result<PublicAst, CompileError> {
        let candidates = candidates.iter().map(Text::from).collect::<Vec<_>>();
        self.build_text(&candidates)
    }
    pub fn build_text(&mut self, candidates: &[Text]) -> Result<PublicAst, CompileError> {
        self.build_text_with_output_check(candidates, |_, _| Ok(()))
    }
    pub fn build_text_with_output_check(
        &mut self,
        candidates: &[Text],
        check: impl FnOnce(&PublicAst, usize) -> Result<(), CompileError>,
    ) -> Result<PublicAst, CompileError> {
        self.build_text_with_input_check(candidates, |nodes, _, bytes| check(nodes, bytes))
    }
    /// Preflight both caller effects and optimized output before admission.
    pub fn build_text_with_input_check(
        &mut self,
        candidates: &[Text],
        check: impl FnOnce(&PublicAst, &[InputMutation], usize) -> Result<(), CompileError>,
    ) -> Result<PublicAst, CompileError> {
        if self.features() == Features::NONE {
            return Ok(self.input.clone());
        }
        let candidates = if self.features().contains(Features::UTILITIES) {
            candidates
        } else {
            &[]
        };
        let prepared = self.compiler.prepare_add_text(candidates)?;
        let nodes = prepared
            .cache
            .values()
            .map(|entry| match &entry.outcome {
                compiler::Outcome::Accepted(nodes) => nodes.len(),
                _ => 0,
            })
            .sum();
        let mut used_variables = BTreeSet::new();
        for entry in prepared.cache.values() {
            match &entry.outcome {
                compiler::Outcome::Accepted(nodes) => {
                    for node in nodes {
                        used_variables.extend(node.used_variables.iter());
                    }
                }
                compiler::Outcome::ExternalVariable(Some(variable)) => {
                    used_variables.insert(variable);
                }
                _ => {}
            }
        }
        let variables = used_variables.len();
        let mut staged = BTreeMap::new();
        let mut locations = BTreeMap::new();
        let mut raw_candidates = BTreeMap::new();
        let input_mutations =
            if nodes != self.previous_node_count || variables != self.previous_variables {
                if let Some(target) = &self.origins.utility_node {
                    let mut candidates: Vec<_> = prepared
                        .cache
                        .values()
                        .filter_map(|entry| match &entry.outcome {
                            compiler::Outcome::Accepted(nodes) => Some(nodes.as_slice()),
                            _ => None,
                        })
                        .flatten()
                        .collect();
                    candidates.sort_by(|a, b| compiler::compare_rules(a, b));
                    let mut raw = Vec::with_capacity(candidates.len());
                    for candidate in candidates {
                        let id = Arc::as_ptr(candidate) as usize;
                        let value = if let Some((_, value)) = self.origins.raw_candidates.get(&id) {
                            value.clone()
                        } else {
                            let value = match self.origins.output_raw(
                                std::slice::from_ref(&candidate.node),
                                None,
                                self.from,
                                &mut staged,
                                &mut locations,
                            ) {
                                Ok(mut values) => values.remove(0),
                                Err(error) => {
                                    self.compiler.discard(prepared)?;
                                    return Err(error);
                                }
                            };
                            raw_candidates.insert(id, (candidate.clone(), value.clone()));
                            value
                        };
                        raw.push(value);
                    }
                    vec![InputMutation::Set {
                        target: target.clone(),
                        field: "nodes".into(),
                        value: Value::Array(Arc::new(raw)),
                    }]
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };
        let output = if self.compiled.is_some()
            && nodes == self.previous_node_count
            && variables == self.previous_variables
        {
            self.compiled
                .as_ref()
                .cloned()
                .ok_or_else(|| syntax("Missing compiled AST"))
        } else {
            self.origins
                .output(&prepared.ast, None, self.from, &mut staged, &mut locations)
                .map(|mut nodes| {
                    if self.license_banner {
                        nodes.insert(0, self.banner.clone());
                    }
                    Arc::new(nodes)
                })
        };
        let output = match output {
            Ok(output) => output,
            Err(error) => {
                self.compiler.discard(prepared)?;
                return Err(error);
            }
        };
        if let Err(error) = compiler::check_limit(
            BANNER
                .len()
                .saturating_add(prepared.report().css.byte_len()),
            self.compiler.limits.css_bytes,
            "css_bytes",
        ) {
            self.compiler.discard(prepared)?;
            return Err(error);
        }
        let mut seen = BTreeSet::new();
        let _ = super::design_system::json_bytes_seen(&Value::Array(self.input.clone()), &mut seen);
        for (_, value) in self.origins.generated_declarations.values() {
            let _ = super::design_system::json_bytes_seen(value, &mut seen);
        }
        for (_, value) in self.origins.raw_candidates.values() {
            let _ = super::design_system::json_bytes_seen(value, &mut seen);
        }
        let raw_bytes = raw_candidates
            .values()
            .map(|(_, value)| {
                std::mem::size_of::<(usize, Arc<compiler::utilities::CompiledCandidate>, Value)>()
                    .saturating_add(5 * std::mem::size_of::<usize>())
                    .saturating_add(super::design_system::json_bytes_seen(value, &mut seen))
            })
            .sum::<usize>();
        let compiled_bytes =
            super::design_system::json_bytes_seen(&Value::Array(output.clone()), &mut seen)
                .saturating_add(mutation_bytes(&input_mutations, &mut seen));
        let location_bytes = locations.len().saturating_mul(
            std::mem::size_of::<(LocationKey, Value)>() + 5 * std::mem::size_of::<usize>(),
        );
        let generated_bytes = staged
            .iter()
            .map(|(key, (_, value))| {
                let previous = self
                    .origins
                    .generated_declarations
                    .get(key)
                    .map_or(0, |(_, value)| declaration_cache_bytes(value));
                declaration_cache_bytes(value).saturating_sub(previous)
            })
            .sum::<usize>()
            .saturating_add(location_bytes);
        if let Err(error) = compiler::check_limit(
            prepared
                .report()
                .metrics
                .cache_bytes
                .saturating_add(compiled_bytes)
                .saturating_add(raw_bytes)
                .saturating_add(generated_bytes),
            self.compiler.limits.cache_bytes,
            "cache_bytes",
        ) {
            self.compiler.discard(prepared)?;
            return Err(error);
        }
        if let Err(error) = check(
            &output,
            &input_mutations,
            prepared
                .report()
                .metrics
                .cache_bytes
                .saturating_add(compiled_bytes)
                .saturating_add(raw_bytes)
                .saturating_add(generated_bytes),
        ) {
            self.compiler.discard(prepared)?;
            return Err(error);
        }
        self.compiler.commit(prepared)?;
        self.compiler.committed.metrics.cache_bytes = self
            .compiler
            .committed
            .metrics
            .cache_bytes
            .saturating_add(compiled_bytes)
            .saturating_add(raw_bytes)
            .saturating_add(generated_bytes);
        self.compiler.sources_cache_bytes = self
            .compiler
            .sources_cache_bytes
            .saturating_add(raw_bytes)
            .saturating_add(generated_bytes);
        self.origins.generated_declarations.extend(staged);
        self.origins.source_locations.extend(locations);
        self.origins.raw_candidates.extend(raw_candidates);
        self.input_mutations = input_mutations;
        self.previous_node_count = nodes;
        self.previous_variables = variables;
        self.compiled = Some(output.clone());
        Ok(output)
    }
}

#[cfg(test)]
#[path = "../../../../../tests/utility/compiler/compile-ast-native.rs"]
mod tests;

#[cfg(test)]
mod canonical_text_tests {
    use super::*;
    use crate::compiler::imports::PublicSetup as _;

    fn import_graph(root: PublicAst, origins: &mut Origins, limit: usize) -> ImportGraph<'_> {
        ImportGraph {
            root,
            base: Text::default(),
            origins,
            views: ImportViews::new(limit),
            initial_mutations: Vec::new(),
            apply_failure_mutations: Vec::new(),
            mutations: Vec::new(),
            import_program: None,
            setup_compatibility: None,
        }
    }

    #[test]
    fn import_begin_and_complete_keep_raw_slots_context_and_captured_versions() {
        let omitted = public_json!({"kind":"declaration", "property":"color", "value":undefined(), "important":false});
        let import = public_json!({"kind":"at-rule", "name":"@import", "params":"\"one.css\"", "nodes":public_json!([])});
        let body = Arc::new(vec![omitted.clone(), import]);
        let left = public_json!({"kind":"context", "context":public_json!({"base":"", "reference":false}), "nodes":Value::Array(body.clone())});
        let right =
            public_json!({"kind":"rule", "selector":".alias", "nodes":Value::Array(body.clone())});
        let root = Arc::new(vec![left.clone(), right.clone()]);
        let mut origins = Origins::default();
        let core = origins.convert(&root, None, 0, 32).unwrap();
        assert_eq!(core[0].children().unwrap().len(), 1);
        let original = core[0].children().unwrap()[0].clone();
        let mut graph = import_graph(root, &mut origins, usize::MAX);
        let (context, begin) = graph.begin(&body, 1).unwrap();
        let first = graph.views.current.array(&body).clone();
        assert_eq!(graph.views.nodes(&body)[0], omitted);
        assert_eq!(graph.views.node(&left)["context"]["base"], public_json!(""));
        assert_eq!(
            graph.views.node(&left)["context"]["reference"],
            public_json!(false)
        );
        let mut loaded = compiler::parser::parse(".one { color: red; }", 32).unwrap();
        let values = graph.parsed(&mut loaded).unwrap();
        let wrapper = AstNode::Context {
            origin: None,
            base: "loaded".into(),
            source_base: None,
            reference: false,
            nodes: loaded,
        };
        let mut imported = AstNode::Context {
            origin: None,
            base: Text::default(),
            source_base: None,
            reference: false,
            nodes: vec![wrapper],
        };
        let completed = graph
            .complete(&body, 1, &original, &mut imported, &values, context.clone())
            .unwrap();
        let Value::Array(alias) = &graph.views.node(&right)["nodes"] else {
            panic!("shared body array")
        };
        assert_eq!(graph.views.nodes(alias), graph.views.nodes(&body));
        assert_eq!(graph.views.nodes(&first), graph.views.nodes(&body));
        assert!(begin.value()["nodes"].as_array().unwrap().is_empty());
        assert_eq!(graph.views.node(&context)["nodes"], *completed.value());
        assert_eq!(
            graph.views.node(&context)["nodes"][0]["context"]["base"],
            public_json!("loaded")
        );
        assert_eq!(
            Arc::as_ptr(graph.views.nodes(&body)[1].as_object().unwrap().identity()),
            Arc::as_ptr(context.as_object().unwrap().identity())
        );
        assert_eq!(body[0], omitted);
        assert_eq!(body[1]["name"], public_json!("@import"));
    }

    #[test]
    fn import_begin_refusal_preserves_the_original_receiver_and_program() {
        let body = Arc::new(vec![
            public_json!({"kind":"at-rule", "name":"@import", "params":"\"one.css\"", "nodes":public_json!([])}),
        ]);
        let mut origins = Origins::default();
        let mut graph = import_graph(body.clone(), &mut origins, 0);
        assert!(matches!(
            graph.begin(&body, 0),
            Err(CompileError::Limit { limit: 0, .. })
        ));
        assert_eq!(graph.views.nodes(&body), body.as_slice());
        assert!(graph.initial_mutations.is_empty() && graph.mutations.is_empty());
        assert!(graph.import_program.is_none());
    }

    #[test]
    fn import_views_follow_earlier_array_storage() {
        let original = Arc::new(vec![public_json!("original")]);
        let first = Arc::new(vec![public_json!("first")]);
        let latest = Arc::new(vec![public_json!("latest")]);
        let mut views = ImportViews::default();
        views.replace(&original, 0, 1, &first).unwrap();
        let first = views.current.array(&original).clone();
        views.replace(&first, 0, 1, &latest).unwrap();
        assert_eq!(views.nodes(&original), latest.as_slice());
        assert_eq!(views.nodes(&first), latest.as_slice());
        assert_eq!(original[0], public_json!("original"));
        assert_eq!(first[0], public_json!("first"));
    }

    #[test]
    fn source_location_cache_keeps_address_identity_alive() {
        let css = Text::from(".one { color: red; }");
        let source = Arc::new(super::super::Source {
            file: Some("one.css".into()),
            code: css.clone(),
        });
        let owner = Arc::downgrade(&source);
        let nodes = compiler::parser::parse_with_source(&css, 32, Some(source.clone())).unwrap();
        let mut origins = Origins::default();
        origins.bind_locations(&nodes);
        drop(nodes);
        drop(source);
        assert!(
            owner.upgrade().is_some(),
            "cached source address must not be reused after native nodes are removed"
        );
        drop(origins);
        assert!(owner.upgrade().is_none());
    }

    #[test]
    fn repeated_imports_publish_fresh_sources_and_share_their_output_declarations() {
        let import = public_json!({"kind":"at-rule", "name":"@import", "params":"\"one.css\"", "nodes":public_json!([])});
        let input = Arc::new(vec![import.clone(), import]);
        let mut api = compile_ast(
            input.clone(),
            CompileOptions {
                from: Some("entry.css".into()),
                ..CompileOptions::default()
            },
            vec![Stylesheet {
                id: "one.css".into(),
                path: Some("/mapped/one.css".into()),
                base: Some("/discover".into()),
                content: ".one { color: red; }".into(),
            }],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let declarations: Vec<_> = api
            .input_mutations()
            .iter()
            .filter_map(|mutation| match mutation {
                InputMutation::Set { field, value, .. } if field == "nodes" => {
                    Some(value[0]["nodes"][0]["nodes"][0].clone())
                }
                _ => None,
            })
            .collect();
        assert_eq!(declarations.len(), 2);
        let source = |value: &Value| value["src"][0].as_object().unwrap().identity().clone();
        assert!(!Arc::ptr_eq(
            &source(&declarations[0]),
            &source(&declarations[1])
        ));
        for declaration in &declarations {
            let (Value::Array(src), Value::Array(dst)) = (&declaration["src"], &declaration["dst"])
            else {
                panic!("locations")
            };
            assert!(!Arc::ptr_eq(src, dst));
            assert!(Arc::ptr_eq(
                &source(declaration),
                dst[0].as_object().unwrap().identity()
            ));
        }
        let output = api.build(&[]).unwrap();
        for (rule, declaration) in output
            .iter()
            .filter(|node| node["kind"] == "rule")
            .zip(declarations)
        {
            assert!(Arc::ptr_eq(
                rule["nodes"][0].as_object().unwrap().identity(),
                declaration.as_object().unwrap().identity()
            ));
        }
        assert_eq!(input[0]["name"], public_json!("@import"));
    }

    #[test]
    fn utility_removal_visits_shared_context_arrays_once() {
        let definition = public_json!({"kind":"at-rule", "name":"@utility", "params":"card", "nodes":public_json!([
            public_json!({"kind":"declaration", "property":"color", "value":"red", "important":false})
        ])});
        let children = Arc::new(vec![definition]);
        let context = public_json!({"kind":"context", "context":public_json!({}), "nodes":Value::Array(children.clone())});
        let api = compile_ast(
            Arc::new(vec![context.clone(), context]),
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let [InputMutation::Splice {
            target: Value::Array(target),
            start,
            delete_count,
            values,
        }] = api.input_mutations()
        else {
            panic!("one removal for the shared caller array")
        };
        assert!(Arc::ptr_eq(target, &children));
        assert_eq!((*start, *delete_count), (0, 1));
        assert!(values.as_array().unwrap().is_empty());
    }

    #[test]
    fn apply_setup_splices_raw_nodes_and_shares_optimized_declarations() {
        let apply = public_json!({"kind":"at-rule", "name":"@apply", "params":"size-[2px]", "nodes":public_json!([])});
        let children = Arc::new(vec![apply.clone()]);
        let input = Arc::new(vec![
            public_json!({"kind":"rule", "selector":".sheet", "nodes":Value::Array(children.clone())}),
        ]);
        let mut api = compile_ast(
            input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let InputMutation::Splice {
            target: Value::Array(target),
            start,
            delete_count,
            values,
        } = &api.input_mutations()[0]
        else {
            panic!("raw apply replacement")
        };
        assert!(Arc::ptr_eq(target, &children));
        assert_eq!((*start, *delete_count), (0, 1));
        assert_eq!(values[0]["property"], public_json!("--tw-sort"));
        assert_eq!(values[0]["value"], public_json!("size"));
        assert_eq!(values[1]["property"], public_json!("width"));
        assert_eq!(values[2]["property"], public_json!("height"));
        let width = values[1].clone();
        let output = api.build(&[]).unwrap();
        let sheet = output
            .iter()
            .find(|node| node["selector"] == ".sheet")
            .unwrap();
        assert_eq!(sheet["nodes"].as_array().unwrap().len(), 2);
        assert!(Arc::ptr_eq(
            width.as_object().unwrap().identity(),
            sheet["nodes"][0].as_object().unwrap().identity()
        ));
        assert_eq!(children[0], apply);
        assert!(api.compiler.apply_replacements.is_empty());
        assert!(api.origins.apply_inputs.is_empty());
    }

    #[test]
    fn candidate_cache_retains_unoptimized_children_shared_with_output() {
        let input = Arc::new(vec![
            public_json!({"kind":"at-rule", "name":"@utility", "params":"raw", "nodes":public_json!([
                public_json!({"kind":"declaration", "property":"--tw-sort", "value":"display", "important":false}),
                public_json!({"kind":"declaration", "property":"display", "value":"block", "important":false}),
                public_json!({"kind":"declaration", "property":"display", "value":"block", "important":false}),
                public_json!({"kind":"rule", "selector":"&", "nodes":public_json!([])})
            ])}),
            public_json!({"kind":"at-rule", "name":"@tailwind", "params":"utilities", "nodes":public_json!([])}),
        ]);
        let mut api = compile_ast(
            input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let output = api.build(&["raw".into()]).unwrap();
        let compiler::Outcome::Accepted(candidates) =
            &api.compiler.cache[&Text::from("raw")].outcome
        else {
            panic!("accepted custom utility")
        };
        let raw = candidates[0].node.children().unwrap();
        assert_eq!(raw.len(), 4);
        assert!(
            matches!(&raw[0], AstNode::Declaration { property, .. } if property == "--tw-sort")
        );
        assert!(
            matches!(&raw[3], AstNode::Rule { selector, nodes, .. } if selector == "&" && nodes.is_empty())
        );
        let optimized = output
            .iter()
            .find(|node| node["selector"] == ".raw")
            .unwrap()["nodes"]
            .as_array()
            .unwrap();
        assert_eq!(optimized.len(), 1);
        let AstNode::Declaration { identity, .. } = &raw[2] else {
            panic!("raw declaration")
        };
        assert!(Arc::ptr_eq(
            optimized[0].as_object().unwrap().identity(),
            api.origins.generated_declarations[&(Arc::as_ptr(identity) as usize)]
                .1
                .as_object()
                .unwrap()
                .identity()
        ));
    }

    #[test]
    fn theme_setup_replaces_original_arrays_and_shares_emitted_declarations() {
        let first = public_json!({"kind":"at-rule", "name":"@theme", "params":"", "nodes":public_json!([public_json!({"kind":"declaration", "property":"--color-card", "value":"red", "important":false})])});
        let second = public_json!({"kind":"at-rule", "name":"@theme", "params":"default", "nodes":public_json!([public_json!({"kind":"declaration", "property":"--color-card", "value":"blue", "important":false})])});
        let utilities = public_json!({"kind":"at-rule", "name":"@tailwind", "params":"utilities", "nodes":public_json!([])});
        let input = Arc::new(vec![first.clone(), second, utilities]);
        let mut api = compile_ast(
            input.clone(),
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let mutations = api.input_mutations();
        let InputMutation::Set {
            target: Value::Array(target),
            field,
            value: replacement,
        } = &mutations[0]
        else {
            panic!("theme replacement")
        };
        assert!(Arc::ptr_eq(target, &input));
        assert_eq!(field, &Text::from("0"));
        assert!(replacement["nodes"].as_array().unwrap().is_empty());
        assert!(matches!(
            &mutations[1],
            InputMutation::Splice {
                start: 1,
                delete_count: 1,
                ..
            }
        ));
        let InputMutation::Set {
            target,
            field,
            value,
        } = &mutations[2]
        else {
            panic!("theme children")
        };
        assert_eq!(field, &Text::from("nodes"));
        assert!(Arc::ptr_eq(
            target.as_object().unwrap().identity(),
            replacement.as_object().unwrap().identity()
        ));
        assert_eq!(value[0]["context"], public_json!({"theme":true}));
        let declaration = value[0]["nodes"][0].clone();
        assert_eq!(declaration["value"], public_json!("red"));
        assert_eq!(declaration["src"], undefined());
        assert_eq!(input[0], first);
        let output = api.build(&["bg-card".into()]).unwrap();
        let emitted = output
            .iter()
            .find(|node| node["selector"] == ":root, :host")
            .unwrap()["nodes"][0]
            .clone();
        assert!(Arc::ptr_eq(
            declaration.as_object().unwrap().identity(),
            emitted.as_object().unwrap().identity()
        ));
    }

    #[test]
    fn caller_effects_share_generated_nodes_and_publish_after_preflight() {
        let utilities = public_json!({"kind":"at-rule", "name":"@tailwind", "params":"utilities", "nodes":public_json!([]), "extra":"kept"});
        let mut api = compile_ast(
            Arc::new(vec![utilities.clone()]),
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        assert_eq!(
            api.input_mutations()
                .iter()
                .map(|mutation| mutation.field().unwrap().clone())
                .collect::<Vec<_>>(),
            vec![Text::from("kind"), Text::from("context")]
        );
        assert_eq!(utilities["kind"], public_json!("at-rule"));
        let first = api.build(&["block".into()]).unwrap();
        let mutations = api.input_mutations();
        assert_eq!(mutations.len(), 1);
        assert_eq!(mutations[0].field().unwrap().clone(), Text::from("nodes"));
        assert!(Arc::ptr_eq(
            mutations[0].target().as_object().unwrap().identity(),
            utilities.as_object().unwrap().identity()
        ));
        let raw = mutations[0].value().clone();
        assert!(!Arc::ptr_eq(
            raw[0].as_object().unwrap().identity(),
            first[1].as_object().unwrap().identity()
        ));
        assert!(Arc::ptr_eq(
            raw[0]["nodes"][0].as_object().unwrap().identity(),
            first[1]["nodes"][0].as_object().unwrap().identity()
        ));
        let retained = api.retained_cache_bytes();
        let raw_count = api.origins.raw_candidates.len();
        let rejected = CompileError::Limit {
            resource: "cache_bytes".into(),
            limit: 1,
        };
        assert_eq!(
            api.build_text_with_input_check(&[Text::from("flex")], |_, mutations, _| {
                assert_eq!(mutations[0].value().as_array().unwrap().len(), 2);
                Err(rejected.clone())
            }),
            Err(rejected)
        );
        assert_eq!(api.retained_cache_bytes(), retained);
        assert_eq!(api.origins.raw_candidates.len(), raw_count);
        let (Value::Array(previous), Value::Array(raw_array)) =
            (api.input_mutations()[0].value(), &raw)
        else {
            panic!("raw array")
        };
        assert!(Arc::ptr_eq(previous, raw_array));
        assert!(Arc::ptr_eq(&api.build(&[]).unwrap(), &first));
        assert!(api.input_mutations().is_empty());
        let next = api.build(&["flex".into()]).unwrap();
        assert!(Arc::ptr_eq(
            raw[0].as_object().unwrap().identity(),
            api.input_mutations()[0].value()[0]
                .as_object()
                .unwrap()
                .identity()
        ));
        assert!(Arc::ptr_eq(
            first[1]["nodes"][0].as_object().unwrap().identity(),
            next[1]["nodes"][0].as_object().unwrap().identity()
        ));
        assert!(
            api.compiler.sources_cache_bytes
                >= api
                    .origins
                    .raw_candidates
                    .values()
                    .map(|(_, value)| value.owned_bytes())
                    .sum::<usize>()
        );
    }

    #[test]
    fn graph_json_measurement_matches_original_units_and_shared_wire_expansion() {
        for unit in 0..=u16::MAX {
            let text = Text::from_units(vec![unit]);
            let value = Value::String(text.clone());
            let size = serde_json::to_string(&value).unwrap().len();
            assert_eq!(json_size(&value, &mut BTreeMap::new(), size).unwrap(), size);
            assert!(json_size(&value, &mut BTreeMap::new(), size - 1).is_err());
            let object =
                Value::Object(Arc::new(super::super::design_system::PublicObject::from([
                    (text, value),
                ])));
            let size = serde_json::to_string(&object).unwrap().len();
            assert_eq!(
                json_size(&object, &mut BTreeMap::new(), size).unwrap(),
                size
            );
        }
        let mut value = public_json!("leaf");
        for _ in 0..18 {
            value = public_json!([value, value]);
        }
        let mut memo = BTreeMap::new();
        assert!(
            matches!(json_size(&value, &mut memo, 1024), Err(CompileError::Limit { resource, limit:1024 }) if resource == "input_bytes")
        );
        assert!(memo.len() <= 18);
        let mut limits = crate::compiler::tests::limits();
        limits.input_bytes = 1;
        assert!(
            matches!(check_input_budget(&[], None, &limits), Err(CompileError::Limit { resource, .. }) if resource == "input_bytes")
        );
    }

    #[test]
    fn multiple_apply_candidates_preserve_each_source_and_tuple_sharing() {
        let source = public_json!({"file":"entry.css", "code":".x { @apply flex block flex; @apply underline; }"});
        let input = Arc::new(vec![
            public_json!({"kind":"rule", "selector":".x", "nodes":public_json!([
                public_json!({"kind":"at-rule", "name":"@apply", "params":"flex block flex", "nodes":public_json!([]), "src":public_json!([source, 5u64, 27u64])}),
                public_json!({"kind":"at-rule", "name":"@apply", "params":"underline", "nodes":public_json!([]), "src":public_json!([source, 29u64, 45u64])})
            ])}),
        ]);
        let mut api = compile_ast(
            input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let output = api.build(&[]).unwrap();
        let nodes = output[1]["nodes"].as_array().unwrap();
        let block = nodes.iter().find(|node| node["value"] == "block").unwrap();
        let flex = nodes.iter().find(|node| node["value"] == "flex").unwrap();
        let underline = nodes
            .iter()
            .find(|node| node["value"] == "underline")
            .unwrap();
        assert_eq!(block["src"], public_json!([source, 17u64, 22u64]));
        assert_eq!(flex["src"], public_json!([source, 23u64, 27u64]));
        assert_eq!(underline["src"], public_json!([source, 36u64, 45u64]));
        assert!(Arc::ptr_eq(
            block["src"][0].as_object().unwrap().identity(),
            source.as_object().unwrap().identity()
        ));
        let (Value::Array(block_source), Value::Array(flex_source)) = (&block["src"], &flex["src"])
        else {
            panic!("expected source tuples")
        };
        assert!(!Arc::ptr_eq(block_source, flex_source));
    }

    #[test]
    fn apply_locations_accept_javascript_number_offsets() {
        let number = |value| Value::Number(serde_json::Number::from_f64(value).unwrap());
        for start in [33.0, 33.5, -1.0] {
            let source = public_json!({"file":"entry.css", "code":"@apply block"});
            let original = public_json!([source, number(start), number(start + 12.0)]);
            let input = public_json!({"kind":"rule", "selector":".authored", "nodes":public_json!([
                public_json!({"kind":"at-rule", "name":"@apply", "params":"block", "nodes":public_json!([]), "src":original})
            ])});
            let mut api = compile_ast(
                Arc::new(vec![input]),
                CompileOptions::default(),
                vec![],
                crate::compiler::tests::limits(),
            )
            .unwrap();
            let output = api.build(&[]).unwrap();
            let location = &output[1]["nodes"][0]["src"];
            assert_eq!(location[1], number(start + 7.0));
            assert_eq!(location[2], number(start + 12.0));
            assert_eq!(original[1], number(start));
            assert!(Arc::ptr_eq(
                location[0].as_object().unwrap().identity(),
                source.as_object().unwrap().identity()
            ));
        }
    }

    #[test]
    fn shared_public_graph_allocations_are_charged_once() {
        let source = public_json!({"file":"entry.css", "code":"x".repeat(4096)});
        let shared = public_json!([source, source]);
        let distinct = public_json!([
            public_json!({"file":"entry.css", "code":"x".repeat(4096)}),
            public_json!({"file":"entry.css", "code":"x".repeat(4096)})
        ]);
        assert!(shared.owned_bytes() < distinct.owned_bytes());
        let mut declaration = public_json!({"kind":"declaration", "property":"--x", "value":"red"});
        let original = declaration.clone();
        declaration["value"] = public_json!("blue");
        assert!(Arc::ptr_eq(
            declaration.as_object().unwrap().identity(),
            original.as_object().unwrap().identity()
        ));
        assert_eq!(original["value"].as_text(), Some(&Text::from("red")));
    }

    #[test]
    fn rejected_output_does_not_publish_generated_declaration_identities() {
        let input = Arc::new(vec![
            public_json!({"kind":"at-rule", "name":"@tailwind", "params":"utilities", "nodes":public_json!([])}),
        ]);
        let mut api = compile_ast(
            input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let first = api.build_text(&[Text::from("block")]).unwrap();
        let declarations = api.origins.generated_declarations.clone();
        let bytes = api.retained_cache_bytes();
        let limit = CompileError::Limit {
            resource: "cache_bytes".into(),
            limit: 1,
        };
        assert_eq!(
            api.build_text_with_output_check(&[Text::from("flex")], |_, _| Err(limit.clone())),
            Err(limit)
        );
        assert_eq!(api.retained_cache_bytes(), bytes);
        assert_eq!(api.origins.generated_declarations, declarations);
        assert!(Arc::ptr_eq(&api.build_text(&[]).unwrap(), &first));
        let next = api.build_text(&[Text::from("flex")]).unwrap();
        let declaration = |ast: &PublicAst| {
            ast.iter()
                .find(|node| node["selector"].as_text() == Some(&Text::from(".block")))
                .unwrap()["nodes"]
                .as_array()
                .unwrap()[0]
                .as_object()
                .unwrap()
                .identity()
                .clone()
        };
        assert!(Arc::ptr_eq(&declaration(&first), &declaration(&next)));
        assert!(next
            .iter()
            .any(|node| node["selector"].as_text() == Some(&Text::from(".flex"))));
    }

    #[test]
    fn failed_setup_retains_completed_function_events_on_the_original_object() {
        let first = public_json!({"kind":"declaration", "property":"color", "value":"--alpha(red / 50%)", "important":false});
        let input = Arc::new(vec![
            first.clone(),
            public_json!({"kind":"declaration", "property":"padding", "value":"--spacing(2)", "important":false}),
        ]);
        let outcome = compile_ast_setup(
            input.clone(),
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let AstSetupOutcome::Failed(failure) = outcome else {
            panic!("expected semantic setup failure")
        };
        assert_eq!(failure.input_mutations.len(), 1);
        let InputMutation::Set {
            target,
            field,
            value,
        } = &failure.input_mutations[0]
        else {
            panic!("expected field assignment")
        };
        assert!(Arc::ptr_eq(
            target.as_object().unwrap().identity(),
            first.as_object().unwrap().identity()
        ));
        assert_eq!(field, &Text::from("value"));
        assert_eq!(
            value,
            &public_json!("color-mix(in oklab, red 50%, transparent)")
        );
        assert_eq!(input[0]["value"], public_json!("--alpha(red / 50%)"));
        assert!(failure.cache_bytes > 0);
        assert!(compile_ast(
            input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits()
        )
        .is_err());
    }

    #[test]
    fn failed_custom_function_retains_the_preceding_ordinary_assignment() {
        let first = public_json!({"kind":"declaration", "property":"color", "value":"--alpha(red / 50%)", "important":false});
        let input = Arc::new(vec![
            first.clone(),
            public_json!({"kind":"at-rule", "name":"@utility", "params":"card", "nodes":public_json!([
                public_json!({"kind":"declaration", "property":"padding", "value":"--spacing(2)", "important":false})
            ])}),
        ]);
        let AstSetupOutcome::Failed(failure) = compile_ast_setup(
            input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap() else {
            panic!("expected custom function failure")
        };
        let [InputMutation::Set {
            target,
            field,
            value,
        }] = failure.input_mutations.as_slice()
        else {
            panic!("expected the preceding ordinary assignment")
        };
        assert!(Arc::ptr_eq(
            target.as_object().unwrap().identity(),
            first.as_object().unwrap().identity()
        ));
        assert_eq!(field, &Text::from("value"));
        assert_eq!(
            value,
            &public_json!("color-mix(in oklab, red 50%, transparent)")
        );
    }

    #[test]
    fn failed_apply_retains_completed_replacements_on_the_original_array() {
        let body = Arc::new(vec![
            public_json!({"kind":"at-rule", "name":"@apply", "params":"block", "nodes":public_json!([])}),
            public_json!({"kind":"at-rule", "name":"@apply", "params":"missing-apply-class", "nodes":public_json!([])}),
        ]);
        let input = Arc::new(vec![
            public_json!({"kind":"rule", "selector":".one", "nodes":Value::Array(body.clone())}),
        ]);
        let AstSetupOutcome::Failed(failure) = compile_ast_setup(
            input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap() else {
            panic!("expected apply failure")
        };
        let [InputMutation::Set {
            target: Value::Array(target),
            field,
            value,
        }] = failure.input_mutations.as_slice()
        else {
            panic!("expected one completed apply replacement")
        };
        assert!(Arc::ptr_eq(target, &body));
        assert_eq!(field, &Text::from("0"));
        assert_eq!(value["property"], public_json!("display"));
        assert_eq!(value["value"], public_json!("block"));
        assert_eq!(body[0]["name"], public_json!("@apply"));
    }

    #[test]
    fn rejected_failed_setup_does_not_authorize_a_mutation_prefix() {
        let input = Arc::new(vec![
            public_json!({"kind":"declaration", "property":"color", "value":"--alpha(red / 50%)", "important":false}),
            public_json!({"kind":"declaration", "property":"padding", "value":"--spacing(2)", "important":false}),
        ]);
        let mut limits = crate::compiler::tests::limits();
        let AstSetupOutcome::Failed(failure) = compile_ast_setup(
            input.clone(),
            CompileOptions::default(),
            vec![],
            limits.clone(),
        )
        .unwrap() else {
            panic!("expected semantic failure")
        };
        limits.cache_bytes = failure.cache_bytes - 1;
        assert!(
            matches!(compile_ast_setup(input, CompileOptions::default(), vec![], limits), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
        );
    }

    #[test]
    fn ast_build_preserves_candidate_and_authored_text_units() {
        let input:PublicAst=Arc::new(serde_json::from_str(r#"[{"kind":"rule","selector":".\ud800","nodes":[{"kind":"declaration","property":"--key","value":"\udc00","important":false}]},{"kind":"at-rule","name":"@tailwind","params":"utilities","nodes":[]}]"#).unwrap());
        let mut api = compile_ast(
            input,
            CompileOptions::default(),
            vec![],
            crate::compiler::tests::limits(),
        )
        .unwrap();
        let mut candidate = Text::from("[--mark:");
        candidate.push_text(&Text::from_units(vec![0xd800]));
        candidate.push_str("]");
        let output = api.build_text(&[candidate]).unwrap();
        assert!(output.iter().any(|node| node["selector"]
            .as_text()
            .is_some_and(|selector| selector.units() == [46, 0xd800])));
        let json = serde_json::to_string(output.as_ref()).unwrap();
        let decoded: Vec<Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, output.as_ref().clone());
    }
}
