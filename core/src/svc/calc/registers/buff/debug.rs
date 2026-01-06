use super::BuffRegister;
use crate::{
    dbg::{DebugResult, check_attr_rid, check_effect_rid, check_item_uid},
    svc::calc::debug::check_rmod,
    ud::UData,
};

impl BuffRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (&item_uid, effect_rids) in self.effect_rids.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in effect_rids {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (aspec, rmods) in self.rmods.iter() {
            check_item_uid(u_data, aspec.item_uid, true)?;
            check_attr_rid(u_data, aspec.attr_rid)?;
            for rmod in rmods {
                check_rmod(u_data, rmod)?;
            }
        }
        Ok(())
    }
}
