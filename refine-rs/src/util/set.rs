use std::{
    collections::HashSet,
    hash::{BuildHasher, Hash},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type RSet<V> = Set<V, rustc_hash::FxBuildHasher>;
impl<V> Default for RSet<V> {
    fn default() -> Self {
        Self {
            data: HashSet::default(),
        }
    }
}
impl<V> RSet<V> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct Set<V, H> {
    data: HashSet<V, H>,
}
impl<V, H> Set<V, H>
where
    V: Eq + Hash,
    H: BuildHasher,
{
    pub(crate) fn contains(&self, val: &V) -> bool {
        self.data.contains(val)
    }
    // Modification methods
    pub(crate) fn insert(&mut self, val: V) -> bool {
        self.data.insert(val)
    }
    pub(crate) fn remove(&mut self, val: &V) -> bool {
        self.data.remove(val)
    }
}
