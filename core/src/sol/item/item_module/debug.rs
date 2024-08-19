use crate::sol::{item::debug, SolDebugResult, SolView};

use super::SolModule;

impl SolModule {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        debug::check_fit(sol_view, &self.get_fit_id())?;
        if let Some(charge_id) = self.get_charge_id() {
            debug::check_item(sol_view, &charge_id)?;
        }
        self.get_projs().debug_consistency_check(sol_view)?;
        Ok(())
    }
}
