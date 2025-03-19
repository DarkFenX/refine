use crate::sol::{
    debug::{SolDebugResult, check_fit, check_item},
    svc::vast::SolValCache,
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
        for item_id in self.mods_svcs_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.rigs_rigslot_calibration.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.drones_volume.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.drones_bandwidth.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.drones_online_bandwidth.keys() {
            // Holds not loaded drones as well
            check_item(uad, item_id, false)?;
        }
        for item_id in self.fighters_volume.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.fighters_online.iter() {
            check_item(uad, item_id, false)?;
        }
        for item_id in self.support_fighters.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.support_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.light_fighters.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.light_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.heavy_fighters.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.heavy_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_support_fighters.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_support_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_light_fighters.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_light_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_heavy_fighters.iter() {
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
        for item_id in self.ship_limited_items.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_svcs_rigs_max_group_fitted_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_svcs_rigs_max_group_fitted_limited.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_svcs_max_group_online_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_svcs_max_group_online_limited.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_max_group_active_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_max_group_active_limited.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.rigs_rig_size.keys() {
            // This container can store info about non-loaded rigs
            check_item(uad, item_id, false)?;
        }
        for item_ids in self.srqs_skill_item_map.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.srqs_missing.keys() {
            check_item(uad, item_id, true)?;
        }
        for (item_id, item_data) in self.mods_charge_group.iter() {
            check_item(uad, item_id, true)?;
            if let SolValCache::Fail(item_fail) = item_data {
                check_item(uad, &item_fail.parent_item_id, true)?;
                // This container can store info about non-loaded charges
                check_item(uad, &item_fail.charge_item_id, false)?;
            }
        }
        for (item_id, item_data) in self.mods_charge_size.iter() {
            check_item(uad, item_id, true)?;
            if let SolValCache::Fail(item_fail) = item_data {
                check_item(uad, &item_fail.parent_item_id, true)?;
                // This container can store info about non-loaded charges
                check_item(uad, &item_fail.charge_item_id, false)?;
            }
        }
        for (item_id, item_data) in self.mods_charge_volume.iter() {
            // This container can store info about non-loaded modules
            check_item(uad, item_id, false)?;
            if let SolValCache::Fail(item_fail) = item_data {
                // This container can store info about non-loaded modules
                check_item(uad, &item_fail.parent_item_id, false)?;
                check_item(uad, &item_fail.charge_item_id, true)?;
            }
        }
        for item_id in self.mods_capital.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.not_loaded.iter() {
            check_item(uad, item_id, false)?;
        }
        for (item_id, item_data) in self.mods_state.iter() {
            check_item(uad, item_id, true)?;
            check_item(uad, &item_data.item_id, true)?;
        }
        for (item_id, item_data) in self.item_kind.iter() {
            check_item(uad, item_id, true)?;
            check_item(uad, &item_data.item_id, true)?;
        }
        for item_id in self.drone_groups.keys() {
            check_item(uad, item_id, true)?;
        }
        for (item_id, item_data) in self.fighter_count.iter() {
            check_item(uad, item_id, true)?;
            check_item(uad, &item_data.item_id, true)?;
        }
        for item_id in self.overload_td_lvl.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_data in self.mods_svcs_max_type_fitted.values() {
            for item_id in item_data.keys() {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_svcs_sec_class_online.keys() {
            check_item(uad, item_id, true)?;
        }
        Ok(())
    }
}
