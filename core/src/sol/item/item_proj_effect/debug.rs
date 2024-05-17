use crate::{
    sol::{item::debug, SolView},
    util::DebugResult,
};

use super::SolProjEffect;

impl SolProjEffect {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for tgt_item_id in self.projs.iter_items() {
            debug::check_item(sol_view, tgt_item_id)?;
        }
        Ok(())
    }
}
