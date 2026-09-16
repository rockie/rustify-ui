use crate::{
    app::{js_error, Editor},
    document::Node,
};
use leptos::prelude::*;
use serde_json::{json, Value};

pub fn instantiate(editor: Editor, id: &str) {
    let center = super::center(editor);
    let mut instance = None;
    let result = editor.edit("Place component instance", |doc| {
        instance = crate::commands::instantiate(doc, id, center);
        Ok(())
    });
    super::report(editor, result);
    editor.select(instance.into_iter().collect());
}

pub fn insert(editor: Editor, kind: &str) {
    let center = super::center(editor);
    let mut selected = None;
    let result=editor.edit(&format!("Insert {kind}"),|doc|{
        let (frame,children): (Value,Vec<Value>)=match kind {
            "button"=>(json!({"name":"Button / Primary","x":center.x-90.,"y":center.y-24.,"w":180,"h":48,"fill":"#8462e8","radius":9,"clip":false}),vec![json!({"x":20,"y":14,"w":140,"h":24,"text":"Get started   ↗","name":"Label","fill":"#ffffff","fontSize":14,"fontWeight":500,"textAlign":"center"})]),
            "badge"=>(json!({"name":"Badge / Status","x":center.x-72.,"y":center.y-17.,"w":144,"h":34,"fill":"#d4e8dc","radius":17}),vec![json!({"x":14,"y":8,"w":120,"h":22,"text":"●  In progress","name":"Status","fontSize":12,"fill":"#537962"})]),
            _=>(json!({"name":"Card / Content","x":center.x-150.,"y":center.y-100.,"w":300,"h":200,"fill":"#eee8fb","radius":16,"shadow":true}),vec![json!({"x":25,"y":29,"w":250,"h":80,"text":"A little\ninspiration.","name":"Heading","fontSize":30,"fill":"#77559b","fontWeight":600,"lineHeight":1.15}),json!({"x":25,"y":139,"w":250,"h":42,"text":"Give your next big idea\na little room to grow.","name":"Body","fontSize":13,"fill":"#a18bb5"})]),
        };
        let make=|kind:&str,properties:Value|->Result<Node,wasm_bindgen::JsValue>{let mut value=serde_json::to_value(Node::new(kind)).map_err(js_error)?;if let(Some(target),Some(properties))=(value.as_object_mut(),properties.as_object()){target.extend(properties.clone());}serde_json::from_value(value).map_err(js_error)};
        let id=doc.add(make("frame",frame)?);
        for mut child in children {child["parentId"]=json!(id);doc.add(make("text",child)?);}
        selected=Some(id);Ok(())
    });
    super::report(editor, result);
    if let Some(id) = selected {
        editor.shell.update(|shell| {
            shell.expanded.insert(id.clone());
        });
        editor.select(vec![id]);
    }
}

#[component]
pub fn Assets(editor: Editor) -> impl IntoView {
    view! {<div id="assets-panel" class="assets-panel" class:hidden=move||editor.shell.with(|shell|shell.left_tab!="assets")>
        <p class="small-label">"Double-click a component to place an instance."</p>
        {move||editor.doc.with(|doc|doc.data.pages.iter().flat_map(|page|&page.nodes).filter(|node|node.flag("component")).map(|node|{
            let id=node.id.clone();let color=safe_color(&node.fill);
            view!{<div class="asset-card" data-component=id.clone() on:dblclick=move |_|instantiate(editor,&id)><div class="asset-preview"><div class="asset-button" style:background=color>{node.name.clone()}</div></div><div class="asset-caption">{node.name.clone()}<span>"◇"</span></div></div>}
        }).collect_view())}
        <h4>"Quick insert"</h4>
        <div class="asset-card" data-asset="button" on:click=move |_|insert(editor,"button")><div class="asset-preview"><span class="asset-button">"Get started ↗"</span></div><div class="asset-caption">"Button / Primary"<span>"＋"</span></div></div>
        <div class="asset-card" data-asset="card" on:click=move |_|insert(editor,"card")><div class="asset-preview"><div style:background="#eee8fb" style:padding="16px 25px" style:border-radius="10px" style:color="#8462e8">"A little inspiration."</div></div><div class="asset-caption">"Card / Content"<span>"＋"</span></div></div>
        <div class="asset-card" data-asset="badge" on:click=move |_|insert(editor,"badge")><div class="asset-preview"><span style:padding="7px 18px" style:background="#d4e8dc" style:color="#537962" style:border-radius="20px">"● In progress"</span></div><div class="asset-caption">"Badge / Status"<span>"＋"</span></div></div>
    </div>}
}

pub fn safe_color(value: &str) -> String {
    if value.starts_with('#')
        && (4..=9).contains(&value.len())
        && value[1..].bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        value.into()
    } else {
        "#8462e8".into()
    }
}
