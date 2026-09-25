//! Registering a listener on something the page owns.
//!
//! `window`, `document` and a scope's container all outlive the scope, and a
//! `Drop` is what normally takes a listener back off them. After a trap there
//! is no `Drop`: `panic = "abort"` runs no destructors, so every listener the
//! dead instance left behind is still there, still holding a closure into a
//! module nothing can trust, and still being called.
//!
//! So each of them is registered with the instance's own abort signal as well.
//! Dropping is still the normal path and still what happens on an ordinary
//! unmount; aborting is the one removal that works when no Rust can run.

#[cfg(target_arch = "wasm32")]
pub use dom::{listen, ListenOptions, Listener};

#[cfg(target_arch = "wasm32")]
pub(crate) use dom::{instance_signal, page_level};

#[cfg(target_arch = "wasm32")]
mod dom {
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::{AbortSignal, AddEventListenerOptions, Event, EventTarget};
    use send_wrapper::SendWrapper;

    /// How this instance's page-level listeners are registered.
    ///
    /// Bare options when there is no host yet - a scope mounted before the
    /// runtime booted has nothing to be aborted by, and is no worse off than it
    /// was. Callers that also want capture or passive set those on what they
    /// get back.
    pub(crate) fn page_level() -> AddEventListenerOptions {
        rustify_makepad::listener_options().unwrap_or_default()
    }

    /// The signal behind [`page_level`], for what the browser has to be told
    /// to stop without any wasm running - an animation frame, say.
    pub(crate) fn instance_signal() -> Option<AbortSignal> {
        rustify_makepad::listener_options()?.get_signal()
    }

    /// How a [`listen`] registration behaves, beyond the abort signal every
    /// one of them carries.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ListenOptions {
        /// Hear the event on its way down, before anything inside the target.
        pub capture: bool,
        /// Promise never to call `preventDefault`, so the browser can scroll
        /// without waiting for the handler.
        ///
        /// Always stated, never left to the browser: Chrome makes a wheel or
        /// touch listener on the window passive unless told otherwise, and a
        /// handler that cannot cancel what it was registered to cancel fails
        /// without a word.
        pub passive: bool,
    }

    /// A listener on a page-owned target, removed when this is dropped.
    ///
    /// `Send` and `Sync` so that it can go into a cleanup, a stored value or a
    /// context like any other Leptos value; it never leaves the one thread a
    /// browser page has.
    #[must_use = "dropping a Listener removes it"]
    pub struct Listener {
        /// Held for its `Drop`, which is what removes the listener.
        _registered: SendWrapper<Registered>,
    }

    struct Registered {
        target: EventTarget,
        event: String,
        capture: bool,
        callback: Closure<dyn FnMut(Event)>,
    }

    impl Drop for Registered {
        fn drop(&mut self) {
            // Removal matches on the capture flag as well as the callback: a
            // capture listener asked to go as a bubbling one stays.
            let _ = self.target.remove_event_listener_with_callback_and_bool(
                &self.event,
                self.callback.as_ref().unchecked_ref(),
                self.capture,
            );
        }
    }

    /// Listens for `event` on `target` until the returned [`Listener`] is
    /// dropped or this instance fails, whichever comes first.
    ///
    /// The second is what a hand-rolled `add_event_listener` misses: after a
    /// trap no destructor runs, and a listener registered without the
    /// instance's abort signal keeps calling into a module that can no longer
    /// be trusted.
    pub fn listen(
        target: &EventTarget,
        event: &str,
        options: ListenOptions,
        handler: impl FnMut(Event) + 'static,
    ) -> Listener {
        let callback = Closure::<dyn FnMut(Event)>::new(handler);
        let registration = page_level();
        registration.set_capture(options.capture);
        registration.set_passive(options.passive);
        let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
            event,
            callback.as_ref().unchecked_ref(),
            &registration,
        );
        Listener {
            _registered: SendWrapper::new(Registered {
                target: target.clone(),
                event: event.to_owned(),
                capture: options.capture,
                callback,
            }),
        }
    }
}
