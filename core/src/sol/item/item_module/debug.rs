use crate::{
    sol::{item::debug, SolView},
    util::DebugResult,
};

use super::SolModule;

impl SolModule {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for projectee_item_id in self.projs.iter_items() {
            debug::check_item(sol_view, projectee_item_id)?;
        }
        Ok(())
    }
}
