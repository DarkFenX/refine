use super::{Vast, VastFitData};
use crate::{
    dbg::{DebugResult, check_attr_rid, check_effect_rid, check_fit_uid, check_item_uid},
    ud::UData,
};

impl Vast {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (&fit_uid, fit_data) in self.fit_datas.iter() {
            check_fit_uid(u_data, fit_uid)?;
            fit_data.consistency_check(u_data)?;
        }
        for &item_uid in self.not_loaded.iter() {
            check_item_uid(u_data, item_uid, false)?;
        }
        for (projectee_uid, projector_data) in self.irr_shield.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for (&projector_uid, projector_data) in projector_data.iter() {
                check_item_uid(u_data, projector_uid, true)?;
                for &effect_rid in projector_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.irr_shield_limitable.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for (&projector_uid, projector_data) in projector_data.iter() {
                check_item_uid(u_data, projector_uid, true)?;
                for &effect_rid in projector_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.irr_armor.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for (&projector_uid, projector_data) in projector_data.iter() {
                check_item_uid(u_data, projector_uid, true)?;
                for &effect_rid in projector_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.irr_armor_limitable.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for (&projector_uid, projector_data) in projector_data.iter() {
                check_item_uid(u_data, projector_uid, true)?;
                for &effect_rid in projector_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.irr_hull.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for (&projector_uid, projector_data) in projector_data.iter() {
                check_item_uid(u_data, projector_uid, true)?;
                for &effect_rid in projector_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.in_cap.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for (&projector_uid, projector_data) in projector_data.iter() {
                check_item_uid(u_data, projector_uid, true)?;
                for &effect_rid in projector_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.in_neuts.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for (&projector_uid, projector_data) in projector_data.iter() {
                check_item_uid(u_data, projector_uid, true)?;
                for &effect_rid in projector_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.in_ecm.iter() {
            // Projectees are not guaranteed to be loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for (&projector_uid, projector_data) in projector_data.iter() {
                check_item_uid(u_data, projector_uid, true)?;
                for &effect_rid in projector_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
        }
        Ok(())
    }
}

impl VastFitData {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for &item_uid in self.mods_svcs_online.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.rigs_offline_calibration.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.drones_volume.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.drones_bandwidth.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.drones_online_bandwidth.keys() {
            // Holds not loaded drones as well
            check_item_uid(u_data, item_uid, false)?;
        }
        for &item_uid in self.fighters_volume.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.fighters_online.iter() {
            // Holds not loaded fighters as well
            check_item_uid(u_data, item_uid, false)?;
        }
        for &item_uid in self.light_fighters.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.light_fighters_online.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.support_fighters.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.support_fighters_online.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.heavy_fighters.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.heavy_fighters_online.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.st_light_fighters.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.st_light_fighters_online.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.st_heavy_fighters.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.st_heavy_fighters_online.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.st_support_fighters.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.st_support_fighters_online.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.mods_turret.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.mods_launcher.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for item_uids in self.slotted_implants.values() {
            for &item_uid in item_uids {
                check_item_uid(u_data, item_uid, true)?;
            }
        }
        for item_uids in self.slotted_boosters.values() {
            for &item_uid in item_uids {
                check_item_uid(u_data, item_uid, true)?;
            }
        }
        for item_uids in self.slotted_subsystems.values() {
            for &item_uid in item_uids {
                check_item_uid(u_data, item_uid, true)?;
            }
        }
        for &item_uid in self.ship_limited_items.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for item_uids in self.mods_svcs_rigs_max_group_fitted_all.values() {
            for &item_uid in item_uids {
                check_item_uid(u_data, item_uid, true)?;
            }
        }
        for &item_uid in self.mods_svcs_rigs_max_group_fitted_limited.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for item_uids in self.mods_svcs_max_group_online_all.values() {
            for &item_uid in item_uids {
                check_item_uid(u_data, item_uid, true)?;
            }
        }
        for &item_uid in self.mods_svcs_max_group_online_limited.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for item_uids in self.mods_max_group_active_all.values() {
            for &item_uid in item_uids {
                check_item_uid(u_data, item_uid, true)?;
            }
        }
        for &item_uid in self.mods_max_group_active_limited.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.rigs_rig_size.keys() {
            // This container can store info about non-loaded rigs
            check_item_uid(u_data, item_uid, false)?;
        }
        for item_uids in self.srqs_skill_item_map.values() {
            for &item_uid in item_uids {
                check_item_uid(u_data, item_uid, true)?;
            }
        }
        for &item_uid in self.srqs_missing.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for (&charge_uid, &cont_uid) in self.charge_group.iter() {
            check_item_uid(u_data, cont_uid, true)?;
            check_item_uid(u_data, charge_uid, true)?;
        }
        for (&charge_uid, &cont_uid) in self.charge_cont_group.iter() {
            check_item_uid(u_data, cont_uid, true)?;
            check_item_uid(u_data, charge_uid, true)?;
        }
        for (&charge_uid, &cont_uid) in self.charge_size.iter() {
            check_item_uid(u_data, cont_uid, true)?;
            check_item_uid(u_data, charge_uid, true)?;
        }
        for (&charge_uid, &cont_uid) in self.charge_volume.iter() {
            check_item_uid(u_data, cont_uid, true)?;
            check_item_uid(u_data, charge_uid, true)?;
        }
        for &item_uid in self.mods_capital.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.not_loaded.iter() {
            check_item_uid(u_data, item_uid, false)?;
        }
        for &item_uid in self.mods_state.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.item_kind.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.drone_groups.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.fighter_squad_size.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.overload_td_lvl.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for item_data in self.mods_svcs_max_type_fitted.values() {
            for &item_uid in item_data.keys() {
                check_item_uid(u_data, item_uid, true)?;
            }
        }
        for &item_uid in self.sec_zone_fitted.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.sec_zone_fitted_wspace_banned.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.sec_zone_online_class.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.sec_zone_active.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.sec_zone_unonlineable_class.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.sec_zone_unactivable.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for (&item_uid, item_data) in self.sec_zone_effect.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for &item_uid in self.mods_active.iter() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for &item_uid in self.mods_rigs_svcs_vs_ship_kind.keys() {
            check_item_uid(u_data, item_uid, true)?;
        }
        for (stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_uid(u_data, stopped_espec.item_uid, false)?;
            for stopper_espec in stopper_especs {
                check_item_uid(u_data, stopper_espec.item_uid, true)?;
            }
        }
        for (projector_espec, projectee_uids) in self.projectee_filter.iter() {
            check_item_uid(u_data, projector_espec.item_uid, true)?;
            check_effect_rid(u_data, projector_espec.effect_rid)?;
            for &projectee_uid in projectee_uids.keys() {
                // Target is not guaranteed to be loaded
                check_item_uid(u_data, projectee_uid, false)?;
            }
        }
        for (projectee_uid, projector_especs) in self.blockable_assistance.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for projector_espec in projector_especs {
                check_item_uid(u_data, projector_espec.item_uid, true)?;
                check_effect_rid(u_data, projector_espec.effect_rid)?;
            }
        }
        for (projectee_uid, projector_especs) in self.blockable_offense.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_uid(u_data, *projectee_uid, false)?;
            for projector_espec in projector_especs {
                check_item_uid(u_data, projector_espec.item_uid, true)?;
                check_effect_rid(u_data, projector_espec.effect_rid)?;
            }
        }
        for (projectee_aspec, projector_especs) in self.resist_immunity.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_uid(u_data, projectee_aspec.item_uid, false)?;
            check_attr_rid(u_data, projectee_aspec.attr_rid)?;
            for projector_espec in projector_especs {
                check_item_uid(u_data, projector_espec.item_uid, true)?;
                check_effect_rid(u_data, projector_espec.effect_rid)?;
            }
        }
        for (&item_uid, attr_rids) in self.cap_consumers_all.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &attr_rid in attr_rids {
                check_attr_rid(u_data, attr_rid)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - damage output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_uid, item_data) in self.dmg_normal.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.dmg_breacher.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - mining output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_uid, item_data) in self.mining_ore.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.mining_ice.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.mining_gas.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - RR output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_uid, item_data) in self.orr_shield.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.orr_armor.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.orr_hull.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - misc output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_uid, item_data) in self.out_neuts.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.out_cap.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - local active tank
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_uid, item_data) in self.lr_shield.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.lr_shield_limitable.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.lr_armor.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.lr_armor_limitable.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        for (&item_uid, item_data) in self.lr_hull.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - cap
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (&item_uid, item_data) in self.cap_consumers_active.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for (&effect_rid, &attr_rid) in item_data.iter() {
                check_effect_rid(u_data, effect_rid)?;
                check_attr_rid(u_data, attr_rid)?;
            }
        }
        for (&item_uid, item_data) in self.cap_injects.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for &effect_rid in item_data.keys() {
                check_effect_rid(u_data, effect_rid)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - misc
        ////////////////////////////////////////////////////////////////////////////////////////////
        for espec in self.aggro_effects.iter() {
            check_item_uid(u_data, espec.item_uid, true)?;
            check_effect_rid(u_data, espec.effect_rid)?;
        }
        Ok(())
    }
}
