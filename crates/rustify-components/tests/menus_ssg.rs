#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
use leptos::prelude::*;
use rustify_components::primitives::{context_menu as cm, dropdown_menu as dm, CheckedState};
fn render(view: impl FnOnce() -> AnyView + Send + 'static) -> String {
    Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "menus".into(),
                ..Default::default()
            },
            view,
        )
        .to_html()
    })
}
fn attribute<'a>(tag: &'a str, key: &str) -> &'a str {
    tag.split(&format!(" {key}=\""))
        .nth(1)
        .unwrap_or_default()
        .split('"')
        .next()
        .unwrap_or_default()
}
fn tag<'a>(html: &'a str, id: &str) -> &'a str {
    html.split('<')
        .find(|tag| attribute(tag, "data-testid") == id)
        .unwrap()
}
#[test]
fn missing_menu_contracts_keep_readable_content_and_disable_actions() {
    let html = render(|| {
        view!{
        <dm::Trigger>"Missing root"</dm::Trigger>
        <dm::Item on_select=|_|{}>"Unbound action"</dm::Item>
        <dm::SubTrigger>"Unbound submenu"</dm::SubTrigger>
        <dm::SubContent>"Readable submenu"</dm::SubContent>
        <dm::Root open=true on_open_change=|_|{}><dm::Trigger>"Actions"</dm::Trigger>
            <dm::Item on_select=|_|{}>"Missing content"</dm::Item>
            <dm::Content><dm::RadioItem value="a">"Missing radio group"</dm::RadioItem></dm::Content>
        </dm::Root>
    }.into_any()
    });
    for issue in [
        "missing-root",
        "missing-sub",
        "missing-content",
        "missing-radio-group",
    ] {
        assert!(html.contains(issue), "{issue}: {html}");
    }
    assert!(
        html.contains("aria-disabled=\"true\"") && html.contains(" disabled"),
        "{html}"
    );
    assert!(
        html.contains("Unbound action") && html.contains("Readable submenu"),
        "{html}"
    );
}
#[test]
fn static_menu_triggers_keep_closed_semantics_and_portals_stay_browser_only() {
    let html = render(|| {
        view!{
        <dm::Root open=false on_open_change=|_|{}><dm::Trigger test_id="trigger">"Document actions"</dm::Trigger>
            <dm::Portal force_mount=true><dm::Content><dm::Item on_select=|_|{}>"Private command"</dm::Item></dm::Content></dm::Portal>
        </dm::Root>
    }.into_any()
    });
    assert_eq!(attribute(tag(&html, "trigger"), "aria-haspopup"), "menu");
    assert_eq!(attribute(tag(&html, "trigger"), "aria-expanded"), "false");
    assert_eq!(attribute(tag(&html, "trigger"), "aria-controls"), "");
    assert!(
        !html.contains("Private command") && !html.contains("role=\"menu\""),
        "{html}"
    );
    Owner::new().with(|| {
        let issues=rustify_ui::StaticRenderIssues::default();
        rustify_ui::provide_static_render_issues(issues.clone());
        let html=view!{<cm::Root open=true on_open_change=|_|{}><cm::Trigger>"Preview"</cm::Trigger><cm::Portal><cm::Content>"Browser command"</cm::Content></cm::Portal></cm::Root>}.to_html();
        assert_eq!(issues.messages().len(),1);
        assert!(!html.contains("Browser command"),"{html}");
    });
}
#[test]
fn inline_menu_parts_retain_roles_and_associations_with_a_missing_portal_diagnostic() {
    let html = render(|| {
        view!{
        <dm::Root open=true on_open_change=|_|{}><dm::Trigger test_id="trigger">"Document actions"</dm::Trigger>
        <dm::Content test_id="content" aria_label="Document commands">
            <dm::Group test_id="group"><dm::Label test_id="label">"Document"</dm::Label><dm::Item on_select=|_|{}>"Open"</dm::Item></dm::Group>
            <dm::Separator test_id="separator" />
            <dm::CheckboxItem checked=CheckedState::Indeterminate on_checked_change=|_|{} test_id="checked">"Details"<dm::ItemIndicator>"Mixed"</dm::ItemIndicator></dm::CheckboxItem>
            <dm::RadioGroup value=Some("a".to_string()) on_value_change=|_|{}><dm::RadioItem value="a" test_id="radio">"Name"</dm::RadioItem><dm::RadioItem value="b">"Date"</dm::RadioItem></dm::RadioGroup>
            <dm::Sub open=true on_open_change=|_|{}><dm::SubTrigger test_id="sub-trigger">"Share"</dm::SubTrigger><dm::SubContent test_id="sub-content"><dm::Item on_select=|_|{}>"Copy link"</dm::Item></dm::SubContent></dm::Sub>
        </dm::Content></dm::Root>
    }.into_any()
    });
    assert_eq!(attribute(tag(&html, "content"), "role"), "menu");
    assert_eq!(
        attribute(tag(&html, "content"), "aria-label"),
        "Document commands"
    );
    assert!(!tag(&html, "content").contains("aria-labelledby="));
    assert_eq!(
        attribute(tag(&html, "content"), "aria-orientation"),
        "vertical"
    );
    assert_eq!(
        attribute(tag(&html, "trigger"), "aria-controls"),
        attribute(tag(&html, "content"), "id")
    );
    assert_eq!(
        attribute(tag(&html, "group"), "aria-labelledby"),
        attribute(tag(&html, "label"), "id")
    );
    assert_eq!(attribute(tag(&html, "checked"), "role"), "menuitemcheckbox");
    assert_eq!(attribute(tag(&html, "checked"), "aria-checked"), "mixed");
    assert_eq!(attribute(tag(&html, "radio"), "role"), "menuitemradio");
    assert_eq!(attribute(tag(&html, "radio"), "aria-checked"), "true");
    assert_eq!(
        attribute(tag(&html, "separator"), "aria-orientation"),
        "horizontal"
    );
    assert_eq!(
        attribute(tag(&html, "sub-trigger"), "aria-controls"),
        attribute(tag(&html, "sub-content"), "id")
    );
    assert!(
        html.contains("data-rustify-issue=\"missing-portal\""),
        "{html}"
    );
    assert!(
        !html.contains("rustify-menu-item"),
        "raw menu acquired a skin: {html}"
    );
}
#[test]
fn duplicate_radio_values_are_readable_but_unavailable() {
    let html = render(|| {
        view!{
        <dm::Root open=true on_open_change=|_|{}><dm::Trigger>"Actions"</dm::Trigger><dm::Content>
            <dm::RadioGroup value=Some("duplicate".to_string()) on_value_change=|_|{}><dm::RadioItem value="duplicate">"First"</dm::RadioItem><dm::RadioItem value="duplicate">"Second"</dm::RadioItem></dm::RadioGroup>
        </dm::Content></dm::Root>
    }.into_any()
    });
    let radios: Vec<_> = html
        .split('<')
        .filter(|tag| attribute(tag, "role") == "menuitemradio")
        .collect();
    assert_eq!(radios.len(), 2);
    for radio in radios {
        assert_eq!(attribute(radio, "data-rustify-issue"), "duplicate-value");
        assert_eq!(attribute(radio, "aria-disabled"), "true");
    }
}
#[test]
fn context_trigger_retains_its_native_target_and_menu_association() {
    let html = render(|| {
        view!{
        <cm::Root open=true on_open_change=|_|{}>
            {cm::trigger_as(leptos::html::article().tabindex(0).attr("data-testid","target").child("Document preview"))}
            <cm::Content test_id="content"><cm::Item on_select=|_|{}>"Copy"</cm::Item></cm::Content>
        </cm::Root>
    }.into_any()
    });
    assert!(tag(&html, "target").starts_with("article "), "{html}");
    assert_eq!(
        attribute(tag(&html, "content"), "aria-labelledby"),
        attribute(tag(&html, "target"), "id")
    );
}

#[test]
fn themed_menu_parameters_reach_parts_and_collect_responsive_styles() {
    use rustify_components::{dropdown_menu as themed, GeometryStyles, Responsive};
    for variant in themed::MenuVariant::ALL {
        let variant = *variant;
        let styles = GeometryStyles::default();
        let output = styles.clone();
        let html = render(move || {
            rustify_components::provide_geometry_styles(output);
            view!{<themed::Root open=true on_open_change=|_|{}><themed::Trigger>"Actions"</themed::Trigger>
                <themed::Content test_id="skin" size=Responsive{initial:Some(themed::MenuSize::One),md:Some(themed::MenuSize::Two),..Default::default()} variant=variant color=Some(rustify_ui::AccentColor::Blue) high_contrast=true class="p-0">
                    <themed::CheckboxItem checked=CheckedState::Checked on_checked_change=|_|{} test_id="check">"Details"</themed::CheckboxItem>
                    <themed::Sub open=true on_open_change=|_|{}><themed::SubTrigger>"Share"</themed::SubTrigger><themed::SubContent test_id="sub"><themed::Item on_select=|_|{}>"Copy link"</themed::Item></themed::SubContent></themed::Sub>
                </themed::Content>
            </themed::Root>}.into_any()
        });
        for part in ["skin", "sub"] {
            assert_eq!(
                attribute(tag(&html, part), "data-variant"),
                variant.as_str()
            );
            assert_eq!(attribute(tag(&html, part), "data-color"), "blue");
            assert_eq!(attribute(tag(&html, part), "data-high-contrast"), "true");
        }
        assert!(
            attribute(tag(&html, "skin"), "class").contains("p-0"),
            "{html}"
        );
        assert!(html.contains("rustify-menu-indicator"), "{html}");
        let css = styles.css();
        assert!(
            css.contains("--rustify-menu-height:var(--rustify-space-5, 24px)")
                && css.contains("--rustify-menu-height:var(--rustify-space-6, 32px)"),
            "{css}"
        );
        assert!(
            css.contains("@media") && css.contains("--rustify-component-color-9"),
            "{css}"
        );
        assert!(
            styles.diagnostics().is_empty(),
            "{:?}",
            styles.diagnostics()
        );
    }
}

#[test]
fn toolbar_missing_context_is_diagnostic_and_named_roots_keep_native_semantics() {
    use rustify_components::primitives::toolbar as tb;
    let html = render(|| {
        view!{
        <tb::Button test_id="orphan">"Unbound"</tb::Button>
        <tb::Root aria_label="" test_id="unnamed"><tb::Button>"Action"</tb::Button></tb::Root>
        <tb::Root aria_label="Editing" orientation=rustify_components::Orientation::Vertical test_id="tools">
            <tb::Button disabled=true test_id="disabled">"Unavailable"</tb::Button>
            <tb::Button test_id="first">"Undo"</tb::Button>
            <tb::Link href="/help" test_id="link">"Help"</tb::Link>
            <tb::Separator test_id="divider" />
        </tb::Root>
    }.into_any()
    });
    assert_eq!(
        attribute(tag(&html, "orphan"), "data-rustify-issue"),
        "missing-root"
    );
    assert_eq!(
        attribute(tag(&html, "unnamed"), "data-rustify-issue"),
        "missing-name"
    );
    assert_eq!(attribute(tag(&html, "tools"), "role"), "toolbar");
    assert_eq!(
        attribute(tag(&html, "tools"), "aria-orientation"),
        "vertical"
    );
    assert_eq!(attribute(tag(&html, "disabled"), "tabindex"), "-1");
    assert_eq!(attribute(tag(&html, "first"), "tabindex"), "0");
    assert_eq!(attribute(tag(&html, "link"), "tabindex"), "-1");
    assert_eq!(attribute(tag(&html, "link"), "href"), "/help");
    assert!(
        !tag(&html, "link").contains("download")
            && !tag(&html, "link").contains("data-rustify-native-navigation")
    );
    assert_eq!(
        attribute(tag(&html, "divider"), "aria-orientation"),
        "horizontal"
    );
}
#[test]
fn toolbar_owns_the_only_tab_stop_across_selected_toggle_groups_and_menu_triggers() {
    use rustify_components::primitives::{toggle_group, toolbar as tb};
    let html = render(|| {
        view!{
        <tb::Root aria_label="Editing">
            <tb::ToggleGroup value=vec!["italic".to_string()] on_change=|_|{} test_id="group">
                <tb::ToggleItem value="bold" test_id="bold">"Bold"</tb::ToggleItem>
                <tb::ToggleItem value="italic" test_id="italic">"Italic"</tb::ToggleItem>
            </tb::ToggleGroup>
            <dm::Root open=false on_open_change=|_|{}>
                {dm::trigger_as(leptos::html::button().attr("data-testid","embedded").add_any_attr(tb::item(false.into())).child("More"))}
                <dm::Portal><dm::Content>"Commands"</dm::Content></dm::Portal>
            </dm::Root>
            <tb::ToggleGroup value={Vec::<String>::new()} on_change=|_|{} disabled=true>
                <tb::ToggleItem value="strike" test_id="strike">"Strike"</tb::ToggleItem>
            </tb::ToggleGroup>
        </tb::Root>
        <toggle_group::Root value=vec!["b".to_string()] on_change=|_|{} toolbar=true>
            <toggle_group::Item value="a" test_id="standalone-a">"A"</toggle_group::Item>
            <toggle_group::Item value="b" test_id="standalone-b">"B"</toggle_group::Item>
        </toggle_group::Root>
    }.into_any()
    });
    assert_eq!(attribute(tag(&html, "group"), "role"), "group");
    for (id, stop) in [
        ("bold", "0"),
        ("italic", "-1"),
        ("embedded", "-1"),
        ("strike", "-1"),
        ("standalone-a", "-1"),
        ("standalone-b", "0"),
    ] {
        assert_eq!(attribute(tag(&html, id), "tabindex"), stop, "{id}: {html}");
        assert_eq!(
            tag(&html, id).matches(" tabindex=").count(),
            1,
            "{id}: {html}"
        );
    }
    assert_eq!(attribute(tag(&html, "italic"), "aria-pressed"), "true");
    assert!(tag(&html, "strike").contains(" disabled"));
}
#[test]
fn menubar_static_triggers_have_one_stop_and_browser_only_panels() {
    use rustify_components::menubar as mb;
    let html = render(|| {
        view!{
        <mb::Root value={None::<String>} on_value_change=|_|{} aria_label="Application" test_id="bar">
            <mb::Menu value="disabled"><mb::Trigger disabled=true test_id="bar-disabled">"Disabled"</mb::Trigger><mb::Portal><mb::Content>"Hidden"</mb::Content></mb::Portal></mb::Menu>
            <mb::Menu value="file"><mb::Trigger test_id="bar-file">"File"</mb::Trigger><mb::Portal><mb::Content>"Private command"</mb::Content></mb::Portal></mb::Menu>
            <mb::Menu value="edit"><mb::Trigger test_id="bar-edit">"Edit"</mb::Trigger><mb::Portal><mb::Content>"Edit command"</mb::Content></mb::Portal></mb::Menu>
        </mb::Root>
    }.into_any()
    });
    assert_eq!(attribute(tag(&html, "bar"), "role"), "menubar");
    for (id, stop) in [
        ("bar-disabled", "-1"),
        ("bar-file", "0"),
        ("bar-edit", "-1"),
    ] {
        let trigger = tag(&html, id);
        assert_eq!(attribute(trigger, "tabindex"), stop, "{html}");
        assert_eq!(attribute(trigger, "role"), "menuitem");
        assert_eq!(attribute(trigger, "aria-haspopup"), "menu");
        assert_eq!(attribute(trigger, "aria-expanded"), "false");
        assert_eq!(attribute(trigger, "aria-controls"), "");
    }
    assert!(!html.contains("Private command") && !html.contains("Edit command"));
}

#[test]
fn navigation_rejects_invalid_delays_and_unbound_parts_without_losing_labels() {
    use rustify_components::primitives::navigation_menu as nav;
    let html = render(|| {
        view!{
        <nav::Trigger test_id="nav-unbound">"Unbound"</nav::Trigger>
        <nav::Root value={None::<String>} on_value_change=|_|{} aria_label="Navigation" delay_ms=u32::MAX test_id="nav-invalid">
            <nav::List><nav::Item value="learn"><nav::Trigger test_id="nav-invalid-trigger">"Learn"</nav::Trigger></nav::Item></nav::List>
        </nav::Root>
        <nav::Root value={None::<String>} on_value_change=|_|{} aria_label="" test_id="nav-unnamed"><nav::List><li>"Destination"</li></nav::List></nav::Root>
    }.into_any()
    });
    assert_eq!(
        attribute(tag(&html, "nav-unbound"), "data-rustify-issue"),
        "missing-item"
    );
    assert_eq!(
        attribute(tag(&html, "nav-invalid"), "data-rustify-issue"),
        "invalid-delay"
    );
    assert!(tag(&html, "nav-invalid-trigger").contains(" disabled"));
    assert_eq!(
        attribute(tag(&html, "nav-unnamed"), "data-rustify-issue"),
        "missing-name"
    );
    assert!(html.contains("Unbound") && html.contains("Learn"));
}
#[test]
fn navigation_static_content_is_inline_and_hidden_links_remain_native() {
    use rustify_components::navigation_menu as nav;
    let html = render(|| {
        view!{
        <nav::Root value={None::<String>} on_value_change=|_|{} aria_label="Documentation" test_id="navigation">
            <nav::List>
                <nav::Item value="learn"><nav::Trigger test_id="nav-learn">"Learn"</nav::Trigger>
                    <nav::Content force_mount=true test_id="nav-content">
                        <nav::Link href="/guide" active=true test_id="nav-guide">"Guide"</nav::Link>
                        <nav::Link href="/download" target="_blank" rel="noopener" download=Some("guide.pdf".to_string()) test_id="nav-download">"Download guide"</nav::Link>
                    </nav::Content>
                </nav::Item>
                <nav::Item value="about"><nav::Link href="/about" test_id="nav-about">"About"</nav::Link></nav::Item>
            </nav::List>
            <nav::Indicator test_id="nav-indicator" /><nav::Viewport test_id="nav-viewport" />
        </nav::Root>
    }.into_any()
    });
    assert!(tag(&html, "navigation").starts_with("nav "));
    assert!(tag(&html, "navigation").contains("data-viewport"));
    let trigger = tag(&html, "nav-learn");
    let content = tag(&html, "nav-content");
    assert_eq!(attribute(trigger, "aria-expanded"), "false");
    assert_eq!(attribute(trigger, "aria-controls"), "");
    assert!(!trigger.contains("tabindex"));
    assert!(content.contains(" hidden") && content.contains(" inert"));
    assert_eq!(
        attribute(content, "aria-labelledby"),
        attribute(trigger, "id")
    );
    let guide = tag(&html, "nav-guide");
    assert_eq!(attribute(guide, "href"), "/guide");
    assert_eq!(attribute(guide, "aria-current"), "page");
    assert!(guide.contains("data-rustify-native-navigation"));
    assert!(!guide.contains("tabindex"));
    assert_eq!(attribute(tag(&html, "nav-download"), "target"), "_blank");
    assert_eq!(
        attribute(tag(&html, "nav-download"), "download"),
        "guide.pdf"
    );
    assert!(html.find("nav-content").unwrap() < html.find("</li>").unwrap());
    assert!(html.find("</ul>").unwrap() < html.find("data-testid=\"nav-indicator\"").unwrap());
    assert_eq!(
        attribute(tag(&html, "nav-indicator"), "data-state"),
        "hidden"
    );
    assert!(tag(&html, "nav-viewport").contains(" hidden"));
}
#[test]
fn sub_navigation_requires_its_own_item_context() {
    use rustify_components::primitives::navigation_menu as nav;
    let html = render(|| {
        view!{
        <nav::Root value={None::<String>} on_value_change=|_|{} aria_label="Documentation"><nav::List><nav::Item value="outer">
            <nav::Trigger>"Outer"</nav::Trigger><nav::Content force_mount=true>
                <nav::Sub value={None::<String>} on_value_change=|_|{}><nav::Trigger test_id="sub-unbound">"Unbound child"</nav::Trigger></nav::Sub>
            </nav::Content>
        </nav::Item></nav::List></nav::Root>
    }.into_any()
    });
    assert_eq!(
        attribute(tag(&html, "sub-unbound"), "data-rustify-issue"),
        "missing-item"
    );
    assert!(tag(&html, "sub-unbound").contains(" disabled"));
}
