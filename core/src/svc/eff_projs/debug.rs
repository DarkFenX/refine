use super::main::EffProjs;
use crate::{
    dbg::{DebugError, DebugResult},
    ud::UData,
};

impl EffProjs {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for ((projector_espec, projectee_uid), svc_proj_data) in self.proj_datas.iter() {
            projector_espec.consistency_check(u_data, true)?;
            // Projectees are not necessarily loaded
            projectee_uid.consistency_check(u_data, false)?;
            svc_proj_data.consistency_check()?;
            let Some(projector_projs) = u_data.items.get(projector_espec.item_uid).get_projs() else {
                return Err(DebugError {});
            };
            let Some(Some(u_proj_data)) = projector_projs.get(projectee_uid) else {
                // Error in either of cases:
                // - when user data item has no projection data - since projection register is
                // supposed to track only relations with projection data
                // - no projection defined on user data item
                return Err(DebugError {});
            };
            // If datas are defined on both, data mismatch is an error
            if u_proj_data != *svc_proj_data {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
