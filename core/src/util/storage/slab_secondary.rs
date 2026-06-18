// This secondary container assumes that code using it ensures that entities behind passed keys
// exist
#[derive(Clone)]
pub(crate) struct SlabSecondary<K, V> {
    data: Vec<Option<V>>,
    key: std::marker::PhantomData<K>,
}
impl<K, V> SlabSecondary<K, V>
where
    K: Into<usize> + From<usize>,
    V: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            data: Vec::new(),
            key: std::marker::PhantomData,
        }
    }
    pub(crate) fn get(&self, key: K) -> &V {
        self.data.get(key.into()).unwrap().as_ref().unwrap()
    }
    pub(crate) fn get_mut(&mut self, key: K) -> &mut V {
        self.data.get_mut(key.into()).unwrap().as_mut().unwrap()
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.as_ref().map(|v| (i.into(), v)))
    }
    pub(crate) fn insert(&mut self, key: K, value: V) {
        let index = key.into();
        if index >= self.data.len() {
            self.data.resize(index + 1, None);
        }
        self.data[index] = Some(value);
    }
    pub(crate) fn remove(&mut self, key: K) {
        let index = key.into();
        if index < self.data.len() {
            self.data[index] = None;
        }
    }
}
