use crate::{defs::SolItemId, err::basic::ItemLoadedError, util::StMap};

use super::item::SolItemAttrValData;

#[derive(Clone)]
pub(in crate::sol::svc::svce_calc) struct SolAttrValData {
    pub(super) data: StMap<SolItemId, SolItemAttrValData>,
}
impl SolAttrValData {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_item_attr_data(
        &self,
        item_id: &SolItemId,
    ) -> Result<&SolItemAttrValData, ItemLoadedError> {
        match self.data.get(item_id) {
            Some(data) => Ok(data),
            // All items known to calculator should be added to the map, so consider absence an
            // error
            None => Err(ItemLoadedError::new(*item_id)),
        }
    }
    pub(in crate::sol::svc::svce_calc) fn get_item_attr_data_mut(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<&mut SolItemAttrValData, ItemLoadedError> {
        match self.data.get_mut(item_id) {
            Some(data) => Ok(data),
            // All items known to calculator should be added to the map, so consider absence an
            // error
            None => Err(ItemLoadedError::new(*item_id)),
        }
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn item_loaded(&mut self, item_id: SolItemId) {
        self.data.insert(item_id, SolItemAttrValData::new());
    }
    pub(in crate::sol::svc::svce_calc) fn item_unloaded(&mut self, item_id: &SolItemId) {
        self.data.remove(item_id);
    }
}
