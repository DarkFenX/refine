use std::{collections::HashMap, hash::Hash};

use super::KsL1Set;

pub(crate) struct KsL2Set<A, B, V> {
    data: HashMap<A, KsL1Set<B, V>>,
    empty: KsL1Set<B, V>,
}
impl<A: Eq + Hash, B: Eq + Hash, V: Eq + Hash> KsL2Set<A, B, V> {
    pub(crate) fn new() -> KsL2Set<A, B, V> {
        Self {
            data: HashMap::new(),
            empty: KsL1Set::new(),
        }
    }
    // Query methods
    pub(crate) fn get_l1(&self, key1: &A) -> Option<&KsL1Set<B, V>> {
        self.data.get(key1)
    }
    pub(crate) fn get_l2(&self, key1: &A, key2: &B) -> impl ExactSizeIterator<Item = &V> {
        match self.get_l1(key1) {
            Some(ks1l) => ks1l.get(key2),
            None => self.empty.get(key2),
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&A, &KsL1Set<B, V>)> {
        self.data.iter()
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key1: A, key2: B, entry: V) {
        let ks1l = self.data.entry(key1).or_insert_with(|| KsL1Set::new());
        ks1l.add_entry(key2, entry);
    }
    pub(crate) fn remove_entry(&mut self, key1: &A, key2: &B, entry: &V) {
        let need_cleanup = match self.data.get_mut(key1) {
            None => return,
            Some(v) => {
                v.remove_entry(key2, entry);
                v.is_empty()
            }
        };
        if need_cleanup {
            self.data.remove(key1);
        }
    }
    pub(crate) fn remove_l1(&mut self, key: &A) -> Option<KsL1Set<B, V>> {
        self.data.remove(key)
    }
}
