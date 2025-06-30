use crate::{
    dbg::{DebugError, DebugResult, check_a_effect_id, check_item_key},
    svc::eprojs::EProjs,
    uad::Uad,
};

impl EProjs {
    pub(crate) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for ((affector_espec, affectee_item_key), svc_range) in self.ranges.iter() {
            check_item_key(uad, affector_espec.item_key, true)?;
            check_a_effect_id(uad, &affector_espec.a_effect_id)?;
            check_item_key(uad, *affectee_item_key, true)?;
            let affector_projs = match uad.items.get(affector_espec.item_key).get_projs() {
                Some(affector_projs) => affector_projs,
                None => return Err(DebugError {}),
            };
            match affector_projs.get(affectee_item_key) {
                Some(Some(uad_range)) => {
                    // If ranges are defined on both, range mismatch is an error
                    if uad_range != svc_range {
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
