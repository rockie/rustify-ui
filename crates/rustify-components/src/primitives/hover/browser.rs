use super::*;
use crate::primitives::hover_grace::{self, Point, Rect};
use leptos::wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Element, Event, PointerEvent};

pub(super) struct Timer {
    id: i32,
    _callback: Closure<dyn FnMut()>,
    _release: rustify_ui::AbortRelease,
}
impl Timer {
    fn new(delay: u32, run: impl Fn() + 'static) -> Option<Self> {
        let callback = Closure::<dyn FnMut()>::new(run);
        let clear = js_sys::Reflect::get(&window(), &"clearTimeout".into())
            .ok()?
            .dyn_into::<js_sys::Function>()
            .ok()?;
        let id = window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                i32::try_from(delay).ok()?,
            )
            .ok()?;
        let release =
            rustify_ui::release_on_abort(clear.bind1(&window(), &id.into()).unchecked_into());
        Some(Self {
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
fn now() -> f64 {
    window()
        .performance()
        .map_or(0., |performance| performance.now())
}
fn listen(
    target: &web_sys::EventTarget,
    name: &str,
    run: impl FnMut(Event) + 'static,
) -> rustify_ui::Listener {
    rustify_ui::listen(target, name, Default::default(), run)
}
fn rect(element: &Element) -> Rect {
    let r = element.get_bounding_client_rect();
    Rect {
        x: r.x(),
        y: r.y(),
        width: r.width(),
        height: r.height(),
    }
}
fn mouse(event: &Event) -> Option<&PointerEvent> {
    event
        .dyn_ref::<PointerEvent>()
        .filter(|event| event.pointer_type() != "touch" && !event.default_prevented())
}
fn selected() -> bool {
    window()
        .get_selection()
        .ok()
        .flatten()
        .is_some_and(|selection| !selection.is_collapsed())
}
impl State {
    pub(super) fn cancel(self) {
        if self.pending.try_get_untracked() == Some(true) {
            self.pending.try_set(false);
        }
        self.timer.try_update_value(|held| {
            held.take();
        });
        self.transit.try_update_value(|held| {
            held.take();
        });
    }
    fn request(self, open: bool, delay: u32) {
        self.cancel();
        if self.context.triggers.get_untracked().len() != 1
            || self.context.portals.get_untracked() != 1
        {
            return;
        }
        if self
            .context
            .open
            .try_get_untracked()
            .is_none_or(|accepted| accepted == open)
            || open && self.blocked.get_value()
        {
            return;
        }
        if open && !self.valid_trigger() {
            return;
        }
        let context = self.context;
        if delay == 0 {
            context.change.try_run(open);
        } else {
            let timer = Timer::new(delay, move || {
                self.pending.try_set(false);
                if !open || self.valid_trigger() {
                    context.change.try_run(open);
                }
            });
            self.pending.set(timer.is_some());
            self.timer.set_value(timer);
        }
    }
    fn blur(self) {
        self.cancel();
        if !self.context.open.get_untracked() || self.blocked.get_value() {
            return;
        }
        let timer = Timer::new(self.options.close_delay, move || {
            self.pending.try_set(false);
            if self.context.open.try_get_untracked() == Some(true) && self.valid_trigger() {
                self.context.change.try_run(false);
            }
        });
        self.pending.set(timer.is_some());
        self.timer.set_value(timer);
    }
    fn open_delay(self) -> u32 {
        if self.provider.is_some_and(|provider| {
            provider.active.get_untracked() > 0 || now() < provider.until.get_value()
        }) {
            0
        } else {
            self.options.open_delay
        }
    }
    fn trigger(self) -> Option<Element> {
        document().get_element_by_id(&self.context.trigger_id.get_untracked())
    }
    fn valid_trigger(self) -> bool {
        self.trigger().is_some_and(|element| {
            element.is_connected()
                && !element
                    .matches(":disabled,[aria-disabled=true]")
                    .unwrap_or(false)
                && element.closest("[hidden],[inert]").ok().flatten().is_none()
                && element.get_client_rects().length() > 0
                && window()
                    .get_computed_style(&element)
                    .ok()
                    .flatten()
                    .is_none_or(|style| {
                        style
                            .get_property_value("visibility")
                            .is_ok_and(|value| value != "hidden" && value != "collapse")
                    })
        })
    }
    fn panel(self) -> Option<Element> {
        self.context.panel.get_untracked().map(Into::into)
    }
    fn enter(self, event: &Event, content: bool) {
        if mouse(event).is_none() {
            return;
        }
        self.inside
            .update_value(|inside| inside[usize::from(content)] = true);
        self.pressed.set_value(false);
        self.cancel();
        if !content {
            self.blocked.set_value(false);
            self.request(true, self.open_delay());
        }
    }
    fn close_if_unoccupied(self) {
        if self.inside.get_value().iter().any(|inside| *inside)
            || self.context.kind == Kind::HoverCard && (self.pressed.get_value() || selected())
        {
            return;
        }
        self.request(false, self.options.close_delay);
    }
    fn leave(self, event: &Event, content: bool) {
        let Some(pointer) = mouse(event) else {
            return;
        };
        self.inside
            .update_value(|inside| inside[usize::from(content)] = false);
        self.cancel();
        if !self.context.open.get_untracked() {
            return;
        }
        if self.context.kind == Kind::HoverCard && (self.pressed.get_value() || selected()) {
            return;
        }
        if self
            .provider
            .is_some_and(|provider| provider.disable_hoverable)
        {
            self.request(false, 0);
            return;
        }
        let Some(source) = (if content {
            self.panel()
        } else {
            self.trigger()
        }) else {
            self.close_if_unoccupied();
            return;
        };
        let Some(target) = (if content {
            self.trigger()
        } else {
            self.panel()
        }) else {
            self.close_if_unoccupied();
            return;
        };
        let hull = hover_grace::polygon(
            Point {
                x: pointer.client_x() as f64,
                y: pointer.client_y() as f64,
            },
            rect(&source),
            rect(&target),
        );
        let listener = listen(document().as_ref(), "pointermove", move |event| {
            let Some(pointer) = mouse(&event) else {
                return;
            };
            let hit = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::Node>().ok());
            if self
                .trigger()
                .is_some_and(|element| element.contains(hit.as_ref()))
                || self
                    .panel()
                    .is_some_and(|element| element.contains(hit.as_ref()))
            {
                self.cancel();
                return;
            }
            if !hover_grace::contains(
                Point {
                    x: pointer.client_x() as f64,
                    y: pointer.client_y() as f64,
                },
                &hull,
            ) {
                self.close_if_unoccupied();
            }
        });
        self.transit.set_value(Some(listener));
    }
}
fn described(context: Context, trigger: &Element) {
    let current = trigger
        .get_attribute("aria-describedby")
        .unwrap_or_default();
    let id = context.panel_id.get_value();
    let mut ids: Vec<&str> = current
        .split_whitespace()
        .filter(|value| *value != id)
        .collect();
    if context.open.get_untracked() {
        ids.push(&id);
    }
    let mut unique = Vec::new();
    for id in ids {
        if !unique.contains(&id) {
            unique.push(id);
        }
    }
    let next = unique.join(" ");
    if next != current {
        if next.is_empty() {
            let _ = trigger.remove_attribute("aria-describedby");
        } else {
            let _ = trigger.set_attribute("aria-describedby", &next);
        }
    }
}
struct Observation {
    observer: web_sys::MutationObserver,
    _callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>,
    _release: rustify_ui::AbortRelease,
}
impl Observation {
    fn new(
        target: &Element,
        attributes: &[&str],
        subtree: bool,
        run: impl Fn() + 'static,
    ) -> Option<Self> {
        let callback =
            Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(move |_, _| run());
        let observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()).ok()?;
        let options = web_sys::MutationObserverInit::new();
        options.set_attributes(true);
        options.set_subtree(subtree);
        options.set_child_list(subtree);
        let filter = js_sys::Array::new();
        for name in attributes {
            filter.push(&(*name).into());
        }
        options.set_attribute_filter(&filter);
        let disconnect = js_sys::Reflect::get(&observer, &"disconnect".into())
            .ok()?
            .dyn_into::<js_sys::Function>()
            .ok()?;
        observer.observe_with_options(target, &options).ok()?;
        let release = rustify_ui::release_on_abort(disconnect.bind0(&observer));
        Some(Self {
            observer,
            _callback: callback,
            _release: release,
        })
    }
}
impl Drop for Observation {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
pub(super) fn install(state: State) {
    let context = state.context;
    let panels = StoredValue::new_local(Vec::<rustify_ui::Listener>::new());
    let open_listeners = StoredValue::new_local(Vec::<rustify_ui::Listener>::new());
    let description = StoredValue::new_local(None::<Observation>);
    let content = StoredValue::new_local(None::<Observation>);
    let lifetime = StoredValue::new_local(None::<Observation>);
    let accepted = StoredValue::new(false);
    Effect::new(move || {
        let open = context.open.get();
        state.cancel();
        if context.kind == Kind::Tooltip {
            if let Some(provider) = state.provider {
                if open != accepted.get_value() {
                    if open {
                        provider.active.update(|n| *n += 1);
                    } else {
                        provider.active.update(|n| *n -= 1);
                        provider.until.set_value(now() + provider.skip as f64);
                    }
                }
            }
            if let Some(trigger) = state.trigger() {
                described(context, &trigger);
            }
        }
        accepted.set_value(open);
        let mut listeners = Vec::new();
        if open && context.kind == Kind::HoverCard {
            listeners.push(listen(document().as_ref(), "pointerup", move |_| {
                if state.pressed.get_value() {
                    state.pressed.set_value(false);
                    state.close_if_unoccupied();
                }
            }));
            listeners.push(listen(document().as_ref(), "selectionchange", move |_| {
                state.close_if_unoccupied()
            }));
        }
        open_listeners.set_value(listeners);
    });
    Effect::new(move || {
        context.triggers.track();
        let pending = state.pending.get();
        let open = context.open.get();
        lifetime.set_value(None);
        if !pending && !open {
            return;
        }
        if !state.valid_trigger() {
            state.cancel();
            return;
        }
        if let Some(parent) = state.trigger().and_then(|trigger| trigger.parent_element()) {
            lifetime.set_value(Observation::new(
                &parent,
                &[
                    "disabled",
                    "aria-disabled",
                    "hidden",
                    "inert",
                    "style",
                    "class",
                ],
                true,
                move || {
                    if !state.valid_trigger() {
                        state.cancel();
                    }
                },
            ));
        }
    });
    Effect::new(move || {
        context.triggers.track();
        let id = context.trigger_id.get();
        description.set_value(None);
        let Some(trigger) = document().get_element_by_id(&id) else {
            return;
        };
        if context.kind == Kind::Tooltip {
            let observed = trigger.clone();
            described(context, &trigger);
            description.set_value(Observation::new(
                &trigger,
                &["aria-describedby"],
                false,
                move || described(context, &observed),
            ));
        }
    });
    Effect::new(move || {
        panels.set_value(Vec::new());
        content.set_value(None);
        let Some(panel) = context.panel.get() else {
            return;
        };
        let panel: Element = panel.into();
        panels.set_value(vec![
            listen(panel.as_ref(), "pointerenter", move |event| {
                state.enter(&event, true)
            }),
            listen(panel.as_ref(), "pointerleave", move |event| {
                state.leave(&event, true)
            }),
            listen(panel.as_ref(), "pointerdown", move |event| {
                if !event.default_prevented() {
                    state.pressed.set_value(true);
                    state.cancel();
                }
            }),
            listen(panel.as_ref(), "focusout", move |event| {
                let destination = event
                    .dyn_ref::<web_sys::FocusEvent>()
                    .and_then(|event| event.related_target())
                    .and_then(|target| target.dyn_into::<web_sys::Node>().ok());
                if !state
                    .panel()
                    .is_some_and(|panel| panel.contains(destination.as_ref()))
                    && !state
                        .trigger()
                        .is_some_and(|trigger| trigger.contains(destination.as_ref()))
                {
                    state.blur();
                }
            }),
        ]);
        if context.kind == Kind::Tooltip {
            let observed = panel.clone();
            let check = move || {
                if context.interaction_issue.get_untracked() == Some("invalid-delay") {
                    return;
                }
                let interactive=observed.query_selector("a[href],area[href],button:not(:disabled),input:not(:disabled):not([type=hidden]),select:not(:disabled),textarea:not(:disabled),iframe,details>summary:first-of-type,[contenteditable]:not([contenteditable=false]),[tabindex]:not([tabindex='-1'])").ok().flatten().is_some();
                let issue = interactive.then_some("interactive-tooltip");
                if context.interaction_issue.get_untracked() != issue {
                    context.interaction_issue.set(issue);
                }
            };
            check();
            content.set_value(Observation::new(
                &panel,
                &["href", "disabled", "type", "tabindex", "contenteditable"],
                true,
                check,
            ));
        }
    });
    on_cleanup(move || {
        state.cancel();
        if accepted.get_value() && context.kind == Kind::Tooltip {
            if let Some(provider) = state.provider {
                provider.active.try_update(|n| *n -= 1);
                provider.until.try_set_value(now() + provider.skip as f64);
            }
        }
        if context.kind == Kind::Tooltip {
            if let Some(trigger) = state.trigger() {
                let id = context.panel_id.get_value();
                let rest = trigger
                    .get_attribute("aria-describedby")
                    .unwrap_or_default()
                    .split_whitespace()
                    .filter(|part| *part != id)
                    .collect::<Vec<_>>()
                    .join(" ");
                if rest.is_empty() {
                    let _ = trigger.remove_attribute("aria-describedby");
                } else {
                    let _ = trigger.set_attribute("aria-describedby", &rest);
                }
            }
        }
        panels.try_update_value(Vec::clear);
        open_listeners.try_update_value(Vec::clear);
        description.try_update_value(|held| {
            held.take();
        });
        content.try_update_value(|held| {
            held.take();
        });
        lifetime.try_update_value(|held| {
            held.take();
        });
    });
}

pub(super) fn trigger<E, At, Ch, A>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    attrs: A,
    state: State,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
    A: leptos::attr::Attribute + Send,
{
    let target = target
        .on(leptos::ev::pointerenter, move |event| {
            state.enter(&event, false)
        })
        .on(leptos::ev::pointerleave, move |event| {
            state.leave(&event, false)
        })
        .on(leptos::ev::pointerdown, move |event| {
            if !event.default_prevented() {
                state.pressed.set_value(true);
                state.cancel();
            }
        })
        .on(leptos::ev::focusin, move |event| {
            if !event.default_prevented() && !state.pressed.get_value() {
                state.blocked.set_value(false);
                state.request(true, 0);
            }
        })
        .on(leptos::ev::focusout, move |event| {
            if event.default_prevented() {
                return;
            }
            let destination = event
                .related_target()
                .and_then(|target| target.dyn_into::<web_sys::Node>().ok());
            if state
                .panel()
                .is_some_and(|panel| panel.contains(destination.as_ref()))
            {
                state.cancel();
            } else {
                state.blur();
            }
        })
        .on(leptos::ev::click, move |event| {
            state.pressed.set_value(false);
            if state.context.kind == Kind::Tooltip && !event.default_prevented() {
                state.blocked.set_value(true);
                state.request(false, 0);
            }
        });
    crate::primitives::Slot(target, attrs)
}
