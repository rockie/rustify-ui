//! Initial directives operate on the shared raw graph before typed lowering.

use super::{
    ast::AstNode,
    origin::current::{Action, Allowance, Current, Visit},
    public_api::{
        design_system::public_json, InputMutation, PublicAst, PublicValue, PublicValue as Value,
    },
    sources::Selection,
    CompileError, Limits,
};
use crate::text::Text;
use std::{collections::BTreeMap, sync::Arc};

pub(super) struct Initial {
    pub nodes: Vec<AstNode>,
    pub sources: Selection,
    pub important: bool,
    pub utilities: bool,
    pub variants: Vec<(super::custom_variants::Definition, Vec<AstNode>)>,
    pub theme: super::theme::Theme,
    pub definitions: Vec<super::custom_utilities::RegisteredDefinition>,
    pub warnings: Vec<Text>,
    pub has_authored_variants: bool,
    pub has_functions: bool,
}

pub(super) struct Registered {
    pub sources: Selection,
    pub important: bool,
    pub utilities: Option<Value>,
    pub variants: Vec<(super::custom_variants::Definition, Value)>,
    pub theme: super::theme::Theme,
    pub theme_rule: Option<Value>,
    pub keyframes: Vec<Value>,
    pub definitions: Vec<(String, bool, Value)>,
    pub authored_variants: Vec<Value>,
}

fn body(node: &Value) -> &PublicAst {
    match &node["nodes"] {
        Value::Array(nodes) => nodes,
        _ => unreachable!("input shape is checked before registration"),
    }
}
fn text<'a>(node: &'a Value, field: &str) -> &'a Text {
    node[field].as_text().expect("validated AST field")
}
fn assign(
    cursor: &mut Visit<'_>,
    node: &Value,
    field: &str,
    value: &Value,
    effects: &mut Vec<InputMutation>,
) -> Result<(), CompileError> {
    let captured = cursor.assign(
        node,
        field,
        value,
        4 * std::mem::size_of::<InputMutation>() + field.len() * 2,
    )?;
    effects.push(InputMutation::Set {
        target: node.clone(),
        field: field.into(),
        value: captured.anchor,
    });
    Ok(())
}
fn replace(
    cursor: &mut Visit<'_>,
    items: &[Value],
    effects: &mut Vec<InputMutation>,
) -> Result<(), CompileError> {
    let target = Value::Array(cursor.array().clone());
    let start = cursor.index();
    if let Some(captured) = cursor.replace_with(items, 4 * std::mem::size_of::<InputMutation>())? {
        if items.len() == 1 {
            effects.push(InputMutation::Set {
                target,
                field: start.to_string().into(),
                value: captured.anchor[0].clone(),
            });
        } else {
            effects.push(InputMutation::Splice {
                target,
                start,
                delete_count: 1,
                values: captured.anchor,
            });
        }
    }
    Ok(())
}

fn media(
    node: &Value,
    cursor: &mut Visit<'_>,
    base: &Text,
    limits: &Limits,
    important: &mut bool,
    effects: &mut Vec<InputMutation>,
) -> Result<(), CompileError> {
    let params = super::value::segment_text(text(node, "params"), 32, limits.ast_depth)?;
    let mut unknown = Text::default();
    let mut unknown_count = 0;
    for param in &params {
        if param.starts_with("source(") {
            let nodes = body(cursor.value(node)).clone();
            cursor.walk(&nodes, limits.ast_depth, &mut |child, inner| {
                if child["kind"] == "at-rule" && child["name"] == "@tailwind" && child["params"] == "utilities" {
                    let mut value = Text::from("utilities ");
                    value.push_text(param);
                    assign(inner, child, "params", &Value::from(value), effects)?;
                    let wrapper = public_json!({"kind":"context","context":public_json!({"sourceBase":base}),"nodes":public_json!([inner.value(child)])});
                    replace(inner, &[wrapper], effects)?;
                    return Ok(Action::Stop);
                }
                Ok(Action::Continue)
            })?;
        } else if param.starts_with("theme(") || param.starts_with("prefix(") {
            let options = if param.starts_with("theme(") {
                Text::from_units(
                    param
                        .units()
                        .get(6..param.units().len().saturating_sub(1))
                        .unwrap_or_default()
                        .to_vec(),
                )
            } else {
                param.clone()
            };
            let reference = param.starts_with("theme(") && options.contains("reference");
            let nodes = body(cursor.value(node)).clone();
            cursor.walk(&nodes, limits.ast_depth, &mut |child, inner| {
                if child["kind"] != "at-rule" {
                    if reference {
                        return Err(CompileError::Syntax { offset: 0, message: "Files imported with `@import \"…\" theme(reference)` must only contain `@theme` blocks.\nUse `@reference \"…\";` instead.".into() });
                    }
                    return Ok(Action::Continue);
                }
                if child["name"] == "@theme" {
                    let mut value = text(child, "params").clone();
                    value.push_str(" ");
                    value.push_text(&options);
                    assign(inner, child, "params", &Value::from(value), effects)?;
                    return Ok(Action::Skip);
                }
                Ok(Action::Continue)
            })?;
        } else if param == "important" {
            *important = true;
        } else if param == "reference" {
            let nodes = body(cursor.value(node)).clone();
            let wrapper = public_json!({"kind":"context","context":public_json!({"reference":true}),"nodes":Value::Array(nodes)});
            assign(cursor, node, "nodes", &public_json!([wrapper]), effects)?;
        } else {
            if unknown_count > 0 {
                unknown.push_str(" ");
            }
            unknown.push_text(param);
            unknown_count += 1;
        }
    }
    if unknown_count > 0 {
        assign(cursor, node, "params", &Value::from(unknown), effects)?;
    } else if !params.is_empty() {
        let nodes = body(cursor.value(node)).clone();
        // Replacement reads the live array, including earlier source wrappers.
        let items = cursor.values(&nodes).to_vec();
        replace(cursor, &items, effects)?;
    }
    Ok(())
}

pub(super) fn run(
    root: &PublicAst,
    current: &mut Current,
    allowance: &mut Allowance,
    base: &Text,
    limits: &Limits,
    effects: &mut Vec<InputMutation>,
) -> Result<Registered, CompileError> {
    let mut sources = Selection::default();
    let mut important = false;
    let mut utilities = None;
    let mut variants = Vec::new();
    let mut theme = super::theme::Theme::default();
    let mut theme_rule = None;
    let mut keyframes = Vec::new();
    let mut definitions = Vec::new();
    let mut authored_variants = Vec::new();
    current.walk(root, allowance, limits.ast_depth, &mut |node, cursor| {
        if node["kind"] != "at-rule" {
            return Ok(Action::Continue);
        }
        let name = text(node, "name");
        let params = text(node, "params");
        let context = cursor.context();
        let base = context
            .get(&Text::from("base"))
            .and_then(Value::as_text)
            .unwrap_or(base)
            .clone();
        let nested = cursor.path().len() > 1;
        if name == "@tailwind" && params.starts_with("utilities") {
            let reference = context
                .get(&Text::from("reference"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
            if utilities.is_some() || reference {
                replace(cursor, &[], effects)?;
                return Ok(Action::Continue);
            }
            sources.register_utilities(
                params,
                &base,
                context
                    .get(&Text::from("sourceBase"))
                    .and_then(Value::as_text),
                limits,
            )?;
            utilities = Some(node.clone());
        }
        if name == "@utility" {
            let (name, functional) = super::custom_utilities::validate_definition(
                params,
                !cursor.values(body(node)).is_empty(),
                nested,
            )?;
            definitions.push((name, functional, node.clone()));
        }
        if name == "@source" {
            sources.register_source(
                params,
                !cursor.values(body(node)).is_empty(),
                nested,
                &base,
                limits,
            )?;
            replace(cursor, &[], effects)?;
            return Ok(Action::Continue);
        }
        if name == "@theme" {
            let reference = context.get(&Text::from("reference")).and_then(Value::as_bool).unwrap_or(false);
            let options = theme.register_options(params, reference, limits.ast_depth)?;
            let children = body(node).clone();
            cursor.walk(&children, limits.ast_depth, &mut |child, inner| {
                if child["kind"] == "at-rule" && child["name"] == "@keyframes" {
                    if !keyframes.iter().any(|previous: &Value| same_object(previous, child)) {
                        keyframes.push(child.clone());
                    }
                    return Ok(Action::Skip);
                }
                if child["kind"] == "comment" { return Ok(Action::Continue); }
                if child["kind"] == "declaration" && text(child, "property").starts_with("--") {
                    let value = child.get("value").and_then(Value::as_text).cloned().unwrap_or_default();
                    theme.register_declaration(text(child, "property"), &value,
                        child.get("src").cloned().map(super::public_api::caller_source),
                        &options, limits.ast_depth)?;
                    return Ok(Action::Continue);
                }
                Err(super::public_api::invalid_theme_child(name, params, child, inner, limits.ast_depth)?)
            })?;
            if theme.mark_anchor() {
                let replacement = public_json!({"kind":"rule","selector":":root, :host","nodes":public_json!([]),"src":node.get("src").cloned().unwrap_or_else(super::public_api::undefined)});
                replace(cursor, std::slice::from_ref(&replacement), effects)?;
                theme_rule = Some(replacement);
            } else {
                replace(cursor, &[], effects)?;
            }
            return Ok(Action::Skip);
        }
        let mut legacy = false;
        if name == "@variant" && !nested {
            let children = body(node).clone();
            let mut slot = cursor.values(&children).is_empty();
            cursor.walk(&children, limits.ast_depth, &mut |child, _| {
                if child["kind"] == "at-rule" && child["name"] == "@slot" {
                    slot = true;
                    return Ok(Action::Stop);
                }
                Ok(Action::Continue)
            })?;
            if slot {
                assign(cursor, node, "name", &Value::from("@custom-variant"), effects)?;
                legacy = true;
            }
        }
        if name == "@custom-variant" || legacy {
            if nested {
                return Err(CompileError::Syntax {
                    offset: 0,
                    message: "`@custom-variant` cannot be nested.".into(),
                });
            }
            // The upstream removes a definition before validating its header.
            // A later failure must retain that completed array operation.
            replace(cursor, &[], effects)?;
            let definition = super::custom_variants::prepare(
                params,
                !cursor.values(body(cursor.value(node))).is_empty(),
                limits,
            )?;
            variants.push((definition, node.clone()));
            return Ok(Action::Continue);
        }
        if name == "@variant" {
            authored_variants.push(node.clone());
        }
        if name == "@media" {
            media(node, cursor, &base, limits, &mut important, effects)?;
        }
        Ok(Action::Continue)
    })?;
    Ok(Registered {
        sources,
        important,
        utilities,
        variants,
        theme,
        theme_rule,
        keyframes,
        definitions,
        authored_variants,
    })
}

/// Validates `@plugin` and `@config` in the order upstream collects them, then
/// reports the JavaScript module the compiler will not load.
pub(super) fn reject_javascript_configuration(
    root: &PublicAst,
    current: &mut Current,
    allowance: &mut Allowance,
    limits: &Limits,
) -> Result<(), CompileError> {
    let syntax = |message: Text| CompileError::Syntax { offset: 0, message };
    let mut found = None;
    current.walk(root, allowance, limits.ast_depth, &mut |node, cursor| {
        if node["kind"] != "at-rule" {
            return Ok(Action::Continue);
        }
        let name = text(node, "name");
        // The visited node is the last path entry; anything before it is a parent.
        let nested = cursor.path().len() > 1;
        let children = match node.get("nodes") {
            Some(Value::Array(nodes)) => cursor.values(nodes).to_vec(),
            _ => Vec::new(),
        };
        if name == "@plugin" {
            if nested {
                return Err(syntax("`@plugin` cannot be nested.".into()));
            }
            if text(node, "params").units().len() <= 2 {
                return Err(syntax("`@plugin` must have a path.".into()));
            }
            for child in &children {
                let child = cursor.value(child);
                if child["kind"] != "declaration" {
                    let mut message = Text::from("Unexpected `@plugin` option:\n\n");
                    message.push_text(&super::ast::to_css(&[printable(child, cursor)?]));
                    message.push_str("\n\n`@plugin` options must be a flat list of declarations.");
                    return Err(syntax(message));
                }
                let Some(value) = child.get("value").and_then(Value::as_text) else {
                    continue;
                };
                // Only a braced part fails; null, booleans, numbers and quoted
                // strings cannot start with `{`.
                for part in super::value::segment_text(value, u16::from(b','), limits.ast_depth)? {
                    let part = part.trim_js();
                    let units = part.units();
                    if units.first() == Some(&u16::from(b'{')) && units.last() == Some(&u16::from(b'}')) {
                        let mut message =
                            Text::from("Unexpected `@plugin` option: Value of declaration `");
                        message.push_text(
                            &super::ast::to_css(&[printable(child, cursor)?]).trim_js(),
                        );
                        message.push_str("` is not supported.\n\nUsing an object as a plugin option is currently only supported in JavaScript configuration files.");
                        return Err(syntax(message));
                    }
                }
            }
            found.get_or_insert_with(|| name.clone());
            return Ok(Action::Skip);
        }
        if name == "@config" {
            if !children.is_empty() {
                return Err(syntax("`@config` cannot have a body.".into()));
            }
            if nested {
                return Err(syntax("`@config` cannot be nested.".into()));
            }
            found.get_or_insert_with(|| name.clone());
            return Ok(Action::Skip);
        }
        Ok(Action::Continue)
    })?;
    match found {
        Some(feature) => Err(CompileError::UnsupportedFeature {
            feature: super::value::scalar_text(&feature)?,
        }),
        None => Ok(()),
    }
}

/// Typed copy of a raw node for error text, printed as upstream `toCss([node])`.
fn printable(node: &Value, cursor: &Visit<'_>) -> Result<AstNode, CompileError> {
    let children = |node: &Value| -> Result<Vec<AstNode>, CompileError> {
        match node.get("nodes") {
            Some(Value::Array(nodes)) => cursor
                .values(nodes)
                .iter()
                .map(|child| printable(cursor.value(child), cursor))
                .collect(),
            _ => Ok(Vec::new()),
        }
    };
    let field = |name: &str| {
        node.get(name)
            .and_then(Value::as_text)
            .cloned()
            .unwrap_or_default()
    };
    let kind = node["kind"].as_text().cloned().unwrap_or_default();
    Ok(if kind == "rule" {
        AstNode::Rule {
            origin: None,
            source: None,
            destination: None,
            selector: field("selector"),
            nodes: children(node)?,
        }
    } else if kind == "at-rule" {
        AstNode::AtRule {
            origin: None,
            source: None,
            destination: None,
            name: field("name"),
            params: field("params"),
            nodes: Some(children(node)?),
        }
    } else if kind == "declaration" {
        AstNode::Declaration {
            origin: None,
            source: None,
            destination: None,
            property: field("property"),
            value: field("value"),
            important: node.get("important") == Some(&Value::Bool(true)),
            identity: Arc::new(()),
        }
    } else if kind == "comment" {
        AstNode::Comment {
            origin: None,
            source: None,
            destination: None,
            value: field("value"),
        }
    } else {
        AstNode::AtRoot {
            origin: None,
            nodes: children(node)?,
        }
    })
}

pub(super) fn normalize_definitions(
    definitions: &[(String, bool, Value)],
    current: &mut Current,
    allowance: &mut Allowance,
    limits: &Limits,
    effects: &mut Vec<InputMutation>,
) -> Result<(Vec<Text>, Vec<super::custom_utilities::CompletionMetadata>), CompileError> {
    let mut warnings = Vec::new();
    let mut completions = Vec::with_capacity(definitions.len());
    for (_, functional, node) in definitions {
        let mut completion = super::custom_utilities::CompletionMetadata::default();
        if !functional {
            completions.push(completion);
            continue;
        }
        let nodes = body(current.value(node)).clone();
        current.walk(&nodes, allowance, limits.ast_depth, &mut |child, cursor| {
            if child["kind"] == "declaration" {
                if let Some(css) = child.get("value").and_then(Value::as_text) {
                    if let Some(value) = super::custom_utilities::normalize_value(
                        css,
                        limits.ast_depth,
                        &mut warnings,
                    )? {
                        assign(cursor, child, "value", &Value::from(value), effects)?;
                    }
                    completion
                        .include_value(text(cursor.value(child), "value"), limits.ast_depth)?;
                }
            }
            Ok(Action::Continue)
        })?;
        completions.push(completion);
    }
    Ok((warnings, completions))
}

pub(super) fn publish_theme(
    theme: &super::theme::Theme,
    rule: Option<&Value>,
    current: &mut Current,
    allowance: &mut Allowance,
    effects: &mut Vec<InputMutation>,
) -> Result<BTreeMap<Text, Value>, CompileError> {
    let mut declarations = BTreeMap::new();
    let Some(rule) = rule else {
        return Ok(declarations);
    };
    let mut nodes = Vec::new();
    for (name, token) in theme.ordered_tokens().filter(|(_, token)| !token.reference) {
        let property = super::candidate::escape_text(&theme.prefix_key_text(name));
        let source = token
            .source
            .as_ref()
            .and_then(|source| source.public.as_deref())
            .cloned()
            .unwrap_or_else(super::public_api::undefined);
        let node = public_json!({"kind":"declaration","property":property,"value":token.value,"important":false,"src":source});
        declarations.insert(property, node.clone());
        nodes.push(node);
    }
    let value = public_json!([
        public_json!({"kind":"context","context":public_json!({"theme":true}),"nodes":nodes})
    ]);
    let captured = current.assign(
        rule,
        "nodes",
        &value,
        allowance,
        4 * std::mem::size_of::<InputMutation>() + 10,
    )?;
    effects.push(InputMutation::Set {
        target: rule.clone(),
        field: "nodes".into(),
        value: captured.anchor,
    });
    Ok(declarations)
}

pub(super) fn same_object(left: &Value, right: &Value) -> bool {
    match (left.as_object(), right.as_object()) {
        (Some(left), Some(right)) => Arc::ptr_eq(left.identity(), right.identity()),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{public_api, Compiler, CompilerInput, SourceRoot};

    fn limits() -> Limits {
        Limits {
            input_bytes: 65536,
            asset_bytes: 65536,
            candidate_count: 1024,
            ast_depth: 32,
            css_bytes: 262144,
            queued_bytes: 262144,
            cache_bytes: 16000000,
        }
    }

    #[test]
    fn native_registration_lowers_substituted_functions_and_keyframes() {
        let nodes = super::super::parser::parse(
            "@theme { --spacing: .25rem; @keyframes spin { to { width: --spacing(2); } } } @utility measure-* { width: --spacing(--value(integer)); } @media (width >= --theme(--spacing)) { .probe { width: --spacing(3); } }",
            32,
        ).unwrap();
        let initial = public_api::register_native(&nodes, "", &limits()).unwrap();
        assert!(
            !super::super::functions::contains(&initial.nodes)
                && !super::super::functions::contains(&initial.theme.keyframes)
                && !super::super::functions::contains(&initial.definitions[0].nodes),
            "the lowered root, utility bodies and keyframes must share completed function values"
        );
        assert!(initial.has_functions);
        assert!(initial.definitions[0].completion.values.integer);
    }

    #[test]
    fn theme_publication_and_completed_functions_survive_a_later_theme_error() {
        let root = Arc::new(vec![
            public_json!({"kind":"at-rule","name":"@theme","params":"","nodes":public_json!([
                public_json!({"kind":"declaration","property":"--spacing","value":".25rem","important":false}),
                public_json!({"kind":"declaration","property":"--x","value":"--spacing(2)","important":false}),
                public_json!({"kind":"declaration","property":"--bad","value":"theme(--missing)","important":false})
            ])}),
        ]);
        let public_api::AstSetupOutcome::Failed(failure) =
            public_api::compile_ast_setup(root, Default::default(), vec![], limits()).unwrap()
        else {
            panic!("unresolved theme function must fail");
        };
        let published = failure
            .input_mutations
            .iter()
            .find_map(|effect| match effect {
                InputMutation::Set { field, value, .. } if field == "nodes" => Some(value),
                _ => None,
            })
            .expect("theme is published before function substitution");
        let declaration = &published[0]["nodes"][1];
        assert_eq!(declaration["value"], "--spacing(2)");
        assert!(failure.input_mutations.iter().any(|effect| matches!(effect,
            InputMutation::Set { target, field, value }
            if same_object(target, declaration) && field == "value" && value == "calc(var(--spacing) * 2)"
        )));
    }

    #[test]
    fn authored_function_error_precedes_a_later_theme_function_error() {
        let error = Compiler::new(
            CompilerInput {
                css: ".first { color: theme(--missing-authored); } @theme { --x: theme(--missing-theme); }".into(),
                entry: "entry.css".into(),
            },
            vec![], limits(),
        ).err().expect("both functions are unresolved");
        assert!(
            matches!(error, CompileError::Syntax { message, .. } if message.contains("--missing-authored"))
        );
    }

    #[test]
    fn hoisted_keyframe_functions_retain_the_completed_caller_assignment() {
        let first = public_json!({"kind":"declaration","property":"width","value":"--spacing(2)","important":false});
        let root = Arc::new(vec![
            public_json!({"kind":"at-rule","name":"@theme","params":"","nodes":public_json!([
                public_json!({"kind":"declaration","property":"--spacing","value":".25rem","important":false}),
                public_json!({"kind":"at-rule","name":"@keyframes","params":"spin","nodes":public_json!([
                    public_json!({"kind":"rule","selector":"to","nodes":public_json!([
                        first.clone(),
                        public_json!({"kind":"declaration","property":"color","value":"theme(--missing)","important":false})
                    ])})
                ])})
            ])}),
        ]);
        let public_api::AstSetupOutcome::Failed(failure) =
            public_api::compile_ast_setup(root, Default::default(), vec![], limits()).unwrap()
        else {
            panic!("unresolved keyframe function must fail");
        };
        assert!(failure.input_mutations.iter().any(|effect| matches!(effect,
            InputMutation::Set { target, field, value }
            if same_object(target, &first) && field == "value" && value == "calc(var(--spacing) * 2)"
        )));
    }

    #[test]
    fn deferred_variant_reads_the_shared_functional_utility_declaration() {
        let shared = public_json!({"kind":"declaration","property":"color","value":"--value(--color)","important":false});
        let variant = public_json!({"kind":"at-rule","name":"@custom-variant","params":"aliased","nodes":public_json!([
            public_json!({"kind":"rule","selector":"&:hover","nodes":public_json!([
                shared.clone(), public_json!({"kind":"at-rule","name":"@slot","params":"","nodes":public_json!([])})
            ])})
        ])});
        let root = Arc::new(vec![
            variant,
            public_json!({"kind":"at-rule","name":"@utility","params":"color-*","nodes":public_json!([shared])}),
            public_json!({"kind":"at-rule","name":"@tailwind","params":"utilities","nodes":public_json!([])}),
        ]);
        let mut api = public_api::compile_ast(root, Default::default(), vec![], limits()).unwrap();
        let output = api.build(&["aliased:block".into()]).unwrap();
        let output = serde_json::to_string(output.as_ref()).unwrap();
        assert!(output.contains("--value(--color-*)"), "{output}");
        assert!(!output.contains("--value(--color)"), "{output}");
    }

    #[test]
    fn compatibility_rejection_in_utility_body_precedes_normalization() {
        let root = Arc::new(vec![
            public_json!({"kind":"at-rule","name":"@utility","params":"color-*","nodes":public_json!([
                public_json!({"kind":"declaration","property":"color","value":"--value(--color)","important":false}),
                public_json!({"kind":"at-rule","name":"@config","params":"\"config.js\"","nodes":public_json!([])})
            ])}),
            public_json!({"kind":"at-rule","name":"@tailwind","params":"utilities","nodes":public_json!([])}),
        ]);
        let public_api::AstSetupOutcome::Failed(failure) =
            public_api::compile_ast_setup(root, Default::default(), vec![], limits()).unwrap()
        else {
            panic!("configuration must be rejected");
        };
        // Upstream validates placement before it would load the module.
        assert!(
            matches!(&failure.error, CompileError::Syntax { message, .. } if message == "`@config` cannot be nested.")
        );
        assert!(failure.input_mutations.is_empty());
        let native = Compiler::new(
            CompilerInput { css: "@utility color-* { color: --value(--color); @config \"config.js\"; } @tailwind utilities;".into(), entry: "entry.css".into() },
            vec![], limits(),
        );
        assert!(
            matches!(native, Err(CompileError::Syntax { message, .. }) if message == "`@config` cannot be nested.")
        );
    }

    #[test]
    fn native_and_public_share_media_sources_and_importance() {
        let input = Arc::new(vec![
            public_json!({"kind":"at-rule","name":"@media","params":"important source(\"src\")","nodes":public_json!([
                public_json!({"kind":"at-rule","name":"@tailwind","params":"utilities","nodes":public_json!([])})
            ])}),
        ]);
        let limits = limits();
        let options = public_api::CompileOptions {
            base: Some("/scope".into()),
            ..Default::default()
        };
        let public =
            public_api::compile_ast(input, options.clone(), vec![], limits.clone()).unwrap();
        let native = Compiler::new_with_options(
            CompilerInput {
                css: "@media important source(\"src\") { @tailwind utilities; }".into(),
                entry: "entry.css".into(),
            },
            vec![],
            limits,
            options,
        )
        .unwrap();
        assert_eq!(public.sources(), native.source_metadata());
        assert_eq!(
            public.sources().root,
            SourceRoot::Path {
                base: "/scope".into(),
                pattern: "src".into()
            }
        );
        assert!(native.important);
        assert!(public.features().contains(public_api::Features::UTILITIES));
    }

    #[test]
    fn source_removal_is_current_across_shared_contexts_and_explicit_overrides() {
        let source = public_json!({"kind":"at-rule","name":"@source","params":"\"file.css\"","nodes":public_json!([])});
        let shared = Arc::new(vec![source]);
        let make = || public_json!({"kind":"context","context":public_json!({"base":"","reference":false}),"nodes":Value::Array(shared.clone())});
        let input = Arc::new(vec![
            public_json!({"kind":"context","context":public_json!({"base":"outer","reference":true}),"nodes":public_json!([make(),make()])}),
        ]);
        let mut current = Current::default();
        let mut allowance = Allowance::default();
        let mut effects = Vec::new();
        let registered = run(
            &input,
            &mut current,
            &mut allowance,
            &"fallback".into(),
            &limits(),
            &mut effects,
        )
        .unwrap();
        assert_eq!(registered.sources.metadata.sources.len(), 1);
        assert_eq!(registered.sources.metadata.sources[0].base, "");
        assert_eq!(registered.sources.metadata.sources[0].pattern, "file.css");
        assert!(current.array(&shared).is_empty());
        assert!(!registered.important && registered.utilities.is_none());
        assert_eq!(effects.len(), 1);
        assert!(
            matches!(&effects[0], InputMutation::Splice { target: Value::Array(target), start: 0, delete_count: 1, .. } if Arc::ptr_eq(target, &shared))
        );
    }

    #[test]
    fn invalid_variant_retains_prior_theme_replacement_and_its_own_removal() {
        let media = public_json!({"kind":"at-rule","name":"@media","params":"theme(inline)","nodes":public_json!([])});
        let root = Arc::new(vec![
            public_json!({"kind":"at-rule","name":"@theme","params":"","nodes":public_json!([
                public_json!({"kind":"declaration","property":"--color-card","value":"red","important":false})
            ])}),
            public_json!({"kind":"at-rule","name":"@custom-variant","params":"Bad (&:hover)","nodes":public_json!([])}),
            media.clone(),
        ]);
        let mut current = Current::default();
        let mut effects = Vec::new();
        let result = run(
            &root,
            &mut current,
            &mut Allowance::default(),
            &Text::default(),
            &limits(),
            &mut effects,
        );
        assert!(
            matches!(result, Err(CompileError::Syntax { message, .. }) if message.starts_with("`@custom-variant Bad`"))
        );
        assert_eq!(current.array(&root).len(), 2);
        assert_eq!(current.array(&root)[0]["selector"], ":root, :host");
        assert!(same_object(&current.array(&root)[1], &media));
        assert!(matches!(
            effects.as_slice(),
            [
                InputMutation::Set { .. },
                InputMutation::Splice { start: 1, .. }
            ]
        ));
    }

    #[test]
    fn earlier_theme_error_prevents_later_variant_removal() {
        let root = Arc::new(vec![
            public_json!({"kind":"at-rule","name":"@theme","params":"prefix(Bad)","nodes":public_json!([])}),
            public_json!({"kind":"at-rule","name":"@custom-variant","params":"Bad (&:hover)","nodes":public_json!([])}),
        ]);
        let mut current = Current::default();
        let mut effects = Vec::new();
        let result = run(
            &root,
            &mut current,
            &mut Allowance::default(),
            &Text::default(),
            &limits(),
            &mut effects,
        );
        assert!(
            matches!(result, Err(CompileError::Syntax { message, .. }) if message.starts_with("The prefix \"Bad\""))
        );
        assert!(effects.is_empty());
        assert!(Arc::ptr_eq(current.array(&root), &root));
    }

    #[test]
    fn legacy_variant_name_assignment_precedes_failed_definition_removal() {
        let node = public_json!({"kind":"at-rule","name":"@variant","params":"Bad (&:hover)","nodes":public_json!([])});
        let root = Arc::new(vec![node.clone()]);
        let mut current = Current::default();
        let mut effects = Vec::new();
        let result = run(
            &root,
            &mut current,
            &mut Allowance::default(),
            &Text::default(),
            &limits(),
            &mut effects,
        );
        assert!(matches!(result, Err(CompileError::Syntax { .. })));
        assert_eq!(current.value(&node)["name"], "@custom-variant");
        assert!(current.array(&root).is_empty());
        assert!(
            matches!(effects.as_slice(), [InputMutation::Set { field, .. }, InputMutation::Splice { start: 0, .. }] if field == "name")
        );
    }
}
