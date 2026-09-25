//! The class-string machinery every component in this crate is built on.
//!
//! Both macros came from Rust/UI; `sources.lock.json` says which files and
//! what the rewrite changed. What it changed, in one line each: the generated
//! components pass a `test_id` through, they no longer re-export another
//! crate's prelude from a module, and the one that renders a link reads this
//! crate's own idea of the current path instead of a router's.

pub mod clx;
pub mod variants;

/// Merges a component's own classes with the caller's, caller last.
///
/// It exists as a function rather than a `tw_merge!` call inside each macro so
/// that the whole crate has one answer about what merging means.
///
/// The component classes are the same unprefixed utilities an application
/// writes, so a caller's class that sets what a component's class sets, in the
/// same state, replaces it. Without the merge both would reach the element,
/// and the order of the stylesheet, not the caller, would decide which shows.
pub fn merge(base: impl AsRef<str>, class: &str) -> String {
    tw_merge::tw_merge!(base.as_ref(), class)
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn a_later_class_replaces_an_earlier_one_of_the_same_kind() {
        assert_eq!(merge("bg-primary p-2", "bg-secondary"), "p-2 bg-secondary");
    }

    #[test]
    fn classes_of_different_kinds_are_both_kept() {
        assert_eq!(merge("bg-primary", "p-2"), "bg-primary p-2");
    }

    #[test]
    fn a_variant_makes_two_classes_different() {
        // `hover:bg-x` and `bg-x` set the same property in different states.
        // A merge that dropped one of them would take away a state the
        // component still draws.
        assert_eq!(
            merge("hover:bg-primary", "bg-secondary"),
            "hover:bg-primary bg-secondary"
        );
    }

    #[test]
    fn a_callers_padding_replaces_the_components() {
        assert_eq!(
            merge("inline-flex p-4 bg-card", "p-6"),
            "inline-flex bg-card p-6"
        );
    }

    #[test]
    fn a_callers_class_in_a_state_replaces_the_components_in_that_state() {
        assert_eq!(
            merge("bg-primary hover:bg-primary/90", "hover:bg-secondary"),
            "bg-primary hover:bg-secondary"
        );
    }
}

/// Both macros are declarative, so nothing in them is type-checked until it is
/// expanded. These three expansions are what makes a change to either macro a
/// compile error here rather than in the first application that uses it.
#[cfg(test)]
#[allow(dead_code)]
mod expansion {
    crate::clx! {TestCard, div, "rounded-lg", "bg-card"}
    crate::void! {TestRule, hr, "border-border"}

    crate::variants! {
        TestBadge {
            base: "inline-flex",
            variants: {
                variant: {
                    Default: "bg-primary",
                    Outline: "border",
                },
                size: {
                    Default: "px-2",
                    Lg: "px-3",
                }
            },
            component: { element: span }
        }
    }

    crate::variants! {
        TestNav {
            base: "inline-flex",
            variants: {
                variant: {
                    Default: "text-foreground",
                },
                size: {
                    Default: "px-2",
                }
            },
            component: { element: span, link: true }
        }
    }

    #[test]
    fn a_variant_carries_its_own_classes_and_the_callers() {
        use tw_merge::IntoTailwindClass;
        let outline = TestBadgeClass {
            variant: TestBadgeVariant::Outline,
            size: TestBadgeSize::Lg,
        };
        assert_eq!(outline.with_class(String::new()), "inline-flex border px-3");
        // The default arms are the defaults, and a caller's class still wins.
        let default = TestBadgeClass {
            variant: TestBadgeVariant::default(),
            size: TestBadgeSize::default(),
        };
        assert_eq!(
            default.with_class("bg-secondary"),
            "inline-flex px-2 bg-secondary"
        );
    }
}
