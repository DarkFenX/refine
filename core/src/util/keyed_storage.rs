use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub(crate) struct KeyedStorage<K, V> {
    data: HashMap<K, HashSet<V>>,
}
impl<K: Eq + Hash, V: Eq + Hash> KeyedStorage<K, V> {
    pub(crate) fn new() -> KeyedStorage<K, V> {
        KeyedStorage { data: HashMap::new() }
    }
    pub(crate) fn get(&self, key: &K) -> Option<&HashSet<V>> {
        self.data.get(key)
    }
    pub(crate) fn add_entry(&mut self, key: K, entry: V) {
        let values = self.data.entry(key).or_insert_with(|| HashSet::new());
        values.insert(entry);
    }
    pub(crate) fn rm_entry(&mut self, key: &K, entry: &V) {
        match self.data.get_mut(key) {
            None => return,
            Some(v) => v.remove(entry),
        };
    }
}
