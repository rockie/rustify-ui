use super::*;
use leptos::wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Element, Event, KeyboardEvent, MouseEvent, PointerEvent};

pub(super) struct SearchTimer {
    id: i32,
    _callback: Closure<dyn FnMut()>,
    _release: Option<rustify_ui::AbortRelease>,
}
impl SearchTimer {
    fn new(context: Context) -> Option<Self> {
        let callback = Closure::<dyn FnMut()>::new(move || {
            context.search.try_update_value(String::clear);
            context.initial.try_update_value(|initial| *initial = None);
        });
        let id = window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                1000,
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
impl Drop for SearchTimer {
    fn drop(&mut self) {
        window().clear_timeout_with_handle(self.id);
    }
}

impl Context {
    pub(super) fn focus(self) {
        if let Some(trigger) = self
            .trigger
            .try_get_untracked()
            .flatten()
            .filter(|trigger| trigger.is_connected())
        {
            let _ = trigger.focus();
        }
    }
    pub(super) fn settle(self) {
        if let Some(select) = self.mirror.try_get_untracked().flatten() {
            let accepted = self.accepted().unwrap_or_default();
            if select.value() != accepted {
                select.set_value(&accepted);
            }
            select.set_disabled(untrack(move || self.disabled()));
        }
    }
    fn reset_search(self) {
        self.search.try_update_value(String::clear);
        self.timer.try_update_value(|timer| {
            timer.take();
        });
    }
    pub(super) fn close_from_layer(self) {
        if self.key.try_get_value().is_none() {
            return;
        }
        let connected = self.triggers.try_get_untracked() == Some(1)
            && self
                .trigger
                .try_get_untracked()
                .flatten()
                .is_some_and(|trigger| trigger.is_connected());
        if !connected {
            if self.anchor_failed.get_value() {
                return;
            }
            self.anchor_failed.set_value(true);
            self.reset_search();
        }
        self.change_open.try_run(false);
    }
    fn request_open(self, open: bool, initial: Initial) {
        if self.disabled.get_untracked() {
            return;
        }
        if open {
            self.initial.set_value(Some(initial));
        }
        self.change_open.try_run(open);
    }
    fn trigger_target(self, event: &Event) -> bool {
        let Some(key) = self.key.try_get_value() else {
            return false;
        };
        event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .and_then(|target| {
                target
                    .closest("[data-rustify-part=select-trigger]")
                    .ok()
                    .flatten()
            })
            .is_some_and(|target| {
                target.get_attribute("data-rustify-select-owner").as_deref() == Some(key.as_str())
            })
    }
    fn target(self, event: &Event) -> Option<Element> {
        let key = self.key.try_get_value()?;
        let target = event
            .target()?
            .dyn_into::<Element>()
            .ok()?
            .closest("[data-rustify-select-owner]")
            .ok()??;
        (target.get_attribute("data-rustify-select-owner").as_deref() == Some(key.as_str()))
            .then_some(target)
    }
    fn event_item(self, event: &Event) -> Option<ItemBinding> {
        let target = self.target(event)?;
        if target.get_attribute("data-rustify-part").as_deref() != Some("select-item") {
            return None;
        }
        self.registry
            .with_untracked(|registry| registry.by_id.get(&target.id()).copied())
            .filter(|item| !untrack(move || self.item_disabled(*item)))
    }
    pub(super) fn trigger_click(self, event: MouseEvent) {
        if !event.default_prevented()
            && self.trigger_target(&event)
            && !untrack(move || self.disabled())
        {
            self.request_open(!self.open.get_untracked(), Initial::Edge(false));
        }
    }
    fn choose(self, value: String) {
        if self.key.try_get_value().is_none()
            || untrack(move || self.disabled())
            || !untrack(|| self.usable(&value))
        {
            return;
        }
        if self.read_only.get_untracked() {
            self.request_open(false, Initial::Edge(false));
            return;
        }
        let event = Cancelable::new(value.clone());
        if let Some(select) = self.select {
            select.try_run(event.clone());
        }
        if event.default_prevented() || self.key.try_get_value().is_none() {
            return;
        }
        if untrack(move || self.disabled() || self.read_only.get())
            || !untrack(|| self.usable(&value))
        {
            return;
        }
        self.change.try_run(Some(value));
        if self.key.try_get_value().is_some() {
            self.change_open.try_run(false);
        }
    }
    fn search_key(self, key: &str) {
        self.search.update_value(|query| query.push_str(key));
        self.timer.set_value(SearchTimer::new(self));
        let current = if self.open.get_untracked() {
            self.active.get_untracked()
        } else {
            self.accepted()
        };
        let next = self.search.with_value(|query| {
            self.index.with_untracked(|index| {
                index.search(query, current.as_deref(), |value| {
                    untrack(|| self.usable(value))
                })
            })
        });
        if self.open.get_untracked() {
            if let Some(next) = next {
                self.active.set(Some(next));
            }
        } else {
            let initial = next.map_or(Initial::Edge(false), Initial::Match);
            self.request_open(true, initial);
        }
    }
    pub(super) fn key(self, event: KeyboardEvent) {
        if event.default_prevented()
            || event.is_composing()
            || !self.trigger_target(&event)
            || untrack(move || self.disabled())
        {
            return;
        }
        let key = event.key();
        let open = self.open.get_untracked();
        if key == "Tab" {
            if open {
                self.request_open(false, Initial::Edge(false));
            }
            return;
        }
        if key == "Escape" {
            if open {
                self.request_open(false, Initial::Edge(false));
                event.prevent_default();
            }
            return;
        }
        if event.ctrl_key() || event.alt_key() || event.meta_key() {
            return;
        }
        if key == " " && !self.search.read_value().is_empty() {
            event.prevent_default();
            self.search_key(&key);
            return;
        }
        match key.as_str() {
            "Enter" | " " => {
                event.prevent_default();
                if open {
                    if let Some(value) = self.active.get_untracked() {
                        self.choose(value);
                    }
                } else {
                    self.request_open(true, Initial::Edge(false));
                }
            }
            "ArrowDown" | "ArrowUp" | "Home" | "End" => {
                event.prevent_default();
                let last = matches!(key.as_str(), "ArrowUp" | "End");
                if !open {
                    self.request_open(true, Initial::Edge(last));
                    return;
                }
                let current = self.active.get_untracked();
                let next = self.index.with_untracked(|index| match key.as_str() {
                    "Home" => index.edge(false, |value| untrack(|| self.usable(value))),
                    "End" => index.edge(true, |value| untrack(|| self.usable(value))),
                    _ => index.step(current.as_deref(), if last { -1 } else { 1 }, |value| {
                        untrack(|| self.usable(value))
                    }),
                });
                self.active.set(next);
            }
            _ if key.chars().count() == 1 => {
                event.prevent_default();
                self.search_key(&key);
            }
            _ => {}
        }
    }
    pub(super) fn content_down(self, event: MouseEvent) {
        if event.default_prevented() {
            return;
        }
        if self.target(&event).is_some_and(|target| {
            matches!(
                target.get_attribute("data-rustify-part").as_deref(),
                Some("select-item" | "select-scroll-up" | "select-scroll-down")
            )
        }) {
            event.prevent_default();
        }
    }
    pub(super) fn content_click(self, event: MouseEvent) {
        if event.default_prevented() {
            return;
        }
        if let Some(target) = self.target(&event) {
            match target.get_attribute("data-rustify-part").as_deref() {
                Some("select-scroll-up") => return self.scroll(true),
                Some("select-scroll-down") => return self.scroll(false),
                _ => {}
            }
        }
        if let Some(item) = self.event_item(&event) {
            self.choose(item.value.get_value());
        }
    }
    pub(super) fn hover(self, event: PointerEvent) {
        if event.default_prevented() || event.pointer_type() == "touch" {
            return;
        }
        if let Some(item) = self.event_item(&event) {
            self.active.set(Some(item.value.get_value()));
        }
    }
    pub(super) fn measure_scroll(self) {
        let Some(viewport) = self.viewport.try_get_untracked().flatten() else {
            return;
        };
        let up = viewport.scroll_top() as f64 > 0.0;
        let down = viewport.scroll_top() as f64 + viewport.client_height() as f64 + 1.0
            < viewport.scroll_height() as f64;
        if self.scroll_up.get_untracked() != up {
            self.scroll_up.set(up);
        }
        if self.scroll_down.get_untracked() != down {
            self.scroll_down.set(down);
        }
    }
    pub(super) fn scroll(self, up: bool) {
        if untrack(move || self.disabled()) {
            return;
        }
        if let Some(viewport) = self.viewport.try_get_untracked().flatten() {
            viewport.scroll_by_with_x_and_y(
                0.0,
                viewport.client_height() as f64 * if up { -1.0 } else { 1.0 },
            );
            self.measure_scroll();
        }
    }
    pub(super) fn connect(self) {
        let was_open = StoredValue::new(false);
        Effect::new(move |_| {
            let open = self.open.get() && !self.disabled();
            self.index.track();
            self.registry.track();
            self.present.track();
            if !open {
                self.active.set(None);
                self.reset_search();
                was_open.set_value(false);
                return;
            }
            if !was_open.get_value() {
                was_open.set_value(true);
                let mut initial = None;
                self.initial.update_value(|held| initial = held.take());
                let chosen = self
                    .accepted()
                    .filter(|value| self.index.with(|index| index.available(value)));
                let active = match initial {
                    Some(Initial::Match(value))
                        if self.index.with(|index| index.available(&value)) =>
                    {
                        Some(value)
                    }
                    Some(Initial::Edge(last)) => {
                        chosen.or_else(|| self.index.with(|index| index.edge(last, |_| true)))
                    }
                    _ => chosen.or_else(|| self.index.with(|index| index.edge(false, |_| true))),
                };
                self.active.set(active);
            }
            if self.present.get()
                && self
                    .active
                    .get_untracked()
                    .as_ref()
                    .is_none_or(|value| !self.usable(value))
            {
                let chosen = self.accepted().filter(|value| self.usable(value));
                self.active.set(chosen.or_else(|| {
                    self.index
                        .with(|index| index.edge(false, |value| self.usable(value)))
                }));
            }
        });
        let observer = StoredValue::new_local(None::<rustify_ui::ResizeObservation>);
        Effect::new(move |_| {
            self.width.track();
            let count = self.triggers.get();
            let Some(trigger) = self
                .trigger
                .get()
                .filter(|trigger| count == 1 && trigger.is_connected())
            else {
                observer.set_value(None);
                return;
            };
            self.anchor_failed.set_value(false);
            let measure = move || {
                if let Some(trigger) = self.trigger.try_get_untracked().flatten() {
                    let width = trigger.get_bounding_client_rect().width();
                    if self
                        .width
                        .try_get_untracked()
                        .is_some_and(|previous| previous != width)
                    {
                        self.width.set(width);
                    }
                }
            };
            measure();
            observer.set_value(rustify_ui::observe_resize(trigger.as_ref(), measure));
        });
        Effect::new(move |_| {
            self.active.track();
            self.registry.track();
            self.index.track();
            if !self.present.get() {
                return;
            }
            let Some(viewport) = self.viewport.get() else {
                return;
            };
            let item = self.active.with(|active| {
                active
                    .as_ref()
                    .and_then(|active| self.registry.with(|registry| registry.one(active)))
            });
            if let Some(item) = item.and_then(|item| item.node.try_get_untracked().flatten()) {
                let row = item.get_bounding_client_rect();
                let frame = viewport.get_bounding_client_rect();
                let top = row.top() - frame.top() - viewport.client_top() as f64;
                let bottom = top + row.height();
                let height = viewport.client_height() as f64;
                if top < 0.0 {
                    viewport.scroll_by_with_x_and_y(0.0, top);
                } else if bottom > height {
                    viewport.scroll_by_with_x_and_y(0.0, bottom - height);
                }
            }
            self.measure_scroll();
        });
        Effect::new(move |_| {
            self.value.track();
            self.index.track();
            self.triggers.track();
            self.disabled.track();
            self.mirror.track();
            self.settle();
        });
        on_cleanup(move || {
            self.reset_search();
            observer.try_update_value(|held| {
                held.take();
            });
        });
    }
}
