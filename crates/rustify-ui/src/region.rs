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
    on_action: impl Fn(A::Action) + Clone + 'static,
    /// Written by the region as it starts, fails or is disposed.
    #[prop(optional, into)]
    state: Option<RwSignal<RegionState>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView
where
    A: rustify_makepad::RegionApp,
    A::Props: Clone + Send + Sync,
{
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
            match rustify_makepad::create_region::<A>(&canvas, on_action.clone()) {
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
        let previous = std::mem::replace(&mut *slot.lock().unwrap(), Slot::Closed);
        if let Slot::Live(id) = previous {
            rustify_makepad::destroy_region(id);
            publish(RegionState::Disposed);
        }
    });

    view! { <canvas node_ref=canvas class=class data-testid=test_id /> }
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
