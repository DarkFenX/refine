use super::RawModStorage;
use crate::{dbg::DebugResult, ud::UData};

impl RawModStorage {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self { data, by_espec } = self;
        for (rmod_id, rmod) in data.iter() {
            // Every stored modifier has to be tracked by its effect, otherwise it has leaked
            if !by_espec.contains_entry(&rmod.affector_espec, &rmod_id) {
                return Err(Default::default());
            }
            rmod.consistency_check(u_data)?;
        }
        for (espec, rmod_ids) in by_espec.iter() {
            espec.consistency_check(u_data, true)?;
            for &rmod_id in rmod_ids {
                if data.get(rmod_id).is_none() {
                    return Err(Default::default());
                }
            }
        }
        Ok(())
    }
}
