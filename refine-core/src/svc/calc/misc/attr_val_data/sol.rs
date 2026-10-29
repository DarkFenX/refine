use super::{attr::AttrEntry, item::ItemAttrData};
use crate::{
    rd::RAttrId,
    svc::calc::{ItemAttrPostproc, misc::Reuse},
    ud::{UData, UItem, UItemId},
    util::{RMap, SSlab},
};

#[derive(Clone)]
pub(in crate::svc::calc) struct AttrValData {
    pub(super) data: SSlab<UItemId, ItemAttrData>,
    // Stores unused containers from unloaded items, to be used later for loaded items
    spare_dense: Reuse<Vec<(RAttrId, AttrEntry)>>,
    spare_sparse: Reuse<RMap<RAttrId, AttrEntry>>,
}
impl AttrValData {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            data: SSlab::new(),
            spare_dense: Reuse::default(),
            spare_sparse: Reuse::default(),
        }
    }
    // Query methods
    pub(in crate::svc::calc) fn get_item_attr_data(&self, item_uid: UItemId) -> Option<&ItemAttrData> {
        self.data.get(item_uid)
    }
    pub(in crate::svc::calc) fn get_item_attr_data_mut(&mut self, item_uid: UItemId) -> Option<&mut ItemAttrData> {
        self.data.get_mut(item_uid)
    }
    // Modification methods
    pub(in crate::svc::calc) fn item_loaded(&mut self, u_data: &UData, item_uid: UItemId, item: &UItem) {
        let mut item_data = ItemAttrData::from_parts(self.spare_dense.take(), self.spare_sparse.take());
        match item {
            UItem::Fighter(..) if let Some(count_attr_rid) = u_data.r_data.get_attr_consts().ftr_sq_size => {
                item_data.reg_postproc(count_attr_rid, ItemAttrPostproc::FighterCount);
            }
            UItem::Ship(..) if let Some(ss_attr_rid) = u_data.r_data.get_attr_consts().pilot_security_status => {
                item_data.reg_postproc(ss_attr_rid, ItemAttrPostproc::SecStatus);
            }
            UItem::Skill(..) if let Some(lvl_attr_rid) = u_data.r_data.get_attr_consts().skill_level => {
                item_data.reg_postproc(lvl_attr_rid, ItemAttrPostproc::SkillLevel);
            }
            _ => (),
        }
        self.data.insert(item_uid, item_data);
    }
    pub(in crate::svc::calc) fn item_unloaded(&mut self, item_uid: UItemId) {
        if let Some(item_data) = self.data.take(item_uid) {
            let (dense_entries, map) = item_data.into_parts();
            self.spare_dense.put(dense_entries);
            self.spare_sparse.put(map);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debug
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl AttrValData {
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                data,
                // Spare containers
                spare_dense: _,
                spare_sparse: _,
            } = self;
            for (item_uid, item_data) in data.iter() {
                item_uid.consistency_check(u_data, true)?;
                item_data.consistency_check()?;
                for (attr_rid, attr_entry) in item_data.iter() {
                    attr_rid.consistency_check(u_data)?;
                    if let Some(attr_val) = attr_entry.value {
                        attr_val.base.consistency_check()?;
                        attr_val.dogma.consistency_check()?;
                        attr_val.extra.consistency_check()?;
                    }
                }
            }
            Ok(())
        }
    }
}
