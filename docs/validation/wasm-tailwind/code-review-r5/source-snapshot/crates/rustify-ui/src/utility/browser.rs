use super::state::{merge_candidates, serialize_input, Batch, Completion, Input, Outcome, State};
use super::{CnContext, CommittedSnapshot, UtilityConfig, UtilityError};
use crate::{instance_failed, release_on_abort, AbortRelease, ScopeRoots};
use js_sys::{Function, Promise};
use leptos::wasm_bindgen::prelude::*;
use rustify_style::protocol::{
    CompileError, CompileReport, CompilerInput, Limits, SourceAsset, SourceContent, SourceSnapshot,
};
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen(module = "/web/utility-host.js")]
extern "C" {
    type UtilityHost;
    #[wasm_bindgen(constructor, catch)]
    fn new(root: &leptos::web_sys::HtmlElement, base: &str) -> Result<UtilityHost, JsValue>;
    #[wasm_bindgen(method, getter)]
    fn owner(this: &UtilityHost) -> String;
    #[wasm_bindgen(method)]
    fn request(this: &UtilityHost, serialized: &str, timeout: u32) -> Promise;
    #[wasm_bindgen(method, catch)]
    fn install(this: &UtilityHost, css_json: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = commitSheet)]
    fn commit_sheet(this: &UtilityHost);
    #[wasm_bindgen(method, catch)]
    fn classes(this: &UtilityHost, byte_limit: usize) -> Result<String, JsValue>;
    #[wasm_bindgen(method, catch)]
    fn rollback(this: &UtilityHost) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch)]
    fn restart(this: &UtilityHost) -> Result<(), JsValue>;
    #[wasm_bindgen(method)]
    fn cleanup(this: &UtilityHost);
    #[wasm_bindgen(method)]
    fn release(this: &UtilityHost) -> Function;
    #[wasm_bindgen(method)]
    fn observe(this: &UtilityHost, callback: &Function, byte_limit: usize);
}

#[derive(Serialize)]
struct CompilerConfig<'a> {
    input: &'a CompilerInput,
    assets: &'a [SourceAsset],
    limits: &'a Limits,
    owner: String,
    revision_base: u64,
}

/// One independent compiler, stylesheet and committed snapshot for a mounted scope.
#[derive(Clone)]
pub struct UtilityScope(Rc<Inner>);
type ObserverCallback = Closure<dyn FnMut(String)>;
struct Inner {
    cn: CnContext,
    state: RefCell<State>,
    host: UtilityHost,
    config: RefCell<UtilityConfig>,
    request_id: Cell<u32>,
    ready_epoch: Cell<Option<u32>>,
    seed: RefCell<Vec<String>>,
    draining: Cell<bool>,
    _abort: AbortRelease,
    observer: RefCell<Option<ObserverCallback>>,
    last_error: RefCell<Option<UtilityError>>,
}

#[derive(Deserialize)]
struct Response {
    operation: String,
    payload: Box<serde_json::value::RawValue>,
}
#[derive(Deserialize)]
struct Prepared {
    token: u32,
    report: CompileReport,
    candidates: Vec<String>,
}

#[derive(Deserialize)]
struct Ready {
    static_candidates: Vec<String>,
}
#[derive(Deserialize)]
struct Observed {
    classes: Option<Vec<String>>,
    error: Option<UtilityError>,
}

fn protocol(error: impl std::fmt::Display) -> UtilityError {
    UtilityError::Protocol {
        message: error.to_string(),
    }
}
fn js_error(error: JsValue) -> UtilityError {
    error
        .as_string()
        .and_then(|value| serde_json::from_str(&value).ok())
        .unwrap_or_else(|| UtilityError::Worker {
            message: format!("{error:?}"),
        })
}

fn validate_config(config: &UtilityConfig) -> Result<(), UtilityError> {
    for (resource, value) in [
        ("input_bytes", config.limits.input_bytes),
        ("asset_bytes", config.limits.asset_bytes),
        ("candidate_count", config.limits.candidate_count),
        ("ast_depth", config.limits.ast_depth),
        ("css_bytes", config.limits.css_bytes),
        ("queued_bytes", config.limits.queued_bytes),
        ("cache_bytes", config.limits.cache_bytes),
        ("request_timeout_ms", config.request_timeout_ms as usize),
    ] {
        if value == 0 {
            return Err(CompileError::InvalidLimit {
                resource: resource.into(),
            }
            .into());
        }
    }
    for (resource, bytes, limit) in [
        (
            "input_bytes",
            config
                .input
                .css
                .len()
                .saturating_add(config.input.entry.len()),
            config.limits.input_bytes,
        ),
        (
            "asset_bytes",
            config.assets.iter().fold(0usize, |sum, asset| {
                sum.saturating_add(asset.id.len())
                    .saturating_add(asset.content.len())
            }),
            config.limits.asset_bytes,
        ),
    ] {
        if bytes > limit {
            return Err(CompileError::Limit {
                resource: resource.into(),
                limit,
            }
            .into());
        }
    }
    // Validate the configuration envelope before starting a Worker or copying its assets.
    serialize_input(
        &CompilerConfig {
            input: &config.input,
            assets: &config.assets,
            limits: &config.limits,
            owner: "00000000-0000-0000-0000-000000000000".into(),
            revision_base: 0,
        },
        config.limits.queued_bytes,
    )?;
    Ok(())
}

impl UtilityScope {
    pub fn new(roots: &ScopeRoots, config: UtilityConfig) -> Result<Self, UtilityError> {
        let cn = rustify_style::cn::Cn::default_with_options(rustify_style::cn::EngineOptions {
            cache_bytes: Some(config.limits.cache_bytes),
            ..Default::default()
        });
        Self::new_with_cn(roots, config, cn)
    }

    /// Configure component merging in the main realm, retaining native callbacks.
    pub fn new_with_cn(
        roots: &ScopeRoots,
        config: UtilityConfig,
        cn: rustify_style::cn::Cn,
    ) -> Result<Self, UtilityError> {
        validate_config(&config)?;
        if instance_failed() {
            return Err(UtilityError::Disposed);
        }
        let base = format!("{}/utility/", roots.base().trim_end_matches('/'));
        let host = UtilityHost::new(&roots.container(), &base).map_err(js_error)?;
        let abort = release_on_abort(host.release());
        let scope = Self(Rc::new(Inner {
            cn: CnContext::new(cn, config.limits.cache_bytes),
            state: RefCell::new(State::new(config.limits.clone())),
            host,
            config: RefCell::new(config),
            request_id: Cell::new(0),
            ready_epoch: Cell::new(None),
            seed: RefCell::new(Vec::new()),
            draining: Cell::new(false),
            _abort: abort,
            observer: RefCell::new(None),
            last_error: RefCell::new(None),
        }));
        let weak = Rc::downgrade(&scope.0);
        let callback = Closure::<dyn FnMut(String)>::new(move |input: String| {
            if instance_failed() {
                return;
            }
            let Some(inner) = weak.upgrade() else {
                return;
            };
            let observed = match serde_json::from_str::<Observed>(&input) {
                Ok(observed) => observed,
                Err(error) => {
                    *inner.last_error.borrow_mut() = Some(protocol(error));
                    return;
                }
            };
            if let Some(error) = observed.error {
                *inner.last_error.borrow_mut() = Some(error);
                return;
            }
            let Some(classes) = observed.classes else {
                return;
            };
            spawn_local(async move {
                let scope = UtilityScope(inner);
                if let Err(error) = scope.ensure_classes(&classes).await {
                    if !instance_failed() {
                        *scope.0.last_error.borrow_mut() = Some(error);
                    }
                }
            });
        });
        *scope.0.observer.borrow_mut() = Some(callback);
        {
            let callback = scope.0.observer.borrow();
            scope.0.host.observe(
                callback.as_ref().unwrap().as_ref().unchecked_ref(),
                scope.0.config.borrow().limits.queued_bytes,
            );
        }
        Ok(scope)
    }

    pub async fn ensure_classes(&self, classes: &[String]) -> Outcome {
        if instance_failed() {
            return Err(UtilityError::Disposed);
        }
        let completion = self.0.state.borrow_mut().enqueue(classes)?;
        self.wait(completion).await
    }

    /// Wait for previously queued operations and return the installed snapshot without recompiling it.
    pub async fn flush(&self) -> Outcome {
        if instance_failed() {
            return Err(UtilityError::Disposed);
        }
        let completion = self.0.state.borrow_mut().enqueue_flush()?;
        self.wait(completion).await
    }

    /// Extracts raw source candidates and variable dependencies before compiling them.
    pub async fn scan_content(&self, content: &[SourceContent]) -> Outcome {
        if instance_failed() {
            return Err(UtilityError::Disposed);
        }
        let completion = self.0.state.borrow_mut().enqueue_content(content)?;
        self.wait(completion).await
    }

    /// Scans a complete virtual source snapshot using the configured CSS source rules.
    pub async fn scan_sources(&self, sources: &SourceSnapshot) -> Outcome {
        if instance_failed() {
            return Err(UtilityError::Disposed);
        }
        let completion = self.0.state.borrow_mut().enqueue_sources(sources)?;
        self.wait(completion).await
    }

    async fn wait(&self, completion: Completion) -> Outcome {
        if !self.0.draining.replace(true) {
            let inner = self.0.clone();
            spawn_local(async move {
                inner.drain().await;
            });
        }
        let result = completion.await.unwrap_or(Err(UtilityError::Disposed));
        if instance_failed() {
            Err(UtilityError::Disposed)
        } else {
            result
        }
    }

    pub fn snapshot(&self) -> Option<Arc<CommittedSnapshot>> {
        self.0.state.borrow().snapshot.clone()
    }
    /// The last successfully installed configuration, including changes whose ack was lost.
    pub fn configuration(&self) -> UtilityConfig {
        self.0.config.borrow().clone()
    }
    /// Provide this value as Leptos context before constructing component children.
    pub fn cn_context(&self) -> CnContext {
        self.0.cn.clone()
    }
    pub fn last_error(&self) -> Option<UtilityError> {
        self.0.last_error.borrow().clone()
    }

    /// Atomically rebuild CSS, GPU programs and component merging from retained inputs.
    /// A rejected preparation leaves the last installed configuration available for retry.
    pub async fn replace_config(
        &self,
        config: UtilityConfig,
        cn: rustify_style::cn::Cn,
    ) -> Outcome {
        if instance_failed() {
            return Err(UtilityError::Disposed);
        }
        validate_config(&config)?;
        let cn = Rc::new(cn);
        // Reject invalid callbacks before canceling any useful work.
        let staged = Rc::new(
            self.0
                .cn
                .prepare(cn, config.limits.cache_bytes)
                .map_err(protocol)?,
        );
        self.replace(config, staged, None).await
    }

    /// Recompiles the installed configuration from the static seed, current DOM classes,
    /// component fragments and `active`, releasing every other retained candidate.
    /// `active` names what the DOM does not show: GPU classes and scanned variable dependencies.
    /// A failure keeps the installed stylesheet, GPU programs, cn and candidates.
    pub async fn rebuild(&self, active: &[String]) -> Outcome {
        if instance_failed() {
            return Err(UtilityError::Disposed);
        }
        let config = self.configuration();
        let staged = Rc::new(
            self.0
                .cn
                .prepare(self.0.cn.committed_engine(), config.limits.cache_bytes)
                .map_err(protocol)?,
        );
        self.replace(config, staged, Some(active.to_vec())).await
    }

    async fn replace(
        &self,
        config: UtilityConfig,
        staged: Rc<super::cn::PreparedCn>,
        active: Option<Vec<String>>,
    ) -> Outcome {
        let mut bytes = serialize_input(
            &CompilerConfig {
                input: &config.input,
                assets: &config.assets,
                limits: &config.limits,
                owner: self.0.host.owner(),
                revision_base: 0,
            },
            config.limits.queued_bytes,
        )?
        .len();
        if let Some(active) = &active {
            bytes =
                bytes.saturating_add(serialize_input(active, config.limits.queued_bytes)?.len());
        }
        let completion = self
            .0
            .state
            .borrow_mut()
            .enqueue_replace(config, staged, active, bytes)?;
        if let Err(error) = self.0.host.restart() {
            self.0.state.borrow_mut().failed = Some(js_error(error));
        }
        self.0.ready_epoch.set(None);
        self.wait(completion).await
    }

    /// Replays installed candidates and variable dependencies while retaining the last stylesheet.
    pub async fn retry(&self) -> Outcome {
        if self.0.draining.get() {
            return Err(CompileError::PendingPrepared.into());
        }
        let classes = {
            let mut state = self.0.state.borrow_mut();
            if state.disposed {
                return Err(UtilityError::Disposed);
            }
            state.epoch = state
                .epoch
                .checked_add(1)
                .ok_or(CompileError::RevisionExhausted)?;
            state.failed = None;
            state.snapshot.as_ref().map_or_else(Vec::new, |snapshot| {
                snapshot.classes.iter().cloned().collect()
            })
        };
        self.0.host.restart().map_err(js_error)?;
        self.0.ready_epoch.set(None);
        // A fresh epoch must replay known candidates instead of taking the installed no-op path.
        let previous = self.0.state.borrow_mut().snapshot.take();
        let completion = self.0.state.borrow_mut().enqueue(&classes);
        self.0.state.borrow_mut().snapshot = previous;
        let completion = completion?;
        self.0.draining.set(true);
        let inner = self.0.clone();
        spawn_local(async move {
            inner.drain().await;
        });
        let result = completion.await.unwrap_or(Err(UtilityError::Disposed));
        if instance_failed() {
            Err(UtilityError::Disposed)
        } else {
            result
        }
    }

    pub fn dispose(&self) {
        self.0.state.borrow_mut().dispose();
        *self.0.seed.borrow_mut() = Vec::new();
        self.0.host.cleanup();
    }
}
impl Inner {
    async fn send(
        &self,
        epoch: u32,
        operation: &str,
        payload: serde_json::Value,
        expected: &str,
        config: &UtilityConfig,
    ) -> Result<Response, UtilityError> {
        let id = self
            .request_id
            .get()
            .checked_add(1)
            .ok_or(CompileError::RevisionExhausted)?;
        self.request_id.set(id);
        let request = serde_json::json!({ "protocol_version": 1, "runtime_id": self.host.owner(),
            "epoch": epoch, "request_id": id, "operation": operation, "payload": payload });
        let response = JsFuture::from(self.host.request(
            &serialize_input(&request, config.limits.queued_bytes)?,
            config.request_timeout_ms,
        ))
        .await;
        if instance_failed() {
            return Err(UtilityError::Disposed);
        }
        let state = self.state.borrow();
        if state.disposed {
            return Err(UtilityError::Disposed);
        }
        if state.epoch != epoch {
            return Err(UtilityError::Superseded);
        }
        drop(state);
        let response: Response = serde_json::from_str(
            &response
                .map_err(js_error)?
                .as_string()
                .ok_or_else(|| protocol("utility response is not a string"))?,
        )
        .map_err(protocol)?;
        if response.operation != expected {
            return Err(protocol(format!(
                "expected {expected}, received {}",
                response.operation
            )));
        }
        Ok(response)
    }

    async fn compile(&self, batch: &Batch) -> Outcome {
        if let Some(error) = &self.state.borrow().failed {
            return Err(error.clone());
        }
        if matches!(batch.input, Input::Flush) {
            if let Some(snapshot) = &self.state.borrow().snapshot {
                return Ok(snapshot.clone());
            }
        }
        let committed_config = self.config.borrow().clone();
        let config = match &batch.input {
            Input::Replace { config, .. } => config.as_ref(),
            _ => &committed_config,
        };
        let fresh = self.ready_epoch.get() != Some(batch.epoch);
        if fresh {
            let serialized = serialize_input(
                &CompilerConfig {
                    input: &committed_config.input,
                    assets: &committed_config.assets,
                    limits: &committed_config.limits,
                    owner: self.host.owner(),
                    revision_base: self
                        .state
                        .borrow()
                        .snapshot
                        .as_ref()
                        .map_or(0, |s| s.report.revision),
                },
                committed_config.limits.queued_bytes,
            )?;
            let response = self
                .send(
                    batch.epoch,
                    "init",
                    serde_json::json!({"config":serialized}),
                    "ready",
                    &committed_config,
                )
                .await?;
            let ready: Ready = serde_json::from_str(response.payload.get()).map_err(protocol)?;
            let mut seed = std::collections::BTreeSet::new();
            merge_candidates(&mut seed, &ready.static_candidates, &config.limits)?;
            let seed: Vec<String> = seed.into_iter().collect();
            let bytes = seed.iter().fold(
                seed.capacity() * std::mem::size_of::<String>(),
                |sum, value| sum.saturating_add(value.capacity()),
            );
            self.state
                .borrow_mut()
                .set_seed_bytes(bytes, &config.limits)?;
            *self.seed.borrow_mut() = seed;
            self.ready_epoch.set(Some(batch.epoch));
        }
        let (staged_cn, active) = match &batch.input {
            Input::Replace { cn, active, .. } => (Some(cn.clone()), active.as_ref()),
            _ => (None, None),
        };
        let mut retained = std::collections::BTreeSet::new();
        if fresh || staged_cn.is_some() {
            if let Some(snapshot) = &self.state.borrow().snapshot {
                if batch.retains_installed() {
                    retained.clone_from(&snapshot.classes);
                }
            }
            merge_candidates(&mut retained, self.seed.borrow().iter(), &config.limits)?;
        }
        if let Some(active) = active {
            merge_candidates(&mut retained, active, &config.limits)?;
        }
        if let Some(staged) = &staged_cn {
            merge_candidates(&mut retained, &staged.candidates, &config.limits)?;
            let active: Vec<String> = serde_json::from_str(
                &self
                    .host
                    .classes(config.limits.queued_bytes)
                    .map_err(js_error)?,
            )
            .map_err(protocol)?;
            merge_candidates(&mut retained, &active, &config.limits)?;
        }
        if fresh
            && !retained.is_empty()
            && matches!(batch.input, Input::Content(_) | Input::Sources(_))
        {
            // A restored Worker must reconstruct already installed inputs before accepting a scan.
            let response = self
                .send(
                    batch.epoch,
                    "add_candidates",
                    serde_json::json!(serialize_input(&retained, config.limits.queued_bytes)?),
                    "prepared",
                    config,
                )
                .await?;
            let replayed: Prepared =
                serde_json::from_str(response.payload.get()).map_err(protocol)?;
            self.send(
                batch.epoch,
                "ack",
                serde_json::json!({"token":replayed.token}),
                "committed",
                config,
            )
            .await?;
        }
        let (operation, payload) = match &batch.input {
            Input::Replace { .. } => (
                "replace_config",
                serde_json::json!({
                    "config": serialize_input(&CompilerConfig { input: &config.input, assets: &config.assets, limits: &config.limits, owner: self.host.owner(), revision_base: 0 }, config.limits.queued_bytes)?,
                    "candidates": serialize_input(&retained, config.limits.queued_bytes)?,
                }),
            ),
            Input::Classes(classes) => {
                let candidates = if fresh {
                    merge_candidates(&mut retained, classes, &config.limits)?;
                    retained.into_iter().collect()
                } else {
                    classes.clone()
                };
                (
                    "add_candidates",
                    serde_json::json!(serialize_input(&candidates, config.limits.queued_bytes)?),
                )
            }
            Input::Flush => (
                "add_candidates",
                serde_json::json!(serialize_input(&retained, config.limits.queued_bytes)?),
            ),
            Input::Content(content) => (
                "scan_content",
                serde_json::json!(serialize_input(content, config.limits.queued_bytes)?),
            ),
            Input::Sources(sources) => (
                "scan_sources",
                serde_json::json!(serialize_input(sources, config.limits.queued_bytes)?),
            ),
        };
        let response = self
            .send(batch.epoch, operation, payload, "prepared", config)
            .await?;
        let prepared: Prepared = serde_json::from_str(response.payload.get()).map_err(protocol)?;
        let checked = self.state.borrow().validate_with_limits(
            &prepared.report,
            &prepared.candidates,
            &config.limits,
            batch.retains_installed(),
        );
        let installation = checked.and_then(|_| {
            if staged_cn
                .as_ref()
                .is_some_and(|staged| !self.cn.is_current(staged))
            {
                return Err(UtilityError::Superseded);
            }
            let css = serde_json::to_string(&prepared.report.css).map_err(protocol)?;
            self.host.install(&css).map_err(js_error)
        });
        if let Err(error) = installation {
            self.send(
                batch.epoch,
                "nack",
                serde_json::json!({"token":prepared.token}),
                "discarded",
                config,
            )
            .await?;
            return Err(error);
        }
        let installation =
            self.state
                .borrow_mut()
                .installed(batch, prepared.report, &prepared.candidates);
        let installed = match installation {
            Ok(installed) => installed,
            Err(error) => {
                self.host.rollback().map_err(js_error)?;
                self.send(
                    batch.epoch,
                    "nack",
                    serde_json::json!({"token":prepared.token}),
                    "discarded",
                    config,
                )
                .await?;
                return Err(error);
            }
        };
        if let Some(staged) = staged_cn {
            *self.config.borrow_mut() = config.clone();
            self.cn.publish(&staged, batch.epoch);
        } else {
            self.cn.advance_epoch(batch.epoch);
        }
        self.host.commit_sheet();
        self.send(
            batch.epoch,
            "ack",
            serde_json::json!({"token":prepared.token}),
            "committed",
            config,
        )
        .await?;
        Ok(installed)
    }

    async fn drain(self: Rc<Self>) {
        loop {
            let batch = self.state.borrow_mut().take_batch();
            let Some(batch) = batch else {
                break;
            };
            let result = self.compile(&batch).await;
            if instance_failed() {
                break;
            }
            if matches!(
                result,
                Err(UtilityError::Compile { .. }
                    | UtilityError::Install { .. }
                    | UtilityError::Superseded)
            ) && matches!(batch.input, Input::Replace { .. })
                && self.state.borrow().epoch == batch.epoch
            {
                // Restore transport from the installed configuration, never the failed preparation.
                if self.host.restart().is_ok() {
                    self.ready_epoch.set(None);
                }
            }
            if let Err(
                error @ (UtilityError::Timeout
                | UtilityError::Worker { .. }
                | UtilityError::Protocol { .. }),
            ) = &result
            {
                self.state.borrow_mut().failed = Some(error.clone());
            }
            if let Err(error) = &result {
                *self.last_error.borrow_mut() = Some(error.clone());
            }
            self.state.borrow_mut().complete(batch, result);
        }
        if !instance_failed() {
            self.draining.set(false);
        }
    }
}
impl Drop for Inner {
    fn drop(&mut self) {
        self.host.cleanup();
    }
}

#[wasm_bindgen]
pub fn rustify_utility_protocol_version() -> u32 {
    1
}
