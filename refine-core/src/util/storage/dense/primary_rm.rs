use slab::Slab;

use crate::util::storage::DenseId;

/// Primary dense storage which allows to remove entries.
#[derive(Clone)]
pub(crate) struct PDenseRm<I, V> {
    data: Slab<V>,
    phantom: std::marker::PhantomData<I>,
}
impl<I, V> PDenseRm<I, V>
where
    I: DenseId,
{
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Slab::with_capacity(capacity),
            phantom: std::marker::PhantomData,
        }
    }
    pub(crate) fn get(&self, id: I) -> Option<&V> {
        self.data.get(id.index())
    }
    pub(crate) fn get_mut(&mut self, id: I) -> Option<&mut V> {
        self.data.get_mut(id.index())
    }
    #[cfg(feature = "dev")]
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (I, &V)> {
        self.data.iter().map(|(index, value)| (I::new(index), value))
    }
    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (I, &mut V)> {
        self.data.iter_mut().map(|(index, value)| (I::new(index), value))
    }
    pub(crate) fn insert(&mut self, value: V) -> I {
        I::new(self.data.insert(value))
    }
    pub(crate) fn remove(&mut self, id: I) -> V {
        self.data.remove(id.index())
    }
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}
