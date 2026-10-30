//! A typed, single native target; event behavior belongs to the owning Root.

use leptos::prelude::*;

/// Appends part attributes to exactly one native element without a wrapper.
///
/// The target retains its content and listeners. Parts own reserved attributes
/// such as role, ID and ARIA relationships; pass these through `attrs` rather
/// than declaring competing versions on the target. `attrs` must not contain
/// target listeners: a Root handles the bubbled event after the caller.
///
/// ```
/// use leptos::prelude::*;
/// use rustify_components::primitives::Slot;
/// let _ = Slot(leptos::html::button().child("Save"),
///     leptos::attr::custom::custom_attribute("data-rustify-part", "trigger"));
/// ```
///
/// Fragments and erased views cannot prove that there is one native target:
///
/// ```compile_fail
/// use leptos::prelude::*;
/// use rustify_components::primitives::Slot;
/// Slot((leptos::html::button(), leptos::html::a()), ());
/// ```
///
/// ```compile_fail
/// use leptos::prelude::*;
/// use rustify_components::primitives::Slot;
/// Slot(view! { <button>"Save"</button> }.into_any(), ());
/// ```
#[allow(non_snake_case)]
pub fn Slot<E, At, Ch, A>(target: leptos::html::HtmlElement<E, At, Ch>, attrs: A) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
    A: leptos::attr::Attribute + Send,
{
    target.add_any_attr(attrs)
}

// Leptos only provides additive class directives for static names. Geometry
// allocates owned names; keep them additive so a typed target retains its class.
#[derive(Clone)]
struct ClassToken(String);
impl leptos::tachys::html::class::IntoClass for ClassToken {
    type AsyncOutput = Self;
    type State = (leptos::tachys::renderer::types::ClassList, String);
    type Cloneable = Self;
    type CloneableOwned = Self;
    fn html_len(&self) -> usize {
        self.0.len()
    }
    fn to_html(self, class: &mut String) {
        class.push_str(&self.0);
    }
    fn hydrate<const FROM_SERVER: bool>(
        self,
        el: &leptos::tachys::renderer::types::Element,
    ) -> Self::State {
        let list = leptos::tachys::renderer::Rndr::class_list(el);
        if !FROM_SERVER {
            leptos::tachys::renderer::Rndr::add_class(&list, &self.0);
        }
        (list, self.0)
    }
    fn build(self, el: &leptos::tachys::renderer::types::Element) -> Self::State {
        self.hydrate::<false>(el)
    }
    fn rebuild(self, state: &mut Self::State) {
        if self.0 != state.1 {
            leptos::tachys::renderer::Rndr::remove_class(&state.0, &state.1);
            leptos::tachys::renderer::Rndr::add_class(&state.0, &self.0);
            state.1 = self.0;
        }
    }
    fn into_cloneable(self) -> Self {
        self
    }
    fn into_cloneable_owned(self) -> Self {
        self
    }
    fn dry_resolve(&mut self) {}
    async fn resolve(self) -> Self {
        self
    }
    fn reset(state: &mut Self::State) {
        leptos::tachys::renderer::Rndr::remove_class(&state.0, &state.1);
    }
}
pub(crate) fn class_token(name: String) -> impl leptos::attr::Attribute {
    leptos::tachys::html::class::class(ClassToken(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decorating_the_target_does_not_invoke_its_listener() {
        use std::sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        };
        let called = Arc::new(AtomicBool::new(false));
        let listener_called = called.clone();
        let _ = Slot(
            leptos::html::button()
                .on(leptos::ev::click, move |_| {
                    listener_called.store(true, Ordering::Relaxed);
                })
                .child("Save"),
            leptos::attr::custom::custom_attribute("data-rustify-part", "trigger"),
        );
        assert!(!called.load(Ordering::Relaxed));
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn native_target_content_and_attributes_survive_without_a_wrapper() {
        let html = Slot(
            leptos::html::a()
                .attr("href", "/settings")
                .child("Settings"),
            leptos::attr::custom::custom_attribute("data-rustify-part", "trigger"),
        )
        .to_html();
        assert!(html.starts_with("<a "), "{html}");
        assert!(html.contains("href=\"/settings\""), "{html}");
        assert!(html.contains("data-rustify-part=\"trigger\""), "{html}");
        assert!(html.ends_with("Settings</a>"), "{html}");
    }
}
