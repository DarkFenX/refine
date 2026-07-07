use slab::Slab;

use super::shared::SlabId;

#[derive(Clone)]
pub(crate) struct PSlab<I, V> {
    data: Slab<V>,
    phantom: std::marker::PhantomData<I>,
}
impl<I, V> PSlab<I, V>
where
    I: SlabId,
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
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (I, &V)> {
        self.data.iter().map(|(index, value)| (I::new(index), value))
    }
    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (I, &mut V)> {
        self.data.iter_mut().map(|(index, value)| (I::new(index), value))
    }
    pub(crate) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut V> {
        self.data.iter_mut().map(|v| v.1)
    }
    pub(crate) fn insert(&mut self, value: V) -> I {
        I::new(self.data.insert(value))
    }
    pub(crate) fn vacant_entry(&mut self) -> VacantEntry<'_, I, V> {
        VacantEntry::new(self.data.vacant_entry())
    }
    pub(crate) fn remove(&mut self, id: I) -> V {
        self.data.remove(id.index())
    }
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}

pub(crate) struct VacantEntry<'a, I, V>
where
    I: SlabId,
{
    inner: slab::VacantEntry<'a, V>,
    phantom: std::marker::PhantomData<I>,
}
impl<'a, I, V> VacantEntry<'a, I, V>
where
    I: SlabId,
{
    fn new(inner: slab::VacantEntry<'a, V>) -> Self {
        Self {
            inner,
            phantom: std::marker::PhantomData,
        }
    }
    pub(crate) fn id(&self) -> I {
        I::new(self.inner.key())
    }
    pub(crate) fn insert(self, value: V) -> &'a mut V {
        self.inner.insert(value)
    }
}
