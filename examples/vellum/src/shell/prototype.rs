use leptos::prelude::*;

use super::fields::{SelectField, Selection};
use crate::app::Editor;
use crate::icons::icon;
use crate::shell;

#[component]
pub fn Prototype(editor: Editor, nodes: Selection) -> impl IntoView {
    let frames = Signal::derive(move || {
        editor.doc.with(|doc| {
            std::iter::once((String::new(), "No destination".into()))
                .chain(
                    doc.nodes()
                        .iter()
                        .filter(|node| {
                            node.kind == "frame"
                                && node.parent_id.as_deref().is_none_or(str::is_empty)
                        })
                        .map(|node| (node.id.clone(), node.name.clone())),
                )
                .collect()
        })
    });
    view! {
        <section class="inspector-section"><div class="section-heading"><span>"Prototype"</span></div>
            <p>"Connect a layer to a frame. In preview, clicking that layer navigates to the destination."</p>
        </section>
        <Show when=move || nodes.with(|nodes| !nodes.is_empty()) fallback=|| view! {
            <section class="inspector-section"><div class="section-heading"><span>"Select a layer"</span></div>
                <p>"Select a button, card, or other layer to add an interaction."</p>
            </section>
        }>
            <section class="inspector-section"><div class="section-heading"><span>"Interaction"</span></div>
                <div class="field-label">"On click → Navigate to"</div>
                <SelectField editor nodes prop="prototypeTarget" options=frames/>
                <p>"Transition: instant. Keyboard arrows also navigate between frames."</p>
            </section>
        </Show>
        <section class="inspector-section"><div class="section-heading"><span>"Flow preview"</span></div>
            <button class="wide-button primary" data-action="present" on:click=move |_| shell::dispatch(editor,"present")>{icon("play",14)}" Present frames"</button>
            <p>"Preview is local. It does not publish or upload your design."</p>
        </section>
    }
}
