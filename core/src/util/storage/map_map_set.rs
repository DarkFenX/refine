use std::hash::{BuildHasher, Hash};

use rustc_hash::FxBuildHasher;

use super::{map::Map, map_set::MapSet};

pub(crate) type RMapRMapRSet<K1, K2, V> = MapMapSet<K1, K2, V, FxBuildHasher, FxBuildHasher, FxBuildHasher>;

#[derive(Clone)]
pub(crate) struct MapMapSet<K1, K2, V, H1, H2, H3> {
    data: Map<K1, MapSet<K2, V, H2, H3>, H1>,
    empty: MapSet<K2, V, H2, H3>,
}
impl<K1, K2, V, H1, H2, H3> MapMapSet<K1, K2, V, H1, H2, H3>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    V: Eq + Hash,
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
    pub(crate) fn add_entry(&mut self, key1: K1, key2: K2, entry: V) {
        let ks1l = self.data.entry(key1).or_insert_with(|| MapSet::new());
        ks1l.add_entry(key2, entry);
    }
    pub(crate) fn remove_entry(&mut self, key1: &K1, key2: &K2, entry: &V) {
        let need_cleanup = match self.data.get_mut(key1) {
            None => return,
            Some(ks1l) => ks1l.remove_entry(key2, entry) && ks1l.is_empty(),
        };
        if need_cleanup {
            self.data.remove(key1);
        }
    }
    pub(crate) fn remove_l1(&mut self, key: &K1) -> Option<MapSet<K2, V, H2, H3>> {
        self.data.remove(key)
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
