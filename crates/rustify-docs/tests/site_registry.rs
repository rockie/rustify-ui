#![cfg(feature = "site-ssr")]

use leptos::prelude::*;
use rustify_docs::site::{
    HelperOperation, HelperRequest, HelperResult, RenderContext, RenderRequest, RenderTarget,
    SiteEnvironment, SiteRegistry, PROTOCOL_VERSION,
};
use serde_json::json;

fn counter(initial: i32, label: String, children: AnyView) -> impl IntoView {
    let value = RwSignal::new(initial);
    view! {
        <div data-counter="">
            <label>{label}</label>
            <input value=move || value.get().to_string() prop:value=move || value.get().to_string() />
            <button on:click=move |_| value.update(|value| *value += 1)>"Increment"</button>
            {children}
        </div>
    }
}

fn counter_demo() -> impl IntoView {
    counter(7, "Demo".to_string(), ().into_any())
}

fn page(context: RenderContext) -> impl IntoView {
    view! { <article><p>{context.route.clone()}</p>{context.demo("CounterDemo")}</article> }
}

fn footer(context: RenderContext) -> impl IntoView {
    view! { <footer>{context.base}</footer> }
}

fn site() -> SiteRegistry {
    rustify_docs::site! {
        components {
            Counter { initial: i32 = 2, label: String = "Counter".to_string() } =>
                |props, children| counter(props.initial, props.label, children),
        }
        demos { CounterDemo => counter_demo, }
        pages { Home { route: "/", title: "Home" } => page, }
        slots { footer => footer, }
    }
}

fn component(props: serde_json::Value) -> RenderRequest {
    RenderRequest {
        key: "counter-key".to_string(),
        seed: "counter-seed".to_string(),
        target: RenderTarget::Component {
            name: "Counter".to_string(),
            props,
            children_html: "<p>Safe <strong>children</strong></p>".to_string(),
        },
    }
}

#[test]
fn generated_props_apply_defaults_once_and_echo_effective_values() {
    let result = site().render(&component(json!({})), &SiteEnvironment::default());
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    assert_eq!(
        result.props,
        Some(json!({"initial": 2, "label": "Counter"}))
    );
    let html = result.html.expect("static HTML");
    assert!(html.contains("value=\"2\""), "{html}");
    assert!(
        html.contains("<p>Safe <strong>children</strong></p>"),
        "{html}"
    );
    assert!(
        html.contains("data-rustify-seed=\"counter-seed\""),
        "{html}"
    );
}

#[test]
fn generated_props_reject_unknown_fields_and_wrong_types() {
    for (props, field) in [
        (json!({"unexpected": 1}), "unexpected"),
        (json!({"initial": "wrong"}), "initial"),
    ] {
        let result = site().render(&component(props), &SiteEnvironment::default());
        assert!(result.html.is_none());
        assert_eq!(result.diagnostics[0].code, "invalid_props");
        assert_eq!(result.diagnostics[0].property.as_deref(), Some(field));
    }
}

#[test]
fn invalid_generated_props_include_the_structured_field_path() {
    let result = site().render(
        &component(json!({"initial": "wrong"})),
        &SiteEnvironment::default(),
    );
    assert_eq!(result.diagnostics[0].property.as_deref(), Some("initial"));
}

#[derive(serde::Serialize, serde::Deserialize)]
struct GroupProps {
    thresholds: Vec<i32>,
}

#[test]
fn invalid_nested_props_include_the_field_and_array_path() {
    let registry = rustify_docs::site! {
        components {
            Groups { groups: Vec<GroupProps> } => |props, _children|
                view! { <p>{props.groups.len()}</p> },
        }
    };
    let result = registry.render(
        &RenderRequest {
            key: "groups".to_string(),
            seed: "groups".to_string(),
            target: RenderTarget::Component {
                name: "Groups".to_string(),
                props: json!({"groups": [{"thresholds": [1, "wrong"]}]}),
                children_html: String::new(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.html.is_none());
    assert_eq!(
        result.diagnostics[0].property.as_deref(),
        Some("groups[0].thresholds[1]")
    );
}

#[test]
fn batch_protocol_echoes_generation_and_each_key() {
    let response = site()
        .handle(HelperRequest {
            protocol_version: PROTOCOL_VERSION,
            generation: "generation-a".to_string(),
            operation: HelperOperation::Render {
                environment: SiteEnvironment::default(),
                items: vec![
                    component(json!({"initial": 9})),
                    RenderRequest {
                        key: "demo-key".to_string(),
                        seed: "demo-seed".to_string(),
                        target: RenderTarget::Demo {
                            name: "CounterDemo".to_string(),
                        },
                    },
                ],
            },
        })
        .expect("batch response");
    assert_eq!(response.generation, "generation-a");
    let HelperResult::Render { items } = response.result else {
        panic!("render response")
    };
    assert_eq!(
        items
            .iter()
            .map(|item| item.key.as_str())
            .collect::<Vec<_>>(),
        ["counter-key", "demo-key"]
    );
    assert!(items
        .iter()
        .all(|item| item.html.is_some() && item.diagnostics.is_empty()));
}

#[test]
fn helper_rejects_wrong_protocol_and_duplicate_batch_identity() {
    assert!(site()
        .handle(HelperRequest {
            protocol_version: 99,
            generation: String::new(),
            operation: HelperOperation::Describe,
        })
        .is_err());
    assert!(site()
        .handle(HelperRequest {
            protocol_version: PROTOCOL_VERSION,
            generation: String::new(),
            operation: HelperOperation::Render {
                environment: SiteEnvironment::default(),
                items: vec![component(json!({})), component(json!({}))],
            },
        })
        .is_err());
}

#[test]
fn compiled_description_contains_normalized_parser_data_and_locked_resources() {
    let description = site().describe();
    assert_eq!(
        description.registry["components"][0]["fields"][0]["default"],
        "2"
    );
    assert_eq!(description.registry["demos"][0]["function"], "counter_demo");
    assert_eq!(description.runtime_protocol, rustify_ui::runtime::PROTOCOL);
    assert!(description
        .resources
        .iter()
        .any(|asset| asset.path == "dom-loader.js"
            && asset.content == rustify_ui::runtime::DOM_LOADER_JS));
    assert!(description
        .resources
        .iter()
        .all(|asset| asset.sha256.len() == 64));
}

#[test]
fn static_builtin_dispatch_uses_the_same_generated_props() {
    let html = site()
        .render_component_static("Counter", json!({"initial": 5}), String::new())
        .expect("static component");
    assert!(html.contains("value=\"5\""), "{html}");
    assert!(!html.contains("data-rustify-scope"), "{html}");
}

#[test]
fn a_page_collects_the_same_scoped_demo_renderer_without_a_second_shell() {
    let result = site().render(
        &RenderRequest {
            key: "home".to_string(),
            seed: "home".to_string(),
            target: RenderTarget::Page {
                name: "Home".to_string(),
                route: "/".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    assert_eq!(result.demos.len(), 1);
    assert!(result
        .html
        .expect("page HTML")
        .contains("data-rustify-demo-placeholder=\"home-demo-0\""));
    assert!(result.demos[0]
        .html
        .as_ref()
        .expect("demo HTML")
        .contains("value=\"7\""));
}

#[test]
fn invalid_surface_demo_names_fail_the_parent_render() {
    let registry = SiteRegistry::new(json!({})).page("Broken", |context| {
        view! { <article>{context.demo("Unknown")}</article> }
    });
    let result = registry.render(
        &RenderRequest {
            key: "broken".to_string(),
            seed: "broken".to_string(),
            target: RenderTarget::Page {
                name: "Broken".to_string(),
                route: "/broken".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.html.is_none());
    assert_eq!(result.diagnostics[0].code, "unknown_demo");
}

#[test]
fn a_real_resource_registers_cross_target_data_and_is_rejected() {
    let registry = SiteRegistry::new(json!({})).demo("Resource", || {
        let data = Resource::new(|| (), |_| async { 42_i32 });
        view! { <p>{move || data.get().unwrap_or_default()}</p> }
    });
    let result = registry.render(
        &RenderRequest {
            key: "resource".to_string(),
            seed: "resource".to_string(),
            target: RenderTarget::Demo {
                name: "Resource".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.html.is_none());
    assert_eq!(result.diagnostics[0].code, "unsupported_initial_data");
}

#[test]
fn a_real_suspense_resource_fallback_is_not_mistaken_for_supported_static_data() {
    let registry = SiteRegistry::new(json!({})).demo("Suspense", || {
        let data = Resource::new(|| (), |_| async { 42_i32 });
        view! { <Suspense fallback=|| view! { <p>"Loading"</p> }><p>{move || data.get()}</p></Suspense> }
    });
    let result = registry.render(
        &RenderRequest {
            key: "suspense".to_string(),
            seed: "suspense".to_string(),
            target: RenderTarget::Demo {
                name: "Suspense".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.html.is_none());
    assert_eq!(result.diagnostics[0].code, "unsupported_initial_data");
}

#[test]
fn a_real_error_boundary_registers_cross_target_errors_and_is_rejected() {
    let registry = SiteRegistry::new(json!({})).demo("Errors", || {
        view! {
            <ErrorBoundary fallback=|_| view! { <p>"Failed"</p> }>
                {Err::<(), _>(std::io::Error::other("failed initial data"))}
            </ErrorBoundary>
        }
    });
    let result = registry.render(
        &RenderRequest {
            key: "errors".to_string(),
            seed: "errors".to_string(),
            target: RenderTarget::Demo {
                name: "Errors".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.html.is_none());
    assert_eq!(result.diagnostics[0].code, "unsupported_initial_data");
}

#[test]
fn server_generation_never_polls_an_author_async_request() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let ran = std::sync::Arc::new(AtomicBool::new(false));
    let probe = ran.clone();
    let registry = SiteRegistry::new(json!({})).demo("Resource", move || {
        let probe = probe.clone();
        let _data = Resource::new(
            || (),
            move |_| {
                let probe = probe.clone();
                async move {
                    probe.store(true, Ordering::Relaxed);
                    42_i32
                }
            },
        );
        view! { <p>"Initial"</p> }
    });
    let _ = registry.render(
        &RenderRequest {
            key: "resource".to_string(),
            seed: "resource".to_string(),
            target: RenderTarget::Demo {
                name: "Resource".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(!ran.load(Ordering::Relaxed));
}

#[test]
fn an_actual_open_dialog_reports_an_unsupported_initial_state() {
    let registry = SiteRegistry::new(json!({})).demo("OpenDialog", || {
        view! {
            <rustify_components::Dialog open=true on_open_change=|_| {} title="Already open">
                <p>"Contents"</p>
            </rustify_components::Dialog>
        }
    });
    let result = registry.render(
        &RenderRequest {
            key: "dialog".to_string(),
            seed: "dialog".to_string(),
            target: RenderTarget::Demo {
                name: "OpenDialog".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.html.is_none());
    assert_eq!(result.diagnostics[0].code, "unsupported_initial_state");
}

#[test]
fn an_actual_initial_toast_reports_an_unsupported_initial_state() {
    let registry = SiteRegistry::new(json!({})).demo("InitialToast", || {
        let toasts = rustify_ui::provide_toasts();
        toasts.show("Already showing", rustify_ui::ToastOptions::default());
        view! { <rustify_components::Toaster /> }
    });
    let result = registry.render(
        &RenderRequest {
            key: "toast".to_string(),
            seed: "toast".to_string(),
            target: RenderTarget::Demo {
                name: "InitialToast".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.html.is_none());
    assert_eq!(result.diagnostics[0].code, "unsupported_initial_state");
}

#[test]
fn an_explicit_client_only_component_does_not_construct_its_host_view() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let constructed = std::sync::Arc::new(AtomicBool::new(false));
    let probe = constructed.clone();
    let registry = rustify_docs::site! {
        components {
            #[client_only(class = "h-48", message = "Requires browser")]
            Browser { initial: i32 = 9 } => move |props, _children| {
                probe.store(true, Ordering::Relaxed);
                view! { <input value=props.initial.to_string() /> }
            },
        }
    };
    let result = registry.render(
        &RenderRequest {
            key: "browser".to_string(),
            seed: "browser".to_string(),
            target: RenderTarget::Component {
                name: "Browser".to_string(),
                props: json!({}),
                children_html: String::new(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.client_only);
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    let html = result.html.expect("fixed placeholder");
    assert!(
        html.contains("Requires browser") && html.contains("h-48"),
        "{html}"
    );
    assert_eq!(result.props, Some(json!({"initial": 9})));
    assert!(!constructed.load(Ordering::Relaxed));
}

static DEFAULT_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn counted_default() -> i32 {
    DEFAULT_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    4
}

#[test]
fn a_default_expression_is_executed_once_per_missing_property() {
    DEFAULT_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);
    let registry = rustify_docs::site! {
        components {
            Counted { initial: i32 = counted_default() } =>
                |props, _children| view! { <input value=props.initial.to_string() /> },
        }
    };
    let result = registry.render(
        &RenderRequest {
            key: "counted".to_string(),
            seed: "counted".to_string(),
            target: RenderTarget::Component {
                name: "Counted".to_string(),
                props: json!({}),
                children_html: String::new(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    assert_eq!(result.props, Some(json!({"initial": 4})));
    assert_eq!(DEFAULT_CALLS.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[test]
fn a_nested_demo_does_not_release_the_parent_resource_suppression() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let ran = std::sync::Arc::new(AtomicBool::new(false));
    let probe = ran.clone();
    let registry = SiteRegistry::new(json!({}))
        .demo("Safe", || view! { <p>"Safe demo"</p> })
        .page("ResourcePage", move |context| {
            let demo = context.demo("Safe");
            let probe = probe.clone();
            let _data = Resource::new(
                || (),
                move |_| {
                    let probe = probe.clone();
                    async move {
                        probe.store(true, Ordering::Relaxed);
                        42_i32
                    }
                },
            );
            view! { <article>{demo}</article> }
        });
    let result = registry.render(
        &RenderRequest {
            key: "page".to_string(),
            seed: "page".to_string(),
            target: RenderTarget::Page {
                name: "ResourcePage".to_string(),
                route: "/resource".to_string(),
            },
        },
        &SiteEnvironment::default(),
    );
    assert!(result.html.is_none());
    assert_eq!(result.diagnostics[0].code, "unsupported_initial_data");
    assert!(!ran.load(Ordering::Relaxed));
}
