//! Ordered CSS syntax trees. Generated rules and renderer programs share declarations.

pub use super::origin::NodeOrigin;
use super::public_api::SourceLocation;
use crate::text::Text;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};

pub const DECLARATION_IDENTITY_BYTES: usize = 2 * std::mem::size_of::<usize>();

pub(super) fn contains_at_rule(nodes: &[AstNode], name: &str) -> bool {
    nodes.iter().any(|node| {
        matches!(node, AstNode::AtRule { name: actual, .. } if actual == name)
            || node
                .children()
                .is_some_and(|nodes| contains_at_rule(nodes, name))
    })
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AstNode {
    AtRoot {
        #[serde(skip)]
        origin: Option<NodeOrigin>,
        nodes: Vec<AstNode>,
    },
    Context {
        #[serde(skip)]
        origin: Option<NodeOrigin>,
        base: Text,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_base: Option<Text>,
        reference: bool,
        nodes: Vec<AstNode>,
    },
    Rule {
        #[serde(skip)]
        origin: Option<NodeOrigin>,
        #[serde(skip)]
        source: Option<SourceLocation>,
        #[serde(skip)]
        destination: Option<SourceLocation>,
        selector: Text,
        nodes: Vec<AstNode>,
    },
    AtRule {
        #[serde(skip)]
        origin: Option<NodeOrigin>,
        #[serde(skip)]
        source: Option<SourceLocation>,
        #[serde(skip)]
        destination: Option<SourceLocation>,
        name: Text,
        params: Text,
        nodes: Option<Vec<AstNode>>,
    },
    Declaration {
        #[serde(skip)]
        origin: Option<NodeOrigin>,
        #[serde(skip)]
        source: Option<SourceLocation>,
        #[serde(skip)]
        destination: Option<SourceLocation>,
        property: Text,
        value: Text,
        important: bool,
        #[serde(skip)]
        identity: Arc<()>,
    },
    Comment {
        #[serde(skip)]
        origin: Option<NodeOrigin>,
        #[serde(skip)]
        source: Option<SourceLocation>,
        #[serde(skip)]
        destination: Option<SourceLocation>,
        value: Text,
    },
}

impl PartialEq for AstNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AtRoot { nodes: a, .. }, Self::AtRoot { nodes: b, .. }) => a == b,
            (
                Self::Context {
                    base: a,
                    source_base: b,
                    reference: c,
                    nodes: d,
                    ..
                },
                Self::Context {
                    base: e,
                    source_base: f,
                    reference: g,
                    nodes: h,
                    ..
                },
            ) => (a, b, c, d) == (e, f, g, h),
            (
                Self::Rule {
                    source: a,
                    destination: b,
                    selector: c,
                    nodes: d,
                    ..
                },
                Self::Rule {
                    source: e,
                    destination: f,
                    selector: g,
                    nodes: h,
                    ..
                },
            ) => (a, b, c, d) == (e, f, g, h),
            (
                Self::AtRule {
                    source: a,
                    destination: b,
                    name: c,
                    params: d,
                    nodes: e,
                    ..
                },
                Self::AtRule {
                    source: f,
                    destination: g,
                    name: h,
                    params: i,
                    nodes: j,
                    ..
                },
            ) => (a, b, c, d, e) == (f, g, h, i, j),
            (
                Self::Declaration {
                    source: a,
                    destination: b,
                    property: c,
                    value: d,
                    important: e,
                    identity: f,
                    ..
                },
                Self::Declaration {
                    source: g,
                    destination: h,
                    property: i,
                    value: j,
                    important: k,
                    identity: l,
                    ..
                },
            ) => (a, b, c, d, e, f) == (g, h, i, j, k, l),
            (
                Self::Comment {
                    source: a,
                    destination: b,
                    value: c,
                    ..
                },
                Self::Comment {
                    source: d,
                    destination: e,
                    value: f,
                    ..
                },
            ) => (a, b, c) == (d, e, f),
            _ => false,
        }
    }
}
impl Eq for AstNode {}

impl<'de> Deserialize<'de> for AstNode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Kind {
            AtRoot,
            Context,
            Rule,
            AtRule,
            Declaration,
            Comment,
        }
        #[derive(Deserialize)]
        struct Fields {
            kind: Kind,
            nodes: Option<Vec<AstNode>>,
            base: Option<Text>,
            source_base: Option<Text>,
            reference: Option<bool>,
            selector: Option<Text>,
            name: Option<Text>,
            params: Option<Text>,
            property: Option<Text>,
            value: Option<Text>,
            important: Option<bool>,
        }
        fn required<T, E: serde::de::Error>(field: Option<T>, name: &'static str) -> Result<T, E> {
            field.ok_or_else(|| E::missing_field(name))
        }
        // Decode Text fields directly; a tagged enum's Content buffer rejects lone units.
        let fields = Fields::deserialize(deserializer)?;
        Ok(match fields.kind {
            Kind::AtRoot => Self::AtRoot {
                origin: None,
                nodes: required(fields.nodes, "nodes")?,
            },
            Kind::Context => Self::Context {
                origin: None,
                base: required(fields.base, "base")?,
                source_base: fields.source_base,
                reference: required(fields.reference, "reference")?,
                nodes: required(fields.nodes, "nodes")?,
            },
            Kind::Rule => Self::Rule {
                origin: None,
                source: None,
                destination: None,
                selector: required(fields.selector, "selector")?,
                nodes: required(fields.nodes, "nodes")?,
            },
            Kind::AtRule => Self::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: required(fields.name, "name")?,
                params: required(fields.params, "params")?,
                nodes: fields.nodes,
            },
            Kind::Declaration => Self::Declaration {
                origin: None,
                source: None,
                destination: None,
                property: required(fields.property, "property")?,
                value: required(fields.value, "value")?,
                important: required(fields.important, "important")?,
                identity: Arc::new(()),
            },
            Kind::Comment => Self::Comment {
                origin: None,
                source: None,
                destination: None,
                value: required(fields.value, "value")?,
            },
        })
    }
}

impl AstNode {
    pub(crate) fn origin(&self) -> Option<&NodeOrigin> {
        match self {
            Self::AtRoot { origin, .. }
            | Self::Context { origin, .. }
            | Self::Rule { origin, .. }
            | Self::AtRule { origin, .. }
            | Self::Declaration { origin, .. }
            | Self::Comment { origin, .. } => origin.as_ref(),
        }
    }
    pub(crate) fn set_origin(&mut self, value: Option<NodeOrigin>) {
        match self {
            Self::AtRoot { origin, .. }
            | Self::Context { origin, .. }
            | Self::Rule { origin, .. }
            | Self::AtRule { origin, .. }
            | Self::Declaration { origin, .. }
            | Self::Comment { origin, .. } => *origin = value,
        }
    }

    pub fn declaration(property: impl Into<Text>, value: impl Into<Text>) -> Self {
        Self::Declaration {
            origin: None,
            source: None,
            destination: None,
            property: property.into(),
            value: value.into(),
            important: false,
            identity: Arc::new(()),
        }
    }

    pub fn source(&self) -> Option<&SourceLocation> {
        match self {
            Self::Rule { source, .. }
            | Self::AtRule { source, .. }
            | Self::Declaration { source, .. }
            | Self::Comment { source, .. } => source.as_ref(),
            Self::Context { .. } | Self::AtRoot { .. } => None,
        }
    }

    pub fn set_source(&mut self, location: Option<SourceLocation>) {
        match self {
            Self::Rule { source, .. }
            | Self::AtRule { source, .. }
            | Self::Declaration { source, .. }
            | Self::Comment { source, .. } => *source = location,
            Self::Context { .. } | Self::AtRoot { .. } => {}
        }
    }

    pub fn destination(&self) -> Option<&SourceLocation> {
        match self {
            Self::Rule { destination, .. }
            | Self::AtRule { destination, .. }
            | Self::Declaration { destination, .. }
            | Self::Comment { destination, .. } => destination.as_ref(),
            Self::Context { .. } | Self::AtRoot { .. } => None,
        }
    }

    pub fn set_destination(&mut self, location: Option<SourceLocation>) {
        match self {
            Self::Rule { destination, .. }
            | Self::AtRule { destination, .. }
            | Self::Declaration { destination, .. }
            | Self::Comment { destination, .. } => *destination = location,
            Self::Context { .. } | Self::AtRoot { .. } => {}
        }
    }

    pub fn children(&self) -> Option<&[Self]> {
        match self {
            Self::AtRoot { nodes, .. }
            | Self::Context { nodes, .. }
            | Self::Rule { nodes, .. }
            | Self::AtRule {
                nodes: Some(nodes), ..
            } => Some(nodes),
            _ => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Self::AtRoot { nodes, .. }
            | Self::Context { nodes, .. }
            | Self::Rule { nodes, .. }
            | Self::AtRule {
                nodes: Some(nodes), ..
            } => Some(nodes),
            _ => None,
        }
    }
}

pub(super) fn source_owned_bytes(nodes: &[AstNode]) -> usize {
    source_owned_bytes_many([nodes])
}

pub(super) fn source_owned_bytes_many<'a>(
    groups: impl IntoIterator<Item = &'a [AstNode]>,
) -> usize {
    fn collect<'a>(
        nodes: &'a [AstNode],
        sources: &mut BTreeMap<usize, &'a super::public_api::Source>,
        public: &mut BTreeMap<usize, &'a super::public_api::PublicValue>,
        origins: &mut Vec<&'a NodeOrigin>,
    ) {
        for node in nodes {
            if let Some(origin) = node.origin() {
                origins.push(origin);
            }
            for location in [node.source(), node.destination()].into_iter().flatten() {
                sources
                    .entry(Arc::as_ptr(&location.source) as usize)
                    .or_insert(&location.source);
                if let Some(value) = &location.public {
                    public.entry(Arc::as_ptr(value) as usize).or_insert(value);
                }
            }
            if let Some(nodes) = node.children() {
                collect(nodes, sources, public, origins);
            }
        }
    }
    let mut sources = BTreeMap::new();
    let mut public = BTreeMap::new();
    let mut origins = Vec::new();
    for nodes in groups {
        collect(nodes, &mut sources, &mut public, &mut origins);
    }
    let mut seen = std::collections::BTreeSet::new();
    let public_bytes = public
        .values()
        .map(|value| {
            std::mem::size_of::<super::public_api::PublicValue>()
                .saturating_add(2 * std::mem::size_of::<usize>())
                .saturating_add(super::public_api::design_system::json_bytes_seen(
                    value, &mut seen,
                ))
        })
        .fold(0usize, usize::saturating_add)
        .saturating_add(
            origins
                .iter()
                .map(|origin| origin.owned_bytes(&mut seen))
                .sum::<usize>(),
        );
    sources
        .values()
        .map(|source| {
            std::mem::size_of::<super::public_api::Source>()
                + 2 * std::mem::size_of::<usize>()
                + source.file.as_ref().map_or(0, String::capacity)
                + source.code.owned_bytes()
        })
        .fold(public_bytes, usize::saturating_add)
}

pub(super) fn find_at_rule_source<'a>(
    nodes: &'a [AstNode],
    name: &str,
) -> Option<&'a SourceLocation> {
    for node in nodes {
        if matches!(node,AstNode::AtRule{name:actual,..} if actual == name) {
            return node.source();
        }
        if let Some(source) = node
            .children()
            .and_then(|nodes| find_at_rule_source(nodes, name))
        {
            return Some(source);
        }
    }
    None
}

pub(super) fn fill_sources(nodes: &mut [AstNode], source: Option<&SourceLocation>) {
    for node in nodes {
        if node.source().is_none() {
            node.set_source(source.cloned());
        }
        if let Some(nodes) = node.children_mut() {
            fill_sources(nodes, source);
        }
    }
}

pub(super) fn replace_sources(nodes: &mut [AstNode], source: Option<&SourceLocation>) {
    for node in nodes {
        node.set_source(source.cloned());
        if let Some(nodes) = node.children_mut() {
            replace_sources(nodes, source);
        }
    }
}

pub fn clone_fresh(nodes: &[AstNode]) -> Vec<AstNode> {
    clone_with_graph(nodes, super::origin::CloneGraph::unlimited(nodes))
}

pub(super) fn clone_fresh_checked(
    nodes: &[AstNode],
    root: Option<super::origin::CloneRoot<'_>>,
    limit: usize,
) -> Result<Vec<AstNode>, super::CompileError> {
    Ok(clone_with_graph(
        nodes,
        super::origin::CloneGraph::new(nodes, root, limit)?,
    ))
}

fn clone_with_graph(nodes: &[AstNode], mut graph: super::origin::CloneGraph) -> Vec<AstNode> {
    fn freshen(
        nodes: &mut [AstNode],
        identities: &mut BTreeMap<usize, Arc<()>>,
        sources: &mut BTreeMap<usize, Arc<super::public_api::Source>>,
        public: &mut super::origin::CloneGraph,
    ) {
        for node in nodes {
            if let Some(origin) = node.origin() {
                node.set_origin(Some(public.clone_origin(origin)));
            }
            let fresh = |location: &SourceLocation,
                         sources: &mut BTreeMap<usize, Arc<super::public_api::Source>>,
                         public: &mut super::origin::CloneGraph| {
                let identity = Arc::as_ptr(&location.source) as usize;
                let mut location = location.clone();
                location.source = sources
                    .entry(identity)
                    .or_insert_with(|| Arc::new((*location.source).clone()))
                    .clone();
                location.public = location
                    .public
                    .as_ref()
                    .map(|value| Arc::new(public.clone_value(value)));
                location
            };
            if let Some(source) = node
                .source()
                .map(|location| fresh(location, sources, public))
            {
                node.set_source(Some(source));
            }
            if let Some(destination) = node
                .destination()
                .map(|location| fresh(location, sources, public))
            {
                node.set_destination(Some(destination));
            }
            if let AstNode::Declaration { identity, .. } = node {
                let original = Arc::as_ptr(identity) as usize;
                *identity = identities
                    .entry(original)
                    .or_insert_with(|| Arc::new(()))
                    .clone();
            }
            if let Some(children) = node.children_mut() {
                freshen(children, identities, sources, public);
            }
        }
    }
    let mut cloned = nodes.to_vec();
    freshen(
        &mut cloned,
        &mut BTreeMap::new(),
        &mut BTreeMap::new(),
        &mut graph,
    );
    cloned
}

#[cfg(all(test, rustify_repo_tests))]
pub(super) fn optimize_rule_children(nodes: &mut Vec<AstNode>) {
    optimize_nodes(nodes, true);
}

pub(super) fn optimize_rules(nodes: &mut Vec<AstNode>) {
    optimize_nodes(nodes, false);
}

fn optimize_nodes(nodes: &mut Vec<AstNode>, rule_children: bool) {
    let mut output = Vec::new();
    for mut node in std::mem::take(nodes) {
        match &mut node {
            AstNode::Context {
                reference, nodes, ..
            } => {
                if !*reference {
                    optimize_nodes(nodes, false);
                    output.append(nodes);
                }
                continue;
            }
            AstNode::Rule {
                selector, nodes, ..
            } => {
                optimize_nodes(nodes, true);
                if nodes.is_empty() {
                    continue;
                }
                if selector == "&" {
                    output.append(nodes);
                    continue;
                }
            }
            AstNode::AtRule { name, nodes, .. } => {
                if let Some(children) = nodes {
                    optimize_nodes(children, false);
                }
                if nodes.as_ref().is_none_or(Vec::is_empty)
                    && ![
                        "@layer",
                        "@charset",
                        "@custom-media",
                        "@namespace",
                        "@import",
                        "@tailwind",
                        "@apply",
                        "@property",
                    ]
                    .iter()
                    .any(|label| name == label)
                {
                    continue;
                }
            }
            AstNode::Declaration { property, .. } if property == "--tw-sort" => continue,
            _ => {}
        }
        output.push(node);
    }
    if rule_children {
        fn declaration_key(node: &AstNode) -> Option<(u8, usize)> {
            let AstNode::Declaration { identity, .. } = node else {
                return None;
            };
            Some(
                node.origin()
                    .and_then(NodeOrigin::object_id)
                    .map_or((0, Arc::as_ptr(identity) as usize), |id| (1, id)),
            )
        }
        let mut seen: BTreeMap<(Text, Text, bool), Vec<(u8, usize)>> = BTreeMap::new();
        for node in &output {
            if let AstNode::Declaration {
                property,
                value,
                important,
                ..
            } = node
            {
                seen.entry((property.clone(), value.clone(), *important))
                    .or_default()
                    .extend(declaration_key(node));
            }
        }
        let removed = seen
            .values()
            .flat_map(|identities| &identities[..identities.len() - 1])
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        output.retain(|node| declaration_key(node).is_none_or(|key| !removed.contains(&key)));
    }
    *nodes = output;
}

pub(super) fn hoist_at_roots(nodes: &mut Vec<AstNode>, globals: &mut Vec<AstNode>) {
    nodes.retain_mut(|node| {
        if matches!(
            node,
            AstNode::Context {
                reference: true,
                ..
            }
        ) {
            return true;
        }
        if let AstNode::AtRoot { nodes, .. } = node {
            let mut resources = std::mem::take(nodes);
            hoist_at_roots(&mut resources, globals);
            globals.extend(resources);
            return false;
        }
        if let Some(children) = node.children_mut() {
            hoist_at_roots(children, globals);
        }
        true
    });
}

pub(super) fn declaration_identity_bytes(nodes: &[AstNode]) -> usize {
    nodes.iter().fold(0usize, |bytes, node| {
        bytes
            .saturating_add(if matches!(node, AstNode::Declaration { .. }) {
                DECLARATION_IDENTITY_BYTES
            } else {
                0
            })
            .saturating_add(node.children().map_or(0, declaration_identity_bytes))
    })
}

pub fn validate_depth(nodes: &[AstNode], limit: usize) -> Result<(), super::CompileError> {
    let mut pending = vec![(nodes, 0usize)];
    while let Some((nodes, depth)) = pending.pop() {
        for node in nodes {
            if let Some(children) = node.children() {
                let next = depth.saturating_add(1);
                if next > limit {
                    return Err(super::CompileError::Limit {
                        resource: "ast_depth".into(),
                        limit,
                    });
                }
                pending.push((children, next));
            }
        }
    }
    Ok(())
}

pub fn to_css(nodes: &[AstNode]) -> Text {
    fn print(nodes: &[AstNode], depth: usize, css: &mut Text) {
        for node in nodes {
            if let AstNode::AtRoot { nodes, .. } = node {
                print(nodes, 0, css);
                continue;
            }
            if let AstNode::Context {
                reference, nodes, ..
            } = node
            {
                if !reference {
                    print(nodes, depth, css);
                }
                continue;
            }
            css.push_str(&"  ".repeat(depth));
            match node {
                AstNode::AtRoot { nodes, .. } => print(nodes, 0, css),
                AstNode::Context { .. } => unreachable!(),
                AstNode::Declaration {
                    property,
                    value,
                    important,
                    ..
                } => {
                    css.push_text(property);
                    css.push_str(": ");
                    css.push_text(value);
                    if *important {
                        css.push_str(" !important");
                    }
                    css.push_str(";\n");
                }
                AstNode::Rule {
                    selector, nodes, ..
                } => {
                    css.push_text(selector);
                    css.push_str(" {\n");
                    print(nodes, depth + 1, css);
                    css.push_str(&"  ".repeat(depth));
                    css.push_str("}\n");
                }
                AstNode::AtRule {
                    name,
                    params,
                    nodes,
                    ..
                } => {
                    css.push_text(name);
                    if let Some(nodes) = nodes.as_ref().filter(|nodes| !nodes.is_empty()) {
                        if !params.is_empty() {
                            css.push_str(" ");
                            css.push_text(params);
                        }
                        css.push_str(" {\n");
                        print(nodes, depth + 1, css);
                        css.push_str(&"  ".repeat(depth));
                        css.push_str("}\n");
                    } else {
                        css.push_str(" ");
                        css.push_text(params);
                        css.push_str(";\n");
                    }
                }
                AstNode::Comment { value, .. } => {
                    css.push_str("/*");
                    css.push_text(value);
                    css.push_str("*/\n");
                }
            }
        }
    }
    let mut css = Text::default();
    print(nodes, 0, &mut css);
    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_source_graph_clones_preserve_sharing_and_charge_metadata() {
        use super::super::public_api::{PublicValue as Value, Source};
        let source: Value = serde_json::from_str(&format!(
            r#"{{"file":"entry\ud800.css","code":"x","metadata":"{}"}}"#,
            "x".repeat(4096)
        ))
        .unwrap();
        let tuple = Value::from(vec![source.clone(), Value::from(0u64), Value::from(1u64)]);
        let mut node = AstNode::declaration("display", "block");
        node.set_source(Some(SourceLocation {
            source: Arc::new(Source {
                file: None,
                code: Text::default(),
            }),
            start: 0,
            end: 1,
            public: Some(Arc::new(tuple)),
        }));
        let nodes = vec![node.clone(), node.clone()];
        let bytes = source_owned_bytes(std::slice::from_ref(&node));
        assert!(bytes >= 8192);
        assert_eq!(source_owned_bytes(&nodes), bytes);
        let first = clone_fresh(&nodes);
        let second = clone_fresh(&nodes);
        let public = |node: &AstNode| node.source().unwrap().public.clone().unwrap();
        let (a, b, c) = (public(&first[0]), public(&first[1]), public(&second[0]));
        let (Value::Array(a_tuple), Value::Array(b_tuple)) = (&*a, &*b) else {
            panic!("source tuples")
        };
        assert!(Arc::ptr_eq(a_tuple, b_tuple));
        assert_eq!(a[0], source);
        assert!(!Arc::ptr_eq(
            a[0].as_object().unwrap().identity(),
            source.as_object().unwrap().identity()
        ));
        assert!(!Arc::ptr_eq(
            a[0].as_object().unwrap().identity(),
            c[0].as_object().unwrap().identity()
        ));
        assert!(source_owned_bytes(&first) < bytes * 2);
    }

    #[test]
    fn all_ast_text_fields_decode_original_units_without_scalar_staging() {
        let json = r#"{"nodes":[{"selector":"\ud800","nodes":[{"value":"\udc00","kind":"comment"}],"kind":"rule"},{"params":"\udc00","name":"@\ud800","nodes":null,"kind":"at_rule"}],"base":"\ud800","source_base":"\udc00","reference":false,"kind":"context"}"#;
        let node: AstNode = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&node).unwrap();
        assert_eq!(serde_json::from_str::<AstNode>(&serialized).unwrap(), node);
        assert_eq!(
            to_css(&[node]).units(),
            &[
                0xd800, 32, 123, 10, 32, 32, 47, 42, 0xdc00, 42, 47, 10, 125, 10, 64, 0xd800, 32,
                0xdc00, 59, 10
            ]
        );
    }

    fn identity(node: &AstNode) -> &Arc<()> {
        let AstNode::Declaration { identity, .. } = node else {
            panic!("expected declaration");
        };
        identity
    }

    #[test]
    fn fresh_clones_preserve_aliases_and_create_a_new_identity_graph() {
        let declaration = AstNode::declaration("width", "13px");
        let original = vec![declaration.clone(), declaration];
        let cloned = clone_fresh(&original);
        assert!(
            cloned == original
                && Arc::ptr_eq(identity(&cloned[0]), identity(&cloned[1]))
                && !Arc::ptr_eq(identity(&original[0]), identity(&cloned[0]))
        );
    }

    #[test]
    fn declaration_identity_is_not_serialized() {
        let original = AstNode::declaration("width", "13px");
        let json = serde_json::to_string(&original).unwrap();
        let restored: AstNode = serde_json::from_str(&json).unwrap();
        assert!(
            json == r#"{"kind":"declaration","property":"width","value":"13px","important":false}"#
                && restored == original
                && !Arc::ptr_eq(identity(&original), identity(&restored))
        );
    }
}
