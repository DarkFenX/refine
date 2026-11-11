use std::hash::{BuildHasher, Hash};

use indexmap::IndexMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular rustc hasher
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type ROrdMap<K, V> = OrdMap<K, V, rustc_hash::FxBuildHasher>;
impl<K, V> Default for ROrdMap<K, V> {
    fn default() -> Self {
        Self {
            data: IndexMap::default(),
        }
    }
}
impl<K, V> ROrdMap<K, V> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Map iterating in insertion order; inserting a key which is already there replaces its value in
/// place.
#[derive(Clone)]
pub(crate) struct OrdMap<K, V, H> {
    data: IndexMap<K, V, H>,
}
impl<K, V, H> OrdMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    pub(crate) fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &V> {
        self.data.values()
    }
    pub(crate) fn into_values(self) -> impl ExactSizeIterator<Item = V> {
        self.data.into_values()
    }
    // Modification methods
    pub(crate) fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.data.insert(key, val)
    }
    pub(crate) fn clear(&mut self) {
        self.data.clear()
    }
}
