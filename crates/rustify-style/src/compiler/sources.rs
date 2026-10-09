//! CSS source declarations and bounded inline candidate expansion.

use super::{value, CompileError, Limits, SourceMetadata, SourcePattern, SourceRoot};
use crate::text::Text;
use std::collections::BTreeSet;

#[derive(Default)]
pub(super) struct Selection {
    pub inline: Vec<Text>,
    pub blocked: BTreeSet<Text>,
    pub metadata: SourceMetadata,
}

impl Selection {
    pub(super) fn register_utilities(
        &mut self,
        params: &Text,
        base: &Text,
        source_base: Option<&Text>,
        limits: &Limits,
    ) -> Result<(), CompileError> {
        for option in value::segment_text(params, 32, limits.ast_depth)? {
            if option.starts_with("source(") {
                let path = &option.units()[7..];
                let path = Text::from_units(path[..path.len().saturating_sub(1)].to_vec());
                let path = value::scalar_text(&path)?;
                if path == "none" {
                    self.metadata.root = SourceRoot::None;
                    continue;
                }
                let quote = path.chars().next();
                if !matches!(quote, Some('\'' | '"')) || path.chars().last() != quote {
                    return Err(syntax("`source(…)` paths must be quoted."));
                }
                self.metadata.root = SourceRoot::Path {
                    base: value::scalar_text(source_base.unwrap_or(base))?,
                    pattern: path
                        .get(1..path.len().saturating_sub(1))
                        .unwrap_or("")
                        .into(),
                };
            }
        }
        Ok(())
    }
    pub(super) fn register_source(
        &mut self,
        params: &Text,
        has_body: bool,
        nested: bool,
        base: &Text,
        limits: &Limits,
    ) -> Result<(), CompileError> {
        if has_body {
            return Err(syntax("`@source` cannot have a body."));
        }
        if nested {
            return Err(syntax("`@source` cannot be nested."));
        }
        let (excluded, params) = params
            .strip_prefix("not ")
            .map_or_else(|| (false, params.clone()), |params| (true, params));
        let inline = params.starts_with("inline(");
        let path = if inline {
            Text::from_units(
                params
                    .units()
                    .get(7..params.units().len().saturating_sub(1))
                    .unwrap_or(&[])
                    .to_vec(),
            )
        } else {
            params
        };
        let units = path.units();
        if !matches!(units.first(), Some(39 | 34)) || units.last() != units.first() {
            return Err(syntax("`@source` paths must be quoted."));
        }
        let unquoted = Text::from_units(
            units
                .get(1..units.len().saturating_sub(1))
                .unwrap_or(&[])
                .to_vec(),
        );
        if inline {
            for pattern in value::segment_text(&unquoted, 32, limits.ast_depth)? {
                let expanded = expand(&pattern, limits, 0)?;
                if excluded {
                    self.blocked.extend(expanded);
                } else {
                    self.inline.extend(expanded);
                }
                super::check_limit(
                    self.inline.len().saturating_add(self.blocked.len()),
                    limits.candidate_count,
                    "candidate_count",
                )?;
            }
        } else {
            self.metadata.sources.push(SourcePattern {
                base: value::scalar_text(base)?,
                pattern: value::scalar_text(&unquoted)?,
                negated: excluded,
            });
        }
        super::check_limit(
            self.inline
                .iter()
                .chain(&self.blocked)
                .map(Text::len)
                .fold(0usize, usize::saturating_add),
            limits.queued_bytes,
            "queued_bytes",
        )?;
        Ok(())
    }
    pub fn owned_bytes(&self) -> usize {
        metadata_owned_bytes(&self.metadata).saturating_add(
            self.inline
                .capacity()
                .saturating_mul(std::mem::size_of::<Text>())
                .saturating_add(
                    self.inline
                        .iter()
                        .map(Text::owned_bytes)
                        .fold(0usize, usize::saturating_add),
                )
                .saturating_add(
                    self.blocked
                        .iter()
                        .map(Text::owned_bytes)
                        .fold(0usize, usize::saturating_add),
                )
                .saturating_add(self.blocked.len().div_ceil(5).saturating_mul(
                    11 * std::mem::size_of::<Text>() + 16 * std::mem::size_of::<usize>(),
                )),
        )
    }
}

fn metadata_owned_bytes(metadata: &SourceMetadata) -> usize {
    let root = match &metadata.root {
        SourceRoot::Path { base, pattern } => base.capacity().saturating_add(pattern.capacity()),
        _ => 0,
    };
    metadata
        .sources
        .capacity()
        .saturating_mul(std::mem::size_of::<SourcePattern>())
        .saturating_add(
            metadata
                .sources
                .iter()
                .map(|source| {
                    source
                        .base
                        .capacity()
                        .saturating_add(source.pattern.capacity())
                })
                .fold(0usize, usize::saturating_add),
        )
        .saturating_add(root)
}

fn syntax(message: impl Into<Text>) -> CompileError {
    CompileError::Syntax {
        offset: 0,
        message: message.into(),
    }
}

pub(super) fn expand(
    pattern: &Text,
    limits: &Limits,
    depth: usize,
) -> Result<Vec<Text>, CompileError> {
    super::check_limit(depth, limits.ast_depth, "ast_depth")?;
    let Some(start) = pattern.find("{") else {
        return Ok(vec![pattern.clone()]);
    };
    let rest = &pattern.units()[start..];
    let mut nesting = 0;
    let mut end = rest.iter().rposition(|unit| *unit == 125);
    for (index, unit) in rest.iter().enumerate() {
        if *unit == 123 {
            nesting += 1;
        } else if *unit == 125 {
            nesting -= 1;
            if nesting == 0 {
                end = Some(index);
                break;
            }
        }
    }
    let end = end.ok_or_else(|| {
        syntax(value::concat_text(&[
            &"The pattern `".into(),
            pattern,
            &"` is not balanced.".into(),
        ]))
    })?;
    let inside = Text::from_units(rest[1..end].to_vec());
    let mut range = Vec::new();
    let mut remaining = inside.clone();
    while let Some((left, right)) = remaining.split_once("..") {
        range.push(left);
        remaining = right;
    }
    range.push(remaining);
    let sequence = matches!(range.len(), 2 | 3)
        && range.iter().all(|part| {
            let units = part.units().strip_prefix(&[45]).unwrap_or(part.units());
            !units.is_empty() && units.iter().all(|unit| (48..=57).contains(unit))
        });
    let parts = if sequence {
        // Only proven ASCII decimal ranges cross into the JS-number implementation.
        let numbers = range
            .iter()
            .map(|part| {
                part.units()
                    .iter()
                    .map(|unit| char::from(*unit as u8))
                    .collect::<String>()
                    .parse::<f64>()
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| CompileError::Limit {
                resource: "candidate_count".into(),
                limit: limits.candidate_count,
            })?;
        let (start, end) = (numbers[0], numbers[1]);
        let step = numbers.get(2).copied().unwrap_or(1.0).abs();
        if step == 0.0 {
            return Err(syntax("Step cannot be zero in sequence expansion."));
        }
        let increasing = start < end;
        let step = if increasing { step } else { -step };
        let mut parts = Vec::new();
        let mut number = start;
        let mut bytes = 0usize;
        while if increasing {
            number <= end
        } else {
            number >= end
        } {
            super::check_limit(
                parts.len().saturating_add(1),
                limits.candidate_count,
                "candidate_count",
            )?;
            let part = Text::from(super::colors::js_string(number));
            bytes = bytes.saturating_add(part.len());
            super::check_limit(bytes, limits.queued_bytes, "queued_bytes")?;
            parts.push(part);
            let next = number + step;
            if next == number {
                return Err(CompileError::Limit {
                    resource: "candidate_count".into(),
                    limit: limits.candidate_count,
                });
            }
            number = next;
        }
        parts
    } else {
        value::segment_text(&inside, 44, limits.ast_depth)?
    };
    let mut expanded = Vec::new();
    for part in parts {
        let next = expand(&part, limits, depth + 1)?;
        super::check_limit(
            expanded.len().saturating_add(next.len()),
            limits.candidate_count,
            "candidate_count",
        )?;
        expanded.extend(next);
    }
    let tail = expand(
        &Text::from_units(rest[end + 1..].to_vec()),
        limits,
        depth + 1,
    )?;
    super::check_limit(
        expanded.len().saturating_mul(tail.len()),
        limits.candidate_count,
        "candidate_count",
    )?;
    let mut output = Vec::new();
    let mut bytes = 0usize;
    let prefix = Text::from_units(pattern.units()[..start].to_vec());
    for tail in tail {
        for part in &expanded {
            bytes = bytes
                .saturating_add(prefix.len())
                .saturating_add(part.len())
                .saturating_add(tail.len());
            super::check_limit(bytes, limits.queued_bytes, "queued_bytes")?;
            output.push(value::concat_text(&[&prefix, part, &tail]));
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::super::{CompileError, Compiler, CompilerInput, Limits, SourceAsset};

    fn limits() -> Limits {
        Limits {
            input_bytes: 262144,
            asset_bytes: 262144,
            candidate_count: 8192,
            ast_depth: 64,
            css_bytes: 2097152,
            queued_bytes: 2097152,
            cache_bytes: 8388608,
        }
    }

    #[test]
    fn inline_braces_preserve_distinct_units_and_number_expansion_order() {
        use crate::text::Text;
        let alternatives = [
            Text::from_units(vec![0xd800]),
            Text::from("\u{fffd}"),
            Text::from("\\ud800"),
        ];
        let pattern = super::value::concat_text(&[
            &"a{".into(),
            &alternatives[0],
            &",".into(),
            &alternatives[1],
            &",".into(),
            &alternatives[2],
            &"}{1..2}".into(),
        ]);
        let mut expected = Vec::new();
        for number in ["1", "2"] {
            for alternative in &alternatives {
                expected.push(super::value::concat_text(&[
                    &"a".into(),
                    alternative,
                    &number.into(),
                ]));
            }
        }
        assert_eq!(super::expand(&pattern, &limits(), 0).unwrap(), expected);
        let pattern = super::value::concat_text(&[&"a{".into(), &alternatives[0]]);
        let error = super::expand(&pattern, &limits(), 0).unwrap_err();
        let CompileError::Syntax { message, .. } = error else {
            panic!("expected imbalance error")
        };
        assert_eq!(
            message,
            super::value::concat_text(&[
                &"The pattern `".into(),
                &pattern,
                &"` is not balanced.".into()
            ])
        );
    }

    #[test]
    fn inline_sources_and_exclusions_do_not_cross_path_scalar_boundary() {
        use crate::{compiler::ast::AstNode, text::Text};
        let opaque = Text::from_units(vec![0xd800, 0xfffd]);
        let candidate = super::value::concat_text(&[&"probe-".into(), &opaque]);
        let nodes = vec![AstNode::Context {
            origin: None,
            base: opaque.clone(),
            source_base: Some(opaque),
            reference: false,
            nodes: vec![
                AstNode::AtRule {
                    origin: None,
                    name: "@source".into(),
                    params: super::value::concat_text(&[
                        &"inline(\"".into(),
                        &candidate,
                        &"\")".into(),
                    ]),
                    nodes: None,
                    source: None,
                    destination: None,
                },
                AstNode::AtRule {
                    origin: None,
                    name: "@source".into(),
                    params: super::value::concat_text(&[
                        &"not inline(\"".into(),
                        &candidate,
                        &"\")".into(),
                    ]),
                    nodes: None,
                    source: None,
                    destination: None,
                },
            ],
        }];
        let initial =
            super::super::public_api::register_native(&nodes, "input.css", &limits()).unwrap();
        let selection = initial.sources;
        assert_eq!(selection.inline, vec![candidate.clone()]);
        assert_eq!(
            selection.blocked.into_iter().collect::<Vec<_>>(),
            vec![candidate]
        );
        assert!(selection.metadata.sources.is_empty());
    }

    #[test]
    fn source_directives_match_official_css_errors_and_incremental_builds() {
        let fixtures: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/source-directives-fixtures.json"
        ))
        .unwrap();
        for case in fixtures["cases"].as_array().unwrap() {
            let id = case["id"].as_str().unwrap();
            let assets: Vec<SourceAsset> = case.get("assets").map_or_else(Vec::new, |assets| {
                serde_json::from_value(assets.clone()).unwrap()
            });
            let result = Compiler::new(
                CompilerInput {
                    css: case["input"].as_str().unwrap().into(),
                    entry: case["entry"].as_str().unwrap_or("/app/input.css").into(),
                },
                assets,
                limits(),
            );
            if let Some(expected) = case["expectedError"].as_str() {
                assert!(
                    matches!(result, Err(CompileError::Syntax { message, .. }) if message == expected),
                    "{id}: expected {expected}"
                );
                continue;
            }
            let mut compiler = result.unwrap_or_else(|error| panic!("{id}: {error:?}"));
            assert_eq!(
                serde_json::to_value(compiler.source_metadata()).unwrap(),
                serde_json::json!({
                    "sources": case["expectedSources"], "root": case["expectedRoot"],
                }),
                "{id}: source metadata"
            );
            if let Some(batches) = case["batches"].as_array() {
                for (batch, expected) in batches
                    .iter()
                    .zip(case["expectedBuilds"].as_array().unwrap())
                {
                    let candidates = serde_json::from_value::<Vec<String>>(batch.clone()).unwrap();
                    let prepared = compiler
                        .prepare_add(&candidates)
                        .unwrap_or_else(|error| panic!("{id}: {error:?}"));
                    assert_eq!(
                        prepared.report.css,
                        expected.as_str().unwrap(),
                        "{id}: batch {batch}"
                    );
                    compiler.commit(prepared).unwrap();
                }
            } else {
                let candidates =
                    serde_json::from_value::<Vec<String>>(case["candidates"].clone()).unwrap();
                let prepared = compiler
                    .prepare_add(&candidates)
                    .unwrap_or_else(|error| panic!("{id}: {error:?}"));
                assert_eq!(
                    prepared.report.css,
                    case["expectedCss"].as_str().unwrap(),
                    "{id}"
                );
                assert!(prepared.report.unsupported.is_empty(), "{id}");
            }
        }
    }

    #[test]
    fn source_expansion_obeys_candidate_byte_and_depth_limits() {
        for (pattern, resource, limit) in [
            ("w-[{1..100}px]", "candidate_count", 4),
            ("{abcdefgh,ijklmnop}", "queued_bytes", 4),
            ("{{{flex,block}}}", "ast_depth", 1),
        ] {
            let mut limits = limits();
            match resource {
                "candidate_count" => limits.candidate_count = limit,
                "queued_bytes" => limits.queued_bytes = limit,
                _ => limits.ast_depth = limit,
            }
            assert!(
                matches!(super::expand(&crate::text::Text::from(pattern), &limits, 0), Err(CompileError::Limit { resource: actual, limit: actual_limit }) if actual == resource && actual_limit == limit),
                "{pattern}"
            );
        }
    }

    #[test]
    fn nonprogressing_javascript_number_ranges_fail_with_a_limit() {
        assert!(
            matches!(super::expand(&crate::text::Text::from("{18014398509481984..18014398509481984}"), &limits(), 0), Err(CompileError::Limit { resource, .. }) if resource == "candidate_count")
        );
    }

    #[test]
    fn inline_source_owned_storage_is_charged_to_the_cache_limit() {
        let mut limits = limits();
        limits.cache_bytes = 1;
        let result = Compiler::new(
            CompilerInput {
                css: "@source inline(\"flex\"); @tailwind utilities;".into(),
                entry: "/app/input.css".into(),
            },
            Vec::new(),
            limits,
        );
        assert!(
            matches!(result, Err(CompileError::Limit { resource, limit: 1 }) if resource == "cache_bytes")
        );
    }

    #[test]
    fn path_metadata_storage_is_charged_to_the_cache_limit() {
        for css in [
            "@source \"./src/**/*.rs\";",
            "@tailwind utilities source(\"./src\");",
        ] {
            let mut limits = limits();
            limits.cache_bytes = 1;
            assert!(matches!(Compiler::new(CompilerInput {
                css: css.into(), entry: "/app/input.css".into(),
            }, Vec::new(), limits), Err(CompileError::Limit { resource, limit: 1 }) if resource == "cache_bytes"));
        }
    }
}
