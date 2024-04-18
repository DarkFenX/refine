use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub(crate) struct KsL1Set<K, V> {
    data: HashMap<K, HashSet<V>>,
    empty: HashSet<V>,
}
impl<K: Eq + Hash, V: Eq + Hash> KsL1Set<K, V> {
    pub(crate) fn new() -> KsL1Set<K, V> {
        Self {
            data: HashMap::new(),
            empty: HashSet::new(),
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
        let values = self.data.entry(key).or_insert_with(|| HashSet::with_capacity(1));
        values.insert(entry);
    }
    pub(crate) fn extend_entries(&mut self, key: K, entries: impl ExactSizeIterator<Item = V>) {
        let values = self
            .data
            .entry(key)
            .or_insert_with(|| HashSet::with_capacity(entries.len()));
        values.extend(entries);
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
    pub(crate) fn remove_key(&mut self, key: &K) -> Option<HashSet<V>> {
        self.data.remove(key)
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

pub(crate) fn extend_vec_from_l1set<K: Eq + Hash, V: Eq + Hash + Clone>(
    vec: &mut Vec<V>,
    storage: &KsL1Set<K, V>,
    key: &K,
) {
    vec.extend(storage.get(key).map(|v| v.clone()));
}
