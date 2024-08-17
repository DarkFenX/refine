use crate::sol::{item::debug, SolDebugResult, SolView};

use super::SolProjEffect;

impl SolProjEffect {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for projectee_item_id in self.get_projs().iter_items() {
            debug::check_item(sol_view, projectee_item_id)?;
        }
        Ok(())
    }
}
