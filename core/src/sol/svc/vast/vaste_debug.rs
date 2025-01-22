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
        for item_id in self.fighters_online.iter() {
            check_item(uad, item_id, false)?;
        }
        for item_id in self.support_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.light_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.heavy_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_support_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_light_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_heavy_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.mods_turret.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.mods_launcher.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.slotted_implants.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_ids in self.slotted_boosters.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_ids in self.slotted_subsystems.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.ship_limited_mods_rigs_subs.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_rigs_max_group_fitted_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_rigs_max_group_fitted_limited.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_rigs_max_group_online_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_rigs_max_group_online_limited.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_rigs_max_group_active_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_rigs_max_group_active_limited.iter() {
            check_item(uad, item_id, true)?;
        }
        Ok(())
    }
}
