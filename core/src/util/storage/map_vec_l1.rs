use std::hash::Hash;

use rustc_hash::FxHashMap;

pub(crate) struct StMapVecL1<K, V> {
    data: FxHashMap<K, Vec<V>>,
    empty: Vec<V>,
}
impl<K: Eq + Hash, V> StMapVecL1<K, V> {
    pub(crate) fn new() -> StMapVecL1<K, V> {
        Self {
            data: FxHashMap::default(),
            empty: Vec::new(),
        }
    }
    pub(crate) fn get(&self, key: &K) -> impl ExactSizeIterator<Item = &V> {
        match self.data.get(key) {
            Some(v) => v.iter(),
            None => self.empty.iter(),
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K, impl ExactSizeIterator<Item = &V>)> {
        self.data.iter().map(|(k, v)| (k, v.iter()))
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key: K, entry: V) {
        let values = self.data.entry(key).or_insert_with(|| Vec::with_capacity(1));
        values.push(entry);
    }
    pub(crate) fn extend_entries(&mut self, key: K, entries: impl ExactSizeIterator<Item = V>) {
        let values = self
            .data
            .entry(key)
            .or_insert_with(|| Vec::with_capacity(entries.len()));
        values.extend(entries);
    }
}
impl<K, V> IntoIterator for StMapVecL1<K, V> {
    type Item = (K, Vec<V>);
    type IntoIter = std::collections::hash_map::IntoIter<K, Vec<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
