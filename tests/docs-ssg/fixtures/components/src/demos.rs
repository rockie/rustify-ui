use leptos::prelude::*;
use rustify_components::{Button, TextField};

pub fn counter(initial: i32, label: String, children: AnyView) -> impl IntoView {
    let value = RwSignal::new(initial);
    view! {
        <div data-counter="" class="min-h-24 space-y-3">
            <p>{label}</p>
            <div class="flex flex-wrap items-center gap-3">
                <Button test_id="counter-increment" on_click=move || value.update(|value| *value += 1)>
                    "Increment"
                </Button>
                <output data-testid="counter-value">{move || value.get()}</output>
                <TextField test_id="counter-input" value=Signal::derive(move || value.get().to_string())
                    on_change=move |input: String| {
                        if let Ok(input) = input.parse() { value.set(input); }
                    }
                    aria_label="Counter value"
                />
            </div>
            {children}
        </div>
    }
}

/// The function and its containing file are displayed verbatim by the docs.
pub fn counter_demo() -> impl IntoView {
    counter(
        7,
        "Résumé — a real registered counter".to_string(),
        view! { <p>"This child is part of the same view on both targets."</p> }.into_any(),
    )
}

pub fn async_after_attach_demo() -> impl IntoView {
    // No asynchronous initial data: the action starts only after a reader clicks.
    let load = Action::new(|_: &()| async { "Loaded after attachment".to_string() });
    view! {
        <div data-async-after-attach="" class="space-y-3">
            <Button test_id="async-start" on_click=move || { load.dispatch(()); }>
                "Load browser data"
            </Button>
            <output data-testid="async-result">
                {move || load.value().get().unwrap_or_else(|| "Not loaded".to_string())}
            </output>
        </div>
    }
}

pub fn failure_demo() -> impl IntoView {
    let value = RwSignal::new(0);
    view! {
        <div data-failure-demo="" class="space-y-3">
            <Button test_id="failure-increment" on_click=move || value.update(|value| *value += 1)>
                "Change before failure"
            </Button>
            <output data-testid="failure-value">{move || value.get()}</output>
            <Button test_id="failure-crash" on_click=|| panic!("docs fixture callback failure")>
                "Crash this example"
            </Button>
        </div>
    }
}

pub fn catalog_proxy() -> impl IntoView {
    let scope = rustify_ui::id::next("catalog").unwrap_or_else(|| "catalog".to_string());
    view! { <crate::catalog::CatalogView scope=scope /> }
}
