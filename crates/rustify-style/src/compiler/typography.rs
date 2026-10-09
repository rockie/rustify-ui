//! Native font-family, weight, size, line-height, letter-spacing and stretch branches.

use super::value::concat_text;
use super::{
    ast::AstNode, candidate::CandidateValue, dependencies::property, theme::Theme, DiagnosticKind,
};
use crate::text::Text;
use std::collections::BTreeSet;

pub struct TypographyOutput {
    pub declarations: Vec<(Text, Text)>,
    pub globals: Vec<AstNode>,
    pub declaration_count: usize,
}

fn output(declarations: Vec<(Text, Text)>, resource: Option<&str>) -> TypographyOutput {
    TypographyOutput {
        declaration_count: declarations.len(),
        declarations,
        globals: resource
            .map(|name| vec![property(name, None, "*")])
            .unwrap_or_default(),
    }
}

// Theme namespaces overlap syntactically but are distinct upstream APIs.
fn ignored(namespace: &str, name: &Text) -> bool {
    let ignored: &[&str] = match namespace {
        "font" => &["weight", "size"],
        "text" => &[
            "color",
            "decoration-color",
            "decoration-thickness",
            "indent",
            "shadow",
            "underline-offset",
        ],
        _ => &[],
    };
    ignored.iter().any(|prefix| {
        name == *prefix
            || name
                .strip_prefix(prefix)
                .is_some_and(|rest| rest.starts_with("-"))
    })
}

type ResolvedOptions = (Text, Vec<Option<Text>>);

// Each nested option uses its own inline/reference flags, not its parent's.
fn resolve_with(
    name: &Text,
    namespace: &str,
    nested: &[&str],
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Option<ResolvedOptions>, super::utilities::FamilyFailure> {
    if ignored(namespace, name) {
        return Ok(None);
    }
    for key in [
        concat_text(&[&format!("--{namespace}-").into(), name]),
        concat_text(&[
            &format!("--{namespace}-").into(),
            &name.replace_ascii(b'.', "_"),
        ]),
    ] {
        for key in [key.clone(), super::candidate::escape_text(&key)] {
            if let Some(base) = theme.resolve_text(&key, used) {
                let options = nested
                    .iter()
                    .map(|suffix| {
                        theme.resolve_text(&concat_text(&[&key, &(*suffix).into()]), used)
                    })
                    .collect();
                return Ok(Some((base, options)));
            }
        }
    }
    Ok(None)
}

fn spacing_multiplier(value: &Text) -> bool {
    super::colors::canonical_nonnegative_text(value, false)
}

fn line_height(
    modifier: &Text,
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Option<Text>, super::utilities::FamilyFailure> {
    if let Some(value) = modifier
        .strip_prefix("[")
        .and_then(|value| value.strip_suffix("]"))
    {
        return match super::value::decode_arbitrary_text(&value, usize::MAX) {
            Ok(value) => Ok(Some(value)),
            Err(super::CompileError::Syntax { .. }) => Ok(None),
            Err(error) => Err(error.into()),
        };
    }
    if let Some(variable) = modifier
        .strip_prefix("(")
        .and_then(|value| value.strip_suffix(")"))
    {
        return Ok(Some(concat_text(&[&"var(".into(), &variable, &")".into()])));
    }
    if let Some(value) = super::colors::resolve_named(modifier, &["leading"], theme, used)? {
        return Ok(Some(value));
    }
    if spacing_multiplier(modifier) {
        return Ok(theme
            .resolve_text(&Text::from("--spacing"), used)
            .map(|spacing| {
                let mut output = Text::from("calc(");
                output.push_text(&spacing);
                output.push_str(" * ");
                output.push_text(modifier);
                output.push_str(")");
                output
            }));
    }
    Ok((modifier == "none").then(|| "1".into()))
}

fn family_name(value: &Text) -> bool {
    if value.starts_with("var(") || super::colors::is_number_text(value) {
        return false;
    }
    let Ok(parts) = super::value::segment_text(value, u16::from(b','), usize::MAX) else {
        return false;
    };
    let mut count = 0;
    for part in parts {
        if part
            .units()
            .first()
            .is_some_and(|unit| (48..=57).contains(unit))
        {
            return false;
        }
        if part.starts_with("var(") {
            continue;
        }
        count += 1;
    }
    count > 0
}

fn stretch_number(value: &Text) -> Option<f64> {
    super::colors::js_number_text(value)
}

fn mark_variables(
    value: &Text,
    used: &mut BTreeSet<Text>,
) -> Result<(), super::utilities::FamilyFailure> {
    fn walk(nodes: &[super::value::ValueNode], used: &mut BTreeSet<Text>) {
        for node in nodes {
            if let super::value::ValueNode::Function { name, nodes } = node {
                if name == "var" {
                    if let Some(super::value::ValueNode::Word { value }) = nodes.first() {
                        if value.starts_with("--") {
                            used.insert(value.clone());
                        }
                    }
                }
                walk(nodes, used);
            }
        }
    }
    match super::value::parse_text(value, usize::MAX) {
        Ok(nodes) => walk(&nodes, used),
        Err(super::CompileError::Syntax { .. }) => {}
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

pub fn compile(
    root: &str,
    value: Option<&CandidateValue>,
    modifier: Option<&Text>,
    theme: &Theme,
    used: &mut BTreeSet<Text>,
) -> Result<Option<TypographyOutput>, super::utilities::FamilyFailure> {
    let result = match root {
        "font" => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            match value.ok_or(super::utilities::FamilyFailure::Candidate(
                DiagnosticKind::InvalidValue,
            ))? {
                CandidateValue::Arbitrary { value, data_type } => {
                    let family = match data_type.as_ref().map(super::colors::known_type) {
                        Some("generic-name" | "family-name") => true,
                        Some(_) => false,
                        None => family_name(value),
                    };
                    if family {
                        output(vec![("font-family".into(), value.clone())], None)
                    } else {
                        output(
                            vec![
                                ("--tw-font-weight".into(), value.clone()),
                                ("font-weight".into(), value.clone()),
                            ],
                            Some("--tw-font-weight"),
                        )
                    }
                }
                CandidateValue::Named { value } => {
                    if let Some((family, options)) = resolve_with(
                        value,
                        "font",
                        &["--font-feature-settings", "--font-variation-settings"],
                        theme,
                        used,
                    )? {
                        let mut declarations = vec![("font-family".into(), family)];
                        for (key, value) in ["font-feature-settings", "font-variation-settings"]
                            .into_iter()
                            .zip(options)
                        {
                            if let Some(value) = value {
                                declarations.push((key.into(), value));
                            }
                        }
                        output(declarations, None)
                    } else {
                        let weight =
                            super::colors::resolve_named(value, &["font-weight"], theme, used)?
                                .ok_or(super::utilities::FamilyFailure::Candidate(
                                    DiagnosticKind::InvalidValue,
                                ))?;
                        output(
                            vec![
                                ("--tw-font-weight".into(), weight.clone()),
                                ("font-weight".into(), weight),
                            ],
                            Some("--tw-font-weight"),
                        )
                    }
                }
            }
        }
        "text" => {
            let value = value.ok_or(super::utilities::FamilyFailure::Candidate(
                DiagnosticKind::InvalidValue,
            ))?;
            // Color has precedence over named size. Probe without committing
            // dependencies; the actual color branch collects its own token use.
            match super::colors::compile(root, Some(value), modifier, theme, &mut BTreeSet::new()) {
                Ok(Some(_)) => return Ok(None),
                Err(super::utilities::FamilyFailure::Candidate(DiagnosticKind::PendingFamily)) => {}
                Err(kind) => return Err(kind),
                Ok(None) => return Ok(None),
            }
            let (size, options) = match value {
                CandidateValue::Arbitrary { value, .. } => (value.clone(), Vec::new()),
                CandidateValue::Named { value } => resolve_with(
                    value,
                    "text",
                    if modifier.is_some() {
                        &[]
                    } else {
                        &["--line-height", "--letter-spacing", "--font-weight"]
                    },
                    theme,
                    used,
                )?
                .ok_or(super::utilities::FamilyFailure::Candidate(
                    DiagnosticKind::InvalidValue,
                ))?,
            };
            let mut declarations = vec![("font-size".into(), size)];
            if let Some(modifier) = modifier {
                declarations.push((
                    "line-height".into(),
                    line_height(modifier, theme, used)?.ok_or(
                        super::utilities::FamilyFailure::Candidate(DiagnosticKind::InvalidValue),
                    )?,
                ));
            } else {
                for ((key, variable), value) in [
                    ("line-height", "--tw-leading"),
                    ("letter-spacing", "--tw-tracking"),
                    ("font-weight", "--tw-font-weight"),
                ]
                .into_iter()
                .zip(options)
                {
                    if let Some(value) = value {
                        let mut output = Text::from(format!("var({variable}, "));
                        output.push_text(&value);
                        output.push_str(")");
                        declarations.push((key.into(), output));
                    }
                }
            }
            output(declarations, None)
        }
        "leading" | "tracking" | "-tracking" => {
            let negative = root.starts_with('-');
            let root = root.trim_start_matches('-');
            let css = match value {
                None if root == "tracking" && modifier.is_none() => {
                    theme.resolve_text(&Text::from("--tracking"), used).ok_or(
                        super::utilities::FamilyFailure::Candidate(DiagnosticKind::InvalidValue),
                    )?
                }
                None => return Err(DiagnosticKind::InvalidValue.into()),
                Some(CandidateValue::Arbitrary { value, .. }) => {
                    if modifier.is_some() {
                        return Err(DiagnosticKind::InvalidValue.into());
                    }
                    value.clone()
                }
                Some(CandidateValue::Named { value }) => {
                    let fraction = modifier
                        .filter(|modifier| {
                            !(modifier.starts_with("[") || modifier.starts_with("("))
                        })
                        .map(|modifier| concat_text(&[value, &"/".into(), modifier]));
                    let name = fraction.as_ref().unwrap_or(value);
                    let namespaces: &[&str] = if root == "leading" {
                        &["leading", "spacing"]
                    } else {
                        &["tracking"]
                    };
                    if let Some(value) =
                        super::colors::resolve_named(name, namespaces, theme, used)?
                    {
                        value
                    } else if root == "leading" && spacing_multiplier(value) {
                        let spacing = theme.resolve_text(&Text::from("--spacing"), used).ok_or(
                            super::utilities::FamilyFailure::Candidate(
                                DiagnosticKind::InvalidValue,
                            ),
                        )?;
                        let mut css = Text::from("calc(");
                        css.push_text(&spacing);
                        css.push_str(" * ");
                        css.push_text(value);
                        css.push_str(")");
                        if modifier.is_some() && !css.contains("/") {
                            return Err(DiagnosticKind::InvalidValue.into());
                        }
                        css
                    } else {
                        return Err(DiagnosticKind::InvalidValue.into());
                    }
                }
            };
            let css = if negative {
                let mut output = Text::from("calc(");
                output.push_text(&css);
                output.push_str(" * -1)");
                output
            } else {
                css
            };
            let (variable, property) = if root == "leading" {
                ("--tw-leading", "line-height")
            } else {
                ("--tw-tracking", "letter-spacing")
            };
            output(
                vec![(variable.into(), css.clone()), (property.into(), css)],
                Some(variable),
            )
        }
        "font-stretch" => {
            if modifier.is_some() {
                return Err(DiagnosticKind::InvalidValue.into());
            }
            let css = match value.ok_or(super::utilities::FamilyFailure::Candidate(
                DiagnosticKind::InvalidValue,
            ))? {
                CandidateValue::Arbitrary { value, .. } => value.clone(),
                CandidateValue::Named { value } => {
                    let number = value
                        .strip_suffix("%")
                        .and_then(|value| stretch_number(&value))
                        .ok_or(super::utilities::FamilyFailure::Candidate(
                            DiagnosticKind::InvalidValue,
                        ))?;
                    if !(50.0..=200.0).contains(&number) || number.fract() != 0.0 {
                        return Err(DiagnosticKind::InvalidValue.into());
                    }
                    value.clone()
                }
            };
            output(vec![("font-stretch".into(), css)], None)
        }
        _ => return Ok(None),
    };
    for (_, value) in &result.declarations {
        mark_variables(value, used)?;
    }
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, CompilerInput, Limits};

    #[test]
    fn nested_typography_option_preserves_non_scalar_units() {
        use super::{compile, CandidateValue, Theme};
        use crate::text::Text;
        use std::collections::BTreeSet;

        let mut line_height = Text::from("var(--line-");
        line_height.push_text(&Text::from_units(vec![0xd800]));
        line_height.push_str(")");
        let mut css = Text::from("@theme inline { --text-brand: 16px; --text-brand--line-height: ");
        css.push_text(&line_height);
        css.push_str("; }");
        let mut nodes = crate::compiler::parser::parse_text(&css, 32).unwrap();
        let theme = Theme::extract(&mut nodes, 32).unwrap();
        let candidate = CandidateValue::Named {
            value: "brand".into(),
        };
        let mut used = BTreeSet::new();
        let result = compile("text", Some(&candidate), None, &theme, &mut used)
            .unwrap()
            .unwrap();
        let mut expected = Text::from("var(--tw-leading, ");
        expected.push_text(&line_height);
        expected.push_str(")");
        assert_eq!(result.declarations[1].1.units(), expected.units());
        let mut variable = Text::from("--line-");
        variable.push_text(&Text::from_units(vec![0xd800]));
        assert!(used.contains(&variable));
    }

    #[test]
    fn typography_branches_match_official_fixtures() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/typography-fixtures.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["cases"].as_array().unwrap() {
            let mut compiler = Compiler::new(
                CompilerInput {
                    css: case["input"].as_str().unwrap().into(),
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
            .unwrap();
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
}
