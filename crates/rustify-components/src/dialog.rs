//! Themed dialog parts and the existing title-bar convenience composition.
use leptos::prelude::*;

const PANEL: &str = "relative w-full max-w-lg rounded-lg border border-border text-popover-foreground p-6 shadow-lg flex flex-col gap-4";
const TITLE_BAR: &str = "flex items-start gap-4";
const CLOSE: &str = "-me-2 -mt-2 shrink-0 rounded-sm p-1 bg-transparent border-0 text-muted-foreground transition-colors cursor-pointer outline-none hover:bg-muted focus-visible:ring-ring/50 focus-visible:ring-[3px]";

#[component]
pub fn Dialog(
    #[prop(into)] open: Signal<bool>,
    on_open_change: impl Fn(bool) + Send + Sync + 'static,
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] description: Signal<String>,
    #[prop(default = true)] modal: bool,
    #[prop(default = true)] close_on_backdrop: bool,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] close_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let change = Callback::new(on_open_change);
    let locale = rustify_ui::use_locale();
    let close_label = StoredValue::new((!close_label.is_empty()).then_some(close_label));
    let close_name = move || {
        close_label
            .get_value()
            .unwrap_or_else(|| locale.text(rustify_ui::Message::Close).to_string())
    };
    let close_id = if test_id.is_empty() {
        "dialog-close".into()
    } else {
        format!("{test_id}-close")
    };
    let close_id = StoredValue::new(close_id);
    let children = StoredValue::new(children);
    let outside = Callback::new(
        move |event: rustify_ui::Cancelable<rustify_ui::LayerDismiss>| {
            if !close_on_backdrop {
                event.prevent_default();
            }
        },
    );
    view! {<Root open=open on_open_change=change modal=modal>
        <Portal class="rui-dialog-layer" test_id=test_id>
            <Show when=move ||modal><Overlay attr:data-name="DialogBackdrop" /></Show>
            <Content align=align on_pointer_down_outside=outside class=crate::macros::merge(PANEL,&class) attr:data-name="Dialog">
                <header class=TITLE_BAR data-name="DialogTitleBar">
                    <div class="flex flex-1 flex-col gap-2">
                        <Title class="m-0 text-lg leading-none font-semibold text-popover-foreground">{move ||title.get()}</Title>
                        <Show when=move ||!description.get().is_empty()><Description class="m-0 text-sm text-muted-foreground">{move ||description.get()}</Description></Show>
                    </div>
                    {close_as(leptos::html::button().class(CLOSE).attr("data-name","DialogClose").attr("data-testid",close_id.get_value())
                        .attr("aria-label",close_name).attr("title",close_name).child(view!{<crate::icon::Icon glyph=crate::icon::Glyph::Close />}))}
                </header>
                {move ||children.with_value(|children|children())}
            </Content>
        </Portal>
    </Root>}
}

pub use crate::primitives::dialog::{Portal, Root};
pub use rustify_ui::ViewportAlign as Align;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogSize {
    One,
    Two,
    #[default]
    Three,
    Four,
}
impl DialogSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three, Self::Four];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<DialogSize> for Signal<crate::Responsive<DialogSize>> {
    fn from(value: DialogSize) -> Self {
        Signal::stored(value.into())
    }
}
#[component]
pub fn Content(
    #[prop(optional, into)] size: Signal<crate::Responsive<DialogSize>>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] layout: Signal<crate::Layout>,
    #[prop(optional, into)] margin: Signal<crate::Margin>,
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
    let theme = rustify_ui::use_resolved_theme();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        if let Some(theme) = theme {
            styles.set_floating_surface(&theme.get())?;
        }
        for (width, size) in size.get().iter() {
            let (space, padding, radius, pixels) = match size {
                DialogSize::One => (3, 12, 4, 8),
                DialogSize::Two => (4, 16, 4, 8),
                DialogSize::Three => (5, 24, 5, 12),
                DialogSize::Four => (6, 32, 5, 12),
            };
            styles.insert(
                width,
                "--rustify-dialog-padding",
                format!("var(--rustify-space-{space}, {padding}px)"),
            )?;
            styles.insert(
                width,
                "--rustify-dialog-radius",
                format!("var(--rustify-radius-{radius}, {pixels}px)"),
            )?;
        }
        let layout = layout.get();
        for (property, variable, value) in [
            ("width", "--rustify-dialog-width", &layout.width),
            ("min-width", "--rustify-dialog-min-width", &layout.min_width),
            ("max-width", "--rustify-dialog-max-width", &layout.max_width),
            ("height", "--rustify-dialog-height", &layout.height),
            (
                "min-height",
                "--rustify-dialog-min-height",
                &layout.min_height,
            ),
            (
                "max-height",
                "--rustify-dialog-max-height",
                &layout.max_height,
            ),
        ] {
            styles.set_dimension_variable(property, variable, value)?;
        }
        margin.get().write(&mut styles)?;
        Ok(styles)
    });
    view! {<crate::primitives::dialog::Content align=align on_escape_key_down=on_escape_key_down on_pointer_down_outside=on_pointer_down_outside
    on_focus_outside=on_focus_outside on_interact_outside=on_interact_outside on_open_auto_focus=on_open_auto_focus on_close_auto_focus=on_close_auto_focus
    class=crate::macros::merge(format!("rustify-dialog-content {geometry}"),&class) test_id=test_id>{children()}</crate::primitives::dialog::Content>}
}
#[component]
pub fn Overlay(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<crate::primitives::dialog::Overlay class=crate::macros::merge("rustify-dialog-overlay",&class) test_id=test_id />}
}
#[component]
pub fn Title(
    #[prop(default=crate::HeadingLevel::H2)] level: crate::HeadingLevel,
    #[prop(optional, into)] margin: Signal<crate::Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        margin.get().write(&mut styles)?;
        Ok(styles)
    });
    view! {<crate::primitives::dialog::Title level=level class=crate::macros::merge(format!("rustify-dialog-title {geometry}"),&class) test_id=test_id>{children()}</crate::primitives::dialog::Title>}
}
#[component]
pub fn Description(
    #[prop(optional, into)] margin: Signal<crate::Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        margin.get().write(&mut styles)?;
        Ok(styles)
    });
    view! {<crate::primitives::dialog::Description class=crate::macros::merge(format!("rustify-dialog-description {geometry}"),&class) test_id=test_id>{children()}</crate::primitives::dialog::Description>}
}
macro_rules! dialog_button {
    ($name:ident) => {
        #[component]
        pub fn $name(#[prop(optional,into)] class:String,#[prop(optional,into)] test_id:String,children:Children)->impl IntoView {
            view!{<crate::primitives::dialog::$name class=crate::macros::merge("rustify-dialog-button",&class) test_id=test_id>{children()}</crate::primitives::dialog::$name>}
        }
    };
}
dialog_button!(Trigger);
dialog_button!(Close);
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    crate::primitives::dialog::trigger_as(target.add_any_attr(
        crate::primitives::slot::class_token("rustify-dialog-button".into()),
    ))
}
pub fn close_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    crate::primitives::dialog::close_as(target.add_any_attr(crate::primitives::slot::class_token(
        "rustify-dialog-button".into(),
    )))
}
