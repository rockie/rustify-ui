#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use leptos::prelude::*;
use rustify_dom::{ssr_scope, MountConfig, ScopeRoots};

fn render(seed: &str) -> String {
    Owner::new().with(|| {
        ssr_scope(
            MountConfig {
                scope: seed.to_string(),
                base: "/rustify-ui/".to_string(),
                url_owner: false,
            },
            || {
                let roots = use_context::<ScopeRoots>().expect("scope context");
                assert_eq!(roots.base(), "/rustify-ui/");
                let first = rustify_dom::id::next("field").expect("scoped allocator");
                let second = rustify_dom::id::next("field").expect("scoped allocator");
                assert_ne!(first, second);
                view! {
                    <label for=first.clone()>"Count"</label>
                    <input id=first value="2" />
                    <button id=second>"Increment"</button>
                }
            },
        )
        .to_html()
    })
}

#[test]
fn scopes_render_the_same_ids_and_explicit_roots_on_every_build() {
    let first = render("counter-a");
    assert_eq!(first, render("counter-a"));
    assert!(
        first.contains("data-rustify-scope=\"counter-a\""),
        "{first}"
    );
    assert!(first.contains("data-rustify-seed=\"counter-a\""), "{first}");
    assert!(first.contains("data-rustify-ssr=\"1\""), "{first}");
    assert!(first.contains("data-rustify-content"), "{first}");
    assert!(first.contains("data-rustify-overlay"), "{first}");
    assert!(first.contains("rui-counter-a-field-1"), "{first}");
    assert!(!first.contains("data-rustify-mounted"), "{first}");
}

#[test]
fn parallel_scopes_keep_their_own_sequence() {
    let expected = render("parallel");
    let workers: Vec<_> = (0..3)
        .map(|_| std::thread::spawn(|| render("parallel")))
        .collect();
    for worker in workers {
        assert_eq!(expected, worker.join().expect("render worker"));
    }
    assert_ne!(expected, render("other"));
}

#[test]
fn sibling_scopes_own_their_context_during_delayed_rendering() {
    Owner::new().with(|| {
        let make = |scope: &str| {
            ssr_scope(
                MountConfig {
                    scope: scope.to_string(),
                    ..MountConfig::default()
                },
                || view! { <span>{move || rustify_dom::id::next("late").expect("scope id")}</span> },
            )
        };
        let first = make("one");
        let second = make("two");
        assert!(rustify_dom::id::next("outside").is_none());
        let html = (first, second).to_html();
        assert!(html.contains("rui-one-late-1"), "{html}");
        assert!(html.contains("rui-two-late-1"), "{html}");
    });
}
