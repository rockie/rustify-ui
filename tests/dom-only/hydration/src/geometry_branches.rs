//! Ordinary CSR consumers need no explicit root geometry collector.

use leptos::prelude::*;
use rustify_components::Progress;

#[component]
pub(crate) fn GeometryBranches() -> impl IntoView {
    let first_open = RwSignal::new(false);
    let second_open = RwSignal::new(false);
    let first_value = RwSignal::new(50.0);
    let second_value = RwSignal::new(25.0);
    view! {
        <button data-testid="first-toggle" on:click=move |_| first_open.update(|open| *open = !*open)>
            "Toggle first"
        </button>
        <button data-testid="second-toggle" on:click=move |_| second_open.update(|open| *open = !*open)>
            "Toggle second"
        </button>
        <button data-testid="first-update" on:click=move |_| first_value.update(|value| *value += 10.0)>
            "Update first"
        </button>
        <button data-testid="second-update" on:click=move |_| second_value.update(|value| *value += 10.0)>
            "Update second"
        </button>
        <Show when=move || first_open.get()>
            <div class="harness-branch-progress">
                <Progress value=first_value aria_label="First progress" test_id="first-progress"/>
            </div>
        </Show>
        <Show when=move || second_open.get()>
            <div class="harness-branch-progress">
                <Progress value=second_value aria_label="Second progress" test_id="second-progress"/>
            </div>
        </Show>
    }
}
