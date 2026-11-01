use super::*;
use leptos::wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Element, Event, FocusEvent, PointerEvent, WheelEvent};

#[derive(Default)]
pub(super) struct State {
    viewport: Option<Element>,
    tracks: [Option<Element>; 2],
    starts: [f64; 2],
    root_listeners: Vec<rustify_ui::Listener>,
    scroll: Option<rustify_ui::Listener>,
    observations: Vec<rustify_ui::ResizeObservation>,
    frame: Option<rustify_ui::FrameHandle>,
    hide: Option<HideTimer>,
    drag_listeners: Vec<rustify_ui::Listener>,
    drag: Option<Drag>,
}
impl Drop for State {
    fn drop(&mut self) {
        self.root_listeners.clear();
        self.scroll.take();
        self.observations.clear();
        self.drag_listeners.clear();
        self.drag.take();
        self.hide.take();
        if let Some(frame) = self.frame.take() {
            frame.cancel();
        }
    }
}
struct HideTimer {
    id: i32,
    _callback: Closure<dyn FnMut()>,
    _release: Option<rustify_ui::AbortRelease>,
}
impl HideTimer {
    fn new(context: Context, delay: i32) -> Option<Self> {
        let callback = Closure::<dyn FnMut()>::new(move || {
            context.activity.try_update(Activity::idle);
            context.browser.try_update_value(|state| {
                state.hide.take();
            });
        });
        let id = window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                delay,
            )
            .ok()?;
        let release = js_sys::Reflect::get(&window(), &"clearTimeout".into())
            .ok()
            .and_then(|method| method.dyn_into::<js_sys::Function>().ok())
            .map(|clear| {
                rustify_ui::release_on_abort(clear.bind1(&window(), &id.into()).unchecked_into())
            });
        Some(Self {
            id,
            _callback: callback,
            _release: release,
        })
    }
}
impl Drop for HideTimer {
    fn drop(&mut self) {
        window().clear_timeout_with_handle(self.id);
    }
}
struct Drag {
    axis: usize,
    pointer: i32,
    grab: f64,
    pending: Option<f64>,
    capture: Element,
    _release: Option<rustify_ui::AbortRelease>,
}
impl Drop for Drag {
    fn drop(&mut self) {
        if self.capture.has_pointer_capture(self.pointer) {
            let _ = self.capture.release_pointer_capture(self.pointer);
        }
    }
}

fn passive() -> rustify_ui::ListenOptions {
    rustify_ui::ListenOptions {
        capture: false,
        passive: true,
    }
}
fn scroll_value(element: &Element, axis: usize) -> f64 {
    let name = if axis == 0 { "scrollLeft" } else { "scrollTop" };
    let value = js_sys::Reflect::get(element.as_ref(), &name.into())
        .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
    // web-sys exposes integer scroll offsets unless its unstable API cfg is enabled.
    js_sys::Number::from(value).value_of()
}
fn set_scroll(element: &Element, axis: usize, value: f64) {
    let name = if axis == 0 { "scrollLeft" } else { "scrollTop" };
    js_sys::Reflect::set(element.as_ref(), &name.into(), &JsValue::from_f64(value))
        .unwrap_or_else(|error| leptos::wasm_bindgen::throw_val(error));
}

pub(super) fn install(context: Context) {
    Effect::new(move || {
        let Some(root) = context.root.get() else {
            return;
        };
        let root: Element = root.into();
        let enter = rustify_ui::listen(root.as_ref(), "pointerenter", passive(), move |_| {
            context.hold(Some(true), None)
        });
        let leave = rustify_ui::listen(root.as_ref(), "pointerleave", passive(), move |_| {
            context.hold(Some(false), None)
        });
        let focus = rustify_ui::listen(root.as_ref(), "focusin", passive(), move |_| {
            context.hold(None, Some(true))
        });
        let focus_root = root.clone();
        let blur = rustify_ui::listen(root.as_ref(), "focusout", passive(), move |event| {
            let inside = event
                .dyn_ref::<FocusEvent>()
                .and_then(FocusEvent::related_target)
                .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                .is_some_and(|target| focus_root.contains(Some(&target)));
            if !inside {
                context.hold(None, Some(false));
            }
        });
        let pointer = rustify_ui::listen(
            root.as_ref(),
            "pointerdown",
            rustify_ui::ListenOptions {
                capture: false,
                passive: false,
            },
            move |event| {
                if let Some(event) = event.dyn_ref::<PointerEvent>() {
                    context.pointer_down(event);
                }
            },
        );
        let mut listeners = vec![enter, leave, focus, blur, pointer];
        // A blocking ancestor wheel listener delays the viewport's native scrolling.
        for track in context.tracks.into_iter().filter_map(|track| track.get()) {
            listeners.push(rustify_ui::listen(
                track.as_ref(),
                "wheel",
                rustify_ui::ListenOptions {
                    capture: false,
                    passive: false,
                },
                move |event| {
                    if let Some(event) = event.dyn_ref::<WheelEvent>() {
                        context.bar_wheel(event);
                    }
                },
            ));
        }
        context.browser.update_value(|state| {
            state.root_listeners = listeners;
        });
    });
    Effect::new(move || {
        let viewport = context.viewport.get().filter(|node| node.is_connected());
        let content = context.content.get().filter(|node| node.is_connected());
        let tracks = context.tracks.map(|node| {
            node.get()
                .filter(|node| node.is_connected())
                .map(Into::<Element>::into)
        });
        let valid = context.viewports.get() == 1
            && context.track_counts.get().iter().all(|count| *count <= 1);
        context.end_drag();
        context.browser.update_value(|state| {
            state.scroll.take();
            state.observations.clear();
            state.viewport = None;
            state.tracks = tracks.clone();
        });
        let nodes = viewport.zip(content).filter(|_| valid);
        let Some((viewport, content)) = nodes else {
            context.metrics.set([AxisMetrics::default(); 2]);
            return;
        };
        let viewport: Element = viewport.into();
        let content: Element = content.into();
        let scroll = rustify_ui::listen(viewport.as_ref(), "scroll", passive(), move |_| {
            context
                .activity
                .try_update(|activity| activity.scrolling = true);
            context.restart_hide();
            context.schedule();
        });
        let observations = [Some(viewport.clone()), Some(content)]
            .into_iter()
            .chain(tracks)
            .flatten()
            .filter_map(|element| rustify_ui::observe_resize(&element, move || context.schedule()))
            .collect();
        context.browser.update_value(|state| {
            state.viewport = Some(viewport);
            state.scroll = Some(scroll);
            state.observations = observations;
        });
        context.schedule();
    });
    Effect::new(move || {
        let axes = context.scrollbars.get();
        context.visibility.track();
        context.delay.track();
        context.direction.track();
        if context.browser.with_value(|state| {
            state.drag.as_ref().is_some_and(|drag| {
                !model::enabled(
                    axes,
                    if drag.axis == 0 {
                        Orientation::Horizontal
                    } else {
                        Orientation::Vertical
                    },
                )
            })
        }) {
            context.end_drag();
        }
        context.cancel_hide();
        context.activity.update(Activity::idle);
        context.schedule();
    });
    on_cleanup(move || {
        context
            .browser
            .try_update_value(|state| *state = State::default());
    });
}

impl Context {
    pub(super) fn end_drag(self) {
        let drag = self
            .browser
            .try_update_value(|state| {
                state.drag_listeners.clear();
                state.drag.take()
            })
            .flatten();
        let ended = drag.is_some();
        drop(drag);
        if ended {
            self.activity.try_update(|activity| {
                activity.dragging = false;
                activity.lingering = true;
            });
            self.restart_hide();
        }
    }
    fn cancel_hide(self) {
        self.browser.try_update_value(|state| {
            state.hide.take();
        });
    }
    fn restart_hide(self) {
        self.cancel_hide();
        let Some(activity) = self.activity.try_get_untracked() else {
            return;
        };
        if activity.held()
            || !matches!(
                self.visibility.try_get_untracked(),
                Some(Visibility::Hover | Visibility::Scroll)
            )
        {
            return;
        }
        let Some(delay) = self.delay.try_get_untracked() else {
            return;
        };
        let timer = HideTimer::new(self, delay.ms);
        self.browser.try_update_value(|state| state.hide = timer);
    }
    fn hold(self, hovered: Option<bool>, focused: Option<bool>) {
        let visibility = self.visibility.get_untracked();
        self.activity.update(|activity| {
            let was_visible = activity.visible(visibility, true);
            if let Some(hovered) = hovered {
                activity.hovered = hovered;
            }
            if let Some(focused) = focused {
                activity.focused = focused;
            }
            activity.lingering = was_visible && !activity.held();
        });
        self.restart_hide();
    }
    fn schedule(self) {
        if self.browser.try_with_value(|state| state.frame.is_none()) != Some(true) {
            return;
        }
        let frame = rustify_ui::next_frame(move || {
            self.browser.try_update_value(|state| {
                state.frame.take();
            });
            self.measure();
        });
        self.browser
            .try_update_value(|state| state.frame = Some(frame));
    }
    fn measure(self) {
        let Some((viewport, tracks)) = self
            .browser
            .try_with_value(|state| {
                state
                    .viewport
                    .clone()
                    .map(|viewport| (viewport, state.tracks.clone()))
            })
            .flatten()
        else {
            return;
        };
        if !viewport.is_connected() {
            self.end_drag();
            return;
        }
        let direction = self.direction.get_untracked();
        let reverse = direction == Direction::RightToLeft;
        let sizes = [
            f64::from(viewport.client_width()),
            f64::from(viewport.client_height()),
        ];
        let contents = [
            f64::from(viewport.scroll_width()),
            f64::from(viewport.scroll_height()),
        ];
        let maxima = [
            (contents[0] - sizes[0]).max(0.),
            (contents[1] - sizes[1]).max(0.),
        ];
        let mut positions = [
            model::logical_left(scroll_value(&viewport, 0), maxima[0], direction),
            scroll_value(&viewport, 1).clamp(0., maxima[1]),
        ];
        let mut starts = [0.; 2];
        let mut lengths = [0.; 2];
        for (axis, track) in tracks.iter().enumerate() {
            if let Some(track) = track.as_ref().filter(|track| track.is_connected()) {
                let rect = track.get_bounding_client_rect();
                starts[axis] = if axis == 0 { rect.left() } else { rect.top() };
                lengths[axis] = if axis == 0 {
                    rect.width()
                } else {
                    rect.height()
                };
            }
        }
        let mut metrics = std::array::from_fn(|axis| {
            AxisMetrics::measure(
                sizes[axis],
                contents[axis],
                lengths[axis],
                positions[axis],
                axis == 0 && reverse,
            )
        });
        let pending = self
            .browser
            .try_update_value(|state| {
                state.starts = starts;
                state
                    .drag
                    .as_mut()
                    .map(|drag| (drag.axis, drag.grab, drag.pending.take()))
            })
            .flatten();
        if let Some((axis, grab, point)) = pending {
            if !metrics[axis].overflow || lengths[axis] <= 0. {
                self.end_drag();
            } else if let Some(point) = point {
                let asked = metrics[axis].scroll_from_pointer(
                    point - starts[axis],
                    grab,
                    axis == 0 && reverse,
                );
                set_scroll(
                    &viewport,
                    axis,
                    if axis == 0 {
                        model::native_left(asked, direction)
                    } else {
                        asked
                    },
                );
                positions[axis] = if axis == 0 {
                    model::logical_left(scroll_value(&viewport, 0), maxima[0], direction)
                } else {
                    scroll_value(&viewport, 1).clamp(0., maxima[1])
                };
                metrics[axis] = AxisMetrics::measure(
                    sizes[axis],
                    contents[axis],
                    lengths[axis],
                    positions[axis],
                    axis == 0 && reverse,
                );
            }
        }
        if self.metrics.get_untracked() != metrics {
            self.metrics.set(metrics);
        }
    }
    fn pointer_down(self, event: &PointerEvent) {
        if event.default_prevented()
            || event.button() != 0
            || !event.is_primary()
            || self.browser.with_value(|state| state.drag.is_some())
        {
            return;
        }
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let owner = target
            .closest("[data-rustify-part=scroll-area-root]")
            .ok()
            .flatten();
        if owner != self.root.get_untracked().map(Into::<Element>::into) {
            return;
        }
        let Some(track) = target
            .closest("[data-rustify-part=scroll-area-scrollbar]")
            .ok()
            .flatten()
        else {
            return;
        };
        let orientation = match track.get_attribute("data-orientation").as_deref() {
            Some("horizontal") => Orientation::Horizontal,
            Some("vertical") => Orientation::Vertical,
            _ => return,
        };
        if !model::enabled(self.scrollbars.get_untracked(), orientation) {
            return;
        }
        self.measure();
        let axis = index(orientation);
        let metrics = self.metrics.get_untracked()[axis];
        if !metrics.overflow || metrics.travel <= 0. {
            return;
        }
        let point = if axis == 0 {
            f64::from(event.client_x())
        } else {
            f64::from(event.client_y())
        };
        let start = self.browser.with_value(|state| state.starts[axis]);
        let on_thumb = target
            .closest("[data-rustify-part=scroll-area-thumb]")
            .ok()
            .flatten()
            .is_some();
        let grab = if on_thumb {
            (point - start - metrics.offset).clamp(0., metrics.thumb)
        } else {
            metrics.thumb / 2.
        };
        if track.set_pointer_capture(event.pointer_id()).is_err() {
            return;
        }
        let release = js_sys::Reflect::get(&track, &"releasePointerCapture".into())
            .ok()
            .and_then(|method| method.dyn_into::<js_sys::Function>().ok())
            .map(|release| {
                rustify_ui::release_on_abort(
                    release
                        .bind1(&track, &event.pointer_id().into())
                        .unchecked_into(),
                )
            });
        let listeners = vec![
            rustify_ui::listen(
                document().as_ref(),
                "pointermove",
                rustify_ui::ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| self.pointer_move(event),
            ),
            rustify_ui::listen(document().as_ref(), "pointerup", passive(), move |event| {
                self.pointer_end(event, true)
            }),
            rustify_ui::listen(
                document().as_ref(),
                "pointercancel",
                passive(),
                move |event| self.pointer_end(event, false),
            ),
            rustify_ui::listen(
                track.as_ref(),
                "lostpointercapture",
                passive(),
                move |event| self.pointer_end(event, false),
            ),
            rustify_ui::listen(window().as_ref(), "blur", passive(), move |_| {
                self.end_drag()
            }),
        ];
        self.browser.update_value(|state| {
            state.drag = Some(Drag {
                axis,
                pointer: event.pointer_id(),
                grab,
                pending: Some(point),
                capture: track,
                _release: release,
            });
            state.drag_listeners = listeners;
        });
        self.activity.update(|activity| {
            activity.dragging = true;
            activity.lingering = false;
        });
        self.cancel_hide();
        if let Some(viewport) = self.viewport.get_untracked() {
            let options = web_sys::FocusOptions::new();
            options.set_prevent_scroll(true);
            let _ = viewport.focus_with_options(&options);
        }
        event.prevent_default();
        self.schedule();
    }
    fn bar_wheel(self, event: &WheelEvent) {
        if event.default_prevented() || event.ctrl_key() {
            return;
        }
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let owner = target
            .closest("[data-rustify-part=scroll-area-root]")
            .ok()
            .flatten();
        if owner != self.root.get_untracked().map(Into::<Element>::into) {
            return;
        }
        let Some(track) = target
            .closest("[data-rustify-part=scroll-area-scrollbar]")
            .ok()
            .flatten()
        else {
            return;
        };
        let axis = if track.get_attribute("data-orientation").as_deref() == Some("horizontal") {
            0
        } else {
            1
        };
        let Some(viewport) = self.browser.with_value(|state| state.viewport.clone()) else {
            return;
        };
        let extent = if axis == 0 {
            viewport.client_width()
        } else {
            viewport.client_height()
        };
        let maximum = f64::from(
            if axis == 0 {
                viewport.scroll_width()
            } else {
                viewport.scroll_height()
            } - extent,
        )
        .max(0.);
        let delta = if axis == 0 {
            event.delta_x()
        } else {
            event.delta_y()
        };
        let delta = delta
            * match event.delta_mode() {
                1 => 16.,
                2 => f64::from(extent),
                _ => 1.,
            };
        let reverse = axis == 0 && self.direction.get_untracked() == Direction::RightToLeft;
        let current = if axis == 0 {
            model::logical_left(
                scroll_value(&viewport, axis),
                maximum,
                self.direction.get_untracked(),
            )
        } else {
            scroll_value(&viewport, axis).clamp(0., maximum)
        };
        let next = (current + if reverse { -delta } else { delta }).clamp(0., maximum);
        if current != next {
            set_scroll(
                &viewport,
                axis,
                if axis == 0 {
                    model::native_left(next, self.direction.get_untracked())
                } else {
                    next
                },
            );
            event.prevent_default();
            self.schedule();
        } else if self.boundary.get_untracked() == Boundary::Stop {
            event.prevent_default();
        }
    }
    fn pointer_move(self, event: Event) {
        let Some(event) = event.dyn_ref::<PointerEvent>() else {
            return;
        };
        let accepted = self
            .browser
            .try_update_value(|state| {
                let Some(drag) = state
                    .drag
                    .as_mut()
                    .filter(|drag| drag.pointer == event.pointer_id())
                else {
                    return false;
                };
                drag.pending = Some(if drag.axis == 0 {
                    f64::from(event.client_x())
                } else {
                    f64::from(event.client_y())
                });
                true
            })
            .unwrap_or(false);
        if accepted {
            event.prevent_default();
            self.schedule();
        }
    }
    fn pointer_end(self, event: Event, flush: bool) {
        let Some(event) = event.dyn_ref::<PointerEvent>() else {
            return;
        };
        if !self.browser.with_value(|state| {
            state
                .drag
                .as_ref()
                .is_some_and(|drag| drag.pointer == event.pointer_id())
        }) {
            return;
        }
        if flush {
            self.measure();
        }
        self.end_drag();
    }
}
