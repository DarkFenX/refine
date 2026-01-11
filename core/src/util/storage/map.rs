use std::{
    borrow::Borrow,
    collections::{HashMap, hash_map::Entry},
    hash::{BuildHasher, Hash},
};

use super::set::Set;

pub(crate) type RMap<K, V> = Map<K, V, rustc_hash::FxBuildHasher>;

#[derive(Clone)]
pub(crate) struct Map<K, V, H> {
    data: HashMap<K, V, H>,
}
impl<K, V, H> Map<K, V, H>
where
    H: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<K, V, H> Map<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    // View methods
    pub(crate) fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.data.get(key)
    }
    pub(crate) fn get_opt(&self, key: Option<K>) -> Option<&V> {
        key.and_then(|key| self.data.get(&key))
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K, &V)> {
        self.data.iter()
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = &K> {
        self.data.keys()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &V> {
        self.data.values()
    }
    pub(crate) fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
    // Modification methods
    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (&K, &mut V)> {
        self.data.iter_mut()
    }
    pub(crate) fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        self.data.entry(key)
    }
    pub(crate) fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }
    pub(crate) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut V> {
        self.data.values_mut()
    }
    pub(crate) fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.data.insert(key, val)
    }
    pub(crate) fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        self.data.extend(iter)
    }
    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
    pub(crate) fn retain(&mut self, func: impl FnMut(&K, &mut V) -> bool) {
        self.data.retain(func)
    }
    pub(crate) fn clear(&mut self) {
        self.data.clear()
    }
    pub(crate) fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional)
    }
    // Consumption methods
    pub(crate) fn into_values(self) -> impl ExactSizeIterator<Item = V> {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
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
impl<K, V, H> IntoIterator for Map<K, V, H> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Const variant of the map
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type CMap<K, V> = Map<K, V, rustc_hash::FxSeededState>;

impl<K, V> Map<K, V, rustc_hash::FxSeededState> {
    pub(crate) const fn const_new() -> Self {
        Self {
            data: HashMap::with_hasher(rustc_hash::FxSeededState::with_seed(0)),
        }
    }
}
impl<K, V> Map<K, V, rustc_hash::FxSeededState>
where
    K: Eq + Hash,
{
    pub(crate) fn const_from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut data = HashMap::with_hasher(rustc_hash::FxSeededState::with_seed(0));
        data.extend(iter);
        Self { data }
    }
}
