use super::BuffRegister;
use crate::{dbg::DebugResult, svc::calc::registers::RawModStorage, ud::UData};

impl BuffRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData, rmods: &RawModStorage) -> DebugResult {
        let Self {
            effect_rids,
            rmods: aspec_rmods,
            projectees,
        } = self;
        for (&item_uid, effect_rids) in effect_rids.iter() {
            item_uid.consistency_check(u_data, true)?;
            for &effect_rid in effect_rids {
                effect_rid.consistency_check(u_data)?;
            }
        }
        for (aspec, rmod_ids) in aspec_rmods.iter() {
            aspec.consistency_check(u_data, true)?;
            for &rmod_id in rmod_ids {
                rmod_id.consistency_check(u_data, rmods)?;
            }
        }
        for (espec, projectee_uids) in projectees.iter() {
            espec.consistency_check(u_data, true)?;
            for &projectee_uid in projectee_uids {
                // Projectees don't have to be loaded for an entry to be added here
                projectee_uid.consistency_check(u_data, false)?;
            }
        }
        Ok(())
    }
}
