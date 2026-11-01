#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use leptos::prelude::*;
use rustify_components::{
    primitives::scroll_area as raw, scroll_area as styled, GeometryStyles, Orientation, Responsive,
};

fn render_scroll_area(view: impl FnOnce() -> AnyView + Send + 'static) -> (String, GeometryStyles) {
    let styles = GeometryStyles::default();
    let output = styles.clone();
    let html = Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "scroll-area-content".into(),
                ..Default::default()
            },
            move || {
                rustify_components::provide_geometry_styles(output);
                view()
            },
        )
        .to_html()
    });
    (html, styles)
}
fn scroll_part<'a>(html: &'a str, part: &str) -> &'a str {
    html.split('<')
        .find(|tag| tag.contains(&format!("data-rustify-part=\"scroll-area-{part}\"")))
        .unwrap()
}

#[test]
fn invalid_scroll_hide_delay_is_diagnosed_without_removing_content() {
    let (html, styles) = render_scroll_area(|| {
        view! {
        <raw::Root hide_delay_ms=u32::MAX aria_label="Entries"><raw::Viewport>"Retained entries"</raw::Viewport></raw::Root>
    }.into_any()
    });
    assert!(html.contains("Retained entries"), "{html}");
    assert_eq!(styles.diagnostics().len(), 1, "{:?}", styles.diagnostics());
    assert_eq!(styles.diagnostics()[0].field, "hide_delay_ms");
    assert!(
        scroll_part(&html, "root").contains("data-rustify-issue=\"hide_delay_ms\""),
        "{html}"
    );
}

#[test]
fn only_the_native_viewport_is_named_and_tabbable() {
    let (html, styles) = render_scroll_area(|| {
        view! {
        <raw::Root aria_label="Entry history" boundary=raw::Boundary::Stop visibility=raw::Visibility::Always>
            <raw::Viewport><ol><li>"First entry"</li><li>"Second entry"</li></ol></raw::Viewport>
            <raw::Scrollbar orientation=Orientation::Horizontal><raw::Thumb /></raw::Scrollbar>
            <raw::Scrollbar><raw::Thumb /></raw::Scrollbar><raw::Corner />
        </raw::Root>
    }.into_any()
    });
    let root = scroll_part(&html, "root");
    let viewport = scroll_part(&html, "viewport");
    assert!(
        !root.contains("tabindex=") && !root.contains("aria-label=") && !root.contains("role="),
        "{root}"
    );
    assert!(
        viewport.contains("tabindex=\"0\"")
            && viewport.contains("role=\"group\"")
            && viewport.contains("aria-label=\"Entry history\""),
        "{viewport}"
    );
    assert!(
        viewport.contains("data-boundary=\"stop\"")
            && viewport.contains("data-scrollbars=\"both\""),
        "{viewport}"
    );
    assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
    assert!(
        html.contains("<ol>") && html.contains("First entry") && html.contains("Second entry"),
        "{html}"
    );
    for track in html
        .split('<')
        .filter(|tag| tag.contains("data-rustify-part=\"scroll-area-scrollbar\""))
    {
        assert!(
            track.contains("aria-hidden=\"true\"") && track.contains("data-disabled"),
            "{track}"
        );
        assert!(!track.contains("tabindex="), "{track}");
    }
    assert!(
        styles.diagnostics().is_empty(),
        "{:?}",
        styles.diagnostics()
    );
    assert!(!html.contains(" style="), "{html}");
}

#[test]
fn disallowed_axis_stays_hidden_and_initial_html_has_no_invented_scroll_metrics() {
    for allowed in raw::Scrollbars::ALL {
        let allowed = *allowed;
        let (html, styles) = render_scroll_area(move || {
            view! {
            <raw::Root scrollbars=allowed visibility=raw::Visibility::Always>
                <raw::Viewport aria_label="History">"A long history remains actual native content."</raw::Viewport>
                <raw::Scrollbar orientation=Orientation::Horizontal><raw::Thumb /></raw::Scrollbar>
                <raw::Scrollbar orientation=Orientation::Vertical><raw::Thumb /></raw::Scrollbar>
                <raw::Corner />
            </raw::Root>
        }.into_any()
        });
        let tracks: Vec<_> = html
            .split('<')
            .filter(|tag| tag.contains("data-rustify-part=\"scroll-area-scrollbar\""))
            .collect();
        assert_eq!(tracks.len(), 2, "{html}");
        assert_eq!(
            tracks[0].contains(" hidden"),
            allowed == raw::Scrollbars::Vertical,
            "{html}"
        );
        assert_eq!(
            tracks[1].contains(" hidden"),
            allowed == raw::Scrollbars::Horizontal,
            "{html}"
        );
        assert_eq!(
            scroll_part(&html, "corner").contains(" hidden"),
            allowed != raw::Scrollbars::Both,
            "{html}"
        );
        assert!(
            styles.css().contains("width:0px") && styles.css().contains("height:0px"),
            "{}",
            styles.css()
        );
        assert!(
            !html.contains("data-x-overflow") && !html.contains("data-y-overflow"),
            "{html}"
        );
    }
}

#[test]
fn root_direction_and_viewport_labelledby_preserve_native_attributes() {
    let (html, _) = render_scroll_area(|| {
        view! {
        <h2 id="history-label">"History"</h2>
        <raw::Root aria_label="Root fallback" direction=Some(Signal::stored(rustify_ui::Direction::RightToLeft)) attr:id="authored-scroll-area">
            <raw::Viewport labelled_by="history-label" attr:data-author="retained">"Entries"</raw::Viewport>
        </raw::Root>
    }.into_any()
    });
    let root = scroll_part(&html, "root");
    assert!(
        root.contains("dir=\"rtl\"") && root.contains("id=\"authored-scroll-area\""),
        "{root}"
    );
    let viewport = scroll_part(&html, "viewport");
    assert!(
        viewport.contains("aria-labelledby=\"history-label\"")
            && viewport.contains("data-author=\"retained\""),
        "{html}"
    );
}

#[test]
fn convenience_ids_still_identify_the_real_native_scrollport() {
    let (html, styles) = render_scroll_area(|| {
        view! {
        <styled::ScrollArea test_id="history" aria_label="History" class="h-32">"Native content"</styled::ScrollArea>
    }.into_any()
    });
    assert!(
        scroll_part(&html, "viewport").contains("data-testid=\"history\""),
        "{html}"
    );
    let root = scroll_part(&html, "root");
    assert!(
        root.contains("data-testid=\"history-root\"") && root.contains("h-32"),
        "{html}"
    );
    assert_eq!(
        html.matches("data-rustify-part=\"scroll-area-scrollbar\"")
            .count(),
        2,
        "{html}"
    );
    assert!(
        styles.diagnostics().is_empty(),
        "{:?}",
        styles.diagnostics()
    );
}

#[test]
fn scroll_area_sizes_radius_and_margin_use_collected_resolved_tokens() {
    for size in styled::ScrollAreaSize::ALL {
        let size = *size;
        let (html, styles) = render_scroll_area(move || {
            view! {
            <styled::Root size=Responsive { initial:Some(styled::ScrollAreaSize::One),md:Some(size),..Default::default() }
                radius=Some(rustify_ui::ThemeRadius::Full)
                margin=rustify_components::Margin { mx:Responsive::from(rustify_components::Spacing::Scale(-2)),..Default::default() }>
                <styled::Viewport aria_label="History">"Entries"</styled::Viewport>
                <styled::Scrollbar><styled::Thumb /></styled::Scrollbar>
            </styled::Root>
        }.into_any()
        });
        let css = styles.css();
        assert!(
            css.contains("@media (min-width:1024px)") || size == styled::ScrollAreaSize::One,
            "{css}"
        );
        assert!(
            css.contains(&format!(
                "--rustify-scrollbar-size:var(--rustify-space-{},",
                size.number()
            )),
            "{css}"
        );
        assert!(
            css.contains("--rustify-scrollbar-radius:9999px")
                && css.contains("margin-left:calc(var(--rustify-space-2) * -1)"),
            "{css}"
        );
        assert!(
            styles.diagnostics().is_empty(),
            "{:?}",
            styles.diagnostics()
        );
        assert!(!html.contains(" style="), "{html}");
    }
}

#[test]
fn missing_scroll_area_context_keeps_content_and_hides_orphan_controls() {
    let (html, _) = render_scroll_area(|| {
        view! {
            <raw::Viewport>"Readable content"</raw::Viewport>
            <raw::Scrollbar><raw::Thumb /></raw::Scrollbar>
            <raw::Corner />
        }
        .into_any()
    });
    assert!(
        html.contains("Readable content")
            && html.contains("missing-root")
            && html.contains("missing-scrollbar"),
        "{html}"
    );
    assert!(
        scroll_part(&html, "scrollbar").contains(" hidden"),
        "{html}"
    );
    assert!(scroll_part(&html, "thumb").contains(" hidden"), "{html}");
}
