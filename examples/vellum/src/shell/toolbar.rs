use crate::{app::Editor, icons::icon};
use leptos::prelude::*;

#[component]
pub fn Toolbar(editor: Editor) -> impl IntoView {
    view! {<div class="floating-toolbar" id="toolbar" role="toolbar" aria-label="Design tools">{
        [("select","Move (V)","cursor"),("frame","Frame (F)","frame"),("rect","Rectangle (R)","rect"),("ellipse","Ellipse (O)","ellipse"),("line","Line (L)","line"),("pen","Pen (P) · click points, Enter to finish","pen"),("text","Text (T)","text"),("hand","Hand (H)","hand")].into_iter().map(|(tool,title,name)|view!{
            {matches!(tool,"rect"|"hand").then(||view!{<div class="toolbar-divider"></div>})}
            <button class="tool" class:active=move||editor.tool.get()==tool data-tool=tool title=title aria-label=title on:click=move |_|crate::pointer::set_tool(editor,tool)>{icon(name,18)}</button>
        }).collect_view()
    }<button class="tool" id="insert-image" title="Place image (⇧⌘K)" aria-label="Place image" on:click=move |_|super::dispatch(editor,"placeImage")>{icon("image",18)}</button><div class="toolbar-divider"></div><button class="tool accent-tool" id="command-button" title="Commands (⌘K)" aria-label="Command palette" on:click=move |_|super::palette::open(editor)>{icon("sparkles",18)}</button></div>}
}
