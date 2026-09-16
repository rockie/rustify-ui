use crate::browser_frame::{request_animation_frame_with_handle, AnimationFrameRequestHandle};
use leptos::prelude::*;
use rustify_ui::{Anchor, Layer};
use serde_json::json;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use crate::{
    app::Editor,
    document::{Node, Page},
    icons::icon,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PromptKind {
    RenameLayer(String),
    RenameFile,
    AddPage,
    RenamePage(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialogState {
    Prompt {
        kind: PromptKind,
        title: String,
        value: String,
    },
    NewFile,
    Help,
    Settings,
    Export,
    Tokens,
    Css {
        id: String,
    },
    Palette,
}

pub fn open(editor: Editor, dialog: DialogState) {
    super::menus::close(editor);
    editor.shell.update(|shell| shell.dialog = Some(dialog));
    editor.input.update(|input| input.modal_open = true);
}

pub fn close(editor: Editor) {
    editor.shell.update(|shell| shell.dialog = None);
    editor.input.update(|input| input.modal_open = false);
}

pub fn prompt(editor: Editor, kind: PromptKind) {
    let contents = editor.doc.with_untracked(|doc| match &kind {
        PromptKind::RenameLayer(id) => doc.get(id).map(|node| ("Rename layer", node.name.clone())),
        PromptKind::RenameFile => Some(("Name your design file", doc.data.name.clone())),
        PromptKind::AddPage => Some(("Add a page", "Untitled page".into())),
        PromptKind::RenamePage(id) => doc
            .data
            .pages
            .iter()
            .find(|page| &page.id == id)
            .map(|page| ("Rename page", page.name.clone())),
    });
    if let Some((title, value)) = contents {
        open(
            editor,
            DialogState::Prompt {
                kind,
                title: title.into(),
                value,
            },
        );
    }
}

#[component]
pub fn Dialogs(editor: Editor) -> impl IntoView {
    let dialog = Memo::new(move |_| editor.shell.with(|shell| shell.dialog.clone()));
    view! { {move || dialog.get().map(|state| view! { <Dialog editor state/> })} }
}

#[component]
fn Dialog(editor: Editor, state: DialogState) -> impl IntoView {
    let state = StoredValue::new(state);
    let node = NodeRef::<leptos::html::Section>::new();
    view! {
        <Layer modal=true anchor=Signal::derive(|| Anchor::Centred) on_close=move || close(editor)
            class="vellum-modal-layer" labelled_by="modal-title" test_id="vellum-dialog-layer">
            <div id="modal-backdrop" class="modal-backdrop" on:click=move |event| {
                if event.target() == event.current_target() { close(editor); }
            }>
                <section id="modal" class="modal" node_ref=node on:keydown=move |event| trap_tab(node,event)>
                    {body(editor,state.get_value())}
                </section>
            </div>
        </Layer>
    }
}

fn body(editor: Editor, state: DialogState) -> AnyView {
    match state {
        DialogState::Prompt { kind, title, value } => {
            view! { <Prompt editor kind title value/> }.into_any()
        }
        DialogState::NewFile => view! { <NewFile editor/> }.into_any(),
        DialogState::Help => view! { <Help editor/> }.into_any(),
        DialogState::Settings => view! { <Settings editor/> }.into_any(),
        DialogState::Export => view! { <Export editor/> }.into_any(),
        DialogState::Tokens => view! { <Tokens editor/> }.into_any(),
        DialogState::Css { id } => view! { <Css editor id/> }.into_any(),
        DialogState::Palette => view! { <super::palette::Palette editor/> }.into_any(),
    }
}

#[component]
fn Header(editor: Editor, title: String, #[prop(optional)] subtitle: String) -> impl IntoView {
    view! { <div class="modal-header"><div><h2 id="modal-title">{title}</h2>
    {(!subtitle.is_empty()).then(||view! { <p class="modal-subtitle">{subtitle}</p> })}
    </div><button class="icon-button" data-close-modal="" title="Close dialog" aria-label="Close dialog" on:click=move |_|close(editor)>{icon("close",18)}</button></div> }
}

#[component]
fn Prompt(editor: Editor, kind: PromptKind, title: String, value: String) -> impl IntoView {
    let value = RwSignal::new(value);
    let node = NodeRef::<leptos::html::Input>::new();
    let focus = StoredValue::new_local(None::<AnimationFrameRequestHandle>);
    node.on_load(move |input| {
        if let Ok(handle) = request_animation_frame_with_handle(move || {
            let _ = input.focus();
            input.select();
        }) {
            focus.set_value(Some(handle));
        }
    });
    on_cleanup(move || {
        focus.update_value(|focus| {
            if let Some(focus) = focus.take() {
                focus.cancel();
            }
        })
    });
    let label = title.clone();
    view! {
        <Header editor title/>
        <form id="prompt-form" on:submit=move |event| {
            event.prevent_default();
            let value = value.get_untracked().trim().to_owned();
            if value.is_empty() { return; }
            close(editor);
            let add_page = matches!(kind,PromptKind::AddPage);
            let result = editor.edit(match kind { PromptKind::RenameLayer(_) => "Rename layer", PromptKind::RenameFile => "Rename document", PromptKind::AddPage => "Add page", PromptKind::RenamePage(_) => "Rename page" }, |doc| {
                match &kind {
                    PromptKind::RenameLayer(id) => { if let Some(node) = doc.get_mut(id) { node.name = value.clone(); doc.touch(Some(id)); } }
                    PromptKind::RenameFile => doc.data.name = value.clone(),
                    PromptKind::RenamePage(id) => { if let Some(page) = doc.data.pages.iter_mut().find(|page| &page.id == id) { page.name = value.clone(); } }
                    PromptKind::AddPage => { let page = Page::new(&value); doc.data.page_id = page.id.clone(); doc.data.pages.push(page); doc.refresh(); }
                }
                Ok(())
            });
            if let Err(error) = result { super::toast(editor,&format!("Could not save name: {error:?}")); }
            else if add_page { editor.select(Vec::new()); editor.fit(None); }
        }>
            <input class="form-input" id="prompt-value" aria-label=label required maxlength="200" autocomplete="off" node_ref=node
                prop:value=move || value.get() on:input=move |event| value.set(event_target_value(&event))/>
            <div class="modal-actions"><button type="button" class="wide-button" data-close-modal="" on:click=move |_| close(editor)>"Cancel"</button>
                <button class="wide-button primary" type="submit">"Save"</button></div>
        </form>
    }
}

#[component]
fn NewFile(editor: Editor) -> impl IntoView {
    view! {
        <Header editor title="Start with a clean canvas.".into()/>
        <p>"Your current document will remain available in Undo. Export a .vellum copy to keep it as a separate file."</p>
        <div class="modal-actions"><button class="wide-button" data-close-modal="" on:click=move |_|close(editor)>"Cancel"</button>
            <button class="wide-button" id="backup-new" on:click=move |_|super::dispatch(editor,"saveFile")>"Export current file"</button>
            <button class="wide-button primary" id="confirm-new" on:click=move |_| { close(editor); super::dispatch(editor,"confirmNew"); }>"New document"</button>
        </div>
    }
}

#[component]
fn Help(editor: Editor) -> impl IntoView {
    let shortcuts = |items: &[(&str, &str)]| {
        items
            .iter()
            .map(
                |(label, key)| view! { <div>{label.to_string()}<kbd>{key.to_string()}</kbd></div> },
            )
            .collect_view()
    };
    view! {
        <Header editor title="A few keys. Endless possibilities.".into() subtitle="Your Vellum field guide.".into()/>
        <h3>"Tools"</h3><div class="shortcut-grid">{shortcuts(&[("Move","V"),("Frame","F"),("Rectangle","R"),("Ellipse","O"),("Line","L"),("Pen","P"),("Text","T"),("Hand","H")])}</div>
        <h3>"Canvas"</h3><div class="shortcut-grid">{shortcuts(&[("Pan","Space + drag"),("Zoom","⌘/Ctrl + scroll"),("Fit all","Shift + 1"),("Fit selection","Shift + 2"),("Actual size","Shift + 0"),("Hide panels","Tab"),("Draw square / circle","Shift + drag"),("Disable snapping","⌘/Ctrl + drag")])}</div>
        <h3>"Editing"</h3><div class="shortcut-grid">{shortcuts(&[("Undo","⌘Z"),("Redo","⇧⌘Z"),("Duplicate","⌘D"),("Group","⌘G"),("Ungroup","⇧⌘G"),("Commands","⌘K"),("Edit text / path","Double-click"),("Finish path","Enter"),("Nudge","Arrow keys"),("Nudge 10px","Shift + arrows"),("Save file","⌘S"),("Place image","⇧⌘K")])}</div>
        <p>"On Windows and Linux, use Ctrl in place of ⌘. Pen: drag an anchor while drawing to create Bézier handles. Alt-drag a handle to break tangent symmetry."</p>
    }
}

#[component]
fn Settings(editor: Editor) -> impl IntoView {
    let statistics = editor.snapshot();
    let renderer = &statistics["renderer"];
    let measured = format!("Backend: {}\nVisible layers: {}\nInstances: {}\nScene draw calls: {}\nCPU submission: {:.2} ms\nDevice pixel ratio: {}\nStorage: {}{}",
        renderer["backend"].as_str().unwrap_or("Makepad WebGL2"), renderer["visibleCount"],renderer["instanceCount"],renderer["drawCalls"],
        renderer["cpuMs"].as_f64().unwrap_or_default(),renderer["dpr"],editor.storage_mode.get_untracked(),
        renderer["gpuError"].as_str().map(|error|format!("\nGPU status: {error}")).unwrap_or_default());
    view! {
        <Header editor title="A workspace that feels like yours.".into()/>
        <div class="stack">
            <label class="checkbox-row"><input type="checkbox" id="settings-theme" prop:checked=move ||!editor.dark.get() on:change=move |_|super::dispatch(editor,"theme")/>"Light appearance"</label>
            <label class="checkbox-row"><input type="checkbox" data-option="grid" prop:checked=move ||editor.grid.get() on:change=move |_|super::dispatch(editor,"grid")/>"Canvas dot grid"</label>
            <label class="checkbox-row"><input type="checkbox" data-option="snap" prop:checked=move ||editor.snap.get() on:change=move |_|super::dispatch(editor,"snap")/>"Smart alignment guides"</label>
            <label class="checkbox-row"><input type="checkbox" data-option="rulers" prop:checked=move ||editor.rulers.get() on:change=move |_|super::dispatch(editor,"rulers")/>"Canvas rulers"</label>
        </div>
        <h3>"Rendering"</h3><pre class="token-code">{measured}</pre>
        <p>"Vellum draws on demand. The displayed time measures CPU scene assembly and command submission, not GPU execution or FPS. WebGPU requires a compatible browser and secure context."</p>
    }
}

#[component]
fn Export(editor: Editor) -> impl IntoView {
    let target = if editor
        .selection
        .with_untracked(|selection| selection.is_empty())
    {
        "current page"
    } else {
        "selection"
    };
    view! {
        <Header editor title="Take your work with you.".into() subtitle="Portable by design. No account required.".into()/>
        <button class="wide-button primary modal-save-file" id="modal-save-file" on:click=move |_|super::dispatch(editor,"saveFile")>{icon("download",18)}" Download .vellum document"</button>
        <p>"Includes every page, editable layer, component, design token, and placed image."</p>
        <h3>{format!("Export {target}")}</h3><div class="property-grid">
            <button class="wide-button" id="modal-png" on:click=move |_|super::dispatch(editor,"exportPNG")>"PNG image · 2×"</button>
            <button class="wide-button" id="modal-svg" on:click=move |_|super::dispatch(editor,"exportSVG")>"SVG vector"</button>
        </div><h3>"Already have a Vellum file?"</h3>
        <button class="wide-button" id="modal-open-file" on:click=move |_| { super::dispatch(editor,"openFile"); close(editor); }>{icon("upload",18)}" Open document"</button>
        <p>"Vellum files are JSON. This editor does not read or write Figma’s proprietary .fig format."</p>
    }
}

#[component]
fn Tokens(editor: Editor) -> impl IntoView {
    let rows = editor.doc.with_untracked(|doc| {
        doc.data.tokens["colors"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|token| {
                let name = token["name"].as_str().unwrap_or_default().to_owned();
                let value = token["value"].as_str().unwrap_or("#8462e8").to_owned();
                (name.clone(), RwSignal::new(name), RwSignal::new(value))
            })
            .collect::<Vec<_>>()
    });
    view! {
        <Header editor title="Design tokens".into() subtitle="Your shared visual foundation, stored in this file.".into()/>
        <div id="token-editor">{rows.iter().cloned().enumerate().map(|(index,(_,name,color))| view! {
            <div class="token-edit-row"><input type="color" aria-label=format!("Token {} color",index+1) data-token-color=index class="token-edit-color"
                prop:value=move ||color.get() on:input=move |event|color.set(event_target_value(&event))/>
                <input class="form-input" aria-label=format!("Token {} name",index+1) data-token-name=index
                    prop:value=move ||name.get() on:input=move |event|name.set(event_target_value(&event))/></div>
        }).collect_view()}</div>
        <p>"Changing a color remaps exact matching fills and strokes throughout the document."</p>
        <div class="modal-actions"><button class="wide-button" id="export-tokens" on:click=move |_|super::dispatch(editor,"exportTokens")>"Export JSON"</button>
            <button class="wide-button primary" id="save-tokens" on:click=move |_| {
                let next = rows.iter().map(|(original,name,color)| {
                    let name = name.get_untracked();
                    json!({"name":if name.is_empty() { original.clone() } else { name },"value":color.get_untracked()})
                }).collect::<Vec<_>>();
                close(editor);
                match editor.edit("Update design tokens",|doc| { crate::commands::remap_color_tokens(doc,json!(next)); Ok(()) }) {
                    Ok(()) => super::toast(editor,"Tokens updated across all pages"),
                    Err(error) => super::toast(editor,&format!("Could not update tokens: {error:?}")),
                }
            }>"Apply tokens"</button></div>
    }
}

#[component]
fn Css(editor: Editor, id: String) -> impl IntoView {
    let node = editor.doc.with_untracked(|doc| doc.get(&id).cloned());
    let name = node
        .as_ref()
        .map(|node| node.name.clone())
        .unwrap_or_default();
    let css = node.as_ref().map(inspect_css).unwrap_or_default();
    let copy = css.clone();
    view! {
        <Header editor title="Inspect CSS".into() subtitle=name/>
        <pre class="token-code">{css}</pre>
        <p>"Geometry and visual styles. Vector paths and text shaping remain renderer-specific."</p>
        <div class="modal-actions"><button class="wide-button primary" id="copy-css" on:click=move |_| {
            rustify_ui::clipboard::copy(&copy,move |result| {
                if !editor.shell.is_disposed() { super::toast(editor,if result.is_ok() { "CSS copied" } else { "Clipboard blocked. Select the code to copy it." }); }
            });
        }>"Copy CSS"</button></div>
    }
}

fn fmt(value: f64) -> String {
    ((value * 100.0 + 0.5).floor() / 100.0).to_string()
}

pub fn inspect_css(node: &Node) -> String {
    let mut css = format!(
        "/* {} */\nposition: absolute;\nleft: {}px;\ntop: {}px;\nwidth: {}px;\nheight: {}px;",
        node.name.replace("*/", ""),
        fmt(node.x),
        fmt(node.y),
        fmt(node.w),
        fmt(node.h)
    );
    if node.rotation != 0.0 {
        css.push_str(&format!("\ntransform: rotate({}deg);", fmt(node.rotation)));
    }
    if node.kind == "text" {
        css.push_str(&format!("\nfont-family: {}, sans-serif;\nfont-size: {}px;\nfont-weight: {};\nline-height: {};\nletter-spacing: {}px;\ntext-align: {};\ncolor: {};",node.font_family,fmt(node.font_size),node.font_weight,fmt(node.line_height),fmt(node.letter_spacing),node.text_align,node.fill));
    } else {
        let fill = if node.fill_type == "linear" {
            format!(
                "linear-gradient({}deg, {}, {})",
                node.gradient_angle + 90.0,
                node.fill,
                node.fill2
            )
        } else if node.fill == "none" {
            "transparent".into()
        } else {
            node.fill.clone()
        };
        css.push_str(&format!("\nbackground: {fill};"));
        if node.radius != 0.0 {
            css.push_str(&format!("\nborder-radius: {}px;", fmt(node.radius)));
        }
        if node.kind == "ellipse" {
            css.push_str("\nborder-radius: 50%;");
        }
        if node.stroke_width != 0.0 {
            css.push_str(&format!(
                "\nborder: {}px solid {};",
                fmt(node.stroke_width),
                node.stroke
            ));
        }
    }
    if node.opacity != 1.0 {
        css.push_str(&format!("\nopacity: {};", node.opacity));
    }
    if node.shadow {
        css.push_str(&format!(
            "\nbox-shadow: {}px {}px {}px {}{:02x};",
            node.shadow_x,
            node.shadow_y,
            node.shadow_blur,
            node.shadow_color,
            (node.shadow_opacity * 255.0 + 0.5).floor() as u32
        ));
    }
    if let Some(layout) = node
        .string("layout")
        .filter(|value| !value.is_empty() && *value != "none")
    {
        css.push_str(&format!(
            "\ndisplay: flex;\nflex-direction: {};\ngap: {}px;\npadding: {}px;",
            if layout == "horizontal" {
                "row"
            } else {
                "column"
            },
            node.number("gap", 16.0),
            node.number("padding", 16.0)
        ));
    }
    css
}

fn trap_tab(node: NodeRef<leptos::html::Section>, event: KeyboardEvent) {
    if event.key() != "Tab" {
        return;
    }
    let Some(node) = node.get() else {
        return;
    };
    let Ok(nodes) = node.query_selector_all("button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex='-1'])") else { return; };
    let items: Vec<_> = (0..nodes.length())
        .filter_map(|index| nodes.item(index)?.dyn_into::<HtmlElement>().ok())
        .collect();
    let (Some(first), Some(last)) = (items.first(), items.last()) else {
        return;
    };
    if let Some(active) = document().active_element() {
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
}
