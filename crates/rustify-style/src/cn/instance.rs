use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::{
    clsx, compiler, engine::TableEngine, merge_configs, tw_join, validators, ClassValue, CnError,
    Config, ConfigExtension, Tables, Utf16Config, Utf16Tables, Utf16Validator, Validator,
};

/// Engine cache and prefix options. A byte budget can be supplied by a facade.
#[derive(Clone, Debug)]
pub struct EngineOptions {
    pub cache_size: usize,
    pub cache_bytes: Option<usize>,
    pub prefix: Option<String>,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            cache_size: 8192,
            cache_bytes: None,
            prefix: None,
        }
    }
}

#[derive(Default)]
struct Cache {
    values: HashMap<Vec<u16>, Vec<u16>>,
    previous: HashMap<Vec<u16>, Vec<u16>>,
    count: usize,
    bytes: usize,
    door: Vec<i32>,
    door_base: usize,
    door_epoch: i32,
    door_marks: usize,
}

/// One immutable cn configuration and its bounded per-instance result cache.
pub struct Cn {
    engine: TableEngine,
    callbacks: Vec<Validator>,
    utf16_callbacks: Vec<Utf16Validator>,
    options: EngineOptions,
    cache: RefCell<Cache>,
    utf16_prefix: Option<Vec<u16>>,
    arguments: RefCell<ArgCache>,
}

impl Default for Cn {
    fn default() -> Self {
        Self {
            engine: TableEngine::new(super::default_tables::tables()),
            callbacks: Vec::new(),
            utf16_callbacks: Vec::new(),
            options: EngineOptions::default(),
            cache: RefCell::new(Cache::default()),
            utf16_prefix: None,
            arguments: RefCell::new(ArgCache::default()),
        }
    }
}

impl Cn {
    /// Initialize the fixed typed tables with a caller-supplied resource budget.
    pub fn default_with_options(options: EngineOptions) -> Self {
        let instance = Self {
            options,
            ..Self::default()
        };
        instance
            .engine
            .set_memory_budget(instance.options.cache_bytes.map(|bytes| bytes / 2));
        instance
    }

    /// Compile a complete configuration and retain its native callbacks.
    pub fn new(config: Config) -> Result<Self, CnError> {
        let compiled = compiler::compile_to_tables(&config)?;
        let options = EngineOptions {
            prefix: compiled.prefix,
            ..EngineOptions::default()
        };
        Self::from_tables(compiled.tables, compiled.validator_impls, options)
    }

    /// Extend the complete fixed default configuration.
    pub fn with_extension(extension: &ConfigExtension) -> Result<Self, CnError> {
        let mut instance = Self::new(merge_configs(&Config::default(), extension))?;
        if let Some(size) = extension.cache_size {
            instance.options.cache_size = size;
        }
        Ok(instance)
    }

    /// Apply a native default-config transform before compiling the instance.
    pub fn from_transform(
        transform: impl FnOnce(Config) -> Result<Config, CnError>,
    ) -> Result<Self, CnError> {
        Self::new(transform(Config::default())?)
    }

    /// Initialize an engine from externally supplied validated tables.
    pub fn from_tables(
        tables: Tables,
        callbacks: Vec<Validator>,
        options: EngineOptions,
    ) -> Result<Self, CnError> {
        tables.validate()?;
        if callbacks.len() != tables.custom_validator_names.len() {
            let name = tables
                .custom_validator_names
                .get(callbacks.len())
                .map_or("<extra implementation>", String::as_str);
            return Err(CnError::new(format!("cn: missing validator {name}")));
        }
        let engine = TableEngine::new(tables);
        engine.set_memory_budget(options.cache_bytes.map(|bytes| bytes / 2));
        Ok(Self {
            engine,
            callbacks,
            utf16_callbacks: Vec::new(),
            options,
            cache: RefCell::new(Cache::default()),
            utf16_prefix: None,
            arguments: RefCell::new(ArgCache::default()),
        })
    }

    /// Compile the complete lossless ECMAScript configuration once.
    pub fn from_utf16_config(
        config: &Utf16Config,
        options: EngineOptions,
    ) -> Result<Self, CnError> {
        let compiled = compiler::compile_utf16(config)?;
        let mut instance =
            Self::from_utf16_tables(compiled.tables, compiled.validator_impls, options)?;
        instance.utf16_prefix = compiled.prefix;
        Ok(instance)
    }
    pub fn from_utf16_tables(
        tables: Utf16Tables,
        callbacks: Vec<Utf16Validator>,
        options: EngineOptions,
    ) -> Result<Self, CnError> {
        tables.validate()?;
        if callbacks.len() != tables.custom_validator_names.len() {
            return Err(CnError::new("cn: missing validator implementation"));
        }
        let engine = TableEngine::new_utf16(tables);
        engine.set_memory_budget(options.cache_bytes.map(|bytes| bytes / 2));
        Ok(Self {
            engine,
            callbacks: Vec::new(),
            utf16_callbacks: callbacks,
            options,
            cache: RefCell::new(Cache::default()),
            utf16_prefix: None,
            arguments: RefCell::new(ArgCache::default()),
        })
    }
    /// Merge code units without replacing lone surrogates. Callback errors propagate.
    pub fn merge_utf16_uncached(&self, input: &[u16]) -> Result<Vec<u16>, CnError> {
        let native_prefix = self
            .options
            .prefix
            .as_ref()
            .map(|s| s.encode_utf16().collect::<Vec<_>>());
        self.engine.merge_utf16(
            input,
            self.utf16_prefix.as_deref().or(native_prefix.as_deref()),
            |op, units| {
                if op < 25 {
                    Ok(validators::test(op, &String::from_utf16_lossy(units)))
                } else if self.utf16_callbacks.is_empty() {
                    {
                        let value = String::from_utf16(units).map_err(|_| {
                            CnError::new("cn: native callback input contains an unpaired surrogate")
                        })?;
                        (self.callbacks[op - 25])(&value)
                    }
                } else {
                    (self.utf16_callbacks[op - 25])(units)
                }
            },
        )
    }
    pub fn merge_utf16(&self, input: &[u16]) -> Result<Vec<u16>, CnError> {
        if self.options.cache_size == 0 {
            return self.merge_utf16_uncached(input);
        }
        if let Some(value) = self.cache.borrow().values.get(input) {
            return Ok(value.clone());
        }
        let hash = span_hash_units(input);
        let was_seen = door_seen(&self.cache.borrow(), hash);
        if was_seen {
            let previous = self.cache.borrow().previous.get(input).cloned();
            if let Some(value) = previous {
                self.insert_result(input, &value, false);
                return Ok(value);
            }
        }
        let output = self.merge_utf16_uncached(input)?;
        if was_seen {
            self.insert_result(input, &output, true);
        } else {
            mark_door(&mut self.cache.borrow_mut(), hash);
        }
        Ok(output)
    }

    fn insert_result(&self, input: &[u16], output: &[u16], count: bool) {
        let bytes = (input.len() + output.len()).saturating_mul(2);
        let limit = self.options.cache_bytes.map(|limit| limit / 4);
        if limit.is_some_and(|limit| bytes > limit) {
            return;
        }
        let mut cache = self.cache.borrow_mut();
        if limit.is_some_and(|limit| cache.bytes.saturating_add(bytes) > limit) {
            // Explicit SDK resource pressure may retire both generations early.
            cache.values.clear();
            cache.previous.clear();
            cache.bytes = 0;
            cache.count = 0;
        }
        if !cache.values.contains_key(input) {
            cache.values.insert(input.to_vec(), output.to_vec());
            cache.bytes += bytes;
        }
        if count {
            cache.count += 1;
            if cache.count > self.options.cache_size {
                cache.count = 0;
                cache.previous = std::mem::take(&mut cache.values);
                cache.bytes = cache
                    .previous
                    .iter()
                    .map(|(key, value)| (key.len() + value.len()) * 2)
                    .sum();
                rotate_door(&mut cache);
            }
        }
    }

    /// Set the facade's explicit argument-cache metadata limit.
    pub fn set_argument_limit(&self, nodes: usize) {
        self.arguments.borrow_mut().node_limit = nodes;
    }
    pub fn argument_string_bytes(&self) -> usize {
        self.arguments.borrow().bytes
    }
    /// clsx wrapper over values already resolved on this call. Mutable JS
    /// arrays/objects must be resolved afresh before entering this method.
    pub fn merge_arguments_utf16(
        &self,
        values: &[Vec<u16>],
        arity_two_strings: bool,
    ) -> Result<Vec<u16>, CnError> {
        let values: Vec<&[u16]> = values
            .iter()
            .map(Vec::as_slice)
            .filter(|v| !v.is_empty())
            .collect();
        if values.is_empty() {
            return Ok(Vec::new());
        }
        if values.len() == 1 {
            return self.merge_utf16(values[0]);
        }
        {
            let mut cache = self.arguments.borrow_mut();
            if let Some(last) = cache.last {
                if let Some(next) = cache.entries[last].next {
                    if cache.entries[next].matches(&values) {
                        cache.last = Some(next);
                        return Ok(cache.entries[next].output.clone());
                    }
                }
                if cache.entries[last].matches(&values) {
                    return Ok(cache.entries[last].output.clone());
                }
            }
        }
        let first = values[0];
        let mut joined = first.to_vec();
        for value in &values[1..] {
            joined.push(32);
            joined.extend_from_slice(value);
        }
        let (skip, hit, mut bucket, revision) = {
            let mut cache = self.arguments.borrow_mut();
            if !cache.current.contains_key(first) {
                if let Some(bucket) = cache.previous.get(first).cloned() {
                    if self.options.cache_bytes.is_some_and(|limit| {
                        cache.bytes.saturating_add(first.len() * 2) > limit / 4
                    }) {
                        cache.clear();
                    } else {
                        cache.current.insert(first.to_vec(), bucket);
                        cache.bytes += first.len() * 2;
                    }
                }
            }
            let bucket = cache.current.get(first).cloned();
            let (skip, hit) = if let Some(bucket) = &bucket {
                let bucket = bucket.borrow();
                (
                    bucket.skip > 0,
                    if bucket.skip > 0 {
                        None
                    } else {
                        bucket
                            .entries
                            .iter()
                            .copied()
                            .find(|id| cache.entries[*id].matches(&values))
                    },
                )
            } else {
                (false, None)
            };
            (skip, hit, bucket, cache.revision)
        };
        if skip {
            bucket.as_ref().unwrap().borrow_mut().skip -= 1;
            self.arguments.borrow_mut().last = None;
            return self.argument_churn(
                if arity_two_strings {
                    values[1]
                } else {
                    &joined
                },
                if arity_two_strings { first } else { &[] },
                arity_two_strings,
            );
        }
        if let Some(hit) = hit {
            bucket.as_ref().unwrap().borrow_mut().miss = 0;
            let mut cache = self.arguments.borrow_mut();
            cache.link(hit);
            return Ok(cache.entries[hit].output.clone());
        }
        if !self.seen_before_utf16(&joined) {
            return self.merge_utf16_uncached(&joined);
        }
        let output = self.merge_utf16(&joined)?;
        let mut cache = self.arguments.borrow_mut();
        let bytes =
            (values.iter().map(|v| v.len()).sum::<usize>() + output.len() + first.len()) * 2;
        let limit = self.options.cache_bytes.map(|limit| limit / 4);
        if cache.entries.len() >= cache.node_limit
            || limit.is_some_and(|limit| cache.bytes.saturating_add(bytes) > limit)
        {
            cache.clear();
        }
        if limit.is_some_and(|limit| bytes > limit) {
            return Ok(output);
        }
        let id = cache.entries.len();
        cache.entries.push(ArgEntry {
            values: values.iter().map(|v| v.to_vec()).collect(),
            output: output.clone(),
            next: None,
        });
        cache.bytes += bytes - first.len() * 2;
        if revision != cache.revision {
            bucket = None;
        }
        let existed = bucket.is_some();
        let bucket = bucket.unwrap_or_else(|| {
            let bucket = Rc::new(RefCell::new(ArgBucket::default()));
            if cache
                .current
                .insert(first.to_vec(), bucket.clone())
                .is_none()
            {
                cache.bytes += first.len() * 2;
            }
            bucket
        });
        let mut bucket = bucket.borrow_mut();
        if existed {
            bucket.miss += 1;
            if bucket.miss > 16 {
                bucket.miss = 16;
                bucket.skip = 1024;
                bucket.entries.clear();
                bucket.at = 0;
            }
        }
        if bucket.entries.len() < 256 {
            bucket.entries.push(id);
        } else {
            let at = bucket.at;
            bucket.entries[at] = id;
            bucket.at = (bucket.at + 1) & 255;
        }
        drop(bucket);
        cache.count += 1;
        if cache.count > 1000 {
            cache.count = 0;
            let retired = std::mem::take(&mut cache.previous);
            cache.bytes -= retired.keys().map(|key| key.len() * 2).sum::<usize>();
            cache.previous = std::mem::take(&mut cache.current);
        }
        cache.link(id);
        Ok(output)
    }
    fn argument_churn(&self, key: &[u16], own: &[u16], join: bool) -> Result<Vec<u16>, CnError> {
        let way = span_hash_units(key) as usize & 4094;
        {
            let cache = self.arguments.borrow();
            for slot in [way, way | 1] {
                if let Some(entry) = cache.churn.get(slot).and_then(Option::as_ref) {
                    if entry.key == key && entry.own == own {
                        return Ok(entry.output.clone());
                    }
                }
            }
        }
        let mut input = Vec::new();
        if join {
            input.extend_from_slice(own);
            input.push(32);
        }
        input.extend_from_slice(key);
        let output = self.merge_utf16(&input)?;
        let mut cache = self.arguments.borrow_mut();
        if cache.churn.is_empty() {
            cache.churn.resize_with(4096, || None);
        }
        let slot = if cache.churn[way].is_none() {
            way
        } else if cache.churn[way | 1].is_none() {
            way | 1
        } else {
            let slot = way | (cache.tick & 1);
            cache.tick += 1;
            slot
        };
        let old = cache.churn[slot].as_ref().map_or(0, ChurnEntry::bytes);
        let bytes = (key.len() + own.len() + output.len()) * 2;
        if self
            .options
            .cache_bytes
            .is_none_or(|limit| cache.bytes - old + bytes <= limit / 4)
        {
            cache.bytes = cache.bytes - old + bytes;
            cache.churn[slot] = Some(ChurnEntry {
                key: key.to_vec(),
                own: own.to_vec(),
                output: output.clone(),
            });
        }
        Ok(output)
    }
    /// Join clsx-style inputs, then merge once.
    pub fn merge(&self, inputs: &[ClassValue<'_>]) -> Result<String, CnError> {
        let values = if let [ClassValue::List(values)] = inputs {
            values.as_slice()
        } else {
            inputs
        };
        let args = values
            .iter()
            .map(|value| {
                clsx(std::slice::from_ref(value))
                    .encode_utf16()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let two_strings =
            inputs.len() == 2 && inputs.iter().all(|v| matches!(v, ClassValue::Text(_)));
        String::from_utf16(&self.merge_arguments_utf16(&args, two_strings)?)
            .map_err(|_| CnError::new("cn: native result contains an unpaired surrogate"))
    }
    /// Join twJoin-style inputs, then merge once.
    pub fn tw_merge(&self, inputs: &[ClassValue<'_>]) -> Result<String, CnError> {
        self.merge_str(&tw_join(inputs))
    }
    /// Join without resolving conflicts.
    pub fn join(&self, inputs: &[ClassValue<'_>]) -> String {
        clsx(inputs)
    }

    /// Merge one already-joined class string, consulting the result cache.
    pub fn merge_str(&self, input: &str) -> Result<String, CnError> {
        let units = self.merge_utf16(&input.encode_utf16().collect::<Vec<_>>())?;
        String::from_utf16(&units)
            .map_err(|_| CnError::new("cn: native result contains an unpaired surrogate"))
    }

    /// Merge without consulting or updating the whole-string result cache.
    pub fn merge_uncached(&self, input: &str) -> Result<String, CnError> {
        let units = self.merge_utf16_uncached(&input.encode_utf16().collect::<Vec<_>>())?;
        String::from_utf16(&units)
            .map_err(|_| CnError::new("cn: native result contains an unpaired surrogate"))
    }

    /// Record a sighting and report whether this bounded doorkeeper retained it.
    pub fn seen_before(&self, input: &str) -> bool {
        self.seen_before_utf16(&input.encode_utf16().collect::<Vec<_>>())
    }
    pub fn seen_before_utf16(&self, input: &[u16]) -> bool {
        let mut cache = self.cache.borrow_mut();
        if self.options.cache_size == 0 {
            return false;
        }
        let hash = span_hash_units(input);
        if door_seen(&cache, hash) {
            return true;
        }
        mark_door(&mut cache, hash);
        false
    }

    pub fn memo_string_bytes(&self) -> usize {
        self.engine.memo_string_bytes()
    }
    /// Retained result count and UTF-16 text bytes across both generations.
    pub fn cache_usage(&self) -> (usize, usize) {
        let cache = self.cache.borrow();
        (cache.values.len() + cache.previous.len(), cache.bytes)
    }
}

fn span_hash_units(units: &[u16]) -> i32 {
    let len = units.len();
    let mut hash =
        (len as u32).wrapping_mul(0x9e3779b1) ^ units.first().copied().unwrap_or(0) as u32;
    if len > 3 {
        let quarter = len >> 2;
        let middle = len >> 1;
        hash =
            (hash ^ ((units[1] as u32) << 8) ^ ((units[2] as u32) << 16) ^ units[quarter] as u32)
                .wrapping_mul(0x85ebca6b);
        hash = (hash
            ^ ((units[middle] as u32) << 8)
            ^ ((units[middle + quarter] as u32) << 16)
            ^ units[len - 3] as u32)
            .wrapping_mul(0xc2b2ae35);
        hash ^= ((units[len - 2] as u32) << 8) ^ ((units[len - 1] as u32) << 16);
        let mut left = 3;
        let mut right = len - 4;
        while left < 8 && left < right {
            hash =
                (hash ^ units[left] as u32 ^ ((units[right] as u32) << 8)).wrapping_mul(0x01000193);
            left += 1;
            right -= 1;
        }
    }
    (hash ^ (hash >> 15)) as i32
}

fn door_seen(cache: &Cache, hash: i32) -> bool {
    if cache.door.is_empty() {
        return false;
    }
    let slot = (hash as usize & 16383) + cache.door_base;
    cache.door[slot] == hash ^ cache.door_epoch
        || cache.door[slot ^ 16384] == hash ^ cache.door_epoch.wrapping_sub(1)
}
fn mark_door(cache: &mut Cache, hash: i32) {
    if cache.door.is_empty() {
        cache.door.resize(32768, 0);
        cache.door_epoch = 1;
    }
    let slot = (hash as usize & 16383) + cache.door_base;
    cache.door[slot] = hash ^ cache.door_epoch;
    cache.door_marks += 1;
    if cache.door_marks > 16384 {
        rotate_door(cache);
    }
}

fn rotate_door(cache: &mut Cache) {
    cache.door_base ^= 16384;
    cache.door_epoch = cache.door_epoch.wrapping_add(1);
    cache.door_marks = 0;
}

#[derive(Default, Clone)]
struct ArgBucket {
    entries: Vec<usize>,
    miss: usize,
    skip: usize,
    at: usize,
}
struct ArgEntry {
    values: Vec<Vec<u16>>,
    output: Vec<u16>,
    next: Option<usize>,
}
impl ArgEntry {
    fn matches(&self, values: &[&[u16]]) -> bool {
        self.values.len() == values.len() && self.values.iter().zip(values).all(|(a, b)| a == b)
    }
}
struct ChurnEntry {
    key: Vec<u16>,
    own: Vec<u16>,
    output: Vec<u16>,
}
impl ChurnEntry {
    fn bytes(&self) -> usize {
        (self.key.len() + self.own.len() + self.output.len()) * 2
    }
}
struct ArgCache {
    current: HashMap<Vec<u16>, Rc<RefCell<ArgBucket>>>,
    previous: HashMap<Vec<u16>, Rc<RefCell<ArgBucket>>>,
    entries: Vec<ArgEntry>,
    last: Option<usize>,
    count: usize,
    bytes: usize,
    node_limit: usize,
    revision: usize,
    churn: Vec<Option<ChurnEntry>>,
    tick: usize,
}
impl Default for ArgCache {
    fn default() -> Self {
        Self {
            current: HashMap::new(),
            previous: HashMap::new(),
            entries: Vec::new(),
            last: None,
            count: 0,
            bytes: 0,
            node_limit: 100000,
            revision: 0,
            churn: Vec::new(),
            tick: 0,
        }
    }
}
impl ArgCache {
    fn clear(&mut self) {
        let nodes = self.node_limit;
        let revision = self.revision.wrapping_add(1);
        *self = Self::default();
        self.node_limit = nodes;
        self.revision = revision;
    }
    fn link(&mut self, id: usize) {
        if let Some(last) = self.last {
            if last != id {
                self.entries[last].next = Some(id);
            }
        }
        self.last = Some(id);
    }
}

#[cfg(test)]
mod utf16_tests {
    use super::*;
    use std::rc::Rc;
    #[test]
    fn native_validator_identity_survives_utf16_compilation() {
        let callback: Validator = Rc::new(|_| Ok(true));
        let config = Config {
            theme: Vec::new(),
            class_groups: vec![
                (
                    "one".into(),
                    vec![super::super::ClassGroupDef::Object(vec![(
                        "a".into(),
                        vec![super::super::ClassGroupDef::Validator(callback.clone())],
                    )])],
                ),
                (
                    "two".into(),
                    vec![super::super::ClassGroupDef::Object(vec![(
                        "b".into(),
                        vec![super::super::ClassGroupDef::Validator(callback)],
                    )])],
                ),
            ],
            conflicting_class_groups: Vec::new(),
            conflicting_class_group_modifiers: Vec::new(),
            order_sensitive_modifiers: Vec::new(),
            postfix_lookup_class_groups: Vec::new(),
            prefix: None,
        };
        assert_eq!(
            compiler::compile_to_tables(&config)
                .unwrap()
                .tables
                .custom_validator_names
                .len(),
            1
        );
    }
    fn probe(callback: Validator, bytes: Option<usize>) -> Cn {
        let mut config = Config::default();
        config.class_groups.push((
            "probe".into(),
            vec![super::super::ClassGroupDef::Object(vec![(
                "probe".into(),
                vec![super::super::ClassGroupDef::Validator(callback)],
            )])],
        ));
        let compiled = compiler::compile_to_tables(&config).unwrap();
        Cn::from_tables(
            compiled.tables,
            compiled.validator_impls,
            EngineOptions {
                cache_size: 1,
                cache_bytes: bytes,
                prefix: None,
            },
        )
        .unwrap()
    }
    #[test]
    fn native_tuple_cache_preserves_the_observed_source_result() {
        let active = Rc::new(std::cell::Cell::new(true));
        let state = active.clone();
        let cn = probe(Rc::new(move |_| Ok(state.get())), None);
        let values = [ClassValue::Text("probe-a"), ClassValue::Text("probe-b")];
        assert_eq!(cn.merge(&values).unwrap(), "probe-b");
        assert_eq!(cn.merge(&values).unwrap(), "probe-b");
        for input in ["p-2 p-4", "m-2 m-4", "gap-2 gap-4"] {
            cn.merge_str(input).unwrap();
            cn.merge_str(input).unwrap();
        }
        let churn = (0..4100)
            .map(|i| format!("v{i}:p-2"))
            .collect::<Vec<_>>()
            .join(" ");
        cn.merge_uncached(&churn).unwrap();
        active.set(false);
        assert_eq!(cn.merge(&values).unwrap(), "probe-b");
        assert_eq!(
            cn.merge_uncached("probe-a probe-b").unwrap(),
            "probe-a probe-b"
        );
    }
    #[test]
    fn all_retained_cache_text_obeys_one_byte_budget() {
        let cn = probe(Rc::new(|_| Ok(true)), Some(128));
        cn.set_argument_limit(4);
        for i in 0..40 {
            let a = format!("probe-{i}");
            let b = format!("probe-{}", i + 1);
            for _ in 0..3 {
                cn.merge(&[ClassValue::Text(&a), ClassValue::Text(&b)])
                    .unwrap();
            }
            let (_, output) = cn.cache_usage();
            let memo = cn.memo_string_bytes();
            let args = cn.argument_string_bytes();
            assert!(output <= 32 && memo <= 64 && args <= 32);
            assert!(output + memo + args <= 128);
        }
    }
}
