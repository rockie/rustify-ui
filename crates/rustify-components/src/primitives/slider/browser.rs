use super::model::Input;
use super::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::{Element, Event, FocusEvent, KeyboardEvent, PointerEvent};

#[derive(Default)]
pub(super) struct BrowserState {
    track: Option<Element>,
    rect: Option<[f64; 4]>,
    capture: Option<(Element, i32, Option<rustify_ui::AbortRelease>)>,
    scroll: Option<rustify_ui::Listener>,
    viewport: Option<rustify_ui::Listener>,
    resize: Option<rustify_ui::ResizeObservation>,
}
impl SliderContext {
    fn owns(self, event: &Event) -> bool {
        let Some(id) = self.id.try_get_value() else {
            return false;
        };
        event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .and_then(|target| {
                target
                    .closest("[data-rustify-part=slider-root]")
                    .ok()
                    .flatten()
            })
            .is_some_and(|root| root.id() == id)
    }
    fn event_entry(self, event: &Event) -> Option<Entry> {
        if !self.owns(event) {
            return None;
        }
        let target = event.target()?.dyn_into::<Element>().ok()?;
        let target = target
            .closest("[data-rustify-part=slider-thumb]")
            .ok()
            .flatten()?;
        self.registry.with_untracked(|registry| {
            registry
                .by_dom
                .get(&target.id())
                .map(|&at| registry.entries[at])
        })
    }
    fn can_start(self, key: &str) -> bool {
        !untrack(move || self.unavailable() || self.read_only.get())
            && self
                .registry
                .with_untracked(|registry| registry.enabled.contains(key))
            && self.accepted.with_untracked(|state| {
                state
                    .model
                    .as_ref()
                    .is_some_and(|model| model.get(key).is_some())
            })
    }
    fn begin(self, input: Input, key: String) {
        let session = self.accepted.with_untracked(|state| {
            state
                .model
                .as_ref()
                .map(|model| Session::new(input, key, model))
        });
        self.session.set_value(session);
        self.session_revision.update(|revision| *revision += 1);
    }
    pub(super) fn end(self, commit: bool) {
        // Take the session before releasing capture: lostpointercapture/blur cannot finish twice.
        let session = self.session.try_update_value(Option::take).flatten();
        self.browser.try_update_value(|browser| {
            let capture = browser.capture.take();
            browser.scroll.take();
            browser.viewport.take();
            browser.resize.take();
            browser.track = None;
            browser.rect = None;
            if let Some((element, pointer, _abort)) = capture {
                if element.has_pointer_capture(pointer) {
                    let _ = element.release_pointer_capture(pointer);
                }
            }
        });
        if let Some(session) = session {
            self.session_revision.try_update(|revision| *revision += 1);
            if commit {
                if let (Some(callback), Some(values)) = (self.commit, session.finish()) {
                    callback.run(values);
                }
            } else if let Some(callback) = self.cancel {
                callback.run(());
            }
        }
    }
    fn request(self, asked: f64) {
        let next = self.session.with_value(|session| {
            let session = session.as_ref()?;
            if !self.can_start(&session.active) {
                return None;
            }
            self.accepted.with_untracked(|state| {
                state
                    .model
                    .as_ref()?
                    .candidate(&session.active, asked, session.neighbours.as_ref())
            })
        });
        let Some(next) = next else {
            return;
        };
        let duplicate = self.session.with_value(|session| {
            session.as_ref().is_none_or(|session| {
                session.last.as_ref().map_or_else(
                    || {
                        self.accepted.with_untracked(|state| {
                            state
                                .model
                                .as_ref()
                                .is_some_and(|model| model.values == next)
                        })
                    },
                    |last| *last == next,
                )
            })
        });
        if duplicate {
            return;
        }
        self.session.update_value(|session| {
            if let Some(session) = session {
                session.request(next.clone());
            }
        });
        self.change.run(next);
    }
    fn pointer_value(self, event: &PointerEvent) -> Option<f64> {
        let rect = self.browser.with_value(|browser| browser.rect);
        let rect = if let Some(rect) = rect {
            rect
        } else {
            let rect = self.browser.with_value(|browser| {
                browser.track.as_ref().map(|track| {
                    let rect = track.get_bounding_client_rect();
                    [rect.left(), rect.top(), rect.width(), rect.height()]
                })
            })?;
            self.browser
                .update_value(|browser| browser.rect = Some(rect));
            rect
        };
        let fraction = if self.orientation == Orientation::Horizontal {
            if rect[2] <= 0.0 {
                return None;
            }
            (f64::from(event.client_x()) - rect[0]) / rect[2]
        } else {
            if rect[3] <= 0.0 {
                return None;
            }
            1.0 - (f64::from(event.client_y()) - rect[1]) / rect[3]
        };
        let fraction = if untrack(move || self.reverse()) {
            1.0 - fraction
        } else {
            fraction
        };
        self.accepted.with_untracked(|state| {
            state.model.as_ref().map(|model| {
                model.config.min + fraction.clamp(0.0, 1.0) * (model.config.max - model.config.min)
            })
        })
    }
    pub(super) fn pointer_down(self, event: PointerEvent) {
        if event.default_prevented()
            || !self.owns(&event)
            || event.button() != 0
            || !event.is_primary()
            || self.session.with_value(Option::is_some)
        {
            return;
        }
        let Some(root) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id(&self.id.get_value()))
        else {
            return;
        };
        let Some(track) = root
            .query_selector("[data-rustify-part=slider-track]")
            .ok()
            .flatten()
        else {
            return;
        };
        self.browser.update_value(|browser| {
            browser.track = Some(track.clone());
            browser.rect = None;
        });
        let Some(value) = self.pointer_value(&event) else {
            return;
        };
        let entry = self.event_entry(&event);
        let key = if let Some(entry) = entry {
            Some(entry.key.get_value())
        } else {
            let focused = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.active_element())
                .map(|element| element.id());
            self.registry.with_untracked(|registry| {
                let focused = focused
                    .as_ref()
                    .and_then(|id| registry.by_dom.get(id))
                    .map(|&at| registry.entries[at].key.get_value());
                let enabled: Vec<_> = registry.enabled.iter().map(String::as_str).collect();
                self.accepted.with_untracked(|state| {
                    state.model.as_ref().and_then(|model| {
                        model
                            .nearest(value, &enabled, focused.as_deref())
                            .map(str::to_owned)
                    })
                })
            })
        };
        let Some(key) = key.filter(|key| self.can_start(key)) else {
            return;
        };
        let element = self
            .registry
            .with_untracked(|registry| {
                registry
                    .entries
                    .iter()
                    .find(|entry| entry.key.read_value().as_str() == key)
                    .map(|entry| entry.dom_id.get_value())
            })
            .and_then(|id| web_sys::window()?.document()?.get_element_by_id(&id));
        let Some(element) = element else {
            return;
        };
        if element.set_pointer_capture(event.pointer_id()).is_err() {
            return;
        }
        let _ = element
            .dyn_ref::<web_sys::HtmlElement>()
            .map(|element| element.focus());
        let abort = js_sys::Reflect::get(&element, &"releasePointerCapture".into())
            .ok()
            .and_then(|method| method.dyn_into::<js_sys::Function>().ok())
            .map(|release| {
                rustify_ui::release_on_abort(
                    release
                        .bind1(&element, &event.pointer_id().into())
                        .unchecked_into::<js_sys::Function>(),
                )
            });
        self.browser.update_value(|browser| {
            browser.capture = Some((element, event.pointer_id(), abort));
            browser.resize = rustify_ui::observe_resize(&track, move || {
                self.browser.try_update_value(|browser| browser.rect = None);
            });
            browser.viewport = web_sys::window().map(|window| {
                rustify_ui::listen(
                    window.as_ref(),
                    "resize",
                    rustify_ui::ListenOptions {
                        capture: false,
                        passive: true,
                    },
                    move |_| {
                        self.browser.try_update_value(|browser| browser.rect = None);
                    },
                )
            });
            browser.scroll = web_sys::window().map(|window| {
                rustify_ui::listen(
                    window.as_ref(),
                    "scroll",
                    rustify_ui::ListenOptions {
                        capture: true,
                        passive: true,
                    },
                    move |_| {
                        self.browser.try_update_value(|browser| browser.rect = None);
                    },
                )
            });
        });
        self.begin(Input::Pointer(event.pointer_id()), key);
        event.prevent_default();
        self.request(value);
    }
    pub(super) fn pointer_move(self, event: PointerEvent) {
        let active = self
            .session
            .try_with_value(|session| {
                session
                    .as_ref()
                    .is_some_and(|session| session.input == Input::Pointer(event.pointer_id()))
            })
            .unwrap_or(false);
        if active && !event.default_prevented() {
            if let Some(value) = self.pointer_value(&event) {
                self.request(value);
            }
        }
    }
    pub(super) fn pointer_up(self, event: PointerEvent) {
        if self
            .session
            .try_with_value(|session| {
                session
                    .as_ref()
                    .is_some_and(|session| session.input == Input::Pointer(event.pointer_id()))
            })
            .unwrap_or(false)
        {
            self.end(true);
        }
    }
    pub(super) fn pointer_cancel(self, event: PointerEvent) {
        if self
            .session
            .try_with_value(|session| {
                session
                    .as_ref()
                    .is_some_and(|session| session.input == Input::Pointer(event.pointer_id()))
            })
            .unwrap_or(false)
        {
            self.end(false);
        }
    }
    pub(super) fn key_down(self, event: KeyboardEvent) {
        if event.default_prevented() || !self.owns(&event) {
            return;
        }
        if event.key() == "Escape" {
            if self.session.with_value(Option::is_some) {
                event.prevent_default();
                self.end(false);
            }
            return;
        }
        let Some(entry) = self.event_entry(&event) else {
            return;
        };
        let key = entry.key.get_value();
        if !self.can_start(&key) {
            return;
        }
        let Some(command) = model::key(
            &event.key(),
            self.orientation,
            untrack(move || self.reverse()),
            self.inverted,
        ) else {
            return;
        };
        if self.session.with_value(|session| {
            session
                .as_ref()
                .is_some_and(|session| session.input != Input::Keyboard || session.active != key)
        }) {
            return;
        }
        event.prevent_default();
        if self.session.with_value(Option::is_none) {
            self.begin(Input::Keyboard, key.clone());
        }
        let asked = self.accepted.with_untracked(|state| {
            let model = state.model.as_ref()?;
            let held = self.session.with_value(|session| {
                session
                    .as_ref()
                    .and_then(|session| session.last.as_ref())
                    .and_then(|last| last.iter().find(|item| item.id == key))
                    .map(|item| item.value)
            });
            Some(match command {
                model::Key::Home => self.bounds(&key).0,
                model::Key::End => self.bounds(&key).1,
                model::Key::Step(delta) => {
                    held.or_else(|| model.get(&key))? + f64::from(delta) * model.config.step
                }
            })
        });
        if let Some(asked) = asked {
            self.request(asked);
        }
    }
    pub(super) fn key_up(self, event: KeyboardEvent) {
        if !self.owns(&event)
            || !matches!(
                event.key().as_str(),
                "ArrowLeft"
                    | "ArrowRight"
                    | "ArrowUp"
                    | "ArrowDown"
                    | "PageUp"
                    | "PageDown"
                    | "Home"
                    | "End"
            )
        {
            return;
        }
        if self.session.with_value(|session| {
            session
                .as_ref()
                .is_some_and(|session| session.input == Input::Keyboard)
        }) {
            self.end(true);
        }
    }
    pub(super) fn focus_out(self, event: FocusEvent) {
        let Some(entry) = self.event_entry(&event) else {
            return;
        };
        if self.session.with_value(|session| {
            session.as_ref().is_some_and(|session| {
                session.input == Input::Keyboard && session.active == entry.key.get_value()
            })
        }) {
            if event.related_target().is_none() {
                let dom_id = entry.dom_id.get_value();
                let key = entry.key.get_value();
                rustify_ui::defer(move || {
                    if self.session.try_with_value(Option::is_some) != Some(true) {
                        return;
                    }
                    let connected = web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| document.get_element_by_id(&dom_id))
                        .is_some_and(|element| element.is_connected());
                    self.end(connected && self.can_start(&key));
                });
            } else {
                self.end(true);
            }
        }
    }
}
