//! Exact public node fields, independent of source spans and native declaration IDs.

use super::public_api::PublicValue;
use super::{
    ast::AstNode,
    public_api::{
        design_system::{json_bytes_seen, public_json},
        InputMutation, PublicValue as Value,
    },
    CompileError,
};
use crate::text::Text;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

type Key = (u8, usize);
pub(super) mod current;
use current::{Allowance, Current};
fn key(value: &Value) -> Option<Key> {
    match value {
        Value::Object(value) => Some((1, Arc::as_ptr(value.identity()) as usize)),
        Value::Array(value) => Some((0, Arc::as_ptr(value) as usize)),
        _ => None,
    }
}
fn storage(value: &Value) -> Option<Key> {
    match value {
        Value::Object(value) => Some((1, Arc::as_ptr(value) as usize)),
        Value::Array(value) => Some((0, Arc::as_ptr(value) as usize)),
        _ => None,
    }
}
#[derive(Clone)]
struct View {
    original: Value,
    current: Value,
}
struct Data {
    value: Value,
    views: Option<Arc<Current>>,
    caller: bool,
}

/// Opaque public provenance; serialization and native structural equality ignore it.
#[derive(Clone)]
pub struct NodeOrigin(Arc<Data>);
impl std::fmt::Debug for NodeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("NodeOrigin")
    }
}
impl NodeOrigin {
    pub(super) fn caller_with_current(value: Value, views: Arc<Current>) -> Self {
        Self(Arc::new(Data {
            value,
            views: Some(views),
            caller: true,
        }))
    }
    pub(super) fn caller(value: Value) -> Self {
        Self(Arc::new(Data {
            value,
            views: None,
            caller: true,
        }))
    }
    pub(super) fn generated(value: Value) -> Self {
        Self(Arc::new(Data {
            value,
            views: None,
            caller: false,
        }))
    }
    pub(super) fn object_id(&self) -> Option<usize> {
        key(&self.0.value).map(|(_, id)| id)
    }
    pub(super) fn is_caller(&self) -> bool {
        self.0.caller
    }
    pub(super) fn fields(&self) -> &Value {
        self.0
            .views
            .as_ref()
            .map_or(&self.0.value, |views| views.value(&self.0.value))
    }
    pub(super) fn owned_bytes(&self, seen: &mut BTreeSet<Key>) -> usize {
        if !seen.insert((3, Arc::as_ptr(&self.0) as usize)) {
            return 0;
        }
        let mut bytes = std::mem::size_of::<Data>() + 2 * std::mem::size_of::<usize>();
        bytes = bytes.saturating_add(json_bytes_seen(&self.0.value, seen));
        if let Some(views) = &self.0.views {
            if seen.insert((4, Arc::as_ptr(views) as usize)) {
                bytes = bytes.saturating_add(
                    std::mem::size_of::<Current>() + 2 * std::mem::size_of::<usize>(),
                );
                bytes = bytes.saturating_add(views.owned_bytes(seen));
            }
        }
        bytes
    }
}

pub(super) enum CloneRoot<'a> {
    Body(&'a NodeOrigin),
    Definition(&'a NodeOrigin),
}

#[derive(Default)]
pub(super) struct Edits {
    current: Current,
    books: BTreeMap<usize, Arc<Current>>,
    mutations: Vec<InputMutation>,
    allowance: Allowance,
}
impl Edits {
    fn include(&mut self, origin: &NodeOrigin) {
        if let Some(views) = &origin.0.views {
            if self
                .books
                .insert(Arc::as_ptr(views) as usize, views.clone())
                .is_none()
            {
                self.current.include(views);
            }
        }
    }
    pub(super) fn from_nodes(nodes: &[AstNode]) -> Self {
        let mut edits = Self::default();
        edits.include_nodes(nodes);
        edits
    }
    pub(super) fn include_nodes(&mut self, nodes: &[AstNode]) {
        for node in nodes {
            if let Some(origin) = node.origin() {
                self.include(origin);
            }
            if let Some(children) = node.children() {
                self.include_nodes(children);
            }
        }
    }
    fn current<'a>(&'a self, value: &'a Value) -> &'a Value {
        self.current.value(value)
    }
    fn fields<'a>(&'a self, origin: &'a NodeOrigin) -> &'a Value {
        self.current(origin.fields())
    }
    fn insert(&mut self, original: Value, current: Value) {
        self.current.insert(original, current);
    }
    fn owned_bytes(&self, seen: &mut BTreeSet<Key>) -> usize {
        let mut bytes = self.current.owned_bytes(seen);
        for (&id, book) in &self.books {
            bytes = bytes.saturating_add(
                std::mem::size_of::<(usize, Arc<Current>)>() + 5 * std::mem::size_of::<usize>(),
            );
            if seen.insert((4, id)) {
                bytes = bytes
                    .saturating_add(
                        std::mem::size_of::<Current>() + 2 * std::mem::size_of::<usize>(),
                    )
                    .saturating_add(book.owned_bytes(seen));
            }
        }
        bytes
    }
    // Refresh only after an explicit edit in this phase; preserve other typed transforms.
    pub(super) fn refresh(&self, node: &mut AstNode) {
        let Some(origin) = node.origin() else { return };
        let Some(view) = key(&origin.0.value).and_then(|key| self.current.values.get(&key)) else {
            return;
        };
        let value = view.current.get("value").and_then(Value::as_text).cloned();
        let params = view.current.get("params").and_then(Value::as_text).cloned();
        match node {
            AstNode::Declaration { value: field, .. } => {
                if let Some(value) = value {
                    *field = value;
                }
            }
            AstNode::AtRule { params: field, .. } => {
                if let Some(params) = params {
                    *field = params;
                }
            }
            _ => {}
        }
    }
    pub(super) fn assign(
        &mut self,
        node: &mut AstNode,
        field: &str,
        value: Text,
    ) -> Result<(), CompileError> {
        let value = Value::String(value);
        if let Some(origin) = node.origin() {
            let record_bytes = if origin.0.caller {
                4 * std::mem::size_of::<InputMutation>() + field.encode_utf16().count() * 2
            } else {
                0
            };
            let captured = self.current.assign(
                &origin.0.value,
                field,
                &value,
                &mut self.allowance,
                record_bytes,
            )?;
            if origin.0.caller {
                self.mutations.push(InputMutation::Set {
                    target: origin.0.value.clone(),
                    field: field.into(),
                    value: captured.version,
                });
            }
        }
        if let Value::String(value) = value {
            match (node, field) {
                (AstNode::Declaration { value: old, .. }, "value") => *old = value,
                (AstNode::AtRule { params: old, .. }, "params") => *old = value,
                _ => {}
            }
        }
        Ok(())
    }
    fn native_value(&mut self, node: &AstNode) -> Value {
        let mut fields=node.origin().map(|origin|self.fields(origin).clone()).unwrap_or_else(|| {
            match node {
                AstNode::Declaration { property,value,important, .. } => public_json!({"kind":"declaration","property":property,"value":value,"important":important}),
                AstNode::Rule { selector, .. } => public_json!({"kind":"rule","selector":selector,"nodes":public_json!([])}),
                AstNode::AtRule { name,params, .. } => public_json!({"kind":"at-rule","name":name,"params":params,"nodes":public_json!([])}),
                AstNode::Comment { value, .. } => public_json!({"kind":"comment","value":value}),
                AstNode::AtRoot { .. } => public_json!({"kind":"at-root","nodes":public_json!([])}),
                AstNode::Context { base,reference, .. } => {
                    let mut context=public_json!({});
                    if !base.is_empty() { context["base"]=public_json!(base); }
                    if *reference { context["reference"]=public_json!(true); }
                    public_json!({"kind":"context","context":context,"nodes":public_json!([])})
                }
            }
        });
        match node {
            AstNode::Declaration {
                property,
                value,
                important,
                ..
            } => {
                fields["property"] = public_json!(property);
                fields["value"] = public_json!(value);
                fields["important"] = public_json!(important);
            }
            AstNode::Rule { selector, .. } => {
                fields["kind"] = public_json!("rule");
                fields["selector"] = public_json!(selector);
            }
            AstNode::AtRule { name, params, .. } => {
                fields["name"] = public_json!(name);
                fields["params"] = public_json!(params);
            }
            AstNode::Comment { value, .. } => fields["value"] = public_json!(value),
            _ => {}
        }
        if let Some(children) = node.children() {
            let original = fields.get("nodes").cloned();
            let mut values = Value::Array(Arc::new(
                children
                    .iter()
                    .map(|child| self.native_value(child))
                    .collect(),
            ));
            if let Some(original) = original {
                if original == values {
                    values = original.clone();
                }
                self.insert(original, values.clone());
            }
            fields["nodes"] = values;
        }
        for (name, location) in [("src", node.source()), ("dst", node.destination())] {
            if let Some(value) = location.and_then(|location| location.public.as_deref()) {
                // Source wrappers used to locate caller directives are not a src assignment.
                if fields.get(name).is_some()
                    || !node.origin().is_some_and(|origin| origin.0.caller)
                {
                    fields[name] = value.clone();
                }
            }
        }
        if let Some(origin) = node.origin() {
            if &fields == self.fields(origin) {
                fields = self.fields(origin).clone();
            }
            self.insert(origin.0.value.clone(), fields.clone());
        }
        fields
    }
    pub(super) fn sync(&mut self, nodes: &[AstNode]) {
        self.include_nodes(nodes);
        for node in nodes {
            self.native_value(node);
        }
    }
    pub(super) fn sync_root(&mut self, origin: &NodeOrigin, nodes: &[AstNode]) {
        self.include(origin);
        self.sync(nodes);
        let mut fields = self.fields(origin).clone();
        if let Some(original) = fields.get("nodes").cloned() {
            let mut current = Value::Array(Arc::new(
                nodes.iter().map(|node| self.native_value(node)).collect(),
            ));
            if original == current {
                current = original.clone();
            }
            self.insert(original, current.clone());
            fields["nodes"] = current;
        }
        self.insert(origin.0.value.clone(), fields);
    }
    pub(super) fn finish(self) -> Snapshot {
        Snapshot {
            views: Arc::new(self.current),
            #[cfg(test)]
            mutations: self.mutations,
        }
    }
}
pub(super) struct Snapshot {
    views: Arc<Current>,
    #[cfg(test)]
    pub mutations: Vec<InputMutation>,
}
impl Snapshot {
    pub(super) fn bind_origin(&self, origin: &mut NodeOrigin) {
        *origin = NodeOrigin(Arc::new(Data {
            value: origin.0.value.clone(),
            views: Some(self.views.clone()),
            caller: origin.0.caller,
        }));
    }
    pub(super) fn bind(&self, nodes: &mut [AstNode]) {
        for node in nodes {
            if let Some(origin) = node.origin() {
                let mut origin = origin.clone();
                self.bind_origin(&mut origin);
                node.set_origin(Some(origin));
            }
            if let Some(children) = node.children_mut() {
                self.bind(children);
            }
        }
    }
}

pub(super) struct CloneGraph {
    edits: Edits,
    pub values: BTreeMap<Key, Value>,
}
impl CloneGraph {
    pub(super) fn new(
        nodes: &[AstNode],
        root: Option<CloneRoot<'_>>,
        limit: usize,
    ) -> Result<Self, CompileError> {
        // Reserve the existing provenance before building any COW field views.
        // The session-wide temporary/staging meter remains a separate boundary.
        let retained = super::ast::source_owned_bytes(nodes);
        let root_bytes = root.as_ref().map_or(0, |root| match root {
            CloneRoot::Body(origin) | CloneRoot::Definition(origin) => {
                origin.owned_bytes(&mut BTreeSet::new())
            }
        });
        super::check_limit(
            retained.saturating_add(root_bytes).saturating_mul(3),
            limit,
            "cache_bytes",
        )?;
        let mut graph = Self::build(nodes, root.as_ref());
        let mut seen = BTreeSet::new();
        let retained = graph.edits.owned_bytes(&mut seen);
        let peak = retained
            .saturating_mul(3)
            .saturating_add(graph.edits.current.values.len().saturating_mul(256));
        super::check_limit(peak, limit, "cache_bytes")?;
        graph.seed(root);
        Ok(graph)
    }
    pub(super) fn unlimited(nodes: &[AstNode]) -> Self {
        Self::build(nodes, None)
    }
    fn build(nodes: &[AstNode], root: Option<&CloneRoot<'_>>) -> Self {
        let mut edits = Edits::from_nodes(nodes);
        if let Some(root) = root {
            let origin = match root {
                CloneRoot::Body(origin) | CloneRoot::Definition(origin) => origin,
            };
            edits.sync_root(origin, nodes);
        } else {
            edits.sync(nodes);
        }
        Self {
            edits,
            values: BTreeMap::new(),
        }
    }
    fn seed(&mut self, root: Option<CloneRoot<'_>>) {
        if let Some(root) = root {
            let origin = match &root {
                CloneRoot::Body(origin) | CloneRoot::Definition(origin) => *origin,
            };
            let value = self.edits.fields(origin).clone();
            let root = match root {
                CloneRoot::Body(_) => value.get("nodes").cloned().unwrap_or(value),
                CloneRoot::Definition(_) => value,
            };
            let _ = self.clone_value(&root);
        }
    }
    pub(super) fn clone_value(&mut self, value: &Value) -> Value {
        let current = self.edits.current(value).clone();
        let Some(key) = storage(&current) else {
            return current;
        };
        if let Some(value) = self.values.get(&key) {
            return value.clone();
        }
        let value = match &current {
            Value::Array(values) => Value::Array(Arc::new(
                values.iter().map(|value| self.clone_value(value)).collect(),
            )),
            Value::Object(values) => Value::Object(Arc::new(
                values
                    .iter()
                    .map(|(key, value)| (key.clone(), self.clone_value(value)))
                    .collect(),
            )),
            _ => current.clone(),
        };
        self.values.insert(key, value.clone());
        value
    }
    pub(super) fn clone_origin(&mut self, origin: &NodeOrigin) -> NodeOrigin {
        NodeOrigin::generated(self.clone_value(&origin.0.value))
    }
}

// Canonical current views keep private cloned array aliases together. Caller
// metadata retains its host references, whose ordered effects are replayed separately.
pub(super) struct Projection {
    edits: Edits,
    values: BTreeMap<Key, Value>,
}
impl Projection {
    pub(super) fn new(nodes: &[AstNode]) -> Self {
        let mut edits = Edits::from_nodes(nodes);
        edits.sync(nodes);
        Self {
            edits,
            values: BTreeMap::new(),
        }
    }
    pub(super) fn fields(&mut self, origin: &NodeOrigin) -> Value {
        if origin.is_caller() {
            self.edits.fields(origin).clone()
        } else {
            self.value(&origin.0.value)
        }
    }
    pub(super) fn bind_location(&mut self, original: &Value, published: &Value) {
        let original = self.edits.current(original).clone();
        let Some(storage_key) = storage(&original) else {
            return;
        };
        if self.values.insert(storage_key, published.clone()).is_some() {
            return;
        }
        match (&original, published) {
            (Value::Object(original), Value::Object(published)) => {
                for (key, value) in original.iter() {
                    if let Some(current) = published.get(key) {
                        self.bind_location(value, current);
                    }
                }
            }
            (Value::Array(original), Value::Array(published)) => {
                for (value, current) in original.iter().zip(published.iter()) {
                    if key(value)
                        .is_none_or(|original| original.0 == 0 || key(current) == Some(original))
                    {
                        self.bind_location(value, current);
                    }
                }
            }
            _ => {}
        }
    }
    pub(super) fn location(&mut self, node: &AstNode, value: &Value) -> Value {
        if node.origin().is_some_and(|origin| !origin.is_caller()) {
            self.value(value)
        } else {
            value.clone()
        }
    }
    fn value(&mut self, value: &Value) -> Value {
        let current = self.edits.current(value).clone();
        let Some(key) = storage(&current) else {
            return current;
        };
        if let Some(value) = self.values.get(&key) {
            return value.clone();
        }
        let output = match &current {
            Value::Object(fields) => {
                let mut output = (**fields).clone();
                for value in output.values_mut() {
                    *value = self.value(value);
                }
                if &output == fields.as_ref() {
                    current.clone()
                } else {
                    Value::Object(Arc::new(output))
                }
            }
            Value::Array(values) => {
                let output: Vec<_> = values.iter().map(|value| self.value(value)).collect();
                if &output == values.as_ref() {
                    current.clone()
                } else {
                    Value::Array(Arc::new(output))
                }
            }
            _ => current.clone(),
        };
        self.values.insert(key, output.clone());
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{
        ast,
        public_api::{Source, SourceLocation},
    };

    fn identity(value: &Value) -> usize {
        match value {
            Value::Object(value) => Arc::as_ptr(value.identity()) as usize,
            Value::Array(value) => Arc::as_ptr(value) as usize,
            _ => panic!("expected an owned public value"),
        }
    }

    #[test]
    fn rejected_cow_assignment_preserves_the_node_and_completed_journal() {
        let fields = public_json!({"kind":"declaration", "property":"color", "value":"red", "important":false});
        let mut node = AstNode::declaration("color", "red");
        node.set_origin(Some(NodeOrigin::caller(fields.clone())));
        let mut edits = Edits::default();
        edits.assign(&mut node, "value", "green".into()).unwrap();
        let before = node.clone();
        let current_before = edits.current(&fields).clone();
        edits.allowance = Allowance::new(0, 0);
        let result = edits.assign(&mut node, "value", "blue".into());
        assert!(
            matches!(result, Err(CompileError::Limit { resource, limit:0 }) if resource == "cache_bytes")
        );
        assert_eq!(node, before, "a refused write must preserve typed fields");
        assert_eq!(
            edits.mutations.len(),
            1,
            "a refused write must not append a journal entry"
        );
        assert_eq!(edits.current(&fields), &current_before);
    }

    #[test]
    fn earlier_array_storage_reads_the_latest_shared_anchor() {
        let original = public_json!(["removed", "kept"]);
        let first = public_json!(["kept"]);
        let latest = public_json!([]);
        let mut edits = Edits::default();
        edits.insert(original.clone(), first.clone());
        edits.insert(first.clone(), latest.clone());
        assert_eq!(identity(edits.current(&original)), identity(&latest));
        assert_eq!(identity(edits.current(&first)), identity(&latest));
        assert_eq!(first, public_json!(["kept"]));
        assert_eq!(original, public_json!(["removed", "kept"]));
    }

    #[test]
    fn array_lineage_survives_a_bound_snapshot_and_released_edit_owner() {
        let original = public_json!(["old"]);
        let first = public_json!(["captured"]);
        let latest = public_json!([]);
        let mut edits = Edits::default();
        edits.insert(original.clone(), first.clone());
        let snapshot = edits.finish();
        let mut origin = NodeOrigin::caller(public_json!({"nodes": first.clone()}));
        snapshot.bind_origin(&mut origin);
        drop(snapshot);
        let mut resumed = Edits::default();
        resumed.include(&origin);
        resumed.insert(original.clone(), latest.clone());
        assert_eq!(identity(resumed.current(&first)), identity(&latest));
        assert_eq!(first, public_json!(["captured"]));
    }

    #[test]
    fn overwritten_array_storage_is_pinned_until_the_snapshot_is_released() {
        let original = public_json!(["old"]);
        let first = public_json!(["version"]);
        let Value::Array(array) = &first else {
            panic!("expected array")
        };
        let retained = Arc::downgrade(array);
        let mut edits = Edits::default();
        edits.insert(original.clone(), first);
        edits.insert(original, public_json!([]));
        assert!(retained.upgrade().is_some());
        let snapshot = edits.finish();
        assert!(retained.upgrade().is_some());
        drop(snapshot);
        assert!(retained.upgrade().is_none());
    }

    #[test]
    fn assigning_children_changes_one_edge_without_changing_the_shared_array() {
        let shared = public_json!(["old"]);
        let left = public_json!({"nodes": shared.clone()});
        let right = public_json!({"nodes": shared.clone()});
        let replacement = public_json!(["new"]);
        let mut current_left = left.clone();
        current_left["nodes"] = replacement.clone();
        let mut edits = Edits::default();
        edits.insert(left.clone(), current_left);
        edits.insert(shared.clone(), public_json!([]));
        assert_eq!(
            identity(&edits.current(&left)["nodes"]),
            identity(&replacement)
        );
        assert!(edits
            .current(&edits.current(&right)["nodes"])
            .as_array()
            .unwrap()
            .is_empty());
        assert_eq!(shared, public_json!(["old"]));
        assert_eq!(identity(edits.current(&left)), identity(&left));
    }

    #[test]
    fn fresh_clone_resolves_earlier_array_versions_across_body_metadata_and_source() {
        let child = public_json!({"kind":"declaration", "property":"color", "value":"red", "important":false});
        let original = public_json!([child.clone(), public_json!("removed")]);
        let first = public_json!([child.clone()]);
        let latest = public_json!([child.clone(), child.clone()]);
        let fields = public_json!({"kind":"rule", "selector":"a", "nodes":original.clone(), "extra":first.clone(), "src":public_json!([public_json!({"file":"source.css", "code":"a {}", "body":first.clone()}), 0usize, 4usize])});
        let mut declaration = AstNode::declaration("color", "red");
        declaration.set_origin(Some(NodeOrigin::caller(child)));
        let mut node = AstNode::Rule {
            origin: Some(NodeOrigin::caller(fields)),
            source: None,
            destination: None,
            selector: "a".into(),
            nodes: vec![declaration.clone(), declaration],
        };
        let mut edits = Edits::default();
        edits.insert(original.clone(), first.clone());
        edits.insert(first.clone(), latest);
        edits.finish().bind(std::slice::from_mut(&mut node));
        let cloned =
            ast::clone_fresh_checked(std::slice::from_ref(&node), None, 1_000_000).unwrap();
        let fields = cloned[0].origin().unwrap().fields();
        assert_eq!(fields["extra"].as_array().unwrap().len(), 2);
        assert_eq!(identity(&fields["nodes"]), identity(&fields["extra"]));
        assert_eq!(
            identity(&fields["nodes"]),
            identity(&fields["src"][0]["body"])
        );
        assert_eq!(identity(&fields["nodes"][0]), identity(&fields["nodes"][1]));
        assert_ne!(identity(&fields["nodes"]), identity(&original));
        assert_ne!(identity(&fields["nodes"]), identity(&first));
        assert_eq!(first.as_array().unwrap().len(), 1);
        let second = ast::clone_fresh(std::slice::from_ref(&node));
        assert_ne!(
            identity(&second[0].origin().unwrap().fields()["nodes"]),
            identity(&fields["nodes"])
        );
    }

    #[test]
    fn checked_clone_counts_overwritten_array_storage_and_preserves_the_input() {
        let original = public_json!([]);
        let retained = public_json!(["x".repeat(8192)]);
        let mut edits = Edits::default();
        edits.insert(original.clone(), retained.clone());
        edits.insert(original.clone(), public_json!([]));
        let mut node = AstNode::Rule {
            origin: Some(NodeOrigin::caller(
                public_json!({"kind":"rule", "selector":"a", "nodes":original.clone()}),
            )),
            source: None,
            destination: None,
            selector: "a".into(),
            nodes: vec![],
        };
        let snapshot = edits.finish();
        snapshot.bind(std::slice::from_mut(&mut node));
        let before = node.origin().unwrap().fields().clone();
        assert!(
            matches!(ast::clone_fresh_checked(std::slice::from_ref(&node), None, 1024), Err(CompileError::Limit { resource, limit:1024 }) if resource == "cache_bytes")
        );
        assert_eq!(identity(node.origin().unwrap().fields()), identity(&before));
        assert_eq!(
            identity(snapshot.views.value(&retained)),
            identity(snapshot.views.value(&original))
        );
        assert_eq!(retained[0], public_json!("x".repeat(8192)));
    }

    #[test]
    fn included_snapshot_addresses_remain_owned_until_the_edit_owner_is_released() {
        let mut origin = NodeOrigin::caller(public_json!({"kind":"comment", "value":"old"}));
        let snapshot = Edits::default().finish();
        let retained = Arc::downgrade(&snapshot.views);
        snapshot.bind_origin(&mut origin);
        let mut edits = Edits::default();
        edits.include(&origin);
        drop(origin);
        drop(snapshot);
        assert!(retained.upgrade().is_some());
        drop(edits);
        assert!(retained.upgrade().is_none());
    }

    #[test]
    fn native_origin_is_ignored_by_serialization_and_equality_but_retained_by_clone() {
        let mut nodes = ast::clone_fresh(
            &crate::compiler::parser::parse(
                "a { color: red; } @media print { a {} } /* kept */",
                32,
            )
            .unwrap(),
        );
        nodes.push(AstNode::AtRoot {
            origin: None,
            nodes: Vec::new(),
        });
        nodes.push(AstNode::Context {
            origin: None,
            base: Text::default(),
            source_base: None,
            reference: false,
            nodes: Vec::new(),
        });
        let original = nodes.clone();
        let serialized = serde_json::to_value(&original).unwrap();
        fn attach(nodes: &mut [AstNode]) {
            for node in nodes {
                node.set_origin(Some(NodeOrigin::caller(public_json!({"extra": "kept"}))));
                if let Some(children) = node.children_mut() {
                    attach(children);
                }
            }
        }
        attach(&mut nodes);
        assert_eq!(nodes, original);
        assert_eq!(serde_json::to_value(&nodes).unwrap(), serialized);
        for (node, cloned) in nodes.iter().zip(nodes.clone()) {
            assert!(Arc::ptr_eq(
                &node.origin().unwrap().0,
                &cloned.origin().unwrap().0
            ));
        }
    }

    #[test]
    fn native_fresh_clone_resolves_current_child_across_body_metadata_and_source() {
        let child = public_json!({"kind": "declaration", "property": "color", "value": "red", "important": false, "extra": "kept"});
        let body = public_json!([child.clone()]);
        let source = public_json!({"file": "source.css", "code": "a {}", "child": child.clone(), "body": body.clone()});
        let tuple = public_json!([
            source,
            Value::Number(serde_json::Number::from_f64(0.5).unwrap()),
            Value::Number(serde_json::Number::from_f64(7.5).unwrap())
        ]);
        let fields = public_json!({"kind": "rule", "selector": "a", "nodes": body, "extra": child.clone(), "src": tuple.clone(), "dst": tuple.clone()});
        let mut declaration = AstNode::declaration("color", "blue");
        declaration.set_origin(Some(NodeOrigin::caller(child.clone())));
        let location = SourceLocation {
            source: Arc::new(Source {
                file: Some("source.css".into()),
                code: "a {}".into(),
            }),
            start: 0,
            end: 7,
            public: Some(Arc::new(tuple)),
        };
        let node = AstNode::Rule {
            origin: Some(NodeOrigin::caller(fields)),
            source: Some(location.clone()),
            destination: Some(location),
            selector: "a".into(),
            nodes: vec![declaration],
        };
        let cloned =
            ast::clone_fresh_checked(std::slice::from_ref(&node), None, usize::MAX).unwrap();
        let fields = cloned[0].origin().unwrap().fields();
        let child = cloned[0].children().unwrap()[0].origin().unwrap().fields();
        assert_eq!(child["value"], public_json!("blue"));
        assert_eq!(child["extra"], public_json!("kept"));
        assert_eq!(identity(&fields["nodes"][0]), identity(child));
        assert_eq!(identity(&fields["extra"]), identity(child));
        assert_eq!(identity(&fields["src"][0]["child"]), identity(child));
        assert_eq!(
            identity(&fields["src"][0]["body"]),
            identity(&fields["nodes"])
        );
        assert_eq!(identity(&fields["dst"]), identity(&fields["src"]));
        assert_eq!(
            identity(cloned[0].source().unwrap().public.as_deref().unwrap()),
            identity(&fields["src"])
        );
        let second = ast::clone_fresh(std::slice::from_ref(&node));
        assert_ne!(
            identity(second[0].origin().unwrap().fields()),
            identity(fields)
        );
        assert_ne!(identity(node.origin().unwrap().fields()), identity(fields));
    }

    #[test]
    fn normalization_refreshes_an_aliased_native_occurrence_before_a_function_guard() {
        let fields = public_json!({"kind": "declaration", "property": "color", "value": "--alpha(red / 50%)", "important": false});
        let mut first = AstNode::declaration("color", "--alpha(red / 50%)");
        first.set_origin(Some(NodeOrigin::caller(fields.clone())));
        let mut nodes = vec![first.clone(), first];
        let theme = crate::compiler::theme::Theme::default();
        let mut edits = Edits::from_nodes(&nodes);
        crate::compiler::functions::substitute_with_edits(
            &mut nodes, &theme, 32, 262144, &mut edits,
        )
        .unwrap();
        let snapshot = edits.finish();
        snapshot.bind(&mut nodes);
        assert_eq!(snapshot.mutations.len(), 1);
        assert_eq!(
            nodes[0].origin().unwrap().fields()["value"],
            nodes[1].origin().unwrap().fields()["value"]
        );
        assert!(!ast::to_css(&nodes).contains("--alpha("));
    }

    #[test]
    fn checked_native_clone_rejects_metadata_budget_and_keeps_the_caller_graph() {
        let mut node = AstNode::declaration("color", "red");
        let fields = public_json!({"kind": "declaration", "property": "color", "value": "red", "important": false, "metadata": "x".repeat(8192)});
        let before = fields.clone();
        node.set_origin(Some(NodeOrigin::caller(fields)));
        assert!(
            matches!(ast::clone_fresh_checked(std::slice::from_ref(&node), None, 1024), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
        );
        assert_eq!(identity(node.origin().unwrap().fields()), identity(&before));
        assert_eq!(node.origin().unwrap().fields(), &before);
        let cloned = ast::clone_fresh_checked(&[node], None, 1_000_000).unwrap();
        assert_eq!(
            cloned[0].origin().unwrap().fields()["metadata"],
            before["metadata"]
        );
    }
}
