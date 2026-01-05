use crate::{
    ud::{UItemId, UProjData},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct UProjs {
    pub(super) data: RMap<UItemId, Option<UProjData>>,
}
impl UProjs {
    pub(in crate::ud::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(crate) fn add(&mut self, item_uid: UItemId, proj_data: Option<UProjData>) {
        self.data.insert(item_uid, proj_data);
    }
    pub(crate) fn remove(&mut self, item_uid: &UItemId) -> Option<Option<UProjData>> {
        self.data.remove(item_uid)
    }
    pub(crate) fn contains(&self, item_uid: &UItemId) -> bool {
        self.data.contains_key(item_uid)
    }
    pub(crate) fn get(&self, item_uid: &UItemId) -> Option<Option<UProjData>> {
        self.data.get(item_uid).copied()
    }
    pub(crate) fn get_proj_data_mut(&mut self, item_uid: &UItemId) -> Option<&mut UProjData> {
        match self.data.get_mut(item_uid) {
            Some(Some(u_proj_data)) => Some(u_proj_data),
            _ => None,
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (UItemId, Option<UProjData>)> {
        self.data.iter().map(|(k, v)| (*k, *v))
    }
    pub(crate) fn iter_projectees(&self) -> impl ExactSizeIterator<Item = UItemId> {
        self.data.keys().copied()
    }
    pub(crate) fn iter_datas_mut(&mut self) -> impl Iterator<Item = &mut UProjData> {
        self.data.values_mut().filter_map(|v| v.as_mut())
    }
    pub(crate) fn iter_projectees_and_datas(&self) -> impl Iterator<Item = (UItemId, UProjData)> {
        self.data
            .iter()
            .filter_map(|(projectee_uid, proj_data)| proj_data.map(|proj_data| (*projectee_uid, proj_data)))
    }
    pub(crate) fn iter_projectees_and_datas_mut(&mut self) -> impl Iterator<Item = (UItemId, &mut UProjData)> {
        self.data
            .iter_mut()
            .filter_map(|(projectee_uid, proj_data)| proj_data.as_mut().map(|proj_data| (*projectee_uid, proj_data)))
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(crate) fn clear(&mut self) {
        self.data.clear();
    }
}
