use crate::{
    defs::SolItemId,
    err::basic::ItemLoadedError,
    sol::{svc::svce_calc::misc::SolItemAttrValData, uad::item::SolItem},
    util::StMap,
};

use super::skill::{skill_level_postprocessor, SKILL_LVL_ATTR};

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
    pub(in crate::sol::svc::svce_calc) fn item_loaded(&mut self, item: &SolItem) {
        let mut item_data = SolItemAttrValData::new();
        if let SolItem::Skill(_) = item {
            item_data
                .postprocessors
                .insert(SKILL_LVL_ATTR, skill_level_postprocessor);
        }
        self.data.insert(item.get_id(), item_data);
    }
    pub(in crate::sol::svc::svce_calc) fn item_unloaded(&mut self, item_id: &SolItemId) {
        self.data.remove(item_id);
    }
}
