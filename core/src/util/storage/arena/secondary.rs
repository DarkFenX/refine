use super::shared::ArenaId;

// This secondary container assumes that code using it ensures that entities behind passed IDs exist
#[derive(Clone)]
pub(crate) struct ArenaSecondary<I, V> {
    data: Vec<Option<V>>,
    key: std::marker::PhantomData<I>,
}
impl<I, V> ArenaSecondary<I, V>
where
    I: ArenaId,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            data: Vec::new(),
            key: std::marker::PhantomData,
        }
    }
    pub(crate) fn get(&self, id: I) -> &V {
        self.data.get(id.index()).unwrap().as_ref().unwrap()
    }
    pub(crate) fn get_mut(&mut self, id: I) -> &mut V {
        self.data.get_mut(id.index()).unwrap().as_mut().unwrap()
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = (I, &V)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(index, value)| value.as_ref().map(|v| (I::new(index), v)))
    }
    pub(crate) fn insert(&mut self, id: I, value: V) {
        let index = id.index();
        if index >= self.data.len() {
            self.data.resize(index + 1, None);
        }
        self.data[index] = Some(value);
    }
    pub(crate) fn remove(&mut self, id: I) {
        let index = id.index();
        if index < self.data.len() {
            self.data[index] = None;
        }
    }
}
