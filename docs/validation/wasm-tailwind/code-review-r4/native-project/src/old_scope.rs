//! AST-level isolation of runtime styles and compiler-owned global resources.

use crate::compiler::value::{self, ValueNode};
use crate::compiler::{ast::AstNode, CompileError};
use crate::text::Text;
use std::collections::{BTreeMap, BTreeSet};

pub fn stylesheet(
    nodes: &[AstNode],
    owner: &str,
    max_depth: usize,
) -> Result<Vec<AstNode>, CompileError> {
    crate::compiler::ast::validate_depth(nodes, max_depth)?;
    if owner.is_empty()
        || !owner
            .bytes()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, b'-' | b'_'))
    {
        return Err(CompileError::Syntax {
            offset: 0,
            message: "style owner must be a nonempty ASCII identifier".into(),
        });
    }
    let mut keyframes = BTreeMap::new();
    collect_keyframes(nodes, owner, &mut keyframes);
    let animated = animation_variables(nodes, max_depth)?;
    let names = Names {
        owner,
        keyframes: &keyframes,
        animated: &animated,
    };
    let mut local = nodes.to_vec();
    let mut globals = Vec::new();
    rewrite(&mut local, &names, &mut globals, max_depth, 1)?;
    let mut result = Vec::new();
    if !local.is_empty() {
        result.push(AstNode::AtRule {
            origin: None,
            source: None,
            destination: None,
            name: "@scope".into(),
            params: format!(
                "([data-rustify-style-owner=\"{owner}\"]) to ([data-rustify-style-owner])"
            )
            .into(),
            nodes: Some(local),
        });
    }
    result.extend(globals);
    crate::compiler::ast::validate_depth(&result, max_depth)?;
    Ok(result)
}

fn collect_keyframes(nodes: &[AstNode], owner: &str, names: &mut BTreeMap<Text, Text>) {
    for node in nodes {
        if let AstNode::AtRule { name, params, .. } = node {
            if name == "@keyframes" {
                let mut renamed = Text::from(format!("rustify-{owner}-"));
                renamed.push_text(params);
                names.insert(params.clone(), renamed);
            }
        }
        if let Some(children) = node.children() {
            collect_keyframes(children, owner, names);
        }
    }
}

/// Where a value is substituted: an `animation-name` list holds only names, while
/// the `animation` shorthand also holds timing, iteration, direction, fill and
/// play-state keywords that take precedence over a keyframes name.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Animation {
    Shorthand,
    Name,
}

fn animation_role(property: &Text) -> Option<Animation> {
    if property == "animation" {
        Some(Animation::Shorthand)
    } else if property == "animation-name" {
        Some(Animation::Name)
    } else {
        None
    }
}

/// Original names of the custom properties whose values can reach `animation`
/// or `animation-name`, directly or through other custom properties. A property
/// that reaches `animation-name` anywhere is read as a name list.
fn animation_variables(
    nodes: &[AstNode],
    limit: usize,
) -> Result<BTreeMap<Text, Animation>, CompileError> {
    fn walk(
        nodes: &[AstNode],
        limit: usize,
        seeds: &mut Vec<(Text, Animation)>,
        references: &mut BTreeMap<Text, BTreeSet<Text>>,
    ) -> Result<(), CompileError> {
        for node in nodes {
            if let AstNode::Declaration {
                property, value, ..
            } = node
            {
                let mut names = BTreeSet::new();
                if let Some(role) = animation_role(property) {
                    variable_references(&value::parse_text(value, limit)?, &mut names);
                    seeds.extend(names.into_iter().map(|name| (name, role)));
                } else if property.starts_with("--") {
                    variable_references(&value::parse_text(value, limit)?, &mut names);
                    references
                        .entry(property.clone())
                        .or_default()
                        .extend(names);
                }
            }
            if let Some(children) = node.children() {
                walk(children, limit, seeds, references)?;
            }
        }
        Ok(())
    }
    let mut pending = Vec::new();
    let mut references = BTreeMap::new();
    walk(nodes, limit, &mut pending, &mut references)?;
    let mut animated = BTreeMap::new();
    while let Some((name, role)) = pending.pop() {
        if animated.get(&name).is_some_and(|known| *known >= role) {
            continue;
        }
        animated.insert(name.clone(), role);
        if let Some(next) = references.get(&name) {
            pending.extend(next.iter().map(|next| (next.clone(), role)));
        }
    }
    Ok(animated)
}

fn variable_references(nodes: &[ValueNode], names: &mut BTreeSet<Text>) {
    for node in nodes {
        if let ValueNode::Function { name, nodes } = node {
            if name == "var" {
                if let Some(ValueNode::Word { value }) = nodes.first() {
                    names.insert(value.clone());
                }
            }
            variable_references(nodes, names);
        }
    }
}

struct Names<'a> {
    owner: &'a str,
    keyframes: &'a BTreeMap<Text, Text>,
    animated: &'a BTreeMap<Text, Animation>,
}

fn internal_variable(value: &Text, owner: &str) -> Option<Text> {
    if !value.starts_with("--tw-") {
        return None;
    }
    let mut renamed = Text::from(format!("--rustify-{owner}-tw-"));
    renamed.push_text(&Text::from_units(value.units()[5..].to_vec()));
    Some(renamed)
}

fn rewrite_value(nodes: &mut [ValueNode], owner: &str) {
    for node in nodes {
        match node {
            ValueNode::Word { value } => {
                if let Some(renamed) = internal_variable(value, owner) {
                    *value = renamed;
                }
            }
            ValueNode::Function { nodes, .. } => rewrite_value(nodes, owner),
            ValueNode::Separator { .. } => {}
        }
    }
}

/// The non-name properties of one `<single-animation>`, each taken at most once.
#[derive(Default)]
struct Layer {
    easing: bool,
    iteration: bool,
    direction: bool,
    fill: bool,
    play: bool,
    name: bool,
}

impl Layer {
    /// Whether `word` is read as a keyframes name. A keyword that is valid for a
    /// property not yet seen in this layer belongs to that property.
    fn is_name(&mut self, word: &Text) -> bool {
        const KEYWORDS: [&[&str]; 5] = [
            &[
                "linear",
                "ease",
                "ease-in",
                "ease-out",
                "ease-in-out",
                "step-start",
                "step-end",
            ],
            &["infinite"],
            &["normal", "reverse", "alternate", "alternate-reverse"],
            &["none", "forwards", "backwards", "both"],
            &["running", "paused"],
        ];
        if word.units().first().is_some_and(|unit| {
            matches!(unit, 34 | 39 | 46 | 48..=57)
                || (matches!(unit, 43 | 45)
                    && word
                        .units()
                        .get(1)
                        .is_some_and(|next| matches!(next, 46 | 48..=57)))
        }) {
            // A <string> name is never renamed, but it still takes the name slot.
            if matches!(word.units()[0], 34 | 39) {
                self.name = true;
            }
            return false;
        }
        let slots = [
            &mut self.easing,
            &mut self.iteration,
            &mut self.direction,
            &mut self.fill,
            &mut self.play,
        ];
        for (slot, keywords) in slots.into_iter().zip(KEYWORDS) {
            if !*slot
                && keywords
                    .iter()
                    .any(|keyword| word.eq_ignore_ascii_case(keyword))
            {
                *slot = true;
                return false;
            }
        }
        !std::mem::replace(&mut self.name, true)
    }
}

/// Renames the keyframes references in a value substituted as `role`. Inside the
/// shorthand, var() contents are unknown here and do not take a slot; a
/// fallback is substituted in place of the reference.
fn rewrite_keyframes(nodes: &mut [ValueNode], names: &Names, role: Animation, layer: &mut Layer) {
    for node in nodes {
        match node {
            ValueNode::Word { value } => {
                if role == Animation::Name || layer.is_name(value) {
                    if let Some(replacement) = names.keyframes.get(value) {
                        *value = replacement.clone();
                    }
                }
            }
            ValueNode::Function { name, nodes } if name == "var" => {
                if let Some(fallback) = nodes.iter().position(
                    |node| matches!(node, ValueNode::Separator { value } if value.contains(",")),
                ) {
                    rewrite_keyframes(&mut nodes[fallback + 1..], names, role, layer);
                }
            }
            // steps(), cubic-bezier() and linear() are the only functions in the shorthand.
            ValueNode::Function { .. } => layer.easing = true,
            ValueNode::Separator { value } => {
                if value.contains(",") {
                    *layer = Layer::default();
                }
            }
        }
    }
}

fn rewrite(
    nodes: &mut Vec<AstNode>,
    names: &Names,
    globals: &mut Vec<AstNode>,
    limit: usize,
    depth: usize,
) -> Result<(), CompileError> {
    if depth > limit {
        return Err(CompileError::Limit {
            resource: "ast_depth".into(),
            limit,
        });
    }
    let mut local = Vec::with_capacity(nodes.len());
    for mut node in std::mem::take(nodes) {
        let mut global = false;
        match &mut node {
            AstNode::Declaration {
                property, value, ..
            } => {
                let role =
                    animation_role(property).or_else(|| names.animated.get(property).copied());
                if let Some(renamed) = internal_variable(property, names.owner) {
                    *property = renamed;
                }
                let mut parsed = value::parse_text(value, limit)?;
                if let Some(role) = role {
                    rewrite_keyframes(&mut parsed, names, role, &mut Layer::default());
                }
                rewrite_value(&mut parsed, names.owner);
                *value = value::to_css_text(&parsed);
            }
            AstNode::Rule { selector, .. } => {
                if selector.units().split(|unit| *unit == 44).all(|part| {
                    let part = Text::from_units(part.to_vec()).trim_whitespace();
                    part == ":root" || part == ":host"
                }) {
                    *selector = ":scope".into();
                }
            }
            AstNode::AtRule { name, params, .. } => {
                if name == "@property" {
                    if let Some(replacement) = internal_variable(params, names.owner) {
                        *params = replacement;
                    } else {
                        return Err(CompileError::UnsupportedFeature {
                            feature: "runtime registration of public @property".into(),
                        });
                    }
                    global = true;
                } else if name == "@keyframes" {
                    *params = names.keyframes[params].clone();
                    global = true;
                } else if name == "@font-face" || name == "@import" {
                    return Err(CompileError::UnsupportedFeature {
                        feature: "runtime global network resources".into(),
                    });
                }
            }
            AstNode::AtRoot { .. } | AstNode::Comment { .. } | AstNode::Context { .. } => {}
        }
        if let Some(children) = node.children_mut() {
            rewrite(children, names, globals, limit, depth + 1)?;
        }
        if global {
            globals.push(node);
        } else {
            local.push(node);
        }
    }
    *nodes = local;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn non_scalar_declarations_keep_units_while_internal_bindings_are_isolated() {
        use crate::text::Text;
        let mut property = Text::from("--tw-");
        property.push_text(&Text::from_units(vec![0xd800]));
        let mut value = Text::from("outer(var(--tw-");
        value.push_text(&Text::from_units(vec![0xdc00]));
        value.push_str("), \"�\", \\\\ud800)");
        let nodes = vec![crate::compiler::ast::AstNode::declaration(property, value)];
        let scoped = super::stylesheet(&nodes, "alpha", 32).unwrap();
        let actual = crate::compiler::ast::to_css(&scoped);
        let mut expected = Text::from("@scope ([data-rustify-style-owner=\"alpha\"]) to ([data-rustify-style-owner]) {\n  --rustify-alpha-tw-");
        expected.push_text(&Text::from_units(vec![0xd800]));
        expected.push_str(": outer(var(--rustify-alpha-tw-");
        expected.push_text(&Text::from_units(vec![0xdc00]));
        expected.push_str("), \"�\", \\\\ud800);\n}\n");
        assert_eq!(actual, expected);
    }
    use super::*;
    use crate::compiler::{ast::to_css, parser};

    #[test]
    fn generated_globals_and_references_are_isolated_without_changing_public_tokens() {
        let ast = parser::parse("@property --tw-shadow { syntax: \"*\"; inherits: false; } @keyframes spin { to { transform: rotate(360deg); } } :root, :host { --color-brand: red; } .shadow { --tw-shadow: var(--color-brand); box-shadow: var(--tw-shadow); animation: spin 1s linear; content: \"--tw-shadow spin\"; }", 32).unwrap();
        let css = to_css(&stylesheet(&ast, "alpha", 32).unwrap());
        assert!(css.contains("@property --rustify-alpha-tw-shadow"));
        assert!(css.contains("@keyframes rustify-alpha-spin"));
        assert!(css.contains(
            "@scope ([data-rustify-style-owner=\"alpha\"]) to ([data-rustify-style-owner])"
        ));
        assert!(css.contains(":scope {"));
        assert!(css.contains("--color-brand: red"));
        assert!(css.contains("box-shadow: var(--rustify-alpha-tw-shadow)"));
        assert!(css.contains("animation: rustify-alpha-spin 1s linear"));
        assert!(css.contains("content: \"--tw-shadow spin\""));
    }

    #[test]
    fn keyframe_references_follow_custom_properties_into_animations() {
        let ast = parser::parse("@keyframes card { to { opacity: 0; } } :root, :host { --animate-card: card 1s linear infinite; --animate-alias: var(--animate-card); --animate-chain: var(--tw-missing, card 2s); --tw-motion: card 3s; --label: card; } .a { animation: var(--animate-alias); } .b { animation: var(--undefined, card 4s); } .c { animation-name: var(--animate-chain); } .d { animation: var(--tw-motion); } .e { content: var(--label); }", 32).unwrap();
        let css = to_css(&stylesheet(&ast, "alpha", 32).unwrap());
        assert!(css.contains("@keyframes rustify-alpha-card"));
        assert!(css.contains("--animate-card: rustify-alpha-card 1s linear infinite"));
        assert!(css.contains("--animate-alias: var(--animate-card)"));
        assert!(
            css.contains("--animate-chain: var(--rustify-alpha-tw-missing, rustify-alpha-card 2s)")
        );
        assert!(css.contains("--rustify-alpha-tw-motion: rustify-alpha-card 3s"));
        assert!(css.contains("animation: var(--undefined, rustify-alpha-card 4s)"));
        assert!(css.contains("--label: card;"));
        let other = to_css(&stylesheet(&ast, "beta", 32).unwrap());
        assert!(other.contains("--animate-card: rustify-beta-card 1s linear infinite"));
    }

    #[test]
    fn keyframe_names_that_match_animation_keywords_stay_in_their_own_slot() {
        let ast = parser::parse("@keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: 1; } } @keyframes paused { to { opacity: 1; } } :root, :host { --ease: linear; --motion: card 1s linear infinite; --name: linear; --rest: 1s linear; } .a { animation: card 1s var(--ease) infinite; } .b { animation: var(--motion); } .c { animation-name: var(--name); } .d { animation: card var(--rest); } .e { animation: linear 1s linear, paused 2s steps(4, end) paused; } .f { animation: var(--undefined, card 1s linear), linear 2s; } .g { animation: \"card\" linear; }", 32).unwrap();
        let css = to_css(&stylesheet(&ast, "alpha", 32).unwrap());
        assert!(css.contains("--ease: linear;"));
        assert!(css.contains("animation: rustify-alpha-card 1s var(--ease) infinite"));
        assert!(css.contains("--motion: rustify-alpha-card 1s linear infinite"));
        assert!(css.contains("--name: rustify-alpha-linear;"));
        assert!(css.contains("--rest: 1s linear;"));
        assert!(css.contains("animation: rustify-alpha-card var(--rest)"));
        assert!(css.contains(
            "animation: linear 1s rustify-alpha-linear, paused 2s steps(4, end) rustify-alpha-paused"
        ));
        assert!(
            css.contains("animation: var(--undefined, rustify-alpha-card 1s linear), linear 2s")
        );
        assert!(css.contains("animation: \"card\" linear"));
    }
}
