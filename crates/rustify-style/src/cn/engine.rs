use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use super::validators;

/// Packed upstream engine tables. String offsets count UTF-16 code units.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tables {
    #[serde(rename = "GROUP_COUNT")]
    pub group_count: usize,
    pub edge_start: Vec<i32>,
    pub label_start: Vec<i32>,
    pub label_text: String,
    pub edge_target: Vec<i32>,
    pub node_group: Vec<i32>,
    pub node_vlist: Vec<i32>,
    pub vlist_pat: Vec<i32>,
    pub vlist_ops: Vec<i32>,
    pub vlist_ref: Vec<i32>,
    pub vlist_group: Vec<i32>,
    pub lit_anchor: Vec<i32>,
    pub lit_group: Vec<i32>,
    pub lit_pool: Vec<i32>,
    pub pool_offsets: Vec<i32>,
    pub pool_text: String,
    pub adj_gid: Vec<i32>,
    pub adj_start: Vec<i32>,
    pub adj_tgt: Vec<i32>,
    pub pat_gid: Vec<i32>,
    pub pat_tgt: Vec<i32>,
    pub postfix_lookup_groups: Vec<i32>,
    pub custom_validator_names: Vec<String>,
    pub order_sensitive_modifiers: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl Tables {
    /// Check table ranges before the runtime indexes untrusted input.
    pub fn validate(&self) -> Result<(), super::CnError> {
        let bad = || super::CnError::new("cn: invalid engine tables");
        let csr = |starts: &[i32], len: usize| {
            starts.first() == Some(&0)
                && starts
                    .last()
                    .is_some_and(|end| *end >= 0 && *end as usize == len)
                && starts.windows(2).all(|pair| pair[0] <= pair[1])
        };
        let indices = |values: &[i32], len: usize| {
            values
                .iter()
                .all(|value| *value >= 0 && (*value as usize) < len)
        };
        let nodes = self.node_group.len();
        if nodes == 0
            || self.node_vlist.len() != nodes
            || self.edge_start.len() != nodes + 1
            || !csr(&self.edge_start, self.edge_target.len())
            || !indices(&self.edge_target, nodes)
            || !csr(&self.label_start, self.label_text.encode_utf16().count())
            || self.label_start.len() != self.edge_target.len() + 1
            || !self.label_start.windows(2).all(|pair| pair[0] < pair[1])
        {
            return Err(bad());
        }
        if !self
            .node_group
            .iter()
            .all(|gid| *gid == -1 || (*gid >= 0 && (*gid as usize) < self.group_count))
            || !self
                .node_vlist
                .iter()
                .all(|list| *list == -1 || (*list >= 0 && (*list as usize) < self.vlist_ref.len()))
            || !csr(&self.vlist_pat, self.vlist_ops.len())
            || !indices(&self.vlist_ref, self.vlist_pat.len().saturating_sub(1))
        {
            return Err(bad());
        }
        let total = self
            .vlist_ref
            .iter()
            .try_fold(0usize, |sum, reference| {
                sum.checked_add(
                    (self.vlist_pat[*reference as usize + 1] - self.vlist_pat[*reference as usize])
                        as usize,
                )
            })
            .ok_or_else(bad)?;
        if total != self.vlist_group.len()
            || !indices(&self.vlist_group, self.group_count)
            || !indices(&self.vlist_ops, 25 + self.custom_validator_names.len())
        {
            return Err(bad());
        }
        if self.lit_anchor.len() != self.lit_group.len()
            || self.lit_anchor.len() != self.lit_pool.len()
            || !self.pool_offsets.len().is_multiple_of(2)
            || !indices(&self.lit_anchor, nodes)
            || !indices(&self.lit_group, self.group_count)
            || !indices(&self.lit_pool, self.pool_offsets.len() / 2)
        {
            return Err(bad());
        }
        let pool_len = self.pool_text.encode_utf16().count();
        if !self.pool_offsets.as_chunks::<2>().0.iter().all(|span| {
            span[0] >= 0
                && span[1] >= 0
                && (span[0] as usize)
                    .checked_add(span[1] as usize)
                    .is_some_and(|end| end <= pool_len)
        }) {
            return Err(bad());
        }
        if self.adj_start.len() != self.adj_gid.len() + 1
            || !csr(&self.adj_start, self.adj_tgt.len())
            || !indices(&self.adj_gid, self.group_count)
            || !indices(&self.adj_tgt, self.group_count)
            || self.pat_gid.len() != self.pat_tgt.len()
            || !indices(&self.pat_gid, self.group_count)
            || !indices(&self.pat_tgt, self.group_count)
            || !indices(&self.postfix_lookup_groups, self.group_count)
        {
            return Err(bad());
        }
        Ok(())
    }
}

/// Lossless ECMAScript table text, measured in UTF-16 code units.
#[derive(Clone, Debug, PartialEq)]
pub struct Utf16Tables {
    pub group_count: usize,
    pub edge_start: Vec<i32>,
    pub label_start: Vec<i32>,
    pub label_text: Vec<u16>,
    pub edge_target: Vec<i32>,
    pub node_group: Vec<i32>,
    pub node_vlist: Vec<i32>,
    pub vlist_pat: Vec<i32>,
    pub vlist_ops: Vec<i32>,
    pub vlist_ref: Vec<i32>,
    pub vlist_group: Vec<i32>,
    pub lit_anchor: Vec<i32>,
    pub lit_group: Vec<i32>,
    pub lit_pool: Vec<i32>,
    pub pool_offsets: Vec<i32>,
    pub pool_text: Vec<u16>,
    pub adj_gid: Vec<i32>,
    pub adj_start: Vec<i32>,
    pub adj_tgt: Vec<i32>,
    pub pat_gid: Vec<i32>,
    pub pat_tgt: Vec<i32>,
    pub postfix_lookup_groups: Vec<i32>,
    pub custom_validator_names: Vec<Vec<u16>>,
    pub order_sensitive_modifiers: Vec<u16>,
    pub prefix: Option<Vec<u16>>,
}

impl Utf16Tables {
    /// Check table ranges before the runtime indexes untrusted input.
    pub fn validate(&self) -> Result<(), super::CnError> {
        let bad = || super::CnError::new("cn: invalid engine tables");
        let csr = |starts: &[i32], len: usize| {
            starts.first() == Some(&0)
                && starts
                    .last()
                    .is_some_and(|end| *end >= 0 && *end as usize == len)
                && starts.windows(2).all(|pair| pair[0] <= pair[1])
        };
        let indices = |values: &[i32], len: usize| {
            values
                .iter()
                .all(|value| *value >= 0 && (*value as usize) < len)
        };
        let nodes = self.node_group.len();
        if nodes == 0
            || self.node_vlist.len() != nodes
            || self.edge_start.len() != nodes + 1
            || !csr(&self.edge_start, self.edge_target.len())
            || !indices(&self.edge_target, nodes)
            || !csr(&self.label_start, self.label_text.len())
            || self.label_start.len() != self.edge_target.len() + 1
            || !self.label_start.windows(2).all(|pair| pair[0] < pair[1])
        {
            return Err(bad());
        }
        if !self
            .node_group
            .iter()
            .all(|gid| *gid == -1 || (*gid >= 0 && (*gid as usize) < self.group_count))
            || !self
                .node_vlist
                .iter()
                .all(|list| *list == -1 || (*list >= 0 && (*list as usize) < self.vlist_ref.len()))
            || !csr(&self.vlist_pat, self.vlist_ops.len())
            || !indices(&self.vlist_ref, self.vlist_pat.len().saturating_sub(1))
        {
            return Err(bad());
        }
        let total = self
            .vlist_ref
            .iter()
            .try_fold(0usize, |sum, reference| {
                sum.checked_add(
                    (self.vlist_pat[*reference as usize + 1] - self.vlist_pat[*reference as usize])
                        as usize,
                )
            })
            .ok_or_else(bad)?;
        if total != self.vlist_group.len()
            || !indices(&self.vlist_group, self.group_count)
            || !indices(&self.vlist_ops, 25 + self.custom_validator_names.len())
        {
            return Err(bad());
        }
        if self.lit_anchor.len() != self.lit_group.len()
            || self.lit_anchor.len() != self.lit_pool.len()
            || !self.pool_offsets.len().is_multiple_of(2)
            || !indices(&self.lit_anchor, nodes)
            || !indices(&self.lit_group, self.group_count)
            || !indices(&self.lit_pool, self.pool_offsets.len() / 2)
        {
            return Err(bad());
        }
        let pool_len = self.pool_text.len();
        if !self.pool_offsets.as_chunks::<2>().0.iter().all(|span| {
            span[0] >= 0
                && span[1] >= 0
                && (span[0] as usize)
                    .checked_add(span[1] as usize)
                    .is_some_and(|end| end <= pool_len)
        }) {
            return Err(bad());
        }
        if self.adj_start.len() != self.adj_gid.len() + 1
            || !csr(&self.adj_start, self.adj_tgt.len())
            || !indices(&self.adj_gid, self.group_count)
            || !indices(&self.adj_tgt, self.group_count)
            || self.pat_gid.len() != self.pat_tgt.len()
            || !indices(&self.pat_gid, self.group_count)
            || !indices(&self.pat_tgt, self.group_count)
            || !indices(&self.postfix_lookup_groups, self.group_count)
        {
            return Err(bad());
        }
        Ok(())
    }
}

impl From<Tables> for Utf16Tables {
    fn from(t: Tables) -> Self {
        Self {
            group_count: t.group_count,
            edge_start: t.edge_start,
            label_start: t.label_start,
            label_text: t.label_text.encode_utf16().collect(),
            edge_target: t.edge_target,
            node_group: t.node_group,
            node_vlist: t.node_vlist,
            vlist_pat: t.vlist_pat,
            vlist_ops: t.vlist_ops,
            vlist_ref: t.vlist_ref,
            vlist_group: t.vlist_group,
            lit_anchor: t.lit_anchor,
            lit_group: t.lit_group,
            lit_pool: t.lit_pool,
            pool_offsets: t.pool_offsets,
            pool_text: t.pool_text.encode_utf16().collect(),
            adj_gid: t.adj_gid,
            adj_start: t.adj_start,
            adj_tgt: t.adj_tgt,
            pat_gid: t.pat_gid,
            pat_tgt: t.pat_tgt,
            postfix_lookup_groups: t.postfix_lookup_groups,
            custom_validator_names: t
                .custom_validator_names
                .into_iter()
                .map(|s| s.encode_utf16().collect())
                .collect(),
            order_sensitive_modifiers: t.order_sensitive_modifiers.encode_utf16().collect(),
            prefix: t.prefix.map(|s| s.encode_utf16().collect()),
        }
    }
}
impl TryFrom<Utf16Tables> for Tables {
    type Error = super::CnError;
    fn try_from(t: Utf16Tables) -> Result<Self, Self::Error> {
        let string = |units: Vec<u16>| {
            String::from_utf16(&units).map_err(|_| {
                super::CnError::invalid_config("unpaired UTF-16 surrogate in native string")
            })
        };
        Ok(Self {
            group_count: t.group_count,
            edge_start: t.edge_start,
            label_start: t.label_start,
            label_text: string(t.label_text)?,
            edge_target: t.edge_target,
            node_group: t.node_group,
            node_vlist: t.node_vlist,
            vlist_pat: t.vlist_pat,
            vlist_ops: t.vlist_ops,
            vlist_ref: t.vlist_ref,
            vlist_group: t.vlist_group,
            lit_anchor: t.lit_anchor,
            lit_group: t.lit_group,
            lit_pool: t.lit_pool,
            pool_offsets: t.pool_offsets,
            pool_text: string(t.pool_text)?,
            adj_gid: t.adj_gid,
            adj_start: t.adj_start,
            adj_tgt: t.adj_tgt,
            pat_gid: t.pat_gid,
            pat_tgt: t.pat_tgt,
            postfix_lookup_groups: t.postfix_lookup_groups,
            custom_validator_names: t
                .custom_validator_names
                .into_iter()
                .map(string)
                .collect::<Result<_, _>>()?,
            order_sensitive_modifiers: string(t.order_sensitive_modifiers)?,
            prefix: t.prefix.map(string).transpose()?,
        })
    }
}

pub(crate) struct TableEngine {
    pub tables: Utf16Tables,
    labels: Vec<u16>,
    literals: Vec<HashMap<Vec<u16>, usize>>,
    validator_start: Vec<usize>,
    conflicts: HashMap<usize, Vec<usize>>,
    postfix_conflicts: HashMap<usize, Vec<usize>>,
    order_sensitive: HashSet<Vec<u16>>,
    memo: Mutex<TokenMemo>,
    scratch: Mutex<Vec<TokenRecord>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Group {
    Static(usize),
    Property(Vec<u16>),
}

impl TableEngine {
    pub(crate) fn new(tables: Tables) -> Self {
        Self::new_utf16(tables.into())
    }
    pub(crate) fn new_utf16(tables: Utf16Tables) -> Self {
        let nodes = tables.node_group.len();
        let mut literals: Vec<HashMap<Vec<u16>, usize>> =
            (0..nodes).map(|_| HashMap::new()).collect();
        let pool = &tables.pool_text;
        for i in 0..tables.lit_anchor.len() {
            let reference = tables.lit_pool[i] as usize;
            let start = tables.pool_offsets[reference * 2] as usize;
            let len = tables.pool_offsets[reference * 2 + 1] as usize;
            literals[tables.lit_anchor[i] as usize].insert(
                pool[start..start + len].to_vec(),
                tables.lit_group[i] as usize,
            );
        }
        let mut validator_start = vec![0];
        for reference in &tables.vlist_ref {
            let pattern = *reference as usize;
            let count = (tables.vlist_pat[pattern + 1] - tables.vlist_pat[pattern]) as usize;
            validator_start.push(validator_start.last().copied().unwrap_or(0) + count);
        }
        let mut conflicts = HashMap::new();
        for (row, gid) in tables.adj_gid.iter().enumerate() {
            conflicts.insert(
                *gid as usize,
                tables.adj_tgt[tables.adj_start[row] as usize..tables.adj_start[row + 1] as usize]
                    .iter()
                    .map(|target| *target as usize)
                    .collect(),
            );
        }
        let mut postfix_conflicts: HashMap<usize, Vec<usize>> = HashMap::new();
        for (&gid, &target) in tables.pat_gid.iter().zip(&tables.pat_tgt) {
            postfix_conflicts
                .entry(gid as usize)
                .or_default()
                .push(target as usize);
        }
        Self {
            labels: tables.label_text.clone(),
            order_sensitive: tables
                .order_sensitive_modifiers
                .split(|c| *c == 32)
                .map(|s| s.to_vec())
                .collect(),
            tables,
            literals,
            validator_start,
            conflicts,
            postfix_conflicts,
            memo: Mutex::new(TokenMemo::default()),
            scratch: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn set_memory_budget(&self, limit: Option<usize>) {
        self.memo.lock().unwrap().byte_limit = limit;
    }

    pub(crate) fn memo_string_bytes(&self) -> usize {
        let memo = self.memo.lock().unwrap();
        memo.bytes + memo.context_bytes + memo.property_bytes
    }

    fn classify<F, E>(&self, bare: &[u16], predicate: &mut F) -> Result<Option<Group>, E>
    where
        F: FnMut(usize, &[u16]) -> Result<bool, E>,
    {
        if bare.first() == Some(&91) && bare.last() == Some(&93) {
            if let Some(colon) = bare[1..bare.len() - 1].iter().position(|c| *c == 58) {
                if colon > 0 {
                    return Ok(Some(Group::Property(bare[1..colon + 1].to_vec())));
                }
            }
        }
        let input = if bare.len() > 1 && bare[0] == 45 {
            &bare[1..]
        } else {
            bare
        };
        let mut node = 0;
        let mut at = 0;
        let mut checkpoints = Vec::new();
        if self.tables.node_vlist[0] >= 0 || !self.literals[0].is_empty() {
            checkpoints.push((0, 0));
        }
        let mut arrived = true;
        while at < input.len() {
            let start = self.tables.edge_start[node] as usize;
            let end = self.tables.edge_start[node + 1] as usize;
            let edge = (start..end)
                .find(|edge| self.labels[self.tables.label_start[*edge] as usize] == input[at]);
            let Some(edge) = edge else {
                arrived = false;
                break;
            };
            let label = &self.labels[self.tables.label_start[edge] as usize
                ..self.tables.label_start[edge + 1] as usize];
            if !input[at..].starts_with(label) {
                arrived = false;
                break;
            }
            at += label.len();
            node = self.tables.edge_target[edge] as usize;
            if at < input.len()
                && input[at] == 45
                && (self.tables.node_vlist[node] >= 0 || !self.literals[node].is_empty())
            {
                checkpoints.push((node, at + 1));
            }
        }
        if arrived && self.tables.node_group[node] >= 0 {
            return Ok(Some(Group::Static(self.tables.node_group[node] as usize)));
        }
        for (node, tail) in checkpoints.into_iter().rev() {
            if tail > input.len() {
                continue;
            }
            let span = &input[tail..];
            if !span.is_empty() {
                if let Some(gid) = self.literals[node].get(span) {
                    return Ok(Some(Group::Static(*gid)));
                }
            }
            let list = self.tables.node_vlist[node];
            if list < 0 {
                continue;
            }
            let list = list as usize;
            let pattern = self.tables.vlist_ref[list] as usize;
            let start = self.tables.vlist_pat[pattern] as usize;
            let end = self.tables.vlist_pat[pattern + 1] as usize;
            for (offset, op) in self.tables.vlist_ops[start..end].iter().enumerate() {
                if predicate(*op as usize, span)? {
                    return Ok(Some(Group::Static(
                        self.tables.vlist_group[self.validator_start[list] + offset] as usize,
                    )));
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn merge<F, E>(
        &self,
        input: &str,
        prefix: Option<&str>,
        mut predicate: F,
    ) -> Result<String, E>
    where
        F: FnMut(usize, &str) -> Result<bool, E>,
        E: From<super::CnError>,
    {
        let input: Vec<_> = input.encode_utf16().collect();
        let prefix = prefix.map(|s| s.encode_utf16().collect::<Vec<_>>());
        self.merge_utf16(&input, prefix.as_deref(), |op, value| {
            predicate(op, &String::from_utf16_lossy(value))
        })
        .map(|units| String::from_utf16_lossy(&units))
    }

    /// The JS engine operates on code units; unpaired surrogates remain unchanged.
    pub(crate) fn merge_utf16<F, E>(
        &self,
        input: &[u16],
        prefix: Option<&[u16]>,
        mut predicate: F,
    ) -> Result<Vec<u16>, E>
    where
        F: FnMut(usize, &[u16]) -> Result<bool, E>,
        E: From<super::CnError>,
    {
        self.memo.lock().unwrap().start_merge();
        let table_prefix = self.tables.prefix.as_ref();
        let prefix = prefix
            .or(table_prefix.map(Vec::as_slice))
            .filter(|p| !p.is_empty())
            .map(|p| {
                let mut p = p.to_vec();
                p.push(58);
                p
            });
        let shared = !self.tables.custom_validator_names.is_empty();
        let mut tokens = Vec::new();
        let mut total_chars = 0;
        for token in input
            .split(|c| is_whitespace_unit(*c))
            .filter(|token| !token.is_empty())
        {
            let start = (token.as_ptr() as usize - input.as_ptr() as usize) / 2;
            let end = start + token.len();
            let index = tokens.len();
            total_chars += token.len();
            if shared {
                let mut scratch = self.scratch.lock().unwrap();
                if index >= scratch.len() {
                    scratch.push(TokenRecord::default());
                }
                scratch[index].start = start;
                scratch[index].end = end;
            }
            // Upstream scratch survives nested validator calls. Its offsets can
            // be overwritten by the nested merge before classification returns.
            // No lock is retained while calling arbitrary user code.
            let cached = self.memo.lock().unwrap().get(token);
            let classification = if let Some(cached) = cached {
                cached
            } else {
                let classification = self.parse(token, prefix.as_deref(), &mut predicate)?;
                self.memo.lock().unwrap().put(token, classification.clone());
                classification
            };
            if shared {
                self.scratch.lock().unwrap()[index].classification = classification.clone();
            }
            tokens.push(TokenRecord {
                start,
                end,
                classification,
            });
        }
        if shared {
            tokens = self.scratch.lock().unwrap()[..tokens.len()].to_vec();
        }
        if tokens.is_empty() {
            return Ok(Vec::new());
        }
        if tokens.len() == 1 {
            return Ok(js_slice(input, tokens[0].start, tokens[0].end).to_vec());
        }
        let near_id_limit = {
            let memo = self.memo.lock().unwrap();
            tokens.len().saturating_add(
                (memo.context_count + 2).max(memo.properties.len() + self.tables.group_count),
            ) >= 2097152
        };
        if near_id_limit {
            let (prior_properties, prior_contexts) = {
                let memo = self.memo.lock().unwrap();
                (
                    memo.properties.iter().cloned().collect::<Vec<_>>(),
                    memo.contexts
                        .iter()
                        .map(|(key, mask)| (key.clone(), *mask))
                        .collect::<Vec<_>>(),
                )
            };
            let mut properties: HashSet<&[u16]> =
                prior_properties.iter().map(Vec::as_slice).collect();
            let mut contexts: HashSet<(&[u16], bool)> = HashSet::new();
            for (key, mask) in &prior_contexts {
                for important in [false, true] {
                    if mask & if important { 2 } else { 1 } != 0 {
                        contexts.insert((key, important));
                    }
                }
            }
            for token in &tokens {
                let (group, context, important, _) = &token.classification;
                if group.is_some() && !context.is_empty() {
                    contexts.insert((context, *important));
                }
                if let Some(Group::Property(property)) = group {
                    properties.insert(property);
                }
            }
            if contexts.len() + 2 >= 2097152
                || properties.len() + self.tables.group_count >= 2097152
            {
                return Err(
                    super::CnError::new("cn: too many distinct classes in one merge").into(),
                );
            }
        }
        let mut claimed: HashMap<(&[u16], bool), HashSet<Group>> = HashMap::new();
        let mut kept = Vec::new();
        let mut did_drop = false;
        for (index, token) in tokens.iter().enumerate().rev() {
            let (group, context, important, postfix) = &token.classification;
            let Some(group) = group else {
                kept.push(index);
                continue;
            };
            let claims = claimed.entry((context, *important)).or_default();
            if !claims.insert(group.clone()) {
                did_drop = true;
                continue;
            }
            kept.push(index);
            if let Group::Static(gid) = group {
                claims.extend(
                    self.conflicts
                        .get(gid)
                        .into_iter()
                        .flatten()
                        .map(|gid| Group::Static(*gid)),
                );
                if *postfix {
                    claims.extend(
                        self.postfix_conflicts
                            .get(gid)
                            .into_iter()
                            .flatten()
                            .map(|gid| Group::Static(*gid)),
                    );
                }
            }
        }
        if !did_drop
            && !tokens.is_empty()
            && input.len() == total_chars + tokens.len() - 1
            && !input.iter().any(|c| *c != 32 && is_whitespace_unit(*c))
        {
            return Ok(input.to_vec());
        }
        kept.reverse();
        let mut output = Vec::new();
        for index in kept {
            let token = &tokens[index];
            if !output.is_empty() {
                output.push(32);
            }
            output.extend_from_slice(js_slice(input, token.start, token.end));
        }
        Ok(output)
    }

    fn parse<F, E>(
        &self,
        token: &[u16],
        prefix: Option<&[u16]>,
        predicate: &mut F,
    ) -> Result<Classification, E>
    where
        F: FnMut(usize, &[u16]) -> Result<bool, E>,
    {
        let parsed = if let Some(prefix) = prefix {
            token.strip_prefix(prefix).filter(|tail| !tail.is_empty())
        } else {
            Some(token)
        };
        let Some(parsed) = parsed else {
            return Ok((None, Vec::new(), false, false));
        };
        let (colon, slash) = separators_units(parsed);
        let mod_start = colon.map_or(0, |at| at + 1);
        let mut bare = &parsed[mod_start..];
        let important = bare.first() == Some(&33) || bare.last() == Some(&33);
        if bare.last() == Some(&33) {
            bare = &bare[..bare.len() - 1];
        } else if bare.first() == Some(&33) {
            bare = &bare[1..];
        }
        let context = colon
            .filter(|at| *at > 0)
            .map_or_else(Vec::new, |at| self.canonicalize(&parsed[..at]));
        let mut postfix = false;
        let group = if let Some(slash) = slash.filter(|slash| *slash > mod_start) {
            let end = slash - mod_start;
            if end < bare.len() {
                let candidate = self.classify(&bare[..end], predicate)?;
                if let Some(Group::Static(gid)) = &candidate {
                    postfix = true;
                    if self.tables.postfix_lookup_groups.contains(&(*gid as i32)) {
                        let full = self.classify(bare, predicate)?;
                        if full.is_some() && full != candidate {
                            postfix = false;
                            full
                        } else {
                            candidate
                        }
                    } else {
                        candidate
                    }
                } else if candidate.is_none() {
                    self.classify(bare, predicate)?
                } else {
                    postfix = true;
                    candidate
                }
            } else {
                self.classify(bare, predicate)?
            }
        } else {
            self.classify(bare, predicate)?
        };
        self.memo.lock().unwrap().track(
            colon.filter(|at| *at > 0).map(|at| &parsed[..at]),
            &context,
            important,
            &group,
        );
        Ok((group, context, important, postfix))
    }

    fn canonicalize(&self, modifiers: &[u16]) -> Vec<u16> {
        let mut sorted = Vec::new();
        let mut segment = Vec::new();
        let mut start = 0;
        let mut depth_b = 0;
        let mut depth_p = 0;
        for (at, c) in modifiers
            .iter()
            .copied()
            .enumerate()
            .chain(std::iter::once((modifiers.len(), 58)))
        {
            if c == 58 && depth_b == 0 && depth_p == 0 {
                let modifier = &modifiers[start..at];
                if modifier.first() == Some(&91) || self.order_sensitive.contains(modifier) {
                    segment.sort();
                    sorted.append(&mut segment);
                    sorted.push(modifier);
                } else {
                    segment.push(modifier);
                }
                start = at + 1;
            } else {
                match c {
                    91 => depth_b += 1,
                    93 => depth_b -= 1,
                    40 => depth_p += 1,
                    41 => depth_p -= 1,
                    _ => {}
                }
            }
        }
        segment.sort();
        sorted.append(&mut segment);
        let mut output = Vec::new();
        for (i, modifier) in sorted.into_iter().enumerate() {
            if i > 0 {
                output.push(58);
            }
            output.extend_from_slice(modifier);
        }
        output
    }
}

type Classification = (Option<Group>, Vec<u16>, bool, bool);
#[derive(Clone, Default)]
struct TokenRecord {
    start: usize,
    end: usize,
    classification: Classification,
}
fn js_slice(input: &[u16], start: usize, end: usize) -> &[u16] {
    let start = start.min(input.len());
    let end = end.min(input.len()).max(start);
    &input[start..end]
}
struct MemoEntry {
    token: Vec<u16>,
    classification: Classification,
}
struct TokenMemo {
    slots: Vec<Option<MemoEntry>>,
    tick: usize,
    bytes: usize,
    byte_limit: Option<usize>,
    context_hashes: HashSet<u32>,
    contexts: HashMap<Vec<u16>, u8>,
    context_count: usize,
    properties: HashSet<Vec<u16>>,
    context_bytes: usize,
    property_bytes: usize,
    context_overflow: bool,
    property_overflow: bool,
}
impl Default for TokenMemo {
    fn default() -> Self {
        Self {
            slots: (0..8192).map(|_| None).collect(),
            tick: 0,
            bytes: 0,
            byte_limit: None,
            context_hashes: HashSet::new(),
            contexts: HashMap::new(),
            context_count: 0,
            properties: HashSet::new(),
            context_bytes: 0,
            property_bytes: 0,
            context_overflow: false,
            property_overflow: false,
        }
    }
}
impl TokenMemo {
    fn start_merge(&mut self) {
        let reset_context = self.context_overflow
            || self.context_count + 2 > 4096
            || self.context_hashes.len() > 4096;
        let reset_property = self.property_overflow || self.properties.len() > 4096;
        if reset_context {
            self.contexts.clear();
            self.context_count = 0;
            self.context_hashes.clear();
            self.context_bytes = 0;
            self.context_overflow = false;
        }
        if reset_property {
            self.properties.clear();
            self.property_bytes = 0;
            self.property_overflow = false;
        }
        if reset_context || reset_property {
            for slot in &mut self.slots {
                *slot = None;
            }
            self.bytes = 0;
        }
    }
    fn room(&self, extra: usize) -> bool {
        self.byte_limit.is_none_or(|limit| {
            self.bytes
                .saturating_add(self.context_bytes)
                .saturating_add(self.property_bytes)
                .saturating_add(extra)
                <= limit
        })
    }
    fn track(
        &mut self,
        raw: Option<&[u16]>,
        canonical: &[u16],
        important: bool,
        group: &Option<Group>,
    ) {
        if group.is_none() {
            return;
        }
        if let Some(raw) = raw {
            if !self.context_overflow {
                let hash = raw.iter().fold(0x811c9dc5u32, |hash, c| {
                    (hash ^ *c as u32).wrapping_mul(0x01000193)
                }) ^ if important { 0x9e3779b9 } else { 0 };
                self.context_hashes.insert(hash);
                let bit = if important { 2u8 } else { 1u8 };
                let mask = self.contexts.get(canonical).copied().unwrap_or(0);
                if mask & bit == 0 {
                    let bytes = if mask == 0 {
                        canonical.len().saturating_mul(2)
                    } else {
                        0
                    };
                    if !self.room(bytes) {
                        self.context_overflow = true;
                    } else {
                        self.context_bytes += bytes;
                        self.context_count += 1;
                        if let Some(mask) = self.contexts.get_mut(canonical) {
                            *mask |= bit;
                        } else {
                            self.contexts.insert(canonical.to_vec(), bit);
                        }
                    }
                }
                if self.context_count + 2 > 4096 || self.context_hashes.len() > 4096 {
                    self.context_overflow = true;
                }
            }
        }
        if let Some(Group::Property(property)) = group {
            if !self.property_overflow && !self.properties.contains(property) {
                let bytes = property.len().saturating_mul(2);
                if !self.room(bytes) {
                    self.property_overflow = true;
                } else {
                    self.property_bytes += bytes;
                    self.properties.insert(property.clone());
                }
                if self.properties.len() > 4096 {
                    self.property_overflow = true;
                }
            }
        }
    }
    fn way(token: &[u16]) -> usize {
        let hash = token
            .iter()
            .fold(0u32, |hash, c| (hash ^ *c as u32).wrapping_mul(0x01000193))
            ^ (token.len() as u32).wrapping_mul(0x9e3779b1);
        ((hash ^ (hash >> 15)) as usize) & 8190
    }
    fn get(&self, token: &[u16]) -> Option<Classification> {
        let way = Self::way(token);
        [way, way | 1].into_iter().find_map(|slot| {
            self.slots[slot]
                .as_ref()
                .filter(|entry| entry.token == token)
                .map(|entry| entry.classification.clone())
        })
    }
    fn put(&mut self, token: &[u16], classification: Classification) {
        let way = Self::way(token);
        let slot = if self.slots[way].is_none() {
            way
        } else if self.slots[way | 1].is_none() {
            way | 1
        } else {
            let old = self.tick;
            self.tick = self.tick.wrapping_add(1);
            if old & 3 != 0 {
                return;
            }
            way | ((self.tick >> 2) & 1)
        };
        let old_bytes = self.slots[slot]
            .as_ref()
            .map_or(0, |entry| memo_bytes(&entry.token, &entry.classification));
        let new_bytes = memo_bytes(token, &classification);
        let bytes = self
            .bytes
            .saturating_sub(old_bytes)
            .saturating_add(new_bytes);
        if self.byte_limit.is_some_and(|limit| {
            bytes
                .saturating_add(self.context_bytes)
                .saturating_add(self.property_bytes)
                > limit
        }) {
            return;
        }
        self.bytes = bytes;
        self.slots[slot] = Some(MemoEntry {
            token: token.to_vec(),
            classification,
        });
    }
}
fn is_whitespace_unit(c: u16) -> bool {
    char::from_u32(c as u32).is_some_and(validators::is_whitespace)
}
pub(crate) fn separators_units(token: &[u16]) -> (Option<usize>, Option<usize>) {
    let mut colon = None;
    let mut slash = None;
    let mut depth_b = 0;
    let mut depth_p = 0;
    for (at, c) in token.iter().copied().enumerate() {
        if depth_b == 0 && depth_p == 0 {
            if c == 58 {
                colon = Some(at);
            } else if c == 47 {
                slash = Some(at);
            }
        }
        match c {
            91 => depth_b += 1,
            93 => depth_b -= 1,
            40 => depth_p += 1,
            41 => depth_p -= 1,
            _ => {}
        }
    }
    (colon, slash)
}

pub(crate) fn separators(token: &str) -> (Option<usize>, Option<usize>) {
    let mut colon = None;
    let mut slash = None;
    let mut depth_b = 0;
    let mut depth_p = 0;
    for (at, c) in token.char_indices() {
        if depth_b == 0 && depth_p == 0 {
            if c == ':' {
                colon = Some(at);
            } else if c == '/' {
                slash = Some(at);
            }
        }
        match c {
            '[' => depth_b += 1,
            ']' => depth_b -= 1,
            '(' => depth_p += 1,
            ')' => depth_p -= 1,
            _ => {}
        }
    }
    (colon, slash)
}

fn memo_bytes(token: &[u16], classification: &Classification) -> usize {
    (token.len()
        + classification.1.len()
        + match &classification.0 {
            Some(Group::Property(value)) => value.len(),
            _ => 0,
        })
    .saturating_mul(2)
}
