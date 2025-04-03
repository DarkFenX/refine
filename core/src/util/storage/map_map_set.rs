use std::hash::Hash;

use super::{HMap, HMapHSet};

#[derive(Clone)]
pub(crate) struct HMapHMapHSet<A, B, V> {
    data: HMap<A, HMapHSet<B, V>>,
    empty: HMapHSet<B, V>,
}
impl<A: Eq + Hash, B: Eq + Hash, V: Eq + Hash> HMapHMapHSet<A, B, V> {
    pub(crate) fn new() -> Self {
        Self {
            data: HMap::new(),
            empty: HMapHSet::new(),
        }
    }
    // Query methods
    pub(crate) fn get_l1_inner(&self, key1: &A) -> Option<&HMapHSet<B, V>> {
        self.data.get(key1)
    }
    pub(crate) fn get_l2(&self, key1: &A, key2: &B) -> impl ExactSizeIterator<Item = &V> {
        match self.get_l1_inner(key1) {
            Some(ks1l) => ks1l.get(key2),
            None => self.empty.get(key2),
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&A, &HMapHSet<B, V>)> {
        self.data.iter()
    }
    pub(crate) fn keys_l2(&self, key1: &A) -> impl ExactSizeIterator<Item = &B> + use<'_, A, B, V> {
        match self.get_l1_inner(key1) {
            Some(ks1l) => ks1l.keys(),
            None => self.empty.keys(),
        }
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key1: A, key2: B, entry: V) {
        let ks1l = self.data.entry(key1).or_insert_with(|| HMapHSet::new());
        ks1l.add_entry(key2, entry);
    }
    pub(crate) fn remove_entry(&mut self, key1: &A, key2: &B, entry: &V) {
        let need_cleanup = match self.data.get_mut(key1) {
            None => return,
            Some(ks1l) => ks1l.remove_entry(key2, entry) && ks1l.is_empty(),
        };
        if need_cleanup {
            self.data.remove(key1);
        }
    }
    pub(crate) fn remove_l1(&mut self, key: &A) -> Option<HMapHSet<B, V>> {
        self.data.remove(key)
    }
}
impl<A: Eq + Hash, B: Eq + Hash, V: Eq + Hash> Default for HMapHMapHSet<A, B, V> {
    fn default() -> Self {
        Self::new()
    }
}
