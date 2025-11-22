use std::{
    collections::hash_map::Entry,
    hash::{BuildHasher, Hash},
};

use rustc_hash::FxBuildHasher;

use super::{map::Map, set::Set};

pub(crate) type RMapRSet<K, V> = MapSet<K, V, FxBuildHasher, FxBuildHasher>;

#[derive(Clone)]
pub(crate) struct MapSet<K, V, H1, H2> {
    data: Map<K, Set<V, H2>, H1>,
    buffer: Set<V, H2>,
}
impl<K, V, H1, H2> MapSet<K, V, H1, H2>
where
    K: Eq + Hash,
    V: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self {
            data: Map::new(),
            buffer: Set::new(),
        }
    }
    pub(crate) fn get(&self, key: &K) -> impl ExactSizeIterator<Item = &V> + use<'_, K, V, H1, H2> {
        match self.data.get(key) {
            Some(v) => v.iter(),
            // Buffer should be empty when this method is called
            None => self.buffer.iter(),
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K, impl ExactSizeIterator<Item = &V>)> {
        self.data.iter().map(|(k, v)| (k, v.iter()))
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = &K> {
        self.data.keys()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = impl ExactSizeIterator<Item = &V>> {
        self.data.values().map(|v| v.iter())
    }
    pub(crate) fn values_inner(&self) -> impl ExactSizeIterator<Item = &Set<V, H2>> {
        self.data.values()
    }
    pub(crate) fn contains_entry(&self, key: &K, entry: &V) -> bool {
        match self.data.get(key) {
            Some(v) => v.contains(entry),
            None => false,
        }
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key: K, entry: V) {
        self.data
            .entry(key)
            .or_insert_with(|| Set::with_capacity(1))
            .insert(entry);
    }
    pub(crate) fn extend_entries(&mut self, key: K, entries: impl ExactSizeIterator<Item = V>) {
        if entries.is_empty() {
            return;
        }
        self.data
            .entry(key)
            .or_insert_with(|| Set::with_capacity(entries.len()))
            .extend(entries);
    }
    pub(crate) fn remove_entry(&mut self, key: &K, entry: &V) -> bool {
        // Returns true only if key has been removed
        let need_cleanup = match self.data.get_mut(key) {
            None => return false,
            Some(v) => v.remove(entry) && v.is_empty(),
        };
        if need_cleanup {
            self.data.remove(key);
        }
        need_cleanup
    }
    pub(crate) fn remove_key(&mut self, key: &K) -> Option<impl ExactSizeIterator<Item = V> + use<K, V, H1, H2>> {
        self.data.remove(key).map(|v| v.into_iter())
    }
    // Buffer methods
    pub(crate) fn buffer_if<F>(&mut self, key: K, filter: F)
    where
        F: FnMut(&V) -> bool,
    {
        if let Entry::Occupied(mut entry) = self.data.entry(key) {
            let set = entry.get_mut();
            self.buffer.extend(set.extract_if(filter));
            if set.is_empty() {
                entry.remove();
            }
        }
    }
    pub(crate) fn iter_buffer(&self) -> impl ExactSizeIterator<Item = &V> {
        self.buffer.iter()
    }
    pub(crate) fn drain_buffer(&mut self) -> impl ExactSizeIterator<Item = V> {
        self.buffer.drain()
    }
}
impl<K, V, H1, H2> Default for MapSet<K, V, H1, H2>
where
    K: Eq + Hash,
    V: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn extend_vec_from_map_set_l1<K, V, H1, H2>(vec: &mut Vec<V>, storage: &MapSet<K, V, H1, H2>, key: &K)
where
    K: Eq + Hash,
    V: Eq + Hash + Copy,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    vec.extend(storage.get(key).copied());
}
