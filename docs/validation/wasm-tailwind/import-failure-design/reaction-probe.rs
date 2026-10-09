//! Disposable scheduling experiment, not the production compiler or an AST gate.
//! Assets contain known import dependencies, not a second CSS parser.
use std::collections::{BTreeMap, VecDeque};

type Outcome = Result<(), String>;
#[derive(Clone, Copy)]
struct Promise(usize);
#[derive(Clone)]
enum Reaction {
    Loaded(usize),
    Nested(usize),
    Member(usize),
    Forward(Promise),
    ParseCss(Promise),
    Caller,
}
struct Cell {
    outcome: Option<Outcome>,
    reactions: Vec<Reaction>,
}
struct Aggregate {
    remaining: usize,
    promise: Promise,
}
struct Task {
    promise: Promise,
    id: String,
    root_index: Option<usize>,
}
#[derive(Clone)]
enum Asset {
    Imports(Vec<String>),
    ParserError,
}
enum Job {
    Marker,
    Reaction(Reaction, Outcome),
}
struct Probe {
    assets: BTreeMap<String, Asset>,
    cells: Vec<Cell>,
    aggregates: Vec<Aggregate>,
    tasks: Vec<Task>,
    ready: VecDeque<Job>,
    trace: Vec<String>,
    epoch: usize,
    caught: Option<(usize, usize, Outcome)>,
    throw_index: Option<usize>,
    throw_nodes: Option<usize>,
}
impl Probe {
    fn promise(&mut self) -> Promise {
        let id = Promise(self.cells.len());
        self.cells.push(Cell {
            outcome: None,
            reactions: vec![],
        });
        id
    }
    fn react(&mut self, promise: Promise, reaction: Reaction) {
        if let Some(outcome) = &self.cells[promise.0].outcome {
            self.ready
                .push_back(Job::Reaction(reaction, outcome.clone()));
        } else {
            self.cells[promise.0].reactions.push(reaction);
        }
    }
    fn settle(&mut self, promise: Promise, outcome: Outcome) {
        let cell = &mut self.cells[promise.0];
        if cell.outcome.is_some() {
            return;
        }
        cell.outcome = Some(outcome.clone());
        for reaction in std::mem::take(&mut cell.reactions) {
            self.ready
                .push_back(Job::Reaction(reaction, outcome.clone()));
        }
    }
    fn scope(&mut self, ids: Vec<String>, root: bool) -> Promise {
        let returned = self.promise();
        let mut members = vec![];
        for (index, id) in ids.into_iter().enumerate() {
            if id == "parameter-error" {
                self.settle(returned, Err("parameters".into()));
                return returned;
            }
            let task_promise = self.promise();
            let loaded = self.promise();
            let task = self.tasks.len();
            self.tasks.push(Task {
                promise: task_promise,
                id: id.clone(),
                root_index: root.then_some(index),
            });
            self.trace.push(format!("[\"load\",{id:?},{}]", self.epoch));
            let outcome = if self.assets.contains_key(&id) {
                Ok(())
            } else {
                Err(format!("missing {id}"))
            };
            self.settle(loaded, outcome);
            self.react(loaded, Reaction::Loaded(task));
            members.push(task_promise);
            if root {
                self.trace
                    .push(format!("[\"index\",\"{index}\",{}]", self.epoch));
                if self.throw_index == Some(index) {
                    self.settle(returned, Err("host index".into()));
                    return returned;
                }
            }
        }
        let all = self.promise();
        let group = self.aggregates.len();
        self.aggregates.push(Aggregate {
            remaining: members.len(),
            promise: all,
        });
        for member in members {
            self.react(member, Reaction::Member(group));
        }
        if self.aggregates[group].remaining == 0 {
            self.settle(all, Ok(()));
        }
        // The upstream early return for an empty promises array has no await.
        if self.aggregates[group].remaining == 0 {
            self.settle(returned, Ok(()));
        } else {
            self.react(all, Reaction::Forward(returned));
        }
        returned
    }
    fn run(&mut self, ids: Vec<String>) {
        // The marker belongs only to the experiment; it is not a delay rule.
        self.ready.push_back(Job::Marker);
        let scope = self.scope(ids, true);
        let parse_css = self.promise();
        let compile_ast = self.promise();
        self.react(scope, Reaction::ParseCss(parse_css));
        self.react(parse_css, Reaction::Forward(compile_ast));
        self.react(compile_ast, Reaction::Caller);
        while let Some(job) = self.ready.pop_front() {
            let Job::Reaction(reaction, outcome) = job else {
                self.epoch += 1;
                if !self.ready.is_empty() {
                    self.ready.push_back(Job::Marker);
                }
                continue;
            };
            match reaction {
                Reaction::Loaded(task) => {
                    if let Err(error) = outcome {
                        self.settle(self.tasks[task].promise, Err(error));
                        continue;
                    }
                    match self.assets[&self.tasks[task].id].clone() {
                        Asset::ParserError => {
                            self.settle(self.tasks[task].promise, Err("parser".into()))
                        }
                        Asset::Imports(ids) => {
                            let child = self.scope(ids, false);
                            self.react(child, Reaction::Nested(task));
                        }
                    }
                }
                Reaction::Nested(task) => {
                    let mut outcome = outcome;
                    if outcome.is_ok() {
                        if let Some(index) = self.tasks[task].root_index {
                            self.trace
                                .push(format!("[\"nodes\",\"{index}\",{}]", self.epoch));
                            if self.throw_nodes == Some(index) {
                                outcome = Err("host nodes".into());
                            }
                        }
                    }
                    self.settle(self.tasks[task].promise, outcome);
                }
                Reaction::Member(group) => {
                    let all = &mut self.aggregates[group];
                    let returned = all.promise;
                    match outcome {
                        Err(error) => self.settle(returned, Err(error)),
                        Ok(()) => {
                            all.remaining -= 1;
                            if all.remaining == 0 {
                                self.settle(returned, Ok(()));
                            }
                        }
                    }
                }
                Reaction::ParseCss(promise) => {
                    if outcome.is_ok() {
                        // Actual parseCss success await on compatibility hooks.
                        let compatibility = self.promise();
                        self.settle(compatibility, Ok(()));
                        self.react(compatibility, Reaction::Forward(promise));
                    } else {
                        self.settle(promise, outcome);
                    }
                }
                Reaction::Forward(promise) => self.settle(promise, outcome),
                Reaction::Caller => self.caught = Some((self.epoch, self.trace.len(), outcome)),
            }
        }
    }
}
fn main() {
    let mut assets = BTreeMap::from([
        ("leaf.css".into(), Asset::Imports(vec![])),
        ("parser.css".into(), Asset::ParserError),
        (
            "nestedparser.css".into(),
            Asset::Imports(vec!["parser.css".into()]),
        ),
    ]);
    for depth in 1..=10 {
        assets.insert(
            format!("chain{depth}.css"),
            Asset::Imports(vec![if depth == 1 {
                "leaf.css".into()
            } else {
                format!("chain{}.css", depth - 1)
            }]),
        );
        assets.insert(
            format!("bad{}.css", depth - 1),
            Asset::Imports(vec![if depth == 1 {
                "missing.css".into()
            } else {
                format!("bad{}.css", depth - 2)
            }]),
        );
    }
    let mut cases = vec![];
    for depth in 0..=10 {
        cases.push((
            format!("chain{depth}-then-missing"),
            vec![
                if depth == 0 {
                    "leaf.css".into()
                } else {
                    format!("chain{depth}.css")
                },
                "missing.css".into(),
            ],
            None,
            None,
        ));
    }
    for depth in 0..4 {
        cases.push((
            format!("nestedbad{depth}-leaf"),
            vec![format!("bad{depth}.css"), "leaf.css".into()],
            None,
            None,
        ));
    }
    for (name, ids) in [
        (
            "nestedgood-first-missing",
            vec!["chain3.css", "missing.css"],
        ),
        ("leaf-chain3", vec!["leaf.css", "chain3.css"]),
        ("chain3-leaf", vec!["chain3.css", "leaf.css"]),
        ("badparams-leaf", vec!["leaf.css", "parameter-error"]),
        ("leaf-parsererror", vec!["leaf.css", "parser.css"]),
        (
            "leaf-nestedparsererror",
            vec!["leaf.css", "nestedparser.css"],
        ),
    ] {
        cases.push((
            name.into(),
            ids.into_iter().map(String::from).collect(),
            None,
            None,
        ));
    }
    cases.push((
        "nested-error-versus-direct".into(),
        vec!["bad0.css".into(), "missing.css".into()],
        None,
        None,
    ));
    cases.push((
        "missing-versus-sync-params".into(),
        vec!["missing.css".into(), "parameter-error".into()],
        None,
        None,
    ));
    cases.push((
        "host-index-first".into(),
        vec!["leaf.css".into(), "chain3.css".into()],
        Some(0),
        None,
    ));
    cases.push((
        "host-nodes-first".into(),
        vec!["leaf.css".into(), "chain3.css".into()],
        None,
        Some(0),
    ));
    for (name, ids, throw_index, throw_nodes) in cases {
        let mut probe = Probe {
            assets: assets.clone(),
            cells: vec![],
            aggregates: vec![],
            tasks: vec![],
            ready: VecDeque::new(),
            trace: vec![],
            epoch: 0,
            caught: None,
            throw_index,
            throw_nodes,
        };
        probe.run(ids);
        let (epoch, seen, outcome) = probe.caught.expect("finite fixture settles");
        println!(
            "{{\"name\":{name:?},\"atCatch\":{epoch},\"trace\":[{}],\"afterCatch\":[{}],\"error\":{}}}",
            probe.trace[..seen].join(","),
            probe.trace[seen..].join(","),
            outcome
                .err()
                .map_or("null".into(), |error| format!("{error:?}"))
        );
    }
}
