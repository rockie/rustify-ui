//! Themed descriptions attached to one native target.
pub use crate::primitives::tooltip::{trigger_as, Portal, Provider, Root, Trigger};
use leptos::prelude::*;

#[component]
pub fn Content(
    #[prop(optional, into)] width: Signal<crate::Responsive<String>>,
    #[prop(optional, into)] min_width: Signal<crate::Responsive<String>>,
    #[prop(optional, into)] max_width: Signal<crate::Responsive<String>>,
    #[prop(optional, into)] position: Signal<rustify_ui::FloatingPosition>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_focus_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_interact_outside: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_open_auto_focus: Option<
        Callback<rustify_ui::Cancelable<()>>,
    >,
    #[prop(optional_no_strip, into)] on_close_auto_focus: Option<
        Callback<rustify_ui::Cancelable<()>>,
    >,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (property, variable, value) in [
            ("width", "--rustify-tooltip-width", width.get()),
            ("min-width", "--rustify-tooltip-min-width", min_width.get()),
            ("max-width", "--rustify-tooltip-max-width", max_width.get()),
        ] {
            styles.set_dimension_variable(property, variable, &value)?;
        }
        Ok(styles)
    });
    view! {<crate::primitives::tooltip::Content position=position
        on_escape_key_down=on_escape_key_down on_pointer_down_outside=on_pointer_down_outside
        on_focus_outside=on_focus_outside on_interact_outside=on_interact_outside
        on_open_auto_focus=on_open_auto_focus on_close_auto_focus=on_close_auto_focus
        class=crate::macros::merge(format!("rustify-tooltip-content {geometry}"),&class) test_id=test_id>
        {children()}
    </crate::primitives::tooltip::Content>}
}

#[component]
pub fn Arrow(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<crate::primitives::tooltip::Arrow class=crate::macros::merge("rustify-tooltip-arrow",&class) test_id=test_id />}
}

/// Describes the supplied native element, keeping trigger attributes on that target.
#[component]
pub fn Tooltip<E, At, Ch>(
    #[prop(into)] open: Signal<bool>,
    on_open_change: impl Fn(bool) + Send + Sync + 'static,
    #[prop(into)] label: Signal<String>,
    trigger: leptos::html::HtmlElement<E, At, Ch>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send + 'static,
    Ch: RenderHtml + Send + 'static,
{
    let class = StoredValue::new(class);
    let test_id = StoredValue::new(test_id);
    view! {<Root open=open on_open_change=Callback::new(on_open_change)>
        {trigger_as(trigger)}
        <Portal><Content class=class.get_value() test_id=test_id.get_value()>
            {move || label.get()}
            <Arrow />
        </Content></Portal>
    </Root>}
}
