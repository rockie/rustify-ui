use leptos::prelude::*;
use leptos::wasm_bindgen::{prelude::*, JsCast};
use rustify_components::{Button, Dialog};
use rustify_ui::{
    mount, AppHandle, MountConfig, Theme, ThemeBoundary, ThemeDocument, ThemeMode, ThemeScope,
    ThemeValuePatch, ThemedScope,
};
use std::{cell::RefCell, collections::BTreeMap, sync::Arc};

thread_local! {static FIXTURES:RefCell<Vec<AppHandle>>=const {RefCell::new(Vec::new())};}

#[component]
fn Samples() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="p-4 bg-background text-foreground" data-testid="theme-fixture-body">
            <div class="rounded-sm bg-card p-4" data-testid="fixture-sm">"sm"</div>
            <div class="rounded-md bg-card p-4" data-testid="fixture-md">"md"</div>
            <div class="rounded-lg bg-card p-4 shadow-lg" data-testid="fixture-lg">"lg"</div>
            <div class="rounded-xl bg-card p-4" data-testid="fixture-xl">"xl"</div>
            <Button on_click=move || open.set(true) test_id="fixture-dialog-open">"Open dialog"</Button>
            <Dialog open=open on_open_change=move |value|open.set(value) title=Signal::derive(||"Compatibility".to_owned()) modal=false test_id="fixture-dialog">
                <p>"One stylesheet. Two provider contracts."</p>
            </Dialog>
        </div>
    }
}

#[component]
fn Fixture(new: bool) -> impl IntoView {
    let radius = RwSignal::new("6px".to_owned());
    let patch = RwSignal::new(ThemeValuePatch::default());
    let document = Signal::derive(move || {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        document
            .styles
            .light
            .0
            .insert("radius".into(), radius.get());
        Arc::new(
            rustify_ui::resolve(
                &document,
                ThemeMode::Light,
                &rustify_ui::theme::font_context(16., false),
            )
            .expect("fixture values"),
        )
    });
    view! {
        {if new {view!{<ThemeScope resolved=document />}.into_any()} else {view!{<ThemedScope theme=Signal::derive(Theme::light) />}.into_any()}}
        <div class="flex gap-2 p-4">
            <Button on_click=move || radius.set("10px".into()) test_id="fixture-change">"Change radius"</Button>
            <Button on_click=move || patch.set(ThemeValuePatch(BTreeMap::from([("radius".into(),"0px".into()),("shadow-opacity".into(),"0.3".into())]))) test_id="fixture-patch">"Patch"</Button>
            <Button on_click=move || patch.set(ThemeValuePatch::default()) test_id="fixture-unpatch">"Unpatch"</Button>
        </div>
        {if new {view!{<ThemeBoundary patch=patch test_id="fixture-boundary"><Samples /></ThemeBoundary>}.into_any()} else {view!{<Samples />}.into_any()}}
    }
}

#[wasm_bindgen]
pub fn catalog_theme_fixtures(legacy: &str, new: &str) -> Result<(), JsValue> {
    catalog_theme_fixtures_dispose();
    let mut handles = Vec::new();
    for (id, new) in [(legacy, false), (new, true)] {
        let element = document()
            .get_element_by_id(id)
            .ok_or_else(|| JsValue::from_str("missing theme fixture root"))?
            .dyn_into::<leptos::web_sys::HtmlElement>()?;
        handles.push(
            mount(
                element,
                MountConfig {
                    scope: format!("catalog-theme-{new}"),
                    ..Default::default()
                },
                move || view! {<Fixture new=new />},
            )
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
        );
    }
    FIXTURES.with(|fixtures| *fixtures.borrow_mut() = handles);
    Ok(())
}

#[wasm_bindgen]
pub fn catalog_theme_fixtures_dispose() {
    FIXTURES.with(|fixtures| fixtures.borrow_mut().clear())
}
