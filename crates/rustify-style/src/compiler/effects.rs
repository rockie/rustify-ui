//! Ring, shadow, and generated content declarations with ordered resources.

use super::colors::resolve_named;
use super::{
    ast::AstNode, candidate::CandidateValue, dependencies, theme::Theme, value::concat_text,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

const BOX_SHADOW: &str = "var(--tw-inset-shadow), var(--tw-inset-ring-shadow), var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow)";

pub struct Effect {
    pub declarations: Vec<(Text, Text)>,
    pub globals: Vec<AstNode>,
}

fn color(
    value: &CandidateValue,
    namespace: &str,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<Text>, super::utilities::FamilyFailure> {
    Ok(match value {
        CandidateValue::Arbitrary { value, data_type }
            if data_type.as_ref().map(super::colors::known_type) == Some("color")
                || value.starts_with("#")
                || ["rgb(", "hsl(", "oklch(", "oklab(", "color("]
                    .iter()
                    .any(|prefix| value.starts_with(prefix)) =>
        {
            Some(value.clone())
        }
        CandidateValue::Named { value } => match value {
            value if value == "current" => Some("currentcolor".into()),
            value if value == "inherit" || value == "transparent" => Some(value.clone()),
            _ => match theme.resolve_text(
                &concat_text(&[&format!("--{namespace}-").into(), value]),
                used,
            ) {
                Some(value) => Some(value),
                None => theme.resolve_text(&concat_text(&[&"--color-".into(), value]), used),
            },
        },
        _ => None,
    })
}

fn length(value: &Text) -> bool {
    [
        "px", "rem", "em", "vh", "vw", "vmin", "vmax", "cm", "mm", "in", "pt", "pc", "ch", "ex",
        "lh", "rlh", "svh", "lvh", "dvh", "svw", "lvw", "dvw", "cqw", "cqh", "cqi", "cqb", "cqmin",
        "cqmax", "Q", "vb", "vi",
    ]
    .iter()
    .any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|number| !number.contains("(") && super::colors::is_number_text(&number))
    }) || ["calc(", "min(", "max(", "clamp("]
        .iter()
        .any(|prefix| value.starts_with(prefix))
}

fn shadow_colors(input: &Text, max_bytes: usize) -> Result<Text, super::utilities::FamilyFailure> {
    super::shadows::replace_shadow_colors(input, max_bytes, |color| {
        concat_text(&[&"var(--tw-shadow-color, ".into(), color, &")".into()])
    })
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
    max_bytes: usize,
) -> Result<Option<Effect>, super::utilities::FamilyFailure> {
    let radius_properties: &[&str] = match root {
        "rounded" => &["border-radius"],
        "rounded-s" => &["border-start-start-radius", "border-end-start-radius"],
        "rounded-e" => &["border-start-end-radius", "border-end-end-radius"],
        "rounded-t" => &["border-top-left-radius", "border-top-right-radius"],
        "rounded-r" => &["border-top-right-radius", "border-bottom-right-radius"],
        "rounded-b" => &["border-bottom-right-radius", "border-bottom-left-radius"],
        "rounded-l" => &["border-top-left-radius", "border-bottom-left-radius"],
        "rounded-ss" => &["border-start-start-radius"],
        "rounded-se" => &["border-start-end-radius"],
        "rounded-ee" => &["border-end-end-radius"],
        "rounded-es" => &["border-end-start-radius"],
        "rounded-tl" => &["border-top-left-radius"],
        "rounded-tr" => &["border-top-right-radius"],
        "rounded-br" => &["border-bottom-right-radius"],
        "rounded-bl" => &["border-bottom-left-radius"],
        _ => &[],
    };
    if !radius_properties.is_empty() {
        let resolved = match value {
            None if modifier.is_none() => theme.resolve_text(&Text::from("--radius"), used),
            Some(CandidateValue::Named { value }) => {
                let fraction = modifier
                    .filter(|modifier| !(modifier.starts_with("[") || modifier.starts_with("(")))
                    .map(|modifier| concat_text(&[value, &"/".into(), modifier]));
                resolve_named(fraction.as_ref().unwrap_or(value), &["radius"], theme, used)?
            }
            Some(CandidateValue::Arbitrary { value, .. }) if modifier.is_none() => {
                Some(value.clone())
            }
            _ => None,
        }
        .ok_or(DiagnosticKind::InvalidValue)?;
        return Ok(Some(Effect {
            declarations: radius_properties
                .iter()
                .map(|property| ((*property).into(), resolved.clone()))
                .collect(),
            globals: Vec::new(),
        }));
    }
    if !matches!(
        root,
        "ring" | "shadow" | "content" | "ring-inset" | "shadow-initial"
    ) {
        return Ok(None);
    }
    if modifier.is_some() {
        return Err((if root == "content" {
            DiagnosticKind::InvalidValue
        } else {
            DiagnosticKind::PendingFamily
        })
        .into());
    }
    let mut effect = Effect {
        declarations: Vec::new(),
        globals: Vec::new(),
    };
    let mut declaration =
        |property: &str, value: Text| effect.declarations.push((property.into(), value));
    match root {
        "ring-inset" => {
            effect.globals = dependencies::box_shadow_properties();
            declaration("--tw-ring-inset", "inset".into());
        }
        "shadow-initial" => {
            effect.globals = dependencies::box_shadow_properties();
            declaration("--tw-shadow-color", "initial".into());
        }
        "content" => {
            let Some(CandidateValue::Arbitrary { value, .. }) = value else {
                return Err(DiagnosticKind::InvalidValue.into());
            };
            effect.globals.push(dependencies::content_property(false));
            declaration("--tw-content", value.clone());
            declaration("content", "var(--tw-content)".into());
        }
        "ring" => {
            let width = match value {
                None => Some(
                    theme
                        .raw_text(&Text::from("--default-ring-width"), used)
                        .unwrap_or_else(|| "1px".into()),
                ),
                Some(CandidateValue::Arbitrary { value, data_type })
                    if data_type.as_ref().map(super::colors::known_type) == Some("length")
                        || (data_type.is_none() && length(value)) =>
                {
                    Some(value.clone())
                }
                Some(CandidateValue::Named { value }) => {
                    if let Some(color) = color(
                        &CandidateValue::Named {
                            value: value.clone(),
                        },
                        "ring-color",
                        theme,
                        used,
                    )? {
                        declaration("--tw-ring-color", color);
                        return Ok(Some(effect));
                    }
                    theme
                        .resolve_text(&concat_text(&[&"--ring-width-".into(), value]), used)
                        .or_else(|| {
                            super::colors::canonical_nonnegative_text(value, true)
                                .then(|| concat_text(&[value, &"px".into()]))
                        })
                }
                Some(CandidateValue::Arbitrary { value, .. }) => {
                    declaration("--tw-ring-color", value.clone());
                    return Ok(Some(effect));
                }
            };
            let Some(width) = width else {
                return Err(DiagnosticKind::InvalidValue.into());
            };
            effect.globals = dependencies::box_shadow_properties();
            let color = theme
                .raw_text(&Text::from("--default-ring-color"), used)
                .unwrap_or_else(|| "currentcolor".into());
            declaration(
                "--tw-ring-shadow",
                concat_text(&[
                    &"var(--tw-ring-inset,) 0 0 0 calc(".into(),
                    &width,
                    &" + var(--tw-ring-offset-width)) var(--tw-ring-color, ".into(),
                    &color,
                    &")".into(),
                ]),
            );
            declaration("box-shadow", BOX_SHADOW.into());
        }
        "shadow" => {
            effect.globals = dependencies::box_shadow_properties();
            let shadow = match value {
                None => theme.raw_text(&Text::from("--shadow"), used),
                Some(CandidateValue::Named { value }) if value == "none" => {
                    Some("0 0 #0000".into())
                }
                Some(CandidateValue::Named { value }) if value == "inherit" => {
                    declaration("--tw-shadow-color", "inherit".into());
                    return Ok(Some(effect));
                }
                Some(CandidateValue::Named { value }) => {
                    theme.raw_text(&concat_text(&[&"--shadow-".into(), value]), used)
                }
                Some(value @ CandidateValue::Arbitrary { .. }) => {
                    if let Some(color) = color(value, "box-shadow-color", theme, used)? {
                        declaration(
                            "--tw-shadow-color",
                            concat_text(&[
                                &"color-mix(in oklab, ".into(),
                                &color,
                                &" var(--tw-shadow-alpha), transparent)".into(),
                            ]),
                        );
                        return Ok(Some(effect));
                    }
                    let CandidateValue::Arbitrary { value, .. } = value else {
                        unreachable!()
                    };
                    Some(value.clone())
                }
            };
            if let Some(shadow) = shadow {
                let shadow = if matches!(value, Some(CandidateValue::Named { value }) if value == "none")
                {
                    shadow
                } else {
                    shadow_colors(&shadow, max_bytes)?
                };
                declaration("--tw-shadow", shadow);
                declaration("box-shadow", BOX_SHADOW.into());
            } else if let Some(color) = match value {
                Some(value) => color(value, "box-shadow-color", theme, used)?,
                None => None,
            } {
                declaration(
                    "--tw-shadow-color",
                    concat_text(&[
                        &"color-mix(in oklab, ".into(),
                        &color,
                        &" var(--tw-shadow-alpha), transparent)".into(),
                    ]),
                );
            } else {
                return Err(DiagnosticKind::InvalidValue.into());
            }
        }
        _ => unreachable!(),
    }
    Ok(Some(effect))
}

#[cfg(test)]
mod complete_tests {
    use crate::compiler::{
        ast::AstNode, CompileError, Compiler, CompilerInput, DiagnosticKind, Limits,
    };

    fn compiler(css: &str) -> Result<Compiler, CompileError> {
        Compiler::new(
            CompilerInput {
                css: css.into(),
                entry: String::new(),
            },
            Vec::new(),
            Limits {
                input_bytes: 65536,
                asset_bytes: 65536,
                candidate_count: 1024,
                ast_depth: 32,
                css_bytes: 262144,
                queued_bytes: 65536,
                cache_bytes: 1048576,
            },
        )
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn complete_radius_and_content_branches_match_official_matrix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/effects-complete-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let mut compiler = match compiler(case["input"].as_str().unwrap()) {
                Ok(compiler) => compiler,
                Err(error) => {
                    failures.push(format!(
                        "{}: constructor error {error:?}",
                        case["id"].as_str().unwrap()
                    ));
                    continue;
                }
            };
            let candidates = case["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            let prepared = compiler.prepare_add(&candidates).unwrap();
            let report = prepared.report();
            if report.css != case["expectedCss"].as_str().unwrap() || !report.unsupported.is_empty()
            {
                failures.push(format!(
                    "{}: actual={:?} expected={:?} pending={:?}",
                    case["id"].as_str().unwrap(),
                    report.css,
                    case["expectedCss"].as_str().unwrap(),
                    report.unsupported
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "{} mismatches; first cases: {:#?}",
            failures.len(),
            &failures[..failures.len().min(10)]
        );
    }

    #[test]
    fn fractional_radius_theme_and_content_resources_keep_distinct_contracts() {
        let mut compiler = compiler("@theme { --radius-card: 13px; --radius-card\\/foo: 17px; --radius-1_5: 19px; --content-card: \"unused\"; } @tailwind utilities;").unwrap();
        let candidates = [
            "rounded-card/foo",
            "rounded-card/[13px]",
            "rounded-1.5",
            "before:content-[\"probe\"]!",
            "after:content-none!",
            "content-card",
            "content-[\"invalid\"]/50",
        ];
        let prepared = compiler
            .prepare_add(&candidates.map(str::to_owned))
            .unwrap();
        let report = prepared.report();
        for candidate in &candidates[..5] {
            assert!(
                report.admitted.iter().any(|item| item == candidate),
                "missing {candidate}"
            );
        }
        assert!(report.unsupported.is_empty());
        for candidate in &candidates[5..] {
            assert!(
                report
                    .rejected
                    .iter()
                    .any(|item| item.candidate == *candidate
                        && item.kind == DiagnosticKind::InvalidValue),
                "missing invalid value {candidate}"
            );
        }
        assert!(report
            .css
            .contains("border-radius: var(--radius-card\\/foo)"));
        assert!(report.css.contains("border-radius: var(--radius-1_5)"));
        assert!(!report.css.contains("--content-card:"));
        let properties = prepared
            .ast()
            .iter()
            .filter_map(|node| match node {
                AstNode::AtRule {
                    name,
                    params,
                    nodes: Some(nodes),
                    ..
                } if name == "@property" && params == "--tw-content" => Some(nodes),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(properties.len(), 1);
        assert!(properties[0].iter().all(|node| matches!(
            node,
            AstNode::Declaration {
                important: false,
                ..
            }
        )));
        assert!(report.css.contains("--tw-content: \"probe\" !important"));
        assert!(report.css.contains("--tw-content: none !important"));
    }
}
