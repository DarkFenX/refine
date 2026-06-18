use cranelift_entity::SecondaryMap;

use super::shared::ArenaId;

// This secondary container assumes that code using it ensures that entities behind passed IDs exist
#[derive(Clone)]
pub(crate) struct ArenaSec<I, V>
where
    I: ArenaId,
    V: Clone,
{
    data: SecondaryMap<I, Option<V>>,
}
impl<I, V> ArenaSec<I, V>
where
    I: ArenaId,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            data: SecondaryMap::new(),
        }
    }
    pub(crate) fn get(&self, id: I) -> &V {
        self.data.get(id).unwrap().as_ref().unwrap()
    }
    pub(crate) fn get_mut(&mut self, id: I) -> &mut V {
        self.data.get_mut(id).unwrap().as_mut().unwrap()
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
