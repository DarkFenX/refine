use std::{
    borrow::Borrow,
    collections::{HashMap, hash_map::Entry},
    hash::{BuildHasher, Hash},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type RMap<K, V> = Map<K, V, rustc_hash::FxBuildHasher>;
impl<K, V> Default for RMap<K, V> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}
impl<K, V> RMap<K, V> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type CMap<K, V> = Map<K, V, rustc_hash::FxSeededState>;
const impl<K, V> Default for CMap<K, V> {
    fn default() -> Self {
        Self {
            data: HashMap::with_hasher(rustc_hash::FxSeededState::with_seed(0)),
        }
    }
}
impl<K, V> CMap<K, V> {
    pub(crate) const fn new() -> Self {
        Self::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct Map<K, V, H> {
    data: HashMap<K, V, H>,
}
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
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K, &V)> {
        self.data.iter()
    }
    pub(crate) fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
    // Modification methods
    pub(crate) fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        self.data.entry(key)
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
}
