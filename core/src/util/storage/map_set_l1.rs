use std::hash::Hash;

use super::{StMap, StSet};

#[derive(Clone)]
pub(crate) struct StMapSetL1<K, V> {
    data: StMap<K, StSet<V>>,
    empty: StSet<V>,
}
impl<K: Eq + Hash, V: Eq + Hash> StMapSetL1<K, V> {
    pub(crate) fn new() -> Self {
        Self {
            data: StMap::new(),
            empty: StSet::new(),
        }
    }
    pub(crate) fn get(&self, key: &K) -> impl ExactSizeIterator<Item = &V> + use<'_, K, V> {
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
    pub(crate) fn values_inner(&self) -> impl ExactSizeIterator<Item = &StSet<V>> {
        self.data.values()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key: K, entry: V) {
        self.data
            .entry(key)
            .or_insert_with(|| StSet::with_capacity(1))
            .insert(entry);
    }
    pub(crate) fn extend_entries(&mut self, key: K, entries: impl ExactSizeIterator<Item = V>) {
        self.data
            .entry(key)
            .or_insert_with(|| StSet::with_capacity(entries.len()))
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
    pub(crate) fn remove_key(&mut self, key: &K) -> Option<impl ExactSizeIterator<Item = V> + use<K, V>> {
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
impl<K: Eq + Hash, V: Eq + Hash> Default for StMapSetL1<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn extend_vec_from_map_set_l1<K: Eq + Hash, V: Eq + Hash + Copy>(
    vec: &mut Vec<V>,
    storage: &StMapSetL1<K, V>,
    key: &K,
) {
    vec.extend(storage.get(key).copied());
}
