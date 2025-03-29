use crate::sol::{
    debug::{DebugResult, check_fit_id, check_item_id},
    svc::vast::ValCache,
    uad::Uad,
};

use super::{Vast, VastFitData};

impl Vast {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for (fit_id, fit_data) in self.fit_datas.iter() {
            check_fit_id(uad, fit_id)?;
            fit_data.debug_consistency_check(uad)?;
        }
        Ok(())
    }
}

impl VastFitData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for item_id in self.mods_svcs_online.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.rigs_rigslot_calibration.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.drones_volume.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.drones_bandwidth.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.drones_online_bandwidth.keys() {
            // Holds not loaded drones as well
            check_item_id(uad, item_id, false)?;
        }
        for item_id in self.fighters_volume.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.fighters_online.iter() {
            check_item_id(uad, item_id, false)?;
        }
        for item_id in self.support_fighters.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.support_fighters_online.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.light_fighters.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.light_fighters_online.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.heavy_fighters.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.heavy_fighters_online.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.standup_support_fighters.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.standup_support_fighters_online.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.standup_light_fighters.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.standup_light_fighters_online.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.standup_heavy_fighters.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.standup_heavy_fighters_online.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.mods_turret.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.mods_launcher.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_ids in self.slotted_implants.values() {
            for item_id in item_ids {
                check_item_id(uad, item_id, true)?;
            }
        }
        for item_ids in self.slotted_boosters.values() {
            for item_id in item_ids {
                check_item_id(uad, item_id, true)?;
            }
        }
        for item_ids in self.slotted_subsystems.values() {
            for item_id in item_ids {
                check_item_id(uad, item_id, true)?;
            }
        }
        for item_id in self.ship_limited_items.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_ids in self.mods_svcs_rigs_max_group_fitted_all.values() {
            for item_id in item_ids {
                check_item_id(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_svcs_rigs_max_group_fitted_limited.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_ids in self.mods_svcs_max_group_online_all.values() {
            for item_id in item_ids {
                check_item_id(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_svcs_max_group_online_limited.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_ids in self.mods_max_group_active_all.values() {
            for item_id in item_ids {
                check_item_id(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_max_group_active_limited.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.rigs_rig_size.keys() {
            // This container can store info about non-loaded rigs
            check_item_id(uad, item_id, false)?;
        }
        for item_ids in self.srqs_skill_item_map.values() {
            for item_id in item_ids {
                check_item_id(uad, item_id, true)?;
            }
        }
        for item_id in self.srqs_missing.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for (item_id, item_data) in self.mods_charge_group.iter() {
            check_item_id(uad, item_id, true)?;
            if let ValCache::Fail(item_fail) = item_data {
                check_item_id(uad, &item_fail.parent_item_id, true)?;
                // This container can store info about non-loaded charges
                check_item_id(uad, &item_fail.charge_item_id, false)?;
            }
        }
        for (item_id, item_data) in self.mods_charge_size.iter() {
            check_item_id(uad, item_id, true)?;
            if let ValCache::Fail(item_fail) = item_data {
                check_item_id(uad, &item_fail.parent_item_id, true)?;
                // This container can store info about non-loaded charges
                check_item_id(uad, &item_fail.charge_item_id, false)?;
            }
        }
        for (item_id, item_data) in self.mods_charge_volume.iter() {
            // This container can store info about non-loaded modules
            check_item_id(uad, item_id, false)?;
            if let ValCache::Fail(item_fail) = item_data {
                // This container can store info about non-loaded modules
                check_item_id(uad, &item_fail.parent_item_id, false)?;
                check_item_id(uad, &item_fail.charge_item_id, true)?;
            }
        }
        for item_id in self.mods_capital.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.not_loaded.iter() {
            check_item_id(uad, item_id, false)?;
        }
        for (item_id, item_data) in self.mods_state.iter() {
            check_item_id(uad, item_id, true)?;
            check_item_id(uad, &item_data.item_id, true)?;
        }
        for (item_id, item_data) in self.item_kind.iter() {
            check_item_id(uad, item_id, true)?;
            check_item_id(uad, &item_data.item_id, true)?;
        }
        for item_id in self.drone_groups.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for (item_id, item_data) in self.fighter_squad_size.iter() {
            check_item_id(uad, item_id, true)?;
            check_item_id(uad, &item_data.item_id, true)?;
        }
        for item_id in self.overload_td_lvl.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_data in self.mods_svcs_max_type_fitted.values() {
            for item_id in item_data.keys() {
                check_item_id(uad, item_id, true)?;
            }
        }
        for item_id in self.sec_zone_fitted.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.sec_zone_fitted_wspace_banned.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.sec_zone_online_class.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.sec_zone_active.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.sec_zone_unonlineable_class.keys() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.sec_zone_unactivable.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.mods_active.iter() {
            check_item_id(uad, item_id, true)?;
        }
        for item_id in self.items_vs_ship_kind.iter() {
            check_item_id(uad, item_id, true)?;
        }
        Ok(())
    }
}
