use super::*;
use leptos::wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Element, HtmlElement, KeyboardEvent, Node};
#[derive(Default)]
pub(super) struct State {
    timer: Option<super::super::timer::Timeout>,
    skip_until: f64,
    previous: Option<String>,
    listeners: Vec<rustify_ui::Listener>,
    observations: Vec<rustify_ui::ResizeObservation>,
    mutation: Option<Mutation>,
    pointer: Option<Node>,
}
struct Mutation {
    observer: web_sys::MutationObserver,
    _callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>,
    _release: rustify_ui::AbortRelease,
}
impl Drop for Mutation {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
pub(super) fn cancel(context: Context) {
    context.browser.update_value(|state| {
        state.timer.take();
    });
}
pub(super) fn enter(context: Context, key: String) {
    cancel(context);
    if !context.valid {
        return;
    }
    let immediate = context.value.get_untracked().is_some()
        || context
            .browser
            .with_value(|state| js_sys::Date::now() < state.skip_until)
        || context.delay == 0;
    if immediate {
        context.request(Some(key));
    } else {
        context.browser.update_value(|state| {
            state.timer = super::super::timer::Timeout::new(context.delay, move || {
                context.request(Some(key.clone()))
            })
        });
    }
}
pub(super) fn leave(context: Context) {
    cancel(context);
    if context.value.get_untracked().is_some() {
        context.browser.update_value(|state| {
            state.timer = super::super::timer::Timeout::new(150, move || context.request(None))
        });
    }
}
fn clean_event(event: &KeyboardEvent) -> bool {
    !event.default_prevented()
        && !event.is_composing()
        && !event.ctrl_key()
        && !event.alt_key()
        && !event.meta_key()
        && !event
            .target()
            .is_some_and(|target| rustify_ui::is_text_entry(&target))
}
fn candidates(root: &Element) -> Vec<HtmlElement> {
    let Ok(nodes) = root
        .query_selector_all("a[href],button,input,select,textarea,[tabindex],[contenteditable]")
    else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|at| nodes.item(at)?.dyn_into::<HtmlElement>().ok())
        .filter(|node| {
            node.tab_index() >= 0
                && !node.has_attribute("disabled")
                && node.get_attribute("aria-disabled").as_deref() != Some("true")
                && node.closest("[hidden],[inert]").ok().flatten().is_none()
                && node.get_client_rects().length() > 0
        })
        .collect()
}
fn move_focus(
    items: Vec<HtmlElement>,
    key: &str,
    direction: Direction,
    orientation: Option<Orientation>,
    event: &KeyboardEvent,
) {
    let current = document().active_element();
    let here = items
        .iter()
        .position(|item| current.as_ref() == Some(item.as_ref()));
    let next = match key {
        "Home" => (!items.is_empty()).then_some(0),
        "End" => items.len().checked_sub(1),
        _ => {
            let step = match key {
                "ArrowRight" => Some(if direction == Direction::RightToLeft {
                    -1
                } else {
                    1
                }),
                "ArrowLeft" => Some(if direction == Direction::RightToLeft {
                    1
                } else {
                    -1
                }),
                "ArrowDown" => Some(1),
                "ArrowUp" => Some(-1),
                _ => None,
            };
            let relevant = orientation.is_none_or(|orientation| orientation.steps(key).is_some());
            if !relevant {
                return;
            }
            step.and_then(|step| super::super::choice::navigation(items.len(), here, step, false))
        }
    };
    if let Some(item) = next.and_then(|at| items.get(at)) {
        event.prevent_default();
        let _ = item.focus();
    }
}
pub(super) fn list_key(context: Context, event: KeyboardEvent) {
    if !clean_event(&event) {
        return;
    }
    let Some(list) = event
        .current_target()
        .and_then(|target| target.dyn_into::<Element>().ok())
    else {
        return;
    };
    let Some(target) = event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok())
    else {
        return;
    };
    if target
        .closest("[data-rustify-part=navigation-menu-list]")
        .ok()
        .flatten()
        .as_ref()
        != Some(&list)
    {
        return;
    }
    if target
        .closest("[data-rustify-part=navigation-menu-content]")
        .ok()
        .flatten()
        .is_some_and(|content| list.contains(Some(&content)))
    {
        return;
    }
    let items = candidates(&list)
        .into_iter()
        .filter(|item| {
            item.get_attribute("data-navigation-owner").as_deref()
                == Some(context.id.get_value().as_str())
        })
        .filter(|item| {
            item.closest("[data-rustify-part=navigation-menu-content]")
                .ok()
                .flatten()
                .is_none_or(|content| !list.contains(Some(&content)))
        })
        .collect();
    move_focus(
        items,
        &event.key(),
        context.direction.get_untracked(),
        Some(context.orientation),
        &event,
    );
}
pub(super) fn trigger_key(member: Member, event: KeyboardEvent) {
    let context = member.context;
    if !clean_event(&event) || !context.active(member.key) {
        return;
    }
    let entry = match context.orientation {
        Orientation::Horizontal => "ArrowDown",
        Orientation::Vertical => {
            if context.direction.get_untracked() == Direction::RightToLeft {
                "ArrowLeft"
            } else {
                "ArrowRight"
            }
        }
    };
    if event.key() != entry {
        return;
    }
    if let Some(content) = member.content.get_untracked() {
        event.prevent_default();
        if let Some(first) = candidates(content.as_ref()).first() {
            let _ = first.focus();
        } else {
            let _ = content.focus();
        }
    }
}
pub(super) fn content_key(member: Member, event: KeyboardEvent) {
    if !clean_event(&event) {
        return;
    }
    let Some(content) = member.content.get_untracked() else {
        return;
    };
    let Some(target) = event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok())
    else {
        return;
    };
    if target
        .closest("[data-rustify-part=navigation-menu-content]")
        .ok()
        .flatten()
        .as_ref()
        != Some(content.as_ref())
    {
        return;
    }
    if event.key() == "Escape" {
        event.prevent_default();
        event.stop_propagation();
        member.context.cancel();
        member.context.request(None);
        if !member.context.active(member.key) {
            if let Some(trigger) = member.trigger.get_untracked() {
                let _ = trigger.focus();
            }
        }
    } else {
        move_focus(
            candidates(content.as_ref()),
            &event.key(),
            member.context.direction.get_untracked(),
            None,
            &event,
        );
    }
}
fn outside(context: Context, target: &Node, overlay: Option<&rustify_ui::OverlayStack>) -> bool {
    !document()
        .get_element_by_id(&context.id.get_value())
        .is_some_and(|root| root.contains(Some(target)))
        && !overlay.is_some_and(|overlay| {
            overlay.depth() > 0 && overlay.overlay_root().contains(Some(target))
        })
}
fn measure(context: Context, entry: &Entry) {
    let Some(root) = document().get_element_by_id(&context.id.get_value()) else {
        return;
    };
    let Some(trigger) = entry.trigger.get_untracked().filter(|trigger| {
        trigger.is_connected() && root.contains(Some(trigger.as_ref())) && !trigger.disabled()
    }) else {
        context.request(None);
        return;
    };
    if let Some(indicator) = context
        .indicator
        .get_untracked()
        .and_then(|node| node.get_untracked())
    {
        let base = root.get_bounding_client_rect();
        let target = trigger.get_bounding_client_rect();
        let (offset, size) = if context.orientation == Orientation::Horizontal {
            (target.left() - base.left(), target.width())
        } else {
            (target.top() - base.top(), target.height())
        };
        let _ = web_sys::HtmlElement::style(indicator.as_ref()).set_property(
            "--rustify-navigation-menu-indicator-offset",
            &format!("{offset}px"),
        );
        let _ = web_sys::HtmlElement::style(indicator.as_ref()).set_property(
            "--rustify-navigation-menu-indicator-size",
            &format!("{size}px"),
        );
    }
    if let (Some(viewport), Some(content)) = (
        context
            .viewport
            .get_untracked()
            .and_then(|node| node.get_untracked()),
        entry.content.get_untracked(),
    ) {
        let _ = web_sys::HtmlElement::style(viewport.as_ref()).set_property(
            "--rustify-navigation-menu-viewport-width",
            &format!("{}px", content.offset_width()),
        );
        let _ = web_sys::HtmlElement::style(viewport.as_ref()).set_property(
            "--rustify-navigation-menu-viewport-height",
            &format!("{}px", content.offset_height()),
        );
    }
}
pub(super) fn install(context: Context, is_root: bool) {
    let overlay = rustify_ui::use_overlay();
    Effect::new(move || {
        let value = context.value.get();
        let entries = context.entries.get();
        context.viewport.track();
        context.indicator.track();
        context.browser.update_value(|state| {
            state.listeners.clear();
            state.observations.clear();
            state.mutation.take();
            state.pointer = None;
            if state.previous != value {
                state.timer.take();
                if value.is_none() {
                    state.skip_until = js_sys::Date::now() + f64::from(context.skip);
                }
                state.previous = value.clone();
            }
        });
        let Some(value) = value else {
            return;
        };
        let Some(entry) = entries.into_iter().find(|entry| entry.key == value) else {
            context.request(None);
            return;
        };
        let Some(trigger) = entry.trigger.get() else {
            return;
        };
        let content = entry.content.get();
        if let Some(viewport) = context.viewport.get() {
            viewport.track();
        }
        if let Some(indicator) = context.indicator.get() {
            indicator.track();
        }
        let Some(root) = document().get_element_by_id(&context.id.get_value()) else {
            return;
        };
        measure(context, &entry);
        if is_root {
            for name in ["pointerdown", "focusin"] {
                let overlay = overlay.clone();
                let listener = rustify_ui::listen(
                    &document().into(),
                    name,
                    rustify_ui::ListenOptions {
                        capture: true,
                        passive: false,
                    },
                    move |event| {
                        let Some(target) = event
                            .target()
                            .and_then(|target| target.dyn_into::<Node>().ok())
                        else {
                            return;
                        };
                        if !outside(context, &target, overlay.as_ref()) {
                            context.browser.update_value(|state| state.pointer = None);
                            return;
                        }
                        let mut duplicate = false;
                        context.browser.update_value(|state| {
                            if name == "pointerdown" {
                                state.pointer = Some(target.clone());
                            } else {
                                duplicate = state.pointer.take().is_some_and(|previous| {
                                    previous == target || previous.contains(Some(&target))
                                });
                            }
                        });
                        if !duplicate {
                            cancel(context);
                            context.request(None);
                        }
                    },
                );
                context
                    .browser
                    .update_value(|state| state.listeners.push(listener));
            }
        }
        let mut elements = vec![root.clone(), trigger.into()];
        if let Some(content) = content {
            elements.push(content.into());
        }
        for element in elements {
            let entry = entry.clone();
            if let Some(observer) =
                rustify_ui::observe_resize(&element, move || measure(context, &entry))
            {
                context
                    .browser
                    .update_value(|state| state.observations.push(observer));
            }
        }
        let scrolling_entry = entry.clone();
        let scroll = rustify_ui::listen(
            root.as_ref(),
            "scroll",
            rustify_ui::ListenOptions {
                capture: true,
                passive: true,
            },
            move |_| measure(context, &scrolling_entry),
        );
        context
            .browser
            .update_value(|state| state.listeners.push(scroll));
        let callback =
            Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(move |_, _| {
                measure(context, &entry)
            });
        if let Ok(observer) = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()) {
            let options = web_sys::MutationObserverInit::new();
            options.set_child_list(true);
            options.set_subtree(true);
            options.set_attributes(true);
            options.set_attribute_filter(&js_sys::Array::of1(&"disabled".into()));
            if observer.observe_with_options(&root, &options).is_ok() {
                let Ok(disconnect) = js_sys::Reflect::get(&observer, &"disconnect".into())
                    .and_then(|value| value.dyn_into::<js_sys::Function>())
                else {
                    observer.disconnect();
                    return;
                };
                let release =
                    rustify_ui::release_on_abort(disconnect.bind0(&observer).unchecked_into());
                context.browser.update_value(|state| {
                    state.mutation = Some(Mutation {
                        observer,
                        _callback: callback,
                        _release: release,
                    })
                });
            }
        }
    });
    on_cleanup(move || {
        context.browser.try_update_value(|state| {
            state.timer.take();
            state.listeners.clear();
            state.observations.clear();
            state.mutation.take();
        });
    });
}
