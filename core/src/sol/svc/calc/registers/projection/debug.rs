use crate::sol::{
    debug::{DebugResult, check_a_effect_id, check_item_id},
    uad::Uad,
};

use super::ProjectionRegister;

impl ProjectionRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for (affector_item_id, a_effect_id, affectee_item_id) in self.ranges.keys() {
            check_item_id(uad, affector_item_id, true)?;
            check_a_effect_id(uad, a_effect_id)?;
            check_item_id(uad, affectee_item_id, true)?;
        }
        Ok(())
    }
}
