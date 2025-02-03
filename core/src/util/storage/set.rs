use std::hash::Hash;

use rustc_hash::FxHashSet;

#[derive(Clone)]
pub(crate) struct StSet<V> {
    data: FxHashSet<V>,
}
impl<V: Eq + Hash> StSet<V> {
    pub(crate) fn new() -> Self {
        Self {
            data: FxHashSet::default(),
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
    pub(crate) fn difference<'a>(&'a self, other: &'a StSet<V>) -> impl Iterator<Item = &'a V> {
        self.data.difference(&other.data)
    }
    // Modification methods
    pub(crate) fn insert(&mut self, val: V) -> bool {
        self.data.insert(val)
    }
    pub(crate) fn extend<I: IntoIterator<Item = V>>(&mut self, iter: I) {
        self.data.extend(iter)
    }
    pub(crate) fn remove(&mut self, val: &V) -> bool {
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
