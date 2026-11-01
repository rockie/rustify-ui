//! Themed avatars reuse the primitive's image requests and deterministic fallback.
pub use crate::primitives::avatar::{Fallback, Image, ImageLoadingStatus};
use crate::{Margin, Responsive};
use leptos::prelude::*;
use rustify_ui::{AccentColor, ThemeMode, ThemeRadius};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    One,
    Two,
    #[default]
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl AvatarSize {
    pub const ALL: &'static [Self] = &[
        Self::One,
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
    ];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<AvatarSize> for Signal<Responsive<AvatarSize>> {
    fn from(value: AvatarSize) -> Self {
        Signal::stored(value.into())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarVariant {
    Solid,
    #[default]
    Soft,
}
impl AvatarVariant {
    pub const ALL: &'static [Self] = &[Self::Solid, Self::Soft];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Soft => "soft",
        }
    }
}
#[component]
pub fn Root(
    #[prop(optional, into)] size: Signal<Responsive<AvatarSize>>,
    #[prop(optional, into)] variant: Signal<AvatarVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let mode = crate::component_color::use_mode();
    let theme = rustify_ui::use_resolved_theme();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        let sizes = size.get();
        let radius = radius.get();
        let scaling = if radius.is_some() {
            theme.map_or(1., |theme| theme.get().config.selected.scaling.factor())
        } else {
            1.
        };
        for (width, size) in std::iter::once((0, sizes.initial.unwrap_or_default())).chain(
            sizes
                .iter()
                .filter(|(width, _)| *width > 0)
                .map(|(width, size)| (width, *size)),
        ) {
            let (length, font_one, font_two, radius_step, radius_pixels) = match size {
                AvatarSize::One => ("var(--rustify-space-5, 24px)", 2, 1, 2, 4.),
                AvatarSize::Two => ("var(--rustify-space-6, 32px)", 3, 2, 2, 4.),
                AvatarSize::Three => ("var(--rustify-space-7, 40px)", 4, 3, 3, 6.),
                AvatarSize::Four => ("var(--rustify-space-8, 48px)", 5, 4, 3, 6.),
                AvatarSize::Five => ("var(--rustify-space-9, 64px)", 6, 6, 4, 8.),
                AvatarSize::Six => ("80px", 7, 7, 5, 12.),
                AvatarSize::Seven => ("96px", 7, 7, 5, 12.),
                AvatarSize::Eight => ("128px", 8, 8, 6, 16.),
                AvatarSize::Nine => ("160px", 9, 9, 6, 16.),
            };
            styles.insert(width, "--rustify-avatar-size", length)?;
            let font_pixels = |step| match step {
                1 => 12,
                2 => 14,
                3 => 16,
                4 => 18,
                5 => 20,
                6 => 24,
                7 => 28,
                8 => 35,
                _ => 60,
            };
            for (property, font) in [
                ("--rustify-avatar-font-one", font_one),
                ("--rustify-avatar-font-two", font_two),
            ] {
                styles.insert(
                    width,
                    property,
                    format!("var(--rustify-font-size-{font}, {}px)", font_pixels(font)),
                )?;
            }
            styles.insert(width,"--rustify-avatar-radius",match radius {
                Some(ThemeRadius::Full)=>"9999px".into(),
                Some(radius)=>format!("{}px",radius_pixels*radius.factor()*scaling),
                None=>format!("max(var(--rustify-radius-{radius_step}, {radius_pixels}px), var(--rustify-radius-full, 0px))"),
            })?;
            let tracking = match font_two {
                1 => "0.0025em",
                2 | 3 => "0em",
                4 => "-0.0025em",
                5 => "-0.005em",
                6 => "-0.00625em",
                7 => "-0.0075em",
                8 => "-0.01em",
                _ => "-0.025em",
            };
            styles.insert(
                width,
                "--rustify-avatar-tracking",
                format!("calc(var(--rustify-letter-spacing, 0em) + {tracking})"),
            )?;
        }
        margin.get().write(&mut styles)?;
        let color = color.get();
        crate::component_color::set_color(
            &mut styles,
            color,
            if color.is_some() {
                mode.get()
            } else {
                ThemeMode::Light
            },
        )?;
        Ok(styles)
    });
    view! {<crate::primitives::avatar::Root class=crate::macros::merge(format!("rustify-color rustify-avatar {geometry}"),&class) test_id=test_id
        attr:data-variant=move ||variant.get().as_str() attr:data-high-contrast=move ||high_contrast.get().then_some("")>
        {children()}
    </crate::primitives::avatar::Root>}
}

/// A named image with readable fallback text. Native attributes reach Image.
#[component]
pub fn Avatar(
    #[prop(into)] src: Signal<String>,
    #[prop(into)] alt: Signal<String>,
    #[prop(into)] fallback: Signal<String>,
    #[prop(optional, into)] size: Signal<Responsive<AvatarSize>>,
    #[prop(optional, into)] variant: Signal<AvatarVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] radius: Signal<Option<ThemeRadius>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional_no_strip, into)] on_loading_status_change: Option<Callback<ImageLoadingStatus>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let class = StoredValue::new(class);
    let test_id = StoredValue::new(test_id);
    view! {<leptos::attribute_interceptor::AttributeInterceptor let:attributes>
        {let attributes=StoredValue::new_local(attributes);
        view! {<Root size=size variant=variant color=color high_contrast=high_contrast radius=radius margin=margin class=class.get_value() test_id=test_id.get_value()
            attr:role=move ||(!alt.get().trim().is_empty()).then_some("img")
            attr:aria-label=move ||{let alt=alt.get();(!alt.trim().is_empty()).then_some(alt)}
            attr:aria-hidden=move ||alt.get().trim().is_empty().then_some("true")>
            <Image src=src alt=alt on_loading_status_change={on_loading_status_change} {..attributes.get_value()} />
            <Fallback attr:data-letters=move ||fallback.get().chars().count().to_string()>{move ||fallback.get()}</Fallback>
        </Root>}}
    </leptos::attribute_interceptor::AttributeInterceptor>}
}
