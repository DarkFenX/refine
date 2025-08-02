use crate::{
    ud::{UItemKey, UProjRange},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct Projs {
    pub(super) data: RMap<UItemKey, Option<UProjRange>>,
}
impl Projs {
    pub(in crate::ud::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(crate) fn add(&mut self, item_key: UItemKey, range: Option<UProjRange>) {
        self.data.insert(item_key, range);
    }
    pub(crate) fn remove(&mut self, item_key: &UItemKey) -> Option<Option<UProjRange>> {
        self.data.remove(item_key)
    }
    pub(crate) fn contains(&self, item_key: &UItemKey) -> bool {
        self.data.contains_key(item_key)
    }
    pub(crate) fn get(&self, item_key: &UItemKey) -> Option<Option<UProjRange>> {
        self.data.get(item_key).copied()
    }
    pub(crate) fn get_range_mut(&mut self, item_key: &UItemKey) -> Option<&mut UProjRange> {
        match self.data.get_mut(item_key) {
            Some(Some(u_proj_range)) => Some(u_proj_range),
            _ => None,
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (UItemKey, Option<UProjRange>)> {
        self.data.iter().map(|(k, v)| (*k, *v))
    }
    pub(crate) fn iter_projectees(&self) -> impl ExactSizeIterator<Item = UItemKey> {
        self.data.keys().copied()
    }
    pub(crate) fn iter_ranges_mut(&mut self) -> impl Iterator<Item = &mut UProjRange> {
        self.data.values_mut().filter_map(|v| v.as_mut())
    }
    pub(crate) fn iter_projectees_and_ranges(&self) -> impl Iterator<Item = (UItemKey, UProjRange)> {
        self.data
            .iter()
            .filter_map(|(projectee_key, prange)| prange.map(|prange| (*projectee_key, prange)))
    }
    pub(crate) fn iter_projectees_and_ranges_mut(&mut self) -> impl Iterator<Item = (UItemKey, &mut UProjRange)> {
        self.data
            .iter_mut()
            .filter_map(|(projectee_key, prange)| prange.as_mut().map(|prange| (*projectee_key, prange)))
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(crate) fn clear(&mut self) {
        self.data.clear();
    }
}
