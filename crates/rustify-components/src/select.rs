//! Themed parts and the legacy convenience view share one select-only combobox.
use crate::{primitives, Responsive};
use leptos::prelude::*;
pub use primitives::select::SelectEntry;
use rustify_ui::{AccentColor, Cancelable, LayerDismiss, ThemeRadius};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectSize {
    One,
    #[default]
    Two,
    Three,
}
impl SelectSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }
}
impl From<SelectSize> for Signal<Responsive<SelectSize>> {
    fn from(value: SelectSize) -> Self {
        Self::stored(value.into())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectTriggerVariant {
    Classic,
    #[default]
    Surface,
    Soft,
    GhostOffset,
    Ghost,
}
impl SelectTriggerVariant {
    pub const ALL: &'static [Self] = &[
        Self::Classic,
        Self::Surface,
        Self::Soft,
        Self::GhostOffset,
        Self::Ghost,
    ];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Surface => "surface",
            Self::Soft => "soft",
            Self::GhostOffset => "ghost-offset",
            Self::Ghost => "ghost",
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectContentVariant {
    #[default]
    Solid,
    Soft,
}
impl SelectContentVariant {
    pub const ALL: &'static [Self] = &[Self::Solid, Self::Soft];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Soft => "soft",
        }
    }
}
#[derive(Clone, Copy)]
struct Context {
    size: Signal<Responsive<SelectSize>>,
}
fn size() -> Signal<Responsive<SelectSize>> {
    use_context::<Context>().map_or_else(
        || Signal::stored(Responsive::default()),
        |context| context.size,
    )
}
fn geometry(
    size: Signal<Responsive<SelectSize>>,
    color: Signal<Option<AccentColor>>,
    radius: Signal<Option<ThemeRadius>>,
    content: bool,
) -> String {
    let mode = crate::component_color::use_mode();
    crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (width, size) in size.get().iter() {
            let n = size.number();
            let (height, padding, gap, radius_step, fallback_height, fallback_font, fallback_line) =
                match n {
                    1 => (5, 2, "var(--rustify-space-1, 4px)", 1, 24, 12, 16),
                    3 => (7, 4, "var(--rustify-space-2, 8px)", 3, 40, 16, 24),
                    _ => (
                        6,
                        3,
                        "calc(var(--rustify-space-1, 4px) * 1.5)",
                        2,
                        32,
                        14,
                        20,
                    ),
                };
            styles.insert(
                width,
                "--rustify-select-height",
                format!("var(--rustify-space-{height}, {fallback_height}px)"),
            )?;
            styles.insert(
                width,
                "--rustify-select-padding",
                format!("var(--rustify-space-{padding}, {}px)", padding * 4),
            )?;
            styles.insert(width, "--rustify-select-gap", gap)?;
            styles.insert(
                width,
                "--rustify-select-ghost-x",
                if n == 3 {
                    "var(--rustify-space-3, 12px)"
                } else {
                    "var(--rustify-space-2, 8px)"
                },
            )?;
            styles.insert(
                width,
                "--rustify-select-ghost-y",
                if n == 3 {
                    "calc(var(--rustify-space-1, 4px) * 1.5)"
                } else {
                    "var(--rustify-space-1, 4px)"
                },
            )?;
            styles.insert(
                width,
                "--rustify-select-font",
                format!("var(--rustify-font-size-{n}, {fallback_font}px)"),
            )?;
            styles.insert(
                width,
                "--rustify-select-line",
                format!("var(--rustify-line-height-{n}, {fallback_line}px)"),
            )?;
            styles.insert(
                width,
                "--rustify-select-radius-base",
                format!(
                    "var(--rustify-radius-{radius_step}, {}px)",
                    match radius_step {
                        1 => 3,
                        3 => 6,
                        _ => 4,
                    }
                ),
            )?;
            styles.insert(
                width,
                "--rustify-select-radius-unit",
                format!(
                    "calc(var(--rustify-unit, 4px) * {})",
                    match n {
                        1 => 0.75,
                        3 => 1.5,
                        _ => 1.0,
                    }
                ),
            )?;
            if content {
                styles.insert(
                    width,
                    "--rustify-select-content-padding",
                    if n == 1 {
                        "var(--rustify-space-1, 4px)"
                    } else {
                        "var(--rustify-space-2, 8px)"
                    },
                )?;
                styles.insert(
                    width,
                    "--rustify-select-item-height",
                    if n == 1 {
                        "var(--rustify-space-5, 24px)"
                    } else {
                        "var(--rustify-space-6, 32px)"
                    },
                )?;
                styles.insert(
                    width,
                    "--rustify-select-indicator-width",
                    if n == 1 {
                        "calc(var(--rustify-space-5, 24px) / 1.2)"
                    } else {
                        "var(--rustify-space-5, 24px)"
                    },
                )?;
                styles.insert(
                    width,
                    "--rustify-select-content-radius",
                    if n == 1 {
                        "var(--rustify-radius-3, 6px)"
                    } else {
                        "var(--rustify-radius-4, 8px)"
                    },
                )?;
                styles.insert(
                    width,
                    "--rustify-select-item-radius",
                    if n == 1 {
                        "var(--rustify-radius-1, 3px)"
                    } else {
                        "var(--rustify-radius-2, 4px)"
                    },
                )?;
                styles.insert(
                    width,
                    "--rustify-select-line",
                    if n == 1 {
                        "var(--rustify-line-height-1, 16px)"
                    } else {
                        "var(--rustify-line-height-2, 20px)"
                    },
                )?;
                styles.insert(
                    width,
                    "--rustify-select-label-font",
                    if n == 1 {
                        "var(--rustify-font-size-1, 12px)"
                    } else {
                        "var(--rustify-font-size-2, 14px)"
                    },
                )?;
            }
        }
        crate::component_color::set_color(&mut styles, color.get(), mode.get())?;
        if let Some(radius) = radius.get() {
            styles.insert(
                0,
                "--rustify-select-radius",
                if radius == ThemeRadius::Full {
                    "9999px".into()
                } else {
                    format!(
                        "calc(var(--rustify-select-radius-unit, 4px) * {})",
                        radius.factor()
                    )
                },
            )?;
        }
        Ok(styles)
    })
}

/// Select state and metadata, with the size inherited by its themed parts.
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>>,
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(into)] entries: Signal<Vec<SelectEntry>>,
    #[prop(optional_no_strip, into)] on_select: Option<Callback<Cancelable<String>>>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
        Callback<Cancelable<LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_open_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional_no_strip, into)] on_close_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] size: Signal<Responsive<SelectSize>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        provide_context(Context { size });
        view! { <primitives::select::Root value=value on_change=on_change open=open on_open_change=on_open_change entries=entries
            on_select=on_select on_escape_key_down=on_escape_key_down on_pointer_down_outside=on_pointer_down_outside
            on_open_auto_focus=on_open_auto_focus on_close_auto_focus=on_close_auto_focus
            id=id name=name form=form placeholder=placeholder aria_label=aria_label labelled_by=labelled_by described_by=described_by
            disabled=disabled read_only=read_only invalid=invalid required=required class=class>{children()}</primitives::select::Root> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] variant: Signal<SelectTriggerVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let geometry = geometry(size(), color, radius, false);
    let children = children.unwrap_or_else(|| {
        Box::new(move || view! { <Value placeholder=placeholder /><Icon /> }.into_any())
    });
    view! { <primitives::select::Trigger class=format!("rustify-color rustify-select-trigger {geometry} {class}") test_id=test_id>{children()}</primitives::select::Trigger> }
        .add_any_attr((leptos::attr::custom::custom_attribute("data-name", "Select"), leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str())))
}
/// The default trigger skin on one caller-owned button, retaining host classes/listeners.
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    let geometry = geometry(size(), Signal::stored(None), Signal::stored(None), false);
    primitives::select::trigger_as(target).add_any_attr((
        leptos::tachys::html::class::class(("rustify-color", true)),
        leptos::tachys::html::class::class(("rustify-select-trigger", true)),
        primitives::slot::class_token(geometry),
        leptos::attr::custom::custom_attribute("data-variant", "surface"),
    ))
}
#[component]
pub fn Value(
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! { <primitives::select::Value placeholder=placeholder class=format!("rustify-select-value {class}") /> }
}
#[component]
pub fn Icon(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! { <primitives::select::Icon class=format!("rustify-select-icon {class}")>{children.map(|children| children()).unwrap_or_else(|| view! { <crate::Icon glyph=crate::Glyph::ChevronDown /> }.into_any())}</primitives::select::Icon> }
}
#[component]
pub fn Content(
    #[prop(optional, into)] variant: Signal<SelectContentVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let geometry = geometry(size(), color, Signal::stored(None), true);
    view! { <primitives::select::Content class=format!("rustify-color rustify-select-content {geometry} {class}") test_id=test_id>{children()}</primitives::select::Content> }
        .add_any_attr((leptos::attr::custom::custom_attribute("data-variant", move || variant.get().as_str()),
            leptos::attr::custom::custom_attribute("data-high-contrast", move || high_contrast.get().then_some(""))))
}
#[component]
pub fn Viewport(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <primitives::select::Viewport class=format!("rustify-select-viewport {class}")>{children()}</primitives::select::Viewport> }
}
#[component]
pub fn Group(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <primitives::select::Group class=format!("rustify-select-group {class}")>{children()}</primitives::select::Group> }
}
#[component]
pub fn Label(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <primitives::select::Label class=format!("rustify-select-label {class}")>{children()}</primitives::select::Label> }
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let children =
        children.unwrap_or_else(|| Box::new(|| view! { <ItemText /><ItemIndicator /> }.into_any()));
    view! { <primitives::select::Item value=value class=format!("rustify-select-item {class}") test_id=test_id>{children()}</primitives::select::Item> }
}
pub fn item_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Div, At, Ch>,
    value: impl Into<String>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    primitives::select::item_as(target, value).add_any_attr(leptos::tachys::html::class::class((
        "rustify-select-item",
        true,
    )))
}
#[component]
pub fn ItemText(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Passing the option through keeps metadata text reactive when no custom children were supplied.
    primitives::select::ItemText(primitives::select::ItemTextProps {
        class: format!("rustify-select-item-text {class}"),
        children,
    })
}
#[component]
pub fn ItemIndicator(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! { <primitives::select::ItemIndicator class=format!("rustify-select-indicator {class}")>{children.map(|children| children()).unwrap_or_else(|| view! { <crate::Icon glyph=crate::Glyph::Check /> }.into_any())}</primitives::select::ItemIndicator> }
}
#[component]
pub fn Separator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitives::select::Separator class=format!("rustify-select-separator {class}") /> }
}
#[component]
pub fn ScrollUpButton(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitives::select::ScrollUpButton class=format!("rustify-select-scroll {class}")><crate::Icon glyph=crate::Glyph::ChevronUp /></primitives::select::ScrollUpButton> }
}
#[component]
pub fn ScrollDownButton(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitives::select::ScrollDownButton class=format!("rustify-select-scroll {class}")><crate::Icon glyph=crate::Glyph::ChevronDown /></primitives::select::ScrollDownButton> }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}
impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}
/// Convenience assembly. Empty strings mean no selection; non-empty values must be unique.
#[component]
pub fn Select(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] options: Signal<Vec<SelectOption>>,
    on_change: impl Fn(String) + Send + Sync + 'static,
    #[prop(into)] open: Signal<bool>,
    on_open_change: impl Fn(bool) + Send + Sync + 'static,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] size: Signal<Responsive<SelectSize>>,
    #[prop(optional, into)] variant: Signal<SelectTriggerVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] content_variant: Signal<SelectContentVariant>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let entries = Signal::derive(move || {
        options.with(|options| {
            options
                .iter()
                .map(|option| SelectEntry {
                    value: option.value.clone(),
                    text_value: option.label.clone(),
                    disabled: option.disabled,
                })
                .collect::<Vec<_>>()
        })
    });
    let value = Signal::derive(move || {
        let value = value.get();
        (!value.is_empty()).then_some(value)
    });
    let change = Callback::new(move |value: Option<String>| on_change(value.unwrap_or_default()));
    let panel_id = format!("{test_id}-list");
    let locale = rustify_ui::use_locale();
    view! { <Root value=value on_change=change open=open on_open_change=on_open_change entries=entries id=id name=name form=form
        placeholder=placeholder aria_label=aria_label labelled_by=labelled_by described_by=described_by disabled=disabled read_only=read_only invalid=invalid required=required size=size>
        <Trigger variant=variant color=color radius=radius class=format!("w-full {class}") test_id=test_id />
        <Content variant=content_variant color=color high_contrast=high_contrast test_id=panel_id>
            <ScrollUpButton /><Viewport>
                <Show when=move || entries.with(Vec::is_empty)><div class="rustify-select-label" data-rustify-part="select-empty">{move || match locale.get() { rustify_ui::Locale::English => "No options", rustify_ui::Locale::Chinese => "暂无选项" }}</div></Show>
                {move || entries.with(|entries| entries.iter().map(|entry| view! { <Item value=entry.value.clone() test_id=format!("option-{}", entry.value) /> }).collect_view())}
            </Viewport><ScrollDownButton />
        </Content>
    </Root> }
}

#[cfg(all(test, feature = "ssr", not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn host_rendering_preserves_a_disabled_select_trigger() {
        let owner = Owner::new();
        let html = owner.with(|| {
            view! {
                <Select
                    value="second"
                    options=vec![
                        SelectOption::new("first", "First"),
                        SelectOption::new("second", "Second"),
                    ]
                    on_change=|_| {}
                    open=false
                    on_open_change=|_| {}
                    disabled=true
                />
            }
            .to_html()
        });
        assert!(
            html.contains(" disabled ") || html.contains(" disabled>"),
            "{html}"
        );
    }
}
