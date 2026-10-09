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
    let plan = plan(nodes, owner, &keyframes, max_depth)?;
    let mut state = Rewrite {
        owner,
        keyframes: &keyframes,
        plan,
        next: 0,
    };
    let mut local = nodes.to_vec();
    let mut globals = Vec::new();
    rewrite(&mut local, &mut state, &mut globals, max_depth, 1)?;
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

/// The slots of one `<single-animation>` are the bits of a layer state, and a set
/// of possible states is a bit set over all 64 of them.
type States = u64;

const EASING: u8 = 1;
const ITERATION: u8 = 2;
const DIRECTION: u8 = 4;
const FILL: u8 = 8;
const PLAY: u8 = 16;
const NAME: u8 = 32;

/// The set holding only the state of a layer with no slot taken.
const EMPTY: States = 1;

/// Keywords that belong to another slot while it is still free in the layer, in
/// preference to the keyframes name.
const KEYWORDS: [(u8, &[&str]); 5] = [
    (
        EASING,
        &[
            "linear",
            "ease",
            "ease-in",
            "ease-out",
            "ease-in-out",
            "step-start",
            "step-end",
        ],
    ),
    (ITERATION, &["infinite"]),
    (
        DIRECTION,
        &["normal", "reverse", "alternate", "alternate-reverse"],
    ),
    (FILL, &["none", "forwards", "backwards", "both"]),
    (PLAY, &["running", "paused"]),
];

/// Takes `slot` in every state of the set. A state that already holds it makes the
/// declaration invalid, so it leaves the set.
fn take(states: States, slot: u8) -> States {
    let mut next = 0;
    for state in 0..64u8 {
        if states >> state & 1 == 1 && state & slot == 0 {
            next |= 1 << (state | slot);
        }
    }
    next
}

/// The states an unknown value can leave behind: any further slots taken.
fn widen(states: States) -> States {
    let mut next = 0;
    for state in 0..64u8 {
        if states >> state & 1 == 1 {
            for wider in 0..64u8 {
                if wider & state == state {
                    next |= 1 << wider;
                }
            }
        }
    }
    next
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Numeric {
    Number,
    Time,
    Other,
}

/// Classifies a CSS number token by its unit, or returns `None` for a non-number.
fn numeric(word: &[u16]) -> Option<Numeric> {
    let digit = |index: usize| word.get(index).is_some_and(|unit| (48..=57).contains(unit));
    let mut end = usize::from(matches!(word.first(), Some(43 | 45)));
    let start = end;
    while digit(end) {
        end += 1;
    }
    if word.get(end) == Some(&46) && digit(end + 1) {
        end += 2;
        while digit(end) {
            end += 1;
        }
    } else if end == start {
        return None;
    }
    if matches!(word.get(end), Some(69 | 101)) {
        let sign = usize::from(matches!(word.get(end + 1), Some(43 | 45)));
        if digit(end + 1 + sign) {
            end += 1 + sign;
            while digit(end) {
                end += 1;
            }
        }
    }
    let unit = Text::from_units(word[end..].to_vec());
    Some(if unit.units().is_empty() {
        Numeric::Number
    } else if unit.eq_ignore_ascii_case("s") || unit.eq_ignore_ascii_case("ms") {
        Numeric::Time
    } else {
        Numeric::Other
    })
}

/// Reads one word in a layer state: the state after it and whether it is the
/// keyframes name, or `None` when it makes the declaration invalid.
fn step(state: u8, word: &Text) -> Option<(u8, bool)> {
    let slot = match word.units().first() {
        // A <string> name is never renamed, but it still takes the name slot.
        Some(34 | 39) => NAME,
        _ => match numeric(word.units()) {
            Some(Numeric::Number) => ITERATION,
            Some(_) => return Some((state, false)),
            None => {
                for (slot, keywords) in KEYWORDS {
                    if state & slot == 0
                        && keywords
                            .iter()
                            .any(|keyword| word.eq_ignore_ascii_case(keyword))
                    {
                        return Some((state | slot, false));
                    }
                }
                return (state & NAME == 0).then_some((state | NAME, true));
            }
        },
    };
    (state & slot == 0).then_some((state | slot, false))
}

/// The type of a math function's result, as the power of its time dimension.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Dimension {
    Exponent(i8),
    Other,
    Unknown,
}

fn sum(total: Option<Dimension>, term: Dimension) -> Dimension {
    match total {
        None => term,
        Some(Dimension::Unknown) => Dimension::Unknown,
        Some(_) if term == Dimension::Unknown => Dimension::Unknown,
        Some(total) if total == term => total,
        Some(_) => Dimension::Other,
    }
}

fn product(term: Dimension, factor: Dimension, divide: bool) -> Dimension {
    match (term, factor) {
        (Dimension::Unknown, _) | (_, Dimension::Unknown) => Dimension::Unknown,
        (Dimension::Exponent(term), Dimension::Exponent(factor)) => {
            Dimension::Exponent(if divide {
                term.saturating_sub(factor)
            } else {
                term.saturating_add(factor)
            })
        }
        _ => Dimension::Other,
    }
}

/// A sum of products, as in `calc()`.
fn calculation(nodes: &[ValueNode]) -> Dimension {
    let mut total = None;
    let mut term = Dimension::Exponent(0);
    let mut divide = false;
    for node in nodes {
        match node {
            ValueNode::Separator { value } => divide |= value.contains("/"),
            ValueNode::Function { name, nodes } => {
                let factor = math(name, nodes).unwrap_or(Dimension::Unknown);
                term = product(term, factor, std::mem::take(&mut divide));
            }
            ValueNode::Word { value } if value == "+" || value == "-" => {
                total = Some(sum(total, term));
                term = Dimension::Exponent(0);
            }
            ValueNode::Word { value } => {
                // `*` is not a value separator, so `10s*2` arrives as one word.
                for (index, factor) in value.units().split(|unit| *unit == 42).enumerate() {
                    if index > 0 {
                        divide = false;
                    }
                    if factor.is_empty() {
                        continue;
                    }
                    let factor = match numeric(factor) {
                        Some(Numeric::Number) => Dimension::Exponent(0),
                        Some(Numeric::Time) => Dimension::Exponent(1),
                        Some(Numeric::Other) => Dimension::Other,
                        None if ["e", "pi", "infinity", "-infinity", "nan"].iter().any(
                            |constant| {
                                Text::from_units(factor.to_vec()).eq_ignore_ascii_case(constant)
                            },
                        ) =>
                        {
                            Dimension::Exponent(0)
                        }
                        None => Dimension::Other,
                    };
                    term = product(term, factor, std::mem::take(&mut divide));
                }
            }
        }
    }
    sum(total, term)
}

/// The result type of a CSS math function, or `None` when `name` is not one.
fn math(name: &Text, nodes: &[ValueNode]) -> Option<Dimension> {
    let named = |names: &[&str]| names.iter().any(|known| name.eq_ignore_ascii_case(known));
    let arguments = || {
        nodes.split(|node| matches!(node, ValueNode::Separator { value } if value.contains(",")))
    };
    if name.units().is_empty() || named(&["calc", "-webkit-calc"]) {
        Some(calculation(nodes))
    } else if named(&["min", "max", "clamp", "hypot", "abs", "round", "mod", "rem"]) {
        let strategy = |argument: &[ValueNode]| {
            let words: Vec<_> = argument
                .iter()
                .filter(|node| !matches!(node, ValueNode::Separator { .. }))
                .collect();
            matches!(words.as_slice(), [ValueNode::Word { value }]
                if ["nearest", "up", "down", "to-zero"].iter().any(|keyword| value.eq_ignore_ascii_case(keyword)))
        };
        let mut result = None;
        for (index, argument) in arguments().enumerate() {
            if index == 0 && name.eq_ignore_ascii_case("round") && strategy(argument) {
                continue;
            }
            result = Some(sum(result, calculation(argument)));
        }
        Some(result.unwrap_or(Dimension::Other))
    } else if named(&["sign", "sin", "cos", "tan", "pow", "sqrt", "log", "exp"]) {
        Some(Dimension::Exponent(0))
    } else if named(&["asin", "acos", "atan", "atan2"]) {
        Some(Dimension::Other)
    } else {
        None
    }
}

/// The states after a non-var() function in the shorthand.
fn function_slot(name: &Text, nodes: &[ValueNode], states: States) -> States {
    if ["steps", "cubic-bezier", "linear"]
        .iter()
        .any(|easing| name.eq_ignore_ascii_case(easing))
    {
        return take(states, EASING);
    }
    match math(name, nodes) {
        Some(Dimension::Exponent(0)) => take(states, ITERATION),
        Some(Dimension::Unknown) => states | take(states, ITERATION),
        // A time, or a type no slot of the shorthand accepts.
        Some(_) => states,
        None => widen(states),
    }
}

/// How a value is substituted: outside animations, as an `animation-name` list,
/// or into the `animation` shorthand at a set of possible layer states.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Use {
    Plain,
    Name,
    Shorthand(States),
}

impl Use {
    fn join(self, other: Use) -> Use {
        match (self, other) {
            (Use::Shorthand(left), Use::Shorthand(right)) => Use::Shorthand(left | right),
            _ => self,
        }
    }
}

#[derive(Clone, Copy)]
enum Resolution {
    Undeclared,
    Cyclic,
    /// The version of the custom property this use needs, and the use after it.
    Version(usize, Use),
}

/// Upper bound on the distinct (custom property, use) pairs one stylesheet resolves.
const CONTEXTS: usize = 4096;

/// Decides keyframes references after var() substitution. A custom property is
/// expanded to each of its declarations in the stylesheet, and a word is renamed
/// only when it is the keyframes name in every layer state those values can
/// produce, including inside the property's own declarations. An internal property
/// whose uses need different renames gets one owner-private copy per extra version,
/// declared beside each of its declarations so that it cascades the same way;
/// version 0 stays in the original declaration. A public one is reported instead.
struct References<'a> {
    owner: &'a str,
    keyframes: &'a BTreeMap<Text, Text>,
    limit: usize,
    declarations: &'a [(Text, Vec<ValueNode>)],
    declared: BTreeMap<Text, Vec<usize>>,
    /// Custom properties whose values can reach `animation` or `animation-name`.
    animated: BTreeSet<Text>,
    /// Each property's distinct values, one per declaration, by version.
    versions: BTreeMap<Text, Vec<Vec<Vec<ValueNode>>>>,
    resolved: BTreeMap<(Text, Use), Resolution>,
    active: BTreeSet<Text>,
}

fn copy_name(owner: &str, name: &Text, version: usize) -> Text {
    if version == 0 {
        return name.clone();
    }
    let mut copy = Text::from(format!("--rustify-{owner}-k{version}-"));
    copy.push_text(&Text::from_units(name.units()[2..].to_vec()));
    copy
}

impl References<'_> {
    fn process(
        &mut self,
        nodes: &[ValueNode],
        mut usage: Use,
    ) -> Result<(Vec<ValueNode>, Use), CompileError> {
        let mut output = nodes.to_vec();
        for node in &mut output {
            match node {
                ValueNode::Word { value } => usage = self.word(value, usage)?,
                ValueNode::Function { name, nodes } if name.eq_ignore_ascii_case("var") => {
                    usage = self.reference(nodes, usage)?;
                }
                ValueNode::Function { name, nodes } => {
                    *nodes = self.process(nodes, Use::Plain)?.0;
                    if let Use::Shorthand(states) = usage {
                        usage = Use::Shorthand(function_slot(name, nodes, states));
                    }
                }
                ValueNode::Separator { value } => {
                    if let Use::Shorthand(states) = usage {
                        if states != 0 && value.contains(",") {
                            usage = Use::Shorthand(EMPTY);
                        }
                    }
                }
            }
        }
        Ok((output, usage))
    }

    fn word(&self, value: &mut Text, usage: Use) -> Result<Use, CompileError> {
        let target = self.keyframes.get(value);
        match usage {
            Use::Plain => {}
            Use::Name => {
                if let Some(target) = target {
                    *value = target.clone();
                }
            }
            Use::Shorthand(states) => {
                let mut next = 0;
                let mut roles = [false; 2];
                for state in 0..64u8 {
                    if states >> state & 1 == 1 {
                        if let Some((after, name)) = step(state, value) {
                            next |= 1 << after;
                            roles[usize::from(name)] = true;
                        }
                    }
                }
                if let Some(target) = target {
                    if roles == [true, true] {
                        return Err(CompileError::UnsupportedFeature {
                            feature: format!(
                                "keyframes name `{}` whose animation role depends on custom property values",
                                String::from_utf16_lossy(value.units())
                            ),
                        });
                    }
                    if roles[1] {
                        *value = target.clone();
                    }
                }
                return Ok(Use::Shorthand(next));
            }
        }
        Ok(usage)
    }

    /// A var() substitutes one of the property's declarations, or its fallback;
    /// without a fallback, a property this stylesheet never declares is unknown.
    fn reference(&mut self, nodes: &mut [ValueNode], usage: Use) -> Result<Use, CompileError> {
        if usage == Use::Shorthand(0) {
            return Ok(usage);
        }
        let fallback = nodes
            .iter()
            .position(|node| matches!(node, ValueNode::Separator { value } if value.contains(",")));
        let (head, tail) = nodes.split_at_mut(fallback.map_or(nodes.len(), |comma| comma + 1));
        let mut after = None;
        if let Some(name) = head.iter_mut().find_map(|node| match node {
            ValueNode::Word { value } => Some(value),
            _ => None,
        }) {
            if usage != Use::Plain || self.animated.contains(&*name) {
                match self.resolve(name, usage)? {
                    Resolution::Version(version, out) => {
                        *name = copy_name(self.owner, name, version);
                        after = Some(out);
                    }
                    Resolution::Cyclic => {}
                    Resolution::Undeclared if fallback.is_none() => {
                        after = Some(match usage {
                            Use::Shorthand(states) => Use::Shorthand(widen(states)),
                            _ => usage,
                        });
                    }
                    Resolution::Undeclared => {}
                }
            }
        }
        if fallback.is_some() {
            let (value, out) = self.process(tail, usage)?;
            for (slot, node) in tail.iter_mut().zip(value) {
                *slot = node;
            }
            after = Some(after.map_or(out, |after: Use| after.join(out)));
        }
        Ok(match usage {
            Use::Shorthand(_) => after.unwrap_or(Use::Shorthand(0)),
            _ => usage,
        })
    }

    fn resolve(&mut self, name: &Text, usage: Use) -> Result<Resolution, CompileError> {
        if let Some(found) = self.resolved.get(&(name.clone(), usage)) {
            return Ok(*found);
        }
        let Some(indices) = self.declared.get(name).cloned() else {
            return Ok(Resolution::Undeclared);
        };
        if self.active.contains(name) {
            return Ok(Resolution::Cyclic);
        }
        if self.active.len() >= self.limit {
            return Err(CompileError::Limit {
                resource: "ast_depth".into(),
                limit: self.limit,
            });
        }
        if self.resolved.len() >= CONTEXTS {
            return Err(CompileError::Limit {
                resource: "keyframes_contexts".into(),
                limit: CONTEXTS,
            });
        }
        self.active.insert(name.clone());
        let declarations = self.declarations;
        let mut values = Vec::with_capacity(indices.len());
        let mut out: Option<Use> = None;
        for index in indices {
            let (value, after) = self.process(&declarations[index].1, usage)?;
            values.push(value);
            out = Some(out.map_or(after, |out| out.join(after)));
        }
        self.active.remove(name);
        let versions = self.versions.entry(name.clone()).or_default();
        let version = match versions.iter().position(|known| *known == values) {
            Some(version) => version,
            None => {
                versions.push(values);
                versions.len() - 1
            }
        };
        let found = Resolution::Version(version, out.unwrap_or(usage));
        self.resolved.insert((name.clone(), usage), found);
        Ok(found)
    }
}

fn variable_references(nodes: &[ValueNode], names: &mut BTreeSet<Text>) {
    for node in nodes {
        if let ValueNode::Function { name, nodes } = node {
            if name.eq_ignore_ascii_case("var") {
                if let Some(value) = nodes.iter().find_map(|node| match node {
                    ValueNode::Word { value } => Some(value),
                    _ => None,
                }) {
                    names.insert(value.clone());
                }
            }
            variable_references(nodes, names);
        }
    }
}

fn collect_declarations(
    nodes: &[AstNode],
    limit: usize,
    declarations: &mut Vec<(Text, Vec<ValueNode>)>,
) -> Result<(), CompileError> {
    for node in nodes {
        if let AstNode::Declaration {
            property, value, ..
        } = node
        {
            declarations.push((property.clone(), value::parse_text(value, limit)?));
        }
        if let Some(children) = node.children() {
            collect_declarations(children, limit, declarations)?;
        }
    }
    Ok(())
}

/// Every declaration's final value in document order, before internal variables
/// are renamed, and the owner-private copies to declare after it.
struct Plan {
    values: Vec<Vec<ValueNode>>,
    copies: Vec<Vec<(Text, Vec<ValueNode>)>>,
    /// The copies of each custom property that has more than one version.
    copied: BTreeMap<Text, Vec<Text>>,
}

fn plan(
    nodes: &[AstNode],
    owner: &str,
    keyframes: &BTreeMap<Text, Text>,
    limit: usize,
) -> Result<Plan, CompileError> {
    let mut declarations = Vec::new();
    collect_declarations(nodes, limit, &mut declarations)?;
    let mut declared = BTreeMap::<Text, Vec<usize>>::new();
    let mut references = BTreeMap::<Text, BTreeSet<Text>>::new();
    let mut pending = Vec::new();
    for (index, (property, value)) in declarations.iter().enumerate() {
        let mut names = BTreeSet::new();
        variable_references(value, &mut names);
        if property.starts_with("--") {
            declared.entry(property.clone()).or_default().push(index);
            references
                .entry(property.clone())
                .or_default()
                .extend(names);
        } else if property == "animation" || property == "animation-name" {
            pending.extend(names);
        }
    }
    let mut animated = BTreeSet::new();
    while let Some(name) = pending.pop() {
        if animated.insert(name.clone()) {
            if let Some(next) = references.get(&name) {
                pending.extend(next.iter().cloned());
            }
        }
    }
    let mentions = |value: &[ValueNode]| {
        let mut names = BTreeSet::new();
        variable_references(value, &mut names);
        names.iter().any(|name| animated.contains(name))
    };
    let mut values: Vec<Option<Vec<ValueNode>>> = vec![None; declarations.len()];
    let mut resolver = References {
        owner,
        keyframes,
        limit,
        declarations: &declarations,
        declared,
        animated: animated.clone(),
        versions: BTreeMap::new(),
        resolved: BTreeMap::new(),
        active: BTreeSet::new(),
    };
    // Uses in document order: the first use of a property decides the value its own
    // declaration keeps.
    for (index, (property, value)) in declarations.iter().enumerate() {
        let usage = if property == "animation" {
            Use::Shorthand(EMPTY)
        } else if property == "animation-name" {
            Use::Name
        } else if property.starts_with("--") || !mentions(value) {
            continue;
        } else {
            Use::Plain
        };
        values[index] = Some(resolver.process(value, usage)?.0);
    }
    // A property no use reaches keeps the plain meaning of what it references.
    for (index, (property, value)) in declarations.iter().enumerate() {
        if property.starts_with("--")
            && !resolver.versions.contains_key(property)
            && mentions(value)
        {
            values[index] = Some(resolver.process(value, Use::Plain)?.0);
        }
    }
    let versions = std::mem::take(&mut resolver.versions);
    let declared = std::mem::take(&mut resolver.declared);
    let mut copies = vec![Vec::new(); declarations.len()];
    let mut copied = BTreeMap::new();
    for (name, versions) in versions {
        // A copy is fixed where the property is declared, so it cannot follow what
        // the application sets on a public property later; only internal ones get it.
        if versions.len() > 1 && !name.starts_with("--tw-") {
            return Err(CompileError::UnsupportedFeature {
                feature: format!(
                    "custom property `{}` used both as a keyframes name and with another meaning",
                    String::from_utf16_lossy(name.units())
                ),
            });
        }
        for (position, &index) in declared[&name].iter().enumerate() {
            values[index] = Some(versions[0][position].clone());
            copies[index] = (1..versions.len())
                .map(|version| {
                    (
                        copy_name(owner, &name, version),
                        versions[version][position].clone(),
                    )
                })
                .collect();
        }
        if versions.len() > 1 {
            let names = (1..versions.len())
                .map(|version| copy_name(owner, &name, version))
                .collect();
            copied.insert(name, names);
        }
    }
    let values = values
        .into_iter()
        .zip(declarations)
        .map(|(value, (_, parsed))| value.unwrap_or(parsed))
        .collect();
    Ok(Plan {
        values,
        copies,
        copied,
    })
}

struct Rewrite<'a> {
    owner: &'a str,
    keyframes: &'a BTreeMap<Text, Text>,
    plan: Plan,
    /// Document index of the next declaration.
    next: usize,
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

fn rewrite(
    nodes: &mut Vec<AstNode>,
    state: &mut Rewrite,
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
    let owner = state.owner;
    let mut local = Vec::with_capacity(nodes.len());
    for mut node in std::mem::take(nodes) {
        let mut global = false;
        let mut copies = Vec::new();
        let mut registered = Vec::new();
        match &mut node {
            AstNode::Declaration {
                property,
                value,
                important,
                ..
            } => {
                let index = state.next;
                state.next += 1;
                let mut parsed = std::mem::take(&mut state.plan.values[index]);
                rewrite_value(&mut parsed, owner);
                *value = value::to_css_text(&parsed);
                for (name, mut copy) in std::mem::take(&mut state.plan.copies[index]) {
                    rewrite_value(&mut copy, owner);
                    let mut declaration = AstNode::declaration(name, value::to_css_text(&copy));
                    if let AstNode::Declaration {
                        important: flag, ..
                    } = &mut declaration
                    {
                        *flag = *important;
                    }
                    copies.push(declaration);
                }
                if let Some(renamed) = internal_variable(property, owner) {
                    *property = renamed;
                }
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
                    if let Some(names) = state.plan.copied.get(params) {
                        registered.clone_from(names);
                    }
                    if let Some(replacement) = internal_variable(params, owner) {
                        *params = replacement;
                    } else {
                        return Err(CompileError::UnsupportedFeature {
                            feature: "runtime registration of public @property".into(),
                        });
                    }
                    global = true;
                } else if name == "@keyframes" {
                    *params = state.keyframes[params].clone();
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
            rewrite(children, state, globals, limit, depth + 1)?;
        }
        if global {
            for name in registered {
                let mut copy = node.clone();
                if let AstNode::AtRule { params, .. } = &mut copy {
                    *params = name;
                }
                globals.push(copy);
            }
            globals.push(node);
        } else {
            local.push(node);
            local.extend(copies);
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

    fn scoped(css: &str, owner: &str) -> Result<String, CompileError> {
        let ast = parser::parse(css, 32).unwrap();
        Ok(to_css(&stylesheet(&ast, owner, 32)?).to_utf8().unwrap())
    }

    #[test]
    fn a_public_property_whose_uses_need_different_names_is_reported() {
        for css in [
            ":root { --ease: linear; } @keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: 0.5; } } .motion { animation: card 10s var(--ease) infinite; } .other { animation-name: var(--ease); animation-duration: 10s; }",
            ":root { --v: linear; } @keyframes linear { to { opacity: 0.5; } } .motion { animation-name: var(--v); } .other { transition-timing-function: var(--v); }",
            ":root { --v: linear; } @keyframes linear { to { opacity: 0.5; } } .fade { transition-timing-function: var(--v); } .motion { animation-name: var(--v); }",
            ":root { --v: linear; --a: var(--v); } @keyframes linear { to { opacity: 0.3; } } .motion { animation-name: var(--a); } .other { transition-timing-function: var(--a); }",
        ] {
            assert!(
                matches!(scoped(css, "alpha"), Err(CompileError::UnsupportedFeature { feature }) if feature.contains("custom property")),
                "{css}"
            );
        }
        let css = scoped(":root { --v: linear; } @keyframes linear { to { opacity: 0.3; } } .motion { animation-name: var(--v); animation-duration: 10s; } .plain { color: var(--color); }", "alpha").unwrap();
        assert!(css.contains("--v: rustify-alpha-linear;"));
        assert!(css.contains("animation-name: var(--v);"));
        assert!(!css.contains("-k1-"));
    }

    #[test]
    fn an_internal_property_gets_a_copy_for_each_use() {
        let css = scoped("@keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: 0.5; } } .motion { --tw-ease: linear; animation: card 10s var(--tw-ease) infinite; } .other { animation-name: var(--tw-ease); } .fade { transition-timing-function: var(--tw-ease); }", "alpha").unwrap();
        assert!(css.contains("--rustify-alpha-tw-ease: linear;"));
        assert!(css.contains("--rustify-alpha-k1-tw-ease: rustify-alpha-linear;"));
        assert!(
            css.contains("animation: rustify-alpha-card 10s var(--rustify-alpha-tw-ease) infinite")
        );
        assert!(css.contains("animation-name: var(--rustify-alpha-k1-tw-ease)"));
        assert!(css.contains("transition-timing-function: var(--rustify-alpha-tw-ease)"));
    }

    #[test]
    fn substituted_values_keep_their_slots_in_the_whole_shorthand() {
        let css = scoped(":root { --x: linear; } @keyframes ease { to { opacity: 0.3; } } .motion { animation: var(--x) 10s ease infinite; }", "alpha").unwrap();
        assert!(css.contains("--x: linear;"));
        assert!(css.contains("animation: var(--x) 10s rustify-alpha-ease infinite"));
        let css = scoped(":root { --name: linear; } @keyframes linear { to { opacity: 0.3; } } .motion { animation: 10s ease var(--name) infinite; }", "alpha").unwrap();
        assert!(css.contains("--name: rustify-alpha-linear;"));
        assert!(css.contains("animation: 10s ease var(--name) infinite"));
        let css = scoped(":root { --x: ease; } @keyframes linear { to { opacity: 0.3; } } .motion { animation: var(--x, linear) 10s linear infinite; }", "alpha").unwrap();
        assert!(css.contains("animation: var(--x, linear) 10s rustify-alpha-linear infinite"));
        let css = scoped("@keyframes linear { to { opacity: 0.3; } } .motion { --tw-a: var(--tw-b) 1s; --tw-b: ease-in; animation: var(--tw-a) linear; }", "alpha").unwrap();
        assert!(css.contains("animation: var(--rustify-alpha-tw-a) rustify-alpha-linear"));
    }

    #[test]
    fn math_times_and_numeric_iterations_take_their_own_slots() {
        for time in [
            "calc(10s)",
            "min(10s, 12s)",
            "calc(2 * 5s)",
            "clamp(1s, 10s, 20s)",
        ] {
            let css = scoped(&format!("@keyframes card {{ to {{ opacity: 0.3; }} }} .motion {{ animation: {time} linear card infinite; }}"), "alpha").unwrap();
            assert!(
                css.contains(&format!(
                    "animation: {time} linear rustify-alpha-card infinite"
                )),
                "{time}: {css}"
            );
        }
        for count in ["2", "0.5", ".5", "1e1", "calc(10s / 5s)", "sign(1s)"] {
            let css = scoped(&format!("@keyframes infinite {{ to {{ opacity: 0.3; }} }} .motion {{ animation: {count} 10s infinite linear; }}"), "alpha").unwrap();
            assert!(
                css.contains(&format!(
                    "animation: {count} 10s rustify-alpha-infinite linear"
                )),
                "{count}: {css}"
            );
        }
        let css = scoped("@keyframes linear { to { opacity: 0.3; } } .motion { animation: steps(4, end) 1s linear, cubic-bezier(0, 0, 1, 1) linear, linear(0, 1) linear, linear 1s linear; }", "alpha").unwrap();
        assert!(css.contains("animation: steps(4, end) 1s rustify-alpha-linear, cubic-bezier(0, 0, 1, 1) rustify-alpha-linear, linear(0, 1) rustify-alpha-linear, linear 1s rustify-alpha-linear"));
    }

    #[test]
    fn a_name_whose_role_depends_on_the_cascade_is_reported() {
        for css in [
            ":root { --x: linear; } .dark { --x: card; } @keyframes ease { to { opacity: 0; } } @keyframes card { to { opacity: 1; } } .motion { animation: var(--x) 1s ease; }",
            "@keyframes linear { to { opacity: 0; } } .motion { animation: var(--external) 1s linear; }",
        ] {
            assert!(matches!(scoped(css, "alpha"), Err(CompileError::UnsupportedFeature { .. })), "{css}");
        }
        let css = scoped("@keyframes card { to { opacity: 0; } } .motion { animation: var(--external) 1s card; } .cycle { --a: var(--b); --b: var(--a); animation: var(--a, card 1s); }", "alpha").unwrap();
        assert!(css.contains("animation: var(--external) 1s rustify-alpha-card"));
        assert!(css.contains("animation: var(--a, rustify-alpha-card 1s)"));
    }

    #[test]
    fn registered_internal_properties_register_their_copies() {
        let css = scoped("@property --tw-ease { syntax: \"*\"; inherits: false; } @keyframes linear { to { opacity: 0; } } .motion { --tw-ease: linear !important; animation: card 1s var(--tw-ease); } .other { animation-name: var(--tw-ease); }", "alpha").unwrap();
        assert!(css.contains("@property --rustify-alpha-tw-ease"));
        assert!(css.contains("@property --rustify-alpha-k1-tw-ease"));
        assert!(css.contains("--rustify-alpha-tw-ease: linear !important;"));
        assert!(css.contains("--rustify-alpha-k1-tw-ease: rustify-alpha-linear !important;"));
        assert!(css.contains("animation-name: var(--rustify-alpha-k1-tw-ease)"));
    }
}
