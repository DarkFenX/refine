use std::{
    collections::hash_map::Entry,
    hash::{BuildHasher, Hash},
};

use super::{
    map::{Map, RMap},
    map_map::MapMap,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type RMapRMapRMap<K1, K2, K3, V> =
    MapMapMap<K1, K2, K3, V, rustc_hash::FxBuildHasher, rustc_hash::FxBuildHasher, rustc_hash::FxBuildHasher>;
impl<K1, K2, K3, V> Default for RMapRMapRMap<K1, K2, K3, V> {
    fn default() -> Self {
        Self { data: RMap::new() }
    }
}
impl<K1, K2, K3, V> RMapRMapRMap<K1, K2, K3, V> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
impl<K1, K2, K3, V> RMapRMapRMap<K1, K2, K3, V>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    K3: Eq + Hash,
{
    pub(crate) fn add_entry(&mut self, key1: K1, key2: K2, key3: K3, value: V) {
        let m2l = self.data.entry(key1).or_default();
        m2l.add_entry(key2, key3, value);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct MapMapMap<K1, K2, K3, V, H1, H2, H3> {
    data: Map<K1, MapMap<K2, K3, V, H2, H3>, H1>,
}
impl<K1, K2, K3, V, H1, H2, H3> MapMapMap<K1, K2, K3, V, H1, H2, H3>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    K3: Eq + Hash,
    H1: BuildHasher,
    H2: BuildHasher,
    H3: BuildHasher,
{
    // Query methods
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K1, &MapMap<K2, K3, V, H2, H3>)> {
        self.data.iter()
    }
    pub(crate) fn get_l1(&self, key1: &K1) -> Option<&MapMap<K2, K3, V, H2, H3>> {
        self.data.get(key1)
    }
    // Modification methods
    pub(crate) fn remove_l3(&mut self, key1: K1, key2: K2, key3: &K3) {
        if let Entry::Occupied(mut entry_l1) = self.data.entry(key1) {
            let map_l2 = entry_l1.get_mut();
            if map_l2.remove_l2(key2, key3) && map_l2.is_empty() {
                entry_l1.remove();
            }
        }
    }
}
