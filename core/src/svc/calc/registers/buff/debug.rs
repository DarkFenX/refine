use super::BuffRegister;
use crate::{
    dbg::{DebugResult, check_a_effect_id, check_item_key},
    svc::calc::debug::check_rmod,
    uad::Uad,
};

impl BuffRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (&item_key, effect_ids) in self.a_effect_ids.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in effect_ids {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (aspec, rmods) in self.rmods.iter() {
            check_item_key(uad, aspec.item_key, true)?;
            for rmod in rmods {
                check_rmod(uad, rmod)?;
            }
        }
        Ok(())
    }
}
