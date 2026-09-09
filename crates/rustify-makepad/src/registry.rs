use std::collections::BTreeMap;

/// Handle to a live GPU region. Ids are allocated from a monotonic counter
/// and never reused, so a handle that outlives its region keeps failing
/// lookups instead of silently pointing at a newer region in the same slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionId(u32);

impl RegionId {
    pub fn raw(self) -> u32 {
        self.0
    }

    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }
}

pub struct Registry<T> {
    next: u32,
    live: BTreeMap<RegionId, T>,
}

impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self {
            next: 1,
            live: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> RegionId {
        let id = RegionId(self.next);
        self.next += 1;
        self.live.insert(id, value);
        id
    }

    pub fn get(&self, id: RegionId) -> Option<&T> {
        self.live.get(&id)
    }

    pub fn get_mut(&mut self, id: RegionId) -> Option<&mut T> {
        self.live.get_mut(&id)
    }

    pub fn remove(&mut self, id: RegionId) -> Option<T> {
        self.live.remove(&id)
    }

    pub fn len(&self) -> usize {
        self.live.len()
    }

    pub fn is_empty(&self) -> bool {
        self.live.is_empty()
    }

    pub fn ids(&self) -> impl Iterator<Item = RegionId> + '_ {
        self.live.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique_and_never_reused() {
        let mut registry = Registry::new();
        let a = registry.insert("a");
        let b = registry.insert("b");
        assert_ne!(a, b);
        assert_eq!(registry.remove(a), Some("a"));
        let c = registry.insert("c");
        assert_ne!(c, a);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn removed_ids_no_longer_resolve() {
        let mut registry = Registry::new();
        let a = registry.insert(1);
        assert_eq!(registry.get_mut(a).copied(), Some(1));
        assert_eq!(registry.remove(a), Some(1));
        assert!(registry.get_mut(a).is_none());
        assert_eq!(registry.remove(a), None);
    }

    #[test]
    fn ids_round_trip_through_their_raw_form() {
        let mut registry = Registry::new();
        let a = registry.insert(());
        assert_eq!(RegionId::from_raw(a.raw()), a);
        assert!(registry.get_mut(RegionId::from_raw(a.raw() + 1)).is_none());
    }
}
