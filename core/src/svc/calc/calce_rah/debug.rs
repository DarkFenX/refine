use super::RahSim;
use crate::{
    dbg::{DebugError, DebugResult, check_item_uid},
    ud::UData,
};

impl RahSim {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for &item_uid in self.resonances.keys() {
            check_item_uid(u_data, item_uid, true)?;
            // RAH sim should never be running during debug requests
            if self.sim_running {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
