use crate::sol::{svc::debug::check_item, SolDebugError, SolDebugResult, SolView};

use super::SolRahSim;

impl SolRahSim {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for item_id in self.resonances.keys() {
            check_item(sol_view, item_id)?;
            // RAH sim should never be running during debug requests
            if self.sim_running {
                return Err(SolDebugError::new());
            }
        }
        Ok(())
    }
}
