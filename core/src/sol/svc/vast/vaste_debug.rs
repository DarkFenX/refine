use super::{Vast, VastFitData};
use crate::sol::{
    debug::{DebugResult, check_fit_key, check_item_key},
    svc::vast::ValCache,
    uad::Uad,
};

impl Vast {
    pub(in crate::sol::svc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (&fit_key, fit_data) in self.fit_datas.iter() {
            check_fit_key(uad, fit_key)?;
            fit_data.consistency_check(uad)?;
        }
        for &item_key in self.not_loaded.iter() {
            check_item_key(uad, item_key, false)?;
        }
        Ok(())
    }
}

impl VastFitData {
    pub(in crate::sol::svc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for &item_key in self.mods_svcs_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.rigs_offline_calibration.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.drones_volume.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.drones_bandwidth.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.drones_online_bandwidth.keys() {
            // Holds not loaded drones as well
            check_item_key(uad, item_key, false)?;
        }
        for &item_key in self.fighters_volume.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.fighters_online.iter() {
            // Holds not loaded fighters as well
            check_item_key(uad, item_key, false)?;
        }
        for &item_key in self.support_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.support_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.light_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.light_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.heavy_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.heavy_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_support_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_support_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_light_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_light_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_heavy_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_heavy_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.mods_turret.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.mods_launcher.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for item_keys in self.slotted_implants.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for item_keys in self.slotted_boosters.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for item_keys in self.slotted_subsystems.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.ship_limited_items.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for item_keys in self.mods_svcs_rigs_max_group_fitted_all.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.mods_svcs_rigs_max_group_fitted_limited.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for item_keys in self.mods_svcs_max_group_online_all.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.mods_svcs_max_group_online_limited.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for item_keys in self.mods_max_group_active_all.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.mods_max_group_active_limited.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.rigs_rig_size.keys() {
            // This container can store info about non-loaded rigs
            check_item_key(uad, item_key, false)?;
        }
        for item_keys in self.srqs_skill_item_map.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.srqs_missing.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for (&module_item_key, module_data) in self.mods_charge_group.iter() {
            check_item_key(uad, module_item_key, true)?;
            if let ValCache::Fail(fail_cache) = module_data {
                check_item_key(uad, fail_cache.parent_item_key, true)?;
                check_item_key(uad, fail_cache.charge_item_key, true)?;
            }
        }
        for (&module_item_key, module_data) in self.mods_charge_size.iter() {
            check_item_key(uad, module_item_key, true)?;
            if let ValCache::Fail(fail_cache) = module_data {
                check_item_key(uad, fail_cache.parent_item_key, true)?;
                check_item_key(uad, fail_cache.charge_item_key, true)?;
            }
        }
        for (&module_item_key, module_data) in self.mods_charge_volume.iter() {
            // This container can store info about non-loaded modules
            check_item_key(uad, module_item_key, false)?;
            if let ValCache::Fail(fail_cache) = module_data {
                // This container can store info about non-loaded modules
                check_item_key(uad, fail_cache.parent_item_key, false)?;
                check_item_key(uad, fail_cache.charge_item_key, true)?;
            }
        }
        for &item_key in self.mods_capital.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.not_loaded.iter() {
            check_item_key(uad, item_key, false)?;
        }
        for &item_key in self.mods_state.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.item_kind.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.drone_groups.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.fighter_squad_size.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.overload_td_lvl.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for item_data in self.mods_svcs_max_type_fitted.values() {
            for &item_key in item_data.keys() {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.sec_zone_fitted.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_fitted_wspace_banned.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_online_class.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_active.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_unonlineable_class.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_unactivable.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.mods_active.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.mods_rigs_svcs_vs_ship_kind.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for (stopped_spec, stopper_specs) in self.stopped_effects.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_key(uad, stopped_spec.item_key, false)?;
            for stopper_spec in stopper_specs {
                check_item_key(uad, stopper_spec.item_key, true)?;
            }
        }
        for (effect_spec, target_item_keys) in self.blockable_assistance.iter() {
            check_item_key(uad, effect_spec.item_key, true)?;
            for &target_item_key in target_item_keys {
                // There is no logic which ensures that projection target is loaded
                check_item_key(uad, target_item_key, false)?;
            }
        }
        for (effect_spec, target_item_keys) in self.blockable_offense.iter() {
            check_item_key(uad, effect_spec.item_key, true)?;
            for &target_item_key in target_item_keys {
                // There is no logic which ensures that projection target is loaded
                check_item_key(uad, target_item_key, false)?;
            }
        }
        Ok(())
    }
}
