use crate::{
    sol::{
        svc::debug::{check_effect, check_item},
        SolView,
    },
    util::DebugResult,
};

use super::SolProjectionRegister;

impl SolProjectionRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for (affector_item_id, effect_id, affectee_item_id) in self.ranges.keys() {
            check_item(sol_view, affector_item_id)?;
            check_effect(sol_view, effect_id)?;
            check_item(sol_view, affectee_item_id)?;
        }
        Ok(())
    }
}
