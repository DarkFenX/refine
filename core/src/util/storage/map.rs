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
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = &K> {
        self.data.keys()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &V> {
        self.data.values()
    }
    pub(crate) fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }
    pub(crate) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut V> {
        self.data.values_mut()
    }
    pub(crate) fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.data.insert(key, val)
    }
    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
    pub(crate) fn retain(&mut self, func: impl FnMut(&K, &mut V) -> bool) {
        self.data.retain(func)
    }
}
