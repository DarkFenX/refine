use super::{Vast, VastFitData};
use crate::{
    dbg::{DebugResult, check_attr_id, check_effect_id, check_fit_id, check_item_id},
    ud::UData,
};

impl Vast {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (&fit_key, fit_data) in self.fit_datas.iter() {
            check_fit_id(u_data, fit_key)?;
            fit_data.consistency_check(u_data)?;
        }
        for &item_key in self.not_loaded.iter() {
            check_item_id(u_data, item_key, false)?;
        }
        for (projectee_key, projector_data) in self.irr_shield.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_id(u_data, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_id(u_data, projector_key, true)?;
                for &effect_key in projector_data.keys() {
                    check_effect_id(u_data, effect_key)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.irr_shield_limitable.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_id(u_data, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_id(u_data, projector_key, true)?;
                for &effect_key in projector_data.keys() {
                    check_effect_id(u_data, effect_key)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.irr_armor.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_id(u_data, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_id(u_data, projector_key, true)?;
                for &effect_key in projector_data.keys() {
                    check_effect_id(u_data, effect_key)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.irr_armor_limitable.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_id(u_data, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_id(u_data, projector_key, true)?;
                for &effect_key in projector_data.keys() {
                    check_effect_id(u_data, effect_key)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.irr_hull.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_id(u_data, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_id(u_data, projector_key, true)?;
                for &effect_key in projector_data.keys() {
                    check_effect_id(u_data, effect_key)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.in_cap.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_id(u_data, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_id(u_data, projector_key, true)?;
                for &effect_key in projector_data.keys() {
                    check_effect_id(u_data, effect_key)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.in_neuts.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_id(u_data, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_id(u_data, projector_key, true)?;
                for &effect_key in projector_data.keys() {
                    check_effect_id(u_data, effect_key)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.in_ecm.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_id(u_data, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_id(u_data, projector_key, true)?;
                for &effect_key in projector_data.keys() {
                    check_effect_id(u_data, effect_key)?;
                }
            }
        }
        Ok(())
    }
}

impl VastFitData {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for &item_key in self.mods_svcs_online.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.rigs_offline_calibration.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.drones_volume.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.drones_bandwidth.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.drones_online_bandwidth.keys() {
            // Holds not loaded drones as well
            check_item_id(u_data, item_key, false)?;
        }
        for &item_key in self.fighters_volume.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.fighters_online.iter() {
            // Holds not loaded fighters as well
            check_item_id(u_data, item_key, false)?;
        }
        for &item_key in self.light_fighters.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.light_fighters_online.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.support_fighters.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.support_fighters_online.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.heavy_fighters.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.heavy_fighters_online.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.st_light_fighters.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.st_light_fighters_online.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.st_heavy_fighters.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.st_heavy_fighters_online.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.st_support_fighters.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.st_support_fighters_online.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.mods_turret.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.mods_launcher.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for item_keys in self.slotted_implants.values() {
            for &item_key in item_keys {
                check_item_id(u_data, item_key, true)?;
            }
        }
        for item_keys in self.slotted_boosters.values() {
            for &item_key in item_keys {
                check_item_id(u_data, item_key, true)?;
            }
        }
        for item_keys in self.slotted_subsystems.values() {
            for &item_key in item_keys {
                check_item_id(u_data, item_key, true)?;
            }
        }
        for &item_key in self.ship_limited_items.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for item_keys in self.mods_svcs_rigs_max_group_fitted_all.values() {
            for &item_key in item_keys {
                check_item_id(u_data, item_key, true)?;
            }
        }
        for &item_key in self.mods_svcs_rigs_max_group_fitted_limited.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for item_keys in self.mods_svcs_max_group_online_all.values() {
            for &item_key in item_keys {
                check_item_id(u_data, item_key, true)?;
            }
        }
        for &item_key in self.mods_svcs_max_group_online_limited.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for item_keys in self.mods_max_group_active_all.values() {
            for &item_key in item_keys {
                check_item_id(u_data, item_key, true)?;
            }
        }
        for &item_key in self.mods_max_group_active_limited.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.rigs_rig_size.keys() {
            // This container can store info about non-loaded rigs
            check_item_id(u_data, item_key, false)?;
        }
        for item_keys in self.srqs_skill_item_map.values() {
            for &item_key in item_keys {
                check_item_id(u_data, item_key, true)?;
            }
        }
        for &item_key in self.srqs_missing.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for (&charge_key, &cont_key) in self.charge_group.iter() {
            check_item_id(u_data, cont_key, true)?;
            check_item_id(u_data, charge_key, true)?;
        }
        for (&charge_key, &cont_key) in self.charge_cont_group.iter() {
            check_item_id(u_data, cont_key, true)?;
            check_item_id(u_data, charge_key, true)?;
        }
        for (&charge_key, &cont_key) in self.charge_size.iter() {
            check_item_id(u_data, cont_key, true)?;
            check_item_id(u_data, charge_key, true)?;
        }
        for (&charge_key, &cont_key) in self.charge_volume.iter() {
            check_item_id(u_data, cont_key, true)?;
            check_item_id(u_data, charge_key, true)?;
        }
        for &item_key in self.mods_capital.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.not_loaded.iter() {
            check_item_id(u_data, item_key, false)?;
        }
        for &item_key in self.mods_state.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.item_kind.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.drone_groups.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.fighter_squad_size.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.overload_td_lvl.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for item_data in self.mods_svcs_max_type_fitted.values() {
            for &item_key in item_data.keys() {
                check_item_id(u_data, item_key, true)?;
            }
        }
        for &item_key in self.sec_zone_fitted.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.sec_zone_fitted_wspace_banned.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.sec_zone_online_class.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.sec_zone_active.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.sec_zone_unonlineable_class.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.sec_zone_unactivable.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for (&item_key, item_data) in self.sec_zone_effect.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for &item_key in self.mods_active.iter() {
            check_item_id(u_data, item_key, true)?;
        }
        for &item_key in self.mods_rigs_svcs_vs_ship_kind.keys() {
            check_item_id(u_data, item_key, true)?;
        }
        for (stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_id(u_data, stopped_espec.item_uid, false)?;
            for stopper_espec in stopper_especs {
                check_item_id(u_data, stopper_espec.item_uid, true)?;
            }
        }
        for (projector_espec, projectee_keys) in self.projectee_filter.iter() {
            check_item_id(u_data, projector_espec.item_uid, true)?;
            check_effect_id(u_data, projector_espec.effect_rid)?;
            for &projectee_key in projectee_keys.keys() {
                // Target is not guaranteed to be loaded
                check_item_id(u_data, projectee_key, false)?;
            }
        }
        for (projectee_key, projector_especs) in self.blockable_assistance.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_id(u_data, *projectee_key, false)?;
            for projector_espec in projector_especs {
                check_item_id(u_data, projector_espec.item_uid, true)?;
                check_effect_id(u_data, projector_espec.effect_rid)?;
            }
        }
        for (projectee_key, projector_especs) in self.blockable_offense.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_id(u_data, *projectee_key, false)?;
            for projector_espec in projector_especs {
                check_item_id(u_data, projector_espec.item_uid, true)?;
                check_effect_id(u_data, projector_espec.effect_rid)?;
            }
        }
        for (projectee_aspec, projector_especs) in self.resist_immunity.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_id(u_data, projectee_aspec.item_key, false)?;
            check_attr_id(u_data, projectee_aspec.attr_key)?;
            for projector_espec in projector_especs {
                check_item_id(u_data, projector_espec.item_uid, true)?;
                check_effect_id(u_data, projector_espec.effect_rid)?;
            }
        }
        for (&item_key, attr_keys) in self.cap_consumers_all.iter() {
            check_item_id(u_data, item_key, true)?;
            for &attr_key in attr_keys {
                check_attr_id(u_data, attr_key)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - damage output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_key, item_data) in self.dmg_normal.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.dmg_breacher.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - mining output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_key, item_data) in self.mining_ore.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.mining_ice.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.mining_gas.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - RR output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_key, item_data) in self.orr_shield.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.orr_armor.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.orr_hull.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - misc output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_key, item_data) in self.out_neuts.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.out_cap.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - local active tank
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_key, item_data) in self.lr_shield.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.lr_shield_limitable.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.lr_armor.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.lr_armor_limitable.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        for (&item_key, item_data) in self.lr_hull.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - cap
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_key, item_data) in self.cap_consumers_active.iter() {
            check_item_id(u_data, item_key, true)?;
            for (&effect_key, &attr_key) in item_data.iter() {
                check_effect_id(u_data, effect_key)?;
                check_attr_id(u_data, attr_key)?;
            }
        }
        for (&item_key, item_data) in self.cap_injects.iter() {
            check_item_id(u_data, item_key, true)?;
            for &effect_key in item_data.keys() {
                check_effect_id(u_data, effect_key)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - misc
        ////////////////////////////////////////////////////////////////////////////////////////////
        for espec in self.aggro_effects.iter() {
            check_item_id(u_data, espec.item_uid, true)?;
            check_effect_id(u_data, espec.effect_rid)?;
        }
        Ok(())
    }
}
