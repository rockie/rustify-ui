#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
//! Form semantics rendered through public component and primitive entry points.

use leptos::prelude::*;
use rustify_components::{primitives as p, Checkbox, RadioGroup, RadioOption, Switch, Toggle};

fn render(view: impl FnOnce() -> AnyView) -> String {
    Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "forms".into(),
                ..Default::default()
            },
            view,
        )
        .to_html()
    })
}
fn tag<'a>(html: &'a str, test_id: &str) -> &'a str {
    let at = html
        .find(&format!("data-testid=\"{test_id}\""))
        .expect("test control");
    let start = html[..at].rfind('<').unwrap();
    &html[start..=at + html[at..].find('>').unwrap()]
}
fn flag(tag: &str, name: &str) -> bool {
    tag.contains(&format!(" {name} ")) || tag.ends_with(&format!(" {name}>"))
}

#[test]
fn choice_mixed_checkbox_exposes_aria_and_accepted_form_state() {
    let html = render(|| {
        view! {
        <Checkbox checked=false indeterminate=true name="consent" value="yes" required=true on_change=|_| {} test_id="mixed" />
    }.into_any()
    });
    let input = tag(&html, "mixed");
    assert!(
        input.starts_with("<input") && input.contains("aria-checked=\"mixed\""),
        "{input}"
    );
    assert!(
        input.contains("name=\"consent\"") && input.contains("value=\"yes\""),
        "{input}"
    );
    assert!(
        flag(input, "required") && !flag(input, "checked"),
        "{input}"
    );
}
#[test]
fn choice_raw_parts_do_not_reference_the_skin_or_generate_names() {
    let html = render(|| {
        view! {
        <p::checkbox::Root checked=p::CheckedState::Checked on_change=|_| {}>
            <p::checkbox::Control test_id="raw" /><p::checkbox::Indicator>"Marker"</p::checkbox::Indicator>
        </p::checkbox::Root>
    }.into_any()
    });
    assert!(flag(tag(&html, "raw"), "checked"), "{html}");
    assert!(!tag(&html, "raw").contains(" name="), "{html}");
    assert!(
        !html.contains("rustify-choice-native") && !html.contains("--primary"),
        "{html}"
    );
}
#[test]
fn choice_anonymous_groups_in_one_form_have_no_native_names_and_independent_selection() {
    let html = render(|| {
        view! {
        <form>
            <RadioGroup value="b" options=vec![RadioOption::new("a", "A"), RadioOption::new("b", "B")] on_change=|_| {} test_id="one" />
            <RadioGroup value="a" options=vec![RadioOption::new("a", "A"), RadioOption::new("b", "B")] on_change=|_| {} test_id="two" />
        </form>
    }.into_any()
    });
    for test_id in ["one-a", "one-b", "two-a", "two-b"] {
        assert!(!tag(&html, test_id).contains(" name="), "{html}");
    }
    assert!(
        flag(tag(&html, "one-b"), "checked") && flag(tag(&html, "two-a"), "checked"),
        "{html}"
    );
    assert_eq!(html.matches("tabindex=\"0\"").count(), 2, "{html}");
}
#[test]
fn choice_named_radios_preserve_explicit_form_owner_and_name() {
    let html = render(|| {
        view! {
        <form id="one"></form><form id="two"></form>
        <RadioGroup value="a" options=vec![RadioOption::new("a", "A")] name="choice" form="one" on_change=|_| {} test_id="one" />
        <RadioGroup value="a" options=vec![RadioOption::new("a", "A")] name="choice" form="two" on_change=|_| {} test_id="two" />
    }.into_any()
    });
    for (test_id, form) in [("one-a", "one"), ("two-a", "two")] {
        let input = tag(&html, test_id);
        assert!(
            input.contains("name=\"choice\"")
                && input.contains(&format!("form=\"{form}\""))
                && flag(input, "checked"),
            "{input}"
        );
    }
}
#[test]
fn choice_group_required_uses_one_proxy_and_accepts_any_enabled_selection() {
    let html = render(|| {
        view! {
        <p::checkbox_group::Root value=vec!["b".to_string()] on_change=|_| {} required=true>
            <p::checkbox_group::Item value="a" test_id="a">"A"</p::checkbox_group::Item>
            <p::checkbox_group::Item value="b" test_id="b">"B"</p::checkbox_group::Item>
        </p::checkbox_group::Root>
        <p::radio_group::Root value=Signal::stored(None::<String>) on_change=|_| {} required=true>
            <p::radio_group::Item value="x" disabled=true test_id="off">"Off"</p::radio_group::Item>
            <p::radio_group::Item value="y" test_id="first">"First"</p::radio_group::Item>
            <p::radio_group::Item value="z" test_id="last">"Last"</p::radio_group::Item>
        </p::radio_group::Root>
    }.into_any()
    });
    assert!(
        !flag(tag(&html, "a"), "required") && !flag(tag(&html, "b"), "required"),
        "{html}"
    );
    assert!(flag(tag(&html, "first"), "required"), "{html}");
    assert!(
        !flag(tag(&html, "off"), "required") && !flag(tag(&html, "last"), "required"),
        "{html}"
    );
}
#[test]
fn choice_switch_has_one_native_successful_control_and_toggle_only_accepted_hidden_value() {
    let html = render(|| {
        view! {
        <Switch checked=true name="notifications" value="yes" on_change=|_| {} test_id="switch" />
        <Toggle pressed=true name="format" value="bold" on_change=|_| {}>"Bold"</Toggle>
        <Toggle pressed=false name="format" value="italic" on_change=|_| {}>"Italic"</Toggle>
    }.into_any()
    });
    let switch = tag(&html, "switch");
    assert!(
        switch.starts_with("<input")
            && switch.contains("type=\"checkbox\"")
            && switch.contains("role=\"switch\""),
        "{html}"
    );
    assert_eq!(html.matches("name=\"notifications\"").count(), 1, "{html}");
    assert_eq!(html.matches("name=\"format\"").count(), 1, "{html}");
    assert!(
        html.contains("value=\"bold\"") && !html.contains("value=\"italic\""),
        "{html}"
    );
}
#[test]
fn choice_cards_have_native_controls_and_real_label_and_description_references() {
    let html = render(|| {
        view! {
        <rustify_components::radio_cards::Root value=Some("a".to_string()) on_change=|_| {} name="plan">
            <rustify_components::radio_cards::Item value="a" test_id="card"
                description=Box::new(|| view! { <span>"Includes support"</span> }.into_any())>"Standard"</rustify_components::radio_cards::Item>
        </rustify_components::radio_cards::Root>
    }.into_any()
    });
    let input = tag(&html, "card");
    assert!(
        flag(input, "checked") && input.contains("name=\"plan\""),
        "{html}"
    );
    for attribute in ["aria-labelledby", "aria-describedby"] {
        let start = input.find(&format!("{attribute}=\"")).unwrap() + attribute.len() + 2;
        let id = &input[start..start + input[start..].find('"').unwrap()];
        assert!(html.contains(&format!("id=\"{id}\"")), "{html}");
    }
}
#[test]
fn choice_group_identity_is_seeded_independent_of_business_values() {
    let view = || {
        view! {
        <RadioGroup value="a" options=vec![RadioOption::new("a", "A")] on_change=|_| {} test_id="one" />
        <RadioGroup value="a" options=vec![RadioOption::new("a", "A")] on_change=|_| {} test_id="two" />
    }.into_any()
    };
    let first = render(view);
    let second = render(view);
    assert_eq!(first, second);
    let owner = |input: &str| {
        let key = "data-rustify-group-id=\"";
        let start = input.find(key).unwrap() + key.len();
        input[start..start + input[start..].find('"').unwrap()].to_string()
    };
    assert_ne!(owner(tag(&first, "one-a")), owner(tag(&first, "two-a")));
}

#[test]
fn choice_anonymous_cards_keep_both_selections_with_and_without_a_form() {
    for in_form in [false, true] {
        let html = render(|| {
            let cards = view! {
                <rustify_components::radio_cards::Root value=Some("same".to_string()) on_change=|_| {}>
                    <rustify_components::radio_cards::Item value="same" test_id="card-one">"One"</rustify_components::radio_cards::Item>
                </rustify_components::radio_cards::Root>
                <rustify_components::radio_cards::Root value=Some("same".to_string()) on_change=|_| {}>
                    <rustify_components::radio_cards::Item value="same" test_id="card-two">"Two"</rustify_components::radio_cards::Item>
                </rustify_components::radio_cards::Root>
            };
            if in_form {
                view! { <form>{cards}</form> }.into_any()
            } else {
                cards.into_any()
            }
        });
        for id in ["card-one", "card-two"] {
            let input = tag(&html, id);
            assert!(
                flag(input, "checked") && !input.contains(" name="),
                "{html}"
            );
        }
    }
}

#[test]
fn choice_visual_matrix_has_external_responsive_geometry_and_no_inline_style() {
    use rustify_components::checkbox::{CheckboxSize, CheckboxVariant};
    for &size in CheckboxSize::ALL {
        for &variant in CheckboxVariant::ALL {
            let styles = rustify_components::GeometryStyles::default();
            let html = render(|| {
                rustify_components::provide_geometry_styles(styles.clone());
                view! { <Checkbox checked=true on_change=|_| {} size=size variant=variant
                color=Some(rustify_ui::AccentColor::Indigo) high_contrast=true /> }
                .into_any()
            });
            assert!(
                html.contains(&format!("data-variant=\"{}\"", variant.as_str())),
                "{html}"
            );
            assert!(html.contains("data-high-contrast=\"\""), "{html}");
            assert!(!html.contains(" style="), "{html}");
            assert!(
                styles.css().contains("--rustify-choice-size:"),
                "{}",
                styles.css()
            );
        }
    }
}

#[test]
fn choice_single_toggle_mode_never_displays_or_submits_multiple_values() {
    let html = render(|| {
        view! {
            <p::toggle_group::Root value=vec!["a".to_string(), "b".to_string()] on_change=|_| {}
                mode=p::SelectionMode::Single name="mode">
                <p::toggle_group::Item value="a" test_id="single-a">"A"</p::toggle_group::Item>
                <p::toggle_group::Item value="b" test_id="single-b">"B"</p::toggle_group::Item>
            </p::toggle_group::Root>
        }
        .into_any()
    });
    assert!(
        html.contains("data-rustify-issue=\"invalid-selection\""),
        "{html}"
    );
    assert_eq!(html.matches("aria-pressed=\"true\"").count(), 1, "{html}");
    assert!(!html.contains("name=\"mode\""), "{html}");
}

#[test]
fn choice_custom_group_target_preserves_host_class_and_exposes_native_item_attributes() {
    let html = render(|| {
        view! {
        <p::radio_group::Root value=Some("a".to_string()) on_change=|_| {}>
            {p::radio_group::item_as(leptos::html::input().attr("data-testid", "custom-item").class("host-class"),
                p::ItemAttributes { id: "custom-radio".into(), aria_label: "Custom choice".into(), disabled: Signal::stored(true), ..p::ItemAttributes::new("a") })}
        </p::radio_group::Root>
    }.into_any()
    });
    let input = tag(&html, "custom-item");
    assert!(
        input.contains("id=\"custom-radio\"") && input.contains("aria-label=\"Custom choice\""),
        "{input}"
    );
    assert!(
        input.contains("host-class") && flag(input, "disabled"),
        "{input}"
    );
}

#[test]
fn choice_themed_custom_control_retains_target_class_and_adds_native_skin() {
    let html = render(|| {
        view! {
        <rustify_components::checkbox::Root checked=p::CheckedState::Checked on_change=|_| {}>
            {rustify_components::checkbox::control_as(leptos::html::input().class("host-class").attr("data-testid", "custom-themed"))}
        </rustify_components::checkbox::Root>
    }.into_any()
    });
    let input = tag(&html, "custom-themed");
    assert!(
        input.contains("host-class") && input.contains("rustify-choice-native"),
        "{input}"
    );
    assert!(flag(input, "checked"), "{input}");
}

#[test]
fn choice_card_without_label_slot_uses_the_supplied_accessible_name() {
    let html = render(|| {
        view! {
        <rustify_components::radio_cards::Root value=Some("a".to_string()) on_change=|_| {}>
            <rustify_components::radio_cards::Item value="a" aria_label="Accessible card" test_id="named-card" />
        </rustify_components::radio_cards::Root>
    }.into_any()
    });
    let input = tag(&html, "named-card");
    assert!(
        input.contains("aria-label=\"Accessible card\"") && !input.contains("aria-labelledby="),
        "{input}"
    );
}

#[test]
fn slider_native_form_values_follow_normalized_numeric_order_and_thumb_order_is_stable() {
    let html = render(|| {
        view! {
        <p::slider::Root values=vec![p::slider::ThumbValue { id: "high".into(), value: 90.0 }, p::slider::ThumbValue { id: "low".into(), value: -10.0 }]
            on_change=|_| {} name="range" form="external" read_only=true>
            <p::slider::Track><p::slider::Range />
                <p::slider::Thumb id="high" aria_label="Upper" test_id="high" />
                <p::slider::Thumb id="low" aria_label="Lower" test_id="low" />
            </p::slider::Track>
        </p::slider::Root>
    }.into_any()
    });
    let high = tag(&html, "high");
    let low = tag(&html, "low");
    assert!(
        high.contains("role=\"slider\"") && high.contains("aria-valuenow=\"90\""),
        "{html}"
    );
    assert!(
        low.contains("aria-valuenow=\"0\"") && low.contains("aria-readonly=\"true\""),
        "{html}"
    );
    assert!(
        html.find("data-testid=\"high\"").unwrap() < html.find("data-testid=\"low\"").unwrap(),
        "{html}"
    );
    let hidden: Vec<_> = html
        .split('<')
        .filter(|tag| tag.starts_with("input "))
        .collect();
    assert_eq!(hidden.len(), 2, "{html}");
    assert!(
        hidden[0].contains("value=\"0\"") && hidden[1].contains("value=\"90\""),
        "{html}"
    );
    assert!(
        hidden
            .iter()
            .all(|input| input.contains("name=\"range\"") && input.contains("form=\"external\"")),
        "{html}"
    );
    assert!(!html.contains(" style="), "{html}");
}

#[test]
fn slider_invalid_initial_props_are_readable_and_do_not_submit() {
    let html = render(|| {
        view! {
        <p::slider::Root values=vec![p::slider::ThumbValue { id: "a".into(), value: 50.0 }]
            on_change=|_| {} step=0.0 name="bad">
            <p::slider::Track><p::slider::Thumb id="a" aria_label="Amount" test_id="invalid-slider" /></p::slider::Track>
        </p::slider::Root>
    }.into_any()
    });
    assert!(
        html.contains("invalid-config") && html.contains("Invalid slider configuration"),
        "{html}"
    );
    assert!(flag(tag(&html, "invalid-slider"), "disabled"), "{html}");
    assert!(!html.contains("type=\"hidden\""), "{html}");
}

#[test]
fn slider_custom_thumb_keeps_native_target_attributes_and_additive_geometry() {
    let styles = rustify_components::GeometryStyles::default();
    let html = render(|| {
        rustify_components::provide_geometry_styles(styles.clone());
        view! {
            <p::slider::Root values=vec![p::slider::ThumbValue { id: "a".into(), value: 40.0 }] on_change=|_| {}>
                <p::slider::Track>
                    {p::slider::thumb_as(leptos::html::button().class("host-thumb").attr("data-host", "kept"), p::slider::ThumbAttributes {
                        id: "a".into(), control_id: "native-thumb".into(), aria_label: "Amount".into(), labelled_by: String::new(),
                        value_text: Signal::stored("Forty".into()), disabled: Signal::stored(false), test_id: "custom-thumb".into(),
                    })}
                </p::slider::Track>
            </p::slider::Root>
        }.into_any()
    });
    let thumb = tag(&html, "custom-thumb");
    assert!(
        thumb.contains("host-thumb")
            && thumb.contains("data-host=\"kept\"")
            && thumb.contains("aria-valuetext=\"Forty\""),
        "{thumb}"
    );
    assert!(
        thumb.contains("id=\"native-thumb\"") && !thumb.contains(" style="),
        "{thumb}"
    );
    assert!(styles.css().contains("left:40%;"), "{}", styles.css());
}

#[test]
fn slider_themed_scalar_and_range_share_parts_form_semantics_and_external_visual_rules() {
    use rustify_components::{
        RangeSlider, Slider, SliderSize, SliderVariant, ThumbLabel, ThumbValue,
    };
    for &size in SliderSize::ALL {
        for &variant in SliderVariant::ALL {
            let styles = rustify_components::GeometryStyles::default();
            let html = render(|| {
                rustify_components::provide_geometry_styles(styles.clone());
                view! {
                    <Slider value=7.4 on_change=|_| {} min=0.0 max=10.0 step=2.0 aria_label="Amount" id="amount"
                        name="amount" read_only=true size=size variant=variant test_id="scalar" />
                    <RangeSlider values=vec![ThumbValue { id: "high".into(), value: 80.0 }, ThumbValue { id: "low".into(), value: 20.0 }]
                        thumbs=vec![ThumbLabel { id: "high".into(), label: "Upper".into() }, ThumbLabel { id: "low".into(), label: "Lower".into() }]
                        on_change=|_| {} name="range" size=size variant=variant test_id="themed-range" />
                }.into_any()
            });
            let scalar = tag(&html, "scalar");
            assert!(
                scalar.contains("id=\"amount\"")
                    && scalar.contains("aria-valuenow=\"8\"")
                    && scalar.contains("aria-readonly=\"true\""),
                "{html}"
            );
            assert!(
                !html.contains("invalid-thumb") && !html.contains(" style="),
                "{html}"
            );
            assert_eq!(html.matches("role=\"slider\"").count(), 3, "{html}");
            assert_eq!(html.matches("type=\"hidden\"").count(), 3, "{html}");
            assert!(
                html.contains(&format!("data-variant=\"{}\"", variant.as_str())),
                "{html}"
            );
            assert!(
                styles.css().contains("--rustify-slider-track:"),
                "{}",
                styles.css()
            );
        }
    }
}

#[test]
fn slider_unnamed_thumb_is_diagnosed_disabled_and_excluded_from_submission() {
    let html = render(|| {
        view! {
        <p::slider::Root values=vec![p::slider::ThumbValue { id: "a".into(), value: 30.0 }] on_change=|_| {} name="amount">
            <p::slider::Track><p::slider::Thumb id="a" test_id="unnamed-thumb" /></p::slider::Track>
        </p::slider::Root>
    }.into_any()
    });
    let thumb = tag(&html, "unnamed-thumb");
    assert!(
        thumb.contains("missing-name") && flag(thumb, "disabled"),
        "{html}"
    );
    assert!(!html.contains("type=\"hidden\""), "{html}");
}

#[test]
fn field_description_and_visible_error_reference_the_native_control_and_source() {
    use rustify_components::{Field, Form, TextField};
    let html = render(|| {
        let form = Form::new(&["email"], || {});
        form.set_application_error("email", Some("Address unavailable".into()));
        view! { <Field form=form field="email" label="Email" description="Use a work address"
            control=move |binding| view! { <TextField value="" on_change=|_| {} id=binding.id()
                invalid=binding.invalid() described_by=binding.described_by() test_id="field-input" /> }.into_any() /> }.into_any()
    });
    let input = tag(&html, "field-input");
    let described = input
        .split("aria-describedby=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    let refs: Vec<_> = described.split_whitespace().collect();
    assert_eq!(refs.len(), 2, "{html}");
    for id in refs {
        assert!(html.contains(&format!("id=\"{id}\"")), "{html}");
    }
    assert!(input.contains("aria-invalid=\"true\""), "{input}");
    assert!(
        html.contains("Use a work address") && html.contains("Address unavailable"),
        "{html}"
    );
    assert!(
        tag(&html, "error-email").contains("data-source=\"application\""),
        "{html}"
    );
}

#[test]
fn field_form_root_has_one_native_form_and_preserves_host_attributes() {
    let html = render(|| {
        let form = rustify_components::Form::new(&["name"], || {});
        view! { <p::form::Root form=form rules=Vec::new id="native-form">
            <input name="plain" value="accepted" />
        </p::form::Root> }
        .add_any_attr((leptos::attr::action("/save"), leptos::attr::method("post")))
        .into_any()
    });
    assert_eq!(html.matches("<form ").count(), 1, "{html}");
    assert!(
        html.contains("id=\"native-form\"")
            && html.contains("action=\"/save\"")
            && html.contains("method=\"post\""),
        "{html}"
    );
    assert!(
        html.contains(" novalidate") && html.contains("name=\"plain\""),
        "{html}"
    );
}

#[test]
fn field_nested_form_root_is_diagnosed_without_a_second_form_or_lost_children() {
    let html = render(|| {
        let outer = rustify_components::Form::new(&[], || {});
        let inner = rustify_components::Form::new(&[], || {});
        view! { <p::form::Root form=outer rules=Vec::new>
            <p::form::Root form=inner rules=Vec::new><span>"Nested content"</span></p::form::Root>
        </p::form::Root> }
        .into_any()
    });
    assert_eq!(html.matches("<form ").count(), 1, "{html}");
    assert!(
        html.contains("nested-form") && html.contains("Nested content"),
        "{html}"
    );
}

#[test]
fn field_parts_keep_native_attributes_and_separate_wrapper_from_editor() {
    let html = render(|| {
        view! {
        <p::text_field::Root value="accepted" on_change=|_| {} id="native-text" name="title" form="external"
            required=true placeholder="Title" class="wrapper" input_class="editor">
            <p::text_field::Slot side=p::field::SlotSide::Start><span>"Prefix"</span></p::text_field::Slot>
            {p::text_field::control_as(leptos::html::input().attr("pattern", "[a-z]+")
                .attr("autocomplete", "nickname").attr("data-testid", "native-text"))}
        </p::text_field::Root>
        <p::text_area::Root value="\n< & >\r\n" on_change=|_| {} name="notes" rows=4 test_id="native-area" />
    }.into_any()
    });
    let input = tag(&html, "native-text");
    for attr in [
        "name=\"title\"",
        "form=\"external\"",
        "value=\"accepted\"",
        "pattern=\"[a-z]+\"",
        "autocomplete=\"nickname\"",
        "class=\"editor\"",
    ] {
        assert!(input.contains(attr), "missing {attr}: {html}");
    }
    assert!(flag(input, "required"), "{input}");
    assert!(!input.contains("wrapper"), "{input}");
    assert!(
        html.contains("data-side=\"start\"") && html.contains("Prefix"),
        "{html}"
    );
    assert!(
        tag(&html, "native-area").contains("name=\"notes\""),
        "{html}"
    );
    assert!(html.contains("\n\n&lt; &amp; &gt;&#13;\n"), "{html}");
    assert!(!html.contains("rustify-field-skin"), "{html}");
}

#[test]
fn field_drafts_submit_only_the_accepted_mirror_and_keep_constraints_on_the_editor() {
    let html = render(|| {
        view! {
        <p::number_field::Root value=12.5 on_change=|_| {} name="amount" required=true min=0.0 max=20.0>
            <p::number_field::Input test_id="draft-number" />
        </p::number_field::Root>
        <p::color_field::Root value="#112233" on_change=|_| {} name="color" required=true>
            <p::color_field::Swatch test_id="accepted-swatch" /><p::color_field::Input test_id="draft-color" />
        </p::color_field::Root>
    }.into_any()
    });
    for test_id in ["draft-number", "draft-color"] {
        let input = tag(&html, test_id);
        assert!(
            flag(input, "required") && !input.contains(" name="),
            "{input}"
        );
    }
    assert_eq!(html.matches("name=\"amount\"").count(), 1, "{html}");
    assert_eq!(html.matches("name=\"color\"").count(), 1, "{html}");
    assert!(
        html.contains("type=\"hidden\" name=\"amount\"") && html.contains("value=\"12.5\""),
        "{html}"
    );
    let number = tag(&html, "draft-number");
    assert!(
        number.contains("aria-valuemin=\"0\"") && number.contains("aria-valuemax=\"20\""),
        "{number}"
    );
    assert!(
        !number.contains(" min=") && !number.contains(" max="),
        "{number}"
    );
    assert!(
        tag(&html, "accepted-swatch").contains("aria-hidden=\"true\""),
        "{html}"
    );
}

#[test]
fn field_themes_share_native_parts_and_slot_overrides_emit_external_responsive_rules() {
    use rustify_components::{text_field as t, Responsive, Spacing, TextArea, TextField};
    use rustify_ui::{AccentColor, ThemeRadius};
    let styles = rustify_components::GeometryStyles::default();
    let html = render(|| {
        rustify_components::provide_geometry_styles(styles.clone());
        view! {
            <TextField value="accepted" on_change=|_| {} name="query" test_id="themed-field"
                size=t::TextFieldSize::Three variant=t::TextFieldVariant::Soft color=Some(AccentColor::Blue) radius=Some(ThemeRadius::Full)
                class="w-64" input_class="font-bold">
                <t::Slot side=t::SlotSide::Start color=Some(AccentColor::Red)
                    gap=Signal::stored(Responsive { initial: Some(Spacing::Scale(1)), md: Some(Spacing::Scale(4)), ..Default::default() })
                    px=Signal::stored(Responsive::from(Spacing::Scale(2)))
                    pl=Signal::stored(Responsive::from(Spacing::Scale(3)))><span>"Icon"</span><span>"Prefix"</span></t::Slot>
            </TextField>
            <TextArea value="notes" on_change=|_| {} name="notes" test_id="themed-area"
                resize=rustify_components::textarea::TextAreaResize::Both />
        }.into_any()
    });
    let input = tag(&html, "themed-field");
    assert!(
        input.contains("rustify-field-input")
            && input.contains("font-bold")
            && input.contains("name=\"query\""),
        "{html}"
    );
    assert!(!input.contains("w-64"), "{html}");
    assert!(
        html.contains("rustify-field-slot") && html.contains("data-color=\"red\""),
        "{html}"
    );
    assert!(
        html.contains("data-side=\"start\"") && html.contains("data-variant=\"soft\""),
        "{html}"
    );
    assert!(!html.contains("style="), "{html}");
    let css = styles.css();
    for declaration in [
        "--rustify-field-height:var(--rustify-space-7, 40px)",
        "--rustify-field-radius:9999px",
        "padding-left:var(--rustify-space-3)",
        "padding-right:var(--rustify-space-2)",
        "gap:var(--rustify-space-1)",
        "@media (min-width:1024px)",
        "gap:var(--rustify-space-4)",
        "resize:both",
    ] {
        assert!(css.contains(declaration), "missing {declaration}: {css}");
    }
}

#[test]
fn field_form_submit_button_is_native_inside_root_and_legacy_outside() {
    use rustify_components::{Form, FormRoot, SubmitButton};
    let html = render(|| {
        let form = Form::new(&[], || {});
        view! { <FormRoot form=form rules=Vec::new>
            <SubmitButton form=form rules=Vec::new test_id="native-submit">"Save"</SubmitButton>
        </FormRoot>
        <SubmitButton form=form rules=Vec::new test_id="legacy-submit">"Save"</SubmitButton> }
        .into_any()
    });
    assert!(
        tag(&html, "native-submit").contains("type=\"submit\""),
        "{html}"
    );
    assert!(
        tag(&html, "legacy-submit").contains("type=\"button\""),
        "{html}"
    );
}

#[test]
fn field_composed_parts_name_the_actual_editor_and_only_reference_present_messages() {
    let html = render(|| {
        let form = rustify_components::Form::new(&["raw"], || {});
        form.set_application_error("raw", Some("Keep this error".into()));
        view! { <p::form::Root form=form rules=Vec::new id="composed-form">
            <p::form::Field form=form field="raw">
                <section><p::form::Label>"Raw label"</p::form::Label><p::form::Description>"Raw hint"</p::form::Description></section>
                <p::text_field::Root value="accepted" on_change=|_| {} name="raw"><p::text_field::Input test_id="composed-input" /></p::text_field::Root>
                <aside><p::form::Message test_id="composed-message" /></aside>
            </p::form::Field>
        </p::form::Root> }.into_any()
    });
    let input = tag(&html, "composed-input");
    let id = input
        .split(" id=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert!(html.contains(&format!("for=\"{id}\"")), "{html}");
    assert!(
        input.contains("form=\"composed-form\"") && input.contains("aria-invalid=\"true\""),
        "{html}"
    );
    let described = input
        .split("aria-describedby=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert_eq!(described.split_whitespace().count(), 2, "{html}");
    for id in described.split_whitespace() {
        assert!(html.contains(&format!("id=\"{id}\"")), "{html}");
    }
    assert!(
        tag(&html, "composed-message").contains("data-source=\"application\""),
        "{html}"
    );
    assert!(
        !html.contains("text-sm") && !html.contains("rustify-field-input"),
        "{html}"
    );
}

#[test]
fn select_closed_parts_render_metadata_label_and_only_the_accepted_native_value() {
    let html = render(|| {
        view! {
        <p::select::Root value=Signal::stored(Some("b".to_string())) on_change=|_| {} open=false on_open_change=|_| {}
            entries=vec![p::select::SelectEntry::new("a", "Apricot"), p::select::SelectEntry::new("b", "Banana")]
            name="fruit" form="external" required=true>
            <p::select::Trigger test_id="select-trigger"><p::select::Value placeholder="Choose fruit" /><p::select::Icon /></p::select::Trigger>
            <p::select::Content><p::select::Viewport><p::select::Item value="a">"Rich Apricot"</p::select::Item>
                <p::select::Item value="b">"Rich Banana"</p::select::Item></p::select::Viewport></p::select::Content>
        </p::select::Root>
    }.into_any()
    });
    let trigger = tag(&html, "select-trigger");
    assert!(
        trigger.contains("role=\"combobox\"") && trigger.contains("aria-expanded=\"false\""),
        "{html}"
    );
    assert!(
        !trigger.contains("aria-activedescendant") && !trigger.contains("aria-controls"),
        "{html}"
    );
    assert!(
        html.contains("Banana") && !html.contains("Rich Banana"),
        "{html}"
    );
    assert_eq!(html.matches("name=\"fruit\"").count(), 1, "{html}");
    assert!(
        html.contains("<select")
            && html.contains("form=\"external\"")
            && html.contains("value=\"b\" selected"),
        "{html}"
    );
    assert!(
        !html.contains("rustify-select-trigger") && !html.contains("style="),
        "{html}"
    );
}

#[test]
fn select_invalid_metadata_and_unknown_value_keep_a_placeholder_and_no_fake_selection() {
    let html = render(|| {
        view! {
        <p::select::Root value=Signal::stored(Some("a".to_string())) on_change=|_| {} open=false on_open_change=|_| {}
            entries=vec![p::select::SelectEntry::new("a", "One"), p::select::SelectEntry::new("a", "Two")] name="choice">
            <p::select::Trigger test_id="invalid-select"><p::select::Value placeholder="Choose" /></p::select::Trigger>
        </p::select::Root>
    }.into_any()
    });
    assert!(
        html.contains("duplicate-entry") && html.contains("Choose"),
        "{html}"
    );
    assert!(
        !html.contains("aria-activedescendant") && !html.contains("aria-selected=\"true\""),
        "{html}"
    );
}

#[test]
fn select_initial_open_reports_static_issue_without_rendering_a_phantom_listbox() {
    Owner::new().with(|| {
        let issues = rustify_ui::StaticRenderIssues::default();
        rustify_ui::provide_static_render_issues(issues.clone());
        let html = view! {
            <p::select::Root value=Signal::stored(Some("a".to_string())) on_change=|_| {} open=true on_open_change=|_| {}
                entries=vec![p::select::SelectEntry::new("a", "Apricot")]>
                <p::select::Trigger test_id="initial-open"><p::select::Value /></p::select::Trigger>
                <p::select::Content><p::select::Item value="a" /></p::select::Content>
            </p::select::Root>
        }.to_html();
        assert_eq!(issues.messages().len(), 1);
        assert!(tag(&html, "initial-open").contains("aria-expanded=\"false\""), "{html}");
        assert!(!html.contains("role=\"listbox\"") && !html.contains("aria-controls"), "{html}");
    });
}

#[test]
fn select_themes_expose_every_trigger_variant_and_size_without_parallel_behavior() {
    use rustify_components::select as s;
    let html = render(|| {
        view! {
        {s::SelectSize::ALL.iter().flat_map(|size| s::SelectTriggerVariant::ALL.iter().map(move |variant| (*size, *variant))).map(|(size, variant)| view! {
            <s::Root value=Signal::stored(None::<String>) on_change=|_| {} open=false on_open_change=|_| {} entries=Vec::new size=size>
                <s::Trigger variant=variant test_id=format!("theme-{}-{}", size.number(), variant.as_str()) />
            </s::Root>
        }).collect_view()}
    }.into_any()
    });
    for size in s::SelectSize::ALL {
        for variant in s::SelectTriggerVariant::ALL {
            let trigger = tag(
                &html,
                &format!("theme-{}-{}", size.number(), variant.as_str()),
            );
            assert!(
                trigger.contains("rustify-select-trigger") && trigger.contains("role=\"combobox\""),
                "{trigger}"
            );
            assert!(
                trigger.contains(&format!("data-variant=\"{}\"", variant.as_str())),
                "{trigger}"
            );
        }
    }
    assert_eq!(s::SelectContentVariant::ALL.len(), 2);
}

#[test]
fn field_explicit_id_is_the_forms_current_focus_target() {
    render(|| {
        let form = rustify_components::Form::new(&["title"], || {});
        let view = view! {
            <p::form::Field form=form field="title"><p::form::Label>"Title"</p::form::Label>
                <p::text_field::Root id="explicit-title" value="accepted" on_change=|_| {}>
                    <p::text_field::Input />
                </p::text_field::Root>
            </p::form::Field>
        };
        assert_eq!(form.control_id("title"), "explicit-title");
        view.into_any()
    });
}
