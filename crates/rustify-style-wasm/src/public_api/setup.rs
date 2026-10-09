//! Host-owned setup jobs survive a failed synchronous constructor.
use super::*;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = queueMicrotask, catch)]
    fn queue_microtask(callback: &JsValue) -> Result<(), JsValue>;
}
struct Crossing {
    setup: core::PreparedAstSetup,
    graph: graph::Graph,
    faults: Vec<JsValue>,
    completion: Option<Completion>,
    license_banner: bool,
}
struct Completion {
    resolve: js_sys::Function,
    reject: js_sys::Function,
    input: core::PublicAst,
    input_js: JsValue,
    limits: Limits,
}
impl Crossing {
    fn start(&mut self) -> Result<Vec<core::AstSetupJob>, JsValue> {
        let Self {
            setup,
            graph,
            faults,
            ..
        } = self;
        setup
            .start(|effect| {
                graph.apply(effect).map_err(|error| {
                    let token = faults.len();
                    faults.push(error);
                    token
                })
            })
            .map_err(error)
    }
    fn drive(&mut self, job: core::AstSetupJob) -> Result<Vec<core::AstSetupJob>, JsValue> {
        let Self {
            setup,
            graph,
            faults,
            ..
        } = self;
        setup
            .drive(job, |effect| {
                graph.apply(effect).map_err(|error| {
                    let token = faults.len();
                    faults.push(error);
                    token
                })
            })
            .map_err(error)
    }
    fn fault(&self, fault: core::AstSetupFault) -> JsValue {
        match fault {
            core::AstSetupFault::Semantic(failure) => error(failure),
            core::AstSetupFault::Host(token) => self.faults[token].clone(),
        }
    }
    fn start_async(&mut self, session: Rc<RefCell<Option<Self>>>) -> Result<(), JsValue> {
        let Self {
            setup,
            graph,
            faults,
            ..
        } = self;
        setup
            .start_async(
                |effect| {
                    graph.apply(effect).map_err(|error| {
                        let token = faults.len();
                        faults.push(error);
                        token
                    })
                },
                |job| {
                    if let Err(error) = post_jobs(session.clone(), std::iter::once(job)) {
                        wasm_bindgen::throw_val(error);
                    }
                },
            )
            .map_err(error)
    }
    fn drive_posted(
        &mut self,
        job: core::AstSetupJob,
        session: Rc<RefCell<Option<Self>>>,
    ) -> Result<(), JsValue> {
        let Self {
            setup,
            graph,
            faults,
            ..
        } = self;
        setup
            .drive_async(
                job,
                |effect| {
                    graph.apply(effect).map_err(|error| {
                        let token = faults.len();
                        faults.push(error);
                        token
                    })
                },
                |job| {
                    if let Err(error) = post_jobs(session.clone(), std::iter::once(job)) {
                        wasm_bindgen::throw_val(error);
                    }
                },
            )
            .map_err(error)
    }
    fn publish(&mut self) -> Result<(), JsValue> {
        if self.completion.is_none() {
            return Ok(());
        }
        let Some(result) = self.setup.take_result() else {
            return Ok(());
        };
        let Some(completion) = self.completion.take() else {
            return Ok(());
        };
        match result {
            Ok(mut compiler) => {
                compiler.set_license_banner(self.license_banner);
                let value = AstCompiler {
                    compiler,
                    input: completion.input,
                    input_js: completion.input_js,
                    input_graph: std::mem::take(&mut self.graph),
                    output_graph: graph::Graph::default(),
                    previous: None,
                    limits: completion.limits,
                };
                completion
                    .resolve
                    .call1(&JsValue::UNDEFINED, &JsValue::from(value))?;
            }
            Err(fault) => {
                completion
                    .reject
                    .call1(&JsValue::UNDEFINED, &self.fault(fault))?;
            }
        }
        Ok(())
    }
    fn reject(&mut self, error: JsValue) {
        if let Some(completion) = self.completion.take() {
            if let Err(error) = completion.reject.call1(&JsValue::UNDEFINED, &error) {
                wasm_bindgen::throw_val(error);
            }
        } else {
            wasm_bindgen::throw_val(error);
        }
    }
    fn report_unhandled(&mut self) {
        for fault in self.setup.take_unhandled() {
            // Let the native Promise host track/report the rejection. Do not
            // throw it in the callback or attach a swallowing catch handler.
            let _ = js_sys::Promise::reject(&self.fault(fault));
        }
    }
}
fn post_jobs(
    session: Rc<RefCell<Option<Crossing>>>,
    jobs: impl IntoIterator<Item = core::AstSetupJob>,
) -> Result<(), JsValue> {
    for job in jobs {
        let session = session.clone();
        let callback = Closure::once_into_js(move || {
            // No session RefCell borrow is held across user setters.
            let Some(mut crossing) = session.borrow_mut().take() else {
                return;
            };
            let result = crossing
                .drive_posted(job, session.clone())
                .and_then(|()| crossing.publish());
            if let Err(error) = result {
                crossing.reject(error);
            }
            crossing.report_unhandled();
            let pending = crossing.setup.has_pending_jobs();
            if pending {
                *session.borrow_mut() = Some(crossing);
            }
        });
        queue_microtask(&callback)?;
    }
    Ok(())
}
enum Entry {
    Constructor,
    Promise,
}
fn prepare(
    input: &JsValue,
    config: &str,
    entry: Entry,
) -> Result<(Crossing, core::PublicAst, Limits), JsValue> {
    let config = configuration(config)?;
    if !is_array(input)? {
        return Err(protocol("AST must be an array"));
    }
    let mut budget = Budget {
        bytes: 0,
        limit: config.limits.cache_bytes,
        resource: "cache_bytes",
    };
    let (decoded, input_graph) = graph::read(input, &config.limits, &mut budget)?;
    let PublicValue::Array(nodes) = decoded else {
        return Err(protocol("AST must be an array"));
    };
    let prepare = match entry {
        Entry::Constructor => core::prepare_ast_setup,
        Entry::Promise => core::prepare_ast_setup_async,
    };
    let setup = prepare(
        nodes.clone(),
        config.options,
        config.assets,
        config.limits.clone(),
    )
    .map_err(error)?;
    if setup
        .cache_bytes()
        .saturating_add(input_graph.retained_bytes())
        .saturating_add(graph::Graph::output_bytes(&[], setup.reserve_mutations()))
        .saturating_add(if matches!(entry, Entry::Promise) {
            std::mem::size_of::<Completion>() + 3 * (64 + 2 * std::mem::size_of::<usize>())
        } else {
            0
        })
        > config.limits.cache_bytes
    {
        return Err(limit("cache_bytes", config.limits.cache_bytes));
    }
    let crossing = Crossing {
        setup,
        graph: input_graph,
        faults: vec![],
        completion: None,
        license_banner: config.license_banner,
    };
    Ok((crossing, nodes, config.limits))
}
pub(super) fn compile(input: JsValue, config: &str) -> Result<AstCompiler, JsValue> {
    let (mut crossing, nodes, limits) = prepare(&input, config, Entry::Constructor)?;
    let mut ready = VecDeque::from(crossing.start()?);
    let result = loop {
        if let Some(result) = crossing.setup.take_result() {
            break result;
        }
        let job = ready
            .pop_front()
            .ok_or_else(|| protocol("AST setup did not settle"))?;
        ready.extend(crossing.drive(job)?);
    };
    crossing.report_unhandled();
    match result {
        Ok(mut compiler) => Ok(AstCompiler {
            compiler: {
                compiler.set_license_banner(crossing.license_banner);
                compiler
            },
            input: nodes,
            input_js: input,
            input_graph: crossing.graph,
            output_graph: graph::Graph::default(),
            previous: None,
            limits,
        }),
        Err(fault) => {
            let error = crossing.fault(fault);
            if !ready.is_empty() {
                post_jobs(Rc::new(RefCell::new(Some(crossing))), ready)?;
            }
            Err(error)
        }
    }
}
/// Compile the supplied AST at the native Promise boundary using virtual assets.
#[wasm_bindgen(js_name = compileAst)]
pub fn compile_ast(input: JsValue, config: &str) -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, reject| {
        let (mut crossing, nodes, limits) = match prepare(&input, config, Entry::Promise) {
            Ok(prepared) => prepared,
            Err(error) => {
                if let Err(error) = reject.call1(&JsValue::UNDEFINED, &error) {
                    wasm_bindgen::throw_val(error);
                }
                return;
            }
        };
        crossing.completion = Some(Completion {
            resolve,
            reject,
            input: nodes,
            input_js: input.clone(),
            limits,
        });
        let session = Rc::new(RefCell::new(None));
        if let Err(error) = crossing
            .start_async(session.clone())
            .and_then(|()| crossing.publish())
        {
            crossing.reject(error);
        }
        crossing.report_unhandled();
        if crossing.setup.has_pending_jobs() {
            *session.borrow_mut() = Some(crossing);
        }
    })
}
