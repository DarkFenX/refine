use crate::{
    defs::SolItemId,
    err::basic::ItemLoadedError,
    sol::{
        svc::calc::{SolItemAttrPostprocs, misc::SolItemAttrValData},
        uad::item::SolItem,
    },
    util::StMap,
};

use super::{
    pp_fighter_count::{FTR_COUNT_ATTR, fighter_count_postproc_fast, fighter_count_postproc_info},
    pp_skill_level::{SKILL_LVL_ATTR, skill_level_postproc_fast, skill_level_postproc_info},
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct SolAttrValData {
    pub(super) data: StMap<SolItemId, SolItemAttrValData>,
}
impl SolAttrValData {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc::calc) fn get_item_attr_data(
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
    pub(in crate::sol::svc::calc) fn get_item_attr_data_mut(
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
    pub(in crate::sol::svc::calc) fn item_loaded(&mut self, item: &SolItem) {
        let mut item_data = SolItemAttrValData::new();
        match item {
            SolItem::Fighter(_) => {
                item_data.postprocs.insert(
                    FTR_COUNT_ATTR,
                    SolItemAttrPostprocs::new(fighter_count_postproc_fast, fighter_count_postproc_info),
                );
            }
            SolItem::Skill(_) => {
                item_data.postprocs.insert(
                    SKILL_LVL_ATTR,
                    SolItemAttrPostprocs::new(skill_level_postproc_fast, skill_level_postproc_info),
                );
            }
            _ => (),
        }
        self.data.insert(item.get_id(), item_data);
    }
    pub(in crate::sol::svc::calc) fn item_unloaded(&mut self, item_id: &SolItemId) {
        self.data.remove(item_id);
    }
}
