use std::hash::{BuildHasher, Hash};

use crate::util::storage::{RSet, SSlab, Set, SlabId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type SSLabRSet<K, V> = SSLabSet<K, V, rustc_hash::FxBuildHasher>;
impl<I, V> Default for SSLabRSet<I, V>
where
    I: SlabId,
    V: Clone,
{
    fn default() -> Self {
        Self {
            data: SSlab::new(),
            empty: RSet::new(),
        }
    }
}
impl<I, V> SSLabRSet<I, V>
where
    I: SlabId,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
impl<I, V> SSLabRSet<I, V>
where
    I: SlabId,
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
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SSLabSet<I, V, H>
where
    I: SlabId,
    V: Clone,
    H: Clone,
{
    data: SSlab<I, Set<V, H>>,
    empty: Set<V, H>,
}
impl<I, V, H> SSLabSet<I, V, H>
where
    I: SlabId,
    V: Eq + Hash + Clone,
    H: BuildHasher + Clone,
{
    pub(crate) fn get(&self, id: I) -> impl ExactSizeIterator<Item = &V> + use<'_, I, V, H> {
        match self.data.get(id) {
            Some(v) => v.iter(),
            None => self.empty.iter(),
        }
    }
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
