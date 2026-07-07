use cranelift_entity::SecondaryMap;

use super::shared::SlabId;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular version
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SSlab<I, V>
where
    I: SlabId,
    V: Clone,
{
    data: SecondaryMap<I, Option<V>>,
}
impl<I, V> SSlab<I, V>
where
    I: SlabId,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            data: SecondaryMap::new(),
        }
    }
    pub(crate) fn get(&self, id: I) -> Option<&V> {
        self.data.get(id).and_then(|v| v.as_ref())
    }
    pub(crate) fn get_mut(&mut self, id: I) -> Option<&mut V> {
        self.data.get_mut(id).and_then(|v| v.as_mut())
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = (I, &V)> {
        self.data
            .iter()
            .filter_map(|(id, value)| value.as_ref().map(|v| (id, v)))
    }
    pub(crate) fn insert(&mut self, id: I, value: V) {
        self.data.insert(id, Some(value));
    }
    pub(crate) fn remove(&mut self, id: I) {
        self.data.remove(id);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Unchecked version - assumes that there is a value behind every requested ID. This assumption
// should be upheld by code using it
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SSlabUnchecked<I, V>
where
    I: SlabId,
    V: Clone,
{
    inner: SSlab<I, V>,
}
impl<I, V> SSlabUnchecked<I, V>
where
    I: SlabId,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self { inner: SSlab::new() }
    }
    pub(crate) fn get(&self, id: I) -> &V {
        self.inner.get(id).unwrap()
    }
    pub(crate) fn get_mut(&mut self, id: I) -> &mut V {
        self.inner.get_mut(id).unwrap()
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = (I, &V)> {
        self.inner.iter()
    }
    pub(crate) fn insert(&mut self, id: I, value: V) {
        self.inner.insert(id, value);
    }
    pub(crate) fn remove(&mut self, id: I) {
        self.inner.remove(id);
    }
}
