use crate::sol::{
    debug::{check_fit, check_item, SolDebugResult},
    uad::SolUad,
};

use super::{SolVast, SolVastFitData};

impl SolVast {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for (fit_id, fit_data) in self.fit_datas.iter() {
            check_fit(uad, fit_id)?;
            fit_data.debug_consistency_check(uad)?;
        }
        Ok(())
    }
}

impl SolVastFitData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for item_id in self.mods_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.rigs_rigslot_calibration.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.drones_volume.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.drones_online_bandwidth.keys() {
            // Holds unloaded drones as well
            check_item(uad, item_id, false)?;
        }
        Ok(())
    }
}
