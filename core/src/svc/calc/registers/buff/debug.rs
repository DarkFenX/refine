use super::BuffRegister;
use crate::{
    dbg::{DebugResult, check_attr_id, check_effect_id, check_item_id},
    svc::calc::debug::check_rmod,
    ud::UData,
};

impl BuffRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (&item_key, effect_keys) in self.effect_keys.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in effect_keys {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (aspec, rmods) in self.rmods.iter() {
            check_item_id(u_data, aspec.item_key, true)?;
            check_attr_id(u_data, aspec.attr_key)?;
            for rmod in rmods {
                check_rmod(u_data, rmod)?;
            }
        }
        Ok(())
    }
}
