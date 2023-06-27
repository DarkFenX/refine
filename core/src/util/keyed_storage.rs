use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub(crate) struct KeyedStorage1L<K, V> {
    data: HashMap<K, HashSet<V>>,
}
impl<K: Eq + Hash, V: Eq + Hash> KeyedStorage1L<K, V> {
    pub(crate) fn new() -> KeyedStorage1L<K, V> {
        Self { data: HashMap::new() }
    }
    pub(crate) fn get(&self, key: &K) -> Option<&HashSet<V>> {
        self.data.get(key)
    }
    pub(crate) fn add(&mut self, key: K, entry: V) {
        let values = self.data.entry(key).or_insert_with(|| HashSet::with_capacity(1));
        values.insert(entry);
    }
    pub(crate) fn extend<I: Iterator<Item = V> + ExactSizeIterator>(&mut self, key: K, entries: I) {
        let values = self
            .data
            .entry(key)
            .or_insert_with(|| HashSet::with_capacity(entries.len()));
        values.extend(entries);
    }
    pub(crate) fn remove(&mut self, key: &K, entry: &V) {
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
    pub(crate) fn drain<I: Iterator<Item = V>>(&mut self, key: &K, entries: I) {
        let need_cleanup = match self.data.get_mut(key) {
            None => return,
            Some(v) => {
                for entry in entries {
                    v.remove(&entry);
                }
                v.is_empty()
            }
        };
        if need_cleanup {
            self.data.remove(key);
        }
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub(crate) struct KeyedStorage2L<A, B, V> {
    data: HashMap<A, KeyedStorage1L<B, V>>,
}
impl<A: Eq + Hash, B: Eq + Hash, V: Eq + Hash> KeyedStorage2L<A, B, V> {
    pub(crate) fn new() -> KeyedStorage2L<A, B, V> {
        Self { data: HashMap::new() }
    }
    pub(crate) fn get_l1(&self, key1: &A) -> Option<&KeyedStorage1L<B, V>> {
        self.data.get(key1)
    }
    pub(crate) fn get_l2(&self, key1: &A, key2: &B) -> Option<&HashSet<V>> {
        match self.get_l1(key1) {
            Some(ks1l) => ks1l.get(key2),
            None => None,
        }
    }
    pub(crate) fn add(&mut self, key1: A, key2: B, entry: V) {
        let ks1l = self.data.entry(key1).or_insert_with(|| KeyedStorage1L::new());
        ks1l.add(key2, entry);
    }
    pub(crate) fn remove(&mut self, key1: &A, key2: &B, entry: &V) {
        let need_cleanup = match self.data.get_mut(key1) {
            None => return,
            Some(v) => {
                v.remove(key2, entry);
                v.is_empty()
            }
        };
        if need_cleanup {
            self.data.remove(key1);
        }
    }
}
