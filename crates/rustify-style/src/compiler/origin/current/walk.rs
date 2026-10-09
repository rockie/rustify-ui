//! Mutation-aware traversal of the current AST graph.

use super::{clone_bytes, Allowance, Captured, Current, Value};
use crate::compiler::{public_api::PublicObject, CompileError};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(in crate::compiler) enum Action {
    #[default]
    Continue,
    Skip,
    Stop,
}

pub(in crate::compiler) struct Visit<'a> {
    current: &'a mut Current,
    allowance: &'a mut Allowance,
    array: &'a Arc<Vec<Value>>,
    index: usize,
    path: &'a [Value],
    context: &'a PublicObject,
    replacement: Option<usize>,
}
impl Visit<'_> {
    pub(in crate::compiler) fn values<'a>(&'a self, array: &'a Arc<Vec<Value>>) -> &'a [Value] {
        self.current.array(array)
    }
    pub(in crate::compiler) fn walk<F>(
        &mut self,
        root: &Arc<Vec<Value>>,
        depth_limit: usize,
        visit: &mut F,
    ) -> Result<Action, CompileError>
    where
        F: FnMut(&Value, &mut Visit<'_>) -> Result<Action, CompileError>,
    {
        self.current.walk(root, self.allowance, depth_limit, visit)
    }
    pub(in crate::compiler) fn value<'a>(&'a self, value: &'a Value) -> &'a Value {
        self.current.value(value)
    }
    pub(in crate::compiler) fn array(&self) -> &Arc<Vec<Value>> {
        self.array
    }
    pub(in crate::compiler) fn index(&self) -> usize {
        self.index
    }
    pub(in crate::compiler) fn path(&self) -> &[Value] {
        self.path
    }
    pub(in crate::compiler) fn context(&self) -> &PublicObject {
        self.context
    }
    pub(in crate::compiler) fn assign(
        &mut self,
        target: &Value,
        field: &str,
        value: &Value,
        record_bytes: usize,
    ) -> Result<Captured, CompileError> {
        self.current
            .assign(target, field, value, self.allowance, record_bytes)
    }
    pub(in crate::compiler) fn replace_with(
        &mut self,
        items: &[Value],
        record_bytes: usize,
    ) -> Result<Option<Captured>, CompileError> {
        if self.replacement.is_some() {
            return Ok(None);
        }
        let captured = self.current.splice(
            &Value::Array(self.array.clone()),
            self.index,
            1,
            items,
            self.allowance,
            record_bytes,
        )?;
        self.replacement = Some(items.len());
        Ok(Some(captured))
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "Registration can edit a separately captured body."
        )
    )]
    pub(in crate::compiler) fn splice(
        &mut self,
        array: &Arc<Vec<Value>>,
        index: usize,
        deleted: usize,
        items: &[Value],
        record_bytes: usize,
    ) -> Result<Captured, CompileError> {
        self.current.splice(
            &Value::Array(array.clone()),
            index,
            deleted,
            items,
            self.allowance,
            record_bytes,
        )
    }
}

fn children(value: &Value) -> Result<Option<Arc<Vec<Value>>>, CompileError> {
    match value.get("nodes") {
        None => Ok(None),
        Some(Value::Array(nodes)) => Ok(Some(nodes.clone())),
        _ => Err(CompileError::Syntax {
            offset: 0,
            message: "Invalid public AST nodes".into(),
        }),
    }
}

impl Current {
    pub(in crate::compiler) fn walk<F>(
        &mut self,
        root: &Arc<Vec<Value>>,
        allowance: &mut Allowance,
        depth_limit: usize,
        visit: &mut F,
    ) -> Result<Action, CompileError>
    where
        F: FnMut(&Value, &mut Visit<'_>) -> Result<Action, CompileError>,
    {
        let bytes = std::mem::size_of::<PublicObject>() + 2 * std::mem::size_of::<usize>();
        allowance.reserve(bytes)?;
        let context = PublicObject::new();
        let mut path = Vec::new();
        let result = self.walk_list(root, allowance, depth_limit, 0, &mut path, &context, visit);
        let path_bytes = path.capacity().saturating_mul(std::mem::size_of::<Value>());
        drop(path);
        drop(context);
        allowance.release(bytes.saturating_add(path_bytes));
        result
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "Each recursion carries the current path, context and depth independently."
    )]
    fn walk_list<F>(
        &mut self,
        array: &Arc<Vec<Value>>,
        allowance: &mut Allowance,
        depth_limit: usize,
        depth: usize,
        path: &mut Vec<Value>,
        context: &PublicObject,
        visit: &mut F,
    ) -> Result<Action, CompileError>
    where
        F: FnMut(&Value, &mut Visit<'_>) -> Result<Action, CompileError>,
    {
        crate::compiler::check_limit(depth, depth_limit, "ast_depth")?;
        let bytes = std::mem::size_of::<Visit<'_>>();
        allowance.reserve(bytes)?;
        let result = (|| {
            let mut index = 0;
            while let Some(node) = self.array(array).get(index) {
                let node = self.value(node).clone();
                if node["kind"] == "context" {
                    let overrides =
                        node.get("context")
                            .and_then(Value::as_object)
                            .ok_or_else(|| CompileError::Syntax {
                                offset: 0,
                                message: "Invalid public AST context".into(),
                            })?;
                    let bytes = context.iter().chain(overrides.iter()).fold(
                        std::mem::size_of::<PublicObject>().saturating_add(
                            context
                                .capacity()
                                .saturating_add(overrides.len())
                                .saturating_mul(256),
                        ),
                        |sum, (key, value)| {
                            sum.saturating_add(key.owned_bytes())
                                .saturating_add(clone_bytes(value))
                        },
                    );
                    allowance.reserve(bytes)?;
                    let mut merged = context.clone();
                    merged.extend(
                        overrides
                            .iter()
                            .map(|(key, value)| (key.clone(), value.clone())),
                    );
                    let result = children(&node).and_then(|nodes| match nodes {
                        Some(nodes) => self.walk_list(
                            &nodes,
                            allowance,
                            depth_limit,
                            depth + 1,
                            path,
                            &merged,
                            visit,
                        ),
                        None => Err(CompileError::Syntax {
                            offset: 0,
                            message: "Invalid public AST nodes".into(),
                        }),
                    });
                    drop(merged);
                    allowance.release(bytes);
                    if result? == Action::Stop {
                        return Ok(Action::Stop);
                    }
                    index += 1;
                    continue;
                }
                if path.len() == path.capacity() {
                    let capacity = path.capacity().saturating_mul(2).max(4);
                    allowance.reserve(
                        (capacity - path.capacity()).saturating_mul(std::mem::size_of::<Value>()),
                    )?;
                    path.reserve_exact(capacity - path.len());
                }
                path.push(node.clone());
                let (result, replacement) = {
                    let mut cursor = Visit {
                        current: self,
                        allowance,
                        array,
                        index,
                        path,
                        context,
                        replacement: None,
                    };
                    let result = visit(&node, &mut cursor);
                    (result, cursor.replacement)
                };
                path.pop();
                let action = result?;
                // Replacement is handled before Stop. Skip/Stop skip only the
                // inserted range; Continue revisits this current array position.
                if let Some(count) = replacement {
                    if action != Action::Continue {
                        index += count;
                    }
                    continue;
                }
                if action == Action::Stop {
                    return Ok(Action::Stop);
                }
                if action != Action::Skip {
                    if let Some(children) = children(self.value(&node))? {
                        path.push(node.clone());
                        let result = self.walk_list(
                            &children,
                            allowance,
                            depth_limit,
                            depth + 1,
                            path,
                            context,
                            visit,
                        );
                        path.pop();
                        if result? == Action::Stop {
                            return Ok(Action::Stop);
                        }
                    }
                }
                index += 1;
            }
            Ok(Action::Continue)
        })();
        allowance.release(bytes);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::Text;
    use serde_json::{json, Value as Json};

    struct Fixture {
        input: Json,
        nodes: Vec<Option<Value>>,
        arrays: Vec<Option<Arc<Vec<Value>>>>,
    }
    impl Fixture {
        fn node(&mut self, id: usize) -> Value {
            if let Some(node) = &self.nodes[id] {
                return node.clone();
            }
            let spec = self.input["nodes"][id].clone();
            let mut fields = PublicObject::new();
            fields.insert("kind".into(), spec["kind"].as_str().unwrap().into());
            fields.insert("tag".into(), spec["tag"].as_str().unwrap().into());
            fields.insert("id".into(), id.into());
            fields.insert(
                "context".into(),
                serde_json::from_value(spec["context"].clone()).unwrap(),
            );
            if let Some(child) = spec["children"].as_u64() {
                fields.insert("nodes".into(), Value::Array(self.array(child as usize)));
            }
            let node = Value::Object(Arc::new(fields));
            self.nodes[id] = Some(node.clone());
            node
        }
        fn array(&mut self, id: usize) -> Arc<Vec<Value>> {
            if let Some(array) = &self.arrays[id] {
                return array.clone();
            }
            let ids = self.input["arrays"][id].as_array().unwrap().clone();
            let values: Arc<Vec<Value>> = Arc::new(
                ids.iter()
                    .map(|id| self.node(id.as_u64().unwrap() as usize))
                    .collect(),
            );
            self.arrays[id] = Some(values.clone());
            values
        }
    }
    fn id(value: &Value) -> usize {
        value["id"].as_u64().unwrap() as usize
    }
    fn action(value: &Json) -> Action {
        match value.as_str().unwrap() {
            "continue" => Action::Continue,
            "skip" => Action::Skip,
            "stop" => Action::Stop,
            _ => panic!("fixture action"),
        }
    }
    fn array_id(value: &Arc<Vec<Value>>, arrays: &[Arc<Vec<Value>>]) -> usize {
        arrays
            .iter()
            .position(|array| Arc::ptr_eq(array, value))
            .unwrap()
    }

    #[test]
    fn current_walk_matches_all_fixed_upstream_cursor_cases() {
        let fixture: Json = serde_json::from_str(include_str!(
            "../../../../../../tests/utility/compiler/current-walk-fixtures.json"
        ))
        .unwrap();
        assert_eq!(fixture["cases"].as_array().unwrap().len(), 18);
        for case in fixture["cases"].as_array().unwrap() {
            let input = &case["input"];
            let mut build = Fixture {
                input: input.clone(),
                nodes: vec![None; input["nodes"].as_array().unwrap().len()],
                arrays: vec![None; input["arrays"].as_array().unwrap().len()],
            };
            let nodes: Vec<_> = (0..build.nodes.len()).map(|id| build.node(id)).collect();
            let arrays: Vec<_> = (0..build.arrays.len()).map(|id| build.array(id)).collect();
            let mut current = Current::default();
            let mut allowance = Allowance::default();
            let mut visits = Vec::new();
            let mut effects = Vec::new();
            let mut captured = Vec::new();
            let result = current.walk(&arrays[input["root"].as_u64().unwrap() as usize], &mut allowance, 32, &mut |node, cursor| {
                let node_id = id(node);
                let path = cursor.path().iter().map(id).collect::<Vec<_>>();
                visits.push(json!({"node":node_id,"parent":path.iter().rev().nth(1),"path":path,"context":**cursor.context()}));
                let op = &input["nodes"][node_id]["operation"];
                match op["type"].as_str() {
                    Some("replace") | Some("rename-error") => {
                        if op["type"] == "rename-error" {
                            cursor.assign(node, "tag", &Value::from(op["name"].as_str().unwrap()), 0)?;
                            effects.push(json!({"type":"rename","node":node_id,"name":op["name"]}));
                        }
                        let ids = op.get("nodes").and_then(Json::as_array).cloned().unwrap_or_default();
                        let replacement: Vec<_> = ids.iter().map(|id| nodes[id.as_u64().unwrap() as usize].clone()).collect();
                        let array = array_id(cursor.array(), &arrays);
                        let index = cursor.index();
                        cursor.replace_with(&replacement, 0)?;
                        effects.push(json!({"type":"replace","array":array,"index":index,"nodes":ids}));
                        assert!(cursor.replace_with(std::slice::from_ref(node), 0)?.is_none());
                        if op["type"] == "rename-error" {
                            return Err(CompileError::Syntax { offset: 0, message: op["error"].as_str().unwrap().into() });
                        }
                        return Ok(action(&op["action"]));
                    }
                    Some("error") => return Err(CompileError::Syntax { offset: 0, message: op["message"].as_str().unwrap().into() }),
                    Some("capture") => if let Value::Array(body) = &cursor.value(node)["nodes"] { captured.push(body.clone()); },
                    Some("edit-array") => {
                        let array = op["array"].as_u64().unwrap() as usize;
                        let index = op["index"].as_u64().unwrap() as usize;
                        let remove = op["remove"].as_u64().unwrap() as usize;
                        let ids = op["nodes"].as_array().unwrap();
                        let replacement: Vec<_> = ids.iter().map(|id| nodes[id.as_u64().unwrap() as usize].clone()).collect();
                        cursor.splice(&arrays[array], index, remove, &replacement, 0)?;
                        effects.push(json!({"type":"edit-array","array":array,"index":index,"remove":remove,"nodes":ids}));
                    }
                    Some("children") => {
                        let array = op["array"].as_u64().unwrap() as usize;
                        cursor.assign(node, "nodes", &Value::Array(arrays[array].clone()), 0)?;
                        effects.push(json!({"type":"children","node":node_id,"array":array}));
                    }
                    Some("action") => return Ok(action(&op["action"])),
                    None => {},
                    _ => panic!("fixture operation"),
                }
                Ok(Action::Continue)
            });
            let error = match &result {
                Ok(_) => None,
                Err(CompileError::Syntax { message, .. }) => Some(message.clone()),
                Err(error) => panic!("unexpected fixture error: {error:?}"),
            };
            let final_nodes: Vec<_> = nodes
                .iter()
                .enumerate()
                .map(|(index, node)| {
                    let node = current.value(node);
                    let mut spec = input["nodes"][index].clone();
                    spec["tag"] = serde_json::to_value(&node["tag"]).unwrap();
                    spec["children"] = match node.get("nodes") {
                        Some(Value::Array(array)) => json!(array_id(array, &arrays)),
                        _ => Json::Null,
                    };
                    spec
                })
                .collect();
            let final_arrays: Vec<_> = arrays
                .iter()
                .map(|array| current.array(array).iter().map(id).collect::<Vec<_>>())
                .collect();
            let deferred: Vec<_> = captured
                .iter()
                .map(|array| current.array(array).iter().map(id).collect::<Vec<_>>())
                .collect();
            let actual = json!({"error":error,"stopped":matches!(result,Ok(Action::Stop)),"visits":visits,"effects":effects,"arrays":final_arrays,"nodes":final_nodes,"deferred":deferred});
            assert_eq!(actual, case["expected"], "{}", case["name"]);
        }
    }

    #[test]
    fn cursor_refusal_and_depth_errors_release_temporary_state() {
        let leaf: Value = serde_json::from_value(json!({"kind":"comment","value":"leaf"})).unwrap();
        let root = Arc::new(vec![leaf]);
        let mut current = Current::default();
        let mut allowance = Allowance::new(0, 0);
        let mut visits = 0;
        assert!(matches!(
            current.walk(&root, &mut allowance, 32, &mut |_, _| {
                visits += 1;
                Ok(Action::Continue)
            }),
            Err(CompileError::Limit { limit: 0, .. })
        ));
        assert_eq!(visits, 0);
        assert_eq!(allowance.bytes, 0);
        assert_eq!(allowance.peak, 0);
        assert!(Arc::ptr_eq(current.array(&root), &root));
        let context = Value::Object(Arc::new(PublicObject::from([
            (Text::from("kind"), Value::from("context")),
            (
                Text::from("context"),
                Value::Object(Arc::new(PublicObject::new())),
            ),
            (Text::from("nodes"), Value::Array(root)),
        ])));
        let root = Arc::new(vec![context]);
        let mut allowance = Allowance::default();
        assert!(
            matches!(current.walk(&root, &mut allowance, 0, &mut |_, _| Ok(Action::Continue)), Err(CompileError::Limit { resource, limit: 0 }) if resource=="ast_depth")
        );
        assert_eq!(allowance.bytes, 0);
        assert!(allowance.peak > 0);
    }
}
