use crate::sol::{svc::debug::check_item, SolDebugResult, SolView};

use super::SolRestatRegModsOnline;

impl SolRestatRegModsOnline {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for item_id in self.items.iter() {
            check_item(sol_view, item_id)?;
        }
        Ok(())
    }
}
