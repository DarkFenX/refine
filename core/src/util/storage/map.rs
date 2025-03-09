use std::{
    borrow::Borrow,
    collections::{HashMap, hash_map::Entry},
    hash::Hash,
};

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::util::storage::StSet;

#[derive(Clone)]
pub struct StMap<K, V> {
    data: FxHashMap<K, V>,
}
impl<K: Eq + Hash, V> StMap<K, V> {
    // Constructors
    pub fn new() -> Self {
        Self {
            data: FxHashMap::default(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: FxHashMap::with_capacity_and_hasher(capacity, FxBuildHasher),
        }
    }
    // View methods
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.data.get(key)
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&K, &V)> {
        self.data.iter()
    }
    pub fn keys(&self) -> impl ExactSizeIterator<Item = &K> {
        self.data.keys()
    }
    pub fn values(&self) -> impl ExactSizeIterator<Item = &V> {
        self.data.values()
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    // Modification methods
    pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (&K, &mut V)> {
        self.data.iter_mut()
    }
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        self.data.entry(key)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }
    pub fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut V> {
        self.data.values_mut()
    }
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.data.insert(key, val)
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
    pub fn retain(&mut self, func: impl FnMut(&K, &mut V) -> bool) {
        self.data.retain(func)
    }
    pub fn clear(&mut self) {
        self.data.clear()
    }
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit()
    }
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional)
    }
    // Consumption methods
    pub fn into_values(self) -> impl ExactSizeIterator<Item = V> {
        self.data.into_values()
    }
    // Set methods
    pub(crate) fn is_subset(&self, other: &StSet<K>) -> bool {
        // (Almost) copy of std::collections::HashSet::is_subset()
        match self.len() <= other.len() {
            true => self.keys().all(|v| other.contains(v)),
            false => false,
        }
    }
    pub(crate) fn difference(&self, other: &StSet<K>) -> impl Iterator<Item = &K> {
        self.keys().filter(|v| !other.contains(v))
    }
}
impl<K: Eq + Hash, V> Default for StMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K, V> IntoIterator for StMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl<K: Eq + Hash, V> FromIterator<(K, V)> for StMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self {
            data: FxHashMap::from_iter(iter),
        }
    }
}
impl<K: Eq + Hash + Clone, V: Clone> From<&HashMap<K, V>> for StMap<K, V> {
    fn from(h_map: &HashMap<K, V>) -> Self {
        Self {
            data: FxHashMap::from_iter(h_map.iter().map(|(k, v)| (k.clone(), v.clone()))),
        }
    }
}
impl<K: Eq + Hash + Clone, V: Clone> From<&StMap<K, V>> for HashMap<K, V> {
    fn from(st_map: &StMap<K, V>) -> HashMap<K, V> {
        Self::from_iter(st_map.iter().map(|(k, v)| (k.clone(), v.clone())))
    }
}
