#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
use leptos::prelude::*;
use rustify_components::{avatar, primitives, separator, GeometryStyles, Responsive};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

fn render(view: impl FnOnce() -> AnyView + Send + 'static) -> (String, String) {
    let styles = GeometryStyles::default();
    let collector = styles.clone();
    let html = Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "media".into(),
                ..Default::default()
            },
            move || {
                rustify_components::provide_geometry_styles(collector);
                view()
            },
        )
        .to_html()
    });
    (html, styles.css())
}
fn tag<'a>(html: &'a str, part: &str) -> &'a str {
    html.split('<')
        .find(|tag| tag.contains(&format!("data-rustify-part=\"{part}\"")))
        .unwrap()
}
#[test]
fn avatar_ssg_keeps_fallback_and_native_image_attributes_without_loading_callbacks() {
    let calls = Arc::new(AtomicUsize::new(0));
    let called = calls.clone();
    let (html, _) = render(move || {
        view! {
        <primitives::avatar::Root attr:aria-label="Jamie Morgan" attr:role="img">
            <primitives::avatar::Image src="/people/jamie.png" alt="Jamie Morgan" attr:width="80" attr:crossorigin="anonymous" attr:referrerpolicy="no-referrer"
                on_loading_status_change=Callback::new(move |_| {called.fetch_add(1,Ordering::Relaxed);}) />
            <primitives::avatar::Fallback delay_ms=500>"JM"</primitives::avatar::Fallback>
        </primitives::avatar::Root>
    }.into_any()
    });
    let image = tag(&html, "avatar-image");
    assert!(
        image.contains("src=\"/people/jamie.png\"") && image.contains("alt=\"Jamie Morgan\""),
        "{html}"
    );
    assert!(
        image.contains("width=\"80\"")
            && image.contains("crossorigin=\"anonymous\"")
            && image.contains("referrerpolicy=\"no-referrer\""),
        "{html}"
    );
    assert!(
        image.contains("aria-hidden=\"true\"") && image.contains("data-state=\"idle\""),
        "{html}"
    );
    assert!(
        tag(&html, "avatar-fallback").contains("data-visible=\"true\""),
        "{html}"
    );
    assert!(
        html.contains("JM") && !tag(&html, "avatar-fallback").contains("aria-hidden"),
        "{html}"
    );
    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert!(!html.contains("style=\""), "{html}");
}
#[test]
fn orphan_avatar_parts_keep_readable_native_content_and_diagnostics() {
    let (html, _) = render(|| {
        view! {
            <primitives::avatar::Image src="/profile.png" alt="Profile photograph" />
            <primitives::avatar::Fallback>"Profile initials"</primitives::avatar::Fallback>
        }
        .into_any()
    });
    assert_eq!(
        html.matches("data-rustify-issue=\"missing-root\"").count(),
        2,
        "{html}"
    );
    assert!(
        html.contains("Profile photograph") && html.contains("Profile initials"),
        "{html}"
    );
    assert!(
        !tag(&html, "avatar-image").contains("aria-hidden"),
        "{html}"
    );
}
#[test]
fn themed_avatar_sizes_and_variants_emit_external_geometry_with_named_fallbacks() {
    let (html, css) = render(|| AvatarSizes().into_any());
    for size in avatar::AvatarSize::ALL {
        assert!(
            html.contains(&format!("data-testid=\"avatar-{}\"", size.number())),
            "{html}"
        );
    }
    assert_eq!(
        html.matches("aria-label=\"Jamie Morgan\"").count(),
        9,
        "{html}"
    );
    assert_eq!(html.matches("data-visible=\"true\"").count(), 9, "{html}");
    for length in [
        "24px", "32px", "40px", "48px", "64px", "80px", "96px", "128px", "160px",
    ] {
        assert!(css.contains(length), "missing {length}: {css}");
    }
    assert!(
        html.contains("data-variant=\"solid\"") && html.contains("data-variant=\"soft\""),
        "{html}"
    );
    assert!(!html.contains("style=\""), "{html}");
}
#[component]
fn AvatarSizes() -> impl IntoView {
    avatar::AvatarSize::ALL.iter().map(|size|view! {
        <avatar::Avatar src="/profile.png" alt="Jamie Morgan" fallback="JM" size=*size
            variant=if size.number()%2==0{avatar::AvatarVariant::Solid}else{avatar::AvatarVariant::Soft}
            test_id=format!("avatar-{}",size.number()) />
    }).collect_view()
}
#[test]
fn decorative_avatar_does_not_announce_its_initials() {
    let (html, _) =
        render(|| view! {<avatar::Avatar src="/ornament.png" alt="" fallback="JM" />}.into_any());
    let root = tag(&html, "avatar-root");
    assert!(root.contains("aria-hidden=\"true\""), "{html}");
    assert!(
        !root.contains("role=\"img\"") && !root.contains("aria-label="),
        "{html}"
    );
}
#[test]
fn aspect_ratio_rejects_nonpositive_and_nonfinite_initial_geometry() {
    for ratio in [0., -1., f64::NAN, f64::INFINITY] {
        let (html, css) = render(move || {
            view! {<primitives::aspect_ratio::Root ratio=ratio><img src="/landscape.png" alt="Rocky coast" /></primitives::aspect_ratio::Root>}.into_any()
        });
        assert!(html.contains("data-rustify-issue=\"ratio\""), "{html}");
        assert!(css.contains("aspect-ratio:1;"), "{css}");
        assert!(
            html.contains("Rocky coast") && !html.contains("style=\""),
            "{html}"
        );
        assert!(!css.contains("NaN") && !css.contains("inf"), "{css}");
    }
}
#[test]
fn aspect_ratio_positive_values_and_children_are_static_without_inline_styles() {
    let (html, css) = render(|| {
        view! {<rustify_components::aspect_ratio::AspectRatio ratio=1.5 attr:data-caller="retained"><span>"Static ratio content"</span></rustify_components::aspect_ratio::AspectRatio>}.into_any()
    });
    assert!(css.contains("aspect-ratio:1.5;"), "{css}");
    assert!(
        html.contains("Static ratio content") && html.contains("data-caller=\"retained\""),
        "{html}"
    );
    assert!(
        !html.contains("data-rustify-issue") && !html.contains("style=\""),
        "{html}"
    );
}
#[test]
fn separator_defaults_are_decorative_and_semantic_vertical_has_no_tabstop() {
    let (html, _) = render(|| {
        view! {
        <primitives::separator::Root />
        <primitives::separator::Root decorative=false orientation=rustify_components::Orientation::Vertical attr:aria-label="Entry groups" />
    }.into_any()
    });
    let separators: Vec<_> = html
        .split('<')
        .filter(|tag| tag.contains("data-rustify-part=\"separator-root\""))
        .collect();
    assert_eq!(separators.len(), 2, "{html}");
    assert!(
        separators[0].contains("role=\"presentation\"")
            && separators[0].contains("aria-hidden=\"true\"")
            && !separators[0].contains("aria-orientation"),
        "{html}"
    );
    assert!(
        separators[1].contains("role=\"separator\"")
            && separators[1].contains("aria-orientation=\"vertical\"")
            && separators[1].contains("aria-label=\"Entry groups\""),
        "{html}"
    );
    assert!(
        separators.iter().all(|tag| !tag.contains("tabindex")),
        "{html}"
    );
}
#[test]
fn separator_responsive_geometry_uses_gray_roles_and_matching_initial_semantics() {
    let (html, css) = render(|| {
        view! {
        <separator::Separator decorative=false
            size=Responsive {initial:Some(separator::SeparatorSize::Two),md:Some(separator::SeparatorSize::Four),..Default::default()}
            orientation=Responsive {initial:Some(rustify_components::Orientation::Horizontal),xs:Some(rustify_components::Orientation::Vertical),..Default::default()}
            attr:data-caller="retained" />
    }.into_any()
    });
    assert!(
        html.contains("aria-orientation=\"horizontal\"")
            && html.contains("data-caller=\"retained\""),
        "{html}"
    );
    assert!(
        css.contains("--rustify-separator-size:var(--rustify-space-6, 32px)")
            && css.contains("--rustify-separator-size:100%"),
        "{css}"
    );
    assert!(
        css.contains("520px") && css.contains("1024px") && css.contains("width:1px"),
        "{css}"
    );
    assert!(css.contains("--rustify-component-color-a6:"), "{css}");
    assert!(
        !html.contains("style=\"") && !tag(&html, "separator-root").contains("tabindex"),
        "{html}"
    );
}
