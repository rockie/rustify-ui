//! Native text editing is a single synchronous document transaction.

use std::cell::Cell;

use crate::browser_frame::{request_animation_frame_with_handle, AnimationFrameRequestHandle};
use leptos::prelude::*;
use rustify_ui::{Anchor, LocalRect, TextEdit};
use serde_json::json;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Event, HtmlTextAreaElement};

use crate::app::Editor;
use crate::text_layout::{font_spec, layout_text};

thread_local! {
    static NEXT_SESSION: Cell<u64> = const { Cell::new(1) };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    pub id: String,
    pub opened: String,
    pub select_all: bool,
    pub token: u64,
}

/// Finishes any earlier session before opening this text layer.
pub fn start(editor: Editor, id: &str, select_all: bool) {
    if !editor
        .doc
        .with_untracked(|doc| doc.get(id).is_some_and(|node| node.kind == "text"))
    {
        return;
    }
    finish(editor);
    let Some(opened) = editor
        .doc
        .with_untracked(|doc| doc.get(id).map(|node| node.text.clone()))
    else {
        return;
    };
    editor.doc.with_untracked(|doc| {
        editor
            .history
            .update_value(|history| history.begin(doc, "Edit text"))
    });
    let token = NEXT_SESSION.with(|next| {
        let token = next.get();
        next.set(token.wrapping_add(1));
        token
    });
    editor.text_session.set(Some(Session {
        id: id.into(),
        opened,
        select_all,
        token,
    }));
    editor.input.update(|input| input.editing = Some(id.into()));
}

/// Commits the current native draft before another operation begins.
pub fn finish(editor: Editor) {
    let Some(session) = editor.text_session.get_untracked() else {
        return;
    };
    let draft = editor
        .text_element
        .with_value(|element| {
            element
                .as_ref()
                .filter(|(token, _)| *token == session.token)
                .map(|(_, element)| element.value())
        })
        .or_else(|| {
            document()
                .query_selector(&format!(".vellum-text-session-{}", session.token))
                .ok()
                .flatten()
                .and_then(|element| element.dyn_into::<HtmlTextAreaElement>().ok())
                .map(|element| element.value())
        });
    complete(editor, session.token, draft);
}

fn take(editor: Editor, token: u64) -> Option<Session> {
    let session = editor
        .text_session
        .try_update(|current| {
            if current
                .as_ref()
                .is_some_and(|session| session.token == token)
            {
                current.take()
            } else {
                None
            }
        })
        .flatten()?;
    editor.text_element.update_value(|element| {
        if element.as_ref().is_some_and(|(id, _)| *id == token) {
            element.take();
        }
    });
    editor.input.update(|input| {
        if input.editing.as_deref() == Some(&session.id) {
            input.editing = None;
        }
    });
    Some(session)
}

fn invalidate(editor: Editor, token: u64) {
    if take(editor, token).is_some() {
        // The pending snapshot predates the external change and must not be restored.
        editor
            .history
            .update_value(|history| history.discard_pending());
    }
}

fn complete(editor: Editor, token: u64, draft: Option<String>) {
    if editor.text_session.is_disposed() {
        return;
    }
    let Some(session) = editor
        .text_session
        .get_untracked()
        .filter(|session| session.token == token)
    else {
        return;
    };
    let unchanged = editor.doc.with_untracked(|doc| {
        doc.get(&session.id)
            .is_some_and(|node| node.text == session.opened)
    });
    if !unchanged {
        invalidate(editor, token);
        return;
    }
    let Some(session) = take(editor, token) else {
        return;
    };
    let mut text = draft.unwrap_or(session.opened).replace("\r\n", "\n");
    if text.ends_with('\n') {
        text.pop();
    }
    editor.doc.update(|doc| {
        if let Some(node) = doc.get_mut(&session.id) {
            node.text = text;
            let name = node.text.trim_matches(|ch| matches!(ch, '\u{0009}'..='\u{000d}' | ' ' | '\u{00a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}'));
            let units:Vec<_> = name.encode_utf16().take(28).collect();
            node.name = if units.is_empty() {"Text".into()} else {String::from_utf16_lossy(&units)};
            node.h = editor.measure.with_value(|measure| (node.font_size * node.line_height).max(layout_text(node, measure).height));
            if node.string("sourceId").is_some_and(|id| !id.is_empty()) {
                let overrides = node.extra.entry("overrides").or_insert_with(|| json!({}));
                if !overrides.is_object() {*overrides = json!({});}
                overrides["text"] = json!(node.text);
                overrides["h"] = json!(node.h);
            }
            doc.touch(Some(&session.id));
        }
        editor.history.update_value(|history| {history.commit(doc);});
    });
}

struct Bindings {
    element: HtmlTextAreaElement,
    input: Closure<dyn FnMut(Event)>,
}

impl Drop for Bindings {
    fn drop(&mut self) {
        let _ = self
            .element
            .remove_event_listener_with_callback("input", self.input.as_ref().unchecked_ref());
    }
}

fn resize(editor: Editor, id: &str, element: &HtmlTextAreaElement) {
    let Some(height) = editor
        .doc
        .with_untracked(|doc| doc.get(id).map(|node| node.h))
    else {
        return;
    };
    let style = element.unchecked_ref::<web_sys::HtmlElement>().style();
    let _ = style.set_property("height", &format!("{height}px"));
    let height = height.max(f64::from(element.scroll_height()));
    let _ = style.set_property("height", &format!("{height}px"));
}

fn bind(
    editor: Editor,
    session: &Session,
    element: HtmlTextAreaElement,
) -> Result<Bindings, JsValue> {
    let input = {
        let element = element.clone();
        let id = session.id.clone();
        Closure::new(move |_: Event| resize(editor, &id, &element))
    };
    let bindings = Bindings { element, input };
    let options =
        rustify_makepad::listener_options().unwrap_or_else(web_sys::AddEventListenerOptions::new);
    bindings
        .element
        .add_event_listener_with_callback_and_add_event_listener_options(
            "input",
            bindings.input.as_ref().unchecked_ref(),
            &options,
        )?;
    Ok(bindings)
}

#[component]
pub fn TextSession(editor: Editor, canvas: NodeRef<leptos::html::Canvas>) -> impl IntoView {
    view! {
        <For each={move || editor.text_session.get().into_iter().collect::<Vec<_>>()} key=|session| session.token children={move |session| view! {<SessionControl editor canvas session/>}}/>
    }
}

#[component]
fn SessionControl(
    editor: Editor,
    canvas: NodeRef<leptos::html::Canvas>,
    session: Session,
) -> impl IntoView {
    let token = session.token;
    let session = StoredValue::new(session);
    let source = Memo::new(move |_| {
        session.with_value(|session| editor.doc.with(|doc| doc.get(&session.id).cloned()))
    });
    let anchor = Signal::derive(move || match (canvas.get(), source.get()) {
        (Some(canvas), Some(node)) => {
            Anchor::region(&canvas.into(), LocalRect::new(0.0, 0.0, node.w, node.h))
        }
        _ => Anchor::Centred,
    });
    let transform = Signal::derive(move || {
        let camera = editor.camera.get();
        editor.frame.with(|frame| {
            session.with_value(|session| {
                frame.world(&session.id).map_or_else(String::new, |item| {
                    let m = item.matrix;
                    format!(
                        "matrix({},{},{},{},{},{})",
                        m[0] * camera.zoom,
                        m[1] * camera.zoom,
                        m[2] * camera.zoom,
                        m[3] * camera.zoom,
                        m[4] * camera.zoom + camera.x,
                        m[5] * camera.zoom + camera.y
                    )
                })
            })
        })
    });
    let value = Signal::derive(move || source.get().map_or_else(String::new, |node| node.text));
    let bindings = StoredValue::new_local(None::<Bindings>);
    let request = StoredValue::new_local(None::<AnimationFrameRequestHandle>);
    let mounted = RwSignal::new(false);
    let view = view! {
        <TextEdit anchor value transform multiline=true class=format!("vellum-text-session vellum-text-session-{token}") test_id="text-editor"
            on_commit=move |value| complete(editor,token,Some(value))
            on_cancel=move || {if editor.text_session.with_untracked(|session| session.as_ref().is_some_and(|session| session.token==token)) {finish(editor);}}
            on_invalidated=move || invalidate(editor,token)/>
    };
    Effect::new(move || {
        if source.get().is_none() {
            invalidate(editor, token);
        }
    });
    Effect::new(move || {
        request.set_value(
            request_animation_frame_with_handle(move || {
                if editor.text_session.is_disposed()
                    || !editor.text_session.with_untracked(|current| {
                        current
                            .as_ref()
                            .is_some_and(|current| current.token == token)
                    })
                {
                    return;
                }
                let Ok(Some(element)) =
                    document().query_selector(&format!(".vellum-text-session-{token}"))
                else {
                    return;
                };
                let Ok(element) = element.dyn_into::<HtmlTextAreaElement>() else {
                    return;
                };
                element.set_spellcheck(false);
                let _ = element.set_attribute("aria-label", "Edit text");
                if !session.with_value(|session| session.select_all) {
                    let _ = element.set_selection_range(0, 0);
                }
                editor
                    .text_element
                    .set_value(Some((token, element.clone())));
                match session.with_value(|session| bind(editor, session, element)) {
                    Ok(listeners) => {
                        bindings.set_value(Some(listeners));
                        mounted.set(true);
                    }
                    Err(error) => editor
                        .error
                        .set(Some(format!("Text input unavailable: {error:?}"))),
                }
            })
            .ok(),
        );
    });
    Effect::new(move || {
        if !mounted.get() {
            return;
        }
        let Some(node) = source.get() else {
            return;
        };
        let opacity = editor.frame.with(|frame| {
            frame
                .world(&node.id)
                .map_or(node.opacity, |item| item.opacity)
        });
        bindings.with_value(|bindings| {
            let Some(bindings) = bindings else {
                return;
            };
            let style = bindings
                .element
                .unchecked_ref::<web_sys::HtmlElement>()
                .style();
            for (name, value) in [
                ("font", font_spec(&node)),
                ("line-height", node.line_height.to_string()),
                ("letter-spacing", format!("{}px", node.letter_spacing)),
                ("text-align", node.text_align.clone()),
                ("text-decoration", node.text_decoration.clone()),
                (
                    "text-transform",
                    match node.text_case.as_str() {
                        "upper" => "uppercase",
                        "lower" => "lowercase",
                        "title" => "capitalize",
                        _ => "none",
                    }
                    .into(),
                ),
                (
                    "direction",
                    if node.direction == "auto" {
                        "inherit".into()
                    } else {
                        node.direction.clone()
                    },
                ),
                (
                    "color",
                    if node.fill == "none" {
                        "#a38bff".into()
                    } else {
                        node.fill.clone()
                    },
                ),
                ("opacity", opacity.to_string()),
            ] {
                let _ = style.set_property(name, &value);
            }
            resize(editor, &node.id, &bindings.element);
        });
    });
    on_cleanup(move || {
        request.update_value(|request| {
            if let Some(request) = request.take() {
                request.cancel();
            }
        });
        bindings.update_value(|bindings| {
            bindings.take();
        });
        editor.text_element.try_update_value(|element| {
            if element.as_ref().is_some_and(|(id, _)| *id == token) {
                element.take();
            }
        });
    });
    view
}
