use super::{public_api, Limits};
use crate::text::Text;
use serde_json::Value;

#[test]
fn theme_payloads_reach_all_family_outputs_as_original_utf16_units() {
    let fixture: Value = serde_json::from_str(include_str!("utf16-family-fixtures.json")).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 560);
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let input =
            Text::from_units(serde_json::from_value(case["inputUtf16Units"].clone()).unwrap());
        let candidates: Vec<String> = serde_json::from_value(case["candidates"].clone()).unwrap();
        let expected: Vec<u16> =
            serde_json::from_value(case["expectedCssUtf16Units"].clone()).unwrap();
        let limits = Limits {
            input_bytes: 262144,
            asset_bytes: 262144,
            candidate_count: 1024,
            ast_depth: 32,
            css_bytes: 262144,
            queued_bytes: 262144,
            cache_bytes: 16000000,
        };
        let mut compiler = public_api::compile_text(&input, Default::default(), Vec::new(), limits)
            .unwrap_or_else(|error| panic!("{id}: compile {error:?}"));
        let css = compiler
            .build(&candidates)
            .unwrap_or_else(|error| panic!("{id}: build {error:?}"));
        let banner: Vec<_> = "/*! tailwindcss v4.1.13 | MIT License | https://tailwindcss.com */\n"
            .encode_utf16()
            .collect();
        let actual = css
            .units()
            .strip_prefix(banner.as_slice())
            .unwrap_or(css.units());
        assert_eq!(actual, expected, "{id}");
    }
}
