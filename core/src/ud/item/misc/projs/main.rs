use crate::{
    ud::{UItemKey, UProjData},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct Projs {
    pub(super) data: RMap<UItemKey, Option<UProjData>>,
}
impl Projs {
    pub(in crate::ud::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(crate) fn add(&mut self, item_key: UItemKey, proj_data: Option<UProjData>) {
        self.data.insert(item_key, proj_data);
    }
    pub(crate) fn remove(&mut self, item_key: &UItemKey) -> Option<Option<UProjData>> {
        self.data.remove(item_key)
    }
    pub(crate) fn contains(&self, item_key: &UItemKey) -> bool {
        self.data.contains_key(item_key)
    }
    pub(crate) fn get(&self, item_key: &UItemKey) -> Option<Option<UProjData>> {
        self.data.get(item_key).copied()
    }
    pub(crate) fn get_proj_data_mut(&mut self, item_key: &UItemKey) -> Option<&mut UProjData> {
        match self.data.get_mut(item_key) {
            Some(Some(u_proj_data)) => Some(u_proj_data),
            _ => None,
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (UItemKey, Option<UProjData>)> {
        self.data.iter().map(|(k, v)| (*k, *v))
    }
    pub(crate) fn iter_projectees(&self) -> impl ExactSizeIterator<Item = UItemKey> {
        self.data.keys().copied()
    }
    pub(crate) fn iter_datas_mut(&mut self) -> impl Iterator<Item = &mut UProjData> {
        self.data.values_mut().filter_map(|v| v.as_mut())
    }
    pub(crate) fn iter_projectees_and_datas(&self) -> impl Iterator<Item = (UItemKey, UProjData)> {
        self.data
            .iter()
            .filter_map(|(projectee_key, proj_data)| proj_data.map(|proj_data| (*projectee_key, proj_data)))
    }
    pub(crate) fn iter_projectees_and_datas_mut(&mut self) -> impl Iterator<Item = (UItemKey, &mut UProjData)> {
        self.data
            .iter_mut()
            .filter_map(|(projectee_key, proj_data)| proj_data.as_mut().map(|proj_data| (*projectee_key, proj_data)))
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(crate) fn clear(&mut self) {
        self.data.clear();
    }
}
