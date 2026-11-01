#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
use leptos::prelude::*;
use rustify_components::{primitives, progress, skeleton, spinner, GeometryStyles, Responsive};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
fn render(
    locale: rustify_ui::Locale,
    view: impl FnOnce() -> AnyView + Send + 'static,
) -> (String, String) {
    let styles = GeometryStyles::default();
    let collector = styles.clone();
    let html = Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "loading".into(),
                ..Default::default()
            },
            move || {
                rustify_components::provide_geometry_styles(collector);
                rustify_ui::provide_locale(locale);
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
fn progress_announces_original_units_and_uses_percentage_only_in_external_geometry() {
    let (html, css) = render(rustify_ui::Locale::English, || {
        view! {
        <primitives::progress::Root value=Some(37.5) max=75. aria_label="Uploaded records"
            value_label=Callback::new(|value:primitives::progress::ProgressValue|format!("{} of {} records",value.value.unwrap_or(0.),value.max))>
            <primitives::progress::Indicator />
        </primitives::progress::Root>
    }.into_any()
    });
    let root = tag(&html, "progress-root");
    assert!(
        root.contains("aria-valuenow=\"37.5\"") && root.contains("aria-valuemax=\"75\""),
        "{html}"
    );
    assert!(
        root.contains("aria-valuetext=\"37.5 of 75 records\"")
            && root.contains("data-state=\"loading\""),
        "{html}"
    );
    assert!(css.contains("width:50%;"), "{css}");
    assert!(!html.contains("style=\""), "{html}");
}
#[test]
fn invalid_progress_does_not_expose_a_false_completed_value() {
    for value in [f64::NAN, f64::INFINITY, -1., 76.] {
        let (html, css) = render(rustify_ui::Locale::English, move || {
            view! {
                <primitives::progress::Root value=Some(value) max=75. aria_label="Uploading">
                    <primitives::progress::Indicator />
                </primitives::progress::Root>
            }
            .into_any()
        });
        let root = tag(&html, "progress-root");
        assert!(
            root.contains("data-rustify-issue=\"value\"")
                && root.contains("data-state=\"indeterminate\""),
            "{html}"
        );
        assert!(
            !root.contains("aria-valuenow") && !css.contains("width:"),
            "{html} {css}"
        );
        assert!(
            !html.contains("NaN") && !css.contains("NaN"),
            "{html} {css}"
        );
    }
}
#[test]
fn unknown_progress_can_use_an_explicit_label_without_claiming_a_numeric_value() {
    let (html, _) = render(rustify_ui::Locale::English, || {
        view! {
        <primitives::progress::Root value=None aria_label="Import" value_label=Callback::new(|value:primitives::progress::ProgressValue|if value.value.is_none(){"Estimating work".to_string()}else{"Measured work".to_string()})>
            <primitives::progress::Indicator />
        </primitives::progress::Root>
    }.into_any()
    });
    assert!(
        tag(&html, "progress-root").contains("aria-valuetext=\"Estimating work\""),
        "{html}"
    );
    assert!(
        !tag(&html, "progress-root").contains("aria-valuenow"),
        "{html}"
    );
}
#[test]
fn progress_indicator_without_a_root_remains_readable_and_diagnostic() {
    let (html, _) = render(rustify_ui::Locale::English, || {
        view! {<primitives::progress::Indicator>"No progress source"</primitives::progress::Indicator>}.into_any()
    });
    assert!(
        html.contains("No progress source") && html.contains("data-rustify-issue=\"missing-root\""),
        "{html}"
    );
    assert!(
        !tag(&html, "progress-indicator").contains("aria-hidden"),
        "{html}"
    );
}
#[test]
fn themed_progress_uses_the_same_units_and_declared_sizes_variants_and_duration() {
    let (html, css) = render(rustify_ui::Locale::English, || {
        progress::ProgressSize::ALL.iter().map(|size|view! {
            <progress::Progress value=37.5 max=75. size=*size
                variant=match size {progress::ProgressSize::One=>progress::ProgressVariant::Classic,progress::ProgressSize::Two=>progress::ProgressVariant::Surface,_=>progress::ProgressVariant::Soft}
                duration_ms=1800 aria_label="Upload" />
        }).collect_view().into_any()
    });
    assert_eq!(html.matches("aria-valuenow=\"37.5\"").count(), 3, "{html}");
    for variant in progress::ProgressVariant::ALL {
        assert!(
            html.contains(&format!("data-variant=\"{}\"", variant.as_str())),
            "{html}"
        );
    }
    assert!(
        css.contains("--rustify-progress-duration:1800ms")
            && css.contains("--rustify-space-1, 4px")
            && css.contains("* 0.75"),
        "{css}"
    );
    assert!(!html.contains("style=\""), "{html}");
}
#[test]
fn spinner_default_name_comes_from_core_locale_and_loaded_content_has_no_status() {
    let (loading, _) = render(rustify_ui::Locale::Chinese, || {
        view! {<spinner::Spinner><input id="retained-draft" value="Keep this draft" /></spinner::Spinner>}.into_any()
    });
    assert_eq!(loading.matches("role=\"status\"").count(), 1, "{loading}");
    assert!(
        tag(&loading, "spinner-status").contains("aria-label=\"加载中…\""),
        "{loading}"
    );
    assert!(
        tag(&loading, "spinner-target").contains("aria-hidden=\"true\"")
            && tag(&loading, "spinner-target").contains("inert"),
        "{loading}"
    );
    assert!(
        loading.contains("id=\"retained-draft\"") && loading.contains("Keep this draft"),
        "{loading}"
    );
    let (loaded, _) = render(rustify_ui::Locale::Chinese, || {
        view! {<spinner::Spinner loading=false><input id="retained-draft" value="Keep this draft" /></spinner::Spinner>}.into_any()
    });
    assert!(
        !loaded.contains("role=\"status\"") && !tag(&loaded, "spinner-target").contains("inert"),
        "{loaded}"
    );
    assert!(
        loaded.contains("id=\"retained-draft\"") && loaded.contains("Keep this draft"),
        "{loaded}"
    );
}
#[test]
fn typed_spinner_preserves_a_complete_table_as_a_status_sibling() {
    let (html, _) = render(rustify_ui::Locale::English, || {
        spinner::spinner_as(
            leptos::html::table()
                .id("retained-table")
                .child(view! {<tbody><tr><td>"Retained cell"</td></tr></tbody>}),
            true,
            "Fetching records",
            spinner::SpinnerSize::Two,
        )
        .into_any()
    });
    assert!(
        html.contains("<table")
            && html.contains("id=\"retained-table\"")
            && html.contains("Retained cell"),
        "{html}"
    );
    let table_start = html.find("<table").unwrap();
    let table_end = html.find("</table>").unwrap();
    assert!(
        !html[table_start..table_end].contains("spinner-status"),
        "{html}"
    );
    assert_eq!(html.matches("role=\"status\"").count(), 1, "{html}");
    assert!(
        tag(&html, "spinner-status").contains("aria-label=\"Fetching records\""),
        "{html}"
    );
}
#[test]
fn skeleton_uses_a_span_without_children_and_a_legal_div_with_children() {
    let (empty, _) = render(rustify_ui::Locale::English, || {
        view! {<skeleton::Skeleton />}.into_any()
    });
    assert!(
        tag(&empty, "skeleton-target").starts_with("span"),
        "{empty}"
    );
    let (content, css) = render(rustify_ui::Locale::English, || {
        view! {
        <skeleton::Skeleton layout=rustify_components::Layout {width:Responsive::from("240px".to_string()),..Default::default()}>
            <p>"Stable block content"</p><input id="skeleton-draft" value="Kept draft" />
        </skeleton::Skeleton>
    }.into_any()
    });
    assert!(
        tag(&content, "skeleton-target").starts_with("div"),
        "{content}"
    );
    assert!(
        content.contains("Stable block content") && content.contains("id=\"skeleton-draft\""),
        "{content}"
    );
    assert!(
        tag(&content, "skeleton-target").contains("aria-hidden=\"true\"")
            && tag(&content, "skeleton-target").contains("inert"),
        "{content}"
    );
    assert!(
        css.contains("width:240px") && !content.contains("style=\""),
        "{css} {content}"
    );
}
#[test]
fn typed_skeleton_preserves_native_attributes_and_does_not_run_the_target_listener() {
    let calls = Arc::new(AtomicUsize::new(0));
    let called = calls.clone();
    let (html, _) = render(rustify_ui::Locale::English, move || {
        skeleton::skeleton_as(
            leptos::html::button()
                .id("native-action")
                .r#type("button")
                .on(leptos::ev::click, move |_| {
                    called.fetch_add(1, Ordering::Relaxed);
                })
                .child("Retained action"),
            true,
            rustify_components::Layout::default(),
        )
        .into_any()
    });
    assert!(
        tag(&html, "skeleton-target").starts_with("button")
            && html.contains("id=\"native-action\"")
            && html.contains("Retained action"),
        "{html}"
    );
    assert_eq!(calls.load(Ordering::Relaxed), 0);
    let (loaded, _) = render(rustify_ui::Locale::English, || {
        skeleton::skeleton_as(
            leptos::html::button()
                .id("native-action")
                .child("Retained action"),
            false,
            rustify_components::Layout::default(),
        )
        .into_any()
    });
    assert!(
        !tag(&loaded, "skeleton-target").contains("inert")
            && !tag(&loaded, "skeleton-target").contains("aria-hidden"),
        "{loaded}"
    );
}
#[test]
fn spinner_sizes_use_resolved_spaces_and_responsive_geometry() {
    let (html, css) = render(rustify_ui::Locale::English, || {
        let simple = spinner::SpinnerSize::ALL
            .iter()
            .map(|size| view! {<spinner::Spinner size=*size label="Refreshing" />})
            .collect_view();
        let responsive = view! {<spinner::Spinner size=Responsive {initial:Some(spinner::SpinnerSize::One),sm:Some(spinner::SpinnerSize::Three),..Default::default()} label="Refreshing" />};
        (simple, responsive).into_any()
    });
    assert_eq!(html.matches("role=\"status\"").count(), 4, "{html}");
    assert!(
        css.contains("var(--rustify-space-3, 12px)")
            && css.contains("var(--rustify-space-4, 16px)")
            && css.contains("* 1.25")
            && css.contains("768px"),
        "{css}"
    );
    assert!(!html.contains("style=\""), "{html}");
}
