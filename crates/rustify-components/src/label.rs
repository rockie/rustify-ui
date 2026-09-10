//! Text that names a control.

use leptos::prelude::*;

const BASE: &str = "rui:flex rui:items-center rui:gap-2 rui:text-sm rui:leading-none rui:font-medium rui:select-none rui:text-foreground";

/// Names the control whose id is `control`, so that clicking the text moves
/// focus there and a screen reader announces the two together.
///
/// Rust/UI's version generated `peer-disabled` classes from the `for`
/// attribute, which produced a class Tailwind had never seen and therefore
/// never compiled. The disabled look comes from the control's own state here.
#[component]
pub fn Label(
    #[prop(optional, into)] control: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let control = (!control.is_empty()).then_some(control);
    view! {
        <label
            class=crate::macros::merge(BASE, &class)
            data-name="Label"
            data-testid=test_id
            r#for=control
        >
            {children()}
        </label>
    }
}
