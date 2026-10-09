//! CSS-native theme values. Unsupported directives are explicit compilation gaps.

use super::{ast::AstNode, value::ValueNode, CompileError};
use crate::text::Text;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct ThemeToken {
    pub(super) source: Option<super::public_api::SourceLocation>,
    pub value: Text,
    css_value: Option<Text>,
    pub inline: bool,
    pub reference: bool,
    pub default: bool,
    pub always_emit: bool,
    pub dependencies: Vec<Text>,
}

impl ThemeToken {
    pub(super) fn output_value(&self) -> &Text {
        self.css_value.as_ref().unwrap_or(&self.value)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct ThemeOptions {
    pub(super) inline: bool,
    pub(super) reference: bool,
    pub(super) default: bool,
    pub(super) always_emit: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Theme {
    pub(super) custom_variants: super::custom_variants::CustomVariants,
    pub tokens: BTreeMap<Text, ThemeToken>,
    pub(super) blocked: BTreeSet<Text>,
    order: Vec<Text>,
    anchor: bool,
    pub keyframes: Vec<AstNode>,
    pub prefix: Option<String>,
    pub(super) legacy: std::sync::OnceLock<Result<super::legacy_theme::LegacyTheme, CompileError>>,
    pub(super) legacy_budget: Option<usize>,
}

impl Theme {
    pub(super) fn ordered_tokens(&self) -> impl Iterator<Item = (&Text, &ThemeToken)> {
        self.order
            .iter()
            .filter_map(|key| self.tokens.get(key).map(|token| (key, token)))
    }

    pub(super) fn register_options(
        &mut self,
        params: &Text,
        reference: bool,
        depth_limit: usize,
    ) -> Result<ThemeOptions, CompileError> {
        let mut options = ThemeOptions {
            reference,
            ..ThemeOptions::default()
        };
        let mut prefix = None;
        for option in super::value::segment_text(params, 32, depth_limit)? {
            if option == "inline" {
                options.inline = true;
            } else if option == "reference" {
                options.reference = true;
            } else if option == "default" {
                options.default = true;
            } else if option == "static" {
                options.always_emit = true;
            } else if option.starts_with("prefix(") && option.ends_with(")") {
                prefix = Some(Text::from_units(
                    option.units()[7..option.units().len() - 1].to_vec(),
                ));
            }
        }
        // Validate only the selected last prefix, as in parseThemeOptions.
        if let Some(prefix) = prefix.filter(|value| !value.is_empty()) {
            if !prefix.units().iter().all(|unit| (97..=122).contains(unit)) {
                let mut message = Text::from("The prefix \"");
                message.push_text(&prefix);
                message.push_str(
                    "\" is invalid. Prefixes must be lowercase ASCII letters (a-z) only.",
                );
                return Err(CompileError::Syntax { offset: 0, message });
            }
            self.prefix = Some(super::value::scalar_text(&prefix)?);
        }
        Ok(options)
    }

    pub(super) fn register_declaration(
        &mut self,
        property: &Text,
        value: &Text,
        source: Option<super::public_api::SourceLocation>,
        options: &ThemeOptions,
        depth_limit: usize,
    ) -> Result<(), CompileError> {
        let property = super::legacy_theme::unescape_key_units(property)?;
        // Namespace validation and resets precede the default-value guard upstream.
        if property.ends_with("-*") {
            if value != "initial" {
                let mut message = Text::from("Invalid theme value `");
                message.push_text(value);
                message.push_str("` for namespace `");
                message.push_text(&property);
                message.push_str("`");
                return Err(CompileError::Syntax { offset: 0, message });
            }
            if property == "--*" {
                self.tokens.clear();
            } else {
                let namespace =
                    Text::from_units(property.units()[..property.units().len() - 2].to_vec());
                let ignored: &[&str] = match namespace.to_utf8().as_deref().unwrap_or("") {
                    "--font" => &["--font-weight", "--font-size"],
                    "--inset" => &["--inset-shadow", "--inset-ring"],
                    "--text" => &[
                        "--text-color",
                        "--text-decoration-color",
                        "--text-decoration-thickness",
                        "--text-indent",
                        "--text-shadow",
                        "--text-underline-offset",
                    ],
                    _ => &[],
                };
                self.tokens.retain(|key, _| {
                    !key.units().starts_with(namespace.units())
                        || ignored.iter().any(|ignored| key.starts_with(ignored))
                });
            }
            self.order.retain(|key| self.tokens.contains_key(key));
        }
        if options.default
            && self
                .tokens
                .get(&property)
                .is_some_and(|value| !value.default)
        {
            return Ok(());
        }
        if value == "initial" {
            self.tokens.remove(&property);
            self.order.retain(|key| self.tokens.contains_key(key));
            return Ok(());
        }
        if !self.tokens.contains_key(&property) {
            self.order.push(property.clone());
        }
        self.tokens.insert(
            property,
            ThemeToken {
                source,
                value: value.clone(),
                css_value: None,
                inline: options.inline,
                reference: options.reference,
                default: options.default,
                always_emit: options.always_emit,
                dependencies: used_variables(&super::value::parse_text(value, depth_limit)?),
            },
        );
        Ok(())
    }

    pub(super) fn mark_anchor(&mut self) -> bool {
        if self.anchor {
            return false;
        }
        self.anchor = true;
        true
    }

    #[cfg(test)]
    pub fn extract(nodes: &mut Vec<AstNode>, depth_limit: usize) -> Result<Self, CompileError> {
        fn read(
            nodes: &mut Vec<AstNode>,
            theme: &mut Theme,
            depth_limit: usize,
            reference: bool,
        ) -> Result<(), CompileError> {
            let mut output = Vec::new();
            for mut node in std::mem::take(nodes) {
                let reference = reference
                    || matches!(
                        &node,
                        AstNode::Context {
                            reference: true,
                            ..
                        }
                    );
                if let AstNode::AtRule {
                    name,
                    params,
                    nodes: children,
                    source,
                    ..
                } = &mut node
                {
                    if name == "@plugin" || name == "@config" {
                        return Err(CompileError::UnsupportedFeature {
                            feature: super::value::scalar_text(name)?,
                        });
                    }
                    if ["@utility", "@custom-variant", "@source"]
                        .iter()
                        .any(|label| name == label)
                    {
                        return Err(CompileError::PendingFeature {
                            feature: super::value::scalar_text(name)?,
                        });
                    }
                    if name == "@theme" {
                        let options = theme.register_options(params, reference, depth_limit)?;
                        let Some(children) = children else {
                            return Err(CompileError::Syntax {
                                offset: 0,
                                message: "@theme requires a block".into(),
                            });
                        };
                        for child in children {
                            if matches!(child, AstNode::AtRule { name, .. } if name == "@keyframes")
                            {
                                theme.keyframes.push(child.clone());
                                continue;
                            }
                            if matches!(child, AstNode::Comment { .. }) {
                                continue;
                            }
                            let AstNode::Declaration {
                                property,
                                value,
                                source: child_source,
                                ..
                            } = child
                            else {
                                return Err(invalid_child(name, params, child));
                            };
                            if !property.starts_with("--") {
                                return Err(invalid_child(name, params, child));
                            }
                            theme.register_declaration(
                                property,
                                value,
                                child_source.clone(),
                                &options,
                                depth_limit,
                            )?;
                        }
                        if theme.mark_anchor() {
                            output.push(AstNode::AtRule {
                                origin: None,
                                source: source.clone(),
                                destination: None,
                                name: "@rustify-theme-anchor".into(),
                                params: Text::default(),
                                nodes: None,
                            });
                        }
                        continue;
                    }
                }
                if let Some(children) = node.children_mut() {
                    read(children, theme, depth_limit, reference)?;
                }
                output.push(node);
            }
            *nodes = output;
            Ok(())
        }
        let mut theme = Self::default();
        read(nodes, &mut theme, depth_limit, false)?;
        Ok(theme)
    }

    pub fn prepare_functions(
        &mut self,
        depth: usize,
        max_bytes: usize,
        cache_bytes: usize,
    ) -> Result<(), CompileError> {
        let mut updates = Vec::new();
        let mut additional = 0usize;
        let owned = self.owned_bytes();
        for (key, token) in &self.tokens {
            if !token.reference {
                let value =
                    super::functions::in_text_value(&token.value, self, false, depth, max_bytes)?;
                if value != token.value {
                    let dependencies = used_variables(&super::value::parse_text(&value, depth)?);
                    additional = additional
                        .saturating_add(value.owned_bytes())
                        .saturating_add(dependencies.capacity() * std::mem::size_of::<Text>())
                        .saturating_add(dependencies.iter().map(Text::owned_bytes).sum::<usize>());
                    super::check_limit(
                        owned
                            .saturating_add(additional)
                            .saturating_add(super::legacy_theme::cache_usage(self).owned_bytes),
                        cache_bytes,
                        "cache_bytes",
                    )?;
                    updates.push((key.clone(), value, dependencies));
                }
            }
        }
        for (key, value, dependencies) in updates {
            if let Some(token) = self.tokens.get_mut(&key) {
                token.css_value = Some(value);
                token.dependencies = dependencies;
            }
        }
        super::check_limit(
            self.owned_bytes()
                .saturating_add(super::legacy_theme::cache_usage(self).owned_bytes),
            cache_bytes,
            "cache_bytes",
        )?;
        self.legacy_budget = Some(cache_bytes.saturating_sub(self.owned_bytes()));
        Ok(())
    }

    pub(super) fn prefix_key_text(&self, name: &Text) -> Text {
        if let Some(prefix) = &self.prefix {
            let rest = name.units().get(2..).unwrap_or(name.units());
            let mut units = Vec::with_capacity(prefix.encode_utf16().count() + 3 + rest.len());
            units.extend([45, 45]);
            units.extend(prefix.encode_utf16());
            units.push(45);
            units.extend_from_slice(rest);
            Text::from_units(units)
        } else {
            name.clone()
        }
    }

    pub(super) fn owned_bytes(&self) -> usize {
        let slots = self.tokens.len().div_ceil(5) * 11;
        slots
            .saturating_mul(
                std::mem::size_of::<(Text, ThemeToken)>() + 2 * std::mem::size_of::<usize>(),
            )
            .saturating_add(self.order.capacity() * std::mem::size_of::<Text>())
            .saturating_add(self.order.iter().map(Text::owned_bytes).sum::<usize>())
            .saturating_add(
                self.tokens
                    .iter()
                    .map(|(key, token)| {
                        key.owned_bytes()
                            .saturating_add(token.value.owned_bytes())
                            .saturating_add(token.css_value.as_ref().map_or(0, Text::owned_bytes))
                            .saturating_add(
                                token.dependencies.capacity() * std::mem::size_of::<Text>(),
                            )
                            .saturating_add(
                                token
                                    .dependencies
                                    .iter()
                                    .map(Text::owned_bytes)
                                    .sum::<usize>(),
                            )
                    })
                    .sum::<usize>(),
            )
    }

    pub fn prefix_key(&self, name: &str) -> String {
        match &self.prefix {
            Some(prefix) => format!("--{prefix}-{}", name.strip_prefix("--").unwrap_or(name)),
            None => name.into(),
        }
    }

    pub fn resolve_text(&self, name: &Text, used: &mut BTreeSet<Text>) -> Option<Text> {
        let token = self.tokens.get(name)?;
        if token.inline {
            used.extend(token.dependencies.iter().cloned());
            return Some(token.value.clone());
        }
        let name = super::candidate::escape_text(&self.prefix_key_text(name));
        let mut output = Text::from("var(");
        output.push_text(&name);
        if token.reference && !token.value.is_empty() {
            output.push_str(", ");
            output.push_text(&token.value);
        } else if !token.reference {
            used.insert(name);
        }
        output.push_str(")");
        Some(output)
    }

    pub fn raw_text(&self, name: &Text, used: &mut BTreeSet<Text>) -> Option<Text> {
        let token = self.tokens.get(name)?;
        used.extend(token.dependencies.iter().cloned());
        Some(token.value.clone())
    }

    pub fn namespace_values_text(&self, namespace: &Text) -> Vec<Text> {
        let mut prefix = namespace.clone();
        prefix.push_str("-");
        self.ordered_tokens()
            .filter(|(name, _)| *name == namespace || name.units().starts_with(prefix.units()))
            .map(|(_, token)| token.value.clone())
            .collect()
    }

    pub fn declarations(&self, used: &BTreeSet<Text>) -> Result<Vec<AstNode>, CompileError> {
        let used: BTreeSet<_> = used
            .iter()
            .map(|key| {
                let key = if let Some(prefix) = &self.prefix {
                    let mut unprefixed = Text::from("--");
                    unprefixed.push_text(&Text::from_units(
                        key.units()
                            .get(3 + prefix.encode_utf16().count()..)
                            .unwrap_or(&[])
                            .to_vec(),
                    ));
                    unprefixed
                } else {
                    key.clone()
                };
                super::legacy_theme::unescape_key_units(&key)
            })
            .collect::<Result<_, _>>()?;
        let mut dependents: BTreeMap<&Text, Vec<&Text>> = BTreeMap::new();
        for (name, token) in &self.tokens {
            if !token.reference {
                for dependency in &token.dependencies {
                    dependents.entry(dependency).or_default().push(name);
                }
            }
        }
        fn required(
            name: &Text,
            theme: &Theme,
            used: &BTreeSet<Text>,
            dependents: &BTreeMap<&Text, Vec<&Text>>,
            seen: &mut BTreeSet<Text>,
        ) -> bool {
            if !seen.insert(name.clone()) {
                return true;
            }
            used.contains(name)
                || theme
                    .tokens
                    .get(name)
                    .is_some_and(|token| token.always_emit)
                || dependents
                    .get(&super::candidate::escape_text(&theme.prefix_key_text(name)))
                    .is_some_and(|names| {
                        names
                            .iter()
                            .any(|name| required(name, theme, used, dependents, seen))
                    })
        }
        Ok(self
            .ordered_tokens()
            .filter(|(key, token)| {
                !token.reference
                    && token.output_value() != "initial"
                    && required(key, self, &used, &dependents, &mut BTreeSet::new())
            })
            .map(|(key, token)| {
                let mut node = AstNode::declaration(
                    super::candidate::escape_text(&self.prefix_key_text(key)),
                    token.output_value().clone(),
                );
                node.set_source(token.source.clone());
                node
            })
            .collect())
    }

    /// Source-discovered variables use the same prefix and escape decoding as theme keys.
    pub(super) fn external_variable(&self, raw: &Text) -> Result<Option<Text>, CompileError> {
        let key = match &self.prefix {
            Some(prefix) => super::value::concat_text(&[
                &"--".into(),
                &Text::from_units(raw.units()[raw.units().len().min(3 + prefix.len())..].to_vec()),
            ]),
            None => raw.clone(),
        };
        let key = super::legacy_theme::unescape_key_units(&key)?;
        Ok(self
            .tokens
            .contains_key(&key)
            .then(|| super::candidate::escape_text(&self.prefix_key_text(&key))))
    }
}

#[cfg(test)]
pub(super) fn invalid_child(name: &Text, params: &Text, child: &AstNode) -> CompileError {
    invalid_children(name, params, std::slice::from_ref(child))
}

pub(super) fn invalid_children(name: &Text, params: &Text, children: &[AstNode]) -> CompileError {
    let css = super::ast::to_css(&[AstNode::AtRule {
        origin: None,
        source: None,
        destination: None,
        name: name.clone(),
        params: params.clone(),
        nodes: Some(children.to_vec()),
    }]);
    let lines = css.units().split(|unit| *unit == 10).collect::<Vec<_>>();
    let mut message =
        Text::from("`@theme` blocks must only contain custom properties or `@keyframes`.\n\n");
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            message.push_str("\n");
        }
        message.push_str(if index == 0 || index >= lines.len() - 2 {
            "  "
        } else {
            "> "
        });
        message.push_text(&Text::from_units(line.to_vec()));
    }
    CompileError::Syntax { offset: 0, message }
}

pub(super) fn used_variables(nodes: &[ValueNode]) -> Vec<Text> {
    fn visit(nodes: &[ValueNode], within_var: bool, output: &mut Vec<Text>) {
        for node in nodes {
            match node {
                ValueNode::Word { value } if within_var && value.starts_with("--") => {
                    output.push(value.clone());
                }
                ValueNode::Function { name, nodes } => {
                    visit(nodes, within_var || name == "var", output)
                }
                _ => {}
            }
        }
    }
    let mut output = Vec::new();
    visit(nodes, false, &mut output);
    output
}

#[cfg(test)]
mod registration_tests {
    use super::{invalid_child, AstNode, CompileError, Text, Theme, ThemeOptions};
    use crate::compiler::public_api::{Source, SourceLocation};
    use std::sync::Arc;

    #[test]
    fn options_keep_top_level_segments_last_prefix_and_inherited_reference() {
        let mut theme = Theme::default();
        let options = theme
            .register_options(
                &"ignored(reference) inline default static prefix(UPPER) prefix(tw)".into(),
                true,
                32,
            )
            .unwrap();
        assert_eq!(
            (options, theme.prefix.as_deref()),
            (
                ThemeOptions {
                    inline: true,
                    reference: true,
                    default: true,
                    always_emit: true,
                },
                Some("tw"),
            ),
        );
    }

    #[test]
    fn options_do_not_split_tabs_or_nested_spaces() {
        let mut theme = Theme::default();
        assert_eq!(
            theme
                .register_options(
                    &"inline\treference ignored(default static)".into(),
                    false,
                    32
                )
                .unwrap(),
            ThemeOptions::default(),
        );
    }

    #[test]
    fn options_preserve_depth_limit_before_prefix_assignment() {
        let mut theme = Theme::default();
        assert_eq!(
            theme.register_options(&"prefix(tw) ignored(nested(value))".into(), false, 1),
            Err(CompileError::Limit {
                resource: "ast_depth".into(),
                limit: 1,
            }),
        );
        assert_eq!(theme.prefix, None);
    }

    #[test]
    fn declaration_retains_unescaped_units_payload_dependencies_and_source_identity() {
        let mut theme = Theme::default();
        let dependency = Text::from_units(vec![45, 45, 120, 45, 0xd800]);
        let mut value = Text::from("var(");
        value.push_text(&dependency);
        value.push_str(")");
        let source = SourceLocation {
            source: Arc::new(Source {
                file: Some("theme.css".into()),
                code: value.clone(),
            }),
            start: 3,
            end: 21,
            public: None,
        };
        let options = ThemeOptions {
            inline: true,
            reference: true,
            default: true,
            always_emit: true,
        };
        theme
            .register_declaration(
                &"--color-\\d800 ".into(),
                &value,
                Some(source.clone()),
                &options,
                32,
            )
            .unwrap();
        let key = Text::from_units(vec![45, 45, 99, 111, 108, 111, 114, 45, 0xd800]);
        let token = &theme.tokens[&key];
        assert_eq!(token.value, value);
        assert_eq!(token.dependencies, vec![dependency]);
        assert_eq!(token.source.as_ref(), Some(&source));
        assert!(Arc::ptr_eq(
            &token.source.as_ref().unwrap().source,
            &source.source,
        ));
        assert_eq!(
            (
                token.inline,
                token.reference,
                token.default,
                token.always_emit
            ),
            (true, true, true, true),
        );
    }

    #[test]
    fn default_initial_keeps_existing_explicit_value() {
        let mut theme = Theme::default();
        let key = Text::from("--color-primary");
        theme
            .register_declaration(&key, &"red".into(), None, &ThemeOptions::default(), 32)
            .unwrap();
        theme
            .register_declaration(
                &key,
                &"initial".into(),
                None,
                &ThemeOptions {
                    default: true,
                    ..ThemeOptions::default()
                },
                32,
            )
            .unwrap();
        assert_eq!(theme.tokens[&key].value, Text::from("red"));
    }

    #[test]
    fn namespace_default_reset_preserves_ignored_keys_and_reinsertion_order() {
        let mut theme = Theme::default();
        let options = ThemeOptions::default();
        for key in ["--font-sans", "--font-weight-bold", "--color-primary"] {
            theme
                .register_declaration(&key.into(), &"value".into(), None, &options, 32)
                .unwrap();
        }
        theme
            .register_declaration(
                &"--font-*".into(),
                &"initial".into(),
                None,
                &ThemeOptions {
                    default: true,
                    ..options
                },
                32,
            )
            .unwrap();
        theme
            .register_declaration(&"--font-sans".into(), &"new".into(), None, &options, 32)
            .unwrap();
        assert_eq!(
            theme
                .ordered_tokens()
                .map(|(key, _)| key.clone())
                .collect::<Vec<_>>(),
            ["--font-weight-bold", "--color-primary", "--font-sans"]
                .map(Text::from)
                .to_vec(),
        );
        theme
            .register_declaration(&"--\\2a ".into(), &"initial".into(), None, &options, 32)
            .unwrap();
        assert!(theme.order.is_empty() && theme.tokens.is_empty());
    }

    #[test]
    fn namespace_value_error_precedes_dependency_depth_error_and_retains_units() {
        let mut theme = Theme::default();
        let mut value = Text::from("var(--");
        value.push_text(&Text::from_units(vec![0xd800]));
        value.push_str(")");
        let mut message = Text::from("Invalid theme value `");
        message.push_text(&value);
        message.push_str("` for namespace `--color-*`");
        assert_eq!(
            theme.register_declaration(
                &"--\\63 olor-*".into(),
                &value,
                None,
                &ThemeOptions::default(),
                0,
            ),
            Err(CompileError::Syntax { offset: 0, message }),
        );
    }

    #[test]
    fn invalid_child_keeps_excerpt_format_and_original_units() {
        let raw = Text::from_units(vec![0xd800]);
        let mut message = Text::from(
            "`@theme` blocks must only contain custom properties or `@keyframes`.\n\n  @theme inline {\n>   color: ",
        );
        message.push_text(&raw);
        message.push_str(";\n  }\n  ");
        assert_eq!(
            invalid_child(
                &"@theme".into(),
                &"inline".into(),
                &AstNode::declaration("color", raw),
            ),
            CompileError::Syntax { offset: 0, message },
        );
    }
}

#[cfg(test)]
mod prefix_tests {
    use crate::compiler::{Compiler, CompilerInput, Limits};

    #[test]
    fn variable_tracking_retains_original_non_scalar_units() {
        let raw =
            crate::text::Text::from_units(vec![118, 97, 114, 40, 45, 45, 120, 45, 0xd800, 41]);
        let nodes = super::super::value::parse_text(&raw, 32).unwrap();
        let observed: Vec<crate::text::Text> = super::used_variables(&nodes);
        assert_eq!(observed.len(), 1);
        assert_eq!(observed[0].units(), &[45, 45, 120, 45, 0xd800]);
    }

    #[test]
    fn prefix_css_matches_fixed_official_inputs() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/prefix-fixtures.json"
        ))
        .unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let compiler = Compiler::new(
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
            );
            if let Some(expected) = case["expectedError"].as_str() {
                assert!(
                    matches!(compiler, Err(crate::compiler::CompileError::Syntax { message, .. }) if message == expected),
                    "{}",
                    case["id"]
                );
                continue;
            }
            let mut compiler = compiler.unwrap();
            let candidates = case["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            let prepared = compiler.prepare_add(&candidates).unwrap();
            assert_eq!(
                prepared.report().css,
                case["expectedCss"].as_str().unwrap(),
                "{}",
                case["id"]
            );
            assert!(prepared.report().unsupported.is_empty(), "{}", case["id"]);
        }
    }
}
