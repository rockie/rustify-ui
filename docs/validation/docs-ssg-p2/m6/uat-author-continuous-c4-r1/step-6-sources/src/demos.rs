use leptos::prelude::*;
use rustify_components::{Button, TextField};

pub fn counter(initial: i32, label: String, children: AnyView) -> impl IntoView {
    let value = RwSignal::new(initial);
    view! {
        <div class="space-y-3">
            <p>{label}</p>
            <div class="flex flex-wrap items-center gap-3">
                <Button test_id="counter-increment" on_click=move || value.update(|value| *value += 1)>
                    "Increment"
                </Button>
                <output data-testid="counter-value">{move || value.get()}</output>
                <TextField test_id="counter-input" aria_label="Counter value"
                    value=Signal::derive(move || value.get().to_string())
                    on_change=move |input: String| {
                        if let Ok(input) = input.parse() { value.set(input); }
                    }
                />
            </div>
            {children}
        </div>
    }
}

pub fn counter_demo() -> impl IntoView {
    counter(
        7,
        "A registered counter — résumé".to_string(),
        view! { <p>"This child belongs to the same view on both targets."</p> }.into_any(),
    )
}

pub fn double_counter_demo() -> impl IntoView {
    counter(21, "Double counter".to_string(), ().into_any())
}

pub fn examples_page(context: rustify_docs::site::RenderContext) -> impl IntoView {
    let first = context.demo("CounterDemo");
    let second = context.demo("CounterDemo");
    view! {
        <section>
            <h2 id="paired-counters">"Two independent counters"</h2>
            {first}
            {second}
        </section>
    }
}


pub fn footer(context: rustify_docs::site::RenderContext) -> impl IntoView {
    view! { <p><a href=context.base>"Home"</a></p> }
}
