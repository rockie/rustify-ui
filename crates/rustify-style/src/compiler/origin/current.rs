//! Current graph storage, independent of public host mutation records.

mod walk;
pub(in crate::compiler) use walk::{Action, Visit};

use super::{json_bytes_seen, key, CompileError, Key, Value, View};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Clone, Copy)]
pub(in crate::compiler) struct Allowance {
    bytes: usize,
    peak: usize,
    limit: usize,
}
impl Default for Allowance {
    fn default() -> Self {
        Self::new(0, usize::MAX)
    }
}
impl Allowance {
    pub(in crate::compiler) fn new(bytes: usize, limit: usize) -> Self {
        Self {
            bytes,
            peak: bytes,
            limit,
        }
    }
    fn reserve(&mut self, bytes: usize) -> Result<(), CompileError> {
        let amount = self
            .bytes
            .checked_add(bytes)
            .ok_or_else(|| CompileError::Limit {
                resource: "cache_bytes".into(),
                limit: self.limit,
            })?;
        super::super::check_limit(amount, self.limit, "cache_bytes")?;
        self.bytes = amount;
        self.peak = self.peak.max(amount);
        Ok(())
    }
    fn release(&mut self, bytes: usize) {
        self.bytes = self.bytes.saturating_sub(bytes);
    }
}

pub(in crate::compiler) struct Captured {
    pub(in crate::compiler) anchor: Value,
    pub(in crate::compiler) version: Value,
}

#[derive(Default)]
struct Freeze {
    temporary_bytes: usize,
    arrays: BTreeMap<usize, (Value, Key)>,
    seeds: BTreeMap<Key, View>,
    memo: BTreeMap<Key, Value>,
}

fn clone_bytes(value: &Value) -> usize {
    match value {
        Value::String(text) => text.owned_bytes(),
        _ => 0,
    }
}

fn array_bytes(length: usize) -> usize {
    std::mem::size_of::<Vec<Value>>()
        .saturating_add(2 * std::mem::size_of::<usize>())
        .saturating_add(length.saturating_mul(std::mem::size_of::<Value>()))
}

// A new B-tree entry can split every node on its search path. Reserve that
// growth before insertion rather than inspecting the tree after allocation.
fn entry_bytes<T>(length: usize) -> usize {
    let levels = length.checked_ilog2().unwrap_or(0) as usize + 1;
    (12 * std::mem::size_of::<T>() + 256).saturating_mul(levels)
}

fn same_storage(left: &Value, right: &Value) -> bool {
    match (super::storage(left), super::storage(right)) {
        (Some(left), Some(right)) => left == right,
        (None, None) => left == right,
        _ => false,
    }
}

#[derive(Default)]
pub(in crate::compiler) struct Current {
    pub(super) values: BTreeMap<Key, View>,
    // Retain storage as well as its address: versions may outlive a current view.
    versions: BTreeMap<usize, (Value, Key)>,
}

impl Current {
    fn anchor(&self, value: &Value) -> Option<Key> {
        let key = key(value)?;
        if key.0 == 0 {
            Some(self.versions.get(&key.1).map_or(key, |(_, anchor)| *anchor))
        } else {
            Some(key)
        }
    }

    pub(in crate::compiler) fn value<'a>(&'a self, value: &'a Value) -> &'a Value {
        self.anchor(value)
            .and_then(|key| self.values.get(&key))
            .map_or(value, |view| &view.current)
    }

    pub(in crate::compiler) fn array<'a>(
        &'a self,
        value: &'a Arc<Vec<Value>>,
    ) -> &'a Arc<Vec<Value>> {
        let key = (0, Arc::as_ptr(value) as usize);
        let anchor = self.versions.get(&key.1).map_or(key, |(_, anchor)| *anchor);
        match self.values.get(&anchor).map(|view| &view.current) {
            Some(Value::Array(current)) => current,
            _ => value,
        }
    }

    fn freeze(
        &self,
        value: &Value,
        allowance: &mut Allowance,
        frozen: &mut Freeze,
    ) -> Result<Value, CompileError> {
        let current = self.value(value);
        let Some(anchor) = self.anchor(value) else {
            allowance.reserve(clone_bytes(current))?;
            return Ok(current.clone());
        };
        if let Some(value) = frozen.memo.get(&anchor) {
            return Ok(value.clone());
        }
        let memo_bytes = entry_bytes::<(Key, Value)>(frozen.memo.len());
        // Reserve the worst insertion path, then retain only the whole-map
        // upper bound: the root has at least one entry, other nodes at least five.
        // Keeping every insertion's path charge made this temporary memo O(n log n).
        let memo_growth = if frozen.memo.len().is_multiple_of(5) {
            entry_bytes::<(Key, Value)>(0)
        } else {
            0
        };
        allowance.reserve(memo_bytes)?;
        frozen.memo.insert(anchor, current.clone());
        allowance.release(memo_bytes.saturating_sub(memo_growth));
        frozen.temporary_bytes = frozen.temporary_bytes.saturating_add(memo_growth);
        let version = match current {
            Value::Array(array) => {
                let mut output: Option<Vec<Value>> = None;
                for (index, child) in array.iter().enumerate() {
                    let version = if key(child).is_some() {
                        Some(self.freeze(child, allowance, frozen)?)
                    } else {
                        None
                    };
                    if output.is_none()
                        && version
                            .as_ref()
                            .is_some_and(|version| !same_storage(child, version))
                    {
                        let strings = array
                            .iter()
                            .fold(0usize, |sum, value| sum.saturating_add(clone_bytes(value)));
                        allowance.reserve(array_bytes(array.len()).saturating_add(strings))?;
                        let mut copy = Vec::with_capacity(array.len());
                        copy.extend(array[..index].iter().cloned());
                        output = Some(copy);
                    }
                    if let Some(output) = &mut output {
                        output.push(version.unwrap_or_else(|| child.clone()));
                    }
                }
                if let Some(output) = output {
                    let temporary_bytes = entry_bytes::<(usize, (Value, Key))>(frozen.arrays.len());
                    allowance.reserve(temporary_bytes)?;
                    frozen.temporary_bytes = frozen.temporary_bytes.saturating_add(temporary_bytes);
                    allowance.reserve(entry_bytes::<(usize, (Value, Key))>(
                        self.versions.len().saturating_add(frozen.arrays.len()),
                    ))?;
                    if !self.values.contains_key(&anchor) && !frozen.seeds.contains_key(&anchor) {
                        let temporary_bytes = entry_bytes::<(Key, View)>(frozen.seeds.len());
                        allowance.reserve(temporary_bytes)?;
                        frozen.temporary_bytes =
                            frozen.temporary_bytes.saturating_add(temporary_bytes);
                        allowance.reserve(entry_bytes::<(Key, View)>(
                            self.values.len().saturating_add(frozen.seeds.len()),
                        ))?;
                        frozen.seeds.insert(
                            anchor,
                            View {
                                original: value.clone(),
                                current: current.clone(),
                            },
                        );
                    }
                    let array = Arc::new(output);
                    let address = Arc::as_ptr(&array) as usize;
                    let version = Value::Array(array);
                    frozen.arrays.insert(address, (version.clone(), anchor));
                    version
                } else {
                    current.clone()
                }
            }
            Value::Object(object) => {
                let mut fields = None;
                for (field, child) in object.iter() {
                    if key(child).is_none() {
                        continue;
                    }
                    let version = self.freeze(child, allowance, frozen)?;
                    if same_storage(child, &version) {
                        continue;
                    }
                    if fields.is_none() {
                        let bytes = object.iter().fold(
                            std::mem::size_of::<super::super::public_api::PublicObject>()
                                .saturating_add(2 * std::mem::size_of::<usize>())
                                .saturating_add(object.capacity().saturating_mul(256)),
                            |sum, (key, value)| {
                                sum.saturating_add(key.owned_bytes())
                                    .saturating_add(clone_bytes(value))
                            },
                        );
                        allowance.reserve(bytes)?;
                        fields = Some((**object).clone());
                    }
                    if let Some(fields) = &mut fields {
                        if let Some(value) = fields.get_mut(field) {
                            *value = version;
                        }
                    }
                }
                fields.map_or_else(|| current.clone(), |fields| Value::Object(Arc::new(fields)))
            }
            _ => current.clone(),
        };
        frozen.memo.insert(anchor, version.clone());
        Ok(version)
    }

    fn publish_freeze(&mut self, frozen: Freeze, allowance: &mut Allowance) {
        self.values.extend(frozen.seeds);
        self.versions.extend(frozen.arrays);
        // The memo and transfer maps are dropped after publication. Their peak
        // reserve must not become a permanent charge on later operations.
        allowance.release(frozen.temporary_bytes);
    }

    pub(in crate::compiler) fn assign(
        &mut self,
        target: &Value,
        field: &str,
        value: &Value,
        allowance: &mut Allowance,
        record_bytes: usize,
    ) -> Result<Captured, CompileError> {
        let Value::Object(object) = self.value(target) else {
            return Err(CompileError::Syntax {
                offset: 0,
                message: "AST assignment target must be an object.".into(),
            });
        };
        let copy_bytes = object.iter().fold(
            std::mem::size_of::<super::super::public_api::PublicObject>()
                .saturating_add(2 * std::mem::size_of::<usize>())
                .saturating_add(object.capacity().saturating_add(1).saturating_mul(256)),
            |sum, (key, value)| {
                sum.saturating_add(key.owned_bytes())
                    .saturating_add(clone_bytes(value))
            },
        );
        let entry_reserve = entry_bytes::<(Key, View)>(self.values.len());
        let reserve = copy_bytes
            .saturating_add(field.encode_utf16().count().saturating_mul(2))
            .saturating_add(clone_bytes(value).saturating_mul(3))
            .saturating_add(entry_reserve)
            .saturating_add(record_bytes);
        let mut draft = *allowance;
        draft.reserve(reserve)?;
        let mut frozen = Freeze::default();
        let captured = Captured {
            anchor: value.clone(),
            version: self.freeze(value, &mut draft, &mut frozen)?,
        };
        let mut fields = (**object).clone();
        fields.insert(field.into(), captured.anchor.clone());
        self.publish_freeze(frozen, &mut draft);
        // Reserve a possible split path before allocation, then retain the
        // whole-map upper bound: non-root B-tree nodes have at least five keys.
        let entry_growth = if self
            .anchor(target)
            .is_some_and(|anchor| !self.values.contains_key(&anchor))
            && self.values.len().is_multiple_of(5)
        {
            entry_bytes::<(Key, View)>(0)
        } else {
            0
        };
        self.insert(target.clone(), Value::Object(Arc::new(fields)));
        draft.release(entry_reserve.saturating_sub(entry_growth));
        *allowance = draft;
        Ok(captured)
    }

    pub(in crate::compiler) fn splice(
        &mut self,
        target: &Value,
        start: usize,
        deleted: usize,
        items: &[Value],
        allowance: &mut Allowance,
        record_bytes: usize,
    ) -> Result<Captured, CompileError> {
        let Value::Array(array) = self.value(target) else {
            return Err(CompileError::Syntax {
                offset: 0,
                message: "AST splice target must be an array.".into(),
            });
        };
        if start > array.len() {
            return Err(CompileError::Syntax {
                offset: 0,
                message: "AST splice index is out of bounds.".into(),
            });
        }
        let end = start.saturating_add(deleted).min(array.len());
        let length = array
            .len()
            .saturating_sub(end - start)
            .saturating_add(items.len());
        let strings = array[..start]
            .iter()
            .chain(items)
            .chain(&array[end..])
            .fold(0usize, |sum, value| sum.saturating_add(clone_bytes(value)));
        let captured_strings = items.iter().fold(0usize, |sum, value| {
            sum.saturating_add(clone_bytes(value))
                .saturating_add(clone_bytes(self.value(value)))
        });
        let reserve = array_bytes(length)
            .saturating_add(array_bytes(items.len()).saturating_mul(2))
            .saturating_add(strings)
            .saturating_add(captured_strings)
            .saturating_add(entry_bytes::<(Key, View)>(self.values.len()))
            .saturating_add(entry_bytes::<(usize, (Value, Key))>(self.versions.len()))
            .saturating_add(record_bytes);
        let mut draft = *allowance;
        draft.reserve(reserve)?;
        let mut output = Vec::with_capacity(length);
        output.extend(array[..start].iter().cloned());
        output.extend(items.iter().cloned());
        output.extend(array[end..].iter().cloned());
        let anchor = Value::Array(Arc::new(items.to_vec()));
        let mut frozen = Freeze::default();
        let version = self.freeze(&anchor, &mut draft, &mut frozen)?;
        let captured = Captured { anchor, version };
        self.publish_freeze(frozen, &mut draft);
        self.insert(target.clone(), Value::Array(Arc::new(output)));
        *allowance = draft;
        Ok(captured)
    }

    pub(super) fn insert(&mut self, original: Value, current: Value) {
        let Some(anchor) = self.anchor(&original) else {
            return;
        };
        if let Value::Array(array) = &current {
            let version = Arc::as_ptr(array) as usize;
            if version != anchor.1 {
                self.versions.insert(version, (current.clone(), anchor));
            }
        }
        self.values
            .entry(anchor)
            .and_modify(|view| view.current = current.clone())
            .or_insert(View { original, current });
    }

    pub(super) fn include(&mut self, current: &Self) {
        self.values.extend(
            current
                .values
                .iter()
                .map(|(key, view)| (*key, view.clone())),
        );
        self.versions.extend(
            current
                .versions
                .iter()
                .map(|(key, version)| (*key, version.clone())),
        );
    }

    pub(in crate::compiler) fn owned_bytes(&self, seen: &mut BTreeSet<Key>) -> usize {
        let entry_bytes = 5 * std::mem::size_of::<usize>();
        let bytes = self.values.values().fold(0usize, |sum, view| {
            sum.saturating_add(std::mem::size_of::<(Key, View)>() + entry_bytes)
                .saturating_add(json_bytes_seen(&view.original, seen))
                .saturating_add(json_bytes_seen(&view.current, seen))
        });
        self.versions.values().fold(bytes, |sum, (version, _)| {
            sum.saturating_add(std::mem::size_of::<(usize, (Value, Key))>() + entry_bytes)
                .saturating_add(json_bytes_seen(version, seen))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::public_api::{design_system::public_json, PublicValue};
    use crate::text::Text;

    #[test]
    fn splice_uses_the_current_shared_array_and_retains_old_storage() {
        let body = public_json!(["first", "second"]);
        let left = public_json!({"nodes":body.clone()});
        let right = public_json!({"nodes":body.clone()});
        let mut current = Current::default();
        let mut allowance = Allowance::default();
        current.splice(&body, 0, 1, &[], &mut allowance, 0).unwrap();
        let first_version = current.value(&body).clone();
        current
            .splice(&first_version, 0, usize::MAX, &[], &mut allowance, 0)
            .unwrap();
        assert_eq!(
            current.value(&current.value(&left)["nodes"]),
            &public_json!([])
        );
        assert_eq!(
            current.value(&current.value(&right)["nodes"]),
            &public_json!([])
        );
        assert_eq!(first_version, public_json!(["second"]));
        assert_eq!(body, public_json!(["first", "second"]));
    }

    #[test]
    fn children_assignment_changes_one_edge_and_captures_its_storage() {
        let shared = public_json!(["old"]);
        let left = public_json!({"nodes":shared.clone()});
        let right = public_json!({"nodes":shared.clone()});
        let replacement = public_json!(["new"]);
        let mut current = Current::default();
        let mut allowance = Allowance::default();
        let captured = current
            .assign(&left, "nodes", &replacement, &mut allowance, 0)
            .unwrap();
        current
            .splice(&shared, 0, 1, &[], &mut allowance, 0)
            .unwrap();
        current
            .splice(&replacement, 0, 1, &[], &mut allowance, 0)
            .unwrap();
        assert_eq!(captured.anchor, public_json!(["new"]));
        assert_eq!(captured.version, public_json!(["new"]));
        assert_eq!(
            current.value(&current.value(&left)["nodes"]),
            &public_json!([])
        );
        assert_eq!(
            current.value(&current.value(&right)["nodes"]),
            &public_json!([])
        );
        assert_eq!(key(current.value(&left)), key(&left));
    }

    #[test]
    fn generated_assignment_freezes_current_descendants_and_source_aliases() {
        let child = public_json!({"value":"old"});
        let body = public_json!([child.clone(), child.clone()]);
        let source =
            public_json!({"file":"source.css", "body":body.clone(), "child":child.clone()});
        let generated = public_json!({"nodes":body.clone(), "extra":child.clone(), "src":public_json!([source, 0usize, 4usize])});
        let receiver = public_json!({"nodes":public_json!([])});
        let mut current = Current::default();
        let mut allowance = Allowance::default();
        current
            .assign(&child, "value", &public_json!("first"), &mut allowance, 0)
            .unwrap();
        let captured = current
            .assign(&receiver, "nodes", &generated, &mut allowance, 0)
            .unwrap();
        current
            .assign(&child, "value", &public_json!("later"), &mut allowance, 0)
            .unwrap();
        current.splice(&body, 0, 2, &[], &mut allowance, 0).unwrap();
        assert_eq!(captured.version["nodes"][0]["value"], public_json!("first"));
        assert_eq!(captured.version["extra"]["value"], public_json!("first"));
        assert_eq!(
            super::super::storage(&captured.version["nodes"]),
            super::super::storage(&captured.version["src"][0]["body"])
        );
        assert_eq!(
            key(&captured.version["nodes"][0]),
            key(&captured.version["nodes"][1])
        );
        assert_eq!(
            key(&captured.version["nodes"][0]),
            key(&captured.version["extra"])
        );
        assert_eq!(current.value(&captured.version["nodes"]), &public_json!([]));
        assert_eq!(
            current.value(&current.value(&receiver)["nodes"])["nodes"],
            body
        );
        assert_eq!(current.value(&child)["value"], public_json!("later"));
        assert_eq!(body.as_array().unwrap().len(), 2);
    }

    #[test]
    fn splice_captures_inserted_fields_before_later_assignments() {
        let child = public_json!({"value":"old"});
        let body = public_json!([]);
        let mut current = Current::default();
        let mut allowance = Allowance::default();
        current
            .assign(&child, "value", &public_json!("first"), &mut allowance, 0)
            .unwrap();
        let captured = current
            .splice(&body, 0, 0, std::slice::from_ref(&child), &mut allowance, 0)
            .unwrap();
        current
            .assign(&child, "value", &public_json!("later"), &mut allowance, 0)
            .unwrap();
        assert_eq!(captured.version[0]["value"], public_json!("first"));
        assert_eq!(captured.anchor[0]["value"], public_json!("old"));
        assert_eq!(
            current.value(&current.value(&body)[0])["value"],
            public_json!("later")
        );
    }

    #[test]
    fn assignment_refusal_preserves_the_completed_prefix_and_allowance() {
        let target = public_json!({"params":"old"});
        let mut current = Current::default();
        let mut allowance = Allowance::default();
        let first = current
            .assign(&target, "params", &public_json!("first"), &mut allowance, 0)
            .unwrap();
        let bytes = allowance.bytes;
        allowance.limit = bytes;
        let result = current.assign(
            &target,
            "params",
            &public_json!("refused"),
            &mut allowance,
            0,
        );
        assert!(
            matches!(result, Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
        );
        assert_eq!(allowance.bytes, bytes);
        assert_eq!(current.value(&target)["params"], public_json!("first"));
        assert_eq!(first.version, public_json!("first"));
        assert_eq!(target["params"], public_json!("old"));
    }

    #[test]
    fn splice_boundary_admits_exact_reserve_and_refuses_one_byte_less() {
        let body = public_json!(["old"]);
        let item = public_json!({"value":"inserted"});
        let mut probe = Current::default();
        let mut measured = Allowance::default();
        probe
            .splice(&body, 0, 1, std::slice::from_ref(&item), &mut measured, 512)
            .unwrap();
        let mut refused = Current::default();
        let mut budget = Allowance::new(0, measured.peak - 1);
        assert!(
            matches!(refused.splice(&body, 0, 1, std::slice::from_ref(&item), &mut budget, 512), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
        );
        assert_eq!(budget.bytes, 0);
        assert_eq!(budget.peak, 0);
        assert!(refused.values.is_empty() && refused.versions.is_empty());
        assert_eq!(body, public_json!(["old"]));
        let mut admitted = Current::default();
        let mut budget = Allowance::new(0, measured.peak);
        admitted
            .splice(&body, 0, 1, std::slice::from_ref(&item), &mut budget, 512)
            .unwrap();
        assert_eq!(budget.bytes, measured.bytes);
        assert_eq!(budget.peak, measured.peak);
        assert_eq!(admitted.value(&body)[0]["value"], public_json!("inserted"));
    }

    #[test]
    fn unchanged_source_metadata_keeps_storage_without_copying_text() {
        let source = public_json!({"file":"one.css", "code":"x".repeat(131072)});
        let target = public_json!({});
        let mut current = Current::default();
        let mut budget = Allowance::new(0, 4096);
        let captured = current
            .assign(&target, "src", &source, &mut budget, 0)
            .unwrap();
        assert!(same_storage(&source, &captured.version));
        assert!(same_storage(&source, &current.value(&target)["src"]));
        assert!(budget.bytes < budget.peak && budget.peak <= 4096);
        assert_eq!(source["code"].as_text().unwrap().units().len(), 131072);
    }

    #[test]
    fn large_cow_field_and_overflow_are_refused_before_installation() {
        let target = public_json!({"metadata":"x".repeat(8192), "value":"old"});
        let mut current = Current::default();
        let mut budget = Allowance::new(0, 1024);
        assert!(
            matches!(current.assign(&target, "value", &public_json!("new"), &mut budget, 0), Err(CompileError::Limit { resource, limit:1024 }) if resource == "cache_bytes")
        );
        assert_eq!(budget.bytes, 0);
        assert!(current.values.is_empty() && current.versions.is_empty());
        let mut budget = Allowance::new(usize::MAX, usize::MAX);
        assert!(
            matches!(current.assign(&target, "value", &public_json!("new"), &mut budget, 0), Err(CompileError::Limit { resource, .. }) if resource == "cache_bytes")
        );
        assert_eq!(budget.bytes, usize::MAX);
        assert_eq!(target["value"], public_json!("old"));
    }

    #[test]
    fn invalid_structural_target_and_index_do_not_spend_or_install() {
        let mut current = Current::default();
        let mut budget = Allowance::default();
        assert!(matches!(
            current.assign(
                &public_json!(false),
                "nodes",
                &public_json!([]),
                &mut budget,
                0
            ),
            Err(CompileError::Syntax { .. })
        ));
        let body = public_json!([]);
        assert!(matches!(
            current.splice(&body, 1, 0, &[], &mut budget, 0),
            Err(CompileError::Syntax { .. })
        ));
        assert_eq!(budget.bytes, 0);
        assert!(current.values.is_empty() && current.versions.is_empty());
    }
}
