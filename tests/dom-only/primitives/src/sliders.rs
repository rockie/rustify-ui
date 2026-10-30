//! Consumer-owned slider previews, acceptance policy and cancellation rollback.
use leptos::prelude::*;
use rustify_components::{primitives as p, Orientation};

fn encode(values: &[p::slider::ThumbValue]) -> String {
    values
        .iter()
        .map(|item| format!("{}={}", item.id, item.value))
        .collect::<Vec<_>>()
        .join(",")
}
#[component]
pub fn Sliders() -> impl IntoView {
    let values = RwSignal::new(vec![
        p::slider::ThumbValue {
            id: "left".into(),
            value: 20.0,
        },
        p::slider::ThumbValue {
            id: "right".into(),
            value: 80.0,
        },
    ]);
    let items = RwSignal::new(vec!["left", "right"]);
    let policy = RwSignal::new("accept".to_string());
    let requests = RwSignal::new(0_u32);
    let last = RwSignal::new(String::new());
    let pending = RwSignal::new(None::<Vec<p::slider::ThumbValue>>);
    let preview_start = RwSignal::new(None::<Vec<p::slider::ThumbValue>>);
    let commits = RwSignal::new(0_u32);
    let committed = RwSignal::new(String::new());
    let cancels = RwSignal::new(0_u32);
    let vertical_asked = RwSignal::new(String::new());
    let readonly = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let left_disabled = RwSignal::new(false);
    let preserve = RwSignal::new(false);
    let bad_step = RwSignal::new(false);
    let direction = RwSignal::new(rustify_ui::Direction::LeftToRight);
    view! {
        <section aria-label="Unstyled sliders">
            <h2>"Controlled named sliders"</h2>
            <label>"Acceptance policy"<select data-testid="slider-policy" on:change:target=move |event| policy.set(event.target().value())>
                <option value="accept">"Accept"</option><option value="refuse">"Refuse"</option><option value="delay">"Delay"</option>
            </select></label>
            <button type="button" data-testid="slider-readonly" on:click=move |_| readonly.update(|value| *value = !*value)>"Read only"</button>
            <button type="button" data-testid="slider-disabled" on:click=move |_| disabled.update(|value| *value = !*value)>"Disabled"</button>
            <button type="button" data-testid="slider-disable-left" on:click=move |_| left_disabled.update(|value| *value = !*value)>"Disable left"</button>
            <button type="button" data-testid="slider-preserve" on:click=move |_| preserve.update(|value| *value = !*value)>"Preserve order"</button>
            <button type="button" data-testid="slider-invalid-step" on:click=move |_| bad_step.update(|value| *value = !*value)>"Invalid step"</button>
            <button type="button" data-testid="slider-rtl" on:click=move |_| direction.update(|value| *value = if *value == rustify_ui::Direction::LeftToRight { rustify_ui::Direction::RightToLeft } else { rustify_ui::Direction::LeftToRight })>"RTL"</button>
            <button type="button" data-testid="slider-apply" on:click=move |_| { if let Some(next) = pending.get_untracked() { values.set(next); pending.set(None); } }>"Apply pending"</button>
            <button type="button" data-testid="slider-remove-left" on:click=move |_| {
                values.update(|values| values.retain(|item| item.id != "left")); items.update(|items| items.retain(|item| *item != "left"));
            }>"Remove left"</button>
            <form id="slider-form" on:submit=move |event| event.prevent_default()>
                <p::DirectionProvider direction=direction>
                    <p::slider::Root values=values name="range" test_id="slider-group" min_steps_between=5_u32
                        preserve_order=preserve step=Signal::derive(move || if bad_step.get() { 0.0 } else { 1.0 })
                        disabled=disabled read_only=readonly on_change={move |next: Vec<p::slider::ThumbValue>| {
                            requests.update(|count| *count += 1); last.set(encode(&next));
                            if preview_start.get_untracked().is_none() { preview_start.set(Some(values.get_untracked())); }
                            match policy.get_untracked().as_str() { "refuse" => {}, "delay" => pending.set(Some(next)), _ => values.set(next) }
                        }}
                        on_commit=Callback::new(move |next: Vec<p::slider::ThumbValue>| { commits.update(|count| *count += 1); committed.set(encode(&next)); preview_start.set(None); })
                        on_cancel=Callback::new(move |()| { cancels.update(|count| *count += 1); if let Some(mut before) = preview_start.get_untracked() { let declared = items.get_untracked(); before.retain(|item| declared.contains(&item.id.as_str())); values.set(before); preview_start.set(None); } })>
                        <p::slider::Track class="sample-track"><p::slider::Range class="sample-range" />
                            <For each=move || items.get() key=|item| *item let:item>
                                <p::slider::Thumb id=item aria_label=item class="sample-thumb" test_id=format!("slider-{item}")
                                    disabled=Signal::derive(move || item == "left" && left_disabled.get()) />
                            </For>
                        </p::slider::Track>
                    </p::slider::Root>
                </p::DirectionProvider>
            </form>
            <p::slider::Root values=vec![p::slider::ThumbValue { id: "vertical".into(), value: 35.0 }] on_change={move |next: Vec<p::slider::ThumbValue>| vertical_asked.set(encode(&next))}
                orientation=Orientation::Vertical test_id="slider-vertical">
                <p::slider::Track class="sample-vertical"><p::slider::Range class="sample-range" />
                    <p::slider::Thumb id="vertical" aria_label="Vertical amount" class="sample-thumb" test_id="slider-vertical-thumb" />
                </p::slider::Track>
            </p::slider::Root>
            <p::slider::Root values=vec![p::slider::ThumbValue { id: "vertical".into(), value: 35.0 }] on_change={move |next: Vec<p::slider::ThumbValue>| vertical_asked.set(encode(&next))}
                orientation=Orientation::Vertical inverted=true test_id="slider-vertical-inverted">
                <p::slider::Track class="sample-vertical"><p::slider::Range class="sample-range" />
                    <p::slider::Thumb id="vertical" aria_label="Inverted vertical amount" class="sample-thumb" test_id="slider-inverted-thumb" />
                </p::slider::Track>
            </p::slider::Root>
            <output data-testid="slider-vertical-last">{move || vertical_asked.get()}</output>
            <output data-testid="slider-values">{move || encode(&values.get())}</output>
            <output data-testid="slider-requests">{move || requests.get()}</output>
            <output data-testid="slider-last">{move || last.get()}</output>
            <output data-testid="slider-commits">{move || commits.get()}</output>
            <output data-testid="slider-committed">{move || committed.get()}</output>
            <output data-testid="slider-cancels">{move || cancels.get()}</output>
        </section>
    }
}
