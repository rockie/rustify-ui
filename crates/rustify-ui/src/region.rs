#[cfg(target_arch = "wasm32")]
use crate::binding::{ActionSink, Binding};
use crate::diagnostics::UiError;
#[cfg(target_arch = "wasm32")]
use crate::diagnostics::{note, record, ErrorKind};
use leptos::prelude::*;
use std::marker::PhantomData;

/// Lifecycle of one GPU region, as the application can observe it.
///
/// `Starting` covers the window between the canvas existing and the region
/// owning a `Cx`; a region that never gets there ends in `Failed` and leaves
/// the surrounding DOM untouched. `Suspended` is a region whose canvas has no
/// drawable area - hidden, or laid out to zero - and is not a failure: it
/// keeps its state and draws again at the size it comes back with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RegionState {
    #[default]
    Starting,
    Ready,
    Suspended,
    /// The canvas lost its GL context. Unlike `Failed`, this one is on its way
    /// back: the region is being rebuilt on a new canvas and the application's
    /// current state will be projected into it. No application state is lost,
    /// because the region never held any.
    Lost,
    Failed(UiError),
    Disposed,
}

/// When a region asks the canvas for a context, counting from the first ask.
///
/// A browser that is taking a context away from something else refuses the
/// next one that asks, and usually has it a moment later; a region that gave
/// up on the first no would send an application down the DOM path over a
/// hiccup. Three asks, and the last of them is answered well inside the two
/// seconds R33 gives a failure to reach the screen - a region that kept
/// asking would instead sit there saying nothing at all.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
const GPU_ATTEMPTS: [i32; 3] = [0, 250, 750];

/// How long to wait before `attempt`, or `None` when there are no more.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn gpu_attempt_delay(attempt: usize) -> Option<i32> {
    GPU_ATTEMPTS.get(attempt).copied()
}

/// What the record says about the retry that is about to happen.
///
/// One string per attempt rather than a formatted number: a diagnostic's
/// detail is `&'static str` on purpose, so that nothing a user typed can ever
/// reach the record through one.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn gpu_retry_detail(attempt: usize) -> &'static str {
    match attempt {
        1 => "the canvas gave no WebGL2 context; asking again in 250 ms (2 of 3)",
        _ => "the canvas gave no WebGL2 context; asking again in 750 ms (3 of 3)",
    }
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
    /// The region's canvas, for an application that has to anchor something
    /// to a rectangle inside it.
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Canvas>>,
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
    use std::sync::atomic::{AtomicBool, Ordering};
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
            let denied = submit_all(&sink, &binding, actions, A::pace, on_action.clone());
            if denied > 0 {
                if let Some(refused) = refused {
                    refused.update(|count| *count += denied);
                }
            }
            drain_soon(sink.clone());
        }
    };
    // Written by the host before the region is even handed back, so the
    // component knows whether it may call itself ready.
    let suspended = Arc::new(AtomicBool::new(false));
    let on_suspended = {
        let suspended = suspended.clone();
        move |now: bool| {
            suspended.store(now, Ordering::Relaxed);
            publish(if now {
                RegionState::Suspended
            } else {
                RegionState::Ready
            });
        }
    };
    let canvas = node_ref.unwrap_or_else(NodeRef::<Canvas>::new);
    // Raised when the browser gives the canvas its context back. A region is
    // built again only then: everything between the loss and the restore
    // would be drawn into a context that is gone.
    let generation = RwSignal::new(0u32);
    // Which ask of the canvas this is. Raised by the wait between two of
    // them, so the effect below runs again without anything else having
    // changed; reset when a restored context makes it a fresh start.
    let attempt = RwSignal::new(0usize);
    // Kept outside the reactive arena so the cleanup closure can still reach
    // the region after the owner's nodes are gone, and so an effect that runs
    // after cleanup cannot create a region nobody will destroy.
    let slot: Arc<Mutex<Slot>> = Arc::new(Mutex::new(Slot::Pending));

    // Ends the region whose context went away and asks for a new canvas. The
    // application hears `Lost` rather than `Failed`: nothing it owns is gone,
    // and it is about to see the same values drawn again.
    let on_context_lost = {
        let slot = slot.clone();
        move || {
            let previous = {
                let mut slot = slot.lock().unwrap();
                match *slot {
                    Slot::Live(id) => {
                        *slot = Slot::Pending;
                        Some(id)
                    }
                    _ => None,
                }
            };
            let Some(id) = previous else {
                return;
            };
            record(
                note(
                    ErrorKind::GpuContextLost,
                    "the canvas lost its WebGL context; waiting for the browser to restore it",
                )
                .in_region(id.raw()),
            );
            // Not a failure: the application keeps its state and the region
            // is built again as soon as the canvas has a context. Until then
            // there is nothing to draw with, so nothing is drawn.
            publish(RegionState::Lost);
            rustify_makepad::destroy_region(id);
        }
    };

    // The other half of the platform's contract: the loss is prevented from
    // being permanent (the host's own listener does that), and the browser
    // says when the canvas can be drawn into again.
    Effect::new(move || {
        let Some(element) = canvas.get() else {
            return;
        };
        let target: leptos::web_sys::EventTarget = element.into();
        let restored = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            attempt.set(0);
            generation.update(|round| *round += 1);
        }) as Box<dyn FnMut()>);
        let _ = target.add_event_listener_with_callback(
            "webglcontextrestored",
            leptos::wasm_bindgen::JsCast::unchecked_ref(restored.as_ref()),
        );
        let holder = StoredValue::new_local(Some((target, restored)));
        on_cleanup(move || {
            if let Some((target, restored)) = holder.try_update_value(Option::take).flatten() {
                let _ = target.remove_event_listener_with_callback(
                    "webglcontextrestored",
                    leptos::wasm_bindgen::JsCast::unchecked_ref(restored.as_ref()),
                );
            }
        });
    });

    Effect::new({
        let slot = slot.clone();
        move || {
            // All three are dependencies: the canvas appearing starts the
            // first region, a restored context starts the next one, and a
            // wait between two asks of the same canvas starts this one again.
            generation.get();
            let asked = attempt.get();
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
            match rustify_makepad::create_region::<A>(
                &canvas,
                deliver_actions.clone(),
                on_suspended.clone(),
                on_context_lost.clone(),
            ) {
                Some(id) => {
                    *slot = Slot::Live(id);
                    // The region starts at whatever its script declares, so
                    // the current projection is applied before anything is
                    // drawn. A rebuilt region needs this: the props signal has
                    // not changed, so its own effect will not run.
                    let current = props.get_untracked();
                    rustify_makepad::apply::<A>(id, move |cx, app| app.apply_props(cx, &current));
                    if !suspended.load(Ordering::Relaxed) {
                        publish(RegionState::Ready);
                    }
                }
                // A refusal is not yet a failure. The region stays
                // `Starting` and asks again; only the last ask decides.
                None => match gpu_attempt_delay(asked + 1) {
                    Some(wait) => {
                        record(note(ErrorKind::GpuInitRetry, gpu_retry_detail(asked + 1)));
                        rustify_makepad::defer_after(wait, move || attempt.set(asked + 1));
                    }
                    None => {
                        *slot = Slot::Closed;
                        record(note(
                            ErrorKind::GpuInitFailed,
                            "the canvas gave no WebGL2 context after three asks",
                        ));
                        publish(RegionState::Failed(UiError::GpuUnavailable));
                    }
                },
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

    // The pixels a region draws are decoration: every control it draws that
    // means something has a DOM entry of its own, and a screen reader that
    // walked the canvas would only find a second, mute copy of it.
    view! { <canvas node_ref=canvas class=class data-testid=test_id aria-hidden="true" /> }
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

/// The turn is taken through the runtime host rather than `setTimeout`: a
/// runtime that fails between two batches has to be able to drop what is
/// still scheduled, and only the host can.
#[cfg(target_arch = "wasm32")]
fn next_turn(sink: ActionSink) {
    rustify_makepad::defer(move || {
        if sink.drain_once() {
            next_turn(sink);
        }
    });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_canvas_is_asked_three_times_and_the_answer_is_inside_two_seconds() {
        assert_eq!(
            gpu_attempt_delay(0),
            Some(0),
            "the first ask is not a retry"
        );
        assert_eq!(gpu_attempt_delay(1), Some(250));
        assert_eq!(gpu_attempt_delay(2), Some(750));
        assert_eq!(gpu_attempt_delay(3), None, "three asks and no more");
        // R33 gives a failure two seconds to be on screen, and the last ask
        // has to have been made and answered inside that.
        let total: i32 = GPU_ATTEMPTS.iter().sum();
        assert!(
            total < 2_000,
            "{total} ms is too long to wait before saying so"
        );
    }

    #[test]
    fn every_retry_says_which_one_it_is() {
        let details: Vec<&str> = (1..GPU_ATTEMPTS.len()).map(gpu_retry_detail).collect();
        assert_eq!(
            details.len(),
            2,
            "the first ask is not announced as a retry"
        );
        for (index, detail) in details.iter().enumerate() {
            assert!(detail.contains(&format!("{} of 3", index + 2)), "{detail}");
        }
    }
}
