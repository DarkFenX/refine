use crate::{
    defs::{EAttrId, SolItemId},
    err::basic::ItemLoadedError,
    sol::svc::svce_calc::SolAttrVal,
    util::StMap,
};

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
    pub(in crate::sol::svc::svce_calc) fn get_item_attrs(
        &self,
        item_id: &SolItemId,
    ) -> Result<&StMap<EAttrId, SolAttrVal>, ItemLoadedError> {
        match self.data.get(item_id) {
            Some(data) => Ok(data.get_attrs()),
            // All items known to calculator should be added to the map, so consider absence an
            // error
            None => Err(ItemLoadedError::new(*item_id)),
        }
    }
    pub(in crate::sol::svc::svce_calc) fn get_item_attrs_mut(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<&mut StMap<EAttrId, SolAttrVal>, ItemLoadedError> {
        match self.data.get_mut(item_id) {
            Some(data) => Ok(data.get_attrs_mut()),
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
