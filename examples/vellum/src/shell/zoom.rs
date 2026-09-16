use crate::{affine::Point, app::Editor, icons::icon};
use leptos::prelude::*;

#[component]
pub fn Zoom(editor: Editor) -> impl IntoView {
    view! {<div class="zoom-controls"><button id="zoom-out" class="icon-button small" title="Zoom out" aria-label="Zoom out" on:click=move |_|editor.zoom_at(0.8,None,None)>{icon("minus",18)}</button><button id="zoom-value" title="Zoom options" on:click=move|event|super::menus::open_zoom(editor,Point::new(event.client_x()as f64,event.client_y()as f64))>{move||format!("{}%",(editor.camera.get().zoom*100.).round()as u32)}</button><button id="zoom-in" class="icon-button small" title="Zoom in" aria-label="Zoom in" on:click=move |_|editor.zoom_at(1.25,None,None)>{icon("plus",18)}</button></div>}
}
