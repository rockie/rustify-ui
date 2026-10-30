//! The native toggle skin; its controlled behavior lives in Primitives.
use crate::primitives;
use leptos::prelude::*;
const ITEM: &str = "inline-flex h-9 min-w-9 items-center justify-center gap-2 rounded-md px-3 text-sm font-medium whitespace-nowrap text-foreground transition-colors outline-none cursor-pointer select-none hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-pressed:bg-primary aria-pressed:text-primary-foreground aria-pressed:hover:bg-primary/90";
#[component]
pub fn Root(
    #[prop(into)] pressed: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(default = "on".into(), into)] value: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let classes = crate::macros::reactive_merge_with(move || vec![ITEM.into(), class.get()]);
    view! { <primitives::toggle::Root pressed=pressed required=required on_change=on_change id=id name=name value=value form=form disabled=disabled
        read_only=read_only invalid=invalid described_by=described_by aria_label=aria_label labelled_by=labelled_by class=Signal::derive(classes) test_id=test_id>
        {children()}
    </primitives::toggle::Root> }
}
pub use Root as Toggle;
pub(crate) fn item_class(class: Signal<String>) -> Signal<String> {
    Signal::derive(crate::macros::reactive_merge_with(move || {
        vec![ITEM.into(), class.get()]
    }))
}
