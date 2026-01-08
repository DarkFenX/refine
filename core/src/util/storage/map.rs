use std::{
    borrow::Borrow,
    collections::{HashMap, hash_map::Entry},
    hash::{BuildHasher, Hash},
};

use rustc_hash::FxBuildHasher;

use super::set::Set;

pub type RMap<K, V> = Map<K, V, FxBuildHasher>;

#[derive(Clone)]
pub struct Map<K, V, H> {
    data: HashMap<K, V, H>,
}
impl<K, V, H> Map<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    // Constructors
    pub fn new() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }
    // View methods
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.data.get(key)
    }
    pub(crate) fn get_opt(&self, key: Option<K>) -> Option<&V> {
        key.and_then(|key| self.data.get(&key))
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
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        self.data.extend(iter)
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
    // Set-alike view methods
    pub(crate) fn is_subset<H2>(&self, other: &Set<K, H2>) -> bool
    where
        H2: BuildHasher + Default,
    {
        // (Almost) copy of std::collections::HashSet::is_subset()
        match self.len() <= other.len() {
            true => self.keys().all(|v| other.contains(v)),
            false => false,
        }
    }
    pub(crate) fn difference<H2>(&self, other: &Set<K, H2>) -> impl Iterator<Item = (&K, &V)>
    where
        H2: BuildHasher + Default,
    {
        self.iter().filter(|(k, _)| !other.contains(k))
    }
}
impl<K, V, H> Default for Map<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<K, V, H> IntoIterator for Map<K, V, H> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl<K, V, H> FromIterator<(K, V)> for Map<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        Self {
            data: HashMap::from_iter(iter),
        }
    }
}
impl<K, V, H> From<HashMap<K, V>> for Map<K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone,
    H: BuildHasher + Default,
{
    fn from(h_map: HashMap<K, V>) -> Self {
        Self {
            data: HashMap::from_iter(h_map.iter().map(|(k, v)| (k.clone(), v.clone()))),
        }
    }
}
impl<K, V, H> From<Map<K, V, H>> for HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
    H: BuildHasher + Default,
{
    fn from(st_map: Map<K, V, H>) -> HashMap<K, V> {
        Self::from_iter(st_map.iter().map(|(k, v)| (k.clone(), v.clone())))
    }
}
