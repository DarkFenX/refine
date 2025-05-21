use super::ProjectionRegister;
use crate::sol::{
    debug::{DebugError, DebugResult, check_a_effect_id, check_item_key},
    uad::Uad,
};

impl ProjectionRegister {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for ((affector_item_key, a_effect_id, affectee_item_key), calc_range) in self.ranges.iter() {
            check_item_key(uad, *affector_item_key, true)?;
            check_a_effect_id(uad, a_effect_id)?;
            check_item_key(uad, *affectee_item_key, true)?;
            let affector_projs = match uad.items.get(*affector_item_key).get_projs() {
                Some(affector_projs) => affector_projs,
                None => return Err(DebugError {}),
            };
            match affector_projs.get(affectee_item_key) {
                Some(uad_range) => match uad_range {
                    // If ranges are defined on both, range mismatch is an error
                    Some(uad_range) => {
                        if uad_range != calc_range {
                            return Err(DebugError {});
                        }
                    }
                    // When UAD item has no distance - it's an error, since projection register is
                    // supposed to track only relations with range
                    None => return Err(DebugError {}),
                },
                // No projection defined on UAD item is an error
                None => return Err(DebugError {}),
            }
        }
        Ok(())
    }
}
