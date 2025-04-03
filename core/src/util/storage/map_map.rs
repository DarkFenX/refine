use std::hash::Hash;

use super::HMap;

#[derive(Clone)]
pub(crate) struct HMapHMap<A, B, V> {
    data: HMap<A, HMap<B, V>>,
}
impl<A: Eq + Hash, B: Eq + Hash, V> HMapHMap<A, B, V> {
    pub(crate) fn new() -> Self {
        Self { data: HMap::new() }
    }
    // Query methods
    pub(crate) fn get_l1(&self, key1: &A) -> Option<&HMap<B, V>> {
        self.data.get(key1)
    }
    pub(crate) fn get_value(&self, key1: &A, key2: &B) -> Option<&V> {
        self.get_l1(key1).and_then(|m1l| m1l.get(key2))
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&A, &HMap<B, V>)> {
        self.data.iter()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &HMap<B, V>> {
        self.data.values()
    }
    // Modification methods
    pub(crate) fn add_value(&mut self, key1: A, key2: B, entry: V) {
        let m1l = self.data.entry(key1).or_insert_with(|| HMap::new());
        m1l.insert(key2, entry);
    }
    pub(crate) fn remove_l2(&mut self, key1: &A, key2: &B) {
        let need_cleanup = match self.data.get_mut(key1) {
            None => return,
            Some(m1l) => m1l.remove(key2).is_some() && m1l.is_empty(),
        };
        if need_cleanup {
            self.data.remove(key1);
        }
    }
    pub(crate) fn remove_l1(&mut self, key: &A) -> Option<HMap<B, V>> {
        self.data.remove(key)
    }
}
impl<A: Eq + Hash, B: Eq + Hash, V> Default for HMapHMap<A, B, V> {
    fn default() -> Self {
        Self::new()
    }
}
