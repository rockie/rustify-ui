//! Editing GPU-drawn text with the browser's own text control.
//!
//! A region draws its text and, when the user starts editing, hands over the
//! rectangle it drew into. A real `input` or `textarea` takes exactly that
//! rectangle for as long as the session lasts, so selection, the caret, the
//! system's undo and - the reason this exists - the input method are the
//! browser's, not a redrawing of them.
//!
//! The session is controlled like everything else: it opens with the value the
//! application holds and proposes a new one. It never writes an outside change
//! over what is being typed; if the value moves underneath it, the session ends
//! and says so.

#[cfg(target_arch = "wasm32")]
pub use dom::TextEdit;

#[cfg(target_arch = "wasm32")]
mod dom {
    use crate::overlay::{use_overlay, Anchor};
    use crate::overlay::{EditSession, LocalRect};
    use leptos::html::{Input, Textarea};
    use leptos::portal::Portal;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::{Element, HtmlElement, HtmlInputElement, HtmlTextAreaElement};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    /// What the native control holds right now.
    fn draft(element: &Element) -> String {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            return input.value();
        }
        if let Some(area) = element.dyn_ref::<HtmlTextAreaElement>() {
            return area.value();
        }
        String::new()
    }

    fn set_draft(element: &Element, value: &str) {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_value(value);
        } else if let Some(area) = element.dyn_ref::<HtmlTextAreaElement>() {
            area.set_value(value);
        }
    }

    fn select_all(element: &Element) {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            let _ = input.select();
        } else if let Some(area) = element.dyn_ref::<HtmlTextAreaElement>() {
            let _ = area.select();
        }
    }

    /// A native text control placed over the rectangle a region drew its text
    /// into.
    ///
    /// The application owns whether the session exists; every way it can end -
    /// commit, cancel, or the value moving underneath it - is a callback, so
    /// the region and the application never hold two different ideas of the
    /// value at once.
    #[component]
    pub fn TextEdit(
        /// The rectangle inside the region that the text occupies.
        #[prop(into)]
        anchor: Signal<Anchor>,
        /// The controlled value. The session opens with it, and ends if it
        /// changes from outside while the session is open.
        #[prop(into)]
        value: Signal<String>,
        /// Optional CSS transform applied after the anchor places the control.
        /// The anchor remains untransformed; use the control's class to choose
        /// a transform origin when supplying an application-space matrix.
        #[prop(optional)]
        transform: Option<Signal<String>>,
        #[prop(optional)] multiline: bool,
        on_commit: impl Fn(String) + Send + Sync + 'static,
        on_cancel: impl Fn() + Send + Sync + 'static,
        /// The value changed underneath the session, so the draft was dropped
        /// rather than written over the new value.
        on_invalidated: impl Fn() + Send + Sync + 'static,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        use_overlay().map(|stack| {
            let root = stack.overlay_root();
            let moved = stack.moved();
            let class = format!("rustify-text-edit {class}");
            let input = NodeRef::<Input>::new();
            let area = NodeRef::<Textarea>::new();
            let rect = RwSignal::new(LocalRect::default());
            // One flag for the whole session: a commit must not be followed by
            // the commit a blur would otherwise make.
            let finished = Arc::new(AtomicBool::new(false));
            let composing = Arc::new(AtomicBool::new(false));
            let on_commit = Arc::new(on_commit);
            let on_cancel = Arc::new(on_cancel);

            let element = move || -> Option<Element> {
                if multiline {
                    area.get().map(Into::into)
                } else {
                    input.get().map(Into::into)
                }
            };

            let commit = {
                let finished = finished.clone();
                let on_commit = on_commit.clone();
                move || {
                    if finished.swap(true, Ordering::Relaxed) {
                        return;
                    }
                    if let Some(element) = element() {
                        on_commit(draft(&element));
                    }
                }
            };
            let cancel = {
                let finished = finished.clone();
                let on_cancel = on_cancel.clone();
                move || {
                    if finished.swap(true, Ordering::Relaxed) {
                        return;
                    }
                    on_cancel();
                }
            };

            // The session takes the value it opened with. Anything that moves
            // it afterwards is the application changing the field underneath
            // the user, which ends the session instead of overwriting it.
            let opened_with = StoredValue::new(None::<String>);
            Effect::new(move || {
                let current = value.get();
                let Some(element) = element() else {
                    return;
                };
                match opened_with.get_value() {
                    None => {
                        opened_with.set_value(Some(current.clone()));
                        set_draft(&element, &current);
                        if let Some(element) = element.dyn_ref::<HtmlElement>() {
                            let _ = element.focus();
                        }
                        select_all(&element);
                    }
                    Some(started) if started != current => {
                        if !finished.swap(true, Ordering::Relaxed) {
                            on_invalidated();
                        }
                    }
                    Some(_) => {}
                }
            });

            // The control covers exactly what the region drew.
            Effect::new(move || {
                moved.get();
                let anchor = anchor.get();
                if let Some(Some(placed)) = anchor.viewport_rect() {
                    rect.set(placed);
                }
            });

            {
                let composing = composing.clone();
                let cancel = cancel.clone();
                let stack = stack.clone();
                Effect::new(move || {
                    if element().is_none() {
                        return;
                    }
                    let composing = composing.clone();
                    let cancel = cancel.clone();
                    stack.begin_edit(EditSession {
                        composing: Arc::new(move || composing.load(Ordering::Relaxed)),
                        cancel: Arc::new(move || cancel()),
                    });
                });
            }

            on_cleanup({
                let stack = stack.clone();
                move || stack.end_edit()
            });

            let on_key = {
                let commit = commit.clone();
                let composing = composing.clone();
                move |event: leptos::ev::KeyboardEvent| {
                    // While composing, the keys belong to the composition. The
                    // scope's handler has already stopped Escape; Enter has to
                    // stop here, before a default action sees it.
                    if composing.load(Ordering::Relaxed) {
                        if event.key() == "Enter" {
                            event.prevent_default();
                        }
                        return;
                    }
                    if event.key() == "Enter" && !multiline {
                        event.prevent_default();
                        commit();
                    }
                }
            };
            let on_composition = {
                let composing = composing.clone();
                move |active: bool| composing.store(active, Ordering::Relaxed)
            };
            // Losing the control commits, exactly once: the same flag the
            // Enter path spends.
            let on_blur = move |_: leptos::ev::FocusEvent| commit();

            // Held in the arena so the rendered control captures nothing but
            // copies, and can therefore be built more than once.
            let class = StoredValue::new(class);
            let test_id = StoredValue::new(test_id);
            let on_key = StoredValue::new(on_key);
            let on_composition = StoredValue::new(on_composition);
            let on_blur = StoredValue::new(on_blur);

            let style_left = move || format!("{}px", rect.get().x);
            let style_top = move || format!("{}px", rect.get().y);
            let style_width = move || format!("{}px", rect.get().width);
            let style_height = move || format!("{}px", rect.get().height);

            view! {
                <Portal mount=root>
                    {move || {
                        if multiline {
                            view! {
                                <textarea
                                    node_ref=area
                                    class=class.get_value()
                                    data-testid=test_id.get_value()
                                    style:left=style_left
                                    style:top=style_top
                                    style:width=style_width
                                    style:height=style_height
                                    style:transform=move || transform.map(|value| value.get()).unwrap_or_default()
                                    on:keydown=move |event| on_key.with_value(|key| key(event))
                                    on:compositionstart=move |_| {
                                        on_composition.with_value(|set| set(true))
                                    }
                                    on:compositionend=move |_| {
                                        on_composition.with_value(|set| set(false))
                                    }
                                    on:blur=move |event| on_blur.with_value(|blur| blur(event))
                                ></textarea>
                            }
                                .into_any()
                        } else {
                            view! {
                                <input
                                    type="text"
                                    node_ref=input
                                    class=class.get_value()
                                    data-testid=test_id.get_value()
                                    style:left=style_left
                                    style:top=style_top
                                    style:width=style_width
                                    style:height=style_height
                                    style:transform=move || transform.map(|value| value.get()).unwrap_or_default()
                                    on:keydown=move |event| on_key.with_value(|key| key(event))
                                    on:compositionstart=move |_| {
                                        on_composition.with_value(|set| set(true))
                                    }
                                    on:compositionend=move |_| {
                                        on_composition.with_value(|set| set(false))
                                    }
                                    on:blur=move |event| on_blur.with_value(|blur| blur(event))
                                />
                            }
                                .into_any()
                        }
                    }}
                </Portal>
            }
        })
    }
}
