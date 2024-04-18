use std::{collections::HashMap, hash::Hash};

pub(crate) struct StMap<K, V> {
    data: HashMap<K, V>,
}
impl<K: Eq + Hash, V> StMap<K, V> {
    pub(crate) fn new() -> StMap<K, V> {
        Self { data: HashMap::new() }
    }
    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K, &V)> {
        self.data.iter()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.data.insert(key, val)
    }
}
