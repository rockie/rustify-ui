use leptos::prelude::*;
use wasm_bindgen::JsCast;

use super::dialogs::{self, DialogState};
use crate::app::Editor;

const COMMANDS: &[(&str, &str)] = &[
    ("New document", "newFile"),
    ("Open .vellum document", "openFile"),
    ("Save portable document", "saveFile"),
    ("Undo", "undo"),
    ("Redo", "redo"),
    ("Fit all to view", "fit"),
    ("Fit selection", "fitSelection"),
    ("Zoom to 100%", "actualSize"),
    ("Duplicate selection", "duplicate"),
    ("Group selection", "group"),
    ("Ungroup selection", "ungroup"),
    ("Frame selection", "frameSelection"),
    ("Create component", "component"),
    ("Bring to front", "front"),
    ("Send to back", "back"),
    ("Distribute horizontally", "distributeH"),
    ("Distribute vertically", "distributeV"),
    ("Place an image", "placeImage"),
    ("Export PNG", "exportPNG"),
    ("Export SVG", "exportSVG"),
    ("Present frames", "present"),
    ("Toggle light / dark theme", "theme"),
    ("Toggle canvas grid", "grid"),
    ("Toggle rulers", "rulers"),
    ("Toggle smart snapping", "snap"),
    ("Edit design tokens", "tokens"),
    ("Inspect CSS", "inspectCSS"),
    ("Add a page", "addPage"),
    ("Rendering settings", "settings"),
    ("Load a local font…", "loadFont"),
    ("Keyboard shortcuts", "help"),
    ("Create 5,000-shape stress test", "stressTest"),
    ("Restore Forma starter document", "resetStarter"),
];

pub fn open(editor: Editor) {
    dialogs::open(editor, DialogState::Palette);
}

#[component]
pub fn Palette(editor: Editor) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let selected = RwSignal::new(0usize);
    let scrolling =
        StoredValue::new_local(None::<crate::browser_frame::AnimationFrameRequestHandle>);
    on_cleanup(move || {
        scrolling.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.cancel();
            }
        })
    });
    let filtered = Memo::new(move |_| {
        let query = query.get().to_lowercase();
        COMMANDS
            .iter()
            .copied()
            .filter(|(label, _)| label.to_lowercase().contains(&query))
            .collect::<Vec<_>>()
    });
    view! {
        <h2 id="modal-title" class="vellum-visually-hidden">"Commands"</h2>
        <input class="command-search" id="command-search" placeholder="What would you like to do?" aria-label="Search commands" autocomplete="off"
            prop:value=move || query.get()
            on:input=move |event| { query.set(event_target_value(&event)); selected.set(0); }
            on:keydown=move |event: web_sys::KeyboardEvent| {
                match event.key().as_str() {
                    "ArrowDown" | "ArrowUp" => {
                        event.prevent_default();
                        let last = filtered.with_untracked(|items| items.len().saturating_sub(1));
                        selected.update(|index| *index = if event.key() == "ArrowDown" { (*index + 1).min(last) } else { index.saturating_sub(1) });
                        // The focused class follows the signal; scrolling waits for that DOM update.
                        scrolling.update_value(|handle| { if let Some(handle) = handle.take() { handle.cancel(); } });
                        if let Ok(handle) = crate::browser_frame::request_animation_frame_with_handle(move || {
                            if let Ok(Some(element)) = document().query_selector("#command-results .focused") {
                                if let Some(element) = element.dyn_ref::<web_sys::HtmlElement>() {
                                    let options = web_sys::ScrollIntoViewOptions::new();
                                    options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                                    element.scroll_into_view_with_scroll_into_view_options(&options);
                                }
                            }
                        }) { scrolling.set_value(Some(handle)); }
                    }
                    "Enter" => {
                        if let Some((_,action)) = filtered.with_untracked(|items| items.get(selected.get_untracked()).copied()) {
                            event.prevent_default(); dialogs::close(editor); super::dispatch(editor,action);
                        }
                    }
                    _ => {}
                }
            }/>
        <div class="command-results" id="command-results">
            {move || { let commands = filtered.get(); if commands.is_empty() {
                view! { <div class="empty-state">"No matching commands."</div> }.into_any()
            } else { commands.into_iter().enumerate().map(|(index,(label,action))| view! {
                <button class=move || if selected.get() == index { "menu-item focused" } else { "menu-item" } data-command=action
                    on:click=move |_| { dialogs::close(editor); super::dispatch(editor,action); }>
                    <span>{label}</span><span class="shortcut">"↵"</span>
                </button>
            }).collect_view().into_any() } }}
        </div>
        <p class="command-hint">"↑ ↓ to navigate "<span>" · "</span>" Enter to run "<span>" · "</span>" Esc to close"</p>
    }
}
