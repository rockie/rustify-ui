//! Isolated design experiment. Not linked into the production compiler.
//! Input IDs describe canonical current node/array cells, including aliases.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    io::{self, Read},
};

#[derive(Clone, Deserialize, Serialize)]
struct Node {
    kind: String,
    tag: String,
    children: Option<usize>,
    context: BTreeMap<String, Value>,
    operation: Option<Operation>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum Operation {
    Replace {
        nodes: Vec<usize>,
        action: Action,
    },
    RenameError {
        name: String,
        error: String,
    },
    Error {
        message: String,
    },
    Capture,
    EditArray {
        array: usize,
        index: usize,
        remove: usize,
        nodes: Vec<usize>,
    },
    Children {
        array: usize,
    },
    Action {
        action: Action,
    },
}

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Action {
    #[default]
    Continue,
    Skip,
    Stop,
}

#[derive(Deserialize)]
struct Graph {
    nodes: Vec<Node>,
    arrays: Vec<Vec<usize>>,
    root: usize,
}

#[derive(Default)]
struct Observations {
    visits: Vec<Value>,
    effects: Vec<Value>,
    captured: Vec<usize>,
}

impl Graph {
    fn walk(
        &mut self,
        array: usize,
        path: &mut Vec<usize>,
        context: &BTreeMap<String, Value>,
        observations: &mut Observations,
    ) -> Result<Action, String> {
        let mut index = 0;
        // Re-read the current canonical array; never traverse a cloned body.
        while index < self.arrays[array].len() {
            let id = self.arrays[array][index];
            if self.nodes[id].kind == "context" {
                let mut merged = context.clone();
                merged.extend(self.nodes[id].context.clone());
                if let Some(children) = self.nodes[id].children
                    && matches!(
                        self.walk(children, path, &merged, observations)?,
                        Action::Stop
                    )
                {
                    return Ok(Action::Stop);
                }
                index += 1;
                continue;
            }
            let parent = path.last().copied();
            path.push(id);
            observations
                .visits
                .push(json!({"node": id, "parent": parent,
                "path": path, "context": context}));
            let mut replacement = None;
            let action = match self.nodes[id].operation.clone() {
                Some(Operation::Replace { nodes, action }) => {
                    let count = nodes.len();
                    self.arrays[array].splice(index..index + 1, nodes.clone());
                    observations
                        .effects
                        .push(json!({"type": "replace", "array": array,
                        "index": index, "nodes": nodes}));
                    replacement = Some(count);
                    action
                }
                Some(Operation::RenameError { name, error }) => {
                    self.nodes[id].tag = name.clone();
                    observations
                        .effects
                        .push(json!({"type": "rename", "node": id, "name": name}));
                    self.arrays[array].remove(index);
                    observations
                        .effects
                        .push(json!({"type": "replace", "array": array,
                        "index": index, "nodes": []}));
                    return Err(error);
                }
                Some(Operation::Error { message }) => return Err(message),
                Some(Operation::Capture) => {
                    observations
                        .captured
                        .push(self.nodes[id].children.expect("fixture body"));
                    Action::Continue
                }
                Some(Operation::EditArray {
                    array,
                    index,
                    remove,
                    nodes,
                }) => {
                    self.arrays[array].splice(index..index + remove, nodes.clone());
                    observations
                        .effects
                        .push(json!({"type": "edit-array", "array": array,
                        "index": index, "remove": remove, "nodes": nodes}));
                    Action::Continue
                }
                Some(Operation::Children { array }) => {
                    self.nodes[id].children = Some(array);
                    observations
                        .effects
                        .push(json!({"type": "children", "node": id, "array": array}));
                    Action::Continue
                }
                Some(Operation::Action { action }) => action,
                None => Action::Continue,
            };
            path.pop();
            // The fixed walk handles replacement before Stop: Stop + replacement
            // skips the inserted range and keeps walking the following siblings.
            if let Some(count) = replacement {
                if !matches!(action, Action::Continue) {
                    index += count;
                }
                continue;
            }
            if matches!(action, Action::Stop) {
                return Ok(Action::Stop);
            }
            if !matches!(action, Action::Skip) {
                // Read children after the visitor, because it may reassign them.
                if let Some(children) = self.nodes[id].children {
                    path.push(id);
                    let action = self.walk(children, path, context, observations)?;
                    path.pop();
                    if matches!(action, Action::Stop) {
                        return Ok(Action::Stop);
                    }
                }
            }
            index += 1;
        }
        Ok(Action::Continue)
    }

    fn observe(mut self) -> Value {
        let mut observations = Observations::default();
        let result = self.walk(
            self.root,
            &mut Vec::new(),
            &BTreeMap::new(),
            &mut observations,
        );
        // Deferred readers resolve the current list cell after the whole walk.
        let deferred: Vec<_> = observations
            .captured
            .iter()
            .map(|&id| self.arrays[id].clone())
            .collect();
        json!({"error": result.as_ref().err(), "stopped": matches!(result, Ok(Action::Stop)),
            "visits": observations.visits, "effects": observations.effects,
            "arrays": self.arrays, "nodes": self.nodes, "deferred": deferred})
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin");
    let cases: Vec<Graph> = serde_json::from_str(&input).expect("fixture JSON");
    let observations: Vec<_> = cases.into_iter().map(Graph::observe).collect();
    println!(
        "{}",
        serde_json::to_string(&observations).expect("output JSON")
    );
}
