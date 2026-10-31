//! Themed panel parts and a convenience tab strip sharing one controlled implementation.

use leptos::prelude::*;

/// One tab, and whether it can be reached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tab {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl Tab {
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

/// Which way the strip runs, and therefore which arrows move along it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    pub(crate) fn attribute(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// The key that moves one step along, and the one that moves one back.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn steps(self, key: &str) -> Option<i32> {
        match (self, key) {
            (Self::Horizontal, "ArrowRight") | (Self::Vertical, "ArrowDown") => Some(1),
            (Self::Horizontal, "ArrowLeft") | (Self::Vertical, "ArrowUp") => Some(-1),
            _ => None,
        }
    }
}

pub use crate::primitives::tabs::{ActivationMode, PanelEntry};

fn margin_geometry(margin: Signal<crate::Margin>) -> String {
    crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        margin.get().write(&mut styles)?;
        Ok(styles)
    })
}
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>>,
    #[prop(into)] entries: Signal<Vec<PanelEntry>>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] activation_mode: ActivationMode,
    #[prop(default = true)] loop_focus: bool,
    #[prop(optional_no_strip, into)] direction: Option<Signal<rustify_ui::Direction>>,
    #[prop(optional, into)] margin: Signal<crate::Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = margin_geometry(margin);
    view! { <crate::primitives::tabs::Root value=value on_change=on_change entries=entries orientation=orientation activation_mode=activation_mode
    loop_focus=loop_focus direction=direction class=format!("{geometry} {class}") test_id=test_id>{children()}</crate::primitives::tabs::Root> }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsSize {
    One,
    #[default]
    Two,
}
impl TabsSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two];
    pub const fn number(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
        }
    }
}
impl From<TabsSize> for Signal<crate::Responsive<TabsSize>> {
    fn from(value: TabsSize) -> Self {
        Signal::stored(value.into())
    }
}
#[component]
pub fn List(
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] size: Signal<crate::Responsive<TabsSize>>,
    #[prop(optional, into)] wrap: Signal<crate::Responsive<crate::FlexWrap>>,
    #[prop(optional, into)] justify: Signal<crate::Responsive<crate::Justify>>,
    #[prop(optional, into)] color: Signal<Option<rustify_ui::AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] margin: Signal<crate::Margin>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let mode = crate::component_color::use_mode();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (width, size) in size.get().iter() {
            let n = size.number();
            styles.insert(
                width,
                "--rustify-tab-height",
                if n == 1 {
                    "var(--rustify-space-6, 32px)"
                } else {
                    "var(--rustify-space-7, 40px)"
                },
            )?;
            styles.insert(
                width,
                "--rustify-tab-padding",
                if n == 1 {
                    "var(--rustify-space-1, 4px)"
                } else {
                    "var(--rustify-space-2, 8px)"
                },
            )?;
            styles.insert(
                width,
                "font-size",
                if n == 1 {
                    "var(--rustify-font-size-1, 12px)"
                } else {
                    "var(--rustify-font-size-2, 14px)"
                },
            )?;
            styles.insert(
                width,
                "line-height",
                if n == 1 {
                    "var(--rustify-line-height-1, 16px)"
                } else {
                    "var(--rustify-line-height-2, 20px)"
                },
            )?;
        }
        for (width, wrap) in wrap.get().iter() {
            styles.insert(width, "flex-wrap", wrap.css())?;
        }
        for (width, justify) in justify.get().iter() {
            styles.insert(width, "justify-content", justify.css())?;
        }
        margin.get().write(&mut styles)?;
        crate::component_color::set_color(&mut styles, color.get(), mode.get())?;
        Ok(styles)
    });
    view! { <crate::primitives::tabs::List aria_label=aria_label class=format!("rustify-color rustify-tabs-list {geometry} {class}")
    attr:data-high-contrast=move || high_contrast.get().then_some("")>{children()}</crate::primitives::tabs::List> }
}
#[component]
pub fn Trigger(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <crate::primitives::tabs::Trigger value=value class=format!("rustify-tab-trigger {class}") test_id=test_id><span class="rustify-tab-inner">{children()}</span></crate::primitives::tabs::Trigger> }
}
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    value: impl Into<String>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    crate::primitives::tabs::trigger_as(
        target.add_any_attr(crate::primitives::slot::class_token(
            "rustify-tab-trigger".into(),
        )),
        value,
    )
}
#[component]
pub fn Content(
    #[prop(into)] value: String,
    #[prop(optional, into)] margin: Signal<crate::Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = margin_geometry(margin);
    view! { <crate::primitives::tabs::Content value=value class=format!("rustify-tab-content {geometry} {class}") test_id=test_id>{children()}</crate::primitives::tabs::Content> }
}

/// The strip and panels below it, using the same behavior as the composable parts.
#[component]
pub fn Tabs(
    #[prop(into)] active: Signal<String>,
    #[prop(into)] tabs: Signal<Vec<Tab>>,
    on_activate: impl Fn(String) + Send + Sync + 'static,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] activation_mode: ActivationMode,
    #[prop(optional, into)] direction: Option<Signal<rustify_ui::Direction>>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let entries = Signal::derive(move || {
        tabs.get()
            .into_iter()
            .map(|tab| PanelEntry {
                value: tab.value,
                disabled: tab.disabled,
            })
            .collect::<Vec<_>>()
    });
    view! { <Root value=Signal::derive(move || Some(active.get())) entries=entries
        on_change=move |value| { if let Some(value)=value {on_activate(value);} }
        orientation=orientation activation_mode=activation_mode direction=direction class=format!("flex gap-2 flex-col {class}") test_id=test_id>
        <List aria_label=aria_label>
            <For each=move || tabs.get() key=|tab|tab.value.clone() let:tab>
                <Trigger value=tab.value.clone() test_id=format!("tab-{}",tab.value)>{tab.label}</Trigger>
            </For>
        </List>
        {children()}
    </Root> }
}
#[component]
pub fn TabPanel(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <Content value=value.clone() class=class test_id=format!("panel-{value}")>{children()}</Content> }
}
