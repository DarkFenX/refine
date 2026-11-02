use std::hash::{BuildHasher, Hash};

use super::map::{Map, RMap};
use crate::util::DenseId;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type RDedupMap<I, K, V> = DedupMap<I, K, V, rustc_hash::FxBuildHasher>;
impl<I, K, V> Default for RDedupMap<I, K, V> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            ids: RMap::new(),
        }
    }
}
impl<I, K, V> RDedupMap<I, K, V> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Map which stores equal values once, with keys pointing to a shared entry.
pub(crate) struct DedupMap<I, K, V, H> {
    values: Vec<V>,
    ids: Map<K, I, H>,
}
impl<I, K, V, H> DedupMap<I, K, V, H>
where
    I: DenseId,
    K: Eq + Hash,
    H: BuildHasher,
{
    // View methods
    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        let id = *self.ids.get(key)?;
        Some(&self.values[id.index()])
    }
    pub(crate) fn get_id(&self, key: &K) -> Option<I> {
        self.ids.get(key).copied()
    }
    pub(crate) fn get_by_id(&self, id: I) -> &V {
        &self.values[id.index()]
    }
    #[expect(dead_code)]
    pub(crate) fn is_same_entry(&self, key1: &K, key2: &K) -> bool {
        match (self.ids.get(key1), self.ids.get(key2)) {
            (Some(id1), Some(id2)) => id1 == id2,
            _ => false,
        }
    }
    // Modification methods
    pub(crate) fn insert(&mut self, key: K, val: V)
    where
        V: PartialEq,
    {
        let index = match self.values.iter().position(|v| *v == val) {
            Some(index) => index,
            None => {
                self.values.push(val);
                self.values.len() - 1
            }
        };
        self.ids.insert(key, I::new(index));
    }
    pub(crate) fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
        self.ids.shrink_to_fit();
    }
}
