use std::{
    collections::HashSet,
    hash::{BuildHasher, Hash},
};

pub(crate) type RSet<V> = Set<V, rustc_hash::FxBuildHasher>;

#[derive(Clone)]
pub(crate) struct Set<V, H> {
    data: HashSet<V, H>,
}
impl<V, H> Set<V, H>
where
    V: Eq + Hash,
    H: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self {
            data: HashSet::default(),
        }
    }
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashSet::with_capacity_and_hasher(capacity, Default::default()),
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = &V> {
        self.data.iter()
    }
    pub(crate) fn contains(&self, val: &V) -> bool {
        self.data.contains(val)
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
    pub(crate) fn difference<'a, H2>(&'a self, other: &'a Set<V, H2>) -> impl Iterator<Item = &'a V>
    where
        H2: BuildHasher + Default,
    {
        self.iter().filter(|v| !other.contains(v))
    }
    pub(crate) fn is_subset<H2>(&self, other: &Set<V, H2>) -> bool
    where
        H2: BuildHasher + Default,
    {
        if self.len() <= other.len() {
            self.iter().all(|v| other.contains(v))
        } else {
            false
        }
    }
    // Modification methods
    pub(crate) fn insert(&mut self, val: V) -> bool {
        self.data.insert(val)
    }
    pub(crate) fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = V>,
    {
        self.data.extend(iter)
    }
    pub(crate) fn remove(&mut self, val: &V) -> bool {
        self.data.remove(val)
    }
    pub(crate) fn drain(&mut self) -> impl ExactSizeIterator<Item = V> {
        self.data.drain()
    }
    pub(crate) fn extract_if<F>(&mut self, filter: F) -> impl Iterator<Item = V>
    where
        F: FnMut(&V) -> bool,
    {
        self.data.extract_if(filter)
    }
    pub(crate) fn clear(&mut self) {
        self.data.clear()
    }
}
impl<V, H> Default for Set<V, H>
where
    V: Eq + Hash,
    H: BuildHasher + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<V, H> FromIterator<V> for Set<V, H>
where
    V: Eq + Hash,
    H: BuildHasher + Default,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        Self {
            data: HashSet::from_iter(iter),
        }
    }
}
impl<V, H> IntoIterator for Set<V, H> {
    type Item = V;
    type IntoIter = std::collections::hash_set::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
