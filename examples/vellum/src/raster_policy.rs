use crate::document::Node;
use std::collections::HashMap;

pub const CACHE_BUDGET: usize = 64 * 1024 * 1024;
pub const MAX_RASTER_DIMENSION: u32 = 2044;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RasterDimensions {
    pub padding: f64,
    pub width: f64,
    pub height: f64,
    pub scale: f64,
    pub pixel_width: u32,
    pub pixel_height: u32,
}

pub fn resolution(zoom: f64, dpr: f64) -> f64 {
    2.0_f64.powf((zoom * dpr).log2().ceil()).clamp(0.5, 4.0)
}

pub fn dimensions(node: &Node, resolution: f64) -> RasterDimensions {
    let padding = if matches!(node.kind.as_str(), "path" | "line") {
        node.stroke_width / 2.0 + 1.0
    } else {
        0.0
    };
    let width = node.w + padding * 2.0;
    let height = node.h + padding * 2.0;
    let scale = resolution
        .min(f64::from(MAX_RASTER_DIMENSION) / width.max(1.0))
        .min(f64::from(MAX_RASTER_DIMENSION) / height.max(1.0));
    RasterDimensions {
        padding,
        width,
        height,
        scale,
        pixel_width: (width * scale)
            .ceil()
            .max(1.0)
            .min(f64::from(MAX_RASTER_DIMENSION)) as u32,
        pixel_height: (height * scale)
            .ceil()
            .max(1.0)
            .min(f64::from(MAX_RASTER_DIMENSION)) as u32,
    }
}

pub fn cache_key(node: &Node, resolution: f64) -> String {
    format!("{}:{}:{resolution}", node.id, node.version)
}

struct Entry<V> {
    value: V,
    bytes: usize,
    used: u64,
}

pub struct LruCache<V> {
    entries: HashMap<String, Entry<V>>,
    budget: usize,
    bytes: usize,
    clock: u64,
}

impl<V> Default for LruCache<V> {
    fn default() -> Self {
        Self::with_budget(CACHE_BUDGET)
    }
}

impl<V> LruCache<V> {
    pub fn with_budget(budget: usize) -> Self {
        Self {
            entries: HashMap::new(),
            budget,
            bytes: 0,
            clock: 0,
        }
    }
    pub fn get(&mut self, key: &str) -> Option<&V> {
        let entry = self.entries.get_mut(key)?;
        self.clock += 1;
        entry.used = self.clock;
        Some(&entry.value)
    }
    /// Stores an entry, evicting older entries; oversized entries leave the cache intact.
    pub fn insert(&mut self, key: String, value: V, bytes: usize) -> bool {
        if bytes > self.budget {
            return false;
        }
        self.remove(&key);
        while self.bytes > self.budget - bytes {
            let oldest = self
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.used)
                .map(|(key, _)| key.clone());
            let Some(oldest) = oldest else {
                break;
            };
            self.remove(&oldest);
        }
        self.clock += 1;
        self.entries.insert(
            key,
            Entry {
                value,
                bytes,
                used: self.clock,
            },
        );
        self.bytes += bytes;
        true
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        let entry = self.entries.remove(key)?;
        self.bytes -= entry.bytes;
        Some(entry.value)
    }
    pub fn clear(&mut self) {
        self.entries.clear();
        self.bytes = 0;
        self.clock = 0;
    }
    pub fn bytes(&self) -> usize {
        self.bytes
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn budget(&self) -> usize {
        self.budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantizes_resolution_upward_in_powers_of_two_and_clamps() {
        let samples = [
            (0.01, 0.5),
            (0.5, 0.5),
            (0.5001, 1.0),
            (1.0, 1.0),
            (1.001, 2.0),
            (2.01, 4.0),
            (20.0, 4.0),
        ];
        for (zoom, expected) in samples {
            assert_eq!(resolution(zoom, 1.0), expected);
        }
        assert_eq!(resolution(0.6, 2.0), 2.0);
    }

    #[test]
    fn pads_only_paths_and_lines_for_strokes() {
        for kind in ["path", "line", "text", "image"] {
            let mut node = Node::new(kind);
            node.w = 100.0;
            node.h = 50.0;
            node.stroke_width = 8.0;
            let d = dimensions(&node, 2.0);
            let expected = if matches!(kind, "path" | "line") {
                (5.0, 110.0, 60.0, 220, 120)
            } else {
                (0.0, 100.0, 50.0, 200, 100)
            };
            assert_eq!(
                (d.padding, d.width, d.height, d.pixel_width, d.pixel_height),
                expected
            );
        }
    }

    #[test]
    fn caps_raster_size_at_2044_and_preserves_aspect_ratio() {
        let mut node = Node::new("image");
        node.w = 4000.0;
        node.h = 2000.0;
        let d = dimensions(&node, 4.0);
        assert_eq!(
            (d.scale, d.pixel_width, d.pixel_height),
            (0.511, 2044, 1022)
        );
    }

    #[test]
    fn allocates_at_least_one_pixel_for_empty_geometry() {
        let mut node = Node::new("text");
        node.w = 0.0;
        node.h = 0.0;
        let d = dimensions(&node, 0.5);
        assert_eq!((d.pixel_width, d.pixel_height), (1, 1));
    }

    #[test]
    fn cache_key_changes_with_node_version_and_resolution() {
        let mut node = Node::new("text");
        node.id = "label".into();
        node.version = 7;
        assert_eq!(cache_key(&node, 0.5), "label:7:0.5");
    }

    #[test]
    fn evicts_least_recently_accessed_entries_with_a_64_mib_default() {
        let mut cache = LruCache::default();
        let quarter = CACHE_BUDGET / 4;
        for key in ["a", "b", "c", "d"] {
            assert!(cache.insert(key.into(), key, quarter));
        }
        assert_eq!(cache.get("a"), Some(&"a"));
        assert!(cache.insert("e".into(), "e", quarter));
        assert_eq!(cache.get("b"), None);
        assert_eq!(
            (cache.budget(), cache.bytes(), cache.len()),
            (CACHE_BUDGET, CACHE_BUDGET, 4)
        );
    }

    #[test]
    fn replacement_updates_bytes_and_access_order_without_stale_entries() {
        let mut cache = LruCache::with_budget(10);
        cache.insert("a".into(), 1, 4);
        cache.insert("b".into(), 2, 4);
        cache.insert("a".into(), 3, 7);
        assert_eq!(
            (
                cache.get("b").copied(),
                cache.get("a").copied(),
                cache.bytes()
            ),
            (None, Some(3), 7)
        );
    }

    #[test]
    fn oversized_entry_does_not_destroy_existing_working_set() {
        let mut cache = LruCache::with_budget(10);
        cache.insert("a".into(), 1, 4);
        assert!(!cache.insert("a".into(), 2, 11));
        assert_eq!((cache.get("a").copied(), cache.bytes()), (Some(1), 4));
    }

    #[test]
    fn remove_and_clear_release_cache_accounting() {
        let mut cache = LruCache::with_budget(10);
        cache.insert("a".into(), 1, 4);
        cache.insert("b".into(), 2, 4);
        assert_eq!(cache.remove("a"), Some(1));
        assert_eq!(cache.bytes(), 4);
        cache.clear();
        assert_eq!((cache.bytes(), cache.is_empty()), (0, true));
    }
}
