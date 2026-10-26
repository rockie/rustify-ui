//! On-demand DOM component fixture using the scope's main-realm cn engine.
use leptos::{prelude::*, wasm_bindgen::prelude::*};
use rustify_components::{Button, ButtonSize};
use rustify_style::cn::{ClassGroupDef, Cn, Config};
use rustify_style::protocol::{CompilerInput, Limits};
use rustify_ui::utility::{UtilityConfig, UtilityScope};
use rustify_ui::{mount, AppHandle, MountConfig, ScopeRoots};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

rustify_components::clx! { CnCard, div, "p-2", "bg-[#123456]" }

struct Live {
    mount: AppHandle,
    scope: UtilityScope,
    caller: RwSignal<String>,
    size: RwSignal<ButtonSize>,
}
thread_local! { static LIVE: RefCell<BTreeMap<String, Live>> = const { RefCell::new(BTreeMap::new()) }; }

fn component_cn(disjoint: bool, fail_callback: bool) -> Result<Cn, JsValue> {
    let mut config = Config::default();
    if disjoint {
        config.class_groups.retain(|(name, _)| name != "p");
        config.class_groups.extend([
            (
                "small-padding".into(),
                vec![ClassGroupDef::Literal("p-2".into())],
            ),
            (
                "large-padding".into(),
                vec![ClassGroupDef::Literal("p-4".into())],
            ),
        ]);
    }
    if fail_callback {
        config.class_groups.retain(|(name, _)| name != "p");
        config.class_groups.push((
            "failing-padding".into(),
            vec![ClassGroupDef::Object(vec![(
                "p".into(),
                vec![ClassGroupDef::Validator(Rc::new(|_| {
                    Err(rustify_style::cn::CnError::new("fixture callback failed"))
                }))],
            )])],
        ));
    }
    Cn::new(config).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen]
pub fn utility_lab_component_mount(id: &str, base: &str, disjoint: bool) -> Result<(), JsValue> {
    utility_lab_component_dispose(id);
    let cn = component_cn(disjoint, false)?;
    let container = document()
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str("missing component fixture root"))?
        .unchecked_into();
    let constructed = Rc::new(RefCell::new(None));
    let runtime = constructed.clone();
    let id_owned = id.to_string();
    let mount = mount(container, MountConfig { scope: id.into(), base: base.into(), url_owner: false }, move || {
        let scope = UtilityScope::new_with_cn(&expect_context::<ScopeRoots>(), UtilityConfig {
            input: CompilerInput { css: format!("@theme {{ --spacing: {}rem; }} @tailwind utilities;", if disjoint { "0.5" } else { "0.25" }).into(), entry: "memory:component.css".into() },
            assets: vec![],
            limits: Limits { input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024, ast_depth: 32, css_bytes: 262144, queued_bytes: 65536, cache_bytes: 1048576 },
            request_timeout_ms: 5000,
        }, cn);
        let caller = RwSignal::new("p-4".to_string());
        let size = RwSignal::new(ButtonSize::Two);
        if let Ok(scope) = &scope { provide_context(scope.cn_context()); }
        *runtime.borrow_mut() = Some((scope, caller, size));
        view! {
            <CnCard class="p-4" test_id=format!("{id_owned}-card")>"Scoped raw classes"</CnCard>
            <Button on_click=|| {} size=Signal::derive(move || rustify_components::Responsive::from(size.get())) class=caller test_id=format!("{id_owned}-button")>"Scoped button"</Button>
        }
    }).map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (scope, caller, size) = constructed
        .borrow_mut()
        .take()
        .ok_or_else(|| JsValue::from_str("component fixture did not mount"))?;
    match scope {
        Ok(scope) => {
            LIVE.with(|live| {
                live.borrow_mut().insert(
                    id.into(),
                    Live {
                        mount,
                        scope,
                        caller,
                        size,
                    },
                );
            });
            Ok(())
        }
        Err(error) => {
            let mut mount = mount;
            mount.dispose();
            Err(JsValue::from_str(&error.to_string()))
        }
    }
}

#[wasm_bindgen]
pub async fn utility_lab_component_ensure(id: &str, classes: &str) -> Result<(), JsValue> {
    let classes: Vec<String> =
        serde_json::from_str(classes).map_err(|error| JsValue::from_str(&error.to_string()))?;
    let scope = LIVE
        .with(|live| live.borrow().get(id).map(|live| live.scope.clone()))
        .ok_or_else(|| JsValue::from_str("component fixture is not mounted"))?;
    scope
        .ensure_classes(&classes)
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    Ok(())
}

#[wasm_bindgen]
pub async fn utility_lab_component_replace(
    id: &str,
    css: &str,
    disjoint: bool,
    fail_callback: bool,
) -> Result<String, JsValue> {
    let scope = LIVE
        .with(|live| live.borrow().get(id).map(|live| live.scope.clone()))
        .ok_or_else(|| JsValue::from_str("component fixture is not mounted"))?;
    let mut config = scope.configuration();
    config.input.css = css.into();
    scope
        .replace_config(config, component_cn(disjoint, fail_callback)?)
        .await
        .map_err(|error| JsValue::from_str(&serde_json::to_string(&error).unwrap_or_default()))?;
    utility_lab_component_report(id)
}

#[wasm_bindgen]
pub async fn utility_lab_component_flush(id: &str) -> Result<String, JsValue> {
    let scope = LIVE
        .with(|live| live.borrow().get(id).map(|live| live.scope.clone()))
        .ok_or_else(|| JsValue::from_str("component fixture is not mounted"))?;
    scope
        .flush()
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    utility_lab_component_report(id)
}

#[wasm_bindgen]
pub async fn utility_lab_component_retry(id: &str) -> Result<String, JsValue> {
    let scope = LIVE
        .with(|live| live.borrow().get(id).map(|live| live.scope.clone()))
        .ok_or_else(|| JsValue::from_str("component fixture is not mounted"))?;
    scope
        .retry()
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    utility_lab_component_report(id)
}

#[wasm_bindgen]
pub fn utility_lab_component_report(id: &str) -> Result<String, JsValue> {
    LIVE.with(|live| {
        let live = live.borrow();
        let scope = &live.get(id).ok_or_else(|| JsValue::from_str("component fixture is not mounted"))?.scope;
        let snapshot = scope.snapshot().ok_or_else(|| JsValue::from_str("component fixture has no installed snapshot"))?;
        serde_json::to_string(&serde_json::json!({"epoch":snapshot.epoch,"cn_epoch":scope.cn_context().epoch(),"report":snapshot.report,"config":scope.configuration()}))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    })
}

#[wasm_bindgen]
pub fn utility_lab_component_update(id: &str, caller: &str, small: bool) {
    LIVE.with(|live| {
        if let Some(live) = live.borrow().get(id) {
            live.caller.set(caller.into());
            live.size.set(if small {
                ButtonSize::One
            } else {
                ButtonSize::Two
            });
        }
    });
}

#[wasm_bindgen]
pub fn utility_lab_component_dispose(id: &str) {
    LIVE.with(|live| {
        if let Some(mut live) = live.borrow_mut().remove(id) {
            live.scope.dispose();
            live.mount.dispose();
        }
    });
}
