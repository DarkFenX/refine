use std::hash::{BuildHasher, Hash};

use crate::util::storage::{DenseId, RSet, SDense, Set};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular rustc hasher
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type SDenseRSet<K, V> = SDenseSet<K, V, rustc_hash::FxBuildHasher>;
impl<I, V> Default for SDenseRSet<I, V>
where
    I: DenseId,
    V: Clone,
{
    fn default() -> Self {
        Self {
            data: SDense::new(),
            empty: RSet::new(),
        }
    }
}
impl<I, V> SDenseRSet<I, V>
where
    I: DenseId,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
impl<I, V> SDenseRSet<I, V>
where
    I: DenseId,
    V: Eq + Hash + Clone,
{
    pub(crate) fn add_entry(&mut self, id: I, value: V) {
        match self.data.get_mut(id) {
            Some(set) => {
                set.insert(value);
            }
            None => {
                let mut set = RSet::with_capacity(1);
                set.insert(value);
                self.data.insert(id, set);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SDenseSet<I, V, H>
where
    I: DenseId,
    V: Clone,
    H: Clone,
{
    data: SDense<I, Set<V, H>>,
    empty: Set<V, H>,
}
impl<I, V, H> SDenseSet<I, V, H>
where
    I: DenseId,
    V: Eq + Hash + Clone,
    H: BuildHasher + Clone,
{
    #[inline(always)]
    pub(crate) fn get(&self, id: I) -> impl ExactSizeIterator<Item = &V> + use<'_, I, V, H> {
        match self.data.get(id) {
            Some(v) => v.iter(),
            None => self.empty.iter(),
        }
    }
    #[cfg(feature = "dev")]
    #[inline(always)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (I, impl ExactSizeIterator<Item = &V>)> {
        self.data.iter().map(|(k, v)| (k, v.iter()))
    }
    pub(crate) fn remove_entry(&mut self, id: I, value: &V) {
        if let Some(set) = self.data.get_mut(id) {
            set.remove(value);
            if set.is_empty() {
                self.data.remove(id);
            }
        }
    }
}
