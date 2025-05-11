use super::{
    pp_fighter_count::{FTR_COUNT_ATTR, fighter_count_postproc_fast, fighter_count_postproc_info},
    pp_sec_status::{SEC_STATUS_ATTR, sec_status_postproc_fast, sec_status_postproc_info},
    pp_skill_level::{SKILL_LVL_ATTR, skill_level_postproc_fast, skill_level_postproc_info},
};
use crate::{
    sol::{
        ItemKey,
        svc::calc::{ItemAttrPostprocs, misc::ItemAttrValData},
        uad::item::UadItem,
    },
    util::RMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct AttrValData {
    pub(super) data: RMap<ItemKey, ItemAttrValData>,
}
impl AttrValData {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc::calc) fn get_item_attr_data(&self, item_key: &ItemKey) -> Option<&ItemAttrValData> {
        self.data.get(item_key)
    }
    pub(in crate::sol::svc::calc) fn get_item_attr_data_mut(
        &mut self,
        item_key: &ItemKey,
    ) -> Option<&mut ItemAttrValData> {
        self.data.get_mut(item_key)
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn item_loaded(&mut self, item_key: ItemKey, item: &UadItem) {
        let mut item_data = ItemAttrValData::new();
        match item {
            UadItem::Fighter(_) => {
                item_data.postprocs.insert(
                    FTR_COUNT_ATTR,
                    ItemAttrPostprocs {
                        fast: fighter_count_postproc_fast,
                        info: fighter_count_postproc_info,
                    },
                );
            }
            UadItem::Ship(_) => {
                item_data.postprocs.insert(
                    SEC_STATUS_ATTR,
                    ItemAttrPostprocs {
                        fast: sec_status_postproc_fast,
                        info: sec_status_postproc_info,
                    },
                );
            }
            UadItem::Skill(_) => {
                item_data.postprocs.insert(
                    SKILL_LVL_ATTR,
                    ItemAttrPostprocs {
                        fast: skill_level_postproc_fast,
                        info: skill_level_postproc_info,
                    },
                );
            }
            _ => (),
        }
        self.data.insert(item_key, item_data);
    }
    pub(in crate::sol::svc::calc) fn item_unloaded(&mut self, item_key: &ItemKey) {
        self.data.remove(item_key);
    }
}
