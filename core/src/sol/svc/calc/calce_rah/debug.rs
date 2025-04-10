use crate::sol::{
    debug::{DebugError, DebugResult, check_item_key},
    uad::Uad,
};

use super::RahSim;

impl RahSim {
    pub(in crate::sol::svc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for &item_key in self.resonances.keys() {
            check_item_key(uad, item_key, true)?;
            // RAH sim should never be running during debug requests
            if self.sim_running {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
