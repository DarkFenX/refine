use crate::sol::{
    debug::{DebugError, DebugResult, check_item_id},
    uad::Uad,
};

use super::RahSim;

impl RahSim {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for item_id in self.resonances.keys() {
            check_item_id(uad, item_id, true)?;
            // RAH sim should never be running during debug requests
            if self.sim_running {
                return Err(DebugError::new());
            }
        }
        Ok(())
    }
}
