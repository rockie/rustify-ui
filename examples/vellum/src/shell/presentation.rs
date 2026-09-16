//! Frame presentation shares the export painter and the scope's modal layer stack.

use leptos::prelude::*;
use rustify_ui::{Anchor, Layer};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, KeyboardEvent, MouseEvent,
};

use crate::{affine::Point, app::Editor, hit, icons::icon};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewState {
    pub frames: Vec<String>,
    pub index: usize,
}

impl PreviewState {
    fn current(&self) -> Option<&str> {
        self.frames.get(self.index).map(String::as_str)
    }
}

pub fn is_open(editor: Editor) -> bool {
    editor.shell.with_untracked(|shell| shell.preview.is_some())
}

pub fn present(editor: Editor) {
    super::text_session::finish(editor);
    let active = editor.selection.with_untracked(|ids| ids.first().cloned());
    let preview = editor.doc.with_untracked(|doc| {
        let frames: Vec<_> = doc
            .nodes()
            .iter()
            .filter(|node| {
                node.kind == "frame"
                    && node.parent_id.as_deref().is_none_or(str::is_empty)
                    && node.visible
            })
            .map(|node| node.id.clone())
            .collect();
        let ancestor = active
            .as_deref()
            .and_then(|id| doc.get(id))
            .and_then(|node| {
                if node.parent_id.as_deref().is_none_or(str::is_empty) {
                    Some(node)
                } else {
                    doc.ancestors(node).last().copied()
                }
            });
        let index = ancestor
            .and_then(|node| frames.iter().position(|id| id == &node.id))
            .unwrap_or_default();
        PreviewState { frames, index }
    });
    if preview.frames.is_empty() {
        super::toast(editor, "Create a frame to present your design.");
        return;
    }
    super::menus::close(editor);
    editor.shell.update(|shell| shell.preview = Some(preview));
    editor.input.update(|input| input.modal_open = true);
}

pub fn step(editor: Editor, delta: i32) {
    editor.shell.update(|shell| {
        if let Some(preview) = shell
            .preview
            .as_mut()
            .filter(|preview| !preview.frames.is_empty())
        {
            preview.index = (preview.index as i64 + i64::from(delta))
                .rem_euclid(preview.frames.len() as i64) as usize;
        }
    });
}

pub fn close(editor: Editor) {
    editor.shell.update(|shell| shell.preview = None);
    let dialog_open = editor.shell.with_untracked(|shell| shell.dialog.is_some());
    editor.input.update(|input| input.modal_open = dialog_open);
}

#[component]
pub fn Presentation(editor: Editor) -> impl IntoView {
    // Keeping the layer mounted while the index changes preserves its focus return target.
    let open = Memo::new(move |_| editor.shell.with(|shell| shell.preview.is_some()));
    view! { {move || open.get().then(|| view! { <Preview editor/> })} }
}

#[component]
fn Preview(editor: Editor) -> impl IntoView {
    let root = NodeRef::<leptos::html::Div>::new();
    let stage = NodeRef::<leptos::html::Div>::new();
    let canvas = NodeRef::<leptos::html::Canvas>::new();
    let preview = Memo::new(move |_| editor.shell.with(|shell| shell.preview.clone()));
    let title = Memo::new(move |_| {
        preview.with(|preview| {
            preview
                .as_ref()
                .and_then(PreviewState::current)
                .and_then(|id| {
                    editor
                        .doc
                        .with(|doc| doc.get(id).map(|node| node.name.clone()))
                })
                .unwrap_or_default()
        })
    });
    let count = move || {
        preview.with(|preview| {
            preview
                .as_ref()
                .map(|preview| format!("{} / {}", preview.index + 1, preview.frames.len()))
                .unwrap_or_default()
        })
    };
    let dimensions = RwSignal::new((0_u32, 0_u32));
    let stage_size = RwSignal::new((0.0_f64, 0.0_f64));
    let displayed = RwSignal::new(None::<String>);
    let generation = RwSignal::new(0_u64);
    let observation = StoredValue::new_local(None::<rustify_ui::ResizeObservation>);

    Effect::new(move || {
        let Some(stage) = stage.get() else {
            return;
        };
        let element: web_sys::Element = stage.into();
        let observed = element.clone();
        let update = move || {
            stage_size.set((
                f64::from(element.client_width()),
                f64::from(element.client_height()),
            ));
        };
        update();
        observation.set_value(rustify_ui::observe_resize(&observed, update));
    });
    on_cleanup(move || {
        observation.update_value(|observation| {
            observation.take();
        })
    });

    Effect::new(move || {
        let Some(target) = canvas.get() else {
            return;
        };
        let Some(id) = preview.with(|preview| {
            preview
                .as_ref()
                .and_then(PreviewState::current)
                .map(str::to_owned)
        }) else {
            return;
        };
        let Some(scale) = editor.doc.with(|doc| {
            doc.get(&id)
                .map(|node| (1000.0 / node.w.max(node.h)).clamp(1.0, 2.0))
        }) else {
            return;
        };
        let token = generation.get_untracked().wrapping_add(1);
        generation.set(token);
        wasm_bindgen_futures::spawn_local(async move {
            let result =
                crate::fileio::export_canvas(editor, std::slice::from_ref(&id), scale).await;
            // A closed preview disposes this generation signal. Late image loads must never
            // access its other signals or overwrite a newer frame's canvas.
            if generation.is_disposed() || generation.get_untracked() != token {
                return;
            }
            if !editor.shell.with_untracked(|shell| {
                shell.preview.as_ref().and_then(PreviewState::current) == Some(id.as_str())
            }) {
                return;
            }
            let result = result.and_then(|rendered| {
                let target: HtmlCanvasElement = target.unchecked_into();
                let context = target
                    .get_context("2d")?
                    .ok_or_else(|| JsValue::from_str("Preview canvas is unavailable."))?
                    .dyn_into::<CanvasRenderingContext2d>()?;
                target.set_width(rendered.width());
                target.set_height(rendered.height());
                context.draw_image_with_html_canvas_element(&rendered, 0.0, 0.0)?;
                dimensions.set((rendered.width(), rendered.height()));
                displayed.set(Some(id));
                Ok(())
            });
            super::report(editor, result);
        });
    });

    let size = Memo::new(move |_| {
        let (width, height) = dimensions.get();
        let (stage_width, stage_height) = stage_size.get();
        if width == 0 || height == 0 {
            return (0.0, 0.0);
        }
        let width = f64::from(width);
        let height = f64::from(height);
        let scale = ((stage_width - 72.0) / width)
            .min((stage_height - 72.0) / height)
            .clamp(0.0, 1.0);
        (width * scale, height * scale)
    });
    view! {
        <Layer modal=true anchor=Signal::derive(|| Anchor::Centred)
            on_close=move || close(editor) class="vellum-modal-layer"
            labelled_by="presentation-title" test_id="vellum-presentation-layer">
            <div id="presentation" class="presentation" node_ref=root on:keydown=move |event| trap_tab(root,event)>
                <div class="presentation-toolbar">
                    <span>"vellum "<span class="muted">"/ Preview"</span></span>
                    <span id="presentation-title">{move || title.get()}</span>
                    <div>
                        <button id="prev-frame" class="icon-button" aria-label="Previous frame" on:click=move |_| step(editor,-1)>{icon("arrowLeft",18)}</button>
                        <span id="presentation-count">{count}</span>
                        <button id="next-frame" class="icon-button" aria-label="Next frame" on:click=move |_| step(editor,1)>{icon("arrowRight",18)}</button>
                        <button id="close-presentation" class="icon-button" aria-label="Close preview" on:click=move |_| close(editor)>{icon("close",18)}</button>
                    </div>
                </div>
                <div class="presentation-stage" id="presentation-stage" node_ref=stage>
                    <canvas id="presentation-canvas" node_ref=canvas aria-label="Frame preview"
                        style:width=move || format!("{}px",size.get().0)
                        style:height=move || format!("{}px",size.get().1)
                        style:aspect-ratio=move || {let (w,h)=dimensions.get();format!("{w} / {}",h.max(1))}
                        on:click=move |event| {
                            if let Some(canvas) = canvas.get_untracked() {
                                follow_link(editor,&canvas,displayed.get_untracked().as_deref(),event);
                            }
                        }/>
                </div>
            </div>
        </Layer>
    }
}

fn follow_link(
    editor: Editor,
    canvas: &HtmlCanvasElement,
    displayed: Option<&str>,
    event: MouseEvent,
) {
    let preview = editor.shell.with_untracked(|shell| shell.preview.clone());
    let Some(preview) = preview else {
        return;
    };
    let Some(id) = preview.current().filter(|id| Some(*id) == displayed) else {
        return;
    };
    let rect = canvas.get_bounding_client_rect();
    if rect.width() == 0.0 || rect.height() == 0.0 {
        return;
    }
    let frame = editor.frame.get_untracked();
    let Some(item) = frame.world(id) else {
        return;
    };
    let world = Point::new(
        item.bounds.x + (f64::from(event.client_x()) - rect.left()) / rect.width() * item.bounds.w,
        item.bounds.y + (f64::from(event.client_y()) - rect.top()) / rect.height() * item.bounds.h,
    );
    let options = hit::HitOptions {
        deep: true,
        zoom: editor.camera.get_untracked().zoom,
        ..Default::default()
    };
    let target = editor.measure.with_value(|measure| {
        let hit = hit::hit_test(&frame, world, options, measure)?;
        editor.doc.with_untracked(|doc| {
            std::iter::once(hit)
                .chain(doc.ancestors(hit))
                .find_map(|node| {
                    node.string("prototypeTarget")
                        .filter(|target| !target.is_empty())
                        .map(str::to_owned)
                })
        })
    });
    if let Some(index) =
        target.and_then(|target| preview.frames.iter().position(|id| id == &target))
    {
        editor.shell.update(|shell| {
            if let Some(preview) = shell.preview.as_mut() {
                preview.index = index;
            }
        });
    }
}

fn trap_tab(root: NodeRef<leptos::html::Div>, event: KeyboardEvent) {
    if event.key() != "Tab" {
        return;
    }
    let Some(root) = root.get_untracked() else {
        return;
    };
    let Ok(buttons) = root.query_selector_all("button:not([disabled])") else {
        return;
    };
    let first = buttons
        .item(0)
        .and_then(|node| node.dyn_into::<HtmlElement>().ok());
    let last = buttons
        .item(buttons.length().saturating_sub(1))
        .and_then(|node| node.dyn_into::<HtmlElement>().ok());
    let (Some(first), Some(last), Some(active)) = (first, last, document().active_element()) else {
        return;
    };
    let target = if event.shift_key() && active == *first.as_ref() {
        Some(last)
    } else if !event.shift_key() && active == *last.as_ref() {
        Some(first)
    } else {
        None
    };
    if let Some(target) = target {
        event.prevent_default();
        let _ = target.focus();
    }
}
