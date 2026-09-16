use crate::{app::Editor, icons::icon};
use leptos::prelude::*;

#[component]
pub fn Pages(editor: Editor) -> impl IntoView {
    view! {
        <div class="pages-heading"><span>"Pages"</span><button class="icon-button small" id="add-page" title="Add page" aria-label="Add page" on:click=move |_|super::dispatch(editor,"addPage")>{icon("plus",18)}</button></div>
        <div id="page-list" class="page-list">{move||editor.doc.with(|doc|doc.data.pages.iter().map(|page|{
            let id=page.id.clone();let rename=id.clone();let active=page.id==doc.data.page_id;
            view!{<button class=if active{"page-row active"}else{"page-row"} data-page=id.clone() on:click=move |_|editor.switch_page(&id) on:dblclick=move |_|super::dialogs::prompt(editor,super::dialogs::PromptKind::RenamePage(rename.clone()))>{icon("page",13)}<span>{page.name.clone()}</span>{active.then(||view!{<span class="page-check">"✓"</span>})}</button>}
        }).collect_view())}</div>
    }
}
