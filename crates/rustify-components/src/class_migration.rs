use crate::button::ButtonClass;
use crate::cn::IntoTailwindClass;
use crate::{ButtonSize, ButtonVariant};
use serde_json::Value;

#[test]
#[cfg(rustify_repo_tests)]
fn captured_component_inputs_match_the_fixed_cn_oracle() {
    let historical: Value = serde_json::from_str(include_str!(
        "../../../tests/utility/migration/fixtures.json"
    ))
    .unwrap();
    let oracle: Value = serde_json::from_str(include_str!(
        "../../../tests/utility/migration/cn-expected.json"
    ))
    .unwrap();
    for (input, expected) in historical["cases"]
        .as_array()
        .unwrap()
        .iter()
        .zip(oracle["cases"].as_array().unwrap())
    {
        assert_eq!(input["id"], expected["id"]);
        let caller = input["callerClass"].as_str().unwrap();
        let actual = if input["kind"] == "button_classes" {
            let variant = match input["variant"].as_str().unwrap() {
                "Default" => ButtonVariant::Default,
                "Secondary" => ButtonVariant::Secondary,
                "Outline" => ButtonVariant::Outline,
                "Ghost" => ButtonVariant::Ghost,
                "Accent" => ButtonVariant::Accent,
                "Destructive" => ButtonVariant::Destructive,
                "Link" => ButtonVariant::Link,
                other => panic!("unrecognized captured variant: {other}"),
            };
            let size = match input["size"].as_str().unwrap() {
                "Default" => ButtonSize::Default,
                "Sm" => ButtonSize::Sm,
                "Lg" => ButtonSize::Lg,
                "Icon" => ButtonSize::Icon,
                other => panic!("unrecognized captured size: {other}"),
            };
            let classes = ButtonClass { variant, size };
            let raw: Vec<_> = input["rawFragments"]
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item.as_str().unwrap())
                .collect();
            assert_eq!(classes.raw_class(), crate::cn::join_text(&raw));
            classes.with_class(caller)
        } else {
            let raw: Vec<_> = input["rawFragments"]
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item.as_str().unwrap())
                .collect();
            crate::macros::merge(crate::cn::join_text(&raw), caller)
        };
        assert_eq!(
            actual,
            expected["expected"].as_str().unwrap(),
            "{}",
            input["id"]
        );
    }
}
