use crate::sol::{
    svc::debug::{check_fit, check_item},
    SolDebugResult, SolView,
};

use super::SolStatRegModsOnline;

impl SolStatRegModsOnline {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for (fit_id, item_ids) in self.items.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        Ok(())
    }
}
