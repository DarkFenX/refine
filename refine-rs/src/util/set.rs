use std::{
    collections::HashSet,
    hash::{BuildHasher, Hash},
};

pub(crate) type RSet<V> = Set<V, rustc_hash::FxBuildHasher>;

#[derive(Clone)]
pub(crate) struct Set<V, H> {
    data: HashSet<V, H>,
}
impl<V, H> Set<V, H>
where
    H: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self {
            data: HashSet::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////////////////////////
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
