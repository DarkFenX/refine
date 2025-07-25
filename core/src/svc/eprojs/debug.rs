use crate::{
    dbg::{DebugError, DebugResult, check_a_effect_id, check_item_key},
    svc::eprojs::EProjs,
    ud::UData,
};

impl EProjs {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for ((projector_espec, projectee_key), svc_range) in self.ranges.iter() {
            check_item_key(u_data, projector_espec.item_key, true)?;
            check_a_effect_id(u_data, &projector_espec.a_effect_id)?;
            // Projectees are not necesssarily loaded
            check_item_key(u_data, *projectee_key, false)?;
            svc_range.consistency_check()?;
            let projector_projs = match u_data.items.get(projector_espec.item_key).get_projs() {
                Some(projector_projs) => projector_projs,
                None => return Err(DebugError {}),
            };
            match projector_projs.get(projectee_key) {
                Some(Some(u_range)) => {
                    // If ranges are defined on both, range mismatch is an error
                    if u_range != *svc_range {
                        return Err(DebugError {});
                    }
                }
                // Error in either of cases:
                // - when UAD item has no distance - since projection register is supposed to track
                // only relations with range
                // - no projection defined on UAD item
                _ => return Err(DebugError {}),
            }
        }
        Ok(())
    }
}
