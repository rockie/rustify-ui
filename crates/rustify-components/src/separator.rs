//! Responsive themed separators using the same semantic primitive.
use crate::{Margin, Orientation, Responsive};
use leptos::prelude::*;
use rustify_ui::AccentColor;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorSize {
    #[default]
    One,
    Two,
    Three,
    Four,
}
impl SeparatorSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three, Self::Four];
    pub const fn number(self) -> u8 {
        self as u8 + 1
    }
}
impl From<SeparatorSize> for Signal<Responsive<SeparatorSize>> {
    fn from(value: SeparatorSize) -> Self {
        Signal::stored(value.into())
    }
}
impl From<Orientation> for Signal<Responsive<Orientation>> {
    fn from(value: Orientation) -> Self {
        Signal::stored(value.into())
    }
}
#[component]
pub fn Separator(
    #[prop(optional, into)] size: Signal<Responsive<SeparatorSize>>,
    #[prop(optional, into)] orientation: Signal<Responsive<Orientation>>,
    #[prop(default=Signal::stored(AccentColor::Gray),into)] color: Signal<AccentColor>,
    #[prop(default=Signal::stored(true),into)] decorative: Signal<bool>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let mode = crate::component_color::use_mode();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = crate::geometry::StyleSet::default();
        for (width, size) in size.get().iter() {
            let length = match size {
                SeparatorSize::One => "var(--rustify-space-4, 16px)",
                SeparatorSize::Two => "var(--rustify-space-6, 32px)",
                SeparatorSize::Three => "var(--rustify-space-9, 64px)",
                SeparatorSize::Four => "100%",
            };
            styles.insert(width, "--rustify-separator-size", length)?;
        }
        for (width, orientation) in orientation.get().iter() {
            let (width_value, height_value) = match orientation {
                Orientation::Horizontal => (
                    "var(--rustify-separator-size, var(--rustify-space-4, 16px))",
                    "1px",
                ),
                Orientation::Vertical => (
                    "1px",
                    "var(--rustify-separator-size, var(--rustify-space-4, 16px))",
                ),
            };
            styles.insert(width, "width", width_value)?;
            styles.insert(width, "height", height_value)?;
        }
        margin.get().write(&mut styles)?;
        crate::component_color::set_color(&mut styles, Some(color.get()), mode.get())?;
        Ok(styles)
    });
    let viewport = RwSignal::new(0_u16);
    #[cfg(target_arch = "wasm32")]
    {
        let listener = StoredValue::new_local(None::<rustify_ui::Listener>);
        Effect::new(move || {
            let responsive = orientation.get().iter().any(|(width, _)| width > 0);
            if decorative.get() || !responsive {
                listener.set_value(None);
                return;
            }
            let width = || {
                window()
                    .inner_width()
                    .ok()
                    .and_then(|width| width.as_f64())
                    .unwrap_or(0.) as u16
            };
            let current = width();
            if viewport.get_untracked() != current {
                viewport.set(current);
            }
            if listener.with_value(Option::is_none) {
                listener.set_value(Some(rustify_ui::listen(
                    window().as_ref(),
                    "resize",
                    Default::default(),
                    move |_| {
                        let width = width();
                        if viewport.try_get_untracked() != Some(width) {
                            let _ = viewport.try_set(width);
                        }
                    },
                )));
            }
        });
        on_cleanup(move || {
            listener.try_set_value(None);
        });
    }
    let semantic_orientation = Signal::derive(move || {
        orientation
            .get()
            .at(viewport.get())
            .copied()
            .unwrap_or_default()
    });
    view! {<crate::primitives::separator::Root orientation=semantic_orientation decorative=decorative
    class=crate::macros::merge(format!("rustify-color rustify-separator {geometry}"),&class) test_id=test_id />}
}
pub use Separator as Root;
