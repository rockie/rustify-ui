#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
use leptos::prelude::*;
use rustify_components::primitives::tabs::{self, PanelEntry};
fn render(view: impl FnOnce() -> AnyView + Send + 'static) -> String {
    Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "panels".into(),
                ..Default::default()
            },
            view,
        )
        .to_html()
    })
}
#[test]
fn illegal_entries_remain_readable_but_cannot_be_activated() {
    let html = render(|| {
        view! {
        <tabs::Root value=Some("duplicate".to_string()) entries=vec![PanelEntry::new("duplicate"), PanelEntry::new("duplicate"), PanelEntry::new("")] on_change=|_| {}>
            <tabs::List aria_label="Views"><tabs::Trigger value="duplicate">"Duplicate"</tabs::Trigger><tabs::Trigger value="">"Empty"</tabs::Trigger></tabs::List>
            <tabs::Content value="duplicate">"Retained content"</tabs::Content>
        </tabs::Root>
    }.into_any()
    });
    assert!(html.contains("duplicate-entry"), "{html}");
    assert!(html.contains("empty-entry"), "{html}");
    assert!(html.contains("disabled"), "{html}");
    assert!(html.contains("Retained content"), "{html}");
    assert!(!html.contains("aria-selected=\"true\""), "{html}");
}
#[test]
fn panels_keep_content_and_stable_associations_without_business_keys_in_ids() {
    let html = render(|| {
        view! {
        <tabs::Root value=Some("a / ' [key]".to_string()) entries=vec![PanelEntry::new("a / ' [key]"),PanelEntry::new("b")] on_change=|_| {}>
            <tabs::List aria_label="Views"><tabs::Trigger value="a / ' [key]">"First"</tabs::Trigger><tabs::Trigger value="b">"Second"</tabs::Trigger></tabs::List>
            <tabs::Content value="a / ' [key]">"First body"</tabs::Content>
            <tabs::Content value="b"><input value="Keep me" /></tabs::Content>
        </tabs::Root>
    }.into_any()
    });
    assert!(html.contains("role=\"tablist\""), "{html}");
    assert!(html.contains("aria-selected=\"true\""), "{html}");
    assert!(html.contains("Keep me"), "{html}");
    assert!(
        html.contains(" hidden") && html.contains(" inert"),
        "{html}"
    );
    let triggers: Vec<_> = html
        .split('<')
        .filter(|tag| tag.contains("role=\"tab\""))
        .collect();
    let panels: Vec<_> = html
        .split('<')
        .filter(|tag| tag.contains("role=\"tabpanel\""))
        .collect();
    for (trigger, panel) in triggers.iter().zip(panels) {
        assert_eq!(attribute(trigger, "aria-controls"), attribute(panel, "id"));
        assert_eq!(
            attribute(trigger, "id"),
            attribute(panel, "aria-labelledby")
        );
        assert!(!attribute(trigger, "id").contains("key"));
    }
}
#[test]
fn missing_root_keeps_labels_and_disables_trigger() {
    let html = render(|| {
        view! { <tabs::Trigger value="a">"Readable"</tabs::Trigger><tabs::Content value="a">"Body"</tabs::Content> }.into_any()
    });
    assert!(
        html.contains("missing-root") && html.contains("disabled"),
        "{html}"
    );
    assert!(html.contains("Readable") && html.contains("Body"), "{html}");
}

#[test]
fn force_mounted_disclosures_are_inert_and_keep_heading_associations() {
    use rustify_components::primitives::{accordion as a, collapsible as c};
    let html = render(|| {
        view!{
        <c::Root open=false on_open_change=|_|{}><c::Trigger>"More"</c::Trigger><c::Content force_mount=true><button>"Hidden action"</button></c::Content></c::Root>
        <a::Root value=a::AccordionValue::Single(Some("a".into())) on_change=|_|{}>
            <a::Item value="a"><a::Header level=rustify_components::HeadingLevel::H2><a::Trigger>"Section"</a::Trigger></a::Header><a::Content>"Expanded"</a::Content></a::Item>
        </a::Root>
    }.into_any()
    });
    assert!(
        html.contains(" hidden") && html.contains(" inert") && html.contains("Hidden action"),
        "{html}"
    );
    assert!(html.contains("<h2") && html.contains("Expanded"), "{html}");
    assert!(html.contains("aria-expanded=\"true\""), "{html}");
    let headers: Vec<_> = html
        .split('<')
        .filter(|tag| tag.contains("data-rustify-part=\"collapsible-trigger\""))
        .collect();
    let panels: Vec<_> = html
        .split('<')
        .filter(|tag| tag.contains("data-rustify-part=\"collapsible-content\""))
        .collect();
    for (trigger, panel) in headers.iter().zip(panels) {
        assert_eq!(
            attribute(trigger, "id"),
            attribute(panel, "aria-labelledby")
        );
        if attribute(trigger, "aria-expanded") == "true" {
            assert_eq!(attribute(trigger, "aria-controls"), attribute(panel, "id"));
        } else {
            assert!(!trigger.contains("aria-controls"), "{trigger}");
        }
    }
}
#[test]
fn a_closed_disclosure_never_points_at_an_unmounted_panel() {
    use rustify_components::primitives::collapsible as c;
    let html = render(|| {
        view! {
        <c::Root open=false on_open_change=|_|{}><c::Trigger>"More"</c::Trigger><c::Content>"Unmounted body"</c::Content></c::Root>
    }.into_any()
    });
    assert!(!html.contains("Unmounted body"), "{html}");
    assert!(!html.contains("aria-controls"), "{html}");
}
fn attribute<'a>(tag: &'a str, name: &str) -> &'a str {
    tag.split(&format!(" {name}=\""))
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
}
#[test]
fn duplicate_accordion_keys_are_disabled_and_never_expand() {
    use rustify_components::primitives::accordion as a;
    let html = render(|| {
        view!{<a::Root value=a::AccordionValue::Single(Some("a".into())) on_change=|_|{}>
        <a::Item value="a"><a::Trigger>"First"</a::Trigger><a::Content force_mount=true>"First body"</a::Content></a::Item>
        <a::Item value="a"><a::Trigger>"Second"</a::Trigger><a::Content force_mount=true>"Second body"</a::Content></a::Item>
    </a::Root>}.into_any()
    });
    assert!(html.contains("duplicate-entry"), "{html}");
    assert_eq!(html.matches("aria-expanded=\"false\"").count(), 2, "{html}");
    assert_eq!(html.matches(" disabled").count(), 2, "{html}");
}
#[test]
fn segmented_choice_and_navigation_keep_native_semantics() {
    use rustify_components::{segmented_control as s, tab_nav as n};
    let html = render(|| {
        view!{<s::Root value=Some("list".to_string()) name="view" on_change=|_|{} aria_label="View"><s::Item value="list">"List"</s::Item><s::Item value="grid">"Grid"</s::Item></s::Root>
        <n::Root aria_label="Pages"><n::Link href="/entries" active=true target="_blank" rel="noopener">"Entries"</n::Link></n::Root>
    }.into_any()
    });
    assert!(
        html.contains("role=\"radiogroup\"") && html.contains("type=\"radio\""),
        "{html}"
    );
    assert!(
        html.contains("name=\"view\"") && html.contains("value=\"list\""),
        "{html}"
    );
    assert!(
        html.contains("<nav")
            && html.contains("href=\"/entries\"")
            && html.contains("aria-current=\"page\""),
        "{html}"
    );
    assert!(
        html.contains("target=\"_blank\"") && html.contains("rel=\"noopener\""),
        "{html}"
    );
    assert!(!html.contains("role=\"tab"), "{html}");
}

#[test]
fn delayed_panels_and_duplicate_parts_keep_unique_relationships() {
    let html = render(|| {
        view!{
        <tabs::Root value=Some("a".to_string()) entries=vec![PanelEntry::new("a")] on_change=|_|{}>
            <tabs::List aria_label="Views"><tabs::Trigger value="a">"First"</tabs::Trigger><tabs::Trigger value="a">"Duplicate"</tabs::Trigger></tabs::List>
            <Show when=||true><tabs::Content value="a">"Delayed body"</tabs::Content></Show>
        </tabs::Root>
    }.into_any()
    });
    let triggers: Vec<_> = html
        .split('<')
        .filter(|tag| tag.contains("role=\"tab\""))
        .collect();
    assert_eq!(triggers.len(), 2, "{html}");
    assert_ne!(attribute(triggers[0], "id"), attribute(triggers[1], "id"));
    assert!(
        triggers
            .iter()
            .all(|tag| tag.contains(" disabled") && tag.contains("duplicate-part")),
        "{html}"
    );
    let panel = html
        .split('<')
        .find(|tag| tag.contains("role=\"tabpanel\""))
        .unwrap();
    assert_eq!(
        attribute(triggers[0], "aria-controls"),
        attribute(panel, "id")
    );
    assert_eq!(
        attribute(triggers[0], "id"),
        attribute(panel, "aria-labelledby")
    );
    assert!(html.contains("Delayed body"), "{html}");
}
#[test]
fn every_declared_panel_size_and_segment_variant_generates_consumable_geometry() {
    use rustify_components::{segmented_control as s, tabs as t, GeometryStyles, Responsive};
    for size in t::TabsSize::ALL {
        let styles = GeometryStyles::default();
        let output = styles.clone();
        let size = *size;
        let html = render(move || {
            rustify_components::provide_geometry_styles(output);
            view!{
            <t::Root value=Some("a".to_string()) entries=vec![PanelEntry::new("a")] on_change=|_|{}>
                <t::List size=size aria_label="Views"><t::Trigger value="a">"A"</t::Trigger></t::List><t::Content value="a">"Body"</t::Content>
            </t::Root>
        }.into_any()
        });
        assert!(html.contains("Body"), "{html}");
        assert!(
            styles.css().contains(if size == t::TabsSize::One {
                "--rustify-tab-height:var(--rustify-space-6, 32px)"
            } else {
                "--rustify-tab-height:var(--rustify-space-7, 40px)"
            }),
            "{}",
            styles.css()
        );
    }
    for size in s::SegmentedControlSize::ALL {
        for variant in s::SegmentedControlVariant::ALL {
            let styles = GeometryStyles::default();
            let output = styles.clone();
            let (size, variant) = (*size, *variant);
            let html = render(move || {
                rustify_components::provide_geometry_styles(output);
                view!{
            <s::Root value=Some("a".to_string()) on_change=|_|{} aria_label="View" size=Responsive::from(size) variant=variant><s::Item value="a">"A"</s::Item></s::Root>
        }.into_any()
            });
            assert!(
                html.contains(&format!("data-variant=\"{}\"", variant.as_str())),
                "{html}"
            );
            assert!(
                styles.css().contains("--rustify-segment-height:"),
                "{}",
                styles.css()
            );
        }
    }
}

#[test]
fn closed_dialog_triggers_have_native_semantics_without_portal_content() {
    use rustify_components::primitives::{alert_dialog, dialog};
    let html = render(|| {
        view!{
        <dialog::Root open=false on_open_change=|_|{}><dialog::Trigger>"Edit item"</dialog::Trigger><dialog::Portal><dialog::Content><dialog::Title>"Private editor"</dialog::Title><dialog::Close>"Close"</dialog::Close></dialog::Content></dialog::Portal></dialog::Root>
        <alert_dialog::Root open=false on_open_change=|_|{}><alert_dialog::Trigger>"Delete item"</alert_dialog::Trigger><alert_dialog::Portal><alert_dialog::Content><alert_dialog::Title>"Delete item?"</alert_dialog::Title><alert_dialog::Description>"This cannot be undone"</alert_dialog::Description><alert_dialog::Cancel>"Keep item"</alert_dialog::Cancel><alert_dialog::Action on_action=|_|{}>"Delete"</alert_dialog::Action></alert_dialog::Content></alert_dialog::Portal></alert_dialog::Root>
    }.into_any()
    });
    assert!(html.contains("aria-haspopup=\"dialog\""), "{html}");
    assert!(html.contains("aria-expanded=\"false\""), "{html}");
    assert!(
        html.contains("Edit item") && html.contains("Delete item"),
        "{html}"
    );
    assert!(
        !html.contains("Private editor") && !html.contains("This cannot be undone"),
        "{html}"
    );
    assert!(
        !html.contains("aria-modal") && !html.contains("role=\"dialog\""),
        "{html}"
    );
}
#[test]
fn orphan_dialog_and_missing_portal_content_remain_readable_and_diagnostic() {
    use rustify_components::primitives::{alert_dialog, dialog};
    let html = render(|| {
        view!{
        <dialog::Content>"Orphan editor"</dialog::Content>
        <alert_dialog::Action on_action=|_|{}>"Orphan action"</alert_dialog::Action>
        <dialog::Root open=true on_open_change=|_|{}><dialog::Content>"Editor without portal"</dialog::Content></dialog::Root>
    }.into_any()
    });
    assert!(
        html.contains("Orphan editor")
            && html.contains("Orphan action")
            && html.contains("Editor without portal"),
        "{html}"
    );
    assert!(
        html.contains("missing-root") && html.contains("missing-portal"),
        "{html}"
    );
    assert!(html.contains("disabled"), "{html}");
}
#[test]
fn an_open_dialog_portal_reports_the_static_render_boundary() {
    use rustify_components::primitives::dialog;
    Owner::new().with(||{
        let issues=rustify_ui::StaticRenderIssues::default();rustify_ui::provide_static_render_issues(issues.clone());
        let html=view!{<dialog::Root open=true on_open_change=|_|{}><dialog::Portal><dialog::Content><dialog::Title>"Not static"</dialog::Title></dialog::Content></dialog::Portal></dialog::Root>}.to_html();
        assert_eq!(issues.messages().len(),1);assert!(!html.contains("Not static"),"{html}");
    });
}

#[test]
fn dialog_sizes_and_responsive_dimensions_generate_scoped_geometry() {
    use rustify_components::{dialog, GeometryStyles, Layout, Margin, Responsive, Spacing};
    for size in dialog::DialogSize::ALL {
        let styles = GeometryStyles::default();
        let output = styles.clone();
        let size = *size;
        let html = render(move || {
            rustify_components::provide_geometry_styles(output);
            view!{
            <dialog::Root open=false on_open_change=|_|{}><dialog::Content size=size margin=Margin {mt:Responsive::from(Spacing::from("-2px")),..Default::default()} layout=Layout {width:Responsive {initial:Some("320px".into()),sm:Some("500px".into()),..Default::default()},max_width:Responsive::from(String::from("550px")),..Default::default()}>
                <dialog::Title margin=Margin {mb:Responsive::from(Spacing::from("18px")),..Default::default()}>"Readable composition diagnostic"</dialog::Title>
                <dialog::Description margin=Margin {mt:Responsive::from(Spacing::from("3px")),..Default::default()}>"Details"</dialog::Description>
            </dialog::Content></dialog::Root>
        }.into_any()
        });
        assert!(html.contains("Readable composition diagnostic"), "{html}");
        let css = styles.css();
        let (space, padding) = match size {
            dialog::DialogSize::One => (3, 12),
            dialog::DialogSize::Two => (4, 16),
            dialog::DialogSize::Three => (5, 24),
            dialog::DialogSize::Four => (6, 32),
        };
        assert!(
            css.contains(&format!(
                "--rustify-dialog-padding:var(--rustify-space-{space}, {padding}px)"
            )),
            "{css}"
        );
        assert!(
            css.contains("--rustify-dialog-width:320px")
                && css.contains("--rustify-dialog-width:500px")
                && css.contains("--rustify-dialog-max-width:550px"),
            "{css}"
        );
        assert!(css.contains("768px"), "{css}");
        assert!(
            css.contains("margin-top:-2px")
                && css.contains("margin-bottom:18px")
                && css.contains("margin-top:3px"),
            "{css}"
        );
        assert!(styles.diagnostics().is_empty());
    }
}
#[test]
fn invalid_dialog_dimensions_are_diagnostic_without_injecting_declarations() {
    use rustify_components::{dialog, GeometryStyles, Layout, Responsive};
    let styles = GeometryStyles::default();
    let output = styles.clone();
    let html = render(move || {
        rustify_components::provide_geometry_styles(output);
        view!{
        <dialog::Content layout=Layout {width:Responsive::from(String::from("10px;position:fixed")),..Default::default()}>"Retained invalid layout"</dialog::Content>
    }.into_any()
    });
    assert!(html.contains("Retained invalid layout"));
    assert_eq!(styles.diagnostics()[0].field, "width");
    assert!(!styles.css().contains("position:fixed"));
}

#[test]
fn modeless_dialogs_do_not_mount_a_pointer_blocking_overlay() {
    let html = render(|| {
        view!{<rustify_components::primitives::dialog::Root open=false modal=false on_open_change=|_|{}>
        <rustify_components::primitives::dialog::Overlay test_id="modeless-overlay" />
    </rustify_components::primitives::dialog::Root>}.into_any()
    });
    assert!(!html.contains("modeless-overlay"), "{html}");
}

#[test]
fn closed_popover_has_one_native_trigger_and_no_static_layer() {
    use rustify_components::primitives::popover;
    let html = render(|| {
        view!{<popover::Root open=false on_open_change=|_|{}>
        {popover::trigger_as(leptos::html::button().attr("data-custom","kept").child("Edit schedule"))}
        <popover::Portal force_mount=true><popover::Content aria_label="Schedule">"Not static"</popover::Content></popover::Portal>
    </popover::Root>}.into_any()
    });
    assert_eq!(html.matches("<button").count(), 1, "{html}");
    assert!(
        html.contains("data-custom=\"kept\"")
            && html.contains("aria-haspopup=\"dialog\"")
            && html.contains("aria-expanded=\"false\""),
        "{html}"
    );
    assert!(
        !html.contains("Not static") && !html.contains("role=\"dialog\""),
        "{html}"
    );
}
#[test]
fn popover_rejects_duplicate_anchors_and_keeps_orphan_content_readable() {
    use rustify_components::primitives::popover;
    let html = render(|| {
        view!{
        <popover::Content aria_label="Orphan">"Readable orphan"</popover::Content>
        <popover::Root open=false on_open_change=|_|{}>
            <popover::Anchor>"First position"</popover::Anchor><popover::Anchor>"Second position"</popover::Anchor>
            <popover::Trigger>"Edit"</popover::Trigger><popover::Portal><popover::Content aria_label="Task">"Task"</popover::Content></popover::Portal>
        </popover::Root>
    }.into_any()
    });
    assert!(
        html.contains("Readable orphan")
            && html.contains("missing-root")
            && html.contains("duplicate-part"),
        "{html}"
    );
}

#[test]
fn themed_popover_rejects_negative_dimensions_before_writing_css_variables() {
    use rustify_components::{popover, GeometryStyles, Layout, Responsive};
    let styles = GeometryStyles::default();
    let output = styles.clone();
    let html = render(move || {
        rustify_components::provide_geometry_styles(output);
        view!{<popover::Root open=false on_open_change=|_|{}><popover::Content aria_label="Details" layout=Layout {width:Responsive::from(String::from("-1px")),..Default::default()}>"Retained details"</popover::Content></popover::Root>}.into_any()
    });
    assert!(html.contains("Retained details"));
    assert!(!styles.css().contains("-1px"));
    assert!(styles
        .diagnostics()
        .iter()
        .any(|error| error.field == "width"));
}
#[test]
fn every_popover_size_and_responsive_dimension_is_collected_for_static_styles() {
    use rustify_components::{popover, GeometryStyles, Layout, Margin, Responsive, Spacing};
    for size in popover::PopoverSize::ALL {
        let styles = GeometryStyles::default();
        let output = styles.clone();
        let size = *size;
        render(move || {
            rustify_components::provide_geometry_styles(output);
            view!{<popover::Root open=false on_open_change=|_|{}><popover::Content aria_label="Details" size=size
                layout=Layout {max_width:Responsive {initial:Some("240px".into()),sm:Some("400px".into()),..Default::default()},..Default::default()}
                margin=Margin {mt:Responsive::from(Spacing::from("-2px")),..Default::default()}>"Details"</popover::Content></popover::Root>}.into_any()
        });
        let css = styles.css();
        let (space, pixels) = match size {
            popover::PopoverSize::One => (3, 12),
            popover::PopoverSize::Two => (4, 16),
            popover::PopoverSize::Three => (5, 24),
            popover::PopoverSize::Four => (6, 32),
        };
        assert!(
            css.contains(&format!(
                "--rustify-popover-padding:var(--rustify-space-{space}, {pixels}px)"
            )),
            "{css}"
        );
        assert!(
            css.contains("--rustify-popover-max-width:240px")
                && css.contains("--rustify-popover-max-width:400px")
                && css.contains("margin-top:-2px"),
            "{css}"
        );
        assert!(styles.diagnostics().is_empty());
    }
}

#[test]
fn supplemental_reading_keeps_native_links_and_a_single_typed_target_in_static_html() {
    use rustify_components::{hover_card, tooltip};
    let html = render(|| {
        view!{
        <hover_card::Root open=false on_open_change=|_|{}><hover_card::Trigger href="/people/morgan">"Morgan profile"</hover_card::Trigger><hover_card::Portal force_mount=true><hover_card::Content>"Private browser content"</hover_card::Content></hover_card::Portal></hover_card::Root>
        <rustify_components::Tooltip open=false on_open_change=|_|{} label="Browser description" trigger=leptos::html::button().r#type("button").attr("data-original","retained").attr("aria-describedby","authored-help").child("Read help") />
        <tooltip::Root open=false on_open_change=|_|{}><tooltip::Trigger>"More help"</tooltip::Trigger><tooltip::Portal><tooltip::Content>"Other browser description"</tooltip::Content></tooltip::Portal></tooltip::Root>
    }.into_any()
    });
    assert!(
        html.contains("href=\"/people/morgan\"") && html.contains("Morgan profile"),
        "{html}"
    );
    assert_eq!(html.matches("<button").count(), 2, "{html}");
    assert!(
        html.contains("data-original=\"retained\"")
            && html.contains("aria-describedby=\"authored-help\""),
        "{html}"
    );
    assert!(
        !html.contains("Browser description")
            && !html.contains("Private browser content")
            && !html.contains("Other browser description"),
        "{html}"
    );
    assert!(
        !html.contains("<span") && !html.contains("role=\"tooltip\""),
        "{html}"
    );
}

#[test]
fn open_hover_portals_report_the_static_render_boundary() {
    use rustify_components::primitives::{hover_card, tooltip};
    Owner::new().with(||{
        let issues=rustify_ui::StaticRenderIssues::default();rustify_ui::provide_static_render_issues(issues.clone());
        let html=view!{
            <hover_card::Root open=true on_open_change=|_|{}><hover_card::Trigger href="/profile">"Profile"</hover_card::Trigger><hover_card::Portal force_mount=true><hover_card::Content>"Not static"</hover_card::Content></hover_card::Portal></hover_card::Root>
            <tooltip::Root open=true on_open_change=|_|{}><tooltip::Trigger>"Help"</tooltip::Trigger><tooltip::Portal><tooltip::Content>"Not static"</tooltip::Content></tooltip::Portal></tooltip::Root>
        }.to_html();
        assert_eq!(issues.messages().len(),2);assert!(!html.contains("Not static"),"{html}");
    });
}

#[test]
fn invalid_tooltip_width_is_rejected_before_custom_property_generation() {
    use rustify_components::{tooltip, GeometryStyles, Responsive};
    let styles = GeometryStyles::default();
    let output = styles.clone();
    let html = render(move || {
        rustify_components::provide_geometry_styles(output);
        view!{
        <tooltip::Root open=false on_open_change=|_|{}><tooltip::Content width=Responsive::from(String::from("-1px"))>"Readable invalid width"</tooltip::Content></tooltip::Root>
    }.into_any()
    });
    assert!(html.contains("Readable invalid width"), "{html}");
    assert!(styles
        .diagnostics()
        .iter()
        .any(|error| error.field == "width"));
    assert!(!styles.css().contains("-1px"));
}
