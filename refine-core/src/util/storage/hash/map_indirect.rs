use std::hash::{BuildHasher, Hash};

use super::map::Map;
use crate::util::DenseId;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular rustc hasher
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type RIndirectMap<I, K, V> = IndirectMap<I, K, V, rustc_hash::FxBuildHasher>;

pub(crate) type RIMapDeduperLinear<I, K, V> = IMapDeduperLinear<I, K, V, rustc_hash::FxBuildHasher>;
impl<I, K, V> RIMapDeduperLinear<I, K, V> {
    pub(crate) fn new() -> Self {
        Self {
            values: Vec::new(),
            ids: Map::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic - map itself
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Map with values in a vector and keys pointing to them by ID; several keys can share a value.
pub(crate) struct IndirectMap<I, K, V, H> {
    values: Vec<V>,
    ids: Map<K, I, H>,
}
impl<I, K, V, H> IndirectMap<I, K, V, H>
where
    I: DenseId,
    K: Eq + Hash,
    H: BuildHasher,
{
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
    pub(crate) fn is_same_entry(&self, key1: &K, key2: &K) -> bool {
        match (self.ids.get(key1), self.ids.get(key2)) {
            (Some(id1), Some(id2)) => id1 == id2,
            _ => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic - builder with linear deduplication
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct IMapDeduperLinear<I, K, V, H> {
    values: Vec<V>,
    ids: Map<K, I, H>,
}
impl<I, K, V, H> IMapDeduperLinear<I, K, V, H>
where
    I: DenseId,
    K: Eq + Hash,
    V: PartialEq,
    H: BuildHasher,
{
    pub(crate) fn insert(&mut self, key: K, val: V) {
        let index = match self.values.iter().position(|v| *v == val) {
            Some(index) => index,
            None => {
                self.values.push(val);
                self.values.len() - 1
            }
        };
        self.ids.insert(key, I::new(index));
    }
    pub(crate) fn build(mut self) -> IndirectMap<I, K, V, H> {
        self.values.shrink_to_fit();
        self.ids.shrink_to_fit();
        IndirectMap {
            values: self.values,
            ids: self.ids,
        }
    }
}
