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
/// that the whole crate has one answer about what merging means, and so the
/// prefix decision below has one place to be stated and tested.
///
/// **The prefix is not configured here, on purpose.** Tailwind v4's
/// `prefix(rui)` puts the prefix in front of the variants (`rui:hover:bg-x`),
/// while `tw_merge`'s own `prefix` option expects it after them
/// (`hover:tw-bg-x`, which is v3's shape). Setting that option would make the
/// parser look for `rui` in a place our classes never put it. Left at its
/// default, `rui` parses as a leading variant, and since every class in this
/// crate carries it, two classes conflict exactly when they would have without
/// it. The tests below are what keeps that true.
pub fn merge(base: impl AsRef<str>, class: &str) -> String {
    tw_merge::tw_merge!(base.as_ref(), class)
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn a_later_class_replaces_an_earlier_one_of_the_same_kind() {
        assert_eq!(
            merge("rui:bg-primary rui:p-2", "rui:bg-secondary"),
            "rui:p-2 rui:bg-secondary"
        );
    }

    #[test]
    fn classes_of_different_kinds_are_both_kept() {
        assert_eq!(merge("rui:bg-primary", "rui:p-2"), "rui:bg-primary rui:p-2");
    }

    #[test]
    fn a_variant_makes_two_classes_different_even_with_the_prefix() {
        // `rui:hover:bg-x` and `rui:bg-x` set the same property in different
        // states. A merge that dropped one of them would be wrong, and it is
        // the case the prefix could plausibly have broken.
        assert_eq!(
            merge("rui:hover:bg-primary", "rui:bg-secondary"),
            "rui:hover:bg-primary rui:bg-secondary"
        );
        assert_eq!(
            merge("rui:hover:bg-primary", "rui:hover:bg-secondary"),
            "rui:hover:bg-secondary"
        );
    }

    #[test]
    fn an_unprefixed_class_does_not_collide_with_a_prefixed_one() {
        // The host page's own Tailwind build has no `rui` prefix. Its classes
        // and ours are different classes, and a merge that treated them as the
        // same would be the class collision the prefix exists to prevent.
        assert_eq!(
            merge("rui:bg-primary", "bg-secondary"),
            "rui:bg-primary bg-secondary"
        );
    }
}

/// Both macros are declarative, so nothing in them is type-checked until it is
/// expanded. These three expansions are what makes a change to either macro a
/// compile error here rather than in the first application that uses it.
#[cfg(test)]
#[allow(dead_code)]
mod expansion {
    crate::clx! {TestCard, div, "rui:rounded-lg", "rui:bg-card"}
    crate::void! {TestRule, hr, "rui:border-border"}

    crate::variants! {
        TestBadge {
            base: "rui:inline-flex",
            variants: {
                variant: {
                    Default: "rui:bg-primary",
                    Outline: "rui:border",
                },
                size: {
                    Default: "rui:px-2",
                    Lg: "rui:px-3",
                }
            },
            component: { element: span }
        }
    }

    crate::variants! {
        TestNav {
            base: "rui:inline-flex",
            variants: {
                variant: {
                    Default: "rui:text-foreground",
                },
                size: {
                    Default: "rui:px-2",
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
        assert_eq!(
            outline.with_class(String::new()),
            "rui:inline-flex rui:border rui:px-3"
        );
        // The default arms are the defaults, and a caller's class still wins.
        let default = TestBadgeClass {
            variant: TestBadgeVariant::default(),
            size: TestBadgeSize::default(),
        };
        assert_eq!(
            default.with_class("rui:bg-secondary"),
            "rui:inline-flex rui:px-2 rui:bg-secondary"
        );
    }
}
