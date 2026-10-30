//! Explicit local writing direction, independent of the chosen language.

use leptos::prelude::*;
use rustify_ui::{provide_direction, Direction};

/// Provides local direction to children and to their scope-owned portals.
#[component]
pub fn DirectionProvider(
    #[prop(into)] direction: Signal<Direction>,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(|| {
        provide_direction(direction);
        view! { <div dir=move || direction.get().attribute() data-rustify-part="direction-provider">{children()}</div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_ui::{provide_locale, use_direction, Locale};

    #[test]
    fn explicit_direction_is_reactive_and_does_not_change_locale() {
        let owner = Owner::new();
        owner.with(|| {
            let locale = provide_locale(Locale::Chinese);
            let direction = RwSignal::new(Direction::RightToLeft);
            provide_direction(direction.into());
            let read = use_direction();
            assert_eq!(read.get_untracked(), Direction::RightToLeft);
            direction.set(Direction::LeftToRight);
            assert_eq!(read.get_untracked(), Direction::LeftToRight);
            assert_eq!(locale.get_untracked(), Locale::Chinese);
        });
    }

    #[test]
    fn nearest_provider_shadows_its_parent() {
        let owner = Owner::new();
        owner.with(|| {
            provide_direction(Signal::derive(|| Direction::RightToLeft));
            let parent = use_direction();
            Owner::new().with(|| {
                provide_direction(Signal::derive(|| Direction::LeftToRight));
                assert_eq!(use_direction().get_untracked(), Direction::LeftToRight);
            });
            assert_eq!(parent.get_untracked(), Direction::RightToLeft);
        });
    }

    #[test]
    fn provider_does_not_replace_the_direction_of_its_siblings() {
        Owner::new().with(|| {
            provide_direction(Signal::derive(|| Direction::LeftToRight));
            let _ = view! {
                <DirectionProvider direction=Signal::derive(|| Direction::RightToLeft)>
                    <span>"Local direction"</span>
                </DirectionProvider>
            };
            assert_eq!(use_direction().get_untracked(), Direction::LeftToRight);
        });
    }

    #[test]
    fn absent_provider_uses_locale_direction() {
        let owner = Owner::new();
        owner.with(|| {
            let locale = provide_locale(Locale::English);
            assert_eq!(
                use_direction().get_untracked(),
                locale.get_untracked().direction()
            );
        });
    }
}
