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
/// It exists as a function rather than an engine call inside each macro so
/// that the whole crate has one answer about what merging means.
///
/// The component classes are the same unprefixed utilities an application
/// writes, so a caller's class that sets what a component's class sets, in the
/// same state, replaces it. Without the merge both would reach the element,
/// and the order of the stylesheet, not the caller, would decide which shows.
pub fn merge(base: impl AsRef<str>, class: &str) -> String {
    crate::cn::merge(&[base.as_ref(), class])
}

/// Retains raw component and caller classes and recomputes them when the
/// enclosing utility scope commits a new cn configuration.
pub fn reactive_merge(
    base: impl AsRef<str>,
    class: &str,
) -> impl Fn() -> String + Send + Sync + Copy + 'static {
    let base = base.as_ref().to_owned();
    let class = class.to_owned();
    reactive_merge_with(move || vec![base.clone(), class.clone()])
}

/// The reactive form for components whose variant, size or caller classes
/// change. Each evaluation passes the original fragments to cn exactly once.
pub fn reactive_merge_with(
    fragments: impl Fn() -> Vec<String> + Send + Sync + 'static,
) -> impl Fn() -> String + Send + Sync + Copy + 'static {
    use leptos::prelude::*;

    #[cfg(feature = "utility")]
    let context = use_context::<rustify_ui::utility::CnContext>();
    #[cfg(feature = "utility")]
    let registration = context.as_ref().and_then(|context| {
        let id = match context.register(untrack(&fragments)) {
            Ok(id) => Some(id),
            Err(error) => {
                context.report_error(error);
                None
            }
        };
        let cleanup_context = context.clone();
        on_cleanup(move || {
            if let Some(id) = id {
                cleanup_context.unregister(id);
            }
        });
        id
    });
    let computed = Memo::new(move |previous: Option<&String>| {
        let fragments = fragments();
        let raw: Vec<&str> = fragments.iter().map(String::as_str).collect();
        #[cfg(feature = "utility")]
        if let Some(context) = &context {
            if let Some(id) = registration {
                if let Err(error) = context.update(id, fragments.clone()) {
                    context.report_error(error);
                    return previous
                        .cloned()
                        .unwrap_or_else(|| crate::cn::join_text(&raw));
                }
            }
            let merged = match registration {
                Some(id) => context.merge_registered(id, &raw),
                None => context.merge(&raw),
            };
            return match merged {
                Ok(class) => class,
                Err(error) => {
                    context.report_error(error);
                    previous
                        .cloned()
                        .unwrap_or_else(|| crate::cn::join_text(&raw))
                }
            };
        }
        #[cfg(not(feature = "utility"))]
        let _ = previous;
        crate::cn::merge(&raw)
    });
    move || computed.get()
}

#[cfg(test)]
mod tests {
    use super::{merge, reactive_merge_with};
    use leptos::prelude::*;

    #[test]
    fn reactive_classes_restore_raw_base_after_caller_override_is_removed() {
        let owner = Owner::new();
        owner.with(|| {
            let caller = RwSignal::new("p-4".to_string());
            let class = reactive_merge_with(move || vec!["p-2 bg-primary".into(), caller.get()]);
            assert_eq!(class(), "bg-primary p-4");
            caller.set(String::new());
            assert_eq!(class(), "p-2 bg-primary");
        });
    }

    #[test]
    fn reactive_classes_merge_once_for_each_source_change() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };

        let owner = Owner::new();
        owner.with(|| {
            let caller = RwSignal::new("p-4".to_string());
            let evaluations = Arc::new(AtomicUsize::new(0));
            let observed = evaluations.clone();
            let class = reactive_merge_with(move || {
                observed.fetch_add(1, Ordering::Relaxed);
                vec!["p-2".into(), caller.get()]
            });
            assert_eq!(class(), "p-4");
            assert_eq!(class(), "p-4");
            assert_eq!(evaluations.load(Ordering::Relaxed), 1);
            caller.set("p-8".into());
            assert_eq!(class(), "p-8");
            assert_eq!(class(), "p-8");
            assert_eq!(evaluations.load(Ordering::Relaxed), 2);
        });
    }

    #[cfg(feature = "utility")]
    #[test]
    fn scoped_merge_keeps_classes_from_disjoint_custom_groups() {
        use crate::cn::{ClassGroupDef, Cn, Config};
        let mut config = Config::default();
        config.class_groups.retain(|(name, _)| name != "p");
        config.class_groups.extend([
            (
                "small-padding".into(),
                vec![ClassGroupDef::Literal("p-2".into())],
            ),
            (
                "large-padding".into(),
                vec![ClassGroupDef::Literal("p-4".into())],
            ),
        ]);
        Owner::new().with(|| {
            let context = rustify_ui::utility::CnContext::new(Cn::new(config).unwrap(), 4096);
            provide_context(context);
            let class = reactive_merge_with(|| vec!["p-2".into(), "p-4".into()]);
            assert_eq!(class(), "p-2 p-4");
        });
    }

    #[cfg(feature = "utility")]
    #[test]
    fn scoped_callback_failure_keeps_the_last_successful_class() {
        use crate::cn::{ClassGroupDef, Cn, CnError, Config};
        use std::{cell::Cell, rc::Rc};
        let fail = Rc::new(Cell::new(false));
        let flag = fail.clone();
        let mut config = Config::default();
        config.class_groups.push((
            "probe".into(),
            vec![ClassGroupDef::Object(vec![(
                "probe".into(),
                vec![ClassGroupDef::Validator(Rc::new(move |_| {
                    if flag.get() {
                        Err(CnError::new("fixture callback failed"))
                    } else {
                        Ok(true)
                    }
                }))],
            )])],
        ));
        Owner::new().with(|| {
            let context = rustify_ui::utility::CnContext::new(Cn::new(config).unwrap(), 4096);
            provide_context(context.clone());
            let caller = RwSignal::new("probe-one".to_string());
            let class = reactive_merge_with(move || vec!["p-2".into(), caller.get()]);
            assert_eq!(class(), "p-2 probe-one");
            fail.set(true);
            caller.set("probe-two".into());
            assert_eq!(class(), "p-2 probe-one");
            assert_eq!(
                context.last_error().as_deref(),
                Some("fixture callback failed")
            );
        });
    }

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
        use crate::cn::IntoTailwindClass;
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
