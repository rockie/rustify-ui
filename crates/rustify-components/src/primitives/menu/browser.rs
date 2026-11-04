use super::*;
use crate::primitives::{
    hover_grace::{self, Point, Rect},
    timer::Timeout,
};
use leptos::wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlElement, KeyboardEvent, PointerEvent};

#[derive(Default)]
pub(crate) struct State {
    search: String,
    search_timer: Option<Timeout>,
    intent: Option<Timeout>,
    grace: Vec<Point>,
    grace_timer: Option<Timeout>,
    transit: Option<rustify_ui::Listener>,
}
fn listen(
    target: &web_sys::EventTarget,
    name: &str,
    run: impl FnMut(Event) + 'static,
) -> rustify_ui::Listener {
    rustify_ui::listen(target, name, Default::default(), run)
}
fn target(event: &Event) -> Option<Element> {
    event.target()?.dyn_into().ok()
}
fn panel(context: Context) -> Option<Element> {
    context.floating.panel.get_untracked().map(Into::into)
}
fn belongs(element: &Element, panel: &Element) -> bool {
    element.closest("[role=menu]").ok().flatten().as_ref() == Some(panel)
}
fn item_at(event: &Event, context: Context) -> Option<Element> {
    let element = target(event)?.closest("[data-menu-item]").ok().flatten()?;
    belongs(&element, &panel(context)?).then_some(element)
}
fn enabled(element: &Element) -> bool {
    element.get_attribute("aria-disabled").as_deref() != Some("true")
}
fn items(panel: &Element) -> Vec<Element> {
    let Ok(nodes) = panel.query_selector_all("[data-menu-item]") else {
        return vec![];
    };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i)?.dyn_into::<Element>().ok())
        .filter(|element| belongs(element, panel))
        .collect()
}
pub(crate) fn focus_element(element: &Element) {
    if let Some(element) = element.dyn_ref::<HtmlElement>() {
        let _ = element.focus();
    }
}
pub(in crate::primitives) fn focus(context: Context) {
    let Some(panel) = panel(context) else {
        return;
    };
    let available: Vec<_> = items(&panel).into_iter().filter(enabled).collect();
    let to = match context.focus.get_value() {
        OpenFocus::Content => None,
        OpenFocus::First => available.first(),
        OpenFocus::Last => available.last(),
    };
    focus_element(to.unwrap_or(&panel));
}
fn clear_grace(context: Context) {
    context.state.try_update_value(|state| {
        state.grace.clear();
        state.grace_timer.take();
        state.transit.take();
    });
}
fn in_grace(context: Context, event: &PointerEvent) -> bool {
    context.state.with_value(|state| {
        hover_grace::contains(
            Point {
                x: event.client_x() as f64,
                y: event.client_y() as f64,
            },
            &state.grace,
        )
    })
}
pub(super) fn install(context: Context, root: bool) {
    Effect::new(move || {
        if !context.floating.open.get() {
            context
                .state
                .update_value(|state| *state = State::default());
        }
    });
    on_cleanup(move || {
        context
            .state
            .try_update_value(|state| *state = State::default());
    });
    if root {
        let held = StoredValue::new(None::<rustify_ui::Listener>);
        Effect::new(move || {
            let active = context.floating.open.get() && context.tree.subs.get() > 0;
            held.update_value(|listener| {
                listener.take();
            });
            if !active {
                return;
            }
            let listener = rustify_ui::listen(
                &document().into(),
                "pointerdown",
                rustify_ui::ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| {
                    let Some(target) = target(&event) else {
                        return;
                    };
                    let Some(panel) = panel(context) else {
                        return;
                    };
                    let trigger = document().get_element_by_id(
                        &context
                            .tree
                            .boundary
                            .get_untracked()
                            .unwrap_or_else(|| context.floating.trigger_id.get_untracked()),
                    );
                    if panel.contains(Some(&target))
                        || trigger.is_some_and(|trigger| trigger.contains(Some(&target)))
                    {
                        return;
                    }
                    let hooks = context.tree.hooks.get_value();
                    model::outside(
                        |event| {
                            if let Some(hook) = hooks.pointer {
                                hook.run(event);
                            }
                        },
                        |event| {
                            if let Some(hook) = hooks.interact {
                                hook.run(event);
                            }
                        },
                        || context.floating.change.run(false),
                    );
                },
            );
            held.set_value(Some(listener));
        });
        on_cleanup(move || {
            held.try_update_value(|listener| {
                listener.take();
            });
        });
    }
}
pub(super) fn sub_lifecycle(context: Context) {
    let counted = StoredValue::new(false);
    let previous_root = StoredValue::new(context.tree.root.open.get_untracked());
    Effect::new(move || {
        let root = context.tree.root.open.get();
        let open = context.floating.open.get();
        let active = root && open;
        if counted.get_value() != active {
            context
                .tree
                .subs
                .update(|n| if active { *n += 1 } else { *n -= 1 });
            counted.set_value(active);
        }
        let closed = previous_root.get_value() && !root;
        previous_root.set_value(root);
        if closed && open {
            context.floating.change.run(false);
        }
    });
    on_cleanup(move || {
        if counted.get_value() {
            context.tree.subs.try_update(|n| *n -= 1);
        }
        if context.floating.open.try_get_untracked() == Some(true) {
            context.floating.change.run(false);
        }
    });
}
pub(super) fn content(context: Context, looping: bool) {
    let held = StoredValue::new(Vec::<rustify_ui::Listener>::new());
    let sub = use_context::<SubContext>()
        .filter(|sub| sub.child.floating.panel_id == context.floating.panel_id);
    Effect::new(move || {
        let Some(node) = context.floating.panel.get() else {
            return;
        };
        let target: web_sys::EventTarget = node.into();
        let down = StoredValue::new_local(None::<Element>);
        held.set_value(vec![
            listen(&target, "keydown", move |event| {
                let Some(key) = event.dyn_ref::<KeyboardEvent>() else {
                    return;
                };
                if key.default_prevented()
                    || key.is_composing()
                    || key.ctrl_key()
                    || key.alt_key()
                    || key.meta_key()
                {
                    return;
                }
                let Some(panel) = panel(context) else {
                    return;
                };
                if !target_from_key(key).is_some_and(|target| belongs(&target, &panel)) {
                    return;
                }
                let name = key.key();
                if name == "Tab" {
                    key.prevent_default();
                    return;
                }
                let backward =
                    if context.direction.get_untracked() == rustify_ui::Direction::RightToLeft {
                        "ArrowRight"
                    } else {
                        "ArrowLeft"
                    };
                if sub.is_some() && name == backward {
                    key.prevent_default();
                    key.stop_propagation();
                    context.floating.change.run(false);
                    return;
                }
                let all = items(&panel);
                let available: Vec<_> = all.iter().filter(|item| enabled(item)).collect();
                let active = document().active_element();
                let current = available
                    .iter()
                    .position(|item| Some(*item) == active.as_ref());
                let index = match name.as_str() {
                    "ArrowDown" => {
                        super::super::choice::navigation(available.len(), current, 1, looping)
                    }
                    "ArrowUp" => {
                        super::super::choice::navigation(available.len(), current, -1, looping)
                    }
                    "Home" | "PageUp" => (!available.is_empty()).then_some(0),
                    "End" | "PageDown" => available.len().checked_sub(1),
                    _ => None,
                };
                if let Some(index) = index {
                    key.prevent_default();
                    focus_element(available[index]);
                    return;
                }
                let searching = context.state.with_value(|state| !state.search.is_empty());
                if name == "Enter" || (name == " " && !searching) {
                    if let Some(item) =
                        active.filter(|item| item.has_attribute("data-menu-item") && enabled(item))
                    {
                        key.prevent_default();
                        if let Some(item) = item.dyn_ref::<HtmlElement>() {
                            item.click();
                        }
                    }
                    return;
                }
                if name.chars().count() != 1 {
                    return;
                }
                key.prevent_default();
                context.state.update_value(|state| {
                    state.search.push_str(&name);
                    state.search_timer = Timeout::new(1000, move || {
                        context.state.try_update_value(|state| state.search.clear());
                    });
                });
                let query = context.state.with_value(|state| state.search.clone());
                let labels: Vec<_> = all
                    .iter()
                    .map(|item| {
                        item.get_attribute("data-text-value")
                            .unwrap_or_else(|| item.text_content().unwrap_or_default())
                    })
                    .collect();
                let enabled: Vec<_> = all.iter().map(enabled).collect();
                let current = all
                    .iter()
                    .position(|item| Some(item) == document().active_element().as_ref());
                if let Some(index) = model::next_match(&labels, &enabled, &query, current) {
                    focus_element(&all[index]);
                }
            }),
            listen(&target, "pointermove", move |event| {
                let Some(pointer) = event
                    .dyn_ref::<PointerEvent>()
                    .filter(|e| e.pointer_type() == "mouse" && !e.default_prevented())
                else {
                    return;
                };
                if in_grace(context, pointer) {
                    return;
                }
                if let Some(item) = item_at(&event, context).filter(enabled) {
                    focus_element(&item);
                }
            }),
            listen(&target, "pointerout", move |event| {
                let Some(pointer) = event
                    .dyn_ref::<PointerEvent>()
                    .filter(|e| e.pointer_type() == "mouse")
                else {
                    return;
                };
                let Some(item) = item_at(&event, context) else {
                    return;
                };
                if item.has_attribute("data-menu-sub-trigger") || in_grace(context, pointer) {
                    return;
                }
                if pointer
                    .related_target()
                    .and_then(|target| target.dyn_into::<Element>().ok())
                    .is_some_and(|target| item.contains(Some(&target)))
                {
                    return;
                }
                if let Some(panel) = panel(context) {
                    focus_element(&panel);
                }
            }),
            listen(&target, "pointerdown", move |event| {
                down.set_value(item_at(&event, context));
            }),
            listen(&target, "pointerup", move |event| {
                let item = item_at(&event, context);
                let pressed = down.get_value();
                down.set_value(None);
                if !event.default_prevented() && item != pressed {
                    if let Some(item) = item
                        .filter(enabled)
                        .and_then(|item| item.dyn_into::<HtmlElement>().ok())
                    {
                        item.click();
                    }
                }
            }),
        ]);
    });
    on_cleanup(move || {
        held.try_update_value(Vec::clear);
    });
}
fn target_from_key(event: &KeyboardEvent) -> Option<Element> {
    event.target()?.dyn_into().ok()
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
pub(super) fn sub_trigger(
    sub: SubContext,
    node: NodeRef<leptos::html::Div>,
    disabled: Signal<bool>,
) {
    let held = StoredValue::new(Vec::<rustify_ui::Listener>::new());
    Effect::new(move || {
        let Some(node) = node.get() else {
            return;
        };
        let element: Element = node.into();
        let target: web_sys::EventTarget = element.clone().into();
        let exit = element.clone();
        held.set_value(vec![
            listen(&target, "keydown", move |event| {
                let Some(key) = event.dyn_ref::<KeyboardEvent>() else {
                    return;
                };
                let forward =
                    if sub.child.direction.get_untracked() == rustify_ui::Direction::RightToLeft {
                        "ArrowLeft"
                    } else {
                        "ArrowRight"
                    };
                if key.key() == forward && !key.default_prevented() && !disabled.get_untracked() {
                    key.prevent_default();
                    key.stop_propagation();
                    sub.child.focus.set_value(OpenFocus::First);
                    if sub.child.floating.open.get_untracked() {
                        focus(sub.child);
                    } else {
                        sub.child.floating.change.run(true);
                    }
                }
            }),
            listen(&target, "pointermove", move |event| {
                let Some(pointer) = event.dyn_ref::<PointerEvent>() else {
                    return;
                };
                if pointer.pointer_type() != "mouse"
                    || disabled.get_untracked()
                    || sub.child.floating.open.get_untracked()
                    || in_grace(sub.parent, pointer)
                {
                    return;
                }
                sub.child.state.update_value(|state| {
                    if state.intent.is_none() {
                        state.intent = Timeout::new(100, move || {
                            sub.child.focus.set_value(OpenFocus::Content);
                            sub.child.floating.change.run(true);
                        });
                    }
                });
            }),
            listen(&target, "pointerleave", move |event| {
                sub.child.state.update_value(|state| {
                    state.intent.take();
                });
                let Some(pointer) = event
                    .dyn_ref::<PointerEvent>()
                    .filter(|e| e.pointer_type() == "mouse")
                else {
                    return;
                };
                if !sub.child.floating.open.get_untracked() {
                    return;
                }
                let Some(content) = panel(sub.child) else {
                    return;
                };
                let polygon = hover_grace::polygon(
                    Point {
                        x: pointer.client_x() as f64,
                        y: pointer.client_y() as f64,
                    },
                    rect(&exit),
                    rect(&content),
                );
                clear_grace(sub.parent);
                let transit = listen(&document().into(), "pointermove", move |event| {
                    let Some(pointer) = event.dyn_ref::<PointerEvent>() else {
                        return;
                    };
                    if target_from_pointer(pointer)
                        .is_some_and(|target| content.contains(Some(&target)))
                        || !in_grace(sub.parent, pointer)
                    {
                        clear_grace(sub.parent);
                    }
                });
                sub.parent.state.update_value(|state| {
                    state.grace = polygon;
                    state.transit = Some(transit);
                    state.grace_timer = Timeout::new(300, move || clear_grace(sub.parent));
                });
            }),
        ]);
    });
    on_cleanup(move || {
        held.try_update_value(Vec::clear);
        clear_grace(sub.parent);
    });
}
fn target_from_pointer(event: &PointerEvent) -> Option<Element> {
    event.target()?.dyn_into().ok()
}

pub(in crate::primitives) fn context_trigger<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    context: Context,
    disabled: Signal<bool>,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    let keys = StoredValue::new_local(model::ContextKey::default());
    let touch = StoredValue::new_local(None::<Timeout>);
    let cancel_touch = move || {
        touch.try_update_value(|timer| {
            timer.take();
        });
    };
    let unavailable = move || {
        disabled.get_untracked()
            || context.floating.triggers.get_untracked().len() != 1
            || context.floating.portals.get_untracked() != 1
    };
    let open_at = move |x: f64, y: f64, request: bool| {
        let Some(element) =
            document().get_element_by_id(&context.floating.trigger_id.get_untracked())
        else {
            return;
        };
        context
            .floating
            .virtual_anchor
            .set(Some(crate::anchor_at(&element, x, y)));
        context.focus.set_value(OpenFocus::Content);
        if request && !context.floating.open.get_untracked() {
            context.floating.change.run(true);
        }
    };
    let release_key = rustify_ui::listen(
        &document().into(),
        "keyup",
        Default::default(),
        move |event| {
            if let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                keys.update_value(|keys| keys.up(&event.key()));
            }
        },
    );
    on_cleanup(move || drop(release_key));
    let held = StoredValue::new(None::<rustify_ui::Listener>);
    Effect::new(move || {
        let id = context.floating.trigger_id.get();
        if let Some(element) = document().get_element_by_id(&id) {
            let focusable=element.matches("button:not(:disabled), a[href], input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [tabindex], [contenteditable=true]").unwrap_or(false);
            context
                .floating
                .interaction_issue
                .set((!focusable).then_some("unfocusable-trigger"));
        }
        let active = context.floating.open.get() && context.floating.modal && !disabled.get();
        held.update_value(|listener| {
            listener.take();
        });
        if active {
            held.set_value(Some(rustify_ui::listen(
                &document().into(),
                "contextmenu",
                rustify_ui::ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| {
                    let Some(event) = event.dyn_ref::<web_sys::MouseEvent>() else {
                        return;
                    };
                    if keys.with_value(|keys| keys.consumes_contextmenu()) {
                        event.prevent_default();
                        return;
                    }
                    let Some(trigger) =
                        document().get_element_by_id(&context.floating.trigger_id.get_untracked())
                    else {
                        return;
                    };
                    let bounds = trigger.get_bounding_client_rect();
                    let x = event.client_x() as f64;
                    let y = event.client_y() as f64;
                    if x >= bounds.left()
                        && x <= bounds.right()
                        && y >= bounds.top()
                        && y <= bounds.bottom()
                    {
                        event.prevent_default();
                        open_at(x, y, false);
                    }
                },
            )));
        }
    });
    on_cleanup(move || {
        held.try_update_value(|listener| {
            listener.take();
        });
        cancel_touch();
    });
    target
        .on(leptos::ev::contextmenu, move |event| {
            if unavailable() {
                return;
            }
            cancel_touch();
            if keys.with_value(|keys| keys.consumes_contextmenu()) {
                event.prevent_default();
                return;
            }
            let request = !event.default_prevented();
            event.prevent_default();
            open_at(event.client_x() as f64, event.client_y() as f64, request);
        })
        .on(leptos::ev::keydown, move |event| {
            if unavailable() || event.default_prevented() || event.is_composing() {
                return;
            }
            let name = event.key();
            if name != "ContextMenu" && !(name == "F10" && event.shift_key()) {
                return;
            }
            event.prevent_default();
            cancel_touch();
            let mut request = false;
            keys.update_value(|keys| request = keys.down(&name, event.shift_key()));
            if !request {
                return;
            }
            if let Some(element) =
                document().get_element_by_id(&context.floating.trigger_id.get_untracked())
            {
                context
                    .floating
                    .virtual_anchor
                    .set(Some(rustify_ui::Anchor::element(&element)));
                context.focus.set_value(OpenFocus::First);
                if context.floating.open.get_untracked() {
                    focus(context);
                } else {
                    context.floating.change.run(true);
                }
            }
        })
        .on(leptos::ev::pointerdown, move |event| {
            if unavailable() || event.default_prevented() {
                return;
            }
            cancel_touch();
            if event.button() == 0
                && !context.floating.modal
                && context.floating.open.get_untracked()
            {
                context.floating.change.run(false);
            }
            if event.pointer_type() != "mouse" {
                let x = event.client_x() as f64;
                let y = event.client_y() as f64;
                touch.set_value(Timeout::new(700, move || open_at(x, y, true)));
            }
        })
        .on(leptos::ev::pointermove, move |_| cancel_touch())
        .on(leptos::ev::pointerup, move |_| cancel_touch())
        .on(leptos::ev::pointercancel, move |_| cancel_touch())
}
