//! The inspector keeps a stable view tree while selected properties change.

use leptos::prelude::*;
use serde_json::{json, Value};

use super::fields::{
    choices, formatted, number, safe_color, selected_nodes, text, ActionButton, ColorField,
    NumberField, SelectField, Selection,
};
use super::prototype::Prototype;
use crate::app::Editor;
use crate::icons::icon;
use crate::shell;

#[component]
pub fn Inspector(editor: Editor) -> impl IntoView {
    let nodes = selected_nodes(editor);
    view! {
        <aside class="right-panel" id="right-panel" aria-label="Properties inspector">
            <div class="inspector-tabs">
                <button data-inspector-tab="design" class:active=move || editor.shell.with(|state| state.inspector_tab == "design")
                    on:click=move |_| editor.shell.update(|state| state.inspector_tab = "design".into())>"Design"</button>
                <button data-inspector-tab="prototype" class:active=move || editor.shell.with(|state| state.inspector_tab == "prototype")
                    on:click=move |_| editor.shell.update(|state| state.inspector_tab = "prototype".into())>"Prototype"</button>
                <button class="icon-button small" id="theme-toggle" title="Toggle light / dark theme" aria-label="Toggle theme"
                    on:click=move |_| shell::dispatch(editor, "theme")>{move || icon(if editor.dark.get() {"sun"} else {"moon"}, 18)}</button>
            </div>
            <div class="inspector-scroll" id="inspector">
                <Show when=move || editor.shell.with(|state| state.inspector_tab == "prototype")
                    fallback=move || view! {
                        <Show when=move || nodes.with(|nodes| !nodes.is_empty()) fallback=move || view! {<PageInspector editor/>}>
                            <NodeInspector editor nodes/>
                        </Show>
                    }>
                    <Prototype editor nodes/>
                </Show>
            </div>
            <div class="inspector-footer"><span>"Made for your next big idea."</span>
                <button class="icon-button small" id="help" title="Keyboard shortcuts" aria-label="Keyboard shortcuts" on:click=move |_| shell::dispatch(editor, "help")>{icon("help", 18)}</button>
            </div>
        </aside>
    }
}

fn tokens(editor: Editor, kind: &str) -> Vec<(usize, Value)> {
    editor.doc.with(|doc| {
        doc.data.tokens[kind]
            .as_array()
            .into_iter()
            .flatten()
            .cloned()
            .enumerate()
            .collect()
    })
}

fn token_string(token: &Value, key: &str) -> String {
    token[key].as_str().unwrap_or("").to_owned()
}

fn report(editor: Editor, result: Result<Value, wasm_bindgen::JsValue>) {
    if let Err(error) = result {
        shell::toast(
            editor,
            error.as_string().unwrap_or_else(|| format!("{error:?}")),
        );
    }
}

#[component]
fn PageInspector(editor: Editor) -> impl IntoView {
    let canvas_color = move || {
        editor
            .shell
            .with(|state| state.canvas_color.clone())
            .unwrap_or_else(|| {
                if editor.dark.get() {
                    "#1a1a1d".into()
                } else {
                    "#e8e7ec".into()
                }
            })
    };
    let canvas_hex = move || {
        editor
            .shell
            .with(|state| state.canvas_color.clone())
            .unwrap_or_else(|| {
                if editor.dark.get() {
                    "1A1A1D".into()
                } else {
                    "E8E7EC".into()
                }
            })
    };
    view! {
        <section class="inspector-section">
            <div class="section-heading"><span>"Page"</span>{icon("sliders", 14)}</div>
            <div class="fill-row"><div class="hex-field">
                <input type="color" id="canvas-color" prop:value=canvas_color aria-label="Canvas color"
                    on:change=move |event| editor.shell.update(|state| state.canvas_color = Some(event_target_value(&event)))/>
                <input type="text" readonly aria-label="Canvas hex" prop:value=canvas_hex/>
            </div><span class="small-label">"100%"</span></div>
            <label class="checkbox-row"><input type="checkbox" data-option="grid" prop:checked=move || editor.grid.get()
                on:change=move |event| editor.grid.set(event_target_checked(&event))/>"Pixel grid"</label>
        </section>
        <section class="inspector-section">
            <div class="section-heading"><span>"Start with a frame"</span></div>
            <div class="property-grid">{[("desktop", "Desktop"), ("phone", "Phone"), ("tablet", "Tablet"), ("square", "Social")].into_iter().map(|(preset, label)| view! {
                <button class="wide-button" data-preset=preset on:click=move |_| shell::preset(editor, preset)>{label}</button>
            }).collect_view()}</div>
        </section>
        <section class="inspector-section">
            <div class="section-heading"><span>"Local color styles"</span><ActionButton editor action="tokens" title="Edit design tokens" symbol="sliders"/></div>
            <For each=move || tokens(editor, "colors") key=|(index, token)| (*index, token.to_string()) children=move |(_, token)| {
                let color = safe_color(&token_string(&token, "value"));
                let background = color.clone();
                let value = token_string(&token, "value").to_uppercase();
                let name = token_string(&token, "name");
                let attribute = color.clone();
                view! {<button class="style-row full-width" data-insert-color=attribute on:click=move |_| report(editor, editor.create_at_center("rect", json!({"w":120,"h":120,"radius":14,"fill":color,"name":"Color swatch"})))>
                    <span class="style-swatch" style:background=background></span><span class="style-info inspector-text-left">{name}<small>{value}</small></span>{icon("component", 12)}
                </button>}
            }/>
        </section>
        <section class="inspector-section">
            <div class="section-heading"><span>"Text styles"</span></div>
            <For each=move || tokens(editor, "typography") key=|(index, token)| (*index, token.to_string()) children=move |(index, token)| {
                let name = token_string(&token, "name");
                let description = format!("Inter · {} / {} · {}", token["size"], formatted(token["size"].as_f64().unwrap_or(0.0) * token["lineHeight"].as_f64().unwrap_or(1.0), 10.0), token["weight"]);
                view! {<button class="typography-style full-width" data-type-style=index on:click=move |_| report(editor, editor.create_at_center("text", json!({"text":"Make something meaningful.","name":token["name"],"fontSize":token["size"],"fontWeight":token["weight"],"lineHeight":token["lineHeight"],"w":520,"h":token["size"].as_f64().unwrap_or(0.0)*2.0,"fill":if editor.dark.get_untracked(){"#d2bfeb"}else{"#755890"}})))>
                    <span class="type-icon">"Aa"</span><div class="inspector-text-left">{name}<small>{description}</small></div>
                </button>}
            }/>
        </section>
        <section class="inspector-section">
            <div class="section-heading"><span>"Your work, your device"</span></div>
            <p>"No account. No uploads. This document is saved locally in your browser."</p>
            <button class="wide-button inspector-mt13" data-action="saveFile" on:click=move |_| shell::dispatch(editor, "saveFile")>{icon("download", 13)}" Save a portable copy"</button>
        </section>
    }
}

#[component]
fn NodeInspector(editor: Editor, nodes: Selection) -> impl IntoView {
    view! {
        <section class="inspector-section">
            <div class="section-heading"><span><span class="section-title">
                {move || nodes.with(|nodes| icon(if nodes.len() > 1 {"layers"} else if let Some(node) = nodes.first() {if node.flag("component") {"component"} else if node.flag("isInstance") {"instance"} else {&node.kind}} else {"rect"}, 14))}
                <span class="selection-name">{move || nodes.with(|nodes| if nodes.len() > 1 {format!("{} layers selected", nodes.len())} else {nodes.first().map_or_else(String::new, |node| node.name.clone())})}</span>
            </span></span><ActionButton editor action="selectionMenu" title="Layer actions"/></div>
            <div class="selection-meta">{move || nodes.with(|nodes| {
                if nodes.len() > 1 {"Edit shared properties".into()}
                else {nodes.first().map_or_else(String::new, |node| if node.flag("isInstance") {"Component instance".into()} else if node.flag("component") {"Main component".into()} else {
                    let mut kind = node.kind.chars();
                    format!("{}{} · {} × {}", kind.next().unwrap_or(' ').to_uppercase(), kind.as_str(), formatted(node.w, 1.0), formatted(node.h, 1.0))
                })}
            })}</div>
        </section>
        <section class="inspector-section">
            <div class="section-heading"><span>"Position"</span></div>
            <div class="align-buttons">{[("alignLeft","left"),("alignCenter","center"),("alignRight","right"),("alignTop","top"),("alignMiddle","middle"),("alignBottom","bottom")].into_iter().map(|(symbol, align)| view! {
                <button data-align=align title=format!("Align {align}") aria-label=format!("Align {align}") on:click=move |_| shell::align(editor, align)>{icon(symbol, 15)}</button>
            }).collect_view()}</div>
            <div class="property-grid"><NumberField editor nodes prop="x" label="X"/><NumberField editor nodes prop="y" label="Y"/>
                <NumberField editor nodes prop="rotation" symbol="rotate" unit="°"/><NumberField editor nodes prop="radius" symbol="radius" min=0.0/>
            </div>
        </section>
        <section class="inspector-section">
            <div class="section-heading"><span>"Layout"</span><ActionButton editor action="toggleLayout" title="Toggle auto layout" symbol="plus"/></div>
            <div class="property-grid"><NumberField editor nodes prop="w" label="W" min=0.1/><NumberField editor nodes prop="h" label="H" min=0.1/></div>
            <Show when=move || nodes.with(|nodes| nodes.first().is_some_and(|node| matches!(node.kind.as_str(), "frame" | "group")))>
                <label class="checkbox-row"><input type="checkbox" data-prop="clip" prop:checked=move || nodes.with(|nodes| nodes.first().is_some_and(|node| node.clip))
                    disabled=move || nodes.with(|nodes| nodes.first().is_some_and(|node| node.kind == "group"))
                    on:change=move |event| shell::set_property(editor, "clip", json!(event_target_checked(&event)), false)/>"Clip content"</label>
                <div class="field-label"><span>"Auto layout"</span><span>"Gap / Padding"</span></div>
                <SelectField editor nodes prop="layout" options=choices(&[("none","Freeform"),("horizontal","Horizontal stack"),("vertical","Vertical stack")])/>
                <Show when=move || text(nodes, "layout") != "none">
                    <div class="property-grid inspector-mt8"><NumberField editor nodes prop="gap" label="↔" min=0.0/><NumberField editor nodes prop="padding" label="⊞" min=0.0/></div>
                    <div class="inspector-mt8"><SelectField editor nodes prop="layoutAlign" options=choices(&[("start","Align start"),("center","Align center"),("end","Align end")])/></div>
                </Show>
            </Show>
        </section>
        <section class="inspector-section">
            <div class="section-heading"><span>"Appearance"</span></div>
            <div class="property-grid"><NumberField editor nodes prop="opacity" symbol="opacity" unit="%" min=0.0 max=100.0/>
                <div class="segmented">
                    <button data-toggle="visible" class:active=move || nodes.with(|nodes| nodes.first().is_some_and(|node| node.visible)) title="Toggle visibility" aria-label="Toggle visibility" on:click=move |_| shell::toggle_property(editor,"visible")>{icon("eye",14)}</button>
                    <button data-toggle="locked" class:active=move || nodes.with(|nodes| nodes.first().is_some_and(|node| node.locked)) title="Toggle lock" aria-label="Toggle lock" on:click=move |_| shell::toggle_property(editor,"locked")>{icon("lock",14)}</button>
                </div>
            </div>
        </section>
        <Show when=move || nodes.with(|nodes| !nodes.is_empty() && nodes.iter().all(|node| node.kind == "text"))><Typography editor nodes/></Show>
        <Show when=move || nodes.with(|nodes| nodes.first().is_some_and(|node| node.kind != "group"))><Fill editor nodes/></Show>
        <Show when=move || nodes.with(|nodes| nodes.first().is_some_and(|node| !matches!(node.kind.as_str(),"group"|"text")))><Stroke editor nodes/></Show>
        <Show when=move || nodes.with(|nodes| nodes.first().is_some_and(|node| !matches!(node.kind.as_str(),"group"|"text"|"path"|"line")))><Effects editor nodes/></Show>
        <Show when=move || nodes.with(|nodes| nodes.first().is_some_and(|node| node.parent_id.as_deref().is_some_and(|id| !id.is_empty())))>
            <section class="inspector-section"><div class="section-heading"><span>"Constraints"</span></div>
                <div class="property-grid"><SelectField editor nodes prop="constraintH" options=choices(&[("left","Left"),("right","Right"),("center","Center"),("stretch","Left + right"),("scale","Scale")])/>
                    <SelectField editor nodes prop="constraintV" options=choices(&[("top","Top"),("bottom","Bottom"),("center","Center"),("stretch","Top + bottom"),("scale","Scale")])/></div>
            </section>
        </Show>
        <section class="inspector-section"><div class="section-heading"><span>"Export"</span></div>
            <div class="property-grid inspector-mb8">
                <select id="export-scale" aria-label="Export scale" prop:value=move || editor.shell.with(|state| state.export_scale.to_string()) on:change=move |event| editor.shell.update(|state| state.export_scale=event_target_value(&event).parse().unwrap_or(2.0))>
                    <option value="1">"1×"</option><option value="2">"2×"</option><option value="3">"3×"</option>
                </select>
                <select id="export-format" aria-label="Export format" prop:value=move || editor.shell.with(|state| state.export_format.clone()) on:change=move |event| editor.shell.update(|state| state.export_format=event_target_value(&event))><option>"PNG"</option><option>"SVG"</option></select>
            </div>
            <button class="export-button" data-action="exportSelection" on:click=move |_| shell::dispatch(editor,"exportSelection")>{icon("download",13)}" Export "{move || nodes.with(|nodes| if nodes.len()>1 {"selection".into()} else {nodes.first().map_or_else(String::new,|node| {let units:Vec<_>=node.name.encode_utf16().collect(); if units.len()>21 {String::from_utf16_lossy(&units[..20])+"…"} else {node.name.clone()}})})}</button>
        </section>
        <section class="inspector-section"><div class="section-heading"><span>"Developer"</span></div><button class="wide-button" data-action="inspectCSS" on:click=move |_| shell::dispatch(editor,"inspectCSS")>{icon("code",14)}" Inspect CSS"</button></section>
    }
}

#[component]
fn Typography(editor: Editor, nodes: Selection) -> impl IntoView {
    let families = Signal::derive(move || {
        let mut names: Vec<String> = [
            "Inter",
            "Arial",
            "Helvetica Neue",
            "Georgia",
            "Times New Roman",
            "Verdana",
            "Courier New",
            "monospace",
            "serif",
            "sans-serif",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        editor.doc.with(|doc| {
            for family in doc.data.fonts.keys() {
                if !names.contains(family) {
                    names.push(family.clone());
                }
            }
        });
        let current = text(nodes, "fontFamily");
        if !names.contains(&current) {
            names.push(current);
        }
        names.into_iter().map(|name| (name.clone(), name)).collect()
    });
    view! {
        <section class="inspector-section"><div class="section-heading"><span>"Typography"</span>{icon("text",14)}</div>
            <div class="stack"><SelectField editor nodes prop="fontFamily" options=families/>
                <div class="property-grid"><SelectField editor nodes prop="fontWeight" options=choices(&[("300","Light"),("400","Regular"),("500","Medium"),("600","Semibold"),("700","Bold"),("800","Extra bold"),("900","Black")])/>
                    <NumberField editor nodes prop="fontSize" label="Ag" min=1.0 max=512.0/></div>
                <div class="property-grid"><NumberField editor nodes prop="lineHeight" label="↕" unit="%" min=50.0 max=500.0/><NumberField editor nodes prop="letterSpacing" label="↔" unit="px" step=0.1/></div>
                <div class="property-grid"><div class="segmented">{[("left","textLeft"),("center","textCenter"),("right","textRight")].into_iter().map(|(align,symbol)| view! {
                    <button data-text-align=align title=format!("Align text {align}") aria-label=format!("Align text {align}") class:active=move || text(nodes,"textAlign")==align on:click=move |_| shell::set_property(editor,"textAlign",json!(align),false)>{icon(symbol,14)}</button>
                }).collect_view()}</div><div class="segmented">
                    <button data-text-style="bold" title="Bold" aria-label="Bold" class:active={move || number(nodes,"fontWeight")>=700.0} on:click=move |_| shell::set_property(editor,"fontWeight",json!(if number(nodes,"fontWeight")>=700.0 {400}else{700}),false)>{icon("bold",14)}</button>
                    <button data-text-style="italic" title="Italic" aria-label="Italic" class:active=move || text(nodes,"fontStyle")=="italic" on:click=move |_| shell::set_property(editor,"fontStyle",json!(if text(nodes,"fontStyle")=="italic" {"normal"}else{"italic"}),false)>{icon("italic",14)}</button>
                    <button data-text-style="underline" title="Underline" aria-label="Underline" class:active=move || text(nodes,"textDecoration")=="underline" on:click=move |_| shell::set_property(editor,"textDecoration",json!(if text(nodes,"textDecoration")=="underline" {"none"}else{"underline"}),false)>{icon("underline",14)}</button>
                </div></div>
                <SelectField editor nodes prop="textCase" options=choices(&[("none","Original case"),("upper","UPPERCASE"),("lower","lowercase"),("title","Title Case")])/>
                <SelectField editor nodes prop="direction" options=choices(&[("auto","Automatic direction"),("ltr","Left to right"),("rtl","Right to left")])/>
                <div class="property-grid"><button class="wide-button" data-action="editText" on:click=move |_| shell::dispatch(editor,"editText")>"Edit text"</button><button class="wide-button" data-action="loadFont" on:click=move |_| shell::dispatch(editor,"loadFont")>"Load font…"</button></div>
            </div>
        </section>
    }
}

#[component]
fn Fill(editor: Editor, nodes: Selection) -> impl IntoView {
    view! {
        <section class="inspector-section"><div class="section-heading"><span>"Fill"</span>
            <button class="icon-button small" data-action="toggleFill" title="Toggle fill" aria-label="Toggle fill" on:click=move |_| shell::dispatch(editor,"toggleFill")>{move || icon(if text(nodes,"fill")=="none" {"plus"}else{"minus"},14)}</button>
        </div>
            <SelectField editor nodes prop="fillType" options=choices(&[("solid","Solid"),("linear","Linear gradient")])/>
            <ColorField editor nodes prop="fill" opacity="fillOpacity"/>
            <Show when=move || text(nodes,"fillType")=="linear"><ColorField editor nodes prop="fill2"/><div class="inspector-mt8"><NumberField editor nodes prop="gradientAngle" label="∠" unit="°"/></div></Show>
            <div class="color-tokens"><For each=move || tokens(editor,"colors") key=|(index,token)| (*index,token.to_string()) children=move |(_,token)| {
                let color = safe_color(&token_string(&token,"value"));
                let background = color.clone();
                let attr = color.clone();
                view! {<button data-fill=attr title=token_string(&token,"name") aria-label=token_string(&token,"name") style:background=background on:click=move |_| shell::set_property(editor,"fill",json!(color),false)></button>}
            }/></div>
        </section>
    }
}

#[component]
fn Stroke(editor: Editor, nodes: Selection) -> impl IntoView {
    view! {
        <section class="inspector-section"><div class="section-heading"><span>"Stroke"</span>
            <button class="icon-button small" data-action="toggleStroke" title="Toggle stroke" aria-label="Toggle stroke" on:click=move |_| shell::dispatch(editor,"toggleStroke")>{move || icon(if number(nodes,"strokeWidth")!=0.0 {"minus"}else{"plus"},14)}</button>
        </div><Show when={move || number(nodes,"strokeWidth")>0.0} fallback=|| view! {<span class="small-label">"No stroke"</span>}>
            <ColorField editor nodes prop="stroke"/><div class="inspector-mt8"><NumberField editor nodes prop="strokeWidth" label="W" min=0.0 max=1000.0/></div>
        </Show></section>
    }
}

#[component]
fn Effects(editor: Editor, nodes: Selection) -> impl IntoView {
    let shadow = move || nodes.with(|nodes| nodes.first().is_some_and(|node| node.shadow));
    view! {
        <section class="inspector-section"><div class="section-heading"><span>"Effects"</span>
            <button class="icon-button small" data-action="toggleShadow" title="Toggle shadow" aria-label="Toggle shadow" on:click=move |_| shell::dispatch(editor,"toggleShadow")>{move || icon(if shadow() {"minus"}else{"plus"},14)}</button>
        </div><Show when=shadow fallback=|| view! {<span class="small-label">"No effects"</span>}>
            <div class="fill-row"><span class="small-label">"Drop shadow"</span><button class="icon-button small inspector-ml-auto" data-action="toggleShadow" title="Remove shadow" aria-label="Remove shadow" on:click=move |_| shell::dispatch(editor,"toggleShadow")>{icon("eye",14)}</button></div>
            <div class="property-grid inspector-mt8"><NumberField editor nodes prop="shadowX" label="X"/><NumberField editor nodes prop="shadowY" label="Y"/>
                <NumberField editor nodes prop="shadowBlur" label="↔" min=0.0/><NumberField editor nodes prop="shadowOpacity" label="α" unit="%" min=0.0 max=100.0/></div>
            <ColorField editor nodes prop="shadowColor"/>
        </Show></section>
    }
}
