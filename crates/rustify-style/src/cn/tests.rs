use super::*;
use serde_json::Value;

#[test]
#[cfg(rustify_repo_tests)]
fn frozen_default_reference_outputs() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/cn/runtime-fixtures.json"
    ))
    .unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        if !matches!(case["api"].as_str(), Some("cn" | "twMerge")) || case["recipe"] != "default" {
            continue;
        }
        let args = case["args"].as_array().unwrap();
        if args.len() != 1 || !args[0].is_string() {
            continue;
        }
        let input = args[0].as_str().unwrap();
        assert_eq!(
            merge(&[input]),
            case["expected"]["value"].as_str().unwrap(),
            "{}: {input}",
            case["id"]
        );
    }
}

#[test]
#[cfg(rustify_repo_tests)]
fn frozen_validator_predicates() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/cn/compiler-fixtures.json"
    ))
    .unwrap();
    for family in fixtures["validators"].as_array().unwrap() {
        let opcode = validators::NAMES
            .iter()
            .position(|name| *name == family["name"])
            .unwrap();
        for case in family["cases"].as_array().unwrap() {
            assert_eq!(
                validators::test(opcode, case["input"].as_str().unwrap()),
                case["predicate"].as_bool().unwrap(),
                "{} {}",
                family["name"],
                case["input"]
            );
        }
    }
}

#[test]
#[cfg(rustify_repo_tests)]
fn frozen_compiled_tables() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/cn/compiler-fixtures.json"
    ))
    .unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let Some(input) = case.get("input") else {
            continue;
        };
        let config = Config::from_json(input).unwrap();
        let compiled = compiler::compile_to_tables(&config).unwrap();
        let expected: Tables = serde_json::from_value(case["expected"]["tables"].clone()).unwrap();
        assert_eq!(compiled.tables, expected, "{}", case["id"]);
        compiled.tables.validate().unwrap();
        let stats = compiler::compile_stats(&config).unwrap();
        assert_eq!(
            serde_json::to_value(stats).unwrap(),
            case["expected"]["stats"]
        );
        use sha2::{Digest, Sha256};
        let source = compiler::compile_to_source(&config, EmitOptions::default()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            case["expected"]["sourceSha256"].as_str().unwrap(),
            "{} JS source",
            case["id"]
        );
        assert_eq!(
            source.len() as u64,
            case["expected"]["sourceBytes"].as_u64().unwrap()
        );
        let source = compiler::compile_to_source(
            &config,
            EmitOptions {
                language: EmitLanguage::Typescript,
                banner: Some("// fixture banner"),
            },
        )
        .unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            case["expected"]["tsSourceSha256"].as_str().unwrap(),
            "{} TS source",
            case["id"]
        );
    }
}

#[cfg(rustify_repo_tests)]
fn class_value(value: &Value) -> ClassValue<'_> {
    if value.is_null() {
        return ClassValue::Null;
    }
    if let Some(text) = value.as_str() {
        return ClassValue::Text(text);
    }
    if let Some(number) = value.as_f64() {
        return ClassValue::Number(number);
    }
    if let Some(value) = value.as_bool() {
        return ClassValue::Bool(value);
    }
    if let Some(values) = value.as_array() {
        return ClassValue::List(values.iter().map(class_value).collect());
    }
    match value["$js"].as_str().unwrap() {
        "undefined" => ClassValue::Undefined,
        "bigint" => ClassValue::BigInt(value["value"].as_str().unwrap()),
        "number" => ClassValue::Number(match value["value"].as_str().unwrap() {
            "NaN" => f64::NAN,
            "Infinity" => f64::INFINITY,
            "-Infinity" => f64::NEG_INFINITY,
            "-0" => -0.0,
            other => panic!("unknown number {other}"),
        }),
        "function" => ClassValue::Function,
        "object" => ClassValue::Dict(
            value["entries"]
                .as_array()
                .unwrap()
                .iter()
                .map(|pair| (pair[0].as_str().unwrap(), class_value(&pair[1])))
                .collect(),
        ),
        other => panic!("unknown value tag {other}"),
    }
}

#[cfg(rustify_repo_tests)]
fn recipe(name: &str) -> Result<Cn, CnError> {
    use std::rc::Rc;
    let extension = match name {
        "default" => return Ok(Cn::default()),
        "fullConfig" => return Cn::new(Config::default()),
        "identityTransform" => return Cn::from_transform(Ok),
        "throwsTransform" => {
            return Cn::from_transform(|_| Err(CnError::new("fixture config failure")));
        }
        "predicate" | "throwsPredicate" | "themeGetter" => {
            let definition = match name {
                "predicate" => ClassGroupDef::Validator(Rc::new(|value| Ok(value.ends_with("vw")))),
                "throwsPredicate" => ClassGroupDef::Validator(Rc::new(|_| {
                    Err(CnError::new("fixture validator failure"))
                })),
                _ => ClassGroupDef::ThemeGetter(Rc::new(|theme| {
                    Ok(theme
                        .iter()
                        .find(|(name, _)| name == "text")
                        .unwrap()
                        .1
                        .clone())
                })),
            };
            let (group, prefix) = if name == "predicate" {
                ("font-size", "text")
            } else if name == "throwsPredicate" {
                ("probe", "probe")
            } else {
                ("customText", "customtext")
            };
            return Cn::with_extension(&ConfigExtension {
                extend: ConfigGroups {
                    class_groups: vec![(
                        group.to_owned(),
                        vec![ClassGroupDef::Object(vec![(
                            prefix.to_owned(),
                            vec![definition],
                        )])],
                    )],
                    ..ConfigGroups::default()
                },
                ..ConfigExtension::default()
            });
        }
        "extendLiteral" => {
            serde_json::json!({"extend":{"classGroups":{"font-size":[{"text":["hero","tiny"]}]}}})
        }
        "extendTheme" => serde_json::json!({"extend":{"theme":{"text":["huge"]}}}),
        "directedConflict" => {
            serde_json::json!({"extend":{"classGroups":{"foo":["foo-a","foo-b"],"bar":["bar-x","bar-y"]},"conflictingClassGroups":{"foo":["bar"]}}})
        }
        "override" => serde_json::json!({"override":{"classGroups":{"shadow":["shadow-custom"]}}}),
        "prefix" => serde_json::json!({"prefix":"tw"}),
        "cacheDisabled" => serde_json::json!({"cacheSize":0}),
        "unknownValidator" => {
            serde_json::json!({"extend":{"classGroups":{"probe":[{"probe":[{"$v":"notAValidator"}]}]}}})
        }
        "orderSensitive" => serde_json::json!({"extend":{"orderSensitiveModifiers":["custom"]}}),
        other => panic!("unknown recipe {other}"),
    };
    Cn::with_extension(&ConfigExtension::from_json(&extension)?)
}

#[test]
#[cfg(rustify_repo_tests)]
fn all_frozen_runtime_outputs_include_join_config_and_callbacks() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/cn/runtime-fixtures.json"
    ))
    .unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let args: Vec<ClassValue<'_>> = case["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(class_value)
            .collect();
        let result = match case["api"].as_str().unwrap() {
            "cn" => Cn::default().merge(&args),
            "twMerge" => Cn::default().tw_merge(&args),
            "clsx" => Ok(clsx(&args)),
            "twJoin" => Ok(tw_join(&args)),
            "lite.clsx" => Ok(lite(&args)),
            api => recipe(case["recipe"].as_str().unwrap()).and_then(|cn| {
                if api == "config.createCn" {
                    cn.merge(&args)
                } else {
                    cn.tw_merge(&args)
                }
            }),
        };
        if case["expected"]["ok"] == true {
            assert_eq!(
                result.unwrap(),
                case["expected"]["value"].as_str().unwrap(),
                "{}",
                case["id"]
            );
        } else {
            assert_eq!(
                result.unwrap_err().message,
                case["expected"]["error"]["message"].as_str().unwrap(),
                "{}",
                case["id"]
            );
        }
    }
}

#[test]
#[cfg(rustify_repo_tests)]
fn frozen_cache_sequences() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/cn/compiler-fixtures.json"
    ))
    .unwrap();
    for case in fixtures["engineCases"].as_array().unwrap() {
        let cn = Cn::from_tables(
            default_tables::tables(),
            vec![],
            EngineOptions {
                cache_size: case["cacheSize"].as_u64().unwrap() as usize,
                ..EngineOptions::default()
            },
        )
        .unwrap();
        for step in case["sequence"].as_array().unwrap() {
            let input = step["input"].as_str().unwrap();
            assert_eq!(
                cn.seen_before(input),
                step["seenBefore"].as_bool().unwrap(),
                "cache {} / {input}",
                case["cacheSize"]
            );
            assert_eq!(
                cn.merge_str(input).unwrap(),
                step["mergeString"].as_str().unwrap()
            );
            assert_eq!(
                cn.merge_uncached(input).unwrap(),
                step["mergeUncached"].as_str().unwrap()
            );
        }
    }
}

#[test]
fn result_cache_respects_entry_and_byte_budgets() {
    let cn = Cn::from_tables(
        default_tables::tables(),
        vec![],
        EngineOptions {
            cache_size: 2,
            cache_bytes: Some(20),
            ..EngineOptions::default()
        },
    )
    .unwrap();
    for input in [
        "p-2 p-4",
        "m-2 m-4",
        "w-[100px] w-[200px]",
        "p-2 p-4",
        "px-4 p-2",
    ] {
        assert_eq!(cn.merge_str(input).unwrap(), merge(&[input]));
        let (entries, bytes) = cn.cache_usage();
        assert!(entries <= 2 && bytes <= 20);
    }
}

#[test]
fn invalid_engine_tables_are_rejected_before_indexing() {
    let mut tables = default_tables::tables();
    tables.edge_target[0] = i32::MAX;
    assert!(Cn::from_tables(tables, vec![], EngineOptions::default()).is_err());
}

#[test]
fn source_emission_preserves_shared_high_surrogate_prefix() {
    use sha2::{Digest, Sha256};
    let fixture: Value = serde_json::from_str(include_str!(
        "../../upstream/cn/unicode-emission.fixture.json"
    ))
    .unwrap();
    let config = Config::from_json(&fixture["input"]).unwrap();
    let source = compiler::compile_to_source(&config, EmitOptions::default()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(source.as_bytes())),
        fixture["sourceSha256"].as_str().unwrap()
    );
}

#[test]
#[cfg(rustify_repo_tests)]
fn frozen_build_token_and_full_artifacts() {
    use sha2::{Digest, Sha256};
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/cn/build-fixtures.json"
    ))
    .unwrap();
    for id in ["tokens", "full", "tsOutput"] {
        let case = fixtures["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["id"] == id)
            .unwrap();
        let out = if id == "tsOutput" {
            "tables-ts.ts".to_owned()
        } else {
            format!("tables-{id}.mjs")
        };
        let tokens = ["p-2", "p-4", "hover:p-2", "hover:p-4"];
        let report = build::build_tokens(
            &Config::default(),
            &tokens,
            build::BuildOptions {
                out: &out,
                full: id == "full",
                previous_source: None,
            },
        )
        .unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(report.source.as_bytes())),
            case["expected"]["sourceSha256"].as_str().unwrap(),
            "{id}"
        );
        assert_eq!(
            serde_json::to_value(report.used_groups).unwrap(),
            case["expected"]["usedGroups"]
        );
        assert_eq!(
            serde_json::to_value(report.total_groups).unwrap(),
            case["expected"]["totalGroups"]
        );
        let unchanged = build::build_tokens(
            &Config::default(),
            &tokens,
            build::BuildOptions {
                out: &out,
                full: id == "full",
                previous_source: Some(&report.source),
            },
        )
        .unwrap();
        assert!(!unchanged.changed);
    }
}

#[test]
#[cfg(rustify_repo_tests)]
fn frozen_build_extractor_cap() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../../tests/utility/cn/build-fixtures.json"
    ))
    .unwrap();
    let case = fixtures["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "token-length-cap")
        .unwrap();
    let content = format!("class=\"p-2 hover:p-4\" <x> {}", "x".repeat(8193));
    let mut tokens = Vec::new();
    let skipped = build::extract_tokens(&content, &mut tokens);
    assert_eq!(
        serde_json::json!({"skippedLong":skipped,"tokens":tokens}),
        case["expected"]
    );
}
