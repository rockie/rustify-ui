/// Declares a component that is a styled element and nothing else.
///
/// The base classes are the component's; the `class` prop is the caller's, and
/// the two are merged so that a caller's `bg-*` replaces the base `bg-*`
/// instead of fighting it in the cascade. `data-name` carries the component's
/// own name and `data-testid` whatever the caller wants to find it by, so a
/// test never has to address one of these by its classes.
///
/// ```ignore
/// clx! {Card, div, "rui:rounded-lg rui:p-4", "rui:bg-card"}
///
/// view! {
///     <Card>"the base classes"</Card>
///     <Card class="rui:bg-muted">"which the caller can replace"</Card>
/// }
/// ```
#[macro_export]
macro_rules! clx {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[::leptos::component]
        pub fn $name(
            #[prop(into, optional)] class: String,
            #[prop(into, optional)] test_id: String,
            children: ::leptos::prelude::Children,
        ) -> impl ::leptos::prelude::IntoView {
            use ::leptos::prelude::*;

            let merged = $crate::macros::merge(
                ::tw_merge::tw_join!($($base_class),+),
                &class,
            );

            ::leptos::view! {
                <$element
                    class=merged
                    data-name=stringify!($name)
                    data-testid=test_id
                >
                    {children()}
                </$element>
            }
        }
    };
}

/// The same, for an element that takes no children.
///
/// <https://developer.mozilla.org/en-US/docs/Glossary/Void_element>
#[macro_export]
macro_rules! void {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[::leptos::component]
        pub fn $name(
            #[prop(into, optional)] class: String,
            #[prop(into, optional)] test_id: String,
        ) -> impl ::leptos::prelude::IntoView {
            use ::leptos::prelude::*;

            let merged = $crate::macros::merge(
                ::tw_merge::tw_join!($($base_class),+),
                &class,
            );

            ::leptos::view! {
                <$element
                    class=merged
                    data-name=stringify!($name)
                    data-testid=test_id
                />
            }
        }
    };
}
