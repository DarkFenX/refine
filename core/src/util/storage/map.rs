use std::{
    borrow::Borrow,
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    iter::FromIterator,
};

#[derive(Clone)]
pub struct StMap<K, V> {
    data: HashMap<K, V>,
}
impl<K: Eq + Hash, V> StMap<K, V> {
    pub(crate) fn new() -> StMap<K, V> {
        Self { data: HashMap::new() }
    }
    pub(crate) fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.data.get(key)
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&K, &V)> {
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
    // Modification methods
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
    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
    pub(crate) fn retain(&mut self, func: impl FnMut(&K, &mut V) -> bool) {
        self.data.retain(func)
    }
    // Consumption methods
    pub(crate) fn into_iter(self) -> impl ExactSizeIterator<Item = (K, V)> {
        self.data.into_iter()
    }
    pub(crate) fn into_values(self) -> impl ExactSizeIterator<Item = V> {
        self.data.into_values()
    }
}
impl<K: Eq + Hash, V> FromIterator<(K, V)> for StMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self {
            data: HashMap::from_iter(iter),
        }
    }
}
impl<K: Eq + Hash + Clone, V: Clone> From<&HashMap<K, V>> for StMap<K, V> {
    fn from(hmap: &HashMap<K, V>) -> Self {
        Self {
            data: HashMap::from_iter(hmap.iter().map(|(k, v)| (k.clone(), v.clone()))),
        }
    }
}
impl<K: Eq + Hash + Clone, V: Clone> Into<HashMap<K, V>> for &StMap<K, V> {
    fn into(self) -> HashMap<K, V> {
        HashMap::from_iter(self.iter().map(|(k, v)| (k.clone(), v.clone())))
    }
}
