//! The page-level host one instance runs under.
//!
//! Everything this instance hands back to the browser - a listener on the
//! window, a task for later, a size observer - has to stop when the instance
//! traps, and a trap runs no destructor. So none of it is registered by Rust
//! alone: the host owns each one, keeps the abort signal they all carry, and
//! drops what is still pending when the instance dies, without calling into
//! the module that died.
//!
//! Two hosts implement the same hooks. A DOM application boots with the
//! runtime this crate ships as a JS module (`web/dom-runtime.js`) through the
//! loader in [`DOM_LOADER_JS`]; an application with GPU regions boots through
//! the Makepad loader, which hands its own host to both Makepad and here.
//! Either way the loader checks [`PROTOCOL`] against what it was written for
//! and the boot below checks it again, so JS from one release and wasm from
//! another refuse to start rather than half-work.

/// The version of the contract between the hooks a loader hands over and the
/// binding below. Raised whenever a hook is added, removed or changes meaning.
pub const PROTOCOL: u32 = 1;

/// The DOM loader, an ES module the page imports: `boot()` instantiates one
/// wasm instance, checks [`PROTOCOL`], creates its host and starts it.
///
/// Readable on any target so that a build step on the host can write it next
/// to the wasm it came with - the same release of this crate, so the two agree.
pub const DOM_LOADER_JS: &str = include_str!("../web/dom-loader.js");

/// The styles the DOM runtime relies on: the failure notice, and the scope's
/// content and overlay planes. Application styling is the application's.
pub const DOM_RUNTIME_CSS: &str = include_str!("../web/dom-runtime.css");

#[cfg(target_arch = "wasm32")]
pub use host::{
    booted, defer, defer_after, instance, listener_options, observe_resize, rustify_runtime_boot,
    rustify_runtime_protocol, ResizeObservation, RuntimeHooks,
};

/// Withdrawn when a host brings its own runtime: such a host creates the
/// page's host itself and owns the boot, so exporting one here would give the
/// page two hosts for one instance.
#[cfg(all(target_arch = "wasm32", not(feature = "external-host")))]
pub use host::rustify_dom_create_host;

#[cfg(target_arch = "wasm32")]
mod host {
    use super::PROTOCOL;
    use leptos::wasm_bindgen::prelude::*;
    use std::cell::RefCell;

    #[wasm_bindgen]
    extern "C" {
        /// What a loader hands to [`rustify_runtime_boot`]. Method names are
        /// the contract with both JS hosts.
        pub type RuntimeHooks;

        #[wasm_bindgen(method, getter)]
        fn protocol(this: &RuntimeHooks) -> u32;

        #[wasm_bindgen(method, getter)]
        fn instance(this: &RuntimeHooks) -> u32;

        #[wasm_bindgen(method, js_name = defer)]
        fn defer_task(this: &RuntimeHooks, callback: &JsValue);

        #[wasm_bindgen(method, js_name = defer_after)]
        fn defer_task_after(this: &RuntimeHooks, callback: &JsValue, ms: i32);

        #[wasm_bindgen(method, js_name = observe_resize)]
        fn observe(
            this: &RuntimeHooks,
            element: &web_sys::Element,
            callback: &JsValue,
        ) -> js_sys::Function;

        #[wasm_bindgen(method, getter)]
        fn signal(this: &RuntimeHooks) -> web_sys::AbortSignal;
    }

    #[cfg(not(feature = "external-host"))]
    #[wasm_bindgen(module = "/web/dom-runtime.js")]
    extern "C" {
        #[wasm_bindgen(js_name = create_dom_host)]
        fn create_dom_host(instance: u32, protocol: u32) -> RuntimeHooks;
    }

    thread_local! {
        static HOOKS: RefCell<Option<RuntimeHooks>> = const { RefCell::new(None) };
    }

    fn with_hooks<R>(f: impl FnOnce(&RuntimeHooks) -> R) -> Option<R> {
        HOOKS.with(|slot| slot.borrow().as_ref().map(f))
    }

    /// The protocol this build speaks, for the loader to compare with its own
    /// before it creates anything.
    #[wasm_bindgen]
    pub fn rustify_runtime_protocol() -> u32 {
        PROTOCOL
    }

    /// A DOM host for instance `instance`, from the JS module this crate ships.
    ///
    /// Created here rather than by the loader because the module is a
    /// wasm-bindgen snippet: its path is the bindgen output's to decide, and
    /// the glue already imports it. The host holds no state between calls, so
    /// every instance gets one of its own.
    #[cfg(not(feature = "external-host"))]
    #[wasm_bindgen]
    pub fn rustify_dom_create_host(instance: u32) -> RuntimeHooks {
        create_dom_host(instance, PROTOCOL)
    }

    /// Starts this instance's runtime with the hooks its loader created.
    ///
    /// Refused when the hooks speak another protocol, or when the instance has
    /// already started: one instance has one host, and a second would leave
    /// the first one's listeners without the signal that ends them.
    #[wasm_bindgen]
    pub fn rustify_runtime_boot(hooks: RuntimeHooks) -> Result<(), JsValue> {
        let protocol = hooks.protocol();
        if protocol != PROTOCOL {
            return Err(js_sys::Error::new(&format!(
                "the runtime host speaks protocol {protocol} and this build {PROTOCOL}; \
                 deploy the loader and the wasm from one build"
            ))
            .into());
        }
        HOOKS.with(|slot| {
            let mut slot = slot.borrow_mut();
            if slot.is_some() {
                return Err(js_sys::Error::new("this instance has already started").into());
            }
            *slot = Some(hooks);
            Ok(())
        })
    }

    /// Whether this instance has a host. A scope is not mounted without one.
    pub fn booted() -> bool {
        HOOKS.with(|slot| slot.borrow().is_some())
    }

    /// The instance number its loader gave it, once it has started.
    pub fn instance() -> Option<u32> {
        with_hooks(|hooks| hooks.instance())
    }

    /// Runs `f` on a fresh browser task, giving the browser its turn in
    /// between.
    ///
    /// The host owns the task rather than `setTimeout` here, so a runtime that
    /// has failed can drop it: the callback is wasm code, and a trapped module
    /// must not be re-entered.
    pub fn defer(f: impl FnOnce() + 'static) {
        let callback = Closure::once_into_js(f);
        with_hooks(|hooks| hooks.defer_task(&callback));
    }

    /// Runs `f` after `ms` milliseconds, on a fresh browser task.
    ///
    /// The host owns the timer for the same reason it owns [`defer`]'s task:
    /// a runtime that fails while the timer is pending drops it instead of
    /// letting it re-enter a trapped module.
    pub fn defer_after(ms: i32, f: impl FnOnce() + 'static) {
        let callback = Closure::once_into_js(f);
        with_hooks(|hooks| hooks.defer_task_after(&callback, ms));
    }

    /// How a listener on something the page owns - `window`, `document`, the
    /// container - must be registered by this instance.
    ///
    /// Removing a listener in `Drop` is the normal path and covers every
    /// ordinary unmount. A trap runs no destructor, and then the only thing
    /// that can take these off the page is the host aborting the signal every
    /// one of them was registered with - so every one of them carries it.
    pub fn listener_options() -> Option<web_sys::AddEventListenerOptions> {
        let signal = with_hooks(|hooks| hooks.signal())?;
        let options = web_sys::AddEventListenerOptions::new();
        options.set_signal(&signal);
        Some(options)
    }

    /// Observes an element's size until dropped or this instance fails.
    /// The host owns the observer so aborting never calls into dead wasm.
    pub fn observe_resize(
        element: &web_sys::Element,
        f: impl FnMut() + 'static,
    ) -> Option<ResizeObservation> {
        let callback = Closure::new(f);
        let release = with_hooks(|hooks| hooks.observe(element, callback.as_ref()))?;
        Some(ResizeObservation {
            release,
            _callback: callback,
        })
    }

    /// A size observation, disconnected before its callback is dropped.
    pub struct ResizeObservation {
        release: js_sys::Function,
        _callback: Closure<dyn FnMut()>,
    }

    impl Drop for ResizeObservation {
        fn drop(&mut self) {
            let _ = self.release.call0(&JsValue::NULL);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_loader_speaks_the_protocol_the_binding_checks() {
        let declared = format!("const PROTOCOL = {PROTOCOL};");
        assert!(
            DOM_LOADER_JS.contains(&declared),
            "dom-loader.js: {declared}"
        );
    }

    #[test]
    fn the_loader_needs_nothing_from_makepad() {
        for import in ["makepad", "wasm_bridge", "embedded.js", "message_bridge"] {
            assert!(
                !DOM_LOADER_JS.contains(import),
                "dom-loader.js mentions {import}"
            );
        }
        assert!(!DOM_LOADER_JS.contains("makepad_resource_base"));
    }

    #[test]
    fn the_runtime_module_is_a_factory_with_no_imports() {
        let runtime = include_str!("../web/dom-runtime.js");
        assert!(
            !runtime.contains("import "),
            "dom-runtime.js imports a module"
        );
        assert!(runtime.contains("export function create_dom_host("));
    }

    #[test]
    fn the_styles_cover_the_planes_the_scope_creates() {
        for selector in [
            ".rustify-fatal",
            "[data-rustify-content]",
            "[data-rustify-overlay]",
            ".rustify-layer",
        ] {
            assert!(DOM_RUNTIME_CSS.contains(selector), "{selector}");
        }
        assert!(!DOM_RUNTIME_CSS.contains(".gpu-region"));
    }
}
