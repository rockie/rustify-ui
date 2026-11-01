use super::*;
use crate::listeners::{listen, ListenOptions, Listener};
use leptos::wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    CssRule, CssRuleList, CssStyleRule, CssStyleSheet, Element, FocusEvent, HtmlElement,
    KeyboardEvent, PointerEvent,
};

#[derive(Default)]
pub(super) struct Client {
    timer: Option<Timer>,
    listeners: Vec<Listener>,
    origin: Option<Element>,
    swipes: BTreeMap<Id, Gesture>,
    geometry: Option<Geometry>,
}

struct Timer {
    id: i32,
    _callback: Closure<dyn FnMut()>,
    _release: crate::AbortRelease,
}
impl Timer {
    fn new(runtime: Runtime, due: f64) -> Result<Self, JsValue> {
        let callback = Closure::<dyn FnMut()>::new(move || {
            if let Some(events) = runtime.model.try_update(|model| model.expire(now())) {
                runtime.dispatch(events);
            }
        });
        let id = window().set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            (due - now()).max(0.).ceil() as i32,
        )?;
        let clear = js_sys::Reflect::get(&window(), &"clearTimeout".into())?
            .dyn_into::<js_sys::Function>()?;
        let release = crate::release_on_abort(clear.bind1(&window(), &id.into()).unchecked_into());
        Ok(Self {
            id,
            _callback: callback,
            _release: release,
        })
    }
}
impl Drop for Timer {
    fn drop(&mut self) {
        window().clear_timeout_with_handle(self.id);
    }
}

struct Gesture {
    pointer: i32,
    x: f64,
    y: f64,
    direction: SwipeDirection,
    threshold: f64,
    buffer: f64,
    active: bool,
    element: Element,
    _release: crate::AbortRelease,
}
impl Drop for Gesture {
    fn drop(&mut self) {
        let _ = self.element.release_pointer_capture(self.pointer);
    }
}

struct Geometry {
    sheet: CssStyleSheet,
    rules: BTreeMap<Id, (CssRule, CssStyleRule)>,
    _release: crate::AbortRelease,
}
impl Geometry {
    fn new() -> Result<Self, JsValue> {
        let sheet = CssStyleSheet::new()?;
        let sheets = document().adopted_style_sheets();
        sheets.push(&sheet);
        document().set_adopted_style_sheets(&sheets);
        let clear =
            js_sys::Reflect::get(&sheet, &"replaceSync".into())?.dyn_into::<js_sys::Function>()?;
        let release = crate::release_on_abort(clear.bind1(&sheet, &"".into()).unchecked_into());
        Ok(Self {
            sheet,
            rules: BTreeMap::new(),
            _release: release,
        })
    }
    fn offset(&mut self, id: Id, dom_id: &str, x: f64, y: f64) -> Result<(), JsValue> {
        if !self.rules.contains_key(&id) {
            let index = self.sheet.css_rules()?.length();
            self.sheet.insert_rule_with_index(
                &format!("@layer components{{:where(.{dom_id}){{translate:0px 0px;}}}}"),
                index,
            )?;
            let rule = self
                .sheet
                .css_rules()?
                .item(index)
                .ok_or_else(|| JsValue::from_str("toast swipe rule was not inserted"))?;
            let children =
                js_sys::Reflect::get(&rule, &"cssRules".into())?.dyn_into::<CssRuleList>()?;
            let style = children
                .item(0)
                .ok_or_else(|| JsValue::from_str("toast swipe declarations are unavailable"))?
                .dyn_into::<CssStyleRule>()?;
            self.rules.insert(id, (rule, style));
        }
        if let Some((_, rule)) = self.rules.get(&id) {
            rule.style()
                .set_property("translate", &format!("{x}px {y}px"))?;
        }
        Ok(())
    }
    fn remove(&mut self, id: Id) {
        if let Some((removed, _)) = self.rules.remove(&id) {
            if let Ok(rules) = self.sheet.css_rules() {
                for index in 0..rules.length() {
                    if rules.item(index).as_ref() == Some(&removed) {
                        let _ = self.sheet.delete_rule(index);
                        break;
                    }
                }
            }
        }
    }
}
impl Drop for Geometry {
    fn drop(&mut self) {
        let sheets = document().adopted_style_sheets();
        let remaining = js_sys::Array::new();
        for index in 0..sheets.length() {
            let sheet = sheets.get(index);
            if sheet != JsValue::from(self.sheet.clone()) {
                remaining.push(&sheet);
            }
        }
        document().set_adopted_style_sheets(&remaining);
    }
}

pub(super) fn install(runtime: Runtime) {
    Effect::new(move || {
        let due = runtime.model.with(Scheduler::next_due);
        runtime.client.update_value(|client| {
            client.timer.take();
        });
        if let Some(due) = due {
            match Timer::new(runtime, due) {
                Ok(timer) => {
                    runtime
                        .client
                        .update_value(|client| client.timer = Some(timer));
                    runtime.issue("timer".into(), None);
                }
                Err(error) => runtime.issue(
                    "timer".into(),
                    Some(ToastIssue {
                        field: "duration_ms",
                        message: format!("browser timer could not be scheduled: {error:?}"),
                    }),
                ),
            }
        }
    });
    let blur = listen(&window(), "blur", ListenOptions::default(), move |_| {
        runtime.pause(None, Pause::WindowBlur, true)
    });
    let focus = listen(&window(), "focus", ListenOptions::default(), move |_| {
        runtime.pause(None, Pause::WindowBlur, false)
    });
    let visibility = listen(
        &document(),
        "visibilitychange",
        ListenOptions::default(),
        move |_| runtime.pause(None, Pause::Hidden, document().hidden()),
    );
    runtime
        .client
        .update_value(|client| client.listeners = vec![blur, focus, visibility]);
    runtime.pause(None, Pause::Hidden, document().hidden());
    runtime.pause(
        None,
        Pause::WindowBlur,
        !document().has_focus().unwrap_or(true),
    );
    on_cleanup(move || {
        runtime.client.try_update_value(|client| {
            client.timer.take();
            client.listeners.clear();
            client.swipes.clear();
            client.geometry.take();
            client.origin.take();
        });
    });
}

fn reachable(element: &Element) -> bool {
    element.is_connected()
        && element
            .dyn_ref::<HtmlElement>()
            .is_some_and(crate::overlay::notification_reachable)
}

fn valid_origin(element: &Element) -> bool {
    reachable(element)
        && !document()
            .body()
            .is_some_and(|body| AsRef::<Element>::as_ref(&body) == element)
}

fn hook(callback: Option<Callback<Cancelable<ToastSwipe>>>, value: ToastSwipe) -> bool {
    let event = Cancelable::new(value);
    if let Some(callback) = callback {
        callback.run(event.clone());
    }
    !event.default_prevented()
}

impl Runtime {
    pub(super) fn holds_focus(self, id: Id) -> bool {
        self.registration(id)
            .and_then(|root| root.node.get_untracked())
            .is_some_and(|node| {
                document()
                    .active_element()
                    .is_some_and(|active| node.contains(Some(&active)))
            })
    }
    pub(super) fn focus_after(self, excluded: Id) {
        if document()
            .active_element()
            .filter(reachable)
            .is_some_and(|active| {
                !document()
                    .body()
                    .is_some_and(|body| AsRef::<Element>::as_ref(&body) == &active)
                    && !self.holds_focus(excluded)
            })
        {
            return;
        }
        let ids = self
            .model
            .try_with_untracked(Scheduler::visible_ids)
            .unwrap_or_default();
        for id in ids.into_iter().filter(|id| *id != excluded) {
            if let Some(node) = self
                .registration(id)
                .and_then(|root| root.node.get_untracked())
            {
                let element: Element = node.into();
                let _ = element.remove_attribute("hidden");
                let _ = element.remove_attribute("inert");
                if reachable(&element) {
                    crate::overlay::focus_first(&element);
                    return;
                }
            }
        }
        if let Some(origin) = self
            .client
            .try_with_value(|client| client.origin.clone())
            .flatten()
            .filter(valid_origin)
        {
            if let Some(origin) = origin.dyn_ref::<HtmlElement>() {
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                let _ = origin.focus_with_options(&options);
            }
            return;
        }
        if let Some(stack) = self.overlay.try_get_value().flatten() {
            crate::overlay::focus_first(&stack.notification_fallback());
        }
    }
    pub(super) fn focus_enter(self, event: &FocusEvent) {
        if let Some(origin) = event
            .related_target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .filter(valid_origin)
        {
            let within_roots = self.registrations.with_untracked(|roots| {
                roots.iter().any(|root| {
                    root.node
                        .get_untracked()
                        .is_some_and(|node| node.contains(Some(&origin)))
                })
            });
            let within_notifications = within_roots || self.presence(Some(&origin)).1;
            if !within_notifications {
                self.client
                    .update_value(|client| client.origin = Some(origin));
            }
        }
        self.reconcile_presence();
    }
    fn presence(self, active: Option<&Element>) -> (bool, bool) {
        let Some(viewport_owner) = self.viewport.try_get_untracked().flatten() else {
            return (false, false);
        };
        if let Some(stack) = self.overlay.try_get_value().flatten() {
            stack.notification_presence(viewport_owner, active)
        } else {
            let Some(viewport) = viewport_owner
                .try_get_value()
                .and_then(|node| node.get_untracked())
            else {
                return (false, false);
            };
            let viewport: Element = viewport.into();
            (
                viewport.matches(":hover").unwrap_or(false),
                active.is_some_and(|active| viewport.contains(Some(active))),
            )
        }
    }
    pub(super) fn reconcile_presence(self) {
        let (hovered, focused) = self.presence(document().active_element().as_ref());
        self.pause(None, Pause::Hover, hovered);
        self.pause(None, Pause::Focus, focused);
    }
    pub(super) fn reconcile_focus(self, next: Option<Element>) {
        let focused = self.presence(next.as_ref()).1;
        self.pause(None, Pause::Focus, focused);
    }
    pub(super) fn key(self, event: &KeyboardEvent) -> bool {
        let ids = self
            .model
            .try_with_untracked(Scheduler::visible_ids)
            .unwrap_or_default();
        if event.key() == "Escape" {
            if let Some(root) = ids.iter().find_map(|id| {
                self.holds_focus(*id)
                    .then(|| self.registration(*id))
                    .flatten()
            }) {
                let escape = Cancelable::new(LayerDismiss::Escape);
                if let Some(callback) = root.escape {
                    callback.run(escape.clone());
                }
                if !escape.default_prevented() {
                    self.request_close(root.id);
                }
                return true;
            }
            return false;
        }
        let Some(hotkey) = self.model.try_with_untracked(|model| model.config.hotkey) else {
            return false;
        };
        let matches = (event.key() == hotkey.key
            || !hotkey.code.is_empty() && event.code() == hotkey.code)
            && event.ctrl_key() == hotkey.modifiers.ctrl
            && event.alt_key() == hotkey.modifiers.alt
            && event.shift_key() == hotkey.modifiers.shift
            && event.meta_key() == hotkey.modifiers.meta;
        if !matches || ids.is_empty() {
            return false;
        }
        let text_key = event.key().chars().count() == 1
            && !event.ctrl_key()
            && !event.alt_key()
            && !event.meta_key();
        let editing = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .is_some_and(|target| {
                target
                    .closest(
                        "input, textarea, select, [contenteditable]:not([contenteditable='false'])",
                    )
                    .ok()
                    .flatten()
                    .is_some()
            });
        if text_key && editing {
            return false;
        }
        for id in ids {
            if let Some(node) = self
                .registration(id)
                .and_then(|root| root.node.get_untracked())
            {
                let element: Element = node.into();
                if !reachable(&element) {
                    continue;
                }
                let origin = document()
                    .active_element()
                    .filter(valid_origin)
                    .filter(|active| {
                        !self.presence(Some(active)).1
                            && !self.registrations.with_untracked(|roots| {
                                roots.iter().any(|root| {
                                    root.node
                                        .get_untracked()
                                        .is_some_and(|node| node.contains(Some(active)))
                                })
                            })
                    });
                self.client.update_value(|client| {
                    if let Some(origin) = origin {
                        client.origin = Some(origin);
                    }
                });
                crate::overlay::focus_first(&element);
                return true;
            }
        }
        false
    }
    pub(super) fn flush_announcements(self) {
        let pending = self
            .pending_announcements
            .try_get_untracked()
            .unwrap_or_default();
        let mut remaining = Vec::new();
        let mut polite = Vec::new();
        let mut assertive = Vec::new();
        for id in pending {
            let Some(root) = self.registration(id) else {
                continue;
            };
            let Some(node) = root.node.get_untracked() else {
                remaining.push(id);
                continue;
            };
            if !self.model.with_untracked(|model| model.visible(id)) {
                continue;
            }
            let mut text = Vec::new();
            if let Ok(parts) = node.query_selector_all("[data-rustify-toast-title], [data-rustify-toast-description], [data-rustify-toast-alt]") {
                for index in 0..parts.length() {
                    if let Some(part) = parts.item(index).and_then(|part| part.dyn_into::<Element>().ok()) {
                        let value = part.get_attribute("data-rustify-toast-alt").or_else(|| part.text_content()).unwrap_or_default();
                        if !value.trim().is_empty() { text.push(value); }
                    }
                }
            }
            if text.is_empty() {
                remaining.push(id);
            }
            if !text.is_empty() {
                let seq = {
                    let mut seq = self.announcement_seq.write();
                    *seq += 1;
                    *seq
                };
                let message = (seq, text.join(" "));
                match root.importance.get_untracked() {
                    ToastImportance::Background => polite.push(message),
                    ToastImportance::Foreground => assertive.push(message),
                }
            }
        }
        self.pending_announcements.try_set(remaining);
        if !polite.is_empty() {
            self.polite.try_set(polite);
        }
        if !assertive.is_empty() {
            self.assertive.try_set(assertive);
        }
    }
    fn offset(self, id: Id, x: f64, y: f64) {
        let Some(root) = self.registration(id) else {
            return;
        };
        let Some(dom_id) = root.identity.try_get_value() else {
            return;
        };
        let outcome = self
            .client
            .try_update_value(|client| -> Result<(), JsValue> {
                if client.geometry.is_none() {
                    client.geometry = Some(Geometry::new()?);
                }
                if let Some(geometry) = &mut client.geometry {
                    geometry.offset(id, &dom_id, x, y)?;
                }
                Ok(())
            });
        match outcome {
            Some(Ok(())) => self.issue(format!("swipe-{id}"), None),
            Some(Err(error)) => self.issue(
                format!("swipe-{id}"),
                Some(ToastIssue {
                    field: "swipe",
                    message: format!("browser swipe geometry failed: {error:?}"),
                }),
            ),
            None => {}
        }
    }
    pub(super) fn remove_geometry(self, id: Id) {
        self.client.try_update_value(|client| {
            if let Some(geometry) = &mut client.geometry {
                geometry.remove(id);
            }
        });
        self.issue(format!("swipe-{id}"), None);
    }
    pub(super) fn cancel_swipe(self, id: Id) {
        let gesture = self
            .client
            .try_update_value(|client| client.swipes.remove(&id))
            .flatten();
        if let Some(gesture) = gesture {
            let active = gesture.active;
            drop(gesture);
            self.offset(id, 0., 0.);
            if active {
                self.pause(Some(id), Pause::Swipe, false);
            }
        }
    }
    pub(super) fn swipe_down(self, id: Id, event: &PointerEvent) {
        if event.default_prevented() || event.button() != 0 || !event.is_primary() {
            return;
        }
        let interactive = event.target().and_then(|target| target.dyn_into::<Element>().ok()).is_some_and(|target| target.closest("button, a[href], input, select, textarea, [contenteditable]:not([contenteditable='false'])").ok().flatten().is_some());
        if interactive {
            return;
        }
        let Some(root) = self.registration(id) else {
            return;
        };
        let Some(node) = root.node.get_untracked() else {
            return;
        };
        let element: Element = node.into();
        let pointer = event.pointer_id();
        let captured = (|| -> Result<crate::AbortRelease, JsValue> {
            element.set_pointer_capture(pointer)?;
            let release = js_sys::Reflect::get(&element, &"releasePointerCapture".into())?
                .dyn_into::<js_sys::Function>()?;
            Ok(crate::release_on_abort(
                release.bind1(&element, &pointer.into()).unchecked_into(),
            ))
        })();
        let release = match captured {
            Ok(release) => release,
            Err(error) => {
                self.issue(
                    format!("swipe-{id}"),
                    Some(ToastIssue {
                        field: "swipe",
                        message: format!("pointer capture failed: {error:?}"),
                    }),
                );
                return;
            }
        };
        let config = self.model.with_untracked(|model| model.config.clone());
        let gesture = Gesture {
            pointer,
            x: f64::from(event.client_x()),
            y: f64::from(event.client_y()),
            direction: config.swipe_direction,
            threshold: config.swipe_threshold,
            buffer: if event.pointer_type() == "touch" {
                10.
            } else {
                2.
            },
            active: false,
            element,
            _release: release,
        };
        self.client.update_value(|client| {
            client.swipes.insert(id, gesture);
        });
    }
    pub(super) fn swipe_move(self, id: Id, event: &PointerEvent) {
        let gesture = self
            .client
            .try_update_value(|client| client.swipes.remove(&id))
            .flatten();
        let Some(mut gesture) = gesture else {
            return;
        };
        if gesture.pointer != event.pointer_id() {
            self.client.update_value(|client| {
                client.swipes.insert(id, gesture);
            });
            return;
        }
        let x = f64::from(event.client_x()) - gesture.x;
        let y = f64::from(event.client_y()) - gesture.y;
        let Some(root) = self.registration(id) else {
            return;
        };
        let (delta_x, delta_y) = gesture.direction.offset(x, y);
        let value = ToastSwipe {
            direction: gesture.direction,
            delta_x,
            delta_y,
        };
        let starting = !gesture.active && gesture.direction.distance(x, y) > gesture.buffer;
        if starting {
            if !hook(root.swipe_start, value) {
                return;
            }
            gesture.active = true;
        }
        let moving = gesture.active && hook(root.swipe_move, value);
        if self.registration(id).is_none() {
            return;
        }
        self.client.update_value(|client| {
            client.swipes.insert(id, gesture);
        });
        if starting {
            self.pause(Some(id), Pause::Swipe, true);
        }
        if moving {
            event.prevent_default();
            self.offset(id, delta_x, delta_y);
        }
    }
    pub(super) fn swipe_up(self, id: Id, event: &PointerEvent, cancelled: bool) {
        let gesture = self
            .client
            .try_update_value(|client| client.swipes.remove(&id))
            .flatten();
        let Some(gesture) = gesture else {
            return;
        };
        if gesture.pointer != event.pointer_id() {
            self.client.update_value(|client| {
                client.swipes.insert(id, gesture);
            });
            return;
        }
        let Some(root) = self.registration(id) else {
            return;
        };
        let x = f64::from(event.client_x()) - gesture.x;
        let y = f64::from(event.client_y()) - gesture.y;
        let (delta_x, delta_y) = gesture.direction.offset(x, y);
        let value = ToastSwipe {
            direction: gesture.direction,
            delta_x,
            delta_y,
        };
        let ended =
            gesture.active && !cancelled && gesture.direction.distance(x, y) > gesture.threshold;
        let accepted = if gesture.active {
            hook(
                if ended {
                    root.swipe_end
                } else {
                    root.swipe_cancel
                },
                value,
            )
        } else {
            false
        };
        let active = gesture.active;
        drop(gesture);
        self.offset(id, 0., 0.);
        if active {
            self.pause(Some(id), Pause::Swipe, false);
        }
        if ended && accepted {
            self.request_close(id);
        }
    }
}
