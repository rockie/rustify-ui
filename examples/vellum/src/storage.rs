//! Browser persistence belongs to the mounted editor and only writes committed data.

use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use js_sys::{Function, Promise};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AbortSignal, BeforeUnloadEvent, Event, EventTarget, IdbDatabase, IdbRequest, IdbTransaction,
    IdbTransactionMode,
};

use crate::{app::Editor, document::Document};

const SAVE_FAILURE: &str =
    "Local storage is full or unavailable. Export your .vellum file to keep your work.";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = __vellumAbortResource)]
    fn abort_resource(signal: &AbortSignal, resource: &JsValue, kind: &str) -> Function;
}

/// Native JS teardown remains callable when a trap prevents Rust destructors.
struct NativeResource(Function);
impl Drop for NativeResource {
    fn drop(&mut self) {
        let _ = self.0.call0(&JsValue::UNDEFINED);
    }
}

fn native_resource(resource: &JsValue, kind: &str) -> Option<NativeResource> {
    let signal = rustify_makepad::listener_options()?.get_signal()?;
    Some(NativeResource(abort_resource(&signal, resource, kind)))
}

fn listen(target: &EventTarget, name: &str, callback: &Function) -> Result<(), JsValue> {
    let options = rustify_makepad::listener_options()
        .ok_or_else(|| JsValue::from_str("Runtime lifecycle is unavailable"))?;
    target.add_event_listener_with_callback_and_add_event_listener_options(name, callback, &options)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveStatus {
    Hydrating,
    Saving,
    Saved,
    Failed,
}

impl SaveStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Hydrating => "Loading locally",
            Self::Saving => "Saving locally",
            Self::Saved => "Saved locally",
            Self::Failed => "Save failed",
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Options {
    pub grid: bool,
    pub snap: bool,
    pub rulers: bool,
    pub theme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_color: Option<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            grid: false,
            snap: true,
            rulers: false,
            theme: "dark".into(),
            canvas_color: None,
            extra: BTreeMap::new(),
        }
    }
}

fn local_storage() -> Result<web_sys::Storage, JsValue> {
    window()
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("Local storage is unavailable"))
}

pub fn preferences() -> (Options, bool) {
    let storage = local_storage().ok();
    let options = storage
        .as_ref()
        .and_then(|storage| storage.get_item("vellum-options").ok().flatten())
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();
    let welcomed = storage
        .as_ref()
        .and_then(|storage| storage.get_item("vellum-welcomed").ok().flatten())
        .is_some_and(|value| !value.is_empty());
    (options, !welcomed)
}

struct Snapshot {
    generation: u64,
    text: String,
}

struct Waiter {
    generation: u64,
    resolve: Function,
    reject: Function,
}

#[derive(Default)]
struct State {
    disposed: bool,
    signal: Option<AbortSignal>,
    database: Option<IdbDatabase>,
    database_lifetime: Option<NativeResource>,
    transaction: Option<IdbTransaction>,
    timer_ticket: u64,
    queued: Option<Snapshot>,
    writing: bool,
    dirty: bool,
    generation: u64,
    saved_generation: u64,
    last_revision: u64,
    saved_text: Option<String>,
    waiters: Vec<Waiter>,
    pending: BTreeMap<u64, Function>,
    next_request: u64,
    listeners: Option<Listeners>,
}

#[derive(Clone, Default)]
pub struct Storage(Rc<RefCell<State>>);

impl Storage {
    fn alive(&self) -> bool {
        let state = self.0.borrow();
        !state.disposed && !state.signal.as_ref().is_some_and(AbortSignal::aborted)
    }

    fn register(&self, reject: Function) -> u64 {
        let mut state = self.0.borrow_mut();
        state.next_request += 1;
        let id = state.next_request;
        state.pending.insert(id, reject);
        id
    }

    fn cancel_timer(&self) {
        let mut state = self.0.borrow_mut();
        state.timer_ticket = state.timer_ticket.wrapping_add(1);
    }

    pub fn dispose(&self) {
        let (transaction, database, waiters, pending, listeners) = {
            let mut state = self.0.borrow_mut();
            state.disposed = true;
            state.timer_ticket = state.timer_ticket.wrapping_add(1);
            state.database_lifetime = None;
            state.queued = None;
            (
                state.transaction.take(),
                state.database.take(),
                std::mem::take(&mut state.waiters),
                std::mem::take(&mut state.pending),
                state.listeners.take(),
            )
        };
        drop(listeners);
        if let Some(transaction) = transaction {
            let _ = transaction.abort();
        }
        if let Some(database) = database {
            database.close();
        }
        let error = JsValue::from_str("Vellum is no longer mounted");
        for waiter in waiters {
            let _ = waiter.reject.call1(&JsValue::UNDEFINED, &error);
        }
        for reject in pending.into_values() {
            let _ = reject.call1(&JsValue::UNDEFINED, &error);
        }
    }

    pub fn snapshot(&self) -> Value {
        let state = self.0.borrow();
        json!({"dirty":state.dirty,"generation":state.generation,"savedGeneration":state.saved_generation,"writing":state.writing})
    }

    fn schedule(&self, editor: Editor) {
        if !self.alive() {
            return;
        }
        self.cancel_timer();
        {
            let mut state = self.0.borrow_mut();
            state.dirty = true;
            state.generation += 1;
        }
        editor.storage_status.set(SaveStatus::Saving);
        self.defer(editor);
    }

    fn defer(&self, editor: Editor) {
        let storage = self.clone();
        let ticket = storage.0.borrow().timer_ticket;
        rustify_makepad::defer_after(500, move || {
            if !storage.alive() || storage.0.borrow().timer_ticket != ticket {
                return;
            }
            // Live inspector values and drag previews must never become the recovery file.
            if editor.history.with_value(|history| history.is_pending()) {
                storage.defer(editor);
                return;
            }
            if let Ok(promise) = storage.enqueue(editor) {
                leptos::task::spawn_local(async move {
                    let _ = JsFuture::from(promise).await;
                });
            }
        });
    }

    fn enqueue(&self, editor: Editor) -> Result<Promise, JsValue> {
        if !self.alive() {
            return Err(JsValue::from_str("Vellum is no longer mounted"));
        }
        if editor.history.with_value(|history| history.is_pending()) {
            return Err(JsValue::from_str("Finish the current edit before saving."));
        }
        self.cancel_timer();
        let text = editor
            .doc
            .with_untracked(Document::serialize)
            .map_err(crate::app::js_error)?;
        let (generation, start) = {
            let mut state = self.0.borrow_mut();
            if !state.writing && state.saved_text.as_deref() == Some(&text) {
                state.saved_generation = state.generation;
                state.dirty = false;
                editor.storage_status.set(SaveStatus::Saved);
                return Ok(Promise::resolve(&JsValue::UNDEFINED));
            }
            state.generation += 1;
            let generation = state.generation;
            state.queued = Some(Snapshot { generation, text });
            state.dirty = true;
            let start = !state.writing;
            state.writing = true;
            (generation, start)
        };
        editor.storage_status.set(SaveStatus::Saving);
        let promise = Promise::new(&mut |resolve, reject| {
            self.0.borrow_mut().waiters.push(Waiter {
                generation,
                resolve,
                reject,
            });
        });
        if start {
            let storage = self.clone();
            leptos::task::spawn_local(async move {
                storage.write_loop(editor).await;
            });
        }
        Ok(promise)
    }

    async fn write_loop(&self, editor: Editor) {
        loop {
            let next = self.0.borrow_mut().queued.take();
            let Some(next) = next else {
                self.0.borrow_mut().writing = false;
                return;
            };
            if !self.alive() {
                return;
            }
            let database = self.0.borrow().database.clone();
            let result = if let Some(database) = database {
                self.write_database(&database, &next.text).await
            } else {
                local_storage().and_then(|storage| storage.set_item("vellum-document", &next.text))
            };
            if !self.alive() {
                return;
            }
            let (waiters, latest) = {
                let mut state = self.0.borrow_mut();
                let mut ready = Vec::new();
                let mut remaining = Vec::new();
                for waiter in state.waiters.drain(..) {
                    if waiter.generation <= next.generation {
                        ready.push(waiter);
                    } else {
                        remaining.push(waiter);
                    }
                }
                state.waiters = remaining;
                let latest = state.generation == next.generation && state.queued.is_none();
                if result.is_ok() {
                    state.saved_generation = next.generation;
                    state.saved_text = Some(next.text);
                    if latest {
                        state.dirty = false;
                    }
                }
                (ready, latest)
            };
            match result {
                Ok(()) => {
                    if latest {
                        editor.storage_status.set(SaveStatus::Saved);
                    }
                    for waiter in waiters {
                        let _ = waiter.resolve.call0(&JsValue::UNDEFINED);
                    }
                }
                Err(error) => {
                    failure(editor);
                    for waiter in waiters {
                        let _ = waiter.reject.call1(&JsValue::UNDEFINED, &error);
                    }
                }
            }
        }
    }

    async fn write_database(&self, database: &IdbDatabase, text: &str) -> Result<(), JsValue> {
        let transaction =
            database.transaction_with_str_and_mode("documents", IdbTransactionMode::Readwrite)?;
        let _transaction_lifetime = native_resource(transaction.as_ref(), "transaction");
        transaction
            .object_store("documents")?
            .put_with_key(&JsValue::from_str(text), &JsValue::from_str("current"))?;
        self.0.borrow_mut().transaction = Some(transaction.clone());
        let mut handlers = None;
        let mut request_id = 0;
        let promise = Promise::new(&mut |resolve, reject| {
            request_id = self.register(reject.clone());
            let complete = Closure::<dyn FnMut(Event)>::new(move |_| {
                let _ = resolve.call0(&JsValue::UNDEFINED);
            });
            let failed = transaction.clone();
            let error = Closure::<dyn FnMut(Event)>::new(move |_| {
                let error = failed
                    .error()
                    .map(JsValue::from)
                    .unwrap_or_else(|| JsValue::from_str("Storage transaction failed"));
                let _ = reject.call1(&JsValue::UNDEFINED, &error);
            });
            let _ = listen(
                transaction.as_ref(),
                "complete",
                complete.as_ref().unchecked_ref(),
            );
            let _ = listen(
                transaction.as_ref(),
                "error",
                error.as_ref().unchecked_ref(),
            );
            let _ = listen(
                transaction.as_ref(),
                "abort",
                error.as_ref().unchecked_ref(),
            );
            handlers = Some((complete, error));
        });
        let result = JsFuture::from(promise).await.map(|_| ());
        if let Some((complete, error)) = &handlers {
            let _ = transaction
                .remove_event_listener_with_callback("complete", complete.as_ref().unchecked_ref());
            let _ = transaction
                .remove_event_listener_with_callback("error", error.as_ref().unchecked_ref());
            let _ = transaction
                .remove_event_listener_with_callback("abort", error.as_ref().unchecked_ref());
        }
        drop(handlers);
        let mut state = self.0.borrow_mut();
        state.pending.remove(&request_id);
        state.transaction = None;
        result
    }
}

fn failure(editor: Editor) {
    if !editor.storage_status.is_disposed()
        && !editor.storage.is_disposed()
        && editor.storage.with_value(Storage::alive)
    {
        editor.storage_status.set(SaveStatus::Failed);
        crate::shell::toast(editor, SAVE_FAILURE);
    }
}

async fn open_database(storage: &Storage) -> Result<IdbDatabase, JsValue> {
    if !storage.alive() {
        return Err(JsValue::from_str("Vellum is no longer mounted"));
    }
    let factory = window()
        .indexed_db()?
        .ok_or_else(|| JsValue::from_str("IndexedDB is unavailable"))?;
    let request = factory.open_with_u32("vellum-editor", 1)?;
    let _open_lifetime = native_resource(request.as_ref(), "open");
    let mut handlers = None;
    let promise = Promise::new(&mut |resolve, reject| {
        let opened = request.clone();
        let owner = storage.clone();
        let success = Closure::<dyn FnMut(Event)>::new(move |_| match opened.result() {
            Ok(value) if owner.alive() => {
                let _ = resolve.call1(&JsValue::UNDEFINED, &value);
            }
            Ok(value) => {
                if let Ok(database) = value.dyn_into::<IdbDatabase>() {
                    database.close();
                }
                let _ = resolve.call0(&JsValue::UNDEFINED);
            }
            Err(error) => {
                let _ = resolve.call1(&JsValue::UNDEFINED, &error);
            }
        });
        let failed = request.clone();
        let error = Closure::<dyn FnMut(Event)>::new(move |_| {
            let error = failed
                .error()
                .ok()
                .flatten()
                .map(JsValue::from)
                .unwrap_or_else(|| JsValue::from_str("Could not open IndexedDB"));
            let _ = reject.call1(&JsValue::UNDEFINED, &error);
        });
        let upgrading = request.clone();
        let upgrade = Closure::<dyn FnMut(Event)>::new(move |_| {
            if let Ok(database) = upgrading
                .result()
                .and_then(|value| value.dyn_into::<IdbDatabase>())
            {
                if database.create_object_store("documents").is_err() {
                    if let Some(transaction) = upgrading.transaction() {
                        let _ = transaction.abort();
                    }
                }
            }
        });
        let _ = listen(
            request.as_ref(),
            "success",
            success.as_ref().unchecked_ref(),
        );
        let _ = listen(request.as_ref(), "error", error.as_ref().unchecked_ref());
        let _ = listen(
            request.as_ref(),
            "upgradeneeded",
            upgrade.as_ref().unchecked_ref(),
        );
        handlers = Some((success, error, upgrade));
    });
    // An open request cannot be canceled. Its callback closes any late connection after disposal.
    let result = JsFuture::from(promise)
        .await
        .and_then(|value| value.dyn_into::<IdbDatabase>());
    if let Some((success, error, upgrade)) = &handlers {
        let _ = request
            .remove_event_listener_with_callback("success", success.as_ref().unchecked_ref());
        let _ =
            request.remove_event_listener_with_callback("error", error.as_ref().unchecked_ref());
        let _ = request
            .remove_event_listener_with_callback("upgradeneeded", upgrade.as_ref().unchecked_ref());
    }
    drop(handlers);
    result
}

async fn read_request(storage: &Storage, request: IdbRequest) -> Result<JsValue, JsValue> {
    let mut handlers = None;
    let mut request_id = 0;
    let promise = Promise::new(&mut |resolve, reject| {
        request_id = storage.register(reject.clone());
        let read = request.clone();
        let rejected = reject.clone();
        let success = Closure::<dyn FnMut(Event)>::new(move |_| match read.result() {
            Ok(value) => {
                let _ = resolve.call1(&JsValue::UNDEFINED, &value);
            }
            Err(error) => {
                let _ = rejected.call1(&JsValue::UNDEFINED, &error);
            }
        });
        let failed = request.clone();
        let error = Closure::<dyn FnMut(Event)>::new(move |_| {
            let error = failed
                .error()
                .ok()
                .flatten()
                .map(JsValue::from)
                .unwrap_or_else(|| JsValue::from_str("Could not read the saved document"));
            let _ = reject.call1(&JsValue::UNDEFINED, &error);
        });
        let _ = listen(
            request.as_ref(),
            "success",
            success.as_ref().unchecked_ref(),
        );
        let _ = listen(request.as_ref(), "error", error.as_ref().unchecked_ref());
        handlers = Some((success, error));
    });
    let result = JsFuture::from(promise).await;
    if let Some((success, error)) = &handlers {
        let _ = request
            .remove_event_listener_with_callback("success", success.as_ref().unchecked_ref());
        let _ =
            request.remove_event_listener_with_callback("error", error.as_ref().unchecked_ref());
    }
    drop(handlers);
    storage.0.borrow_mut().pending.remove(&request_id);
    result
}

async fn restore(storage: &Storage, editor: Editor) -> Result<Option<String>, JsValue> {
    let attempt = async {
        let database = open_database(storage).await?;
        if !storage.alive() {
            database.close();
            return Err(JsValue::from_str("Vellum is no longer mounted"));
        }
        {
            let mut state = storage.0.borrow_mut();
            state.database_lifetime = native_resource(database.as_ref(), "database");
            state.database = Some(database.clone());
        }
        let transaction = database.transaction_with_str("documents")?;
        let _transaction_lifetime = native_resource(transaction.as_ref(), "transaction");
        let request = transaction
            .object_store("documents")?
            .get(&JsValue::from_str("current"))?;
        read_request(storage, request)
            .await
            .map(|value| value.as_string())
    }
    .await;
    if !storage.alive() {
        return Err(JsValue::from_str("Vellum is no longer mounted"));
    }
    match attempt {
        Ok(saved) => Ok(saved),
        Err(_) => {
            if let Some(database) = storage.0.borrow_mut().database.take() {
                database.close();
            }
            editor.storage_mode.set("localStorage");
            local_storage()?.get_item("vellum-document")
        }
    }
}

pub fn install(editor: Editor, options: Options) {
    let storage = editor.storage.get_value();
    storage.0.borrow_mut().signal =
        rustify_makepad::listener_options().and_then(|options| options.get_signal());
    let initial = editor.doc.with_untracked(|doc| doc.data.clone());
    let hydration = storage.clone();
    leptos::task::spawn_local(async move {
        let restored = restore(&hydration, editor).await;
        if !hydration.alive() {
            return;
        }
        let unchanged = editor.doc.with_untracked(|doc| doc.data == initial)
            && !editor.history.with_value(|history| history.is_pending());
        let mut storage_failed = false;
        match restored {
            Ok(Some(text)) if !text.is_empty() => match Document::parse(&text) {
                Ok(doc) if unchanged => {
                    hydration.0.borrow_mut().saved_text = doc.serialize().ok();
                    editor.doc.set(doc);
                    editor.selection.set(Vec::new());
                    editor.shell.update(|shell| shell.expanded.clear());
                }
                Ok(_) => {}
                Err(_) => crate::shell::toast(
                    editor,
                    "Saved file could not be restored. Your starter document is open.",
                ),
            },
            Ok(_) => {}
            Err(_) => {
                storage_failed = true;
                failure(editor);
            }
        }
        let loaded_revision = editor.doc.with_untracked(|doc| doc.revision);
        let _ = editor.load_stored_fonts().await;
        if !hydration.alive() {
            return;
        }
        if unchanged && editor.doc.with_untracked(|doc| doc.revision) == loaded_revision {
            editor.fit(None);
        }
        hydration.0.borrow_mut().last_revision = editor.doc.with_untracked(|doc| doc.revision);
        editor.hydrated.set(true);
        let current = editor.doc.with_untracked(Document::serialize).ok();
        if current.is_some() && current == hydration.0.borrow().saved_text {
            editor.storage_status.set(SaveStatus::Saved);
        } else if !storage_failed {
            hydration.schedule(editor);
        }
    });
    Effect::new(move || {
        let revision = editor.doc.with(|doc| doc.revision);
        if !editor.hydrated.get() {
            return;
        }
        let storage = editor.storage.get_value();
        let changed = {
            let mut state = storage.0.borrow_mut();
            let changed = state.last_revision != revision;
            state.last_revision = revision;
            changed
        };
        if changed {
            untrack(move || storage.schedule(editor));
        }
    });
    let saved_options = StoredValue::new_local(serde_json::to_string(&options).ok());
    let settings = Memo::new(move |_| Options {
        theme: if editor.dark.get() { "dark" } else { "light" }.into(),
        grid: editor.grid.get(),
        snap: editor.snap.get(),
        rulers: editor.rulers.get(),
        canvas_color: editor.shell.with(|shell| shell.canvas_color.clone()),
        extra: options.extra.clone(),
    });
    let settings_storage = storage.clone();
    Effect::new(move || {
        let text = settings.with(|options| serde_json::to_string(options).ok());
        if !settings_storage.alive() {
            return;
        }
        if text != saved_options.get_value() {
            if let (Some(text), Ok(storage)) = (&text, local_storage()) {
                if storage.set_item("vellum-options", text).is_ok() {
                    saved_options.set_value(Some(text.clone()));
                }
            }
        }
    });
    let welcome = Memo::new(move |_| editor.shell.with(|shell| shell.welcome));
    let welcome_storage = storage.clone();
    Effect::new(move || {
        if !welcome_storage.alive() {
            return;
        }
        if !welcome.get() {
            if let Ok(storage) = local_storage() {
                let _ = storage.set_item("vellum-welcomed", "1");
            }
        }
    });
    let visibility = Closure::<dyn FnMut(Event)>::new(move |_| {
        if document().hidden() && !editor.storage.is_disposed() {
            let storage = editor.storage.get_value();
            if storage.0.borrow().dirty
                && !editor.history.with_value(|history| history.is_pending())
            {
                if let Ok(promise) = storage.enqueue(editor) {
                    leptos::task::spawn_local(async move {
                        let _ = JsFuture::from(promise).await;
                    });
                }
            }
        }
    });
    let unload = Closure::<dyn FnMut(BeforeUnloadEvent)>::new(move |event: BeforeUnloadEvent| {
        if !editor.storage.is_disposed()
            && (editor
                .storage
                .with_value(|storage| storage.0.borrow().dirty)
                || editor.history.with_value(|history| history.is_pending()))
        {
            event.prevent_default();
            event.set_return_value("");
        }
    });
    let _ = listen(
        document().as_ref(),
        "visibilitychange",
        visibility.as_ref().unchecked_ref(),
    );
    let _ = listen(
        window().as_ref(),
        "beforeunload",
        unload.as_ref().unchecked_ref(),
    );
    storage.0.borrow_mut().listeners = Some(Listeners { visibility, unload });
}

struct Listeners {
    visibility: Closure<dyn FnMut(Event)>,
    unload: Closure<dyn FnMut(BeforeUnloadEvent)>,
}
impl Drop for Listeners {
    fn drop(&mut self) {
        let _ = document().remove_event_listener_with_callback(
            "visibilitychange",
            self.visibility.as_ref().unchecked_ref(),
        );
        let _ = window().remove_event_listener_with_callback(
            "beforeunload",
            self.unload.as_ref().unchecked_ref(),
        );
    }
}

/// Explicit save waits until this committed snapshot has completed its storage transaction.
pub async fn save(editor: Editor) -> Result<(), JsValue> {
    if editor.storage.is_disposed() {
        return Err(JsValue::from_str("Vellum is no longer mounted"));
    }
    if !editor.hydrated.get_untracked() {
        return Err(JsValue::from_str("The saved document is still loading."));
    }
    crate::shell::text_session::finish(editor);
    let promise = editor.storage.get_value().enqueue(editor)?;
    JsFuture::from(promise).await.map(|_| ())
}
