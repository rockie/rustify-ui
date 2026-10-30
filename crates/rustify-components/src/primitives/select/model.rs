use super::SelectEntry;
use std::collections::{HashMap, HashSet};

/// Metadata is rebuilt on entries changes, never from rendered option text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Index {
    pub entries: Vec<SelectEntry>,
    by_value: HashMap<String, usize>,
    normalized: Vec<String>,
    pub invalid: HashSet<String>,
    enabled: Vec<usize>,
    enabled_rank: HashMap<String, usize>,
}
impl Index {
    pub fn new(entries: Vec<SelectEntry>) -> Self {
        let mut by_value = HashMap::with_capacity(entries.len());
        let mut invalid = HashSet::new();
        for (at, entry) in entries.iter().enumerate() {
            if entry.value.is_empty() || by_value.insert(entry.value.clone(), at).is_some() {
                invalid.insert(entry.value.clone());
            }
        }
        let enabled: Vec<_> = entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| !entry.disabled && !invalid.contains(&entry.value))
            .map(|(at, _)| at)
            .collect();
        let enabled_rank = enabled
            .iter()
            .enumerate()
            .map(|(rank, &at)| (entries[at].value.clone(), rank))
            .collect();
        let normalized = entries
            .iter()
            .map(|entry| entry.text_value.to_lowercase())
            .collect();
        Self {
            entries,
            by_value,
            normalized,
            invalid,
            enabled,
            enabled_rank,
        }
    }
    pub fn entry(&self, value: &str) -> Option<&SelectEntry> {
        (!self.invalid.contains(value))
            .then(|| self.by_value.get(value).map(|&at| &self.entries[at]))
            .flatten()
    }
    pub fn available(&self, value: &str) -> bool {
        self.enabled_rank.contains_key(value)
    }
    pub fn enabled(&self) -> impl Iterator<Item = &SelectEntry> {
        self.enabled.iter().map(|&at| &self.entries[at])
    }
    pub fn edge(&self, last: bool, usable: impl Fn(&str) -> bool) -> Option<String> {
        if last {
            self.enabled
                .iter()
                .rev()
                .find(|&&at| usable(&self.entries[at].value))
        } else {
            self.enabled
                .iter()
                .find(|&&at| usable(&self.entries[at].value))
        }
        .map(|&at| self.entries[at].value.clone())
    }
    pub fn step(
        &self,
        current: Option<&str>,
        delta: i32,
        usable: impl Fn(&str) -> bool,
    ) -> Option<String> {
        let len = self.enabled.len();
        if len == 0 {
            return None;
        }
        let here = current
            .and_then(|current| self.enabled_rank.get(current))
            .map(|&at| at as i32)
            .unwrap_or(if delta < 0 { 0 } else { -1 });
        (1..=len)
            .map(|step| (here + delta * step as i32).rem_euclid(len as i32) as usize)
            .map(|at| &self.entries[self.enabled[at]].value)
            .find(|value| usable(value))
            .cloned()
    }
    pub fn search(
        &self,
        query: &str,
        current: Option<&str>,
        usable: impl Fn(&str) -> bool,
    ) -> Option<String> {
        let mut characters = query.chars();
        let first = characters.next()?;
        let repeated = characters.all(|character| character == first);
        let query = if repeated {
            first.to_string()
        } else {
            query.to_string()
        }
        .to_lowercase();
        let single = query.chars().count() == 1;
        let len = self.enabled.len();
        if len == 0 {
            return None;
        }
        let here = current
            .and_then(|current| self.enabled_rank.get(current))
            .copied()
            .unwrap_or(0);
        (0..len)
            .map(|step| self.enabled[(here + step) % len])
            .filter(|&at| !single || current != Some(self.entries[at].value.as_str()))
            .find(|&at| usable(&self.entries[at].value) && self.normalized[at].starts_with(&query))
            .map(|at| self.entries[at].value.clone())
            .filter(|next| Some(next.as_str()) != current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn index() -> Index {
        Index::new(vec![
            SelectEntry::new("a", "Apricot"),
            SelectEntry::new("b", "Banana"),
            SelectEntry::new("c", "Blackberry"),
            SelectEntry::new("d", "Blueberry").disabled(),
            SelectEntry::new("e", "Cherry"),
        ])
    }
    #[test]
    fn metadata_drives_labels_and_navigation_without_any_mounted_items() {
        let index = index();
        assert_eq!(index.entry("b").unwrap().text_value, "Banana");
        assert_eq!(index.edge(false, |_| true).as_deref(), Some("a"));
        assert_eq!(index.edge(true, |_| true).as_deref(), Some("e"));
        assert_eq!(index.step(Some("c"), 1, |_| true).as_deref(), Some("e"));
        assert_eq!(index.step(Some("a"), -1, |_| true).as_deref(), Some("e"));
        assert!(!index.available("d"));
    }
    #[test]
    fn repeated_characters_cycle_and_a_longer_prefix_keeps_its_meaning() {
        let index = index();
        assert_eq!(index.search("b", Some("a"), |_| true).as_deref(), Some("b"));
        assert_eq!(
            index.search("bb", Some("b"), |_| true).as_deref(),
            Some("c")
        );
        assert_eq!(
            index.search("bbb", Some("c"), |_| true).as_deref(),
            Some("b")
        );
        assert_eq!(
            index.search("bl", Some("b"), |_| true).as_deref(),
            Some("c")
        );
        assert_eq!(index.search("BL", Some("c"), |_| true), None);
        assert_eq!(index.search("bl", Some("b"), |value| value != "c"), None);
    }
    #[test]
    fn duplicate_and_empty_entry_identities_are_not_selectable_or_shown_as_values() {
        let index = Index::new(vec![
            SelectEntry::new("", "Placeholder collision"),
            SelectEntry::new("a", "One"),
            SelectEntry::new("a", "Two"),
            SelectEntry::new("b", "Valid"),
        ]);
        assert!(index.entry("").is_none() && index.entry("a").is_none());
        assert_eq!(index.edge(false, |_| true).as_deref(), Some("b"));
        assert_eq!(index.invalid.len(), 2);
    }
    #[test]
    fn unicode_search_and_a_missing_rendered_item_do_not_create_an_invalid_target() {
        let index = Index::new(vec![
            SelectEntry::new("a", "苹果"),
            SelectEntry::new("b", "苹果汁"),
            SelectEntry::new("c", "梨"),
        ]);
        assert_eq!(index.search("苹", None, |_| true).as_deref(), Some("a"));
        assert_eq!(
            index.search("苹苹", Some("a"), |_| true).as_deref(),
            Some("b")
        );
        assert_eq!(
            index.search("苹果", None, |value| value != "a").as_deref(),
            Some("b")
        );
        assert_eq!(index.edge(false, |_| false), None);
    }
}
