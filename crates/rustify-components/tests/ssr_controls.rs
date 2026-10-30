#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use leptos::prelude::*;
use rustify_components::{
    Checkbox, RadioGroup, RadioOption, Slider, Switch, Tab, Tabs, TextArea, TextField, ToggleGroup,
    ToggleItem,
};

fn render<N: IntoView>(view: impl FnOnce() -> N) -> String {
    Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "native-controls".to_string(),
                ..Default::default()
            },
            view,
        )
        .to_html()
    })
}

fn opening_tag<'a>(html: &'a str, test_id: &str) -> &'a str {
    let marker = format!("data-testid=\"{test_id}\"");
    let at = html.find(&marker).expect("control test id");
    let start = html[..at].rfind('<').expect("opening tag");
    let end = at + html[at..].find('>').expect("end of opening tag");
    &html[start..=end]
}

fn has_boolean_attribute(tag: &str, name: &str) -> bool {
    tag.contains(&format!(" {name} ")) || tag.ends_with(&format!(" {name}>"))
}

#[test]
fn text_field_serializes_its_initial_value_and_native_flags() {
    let html = render(|| {
        view! {
            <TextField value="rust & ui" on_change=|_| {} disabled=true read_only=true test_id="text" />
        }
    });
    let tag = opening_tag(&html, "text");
    assert!(
        tag.contains("value=\"rust &amp; ui\"")
            && has_boolean_attribute(tag, "disabled")
            && has_boolean_attribute(tag, "readonly"),
        "{tag}"
    );
}

#[test]
fn writable_enabled_text_fields_omit_false_native_flags() {
    let html = render(|| {
        view! { <TextField value="text" on_change=|_| {} test_id="text" /> }
    });
    let tag = opening_tag(&html, "text");
    assert!(
        !has_boolean_attribute(tag, "disabled") && !has_boolean_attribute(tag, "readonly"),
        "{tag}"
    );
}

#[test]
fn text_area_serializes_escaped_multiline_contents_instead_of_a_value_attribute() {
    let html = render(|| {
        view! {
            <TextArea
                value="一<&>\n\r</textarea><script>bad</script>"
                on_change=|_| {}
                test_id="area"
            />
        }
    });
    let tag = opening_tag(&html, "area");
    let start = html.find(tag).expect("textarea opening tag") + tag.len();
    let end = start + html[start..].find("</textarea>").expect("textarea end");
    assert_eq!(
        (&html[start..end], tag.contains(" value=")),
        (
            "一&lt;&amp;&gt;\n&#13;&lt;/textarea&gt;&lt;script&gt;bad&lt;/script&gt;",
            false
        ),
        "{html}"
    );
}

#[test]
fn text_area_preserves_a_leading_line_feed_through_document_parsing() {
    let html = render(|| {
        view! { <TextArea value="\nFirst\nSecond" on_change=|_| {} test_id="area" /> }
    });
    let tag = opening_tag(&html, "area");
    let start = html.find(tag).expect("textarea opening tag") + tag.len();
    assert!(
        html[start..].starts_with("\n\nFirst\nSecond</textarea>"),
        "{html}"
    );
}

#[test]
fn text_area_serializes_disabled_and_read_only_flags() {
    let html = render(|| {
        view! {
            <TextArea value="notes" on_change=|_| {} disabled=true read_only=true test_id="area" />
        }
    });
    let tag = opening_tag(&html, "area");
    assert!(
        has_boolean_attribute(tag, "disabled") && has_boolean_attribute(tag, "readonly"),
        "{tag}"
    );
}

#[test]
fn checked_disabled_checkboxes_emit_the_native_state() {
    let html = render(|| {
        view! { <Checkbox checked=true on_change=|_| {} disabled=true test_id="check" /> }
    });
    let tag = opening_tag(&html, "check");
    assert!(
        has_boolean_attribute(tag, "checked") && has_boolean_attribute(tag, "disabled"),
        "{tag}"
    );
}

#[test]
fn unchecked_enabled_checkboxes_omit_false_native_state() {
    let html = render(|| {
        view! { <Checkbox checked=false on_change=|_| {} test_id="check" /> }
    });
    let tag = opening_tag(&html, "check");
    assert!(
        !has_boolean_attribute(tag, "checked") && !has_boolean_attribute(tag, "disabled"),
        "{tag}"
    );
}

#[test]
fn radio_options_emit_selection_and_each_options_native_availability() {
    let html = render(|| {
        view! {
            <RadioGroup
                value="second"
                options=vec![
                    RadioOption::new("first", "First").disabled(),
                    RadioOption::new("second", "Second"),
                ]
                on_change=|_| {}
                test_id="radio"
            />
        }
    });
    let first = opening_tag(&html, "radio-first");
    let second = opening_tag(&html, "radio-second");
    assert!(
        !has_boolean_attribute(first, "checked")
            && has_boolean_attribute(first, "disabled")
            && has_boolean_attribute(second, "checked")
            && !has_boolean_attribute(second, "disabled"),
        "{first}\n{second}"
    );
}

#[test]
fn a_disabled_radio_group_emits_disabled_for_every_option() {
    let html = render(|| {
        view! {
            <RadioGroup
                value="first"
                options=vec![RadioOption::new("first", "First"), RadioOption::new("second", "Second")]
                on_change=|_| {}
                disabled=true
                test_id="radio"
            />
        }
    });
    assert!(
        ["radio-first", "radio-second"]
            .into_iter()
            .all(|id| has_boolean_attribute(opening_tag(&html, id), "disabled")),
        "{html}"
    );
}

#[test]
fn sliders_emit_the_snapped_value_and_native_availability() {
    let html = render(|| {
        view! {
            <Slider value=7.4 on_change=|_| {} min=0.0 max=10.0 step=2.0 disabled=true aria_label="Amount" test_id="range" />
        }
    });
    let tag = opening_tag(&html, "range");
    assert!(
        tag.contains("aria-valuenow=\"8\"")
            && tag.contains("role=\"slider\"")
            && has_boolean_attribute(tag, "disabled"),
        "{tag}"
    );
}

#[test]
fn disabled_switches_emit_native_availability() {
    let html = render(|| {
        view! { <Switch checked=true on_change=|_| {} disabled=true test_id="switch" /> }
    });
    let tag = opening_tag(&html, "switch");
    assert!(has_boolean_attribute(tag, "disabled"), "{tag}");
}

#[test]
fn unavailable_tabs_emit_native_disabled_state() {
    let html = render(|| {
        view! {
            <Tabs
                active="first"
                tabs=vec![Tab::new("first", "First"), Tab::new("second", "Second").disabled()]
                on_activate=|_| {}
            >
                <span>"Tab contents"</span>
            </Tabs>
        }
    });
    let tag = opening_tag(&html, "tab-second");
    assert!(has_boolean_attribute(tag, "disabled"), "{tag}");
}

#[test]
fn toggle_groups_combine_group_and_item_availability_in_native_buttons() {
    let html = render(|| {
        view! {
            <ToggleGroup
                value=vec!["first".to_string()]
                items=vec![ToggleItem::new("first", "First"), ToggleItem::new("second", "Second").disabled()]
                on_change=|_| {}
                disabled=true
                test_id="toggle"
            />
        }
    });
    assert!(
        ["toggle-first", "toggle-second"]
            .into_iter()
            .all(|id| has_boolean_attribute(opening_tag(&html, id), "disabled")),
        "{html}"
    );
}
