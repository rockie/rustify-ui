//! A link that knows whether it points at the page you are on.
//!
//! Rust/UI's version asked `leptos_router` where it was. This one asks this
//! crate, which the SDK's router will tell in M4 and which an application
//! without one never sets - a link with no current path is simply never
//! current, and that is the right answer for a page that does not navigate.

use leptos::prelude::*;

const BASE: &str = "text-primary underline-offset-4 rounded-sm hover:underline outline-none focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-disabled:pointer-events-none aria-disabled:opacity-50";

/// How much of the current path a link needs to match to count as current.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkMatch {
    /// The page it points at, or any page under it. A link to `/objects`
    /// stays marked while the page shows `/objects/17`.
    #[default]
    Within,
    /// That page and no other.
    Exact,
}

#[component]
pub fn Link(
    #[prop(into)] href: String,
    #[prop(optional)] match_: LinkMatch,
    /// A link with nowhere to go keeps its place in the text and loses its
    /// href: `<a>` has no disabled attribute, and a link that still navigates
    /// while looking disabled is worse than one that looks ordinary.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let path = crate::current_path();
    let target = href.clone();
    let current = move || {
        let path = path.as_ref()?.get();
        let matches = match match_ {
            LinkMatch::Within => crate::is_within(&path, &target),
            LinkMatch::Exact => path.trim_end_matches('/') == target.trim_end_matches('/'),
        };
        matches.then_some("page")
    };
    view! {
        <a
            class=crate::macros::merge(BASE, &class)
            data-name="Link"
            data-testid=test_id
            href=move || (!disabled.get()).then(|| href.clone())
            aria-current=current
            aria-disabled=move || disabled.get().then_some("true")
        >
            {children()}
        </a>
    }
}
