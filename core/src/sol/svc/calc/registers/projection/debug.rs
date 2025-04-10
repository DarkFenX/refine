use crate::sol::{
    debug::{DebugResult, check_a_effect_id, check_item_key},
    uad::Uad,
};

use super::ProjectionRegister;

impl ProjectionRegister {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (affector_item_key, a_effect_id, affectee_item_key) in self.ranges.keys() {
            check_item_key(uad, *affector_item_key, true)?;
            check_a_effect_id(uad, a_effect_id)?;
            check_item_key(uad, *affectee_item_key, true)?;
        }
        Ok(())
    }
}
