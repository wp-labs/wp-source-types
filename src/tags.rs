use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smol_str::SmolStr;

const INLINE_TAG_CAPACITY: usize = 16;

/// A lightweight, sorted collection of key-value string tags.
///
/// Tags are stored in a `SmallVec` with inline capacity for up to 16 entries,
/// avoiding heap allocation for typical use cases. Keys are kept sorted
/// for efficient binary search lookup.
///
/// Both keys and values use `SmolStr` for small string optimization,
/// providing zero-allocation storage for strings ≤22 bytes.
///
/// # Example
/// ```ignore
/// let mut tags = Tags::new();
/// tags.set("env", "prod");
/// tags.set("region", "us-west");
///
/// assert_eq!(tags.get("env"), Some("prod"));
/// assert!(tags.contains_key("region"));
/// ```
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Tags {
    item: SmallVec<[(SmolStr, SmolStr); INLINE_TAG_CAPACITY]>,
}

impl Tags {
    /// Create a new empty Tags collection.
    pub fn new() -> Self {
        Self {
            item: SmallVec::new(),
        }
    }

    /// Set a tag value. If the key exists, the value is updated.
    ///
    /// Keys are kept sorted for efficient lookup.
    pub fn set(&mut self, key: impl Into<SmolStr>, value: impl Into<SmolStr>) {
        let key = key.into();
        let value = value.into();
        match self
            .item
            .binary_search_by(|(existing, _)| existing.as_str().cmp(key.as_str()))
        {
            Ok(idx) => {
                self.item[idx].1 = value;
            }
            Err(idx) => {
                self.item.insert(idx, (key, value));
            }
        }
    }

    /// Get a tag value by key.
    ///
    /// Returns `None` if the key doesn't exist.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.item
            .binary_search_by(|(existing, _)| existing.as_str().cmp(key))
            .ok()
            .and_then(|idx| self.item.get(idx))
            .map(|(_, val)| val.as_str())
    }

    /// Check if a key exists in the tags.
    pub fn contains_key(&self, key: &str) -> bool {
        self.item
            .binary_search_by(|(existing, _)| existing.as_str().cmp(key))
            .is_ok()
    }

    /// Remove a tag by key.
    ///
    /// Returns the removed value if the key existed.
    pub fn remove(&mut self, key: &str) -> Option<SmolStr> {
        match self
            .item
            .binary_search_by(|(existing, _)| existing.as_str().cmp(key))
        {
            Ok(idx) => Some(self.item.remove(idx).1),
            Err(_) => None,
        }
    }

    /// Check if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.item.is_empty()
    }

    /// Return the number of tags.
    pub fn len(&self) -> usize {
        self.item.len()
    }

    /// Iterate over all key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.item.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// Iterate over all keys.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.item.iter().map(|(k, _)| k.as_str())
    }

    /// Iterate over all values.
    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.item.iter().map(|(_, v)| v.as_str())
    }

    /// Clear all tags.
    pub fn clear(&mut self) {
        self.item.clear();
    }
}

// Deprecated compatibility alias
impl Tags {
    /// Set a tag value.
    #[deprecated(since = "0.6.0", note = "Use `set()` instead")]
    pub fn set_tag(&mut self, key: &str, value: String) {
        self.set(SmolStr::from(key), SmolStr::from(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_keep_sorted_insert_and_update() {
        let mut tags = Tags::new();
        tags.set("b", "2");
        tags.set("a", "1");
        tags.set("c", "3");
        assert_eq!(tags.get("a"), Some("1"));
        assert_eq!(tags.get("b"), Some("2"));
        assert_eq!(tags.get("c"), Some("3"));
        // Update
        tags.set("b", "updated");
        assert_eq!(tags.get("b"), Some("updated"));
    }

    #[test]
    fn tags_helpers_cover_empty_and_missing() {
        let tags = Tags::new();
        assert!(tags.is_empty());
        assert_eq!(tags.len(), 0);
        assert_eq!(tags.get("nonexistent"), None);
    }

    #[test]
    fn tags_contains_key() {
        let mut tags = Tags::new();
        tags.set("x", "v");
        assert!(tags.contains_key("x"));
        assert!(!tags.contains_key("y"));
    }

    #[test]
    fn tags_remove() {
        let mut tags = Tags::new();
        tags.set("x", "v");
        assert_eq!(tags.remove("x"), Some(SmolStr::from("v")));
        assert!(tags.is_empty());
        assert_eq!(tags.remove("x"), None);
    }

    #[test]
    fn tags_iter() {
        let mut tags = Tags::new();
        tags.set("b", "2");
        tags.set("a", "1");
        let pairs: Vec<_> = tags.iter().collect();
        assert_eq!(pairs, vec![("a", "1"), ("b", "2")]);
    }

    #[test]
    fn tags_keys_and_values() {
        let mut tags = Tags::new();
        tags.set("x", "v");
        let keys: Vec<_> = tags.keys().collect();
        let values: Vec<_> = tags.values().collect();
        assert_eq!(keys, vec!["x"]);
        assert_eq!(values, vec!["v"]);
    }

    #[test]
    fn tags_clear() {
        let mut tags = Tags::new();
        tags.set("x", "v");
        tags.clear();
        assert!(tags.is_empty());
    }

    #[test]
    fn tags_deprecated_set_tag_still_works() {
        let mut tags = Tags::new();
        #[allow(deprecated)]
        tags.set_tag("k", "value".into());
        assert_eq!(tags.get("k"), Some("value"));
    }
}
