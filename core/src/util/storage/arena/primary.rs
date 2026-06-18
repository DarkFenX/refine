use slab::Slab;

use super::shared::ArenaId;

pub(crate) struct ArenaPrm<I, V>
where
    I: ArenaId,
{
    data: Slab<V>,
    phantom: std::marker::PhantomData<I>,
}
impl<I, V> ArenaPrm<I, V>
where
    I: ArenaId,
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
    pub(crate) fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.data.iter_mut().map(|v| v.1)
    }
    pub(crate) fn vacant_entry(&mut self) -> VacantEntry<'_, I, V> {
        VacantEntry::new(self.data.vacant_entry())
    }
}

pub(crate) struct VacantEntry<'a, I, V>
where
    I: ArenaId,
{
    inner: slab::VacantEntry<'a, V>,
    phantom: std::marker::PhantomData<I>,
}
impl<'a, I, V> VacantEntry<'a, I, V>
where
    I: ArenaId,
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
