use std::hash::Hash;

use rustc_hash::FxHashMap;

use super::StMapSetL1;

pub(crate) struct StMapSetL2<A, B, V> {
    data: FxHashMap<A, StMapSetL1<B, V>>,
    empty: StMapSetL1<B, V>,
}
impl<A: Eq + Hash, B: Eq + Hash, V: Eq + Hash> StMapSetL2<A, B, V> {
    pub(crate) fn new() -> StMapSetL2<A, B, V> {
        Self {
            data: FxHashMap::default(),
            empty: StMapSetL1::new(),
        }
    }
    // Query methods
    pub(crate) fn get_l1(&self, key1: &A) -> Option<&StMapSetL1<B, V>> {
        self.data.get(key1)
    }
    pub(crate) fn get_l2(&self, key1: &A, key2: &B) -> impl ExactSizeIterator<Item = &V> {
        match self.get_l1(key1) {
            Some(ks1l) => ks1l.get(key2),
            None => self.empty.get(key2),
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&A, &StMapSetL1<B, V>)> {
        self.data.iter()
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key1: A, key2: B, entry: V) {
        let ks1l = self.data.entry(key1).or_insert_with(|| StMapSetL1::new());
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
    pub(crate) fn remove_l1(&mut self, key: &A) -> Option<StMapSetL1<B, V>> {
        self.data.remove(key)
    }
}
