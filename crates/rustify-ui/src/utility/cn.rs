//! Main-realm component merging and retained raw class fragments.
use leptos::prelude::*;
use rustify_style::cn::{ClassValue, Cn, CnError};
use send_wrapper::SendWrapper;
#[cfg(any(target_arch = "wasm32", test))]
use std::collections::BTreeSet;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

/// The cn engine used by components inside one utility scope.
///
/// Provide the scope's context before constructing its component children.
/// Callback handles stay in this realm and never enter Worker messages.
#[derive(Clone)]
pub struct CnContext {
    raw: SendWrapper<Rc<RefCell<RawClasses>>>,
    error: RwSignal<Option<String>>,
    committed: RwSignal<SendWrapper<Rc<CommittedCn>>>,
}

#[derive(Clone)]
struct CommittedCn {
    engine: Rc<Cn>,
    epoch: u32,
    classes: Rc<BTreeMap<u64, (Vec<String>, String)>>,
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) struct PreparedCn {
    engine: Rc<Cn>,
    generation: u64,
    limit: usize,
    classes: Rc<BTreeMap<u64, (Vec<String>, String)>>,
    pub candidates: BTreeSet<String>,
}

struct RawClasses {
    next_id: u64,
    generation: u64,
    fragments: BTreeMap<u64, Vec<String>>,
    bytes: usize,
    limit: usize,
}

impl CnContext {
    /// Construct a context with an explicit budget for retained raw fragments.
    /// Most applications obtain this context from `UtilityScope::cn_context`.
    pub fn new(engine: Cn, raw_byte_limit: usize) -> Self {
        Self {
            raw: SendWrapper::new(Rc::new(RefCell::new(RawClasses {
                next_id: 0,
                generation: 0,
                fragments: BTreeMap::new(),
                bytes: 0,
                limit: raw_byte_limit,
            }))),
            error: RwSignal::new(None),
            committed: RwSignal::new(SendWrapper::new(Rc::new(CommittedCn {
                engine: Rc::new(engine),
                epoch: 1,
                classes: Rc::new(BTreeMap::new()),
            }))),
        }
    }

    /// Merge the original fragments once, tracking the committed engine.
    pub fn merge(&self, fragments: &[&str]) -> Result<String, CnError> {
        let committed = self.committed.get();
        let engine = &committed.engine;
        let values: Vec<_> = fragments
            .iter()
            .map(|text| ClassValue::Text(text))
            .collect();
        engine.merge(&values)
    }

    /// Merge registered raw fragments using the preflight result of the committed configuration.
    pub fn merge_registered(&self, id: u64, fragments: &[&str]) -> Result<String, CnError> {
        let committed = self.committed.get();
        if let Some((raw, class)) = committed.classes.get(&id) {
            if raw.iter().map(String::as_str).eq(fragments.iter().copied()) {
                return Ok(class.clone());
            }
        }
        self.merge(fragments)
    }

    /// The configuration epoch last installed alongside the scope stylesheet.
    pub fn epoch(&self) -> u32 {
        self.committed.get_untracked().epoch
    }

    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn prepare(&self, engine: Rc<Cn>, limit: usize) -> Result<PreparedCn, CnError> {
        // Clone before invoking callbacks, which may reenter the component registry.
        let (generation, fragments, bytes) = {
            let raw = self.raw.borrow();
            if raw.bytes > limit {
                return Err(CnError::new("cn: component raw classes exceed cache_bytes"));
            }
            (raw.generation, raw.fragments.clone(), raw.bytes)
        };
        if bytes > limit {
            return Err(CnError::new("cn: component raw classes exceed cache_bytes"));
        }
        let mut candidates = BTreeSet::new();
        let mut classes = BTreeMap::new();
        let mut output_bytes = bytes;
        for (id, fragments) in fragments {
            let values: Vec<_> = fragments
                .iter()
                .map(|text| ClassValue::Text(text))
                .collect();
            let class = engine.merge(&values)?;
            output_bytes = output_bytes.saturating_add(class.len());
            if output_bytes > limit {
                return Err(CnError::new(
                    "cn: prepared component classes exceed cache_bytes",
                ));
            }
            candidates.extend(class.split_whitespace().map(str::to_owned));
            classes.insert(id, (fragments, class));
        }
        let staged = PreparedCn {
            engine,
            generation,
            limit,
            classes: Rc::new(classes),
            candidates,
        };
        if !self.is_current(&staged) {
            return Err(CnError::new(
                "cn: raw classes changed during configuration preparation",
            ));
        }
        Ok(staged)
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn committed_engine(&self) -> Rc<Cn> {
        self.committed.get_untracked().engine.clone()
    }

    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn is_current(&self, prepared: &PreparedCn) -> bool {
        self.raw.borrow().generation == prepared.generation
    }

    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn publish(&self, prepared: &PreparedCn, epoch: u32) {
        self.raw.borrow_mut().limit = prepared.limit;
        self.committed.set(SendWrapper::new(Rc::new(CommittedCn {
            engine: prepared.engine.clone(),
            epoch,
            classes: prepared.classes.clone(),
        })));
        self.error.set(None);
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn advance_epoch(&self, epoch: u32) {
        if self.epoch() != epoch {
            self.committed
                .update(|committed| Rc::make_mut(committed).epoch = epoch);
        }
    }

    /// Retain a component's original fragments until its owner is cleaned up.
    pub fn register(&self, fragments: Vec<String>) -> Result<u64, CnError> {
        let mut raw = self.raw.borrow_mut();
        let id = raw
            .next_id
            .checked_add(1)
            .ok_or_else(|| CnError::new("cn: component id exhausted"))?;
        let bytes = fragment_bytes(&fragments);
        raw.check_size(raw.bytes.saturating_add(bytes))?;
        raw.generation = raw.generation.wrapping_add(1);
        raw.next_id = id;
        raw.bytes += bytes;
        raw.fragments.insert(id, fragments);
        Ok(id)
    }

    /// Update retained props without replacing the committed engine.
    pub fn update(&self, id: u64, fragments: Vec<String>) -> Result<(), CnError> {
        let mut raw = self.raw.borrow_mut();
        let previous = raw
            .fragments
            .get(&id)
            .ok_or_else(|| CnError::new("cn: component registration missing"))?;
        if previous == &fragments {
            return Ok(());
        }
        let bytes = raw
            .bytes
            .saturating_sub(fragment_bytes(previous))
            .saturating_add(fragment_bytes(&fragments));
        raw.check_size(bytes)?;
        raw.generation = raw.generation.wrapping_add(1);
        raw.fragments.insert(id, fragments);
        raw.bytes = bytes;
        drop(raw);
        self.committed.update_untracked(|committed| {
            Rc::make_mut(&mut Rc::make_mut(committed).classes).remove(&id);
        });
        Ok(())
    }

    /// Release a component's retained fragments when its Leptos owner dies.
    pub fn unregister(&self, id: u64) {
        let mut raw = self.raw.borrow_mut();
        if let Some(fragments) = raw.fragments.remove(&id) {
            raw.generation = raw.generation.wrapping_add(1);
            raw.bytes = raw.bytes.saturating_sub(fragment_bytes(&fragments));
            self.committed.update_untracked(|committed| {
                Rc::make_mut(&mut Rc::make_mut(committed).classes).remove(&id);
            });
        }
    }

    /// Record a component callback or retention error without changing its class.
    pub fn report_error(&self, error: CnError) {
        self.error.set(Some(error.to_string()));
    }

    pub fn last_error(&self) -> Option<String> {
        self.error.get_untracked()
    }
}

impl RawClasses {
    fn check_size(&self, bytes: usize) -> Result<(), CnError> {
        if bytes > self.limit {
            Err(CnError::new("cn: component raw classes exceed cache_bytes"))
        } else {
            Ok(())
        }
    }
}

fn fragment_bytes(fragments: &[String]) -> usize {
    // Include owned strings, their buffers and a conservative BTree node share.
    fragments
        .iter()
        .fold(std::mem::size_of::<Vec<String>>() + 64, |bytes, text| {
            bytes
                .saturating_add(std::mem::size_of::<String>())
                .saturating_add(text.capacity())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_style::cn::{ClassGroupDef, Config};

    fn disjoint_padding() -> Cn {
        let mut config = Config::default();
        config.class_groups.retain(|(name, _)| name != "p");
        config.class_groups.extend([
            (
                "padding-small".into(),
                vec![ClassGroupDef::Literal("p-2".into())],
            ),
            (
                "padding-large".into(),
                vec![ClassGroupDef::Literal("p-4".into())],
            ),
        ]);
        Cn::new(config).unwrap()
    }

    #[test]
    fn committed_engine_change_recomputes_retained_raw_fragments() {
        Owner::new().with(|| {
            let context = CnContext::new(Cn::default(), 4096);
            provide_context(context.clone());
            let computed_context = context.clone();
            let class = Memo::new(move |_| computed_context.merge(&["p-2", "p-4"]).unwrap());
            assert_eq!(class.get(), "p-4");
            let prepared = context.prepare(Rc::new(disjoint_padding()), 4096).unwrap();
            context.publish(&prepared, 2);
            assert_eq!(class.get(), "p-2 p-4");
        });
    }

    #[test]
    fn prepared_configuration_does_not_change_classes_until_published() {
        Owner::new().with(|| {
            let context = CnContext::new(Cn::default(), 4096);
            let id = context.register(vec!["p-2".into(), "p-4".into()]).unwrap();
            let staged = context.prepare(Rc::new(disjoint_padding()), 4096).unwrap();
            assert_eq!(
                context.merge_registered(id, &["p-2", "p-4"]).unwrap(),
                "p-4"
            );
            assert!(staged.candidates.contains("p-2"));
            context.publish(&staged, 2);
            assert_eq!(
                context.merge_registered(id, &["p-2", "p-4"]).unwrap(),
                "p-2 p-4"
            );
            assert_eq!(context.epoch(), 2);
        });
    }

    #[test]
    fn publishing_premerged_classes_does_not_repeat_user_callbacks() {
        use std::cell::Cell;
        Owner::new().with(|| {
            let calls = Rc::new(Cell::new(0));
            let observed = calls.clone();
            let mut config = Config::default();
            config.class_groups.push((
                "probe".into(),
                vec![ClassGroupDef::Object(vec![(
                    "probe".into(),
                    vec![ClassGroupDef::Validator(Rc::new(move |_| {
                        observed.set(observed.get() + 1);
                        Ok(true)
                    }))],
                )])],
            ));
            let context = CnContext::new(Cn::default(), 4096);
            let id = context.register(vec!["probe-one".into()]).unwrap();
            let staged = context
                .prepare(Rc::new(Cn::new(config).unwrap()), 4096)
                .unwrap();
            assert_eq!(calls.get(), 1);
            context.publish(&staged, 2);
            assert_eq!(
                context.merge_registered(id, &["probe-one"]).unwrap(),
                "probe-one"
            );
            assert_eq!(
                context.merge_registered(id, &["probe-one"]).unwrap(),
                "probe-one"
            );
            assert_eq!(calls.get(), 1);
        });
    }

    #[test]
    fn changed_raw_fragments_invalidate_the_prepared_configuration() {
        Owner::new().with(|| {
            let context = CnContext::new(Cn::default(), 4096);
            let id = context.register(vec!["p-2".into()]).unwrap();
            let staged = context.prepare(Rc::new(disjoint_padding()), 4096).unwrap();
            context.update(id, vec!["p-4".into()]).unwrap();
            assert!(!context.is_current(&staged));
            assert_eq!(context.epoch(), 1);
        });
    }

    #[test]
    fn registration_updates_and_cleanup_release_the_exact_byte_budget() {
        Owner::new().with(|| {
            let input = vec!["p-2".to_string()];
            let bytes = fragment_bytes(&input);
            let context = CnContext::new(Cn::default(), bytes);
            let id = context.register(input.clone()).unwrap();
            assert!(context.register(input.clone()).is_err());
            assert!(context.update(id, vec!["a".repeat(bytes)]).is_err());
            assert_eq!(context.raw.borrow().fragments[&id], input);
            context.update(id, vec!["x".into()]).unwrap();
            context.unregister(id);
            assert_eq!(context.raw.borrow().bytes, 0);
            assert!(context.register(input).is_ok());
        });
    }
}
