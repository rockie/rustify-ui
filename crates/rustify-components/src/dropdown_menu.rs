//! Themed command menus sharing the primitive lifecycle and requests.
pub use crate::primitives::dropdown_menu::{trigger_as, Root, Trigger};
use crate::primitives::menu;
pub use crate::primitives::menu::{Group, Portal, RadioGroup, Sub};
use leptos::prelude::*;
use rustify_ui::{AccentColor, Cancelable, FloatingPosition, LayerDismiss, Shortcut};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuSize {
    One,
    #[default]
    Two,
}
impl MenuSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<MenuSize> for Signal<crate::Responsive<MenuSize>> {
    fn from(value: MenuSize) -> Self {
        Signal::stored(value.into())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuVariant {
    #[default]
    Solid,
    Soft,
}
impl MenuVariant {
    pub const ALL: &'static [Self] = &[Self::Solid, Self::Soft];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Soft => "soft",
        }
    }
}
#[derive(Clone, Copy, Default)]
pub(crate) struct Appearance {
    pub(crate) size: Signal<crate::Responsive<MenuSize>>,
    pub(crate) variant: Signal<MenuVariant>,
    pub(crate) color: Signal<Option<AccentColor>>,
    pub(crate) high_contrast: Signal<bool>,
}
pub(crate) fn inherited() -> Appearance {
    use_context::<Appearance>().unwrap_or_default()
}
#[derive(Clone, Copy)]
pub(crate) enum Surface {
    Dropdown,
    Menubar,
    Context,
    Sub,
}

macro_rules! content {
    ($name:ident,$surface:ident,$size:expr,$variant:expr,$color:expr,$contrast:expr) => {
        #[component]
        pub fn $name(
            #[prop(default=$size,into)] size: Signal<crate::Responsive<MenuSize>>,
            #[prop(default=$variant,into)] variant: Signal<MenuVariant>,
            #[prop(default=$color,into)] color: Signal<Option<AccentColor>>,
            #[prop(default=$contrast,into)] high_contrast: Signal<bool>,
            #[prop(optional)] loop_focus: bool,
            #[prop(optional, into)] position: Option<Signal<FloatingPosition>>,
            #[prop(optional, into)] aria_label: Signal<String>,
            #[prop(optional_no_strip, into)] on_escape_key_down: Option<
                Callback<Cancelable<LayerDismiss>>,
            >,
            #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
                Callback<Cancelable<LayerDismiss>>,
            >,
            #[prop(optional_no_strip, into)] on_focus_outside: Option<
                Callback<Cancelable<LayerDismiss>>,
            >,
            #[prop(optional_no_strip, into)] on_interact_outside: Option<
                Callback<Cancelable<LayerDismiss>>,
            >,
            #[prop(optional_no_strip, into)] on_open_auto_focus: Option<Callback<Cancelable<()>>>,
            #[prop(optional_no_strip, into)] on_close_auto_focus: Option<Callback<Cancelable<()>>>,
            #[prop(optional, into)] class: String,
            #[prop(optional, into)] test_id: String,
            children: ChildrenFn,
        ) -> impl IntoView {
            surface(
                Surface::$surface,
                Appearance {
                    size,
                    variant,
                    color,
                    high_contrast,
                },
                loop_focus,
                position,
                aria_label,
                crate::primitives::floating::Hooks {
                    escape: on_escape_key_down,
                    pointer: on_pointer_down_outside,
                    focus: on_focus_outside,
                    interact: on_interact_outside,
                    open: on_open_auto_focus,
                    close: on_close_auto_focus,
                },
                class,
                test_id,
                children,
            )
        }
    };
}
pub(crate) use content;
content!(
    Content,
    Dropdown,
    Signal::default(),
    Signal::default(),
    Signal::default(),
    Signal::default()
);
content!(
    SubContent,
    Sub,
    inherited().size,
    inherited().variant,
    inherited().color,
    inherited().high_contrast
);

#[allow(clippy::too_many_arguments)]
pub(crate) fn surface(
    kind: Surface,
    appearance: Appearance,
    looping: bool,
    position: Option<Signal<FloatingPosition>>,
    aria_label: Signal<String>,
    hooks: crate::primitives::floating::Hooks,
    class: String,
    test_id: String,
    children: ChildrenFn,
) -> AnyView {
    let mode = crate::component_color::use_mode();
    let theme = rustify_ui::use_resolved_theme();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        if let Some(theme) = theme {
            styles.set_floating_surface(&theme.get())?;
        }
        crate::component_color::set_color(&mut styles, appearance.color.get(), mode.get())?;
        for (width, size) in appearance.size.get().iter() {
            let one = *size == MenuSize::One;
            for (name, value) in [
                (
                    "--rustify-menu-padding",
                    if one {
                        "var(--rustify-space-1, 4px)"
                    } else {
                        "var(--rustify-space-2, 8px)"
                    },
                ),
                (
                    "--rustify-menu-height",
                    if one {
                        "var(--rustify-space-5, 24px)"
                    } else {
                        "var(--rustify-space-6, 32px)"
                    },
                ),
                (
                    "--rustify-menu-item-x",
                    if one {
                        "var(--rustify-space-2, 8px)"
                    } else {
                        "var(--rustify-space-3, 12px)"
                    },
                ),
                (
                    "--rustify-menu-check-x",
                    if one {
                        "calc(var(--rustify-space-5, 24px) / 1.2)"
                    } else {
                        "var(--rustify-space-5, 24px)"
                    },
                ),
                (
                    "--rustify-menu-font",
                    if one {
                        "var(--rustify-font-size-1, 12px)"
                    } else {
                        "var(--rustify-font-size-2, 14px)"
                    },
                ),
                (
                    "--rustify-menu-line",
                    if one {
                        "var(--rustify-line-height-1, 16px)"
                    } else {
                        "var(--rustify-line-height-2, 20px)"
                    },
                ),
                (
                    "--rustify-menu-radius",
                    if one {
                        "var(--rustify-radius-3, 6px)"
                    } else {
                        "var(--rustify-radius-4, 8px)"
                    },
                ),
                (
                    "--rustify-menu-item-radius",
                    if one {
                        "var(--rustify-radius-1, 3px)"
                    } else {
                        "var(--rustify-radius-2, 4px)"
                    },
                ),
                (
                    "--rustify-menu-icon",
                    if one {
                        "var(--rustify-space-2, 8px)"
                    } else {
                        "calc(var(--rustify-space-2, 8px) * 1.25)"
                    },
                ),
            ] {
                styles.insert(width, name, value)?;
            }
        }
        Ok(styles)
    });
    let direction = rustify_ui::use_direction();
    let width = RwSignal::new(0_u16);
    #[cfg(target_arch = "wasm32")]
    {
        let read = move || {
            window()
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.) as u16
        };
        width.set(read());
        let listener =
            rustify_ui::listen(&window().into(), "resize", Default::default(), move |_| {
                width.set(read())
            });
        on_cleanup(move || drop(listener));
    }
    let position = position.unwrap_or_else(|| {
        Signal::derive(move || {
            let size = appearance
                .size
                .get()
                .at(width.get())
                .copied()
                .unwrap_or_default();
            FloatingPosition {
                side: match kind {
                    Surface::Dropdown | Surface::Menubar => rustify_ui::Side::Bottom,
                    Surface::Context => rustify_ui::Side::Right,
                    Surface::Sub => {
                        if direction.get() == rustify_ui::Direction::RightToLeft {
                            rustify_ui::Side::Left
                        } else {
                            rustify_ui::Side::Right
                        }
                    }
                },
                align: rustify_ui::Align::Start,
                side_offset: match kind {
                    Surface::Dropdown | Surface::Menubar => 4.,
                    Surface::Context => 2.,
                    Surface::Sub => 1.,
                },
                align_offset: if matches!(kind, Surface::Dropdown | Surface::Menubar) {
                    0.
                } else {
                    -4. * f64::from(size.number())
                },
                collision_padding: rustify_ui::CollisionPadding {
                    top: 10.,
                    right: 10.,
                    bottom: 10.,
                    left: 10.,
                },
                ..Default::default()
            }
        })
    });
    let class = crate::macros::reactive_merge("rustify-menu rustify-color", &class);
    let children = StoredValue::new(children);
    let children: ChildrenFn = std::sync::Arc::new(move || {
        provide_context(appearance);
        view! { <div class="rustify-menu-viewport">{children.with_value(|children|children())}</div> }.into_any()
    });
    let view = if matches!(kind, Surface::Sub) {
        menu::SubContent(
            menu::SubContentProps::builder()
                .loop_focus(looping)
                .position(position)
                .aria_label(aria_label)
                .on_escape_key_down(hooks.escape)
                .on_pointer_down_outside(hooks.pointer)
                .on_focus_outside(hooks.focus)
                .on_interact_outside(hooks.interact)
                .on_open_auto_focus(hooks.open)
                .on_close_auto_focus(hooks.close)
                .test_id(test_id)
                .children(children)
                .build(),
        )
        .into_any()
    } else if matches!(kind, Surface::Menubar) {
        crate::primitives::menubar::content(
            looping,
            position,
            aria_label,
            hooks,
            String::new(),
            test_id,
            children,
        )
    } else {
        menu::content(
            looping,
            position,
            aria_label,
            hooks,
            String::new(),
            test_id,
            children,
        )
    };
    view.add_any_attr((
        leptos::tachys::html::class::class(move || format!("{} {}", class(), geometry)),
        leptos::attr::custom::custom_attribute("data-name", "MenuContent"),
        leptos::attr::custom::custom_attribute("data-variant", move || {
            appearance.variant.get().as_str()
        }),
        leptos::attr::custom::custom_attribute("data-color", move || {
            appearance.color.get().map(AccentColor::as_str)
        }),
        leptos::attr::custom::custom_attribute("data-high-contrast", move || {
            appearance.high_contrast.get().then_some("true")
        }),
    ))
    .into_any()
}
fn item_style(color: Signal<Option<AccentColor>>) -> impl leptos::attr::Attribute {
    let mode = crate::component_color::use_mode();
    let appearance = inherited();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        crate::component_color::set_color(&mut styles, color.get(), mode.get())?;
        Ok(styles)
    });
    (
        crate::primitives::slot::class_token(geometry),
        leptos::attr::custom::custom_attribute("data-menu-variant", move || {
            appearance.variant.get().as_str()
        }),
        leptos::attr::custom::custom_attribute("data-menu-contrast", move || {
            appearance.high_contrast.get().then_some("")
        }),
        leptos::attr::custom::custom_attribute("data-color", move || {
            color.get().map(AccentColor::as_str)
        }),
    )
}
fn shortcut(keys: Option<Shortcut>) -> impl IntoView {
    keys.map(|keys|view!{<kbd class="rustify-menu-shortcut" aria-hidden="true">{keys.to_string()}</kbd>})
}
#[component]
pub fn Item(
    #[prop(into)] on_select: Callback<Cancelable<()>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] text_value: Option<String>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional_no_strip)] shortcut: Option<Shortcut>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let keys = shortcut.as_ref().map(|keys| keys.to_string());
    menu::Item(
        menu::ItemProps::builder()
            .on_select(on_select)
            .disabled(disabled)
            .text_value(text_value)
            .described_by(described_by)
            .class(crate::macros::merge("rustify-menu-item", &class))
            .test_id(test_id)
            .children(Box::new(move || {
                view! { {children()}{self::shortcut(shortcut)} }.into_any()
            }))
            .build(),
    )
    .add_any_attr((item_style(color), leptos::attr::aria_keyshortcuts(keys)))
}
#[component]
pub fn CheckboxItem(
    #[prop(into)] checked: Signal<crate::primitives::CheckedState>,
    #[prop(into)] on_checked_change: Callback<bool>,
    #[prop(optional_no_strip, into)] on_select: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] text_value: Option<String>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional_no_strip)] shortcut: Option<Shortcut>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let keys = shortcut.as_ref().map(|keys| keys.to_string());
    menu::CheckboxItem(menu::CheckboxItemProps::builder().checked(checked).on_checked_change(on_checked_change).on_select(on_select).disabled(disabled).text_value(text_value).described_by(described_by).class(crate::macros::merge("rustify-menu-item", &class)).test_id(test_id)
        .children(Box::new(move ||view!{<ItemIndicator><svg viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="2"><path d=move ||if checked.get()==crate::primitives::CheckedState::Indeterminate {"M2 6h8"}else{"m2 6 3 3 5-6"} /></svg></ItemIndicator>{children()}{self::shortcut(shortcut)} }.into_any())).build())
        .add_any_attr((item_style(color),leptos::attr::aria_keyshortcuts(keys)))
}
#[component]
pub fn RadioItem(
    #[prop(into)] value: String,
    #[prop(optional_no_strip, into)] on_select: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] text_value: Option<String>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    menu::RadioItem(menu::RadioItemProps::builder().value(value).on_select(on_select).disabled(disabled).text_value(text_value).described_by(described_by).class(crate::macros::merge("rustify-menu-item", &class)).test_id(test_id)
        .children(Box::new(move ||view!{<ItemIndicator><svg viewBox="0 0 12 12" fill="currentColor"><circle cx="6" cy="6" r="3" /></svg></ItemIndicator>{children()} }.into_any())).build()).add_any_attr(item_style(color))
}
#[component]
pub fn SubTrigger(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] text_value: Option<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <menu::SubTrigger disabled=disabled text_value=text_value class=crate::macros::merge("rustify-menu-item", &class) test_id=test_id>
        {children()}<svg class="rustify-menu-chevron" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true"><path d="m4 2 4 4-4 4" /></svg>
    </menu::SubTrigger> }.add_any_attr(item_style(Signal::default()))
}
#[component]
pub fn ItemIndicator(
    #[prop(optional)] force_mount: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {<menu::ItemIndicator force_mount=force_mount class=crate::macros::merge("rustify-menu-indicator", &class) test_id=test_id>{children()}</menu::ItemIndicator>}
}
#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {<menu::Label class=crate::macros::merge("rustify-menu-label", &class) test_id=test_id>{children()}</menu::Label>}
}
#[component]
pub fn Separator(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<menu::Separator class=crate::macros::merge("rustify-menu-separator", &class) test_id=test_id />}
}
#[component]
pub fn Arrow(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    view! {<menu::Arrow class=crate::macros::merge("rustify-menu-arrow", &class) test_id=test_id />}
}
