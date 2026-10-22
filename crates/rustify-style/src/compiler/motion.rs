//! Native animation and transition timing declarations and property resources.

use super::value::concat_text;
use super::{
    ast::AstNode,
    candidate::CandidateValue,
    colors::{canonical_nonnegative_text, resolve_named},
    dependencies::property,
    theme::Theme,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct MotionOutput {
    pub declarations: Vec<(Text, Text)>,
    pub globals: Vec<AstNode>,
    pub declaration_count: usize,
}

const TRANSITION_COLORS: &str = "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to";
const TRANSITION_DEFAULT: &str = "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to, opacity, box-shadow, transform, translate, scale, rotate, filter, -webkit-backdrop-filter, backdrop-filter, display, content-visibility, overlay, pointer-events";

fn output(declarations: Vec<(Text, Text)>, resource: Option<&str>) -> MotionOutput {
    MotionOutput {
        declaration_count: declarations.len(),
        declarations,
        globals: resource
            .map(|name| vec![property(name, None, "*")])
            .unwrap_or_default(),
    }
}

fn transition(
    value: Text,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<MotionOutput, super::utilities::FamilyFailure> {
    let timing = theme
        .resolve_text(&Text::from("--default-transition-timing-function"), used)
        .unwrap_or_else(|| "ease".into());
    let duration = theme
        .resolve_text(&Text::from("--default-transition-duration"), used)
        .unwrap_or_else(|| "0s".into());
    Ok(output(
        vec![
            ("transition-property".into(), value),
            (
                "transition-timing-function".into(),
                concat_text(&[&"var(--tw-ease, ".into(), &timing, &")".into()]),
            ),
            (
                "transition-duration".into(),
                concat_text(&[&"var(--tw-duration, ".into(), &duration, &")".into()]),
            ),
        ],
        None,
    ))
}

fn value(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    namespace: &str,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Text, super::utilities::FamilyFailure> {
    if root == "duration" && (modifier.is_some() || value.is_none()) {
        return Err(DiagnosticKind::InvalidValue.into());
    }
    match value {
        None => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            if root == "transition" {
                return Ok(TRANSITION_DEFAULT.into());
            }
            theme
                .resolve_text(&Text::from(format!("--{namespace}")), used)
                .ok_or(super::utilities::FamilyFailure::Candidate(
                    DiagnosticKind::InvalidValue,
                ))
        }
        Some(CandidateValue::Arbitrary { value, .. }) => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            Ok(value.clone())
        }
        Some(CandidateValue::Named { value }) => {
            let fraction = modifier
                .filter(|modifier| !modifier.starts_with("[") && !modifier.starts_with("("))
                .map(|modifier| concat_text(&[value, &"/".into(), modifier]));
            if let Some(css) = resolve_named(
                fraction.as_ref().unwrap_or(value),
                &[namespace],
                theme,
                used,
            )? {
                return Ok(css);
            }
            if matches!(root, "delay" | "duration")
                && modifier.is_none()
                && canonical_nonnegative_text(value, true)
            {
                return Ok(concat_text(&[value, &"ms".into()]));
            }
            Err(DiagnosticKind::InvalidValue.into())
        }
    }
}

pub fn compile(
    root: &str,
    candidate_value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<MotionOutput>, super::utilities::FamilyFailure> {
    let static_output = match root {
        "animate-none" => Some(output(vec![("animation".into(), "none".into())], None)),
        "transition-none" => Some(output(
            vec![("transition-property".into(), "none".into())],
            None,
        )),
        "transition-all" => Some(transition("all".into(), theme, used)?),
        "transition-colors" => Some(transition(TRANSITION_COLORS.into(), theme, used)?),
        "transition-opacity" => Some(transition("opacity".into(), theme, used)?),
        "transition-shadow" => Some(transition("box-shadow".into(), theme, used)?),
        "transition-transform" => Some(transition(
            "transform, translate, scale, rotate".into(),
            theme,
            used,
        )?),
        "transition-discrete" => Some(output(
            vec![("transition-behavior".into(), "allow-discrete".into())],
            None,
        )),
        "transition-normal" => Some(output(
            vec![("transition-behavior".into(), "normal".into())],
            None,
        )),
        "duration-initial" => Some(output(
            vec![("--tw-duration".into(), "initial".into())],
            Some("--tw-duration"),
        )),
        "ease-initial" => Some(output(
            vec![("--tw-ease".into(), "initial".into())],
            Some("--tw-ease"),
        )),
        "ease-linear" => Some(output(
            vec![
                ("--tw-ease".into(), "linear".into()),
                ("transition-timing-function".into(), "linear".into()),
            ],
            Some("--tw-ease"),
        )),
        _ => None,
    };
    if static_output.is_some() {
        return Ok(static_output);
    }
    let namespace = match root {
        "animate" => "animate",
        "transition" => "transition-property",
        "delay" => "transition-delay",
        "duration" => "transition-duration",
        "ease" => "ease",
        _ => return Ok(None),
    };
    let value = value(root, candidate_value, modifier, namespace, theme, used)?;
    Ok(Some(match root {
        "transition" => transition(value, theme, used)?,
        "animate" => output(vec![("animation".into(), value)], None),
        "delay" => output(vec![("transition-delay".into(), value)], None),
        "duration" => output(
            vec![
                ("--tw-duration".into(), value.clone()),
                ("transition-duration".into(), value),
            ],
            Some("--tw-duration"),
        ),
        _ => output(
            vec![
                ("--tw-ease".into(), value.clone()),
                ("transition-timing-function".into(), value),
            ],
            Some("--tw-ease"),
        ),
    }))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, CompilerInput, Limits};

    fn compiler(css: &str) -> Compiler {
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
        .unwrap()
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn motion_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/motion-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let mut compiler = compiler(case["input"].as_str().unwrap());
            let candidates = case["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| crate::text::Text::from(value.as_str().unwrap()))
                .collect::<Vec<_>>();
            let prepared = compiler.prepare_add_text(&candidates).unwrap();
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
    fn animation_keeps_keyframe_dependencies_and_timing_resources_are_deduplicated() {
        let mut compiler = compiler(
            "@theme { --animate-card: card 1s; @keyframes card { to { opacity: 0; } } @keyframes unused { to { opacity: 1; } } } @tailwind utilities;",
        );
        let prepared = compiler
            .prepare_add(&[
                "animate-card!".into(),
                "duration-150".into(),
                "duration-200".into(),
                "ease-linear".into(),
            ])
            .unwrap();
        let css = prepared.report().css.to_utf8().unwrap();
        assert!(css.contains("@keyframes card"));
        assert!(!css.contains("@keyframes unused"));
        assert_eq!(css.matches("@property --tw-duration").count(), 1);
        assert_eq!(css.matches("@property --tw-ease").count(), 1);
        assert!(css.contains("animation: var(--animate-card) !important;"));
        let program = prepared
            .report()
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "animate-card!")
            .unwrap();
        assert!(program
            .declarations
            .iter()
            .any(|declaration| declaration.property == "animation" && declaration.important));
    }
}
