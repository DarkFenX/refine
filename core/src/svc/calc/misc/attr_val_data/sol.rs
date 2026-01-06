use super::{
    data::ItemAttrData,
    pp_fighter_count::{fighter_count_postproc_fast, fighter_count_postproc_info},
    pp_sec_status::{sec_status_postproc_fast, sec_status_postproc_info},
    pp_skill_level::{skill_level_postproc_fast, skill_level_postproc_info},
};
use crate::{
    svc::calc::ItemAttrPostprocs,
    ud::{UData, UItem, UItemId},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::svc::calc) struct AttrValData {
    pub(super) data: RMap<UItemId, ItemAttrData>,
}
impl AttrValData {
    pub(in crate::svc::calc) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(in crate::svc::calc) fn get_item_attr_data(&self, item_uid: &UItemId) -> Option<&ItemAttrData> {
        self.data.get(item_uid)
    }
    pub(in crate::svc::calc) fn get_item_attr_data_mut(&mut self, item_uid: &UItemId) -> Option<&mut ItemAttrData> {
        self.data.get_mut(item_uid)
    }
    // Modification methods
    pub(in crate::svc::calc) fn item_loaded(&mut self, u_data: &UData, item_uid: UItemId, item: &UItem) {
        let mut item_data = ItemAttrData::new();
        match item {
            UItem::Fighter(_) if let Some(count_attr_rid) = u_data.src.get_attr_consts().ftr_sq_size => {
                item_data.reg_postproc(
                    count_attr_rid,
                    ItemAttrPostprocs {
                        fast: fighter_count_postproc_fast,
                        info: fighter_count_postproc_info,
                    },
                );
            }
            UItem::Ship(_) if let Some(ss_attr_rid) = u_data.src.get_attr_consts().pilot_security_status => {
                item_data.reg_postproc(
                    ss_attr_rid,
                    ItemAttrPostprocs {
                        fast: sec_status_postproc_fast,
                        info: sec_status_postproc_info,
                    },
                );
            }
            UItem::Skill(_) if let Some(lvl_attr_rid) = u_data.src.get_attr_consts().skill_level => {
                item_data.reg_postproc(
                    lvl_attr_rid,
                    ItemAttrPostprocs {
                        fast: skill_level_postproc_fast,
                        info: skill_level_postproc_info,
                    },
                );
            }
            _ => (),
        }
        self.data.insert(item_uid, item_data);
    }
    pub(in crate::svc::calc) fn item_unloaded(&mut self, item_uid: &UItemId) {
        self.data.remove(item_uid);
    }
}
