use crate::{
    sol::{item::debug, SolView},
    util::DebugResult,
};

use super::SolFighter;

impl SolFighter {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for autocharge_item_id in self.autocharges.values() {
            debug::check_item(sol_view, autocharge_item_id)?;
        }
        Ok(())
    }
}
