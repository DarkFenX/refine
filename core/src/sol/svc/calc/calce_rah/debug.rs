use crate::sol::{
    debug::{SolDebugError, SolDebugResult, check_item},
    uad::SolUad,
};

use super::SolRahSim;

impl SolRahSim {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for item_id in self.resonances.keys() {
            check_item(uad, item_id, true)?;
            // RAH sim should never be running during debug requests
            if self.sim_running {
                return Err(SolDebugError::new());
            }
        }
        Ok(())
    }
}
