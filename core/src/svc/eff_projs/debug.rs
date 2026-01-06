use crate::{
    dbg::{DebugError, DebugResult, check_effect_rid, check_item_uid},
    svc::eff_projs::EffProjs,
    ud::UData,
};

impl EffProjs {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for ((projector_espec, projectee_uid), svc_proj_data) in self.proj_datas.iter() {
            check_item_uid(u_data, projector_espec.item_uid, true)?;
            check_effect_rid(u_data, projector_espec.effect_rid)?;
            // Projectees are not necessarily loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            svc_proj_data.consistency_check()?;
            let projector_projs = match u_data.items.get(projector_espec.item_uid).get_projs() {
                Some(projector_projs) => projector_projs,
                None => return Err(DebugError {}),
            };
            match projector_projs.get(projectee_uid) {
                Some(Some(u_proj_data)) => {
                    // If datas are defined on both, data mismatch is an error
                    if u_proj_data != *svc_proj_data {
                        return Err(DebugError {});
                    }
                }
                // Error in either of cases:
                // - when user data item has no projection data - since projection register is
                // supposed to track only relations with projection data
                // - no projection defined on user data item
                _ => return Err(DebugError {}),
            }
        }
        Ok(())
    }
}
