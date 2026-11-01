//! Progress values stay in their original units; unknown work has no percentage.
use leptos::prelude::*;
mod model;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressValue {
    pub value: Option<f64>,
    pub max: f64,
}
#[derive(Clone, Copy)]
struct Context(Memo<model::State>);
#[component]
pub fn Root(
    #[prop(optional, into)] value: Signal<Option<f64>>,
    #[prop(default=Signal::stored(100.),into)] max: Signal<f64>,
    #[prop(optional_no_strip, into)] value_label: Option<Callback<ProgressValue, String>>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view=owner.with(move || {
        let state=Memo::new(move |_|model::State::new(value.get(),max.get()));
        provide_context(Context(state));
        view! {<div class=crate::macros::reactive_merge("",&class) data-rustify-part="progress-root" data-testid=test_id
            role="progressbar" aria-label=(!aria_label.is_empty()).then_some(aria_label)
            aria-labelledby=(!labelled_by.is_empty()).then_some(labelled_by)
            aria-valuemin="0" aria-valuemax=move ||state.get().value.max.to_string()
            aria-valuenow=move ||state.get().value.value.map(|value|value.to_string())
            aria-valuetext=move ||value_label.and_then(|label|label.try_run(state.get().value))
            data-state=move ||state.get().state() data-value=move ||state.get().value.value.map(|value|value.to_string())
            data-max=move ||state.get().value.max.to_string() data-rustify-issue=move ||state.get().issue>
            {children()}
        </div>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn Indicator(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<Context>();
    let state =
        move || context.map_or_else(|| model::State::new(None, 100.), |context| context.0.get());
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        if let Some(percent) = state().percent() {
            styles.insert(0, "width", format!("{percent}%"))?;
        }
        Ok(styles)
    });
    view! {<div class=crate::macros::merge(geometry,&class) data-rustify-part="progress-indicator" data-testid=test_id
        data-state=move ||state().state() data-value=move ||state().value.value.map(|value|value.to_string()) data-max=move ||state().value.max.to_string()
        aria-hidden=context.is_some().then_some("true") data-rustify-issue=context.is_none().then_some("missing-root")>
        {children.map(|children|children())}
    </div>}
}
