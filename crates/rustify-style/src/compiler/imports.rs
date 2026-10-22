//! Virtual stylesheet resolution; no filesystem or network access in the core.

use super::{
    ast::AstNode,
    parser,
    value::{self, ValueNode},
    CompileError, Limits,
};
use crate::text::Text;
mod bundle;
pub(crate) mod schedule;

struct Import {
    uri: String,
    layer: Option<Text>,
    media: Option<Text>,
    supports: Option<Text>,
}

fn parameters(input: &Text, depth: usize) -> Result<Option<Import>, CompileError> {
    let nodes = value::parse_text(input, depth)?;
    let mut import = Import {
        uri: String::new(),
        layer: None,
        media: None,
        supports: None,
    };
    for (index, node) in nodes.iter().enumerate() {
        if matches!(node, ValueNode::Separator { .. }) {
            continue;
        }
        if let ValueNode::Word { value } = node {
            if import.uri.is_empty() {
                if !value.starts_with("\"") && !value.starts_with("'") {
                    return Ok(None);
                }
                let value = value::scalar_text(value)?;
                import.uri = value[1..value.len() - 1].into();
                continue;
            }
        }
        if matches!(node, ValueNode::Function { name, .. } if name.eq_ignore_ascii_case("url")) {
            return Ok(None);
        }
        if import.uri.is_empty() {
            return Ok(None);
        }
        let (name, children) = match node {
            ValueNode::Word { value } => (value, None),
            ValueNode::Function { name, nodes } => (name, Some(nodes)),
            _ => unreachable!(),
        };
        if name.eq_ignore_ascii_case("layer") {
            if import.layer.as_ref().is_some_and(|layer| !layer.is_empty()) {
                return Ok(None);
            }
            if import
                .supports
                .as_ref()
                .is_some_and(|supports| !supports.is_empty())
            {
                return Err(CompileError::Syntax { offset: 0, message: "`layer(…)` in an `@import` should come before any other functions or conditions".into() });
            }
            import.layer =
                Some(children.map_or_else(Text::default, |nodes| value::to_css_text(nodes)));
            continue;
        }
        if name.eq_ignore_ascii_case("supports") && children.is_some() {
            if import
                .supports
                .as_ref()
                .is_some_and(|supports| !supports.is_empty())
            {
                return Ok(None);
            }
            import.supports = children.map(|nodes| value::to_css_text(nodes));
            continue;
        }
        import.media = Some(value::to_css_text(&nodes[index..]));
        break;
    }
    Ok((!import.uri.is_empty()).then_some(import))
}

pub(super) fn normalize(path: &str) -> String {
    let absolute = path.starts_with('/');
    let mut parts = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." if parts.last().is_some_and(|part| *part != "..") => {
                parts.pop();
            }
            ".." if absolute => {}
            _ => parts.push(part),
        }
    }
    let path = parts.join("/");
    if absolute {
        format!("/{path}")
    } else if path.is_empty() {
        ".".into()
    } else {
        path
    }
}
pub(super) fn directory(path: &str) -> &str {
    path.rsplit_once('/')
        .map_or(".", |(base, _)| if base.is_empty() { "/" } else { base })
}
/// Wrappers built from `@import` conditions map back to the import itself.
fn wrapper(
    name: &str,
    params: Text,
    nodes: Vec<AstNode>,
    source: Option<super::public_api::SourceLocation>,
) -> Vec<AstNode> {
    vec![AstNode::AtRule {
        origin: None,
        source,
        destination: None,
        name: name.into(),
        params,
        nodes: Some(nodes),
    }]
}

pub(super) struct Expansion {
    pub has_import: bool,
    pub calls: Vec<super::public_api::StylesheetLoad>,
}

pub(super) trait PublicSetup {
    fn root(&self) -> super::public_api::PublicAst;
    fn parsed(
        &mut self,
        nodes: &mut [AstNode],
    ) -> Result<super::public_api::PublicAst, CompileError>;
    fn begin(
        &mut self,
        parent: &super::public_api::PublicAst,
        index: usize,
    ) -> Result<
        (
            super::public_api::PublicValue,
            super::public_api::InputMutation,
        ),
        CompileError,
    >;
    fn scheduled(
        &mut self,
        program: schedule::Program,
        prefix: Vec<super::public_api::InputMutation>,
    );
    fn initial(&mut self, limits: &Limits) -> Result<super::registration::Initial, CompileError>;
    fn compatibility(&mut self);
    fn failed_apply(
        &mut self,
        replacements: Vec<super::apply::Replacement>,
    ) -> Result<(), CompileError>;
    fn complete(
        &mut self,
        parent: &super::public_api::PublicAst,
        index: usize,
        original: &AstNode,
        imported: &mut AstNode,
        loaded: &super::public_api::PublicAst,
        context: super::public_api::PublicValue,
    ) -> Result<super::public_api::InputMutation, CompileError>;
}

pub(super) fn expand_with_base(
    nodes: &mut Vec<AstNode>,
    base: &str,
    assets: &[super::public_api::Stylesheet],
    limits: &Limits,
    track: bool,
    public: Option<&mut dyn PublicSetup>,
) -> Result<Expansion, CompileError> {
    let mut public = public;
    let expanded = if super::ast::contains_at_rule(nodes, "@import")
        || super::ast::contains_at_rule(nodes, "@reference")
    {
        bundle::expand(
            nodes,
            base,
            assets,
            limits,
            track,
            public
                .as_mut()
                .map(|public| &mut **public as &mut dyn PublicSetup),
        )?
    } else {
        Expansion {
            has_import: false,
            calls: vec![],
        }
    };
    Ok(expanded)
}

#[cfg(test)]
mod tests {
    use super::super::{Compiler, CompilerInput, Limits, SourceAsset};

    fn limits() -> Limits {
        Limits {
            input_bytes: 65536,
            asset_bytes: 65536,
            candidate_count: 1024,
            ast_depth: 32,
            css_bytes: 262144,
            queued_bytes: 65536,
            cache_bytes: 1048576,
        }
    }

    fn compile(
        input: &str,
        assets: Vec<SourceAsset>,
        limits: Limits,
    ) -> Result<Compiler, super::CompileError> {
        Compiler::new(
            CompilerInput {
                css: input.into(),
                entry: "/app/input.css".into(),
            },
            assets,
            limits,
        )
    }

    #[test]
    fn missing_and_normalized_duplicate_assets_are_explicit_errors() {
        assert!(
            matches!(compile("@import \"missing.css\";", vec![], limits()), Err(super::CompileError::MissingAsset { id, base }) if id == "missing.css" && base == "/app")
        );
        let assets = vec![
            SourceAsset {
                id: "/app/theme.css".into(),
                content: String::new().into(),
            },
            SourceAsset {
                id: "/app/./theme.css".into(),
                content: String::new().into(),
            },
        ];
        assert!(matches!(
            compile("", assets, limits()),
            Err(super::CompileError::DuplicateAsset { .. })
        ));
    }

    #[test]
    fn recursive_imports_obey_ast_depth_and_upstream_recursion_ceiling() {
        let assets = vec![SourceAsset {
            id: "/app/a.css".into(),
            content: "@import \"a.css\";".into(),
        }];
        assert!(
            matches!(compile("@import \"a.css\";", assets.clone(), limits()), Err(super::CompileError::Limit { resource, limit: 32 }) if resource == "ast_depth")
        );
        let mut limits = limits();
        limits.ast_depth = 128;
        assert!(
            matches!(compile("@import \"a.css\";", assets, limits), Err(super::CompileError::Syntax { message, .. }) if message == "Exceeded maximum recursion depth while resolving `a.css` in `/app`)")
        );
    }

    #[test]
    fn repeated_expansion_is_bounded_before_parsing_copies() {
        let assets = vec![SourceAsset {
            id: "/app/a.css".into(),
            content: format!("/*{}*/", "x".repeat(56)).into(),
        }];
        let mut limits = limits();
        limits.input_bytes = 128;
        limits.asset_bytes = 100;
        assert!(
            matches!(compile(&"@import \"a.css\";".repeat(4), assets, limits), Err(super::CompileError::Limit { resource, limit: 228 }) if resource == "expanded_bytes")
        );
    }

    #[test]
    fn layer_after_supports_has_the_upstream_error() {
        assert!(
            matches!(compile("@import \"a.css\" supports(display:grid) layer(x);", vec![], limits()), Err(super::CompileError::Syntax { message, .. }) if message == "`layer(…)` in an `@import` should come before any other functions or conditions")
        );
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn virtual_imports_match_official_css() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/imports-fixtures.json"
        ))
        .unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let assets: Vec<SourceAsset> = serde_json::from_value(case["assets"].clone()).unwrap();
            let compiler = Compiler::new(
                CompilerInput {
                    css: case["input"].as_str().unwrap().into(),
                    entry: case["entry"].as_str().unwrap().into(),
                },
                assets,
                limits(),
            );
            if let Some(message) = case["expectedError"].as_str() {
                assert!(
                    matches!(&compiler, Err(super::CompileError::Syntax { message: actual, .. }) if actual == message),
                    "{}: {:?}",
                    case["id"],
                    compiler.err()
                );
                continue;
            }
            let mut compiler = compiler.unwrap_or_else(|error| panic!("{}: {error:?}", case["id"]));
            let candidates: Vec<String> =
                serde_json::from_value(case["candidates"].clone()).unwrap();
            let prepared = compiler.prepare_add(&candidates).unwrap();
            assert_eq!(
                prepared.report.css,
                case["expectedCss"].as_str().unwrap(),
                "{}",
                case["id"]
            );
            assert!(prepared.report.unsupported.is_empty(), "{}", case["id"]);
        }
    }
}
