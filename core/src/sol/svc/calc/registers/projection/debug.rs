use crate::sol::{
    svc::debug::{check_effect, check_item},
    uad::SolUad,
    SolDebugResult,
};

use super::SolProjectionRegister;

impl SolProjectionRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for (affector_item_id, effect_id, affectee_item_id) in self.ranges.keys() {
            check_item(uad, affector_item_id, true)?;
            check_effect(uad, effect_id)?;
            check_item(uad, affectee_item_id, true)?;
        }
        Ok(())
    }
}
