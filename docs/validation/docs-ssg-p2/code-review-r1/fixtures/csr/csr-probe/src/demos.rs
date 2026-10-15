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
                <output class={move || if value.get() >= 8 { active_class() } else { "p-1" }} data-testid="counter-value">{move || value.get()}</output>
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

fn active_class() -> &'static str {
    #[cfg(target_arch = "wasm32")] { browser_palette::ACTIVE_CLASS }
    #[cfg(not(target_arch = "wasm32"))] { "p-1" }
}

// Force the author source to be rebuilt for the control observation.
