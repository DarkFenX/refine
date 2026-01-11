use std::{
    collections::hash_map::Entry,
    hash::{BuildHasher, Hash},
};

use super::{map::Map, map_set::MapSet};

pub(crate) type RMapRMapRSet<K1, K2, V> =
    MapMapSet<K1, K2, V, rustc_hash::FxBuildHasher, rustc_hash::FxBuildHasher, rustc_hash::FxBuildHasher>;

#[derive(Clone)]
pub(crate) struct MapMapSet<K1, K2, V, H1, H2, H3> {
    data: Map<K1, MapSet<K2, V, H2, H3>, H1>,
    empty: MapSet<K2, V, H2, H3>,
}
impl<K1, K2, V, H1, H2, H3> MapMapSet<K1, K2, V, H1, H2, H3>
where
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
    H3: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self {
            data: Map::new(),
            empty: MapSet::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<K1, K2, V, H1, H2, H3> MapMapSet<K1, K2, V, H1, H2, H3>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    V: Eq + Hash,
    H1: BuildHasher,
    H2: BuildHasher,
    H3: BuildHasher,
{
    // Query methods
    pub(crate) fn get_l1_inner(&self, key1: &K1) -> Option<&MapSet<K2, V, H2, H3>> {
        self.data.get(key1)
    }
    pub(crate) fn get_l2(&self, key1: &K1, key2: &K2) -> impl ExactSizeIterator<Item = &V> {
        match self.get_l1_inner(key1) {
            Some(ks1l) => ks1l.get(key2),
            None => self.empty.get(key2),
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K1, &MapSet<K2, V, H2, H3>)> {
        self.data.iter()
    }
    pub(crate) fn keys_l2(&self, key1: &K1) -> impl ExactSizeIterator<Item = &K2> + use<'_, K1, K2, V, H1, H2, H3> {
        match self.get_l1_inner(key1) {
            Some(ks1l) => ks1l.keys(),
            None => self.empty.keys(),
        }
    }
    // Modification methods
    pub(crate) fn remove_l1(&mut self, key: &K1) -> Option<MapSet<K2, V, H2, H3>> {
        self.data.remove(key)
    }
}
impl<K1, K2, V, H1, H2, H3> MapMapSet<K1, K2, V, H1, H2, H3>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    V: Eq + Hash,
    H1: BuildHasher,
    H2: BuildHasher + Default,
    H3: BuildHasher + Default,
{
    pub(crate) fn add_entry(&mut self, key1: K1, key2: K2, value: V) {
        let ks1l = self.data.entry(key1).or_default();
        ks1l.add_entry(key2, value);
    }
    pub(crate) fn remove_entry(&mut self, key1: K1, key2: K2, value: &V) {
        if let Entry::Occupied(mut entry) = self.data.entry(key1) {
            let mapset = entry.get_mut();
            mapset.remove_entry(key2, value);
            if mapset.is_empty() {
                entry.remove();
            }
        }
    }
}
impl<K1, K2, V, H1, H2, H3> Default for MapMapSet<K1, K2, V, H1, H2, H3>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    V: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
    H3: BuildHasher + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
