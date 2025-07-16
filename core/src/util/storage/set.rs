use std::{
    collections::HashSet,
    hash::{BuildHasher, Hash},
};

use rustc_hash::FxBuildHasher;

pub type RSet<V> = Set<V, FxBuildHasher>;

#[derive(Clone)]
pub struct Set<V, H> {
    data: HashSet<V, H>,
}
impl<V, H> Set<V, H>
where
    V: Eq + Hash,
    H: BuildHasher + Default,
{
    pub fn new() -> Self {
        Self {
            data: HashSet::default(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashSet::with_capacity_and_hasher(capacity, Default::default()),
        }
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &V> {
        self.data.iter()
    }
    pub fn contains(&self, val: &V) -> bool {
        self.data.contains(val)
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn difference<'a, H2>(&'a self, other: &'a Set<V, H2>) -> impl Iterator<Item = &'a V>
    where
        H2: BuildHasher + Default,
    {
        self.iter().filter(|v| !other.contains(v))
    }
    pub fn is_subset<H2>(&self, other: &Set<V, H2>) -> bool
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
    pub fn insert(&mut self, val: V) -> bool {
        self.data.insert(val)
    }
    pub fn extend<I: IntoIterator<Item = V>>(&mut self, iter: I) {
        self.data.extend(iter)
    }
    pub fn remove(&mut self, val: &V) -> bool {
        self.data.remove(val)
    }
    pub fn drain(&mut self) -> impl ExactSizeIterator<Item = V> {
        self.data.drain()
    }
    pub fn clear(&mut self) {
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
