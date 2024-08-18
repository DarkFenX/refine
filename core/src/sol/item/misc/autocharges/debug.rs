use crate::sol::{item::debug, SolDebugResult, SolView};

use super::SolAutocharges;

impl SolAutocharges {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for autocharge_id in self.values() {
            debug::check_item(sol_view, autocharge_id)?;
        }
        Ok(())
    }
}
