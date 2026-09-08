#[cfg(target_arch = "wasm32")]
use crate::binding::{ActionSink, Binding};
use crate::diagnostics::UiError;
use leptos::prelude::*;
use std::marker::PhantomData;

/// Lifecycle of one GPU region, as the application can observe it.
///
/// `Starting` covers the window between the canvas existing and the region
/// owning a `Cx`; a region that never gets there ends in `Failed` and leaves
/// the surrounding DOM untouched.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RegionState {
    #[default]
    Starting,
    Ready,
    Failed(UiError),
    Disposed,
}

/// One Makepad-drawn area inside a Leptos view.
///
/// `props` is the controlled projection of application state into the region;
/// every change is applied to the region app at the next safe point. Actions
/// the region emits arrive through `on_action` after the pump that produced
/// them has finished. `app` only names the region app type.
#[cfg(target_arch = "wasm32")]
#[component]
pub fn GpuRegion<A>(
    app: PhantomData<A>,
    props: Signal<A::Props>,
    on_action: impl Fn(A::Action) + Clone + Send + Sync + 'static,
    /// Written by the region as it starts, fails or is disposed.
    #[prop(optional, into)]
    state: Option<RwSignal<RegionState>>,
    /// Raised by the number of actions the scope refused because its queue was
    /// full. A refused action did not run and changed nothing, so the
    /// application can report it as not executed and let the user retry.
    #[prop(optional, into)]
    refused: Option<RwSignal<usize>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView
where
    A: rustify_makepad::RegionApp,
    A::Props: Clone + Send + Sync,
    A::Action: Send,
{
    use crate::binding::submit_all;
    use leptos::html::Canvas;
    use rustify_makepad::RegionId;
    use std::sync::{Arc, Mutex};

    /// `Closed` covers both "never started" and "already destroyed": either
    /// way no region belongs to this canvas any more and none will.
    #[derive(Clone, Copy)]
    enum Slot {
        Pending,
        Live(RegionId),
        Closed,
    }

    let _ = app;
    let publish = move |next: RegionState| {
        if let Some(state) = state {
            state.set(next);
        }
    };
    // One order per mount scope; a region rendered outside one orders only
    // against itself.
    let sink = use_context::<ActionSink>().unwrap_or_default();
    // Closed on cleanup: actions still queued for this region hold its
    // callback directly, and the application must not be called for a region
    // it has already taken out of the view.
    let binding = Binding::new();
    let deliver_actions = {
        let binding = binding.clone();
        move |actions: Vec<A::Action>| {
            let denied = submit_all(&sink, &binding, actions, on_action.clone());
            if denied > 0 {
                if let Some(refused) = refused {
                    refused.update(|count| *count += denied);
                }
            }
            drain_soon(sink.clone());
        }
    };
    let canvas = NodeRef::<Canvas>::new();
    // Kept outside the reactive arena so the cleanup closure can still reach
    // the region after the owner's nodes are gone, and so an effect that runs
    // after cleanup cannot create a region nobody will destroy.
    let slot: Arc<Mutex<Slot>> = Arc::new(Mutex::new(Slot::Pending));

    Effect::new({
        let slot = slot.clone();
        move || {
            if !matches!(*slot.lock().unwrap(), Slot::Pending) {
                return;
            }
            let Some(canvas) = canvas.get() else {
                return;
            };
            let mut slot = slot.lock().unwrap();
            if !matches!(*slot, Slot::Pending) {
                return;
            }
            match rustify_makepad::create_region::<A>(&canvas, deliver_actions.clone()) {
                Some(id) => {
                    *slot = Slot::Live(id);
                    publish(RegionState::Ready);
                }
                None => {
                    *slot = Slot::Closed;
                    publish(RegionState::Failed(UiError::GpuUnavailable));
                }
            }
        }
    });

    Effect::new({
        let slot = slot.clone();
        move || {
            let next = props.get();
            if let Slot::Live(id) = *slot.lock().unwrap() {
                rustify_makepad::apply::<A>(id, move |cx, app| app.apply_props(cx, &next));
            }
        }
    });

    on_cleanup(move || {
        binding.close();
        let previous = std::mem::replace(&mut *slot.lock().unwrap(), Slot::Closed);
        if let Slot::Live(id) = previous {
            rustify_makepad::destroy_region(id);
            publish(RegionState::Disposed);
        }
    });

    view! { <canvas node_ref=canvas class=class data-testid=test_id /> }
}

/// Delivers the pending actions, giving the browser a turn between batches so
/// a flood of input cannot hold the frame. The first batch runs immediately:
/// the caller is already at a safe point and the common case is one action.
#[cfg(target_arch = "wasm32")]
fn drain_soon(sink: ActionSink) {
    if !sink.arm() {
        return;
    }
    if sink.drain_once() {
        next_turn(sink);
    }
}

#[cfg(target_arch = "wasm32")]
fn next_turn(sink: ActionSink) {
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;

    let resume = Closure::once_into_js(move || {
        if sink.drain_once() {
            next_turn(sink.clone());
        }
    });
    let _ = leptos::prelude::window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(resume.unchecked_ref(), 0);
}

#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn GpuRegion<A: 'static>(
    app: PhantomData<A>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let _ = app;
    view! { <canvas class=class /> }
}
