use cranelift_entity::SecondaryMap;

use crate::util::storage::DenseId;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular version
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SDense<I, V>
where
    I: DenseId,
    V: Clone,
{
    data: SecondaryMap<I, Option<V>>,
}
impl<I, V> SDense<I, V>
where
    I: DenseId,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            data: SecondaryMap::new(),
        }
    }
    #[inline(always)]
    pub(crate) fn get(&self, id: I) -> Option<&V> {
        self.data.get(id).and_then(|v| v.as_ref())
    }
    pub(crate) fn get_mut(&mut self, id: I) -> Option<&mut V> {
        self.data.get_mut(id).and_then(|v| v.as_mut())
    }
    #[cfg(feature = "dev")]
    #[inline(always)]
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
    pub(crate) fn take(&mut self, id: I) -> Option<V> {
        self.data.get_mut(id).and_then(Option::take)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Unchecked version - assumes that there is a value behind every requested ID. This assumption
// should be upheld by code using it
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SDenseUnchecked<I, V>
where
    I: DenseId,
    V: Clone,
{
    inner: SDense<I, V>,
}
impl<I, V> SDenseUnchecked<I, V>
where
    I: DenseId,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self { inner: SDense::new() }
    }
    #[inline(always)]
    pub(crate) fn get(&self, id: I) -> &V {
        self.inner.get(id).unwrap()
    }
    pub(crate) fn get_mut(&mut self, id: I) -> &mut V {
        self.inner.get_mut(id).unwrap()
    }
    #[cfg(feature = "dev")]
    #[inline(always)]
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
