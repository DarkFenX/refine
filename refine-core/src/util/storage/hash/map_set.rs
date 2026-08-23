use std::{
    collections::hash_map::Entry,
    hash::{BuildHasher, Hash},
};

use super::{
    map::{Map, RMap},
    set::{RSet, Set},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type RMapRSet<K, V> = MapSet<K, V, rustc_hash::FxBuildHasher, rustc_hash::FxBuildHasher>;
impl<K, V> Default for RMapRSet<K, V> {
    fn default() -> Self {
        Self {
            data: RMap::new(),
            buffer: RSet::new(),
        }
    }
}
impl<K, V> RMapRSet<K, V> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
impl<K, V> RMapRSet<K, V>
where
    K: Eq + Hash,
    V: Eq + Hash,
{
    pub(crate) fn add_entry(&mut self, key: K, value: V) {
        self.data
            .entry(key)
            .or_insert_with(|| RSet::with_capacity(1))
            .insert(value);
    }
    pub(crate) fn extend_entries(&mut self, key: K, entries: impl ExactSizeIterator<Item = V>) {
        if entries.len() == 0 {
            return;
        }
        self.data
            .entry(key)
            .or_insert_with(|| RSet::with_capacity(entries.len()))
            .extend(entries);
    }
    pub(crate) fn remove_key(&mut self, key: &K) -> impl ExactSizeIterator<Item = V> + use<K, V> {
        self.data.remove(key).unwrap_or_default().into_iter()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct MapSet<K, V, H1, H2> {
    data: Map<K, Set<V, H2>, H1>,
    buffer: Set<V, H2>,
}
impl<K, V, H1, H2> MapSet<K, V, H1, H2>
where
    K: Eq + Hash,
    V: Eq + Hash,
    H1: BuildHasher,
    H2: BuildHasher,
{
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
    #[expect(dead_code)]
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = &K> {
        self.data.keys()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = impl ExactSizeIterator<Item = &V>> {
        self.data.values().map(|v| v.iter())
    }
    pub(crate) fn values_inner(&self) -> impl ExactSizeIterator<Item = &Set<V, H2>> {
        self.data.values()
    }
    #[expect(dead_code)]
    pub(crate) fn contains_entry(&self, key: &K, value: &V) -> bool {
        match self.data.get(key) {
            Some(set) => set.contains(value),
            None => false,
        }
    }
    #[expect(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn remove_entry(&mut self, key: K, value: &V) {
        if let Entry::Occupied(mut entry) = self.data.entry(key) {
            let set = entry.get_mut();
            set.remove(value);
            if set.is_empty() {
                entry.remove();
            }
        }
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
    pub(crate) fn drain_buffer(&mut self) -> impl ExactSizeIterator<Item = V> {
        self.buffer.drain()
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
