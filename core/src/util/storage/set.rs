use std::hash::Hash;

use rustc_hash::{FxBuildHasher, FxHashSet};

#[derive(Clone)]
pub struct StSet<V> {
    data: FxHashSet<V>,
}
impl<V: Eq + Hash> StSet<V> {
    pub fn new() -> Self {
        Self {
            data: FxHashSet::default(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: FxHashSet::with_capacity_and_hasher(capacity, FxBuildHasher),
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
    pub fn difference<'a>(&'a self, other: &'a StSet<V>) -> impl Iterator<Item = &'a V> {
        self.data.difference(&other.data)
    }
    pub fn is_subset(&self, other: &StSet<V>) -> bool {
        self.data.is_subset(&other.data)
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
}
impl<V: Eq + Hash> Default for StSet<V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<V: Eq + Hash> FromIterator<V> for StSet<V> {
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        Self {
            data: FxHashSet::from_iter(iter),
        }
    }
}
impl<V> IntoIterator for StSet<V> {
    type Item = V;
    type IntoIter = std::collections::hash_set::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
