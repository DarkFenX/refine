use cranelift_entity::PrimaryMap;

use crate::util::storage::DenseId;

/// Primary dense storage which never removes entries, but offers cheaper access in return.
pub(crate) struct PDense<I, V>
where
    I: DenseId,
{
    data: PrimaryMap<I, V>,
}
impl<I, V> PDense<I, V>
where
    I: DenseId,
{
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: PrimaryMap::with_capacity(capacity),
        }
    }
    pub(crate) fn get(&self, id: I) -> Option<&V> {
        self.data.get(id)
    }
    pub(crate) fn get_mut(&mut self, id: I) -> Option<&mut V> {
        self.data.get_mut(id)
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (I, &V)> {
        self.data.iter()
    }
    pub(crate) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut V> {
        self.data.values_mut()
    }
    /// ID the next pushed value will get.
    pub(crate) fn next_id(&self) -> I {
        self.data.next_key()
    }
    pub(crate) fn push(&mut self, value: V) -> I {
        self.data.push(value)
    }
}
