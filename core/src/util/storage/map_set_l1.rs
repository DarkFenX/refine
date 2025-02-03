use std::hash::Hash;

use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

#[derive(Clone)]
pub(crate) struct StMapSetL1<K, V> {
    data: FxHashMap<K, FxHashSet<V>>,
    empty: FxHashSet<V>,
}
impl<K: Eq + Hash, V: Eq + Hash> StMapSetL1<K, V> {
    pub(crate) fn new() -> StMapSetL1<K, V> {
        Self {
            data: FxHashMap::default(),
            empty: FxHashSet::default(),
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
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = impl ExactSizeIterator<Item = &V>> {
        self.data.values().map(|v| v.iter())
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key: K, entry: V) {
        self.data
            .entry(key)
            .or_insert_with(|| FxHashSet::with_capacity_and_hasher(1, FxBuildHasher))
            .insert(entry);
    }
    pub(crate) fn extend_entries(&mut self, key: K, entries: impl ExactSizeIterator<Item = V>) {
        self.data
            .entry(key)
            .or_insert_with(|| FxHashSet::with_capacity_and_hasher(entries.len(), FxBuildHasher))
            .extend(entries);
    }
    pub(crate) fn remove_entry(&mut self, key: &K, entry: &V) {
        let need_cleanup = match self.data.get_mut(key) {
            None => return,
            Some(v) => {
                v.remove(entry);
                v.is_empty()
            }
        };
        if need_cleanup {
            self.data.remove(key);
        }
    }
    pub(crate) fn remove_key(&mut self, key: &K) -> Option<impl ExactSizeIterator<Item = V>> {
        self.data.remove(key).map(|v| v.into_iter())
    }
    pub(crate) fn drain_entries<'a>(&mut self, key: &K, entries: impl Iterator<Item = &'a V>)
    where
        V: 'a,
    {
        let need_cleanup = match self.data.get_mut(key) {
            None => return,
            Some(v) => {
                for entry in entries {
                    v.remove(entry);
                }
                v.is_empty()
            }
        };
        if need_cleanup {
            self.data.remove(key);
        }
    }
}

pub(crate) fn extend_vec_from_map_set_l1<K: Eq + Hash, V: Eq + Hash + Copy>(
    vec: &mut Vec<V>,
    storage: &StMapSetL1<K, V>,
    key: &K,
) {
    vec.extend(storage.get(key).copied());
}
