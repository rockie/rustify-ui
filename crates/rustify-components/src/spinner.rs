//! Named loading indicators that preserve children and native target identity.
use crate::{Margin, Responsive};
use leptos::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    One,
    #[default]
    Two,
    Three,
}
impl SpinnerSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<SpinnerSize> for Signal<Responsive<SpinnerSize>> {
    fn from(value: SpinnerSize) -> Self {
        Signal::stored(value.into())
    }
}
fn geometry(size: Signal<Responsive<SpinnerSize>>, margin: Signal<Margin>) -> String {
    crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (width, size) in size.get().iter() {
            styles.insert(
                width,
                "--rustify-spinner-size",
                match size {
                    SpinnerSize::One => "var(--rustify-space-3, 12px)",
                    SpinnerSize::Two => "var(--rustify-space-4, 16px)",
                    SpinnerSize::Three => "calc(var(--rustify-space-4, 16px) * 1.25)",
                },
            )?;
        }
        margin.get().write(&mut styles)?;
        Ok(styles)
    })
}
/// Decorates a complete native target inside a flow-content parent and adds
/// its status sibling. For table rows/cells use Skeleton or a legal container.
pub fn spinner_as<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    loading: impl Into<Signal<bool>>,
    label: impl Into<Signal<String>>,
    size: impl Into<Signal<Responsive<SpinnerSize>>>,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    let geometry = geometry(size.into(), Signal::default());
    let class = format!("rustify-spinner {geometry}");
    crate::primitives::spinner::with_target(
        target.add_any_attr((
            crate::primitives::slot::class_token("rustify-spinner".into()),
            crate::primitives::slot::class_token(geometry),
        )),
        loading.into(),
        label.into(),
        class,
    )
}
#[component]
pub fn Spinner(
    #[prop(default=Signal::stored(true),into)] loading: Signal<bool>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] size: Signal<Responsive<SpinnerSize>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let geometry = geometry(size, margin);
    crate::primitives::spinner::Root(crate::primitives::spinner::RootProps {
        loading,
        label,
        class: crate::macros::merge(format!("rustify-spinner {geometry}"), &class),
        test_id,
        children,
    })
}
pub use Spinner as Root;
