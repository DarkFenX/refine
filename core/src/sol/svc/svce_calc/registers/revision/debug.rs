use crate::{
    sol::{svc::svce_calc::debug::check_modifier, SolView},
    util::DebugResult,
};

use super::SolRevisionRegister;

impl SolRevisionRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for modifier in self.item_add.iter() {
            check_modifier(sol_view, modifier)?;
        }
        for modifier in self.item_remove.iter() {
            check_modifier(sol_view, modifier)?;
        }
        Ok(())
    }
}
