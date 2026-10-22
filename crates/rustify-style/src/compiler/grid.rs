//! Native grid tracks and placement declarations.

use super::value::concat_text;
use super::{
    candidate::CandidateValue,
    colors::{canonical_nonnegative_text, resolve_named},
    theme::Theme,
    DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct GridOutput {
    pub declarations: Vec<(Text, Text)>,
    pub declaration_count: usize,
}

fn output(property: &str, value: Text) -> GridOutput {
    GridOutput {
        declarations: vec![(property.into(), value)],
        declaration_count: 1,
    }
}

pub fn compile(
    root: &str,
    candidate_value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<crate::text::Text>,
) -> Result<Option<GridOutput>, super::utilities::FamilyFailure> {
    let static_value = match root {
        "col-auto" => Some(("grid-column", "auto")),
        "col-span-full" => Some(("grid-column", "1 / -1")),
        "col-start-auto" => Some(("grid-column-start", "auto")),
        "col-end-auto" => Some(("grid-column-end", "auto")),
        "row-auto" => Some(("grid-row", "auto")),
        "row-span-full" => Some(("grid-row", "1 / -1")),
        "row-start-auto" => Some(("grid-row-start", "auto")),
        "row-end-auto" => Some(("grid-row-end", "auto")),
        "grid-flow-row" => Some(("grid-auto-flow", "row")),
        "grid-flow-col" => Some(("grid-auto-flow", "column")),
        "grid-flow-dense" => Some(("grid-auto-flow", "dense")),
        "grid-flow-row-dense" => Some(("grid-auto-flow", "row dense")),
        "grid-flow-col-dense" => Some(("grid-auto-flow", "column dense")),
        "auto-cols-auto" => Some(("grid-auto-columns", "auto")),
        "auto-cols-min" => Some(("grid-auto-columns", "min-content")),
        "auto-cols-max" => Some(("grid-auto-columns", "max-content")),
        "auto-cols-fr" => Some(("grid-auto-columns", "minmax(0, 1fr)")),
        "auto-rows-auto" => Some(("grid-auto-rows", "auto")),
        "auto-rows-min" => Some(("grid-auto-rows", "min-content")),
        "auto-rows-max" => Some(("grid-auto-rows", "max-content")),
        "auto-rows-fr" => Some(("grid-auto-rows", "minmax(0, 1fr)")),
        "grid-cols-none" => Some(("grid-template-columns", "none")),
        "grid-cols-subgrid" => Some(("grid-template-columns", "subgrid")),
        "grid-rows-none" => Some(("grid-template-rows", "none")),
        "grid-rows-subgrid" => Some(("grid-template-rows", "subgrid")),
        _ => None,
    };
    if let Some((property, value)) = static_value {
        return Ok(Some(output(property, value.into())));
    }
    let negative = root.starts_with('-');
    let root = root.strip_prefix('-').unwrap_or(root);
    if negative
        && !matches!(
            root,
            "col" | "col-start" | "col-end" | "row" | "row-start" | "row-end"
        )
    {
        return Ok(None);
    }
    let (property, namespace) = match root {
        "col" => ("grid-column", Some("grid-column")),
        "col-start" => ("grid-column-start", Some("grid-column-start")),
        "col-end" => ("grid-column-end", Some("grid-column-end")),
        "row" => ("grid-row", Some("grid-row")),
        "row-start" => ("grid-row-start", Some("grid-row-start")),
        "row-end" => ("grid-row-end", Some("grid-row-end")),
        "col-span" => ("grid-column", None),
        "row-span" => ("grid-row", None),
        "grid-cols" => ("grid-template-columns", Some("grid-template-columns")),
        "grid-rows" => ("grid-template-rows", Some("grid-template-rows")),
        "auto-cols" => ("grid-auto-columns", Some("grid-auto-columns")),
        "auto-rows" => ("grid-auto-rows", Some("grid-auto-rows")),
        _ => return Ok(None),
    };
    let template = matches!(root, "grid-cols" | "grid-rows");
    let mut value = match candidate_value {
        None => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            namespace
                .and_then(|namespace| {
                    theme.resolve_text(&Text::from(format!("--{namespace}")), used)
                })
                .ok_or(super::utilities::FamilyFailure::Candidate(
                    DiagnosticKind::InvalidValue,
                ))?
        }
        Some(CandidateValue::Arbitrary { value, .. }) => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            value.clone()
        }
        Some(CandidateValue::Named { value }) => {
            let fraction = modifier
                .filter(|modifier| !modifier.starts_with("[") && !modifier.starts_with("("))
                .map(|modifier| concat_text(&[value, &"/".into(), modifier]));
            let resolved = namespace
                .map(|namespace| {
                    resolve_named(
                        fraction.as_ref().unwrap_or(value),
                        &[namespace],
                        theme,
                        used,
                    )
                })
                .transpose()?
                .flatten();
            if let Some(value) = resolved {
                value
            } else {
                if matches!(root, "auto-cols" | "auto-rows")
                    || modifier.is_some()
                    || !canonical_nonnegative_text(value, true)
                    || template && value == "0"
                {
                    return Err(DiagnosticKind::InvalidValue.into());
                }
                if template {
                    concat_text(&[&"repeat(".into(), value, &", minmax(0, 1fr))".into()])
                } else {
                    value.clone()
                }
            }
        }
    };
    if negative {
        value = concat_text(&[&"calc(".into(), &value, &" * -1)".into()]);
    }
    if matches!(root, "col-span" | "row-span") {
        value = concat_text(&[&"span ".into(), &value, &" / span ".into(), &value]);
    }
    Ok(Some(output(property, value)))
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
    fn grid_branches_match_official_fixtures_and_prefix() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/grid-fixtures.json"
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
    fn zero_span_negative_placement_and_tracks_preserve_ir_without_resources() {
        let mut compiler = compiler("@tailwind utilities;");
        let prepared = compiler
            .prepare_add(&[
                "col-span-0".into(),
                "-col-start-7!".into(),
                "grid-cols-[minmax(13px,1fr)_2fr]".into(),
                "grid-cols-0".into(),
            ])
            .unwrap();
        let report = prepared.report();
        assert!(report.css.contains("grid-column: span 0 / span 0;"));
        assert!(report
            .css
            .contains("grid-column-start: calc(7 * -1) !important;"));
        assert!(report
            .css
            .contains("grid-template-columns: minmax(13px,1fr) 2fr;"));
        assert!(!report.css.contains("@property"));
        assert!(!report
            .admitted
            .iter()
            .any(|candidate| candidate == "grid-cols-0"));
        let program = report
            .gpu_programs
            .iter()
            .find(|program| program.candidate == "grid-cols-[minmax(13px,1fr)_2fr]")
            .unwrap();
        assert_eq!(program.declarations[0].property, "grid-template-columns");
    }
}
