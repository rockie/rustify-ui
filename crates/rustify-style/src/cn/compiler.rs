//! Compilation of ordered cn configurations into the upstream table format.

use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

pub use super::emit::{compile_to_source, compile_utf16_model_to_source};
use super::{CnError, Config, Tables, Validator};
use super::{Utf16ClassGroupDef, Utf16Config, Utf16Tables, Utf16Validator};

/// Compiled tables and custom callback implementations, kept in one owner.
pub struct CompiledTables {
    pub tables: Tables,
    pub validator_impls: Vec<Validator>,
    pub prefix: Option<String>,
}

/// Table compilation model, including the factored literal sets used by emission.
pub struct CompiledModel {
    pub compiled: CompiledTables,
    pub sets: Vec<Vec<String>>,
    pub attachments: Vec<(usize, usize, usize)>,
}

/// Observable size counters from the table compiler.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileStats {
    pub groups: usize,
    pub nodes: usize,
    pub edges: usize,
    pub lifted_literals: usize,
    pub unique_tails: usize,
    pub vlists: usize,
}

pub fn compile_stats(config: &Config) -> Result<CompileStats, CnError> {
    let model = compile_model(config)?;
    let tables = &model.compiled.tables;
    Ok(CompileStats {
        groups: tables.group_count,
        nodes: tables.node_group.len(),
        edges: tables.edge_target.len(),
        lifted_literals: tables.lit_anchor.len(),
        unique_tails: tables.pool_offsets.len() / 2,
        vlists: tables.vlist_ref.len(),
    })
}

pub fn compile_to_tables(config: &Config) -> Result<CompiledTables, CnError> {
    Ok(compile_model(config)?.compiled)
}

#[derive(Default)]
struct PartNode {
    next: Vec<(Vec<u16>, PartNode)>,
    depth: usize,
    group: Option<usize>,
    validators: Vec<(usize, usize)>,
    literals: Vec<(Vec<u16>, usize)>,
}

impl PartNode {
    fn path(&mut self, path: &[u16], limit: Option<usize>) -> Result<&mut Self, CnError> {
        let mut node = self;
        for part in path.split(|c| *c == 45) {
            if limit.is_some_and(|limit| node.depth >= limit) {
                return Err(CnError::new("cn: input limit: class path depth exceeded"));
            }
            let index = node
                .next
                .iter()
                .position(|(key, _)| key == part)
                .unwrap_or(node.next.len());
            if index == node.next.len() {
                node.next.push((
                    part.to_vec(),
                    Self {
                        depth: node.depth + 1,
                        ..Self::default()
                    },
                ));
            }
            node = &mut node.next[index].1;
        }
        Ok(node)
    }

    fn liftable(&self) -> bool {
        self.validators.is_empty() && self.next.iter().all(|(_, child)| child.liftable())
    }

    fn collect(&self, prefix: &[u16], out: &mut Vec<(Vec<u16>, usize)>) {
        if let Some(gid) = self.group {
            out.push((prefix.to_vec(), gid));
        }
        for (part, child) in &self.next {
            let mut joined = prefix.to_vec();
            joined.push(45);
            joined.extend_from_slice(part);
            child.collect(&joined, out);
        }
    }

    fn prune(&mut self) {
        let mut retained = Vec::new();
        for (part, mut child) in std::mem::take(&mut self.next) {
            if child.liftable() {
                child.collect(&part, &mut self.literals);
            } else {
                child.prune();
                retained.push((part, child));
            }
        }
        self.next = retained;
    }
}

fn process(
    definition: &Utf16ClassGroupDef,
    node: &mut PartNode,
    gid: usize,
    config: &Utf16Config,
    callbacks: &mut Vec<Utf16Validator>,
    themes: &mut Vec<Vec<u16>>,
    budget: &mut CompilationBudget,
) -> Result<(), CnError> {
    budget.enter()?;
    let result = (|| {
        match definition {
            Utf16ClassGroupDef::Literal(literal) => {
                let node = if literal.is_empty() {
                    node
                } else {
                    node.path(literal, budget.limits.map(|limits| limits.0))?
                };
                node.group = Some(gid);
            }
            Utf16ClassGroupDef::Object(entries) => {
                for (key, definitions) in entries {
                    let child = node.path(key, budget.limits.map(|limits| limits.0))?;
                    for definition in definitions {
                        process(definition, child, gid, config, callbacks, themes, budget)?;
                    }
                }
            }
            Utf16ClassGroupDef::Builtin(op) => {
                if *op >= super::validators::NAMES.len() {
                    return Err(CnError::invalid_config("unknown validator opcode"));
                }
                node.validators.push((*op, gid));
            }
            Utf16ClassGroupDef::Theme(key) => {
                if themes.contains(key) {
                    return Err(CnError::invalid_config("cyclic theme scale"));
                }
                themes.push(key.clone());
                if let Some((_, definitions)) = config.theme.iter().find(|(name, _)| name == key) {
                    for definition in definitions {
                        process(definition, node, gid, config, callbacks, themes, budget)?;
                    }
                }
                themes.pop();
            }
            Utf16ClassGroupDef::Validator(callback) => {
                let index = callbacks
                    .iter()
                    .position(|existing| Rc::ptr_eq(existing, callback))
                    .unwrap_or(callbacks.len());
                if index == callbacks.len() {
                    callbacks.push(callback.clone());
                }
                node.validators.push((25 + index, gid));
            }
            Utf16ClassGroupDef::Deferred(callback) => {
                process(&callback()?, node, gid, config, callbacks, themes, budget)?;
            }
            Utf16ClassGroupDef::Sequence(callback) => {
                let pointer = Rc::as_ptr(callback) as *const () as usize;
                if budget.getters.contains(&pointer) {
                    return Err(CnError::invalid_config("cyclic definition sequence"));
                }
                budget.getters.push(pointer);
                let result = (|| {
                    for definition in callback()? {
                        process(&definition?, node, gid, config, callbacks, themes, budget)?;
                    }
                    Ok(())
                })();
                budget.getters.pop();
                result?;
            }
            Utf16ClassGroupDef::ThemeGetter(callback) => {
                let pointer = Rc::as_ptr(callback) as *const () as usize;
                if budget.getters.contains(&pointer) {
                    return Err(CnError::invalid_config("cyclic theme getter"));
                }
                budget.getters.push(pointer);
                let result = (|| {
                    for definition in callback(&config.theme)? {
                        process(&definition, node, gid, config, callbacks, themes, budget)?;
                    }
                    Ok(())
                })();
                budget.getters.pop();
                result?;
            }
        }
        Ok(())
    })();
    budget.depth -= 1;
    result
}

#[derive(Default)]
struct CharNode {
    edges: BTreeMap<u16, usize>,
    group: Option<usize>,
    validators: Option<usize>,
    literal_anchor: bool,
}

fn insert_chars(
    nodes: &mut Vec<CharNode>,
    mut at: usize,
    text: &[u16],
    limit: Option<usize>,
) -> Result<usize, CnError> {
    for &c in text {
        if let Some(next) = nodes[at].edges.get(&c) {
            at = *next;
        } else {
            let next = nodes.len();
            if limit.is_some_and(|limit| next >= limit) {
                return Err(CnError::new(
                    "cn: input limit: compiled node count exceeded",
                ));
            }
            nodes.push(CharNode::default());
            nodes[at].edges.insert(c, next);
            at = next;
        }
    }
    Ok(at)
}

type Literal = (usize, Vec<u16>, usize);
type ValidatorList = Vec<(usize, usize)>;

fn flatten(
    part: &PartNode,
    at: usize,
    root: bool,
    nodes: &mut Vec<CharNode>,
    lists: &mut Vec<ValidatorList>,
    literals: &mut Vec<Literal>,
    limit: Option<usize>,
) -> Result<(), CnError> {
    nodes[at].group = part.group;
    if !part.validators.is_empty() {
        let index = lists
            .iter()
            .position(|list| list == &part.validators)
            .unwrap_or(lists.len());
        if index == lists.len() {
            lists.push(part.validators.clone());
        }
        nodes[at].validators = Some(index);
    }
    for (tail, gid) in &part.literals {
        literals.push((at, tail.clone(), *gid));
        nodes[at].literal_anchor = true;
    }
    for (key, child) in &part.next {
        let entry = if root {
            at
        } else {
            insert_chars(nodes, at, &[45], limit)?
        };
        let child_at = insert_chars(nodes, entry, key, limit)?;
        flatten(child, child_at, false, nodes, lists, literals, limit)?;
    }
    Ok(())
}

struct RadixNode {
    edges: Vec<(Vec<u16>, usize)>,
    group: Option<usize>,
    validators: Option<usize>,
}

fn radix(
    old: usize,
    nodes: &[CharNode],
    output: &mut Vec<RadixNode>,
    remap: &mut [usize],
    depth: usize,
    limit: Option<usize>,
) -> Result<(), CnError> {
    if limit.is_some_and(|limit| depth > limit) {
        return Err(CnError::new("cn: input limit: radix depth exceeded"));
    }
    let new = output.len();
    remap[old] = new;
    let mut edges = Vec::new();
    for (&c, &target) in &nodes[old].edges {
        let mut label = vec![c];
        let mut at = target;
        while nodes[at].edges.len() == 1
            && nodes[at].group.is_none()
            && nodes[at].validators.is_none()
            && !nodes[at].literal_anchor
        {
            if let Some((&c, &target)) = nodes[at].edges.first_key_value() {
                label.push(c);
                at = target;
            }
        }
        edges.push((label, at));
    }
    output.push(RadixNode {
        edges,
        group: nodes[old].group,
        validators: nodes[old].validators,
    });
    let children: Vec<_> = output[new]
        .edges
        .iter()
        .map(|(_, target)| *target)
        .collect();
    for child in children {
        radix(child, nodes, output, remap, depth + 1, limit)?;
    }
    Ok(())
}

fn prefix_sums(counts: impl IntoIterator<Item = usize>) -> Vec<i32> {
    let mut output = vec![0];
    let mut sum = 0;
    for count in counts {
        sum += count as i32;
        output.push(sum);
    }
    output
}

fn intern_group(names: &mut Vec<Vec<u16>>, name: &[u16]) -> usize {
    let index = names
        .iter()
        .position(|known| known == name)
        .unwrap_or(names.len());
    if index == names.len() {
        names.push(name.to_vec());
    }
    index
}

/// Compile the full configuration, including native validator/theme callbacks.
pub fn compile_utf16_model(
    config: &Utf16Config,
    limits: Option<(usize, usize)>,
    hooks: Option<&Utf16CompileHooks>,
) -> Result<Utf16CompiledModel, CnError> {
    let mut budget = CompilationBudget {
        limits,
        ..CompilationBudget::default()
    };
    let mut root = PartNode::default();
    let mut names = Vec::new();
    let mut callbacks = Vec::new();
    for (name, definitions) in &config.class_groups {
        let gid = intern_group(&mut names, name);
        for definition in definitions {
            process(
                definition,
                &mut root,
                gid,
                config,
                &mut callbacks,
                &mut Vec::new(),
                &mut budget,
            )?;
        }
    }
    root.prune();
    let mut nodes = vec![CharNode::default()];
    let mut lists = Vec::new();
    let mut literals = Vec::new();
    flatten(
        &root,
        0,
        true,
        &mut nodes,
        &mut lists,
        &mut literals,
        limits.map(|limits| limits.1),
    )?;
    let mut node_remap = vec![0; nodes.len()];
    let mut radix_nodes = Vec::new();
    radix(
        0,
        &nodes,
        &mut radix_nodes,
        &mut node_remap,
        0,
        limits.map(|limits| limits.0),
    )?;
    let mut list_remap = HashMap::new();
    let mut reordered_lists = Vec::new();
    for node in &mut radix_nodes {
        if let Some(old) = node.validators {
            let next = reordered_lists.len();
            let new = *list_remap.entry(old).or_insert_with(|| {
                reordered_lists.push(lists[old].clone());
                next
            });
            node.validators = Some(new);
        }
    }
    for literal in &mut literals {
        literal.0 = node_remap[literal.0];
    }
    literals.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.2.cmp(&b.2))
            .then_with(|| a.1.cmp(&b.1))
    });
    let mut attachments: Vec<(usize, usize, usize)> = Vec::new();
    let mut sets: Vec<Vec<Vec<u16>>> = Vec::new();
    let mut set_owners = Vec::new();
    let mut at = 0;
    while at < literals.len() {
        let (anchor, _, gid) = &literals[at];
        let mut end = at + 1;
        while end < literals.len() && literals[end].0 == *anchor && literals[end].2 == *gid {
            end += 1;
        }
        let tails: Vec<Vec<u16>> = literals[at..end]
            .iter()
            .map(|(_, tail, _)| tail.clone())
            .collect();
        for tail in &tails {
            if tail.contains(&124) || tail.contains(&32) {
                return Err(CnError::new(format!(
                    "cn compiler: tail contains delimiter: {}",
                    String::from_utf16_lossy(tail)
                )));
            }
        }
        let index = sets
            .iter()
            .position(|set| set == &tails)
            .unwrap_or(sets.len());
        if index == sets.len() {
            set_owners.push((*anchor, *gid));
            sets.push(tails);
        }
        attachments.push((*anchor, *gid, index));
        at = end;
    }
    attachments.sort_by_key(|(anchor, _, set)| (*anchor, *set));
    let attachment_owns_set = attachments
        .iter()
        .map(|(anchor, gid, set)| set_owners[*set] == (*anchor, *gid))
        .collect();
    let targets = if let Some(hooks) = hooks {
        (hooks.targets)()?
    } else {
        config
            .conflicting_class_groups
            .iter()
            .chain(&config.conflicting_class_group_modifiers)
            .flat_map(|(_, targets)| targets.iter().cloned())
            .chain(config.postfix_lookup_class_groups.iter().cloned())
            .collect()
    };
    for target in &targets {
        intern_group(&mut names, target);
    }
    let mut group_remap = vec![None; names.len()];
    let mut next_group = 0;
    let mut renumber = |old: usize| -> usize {
        *group_remap[old].get_or_insert_with(|| {
            let next = next_group;
            next_group += 1;
            next
        })
    };
    for list in &mut reordered_lists {
        for (_, gid) in list {
            *gid = renumber(*gid);
        }
    }
    for (_, gid, _) in &mut attachments {
        *gid = renumber(*gid);
    }
    for node in &mut radix_nodes {
        if let Some(gid) = node.group {
            node.group = Some(renumber(gid));
        }
    }
    for old in 0..names.len() {
        renumber(old);
    }
    let mut new_names: Vec<&[u16]> = vec![&[]; names.len()];
    let mut group_ids = HashMap::new();
    for (old, name) in names.iter().enumerate() {
        let new = group_remap[old].unwrap_or(0);
        new_names[new] = name;
        group_ids.insert(name.as_slice(), new as i32);
    }
    let hook_ids = hooks.map(|_| {
        Rc::new(
            group_ids
                .iter()
                .map(|(key, value)| (key.to_vec(), *value))
                .collect::<HashMap<_, _>>(),
        )
    });
    let mut adj_gid = Vec::new();
    let mut adj_counts = Vec::new();
    let mut adj_tgt = Vec::new();
    let mut pat_gid = Vec::new();
    let mut pat_tgt = Vec::new();
    for (gid, name) in new_names.iter().enumerate() {
        if let Some(hooks) = hooks {
            (hooks.prepare_conflicts)(name, hook_ids.as_ref().unwrap().clone())?;
        }
        for modifiers in [false, true] {
            let targets = if let Some(hooks) = hooks {
                (hooks.conflicts)(name, modifiers, hook_ids.as_ref().unwrap().clone())?
            } else {
                let groups = if modifiers {
                    &config.conflicting_class_group_modifiers
                } else {
                    &config.conflicting_class_groups
                };
                groups
                    .iter()
                    .find(|(key, _)| key == name)
                    .map(|(_, targets)| {
                        targets
                            .iter()
                            .map(|target| group_ids.get(target.as_slice()).copied().unwrap_or(0))
                            .collect()
                    })
                    .unwrap_or_default()
            };
            if modifiers {
                for target in targets {
                    pat_gid.push(gid as i32);
                    pat_tgt.push(target);
                }
            } else if !targets.is_empty() {
                adj_gid.push(gid as i32);
                adj_counts.push(targets.len());
                adj_tgt.extend(targets);
            }
        }
    }
    let postfix_lookup = if let Some(hooks) = hooks {
        (hooks.postfix)(hook_ids.as_ref().unwrap().clone())?
    } else {
        config
            .postfix_lookup_class_groups
            .iter()
            .map(|name| group_ids[name.as_slice()])
            .collect()
    };
    let mut custom_order = Vec::new();
    let mut patterns: Vec<Vec<i32>> = Vec::new();
    let mut list_refs = Vec::new();
    let mut list_groups = Vec::new();
    for list in &reordered_lists {
        let mut ops = Vec::new();
        for &(op, gid) in list {
            let opcode = if op >= 25 {
                let old = op - 25;
                let new = custom_order
                    .iter()
                    .position(|known| *known == old)
                    .unwrap_or(custom_order.len());
                if new == custom_order.len() {
                    custom_order.push(old);
                }
                25 + new
            } else {
                op
            };
            ops.push(opcode as i32);
            list_groups.push(gid as i32);
        }
        let reference = patterns
            .iter()
            .position(|pattern| pattern == &ops)
            .unwrap_or(patterns.len());
        if reference == patterns.len() {
            patterns.push(ops);
        }
        list_refs.push(reference as i32);
    }
    let mut pool_text = Vec::new();
    let mut pool_length = 0;
    let mut pool_offsets = Vec::new();
    let mut tail_ids = HashMap::new();
    let mut lit_anchor = Vec::new();
    let mut lit_group = Vec::new();
    let mut lit_pool = Vec::new();
    for (anchor, gid, set) in &attachments {
        for tail in &sets[*set] {
            let next = tail_ids.len();
            let reference = *tail_ids.entry(tail.as_slice()).or_insert_with(|| {
                let length = tail.len();
                pool_offsets.extend([pool_length as i32, length as i32]);
                pool_length += length;
                pool_text.extend_from_slice(tail);
                next
            });
            lit_anchor.push(*anchor as i32);
            lit_group.push(*gid as i32);
            lit_pool.push(reference as i32);
        }
    }
    let mut label_text = Vec::new();
    let mut edge_target = Vec::new();
    let mut label_counts = Vec::new();
    for node in &radix_nodes {
        for (label, target) in &node.edges {
            label_counts.push(label.len());
            label_text.extend(label);
            edge_target.push(node_remap[*target] as i32);
        }
    }
    let (order_sensitive_modifiers, prefix) = if let Some(hooks) = hooks {
        (hooks.finish)()?
    } else {
        (
            config.order_sensitive_modifiers.clone(),
            config.prefix.clone(),
        )
    };
    let tables = Utf16Tables {
        group_count: names.len(),
        edge_start: prefix_sums(radix_nodes.iter().map(|node| node.edges.len())),
        label_start: prefix_sums(label_counts),
        label_text,
        edge_target,
        node_group: radix_nodes
            .iter()
            .map(|node| node.group.map_or(-1, |gid| gid as i32))
            .collect(),
        node_vlist: radix_nodes
            .iter()
            .map(|node| node.validators.map_or(-1, |list| list as i32))
            .collect(),
        vlist_pat: prefix_sums(patterns.iter().map(Vec::len)),
        vlist_ops: patterns.into_iter().flatten().collect(),
        vlist_ref: list_refs,
        vlist_group: list_groups,
        lit_anchor,
        lit_group,
        lit_pool,
        pool_offsets,
        pool_text,
        adj_gid,
        adj_start: prefix_sums(adj_counts),
        adj_tgt,
        pat_gid,
        pat_tgt,
        postfix_lookup_groups: postfix_lookup,
        custom_validator_names: custom_order
            .iter()
            .map(|old| format!("$c{old}").encode_utf16().collect())
            .collect(),
        order_sensitive_modifiers: order_sensitive_modifiers.join(&[32][..]),
        prefix: None,
    };
    Ok(Utf16CompiledModel {
        compiled: Utf16CompiledTables {
            tables,
            validator_impls: custom_order
                .iter()
                .map(|old| callbacks[*old].clone())
                .collect(),
            prefix: prefix.clone(),
        },
        sets,
        attachments,
        literals,
        attachment_owns_set,
    })
}

/// A project-fitted configuration; unseen groups may pass through unmerged.
pub struct SubsetResult {
    pub config: Config,
    pub used_groups: usize,
    pub total_groups: usize,
}

impl PartNode {
    fn classify(
        &self,
        parts: &[&[u16]],
        at: usize,
        callbacks: &[Utf16Validator],
    ) -> Result<Option<usize>, CnError> {
        if at == parts.len() {
            return Ok(self.group);
        }
        if let Some((_, child)) = self.next.iter().find(|(key, _)| key == parts[at]) {
            if let Some(gid) = child.classify(parts, at + 1, callbacks)? {
                return Ok(Some(gid));
            }
        }
        let rest = parts[at..].join(&[45][..]);
        for &(op, gid) in &self.validators {
            let matched = if op < 25 {
                super::validators::test(op, &String::from_utf16_lossy(&rest))
            } else {
                callbacks[op - 25](&rest)?
            };
            if matched {
                return Ok(Some(gid));
            }
        }
        Ok(None)
    }
}

/// Classify a complete token corpus and retain only groups it reaches.
pub fn subset_config<'a>(
    base: &Config,
    tokens: impl IntoIterator<Item = &'a str>,
) -> Result<SubsetResult, CnError> {
    let mut budget = CompilationBudget::default();
    let units = Utf16Config::from(base.clone());
    let mut root = PartNode::default();
    let mut callbacks = Vec::new();
    for (gid, (_, definitions)) in units.class_groups.iter().enumerate() {
        for definition in definitions {
            process(
                definition,
                &mut root,
                gid,
                &units,
                &mut callbacks,
                &mut Vec::new(),
                &mut budget,
            )?;
        }
    }
    let prefix = base
        .prefix
        .as_ref()
        .filter(|p| !p.is_empty())
        .map(|prefix| format!("{prefix}:"));
    let mut reached = std::collections::HashSet::new();
    for token in tokens {
        let token = if let Some(prefix) = &prefix {
            let Some(token) = token.strip_prefix(prefix) else {
                continue;
            };
            token
        } else {
            token
        };
        if token.is_empty() {
            continue;
        }
        let (colon, slash) = super::engine::separators(token);
        let start = colon.map_or(0, |at| at + 1);
        let bare = &token[start..];
        let bare = if let Some(bare) = bare.strip_suffix('!') {
            bare
        } else {
            bare.strip_prefix('!').unwrap_or(bare)
        };
        let mut candidates = vec![bare];
        if let Some(slash) = slash.filter(|slash| colon.is_none_or(|colon| *slash > colon)) {
            candidates.push(
                token[start..slash]
                    .strip_prefix('!')
                    .unwrap_or(&token[start..slash]),
            );
        }
        for candidate in candidates {
            if candidate.starts_with('[') && candidate.ends_with(']') {
                continue;
            }
            let candidate: Vec<_> = candidate.encode_utf16().collect();
            let parts: Vec<&[u16]> = candidate.split(|c| *c == 45).collect();
            let skip = usize::from(parts.len() > 1 && parts[0].is_empty());
            if let Some(gid) = root.classify(&parts, skip, &callbacks)? {
                reached.insert(base.class_groups[gid].0.as_str());
            }
        }
    }
    let mut config = base.clone();
    config
        .class_groups
        .retain(|(name, _)| reached.contains(name.as_str()));
    config
        .conflicting_class_groups
        .retain(|(name, _)| reached.contains(name.as_str()));
    config
        .conflicting_class_group_modifiers
        .retain(|(name, _)| reached.contains(name.as_str()));
    Ok(SubsetResult {
        config,
        used_groups: reached.len(),
        total_groups: base.class_groups.len(),
    })
}

/// Lossless fitted configuration; callbacks remain in their original owner.
pub struct Utf16SubsetResult {
    pub config: Utf16Config,
    pub used_groups: usize,
    pub total_groups: usize,
}
/// Fit UTF-16 tokens before literal lifting. Prefix is read after definitions.
pub fn subset_utf16_config_bounded(
    base: &Utf16Config,
    tokens: &[Vec<u16>],
    max_depth: usize,
    max_nodes: usize,
    prefix: impl FnOnce() -> Result<Option<Vec<u16>>, CnError>,
) -> Result<Utf16SubsetResult, CnError> {
    let mut budget = CompilationBudget {
        limits: Some((max_depth, max_nodes)),
        ..CompilationBudget::default()
    };
    let mut root = PartNode::default();
    let mut callbacks = Vec::new();
    for (gid, (_, definitions)) in base.class_groups.iter().enumerate() {
        for definition in definitions {
            process(
                definition,
                &mut root,
                gid,
                base,
                &mut callbacks,
                &mut Vec::new(),
                &mut budget,
            )?;
        }
    }
    let prefix = prefix()?.filter(|p| !p.is_empty()).map(|mut p| {
        p.push(58);
        p
    });
    let mut reached = std::collections::HashSet::new();
    for token in tokens {
        let token = if let Some(prefix) = &prefix {
            let Some(token) = token.strip_prefix(prefix.as_slice()) else {
                continue;
            };
            token
        } else {
            token.as_slice()
        };
        if token.is_empty() {
            continue;
        }
        let (colon, slash) = super::engine::separators_units(token);
        let start = colon.map_or(0, |at| at + 1);
        let bare = &token[start..];
        let bare = bare
            .strip_suffix(&[33])
            .unwrap_or_else(|| bare.strip_prefix(&[33]).unwrap_or(bare));
        let mut candidates = vec![bare];
        if let Some(slash) = slash.filter(|slash| colon.is_none_or(|colon| *slash > colon)) {
            candidates.push(
                token[start..slash]
                    .strip_prefix(&[33])
                    .unwrap_or(&token[start..slash]),
            );
        }
        for candidate in candidates {
            if candidate.first() == Some(&91) && candidate.last() == Some(&93) {
                continue;
            }
            let parts: Vec<&[u16]> = candidate.split(|c| *c == 45).collect();
            let skip = usize::from(parts.len() > 1 && parts[0].is_empty());
            if let Some(gid) = root.classify(&parts, skip, &callbacks)? {
                reached.insert(base.class_groups[gid].0.clone());
            }
        }
    }
    let mut config = base.clone();
    config
        .class_groups
        .retain(|(name, _)| reached.contains(name));
    config
        .conflicting_class_groups
        .retain(|(name, _)| reached.contains(name));
    config
        .conflicting_class_group_modifiers
        .retain(|(name, _)| reached.contains(name));
    Ok(Utf16SubsetResult {
        config,
        used_groups: reached.len(),
        total_groups: base.class_groups.len(),
    })
}

/// Lossless compiled configuration and its realm-local callback handles.
pub struct Utf16CompiledTables {
    pub tables: Utf16Tables,
    pub validator_impls: Vec<Utf16Validator>,
    pub prefix: Option<Vec<u16>>,
}
pub struct Utf16CompiledModel {
    pub compiled: Utf16CompiledTables,
    pub sets: Vec<Vec<Vec<u16>>>,
    pub attachments: Vec<(usize, usize, usize)>,
    /// Original sorted literal entries retain their pre-renumber group IDs.
    pub literals: Vec<(usize, Vec<u16>, usize)>,
    /// The first attachment for a set shares its tails array with that set.
    pub attachment_owns_set: Vec<bool>,
}
pub fn compile_utf16(config: &Utf16Config) -> Result<Utf16CompiledTables, CnError> {
    Ok(compile_utf16_model(config, None, None)?.compiled)
}

/// Native adapter around the same code-unit compiler used by JS.
pub fn compile_model(config: &Config) -> Result<CompiledModel, CnError> {
    let model = compile_utf16_model(&Utf16Config::from(config.clone()), None, None)?;
    let callbacks = model
        .compiled
        .validator_impls
        .into_iter()
        .map(|callback| {
            Rc::new(move |value: &str| callback(&value.encode_utf16().collect::<Vec<_>>()))
                as Validator
        })
        .collect();
    let text = |units: Vec<u16>| {
        String::from_utf16(&units)
            .map_err(|_| CnError::invalid_config("unpaired UTF-16 surrogate in native tables"))
    };
    Ok(CompiledModel {
        compiled: CompiledTables {
            tables: model.compiled.tables.try_into()?,
            validator_impls: callbacks,
            prefix: model.compiled.prefix.map(text).transpose()?,
        },
        sets: model
            .sets
            .into_iter()
            .map(|set| set.into_iter().map(text).collect())
            .collect::<Result<_, _>>()?,
        attachments: model.attachments,
    })
}

#[derive(Default)]
struct CompilationBudget {
    limits: Option<(usize, usize)>,
    depth: usize,
    nodes: usize,
    getters: Vec<usize>,
}
impl CompilationBudget {
    fn enter(&mut self) -> Result<(), CnError> {
        self.nodes = self.nodes.saturating_add(1);
        if self
            .limits
            .is_some_and(|(depth, nodes)| self.depth >= depth || self.nodes > nodes)
        {
            return Err(CnError::new(
                "cn: input limit: configuration expansion exceeded",
            ));
        }
        self.depth += 1;
        Ok(())
    }
}
/// Compile dynamically expanded JS definitions within explicit caller limits.
pub fn compile_utf16_bounded(
    config: &Utf16Config,
    max_depth: usize,
    max_nodes: usize,
) -> Result<Utf16CompiledTables, CnError> {
    if max_depth == 0 || max_nodes == 0 {
        return Err(CnError::invalid_config("compile limits must be positive"));
    }
    Ok(compile_utf16_model(config, Some((max_depth, max_nodes)), None)?.compiled)
}

pub type Utf16GroupIds = Rc<HashMap<Vec<u16>, i32>>;
pub type Utf16ConflictPreparation = Rc<dyn Fn(&[u16], Utf16GroupIds) -> Result<(), CnError>>;
pub type Utf16ConflictReader = Rc<dyn Fn(&[u16], bool, Utf16GroupIds) -> Result<Vec<i32>, CnError>>;
pub type Utf16FinishReader = Rc<dyn Fn() -> Result<(Vec<Vec<u16>>, Option<Vec<u16>>), CnError>>;

/// Realm-local property reads at the original compiler's observable stages.
pub struct Utf16CompileHooks {
    pub targets: Rc<dyn Fn() -> Result<Vec<Vec<u16>>, CnError>>,
    pub prepare_conflicts: Utf16ConflictPreparation,
    pub conflicts: Utf16ConflictReader,
    pub postfix: Rc<dyn Fn(Utf16GroupIds) -> Result<Vec<i32>, CnError>>,
    pub finish: Utf16FinishReader,
}
pub fn compile_utf16_dynamic_bounded(
    config: &Utf16Config,
    hooks: &Utf16CompileHooks,
    max_depth: usize,
    max_nodes: usize,
) -> Result<Utf16CompiledTables, CnError> {
    Ok(compile_utf16_model(config, Some((max_depth, max_nodes)), Some(hooks))?.compiled)
}
