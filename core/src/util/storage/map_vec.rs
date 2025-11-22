use std::hash::{BuildHasher, Hash};

use rustc_hash::FxBuildHasher;

use super::map::Map;

pub(crate) type RMapVec<K, V> = MapVec<K, V, FxBuildHasher>;

pub(crate) struct MapVec<K, V, H> {
    data: Map<K, Vec<V>, H>,
    empty: Vec<V>,
}
impl<K, V, H> MapVec<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self {
            data: Map::new(),
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
        if entries.is_empty() {
            return;
        }
        let values = self
            .data
            .entry(key)
            .or_insert_with(|| Vec::with_capacity(entries.len()));
        values.extend(entries);
    }
    // Consumption methods
    pub(crate) fn into_values(self) -> impl Iterator<Item = Vec<V>> {
        self.data.into_values()
    }
}
impl<K, V, H> Default for MapVec<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<K, V, H> IntoIterator for MapVec<K, V, H> {
    type Item = (K, Vec<V>);
    type IntoIter = std::collections::hash_map::IntoIter<K, Vec<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
