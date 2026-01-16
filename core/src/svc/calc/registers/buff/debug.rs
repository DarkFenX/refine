use super::BuffRegister;
use crate::{dbg::DebugResult, ud::UData};

impl BuffRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (&item_uid, effect_rids) in self.effect_rids.iter() {
            item_uid.consistency_check(u_data, true)?;
            for &effect_rid in effect_rids {
                effect_rid.consistency_check(u_data)?;
            }
        }
        for (aspec, rmods) in self.rmods.iter() {
            aspec.consistency_check(u_data, true)?;
            for rmod in rmods {
                rmod.consistency_check(u_data)?;
            }
        }
        Ok(())
    }
}
