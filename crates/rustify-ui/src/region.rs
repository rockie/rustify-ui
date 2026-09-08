use leptos::prelude::*;
use std::marker::PhantomData;

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

    #[derive(Clone, Copy)]
    enum Slot {
        Pending,
        Live(RegionId),
        Disposed,
    }

    let _ = app;
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
            if let Some(id) = rustify_makepad::create_region::<A>(&canvas, on_action.clone()) {
                *slot = Slot::Live(id);
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
        let previous = std::mem::replace(&mut *slot.lock().unwrap(), Slot::Disposed);
        if let Slot::Live(id) = previous {
            rustify_makepad::destroy_region(id);
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
