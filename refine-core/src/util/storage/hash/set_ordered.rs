use std::hash::{BuildHasher, Hash};

use indexmap::{IndexSet, set::Slice};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type ROrdSet<V> = OrdSet<V, rustc_hash::FxBuildHasher>;
impl<V> Default for ROrdSet<V> {
    fn default() -> Self {
        Self {
            data: IndexSet::default(),
        }
    }
}
impl<V> ROrdSet<V> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct OrdSet<V, H> {
    data: IndexSet<V, H>,
}
impl<V, H> OrdSet<V, H>
where
    V: Eq + Hash,
    H: BuildHasher,
{
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
    // Modification methods
    pub(crate) fn insert(&mut self, val: V) -> bool {
        self.data.insert(val)
    }
    pub(crate) fn insert_and_get_index(&mut self, val: V) -> (usize, bool) {
        self.data.insert_full(val)
    }
}
impl<T, S> core::ops::Index<core::ops::RangeFrom<usize>> for OrdSet<T, S> {
    type Output = Slice<T>;

    fn index(&self, range: core::ops::RangeFrom<usize>) -> &Self::Output {
        self.data.index(range)
    }
}
impl<V, H> IntoIterator for OrdSet<V, H> {
    type Item = V;
    type IntoIter = indexmap::set::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
