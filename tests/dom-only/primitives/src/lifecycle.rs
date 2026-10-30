//! Conditional field ownership and recoverable part composition.
use leptos::prelude::*;
use rustify_components::{primitives as p, Form};

#[component]
pub fn Lifecycle() -> impl IntoView {
    let visible = RwSignal::new(true);
    let duplicate = RwSignal::new(false);
    let mounted = RwSignal::new(true);
    let missing = RwSignal::new(false);
    let value = RwSignal::new("title".to_string());
    let decimals = RwSignal::new(vec![
        p::slider::ThumbValue {
            id: "low".into(),
            value: 100.2,
        },
        p::slider::ThumbValue {
            id: "high".into(),
            value: 100.3,
        },
    ]);
    let form = Form::new(&["title"], || {});
    let legacy = Form::new(&["title"], || {});
    view! {
        <section data-testid="lifecycle">
            <p::slider::Root values=decimals on_change=move |v| decimals.set(v) min=100.0 max=200.0 step=0.1 min_steps_between=1>
                <p::slider::Track><p::slider::Thumb id="low" aria_label="Decimal low" test_id="decimal-low" /><p::slider::Thumb id="high" aria_label="Decimal high" /></p::slider::Track>
            </p::slider::Root>
            <button data-testid="input-toggle" on:click=move |_| visible.update(|v| *v = !*v)>"Toggle input"</button>
            <button data-testid="input-duplicate" on:click=move |_| duplicate.update(|v| *v = !*v)>"Toggle second input"</button>
            <button data-testid="control-toggle" on:click=move |_| mounted.update(|v| *v = !*v)>"Replace control"</button>
            <button data-testid="missing-toggle" on:click=move |_| missing.update(|v| *v = !*v)>"Show orphan parts"</button>
            <p::form::Root form=form rules=move || vec![("title", "Required".into())] id="lifecycle-form">
                <p::form::Field form=form field="title">
                    <p::form::Label test_id="lifecycle-label">"Explicit title"</p::form::Label>
                    <Show when=move || mounted.get()>
                        <p::text_field::Root id="explicit-title" value=value on_change=move |v| value.set(v) name="title">
                            <Show when=move || visible.get()><p::text_field::Input test_id="lifecycle-input" /></Show>
                            <Show when=move || duplicate.get()><p::text_field::Input test_id="lifecycle-second" /></Show>
                        </p::text_field::Root>
                    </Show>
                </p::form::Field>
                <button type="submit" data-testid="lifecycle-submit">"Validate"</button>
            </p::form::Root>
            <p::form::Field form=form field="implicit">
                <p::form::Label test_id="implicit-label">"Implicit title"</p::form::Label>
                <Show when=move || mounted.get()><p::text_field::Root value=value on_change=move |v| value.set(v)>
                    <p::text_field::Input test_id="implicit-input" />
                </p::text_field::Root></Show>
            </p::form::Field>
            <p::form::Field form=legacy field="title">
                <p::form::Label>"Legacy title"</p::form::Label>
                <p::text_field::Root id="legacy-title" value=value on_change=move |v| value.set(v)>
                    <p::text_field::Input test_id="legacy-input" />
                </p::text_field::Root>
            </p::form::Field>
            <button data-testid="legacy-submit" on:click=move |_| { legacy.submit(|| vec![("title", "Required".into())]); }>"Validate legacy"</button>
            <Show when=move || missing.get()>
                <div data-testid="orphan-parts">
                    <p::select::Value /><p::select::Content><span>"Content child"</span></p::select::Content>
                    <p::select::Viewport><span>"Viewport child"</span></p::select::Viewport>
                    <p::select::ItemText>"Text child"</p::select::ItemText>
                    <p::select::ItemIndicator>"Indicator child"</p::select::ItemIndicator>
                    <p::select::ScrollUpButton>"Up child"</p::select::ScrollUpButton>
                    <p::select::ScrollDownButton>"Down child"</p::select::ScrollDownButton>
                    <p::form::Description>"Description child"</p::form::Description><p::form::Message />
                </div>
            </Show>
        </section>
    }
}
