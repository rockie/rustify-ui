//! A DOM application of the SDK with its default features, built the way an
//! outside consumer builds one: plain Cargo, wasm-bindgen, no Makepad.
//!
//! One scope shows what the runtime host owns for an instance - a toast that
//! hides itself on a timer, a dialog on the overlay stack, a size observer, a
//! listener on the window - and the exports below let a page start several
//! instances of it and kill one.

#[cfg(target_arch = "wasm32")]
mod field_samples;
#[cfg(target_arch = "wasm32")]
mod form_benchmark;
#[cfg(target_arch = "wasm32")]
mod select_samples;

#[cfg(target_arch = "wasm32")]
mod app {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_components::{Button, Dialog, Toaster, Tooltip};
    use rustify_ui::{
        listen, mount, provide_toasts, AbortRelease, AppHandle, ListenOptions, MountConfig,
        ResizeObservation, ToastOptions,
    };
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use wasm_bindgen::prelude::*;

    /// What a page reads back from one scope.
    #[derive(Clone, Copy)]
    struct Counters {
        count: RwSignal<u32>,
        keys: RwSignal<u32>,
        resizes: RwSignal<u32>,
        dialog: RwSignal<bool>,
        tooltip: RwSignal<bool>,
    }

    thread_local! {
        static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
        static COUNTERS: RefCell<BTreeMap<String, Counters>> = const { RefCell::new(BTreeMap::new()) };
        static RELEASES: RefCell<Vec<AbortRelease>> = const { RefCell::new(Vec::new()) };
        static NEXT_HANDLE: std::cell::Cell<u32> = const { std::cell::Cell::new(1) };
    }

    #[component]
    fn Consumer(scope: String) -> impl IntoView {
        let toasts = provide_toasts();
        let counters = Counters {
            count: RwSignal::new(0),
            keys: RwSignal::new(0),
            resizes: RwSignal::new(0),
            dialog: RwSignal::new(false),
            tooltip: RwSignal::new(false),
        };
        COUNTERS.with(|all| all.borrow_mut().insert(scope.clone(), counters));

        // A listener on something the page owns: it has to end when the
        // scope does, and when the instance dies with no destructor run.
        let keys = counters.keys;
        let listener = listen(
            window().as_ref(),
            "keydown",
            ListenOptions::default(),
            move |_| keys.update(|n| *n += 1),
        );
        on_cleanup(move || drop(listener));

        // A size observer the host owns, for the same reason.
        let observed = NodeRef::<leptos::html::Div>::new();
        let observation = StoredValue::new_local(None::<ResizeObservation>);
        let resizes = counters.resizes;
        Effect::new(move |_| {
            if let Some(element) = observed.get() {
                let element: &leptos::web_sys::Element = element.as_ref();
                observation.set_value(rustify_ui::observe_resize(element, move || {
                    resizes.update(|n| *n += 1)
                }));
            }
        });

        let id = move |name: &str| format!("{scope}-{name}");
        let dialog = counters.dialog;
        let dialog_id = StoredValue::new(id("dialog"));
        let tooltip = counters.tooltip;
        let tooltip_anchor = id("tooltip-anchor");
        view! {
            <section data-testid=id("app")>
                <button
                    type="button"
                    data-testid=id("increment")
                    on:click=move |_| counters.count.update(|n| *n += 1)
                >
                    "increment"
                </button>
                <output data-testid=id("count")>{move || counters.count.get()}</output>
                <output data-testid=id("keys")>{move || counters.keys.get()}</output>
                <div node_ref=observed data-testid=id("box") class="w-40 h-10 border border-border">
                    {move || counters.resizes.get()}
                </div>
                <Button
                    test_id=id("toast-show")
                    on_click=move || {
                        toasts.show(
                            "a toast that hides itself",
                            ToastOptions { duration_ms: 400, ..ToastOptions::default() },
                        );
                    }
                >
                    "show a toast"
                </Button>
                <Button test_id=id("dialog-open") on_click=move || dialog.set(true)>
                    "open the dialog"
                </Button>
                <Show when=move || dialog.get() fallback=|| ()>
                    <Dialog
                        test_id=dialog_id.get_value()
                        title="a dialog"
                        open=dialog
                        on_open_change=move |open| dialog.set(open)
                    >
                        <p>"The rest of the scope is inert while this is open."</p>
                    </Dialog>
                </Show>
                // A layer anchored to an element: it has to follow the anchor
                // when the anchor changes size.
                <Tooltip
                    open=tooltip
                    on_open_change=move |open| tooltip.set(open)
                    label="anchored below the button"
                    test_id=id("tooltip")
                >
                    <button type="button" data-testid=tooltip_anchor>
                        "anchor"
                    </button>
                </Tooltip>
                <button
                    type="button"
                    data-testid=id("trap")
                    on:click=move |_| unreachable!("deliberate trap in a DOM handler")
                >
                    "trap in a handler"
                </button>
                <Toaster test_id=id("toaster") />
            </section>
        }
    }

    /// Mounts the application into `container_id`. Refused, with the reason,
    /// when the runtime has not been started.
    #[wasm_bindgen]
    pub fn dom_consumer_mount(container_id: &str, url_owner: bool) -> Result<u32, JsValue> {
        let container = document()
            .get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str("container not found"))?
            .unchecked_into::<leptos::web_sys::HtmlElement>();
        let scope = container_id.to_string();
        let config = MountConfig {
            scope: scope.clone(),
            url_owner,
            base: String::new(),
        };
        let handle = mount(
            container,
            config,
            move || view! { <Consumer scope=scope /> },
        )
        .map_err(|error| JsValue::from_str(&format!("{error:?}: {error}")))?;
        let id = NEXT_HANDLE.with(|next| next.replace(next.get() + 1));
        HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
        Ok(id)
    }

    /// Unmounts a scope. `false` when there was none by that number.
    #[wasm_bindgen]
    pub fn dom_consumer_dispose(handle: u32) -> bool {
        HANDLES
            .with(|handles| handles.borrow_mut().remove(&handle))
            .is_some()
    }

    /// `{"count":n,"keys":n,"resizes":n,"dialog":bool}` for one scope.
    #[wasm_bindgen]
    pub fn dom_consumer_state(scope: &str) -> Option<String> {
        let counters = COUNTERS.with(|all| all.borrow().get(scope).copied())?;
        Some(format!(
            "{{\"count\":{},\"keys\":{},\"resizes\":{},\"dialog\":{}}}",
            counters.count.get_untracked(),
            counters.keys.get_untracked(),
            counters.resizes.get_untracked(),
            counters.dialog.get_untracked()
        ))
    }

    /// Opens or closes one scope's tooltip, as hovering its anchor would.
    #[wasm_bindgen]
    pub fn dom_consumer_tooltip(scope: &str, open: bool) -> bool {
        COUNTERS
            .with(|all| all.borrow().get(scope).copied())
            .map(|counters| counters.tooltip.set(open))
            .is_some()
    }

    /// Names this instance in its diagnostics and in the marks it leaves on
    /// the page.
    #[wasm_bindgen]
    pub fn dom_consumer_identify(runtime: u32) {
        rustify_ui::identify_runtime(runtime, "dom-consumer");
    }

    /// This instance's diagnostic record.
    #[wasm_bindgen]
    pub fn dom_consumer_diagnostics() -> String {
        rustify_ui::report_json()
    }

    /// Whether the build reaches the clipboard, which needs web-sys's
    /// unstable APIs.
    #[wasm_bindgen]
    pub fn dom_consumer_clipboard_available() -> bool {
        rustify_ui::clipboard::available()
    }

    /// Has `release` called when this instance fails. It is JS, so it runs
    /// when no Rust can.
    #[wasm_bindgen]
    pub fn dom_consumer_release_on_abort(release: js_sys::Function) {
        let held = rustify_ui::release_on_abort(release);
        RELEASES.with(|releases| releases.borrow_mut().push(held));
    }

    /// A real trap from inside an export call: wasm32 aborts on panic.
    #[wasm_bindgen]
    pub fn dom_consumer_trap() {
        unreachable!("deliberate trap");
    }

    /// A trap from inside a task the host is running. The export returns
    /// normally; the trap happens on the host's turn.
    #[wasm_bindgen]
    pub fn dom_consumer_trap_deferred() {
        rustify_ui::defer(|| unreachable!("deliberate trap in a deferred task"));
    }
}
