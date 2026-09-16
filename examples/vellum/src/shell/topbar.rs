use crate::{app::Editor, icons::icon};
use leptos::prelude::*;

#[component]
pub fn Topbar(editor: Editor) -> impl IntoView {
    view! {<header class="topbar">
        <div class="brand-section"><button class="brand" id="main-menu" aria-label="Vellum menu" title="Vellum menu" on:click=move |_|super::menus::open_main(editor)><svg width="23" height="23" viewBox="0 0 32 32"><path d="M4 7h7l5 11 5-11h7L17 28h-3Z" fill="currentColor"/></svg><span>"vellum"</span><span class="brand-chevron">"⌄"</span></button><div class="top-divider"></div><button class="icon-button" id="toggle-left" title="Toggle layers panel" aria-label="Toggle layers panel" on:click=move |_|editor.shell.update(|s|s.left_hidden=!s.left_hidden)>{icon("panel",18)}</button></div>
        <div class="file-breadcrumb"><span class="muted">"Workspace"</span><span class="crumb-slash">"/"</span><button id="file-name" title="Rename document" on:click=move |_|super::dispatch(editor,"renameFile")>{move||editor.doc.with(|doc|doc.data.name.clone())}</button><span class="file-chevron">"⌄"</span><span class="save-indicator" id="save-indicator"><i style:background=move||if editor.storage_status.get()==crate::storage::SaveStatus::Failed{"var(--danger)"}else{""}></i>{move||format!(" {}",editor.storage_status.get().label())}</span></div>
        <div class="top-actions"><span class="local-badge">"LOCAL FILE"</span><button class="avatar" id="profile" title="Local-first · no account required" on:click=move |_|super::toast(editor,"Your local workspace. No account, presence simulation, or cloud upload.")>"Y"</button><button class="icon-button" id="present" title="Present frames" aria-label="Present frames" on:click=move |_|super::dispatch(editor,"present")>{icon("play",18)}</button><button class="share-button" id="share" title="Save a portable Vellum file" on:click=move |_|super::dispatch(editor,"export")>"Export "{icon("arrowUpRight",18)}</button></div>
    </header>}
}
