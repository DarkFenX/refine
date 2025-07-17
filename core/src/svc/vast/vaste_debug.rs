use super::{Vast, VastFitData};
use crate::{
    dbg::{DebugResult, check_a_effect_id, check_fit_key, check_item_key},
    uad::Uad,
};

impl Vast {
    pub(in crate::svc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (&fit_key, fit_data) in self.fit_datas.iter() {
            check_fit_key(uad, fit_key)?;
            fit_data.consistency_check(uad)?;
        }
        for &item_key in self.not_loaded.iter() {
            check_item_key(uad, item_key, false)?;
        }
        for (projectee_key, projector_data) in self.irr_shield.iter() {
            // Projectee are not guaranteed to be loaded
            check_item_key(uad, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_key(uad, projector_key, true)?;
                for a_effect_id in projector_data.keys() {
                    check_a_effect_id(uad, &a_effect_id)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.irr_shield_limitable.iter() {
            // Projectee are not guaranteed to be loaded
            check_item_key(uad, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_key(uad, projector_key, true)?;
                for a_effect_id in projector_data.keys() {
                    check_a_effect_id(uad, &a_effect_id)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.irr_armor.iter() {
            // Projectee are not guaranteed to be loaded
            check_item_key(uad, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_key(uad, projector_key, true)?;
                for a_effect_id in projector_data.keys() {
                    check_a_effect_id(uad, &a_effect_id)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.irr_armor_limitable.iter() {
            // Projectee are not guaranteed to be loaded
            check_item_key(uad, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_key(uad, projector_key, true)?;
                for a_effect_id in projector_data.keys() {
                    check_a_effect_id(uad, &a_effect_id)?;
                }
            }
        }
        for (projectee_key, projector_data) in self.irr_hull.iter() {
            // Projectee are not guaranteed to be loaded
            check_item_key(uad, *projectee_key, false)?;
            for (&projector_key, projector_data) in projector_data.iter() {
                check_item_key(uad, projector_key, true)?;
                for a_effect_id in projector_data.keys() {
                    check_a_effect_id(uad, &a_effect_id)?;
                }
            }
        }
        Ok(())
    }
}

impl VastFitData {
    pub(in crate::svc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
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
        for &item_key in self.light_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.light_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.support_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.support_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.heavy_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.heavy_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.st_light_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.st_light_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.st_heavy_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.st_heavy_fighters_online.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.st_support_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.st_support_fighters_online.iter() {
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
        for (&charge_key, &cont_key) in self.charge_group.iter() {
            check_item_key(uad, cont_key, true)?;
            check_item_key(uad, charge_key, true)?;
        }
        for (&charge_key, &cont_key) in self.charge_cont_group.iter() {
            check_item_key(uad, cont_key, true)?;
            check_item_key(uad, charge_key, true)?;
        }
        for (&charge_key, &cont_key) in self.charge_size.iter() {
            check_item_key(uad, cont_key, true)?;
            check_item_key(uad, charge_key, true)?;
        }
        for (&charge_key, &cont_key) in self.charge_volume.iter() {
            check_item_key(uad, cont_key, true)?;
            check_item_key(uad, charge_key, true)?;
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
        for (stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_key(uad, stopped_espec.item_key, false)?;
            for stopper_espec in stopper_especs {
                check_item_key(uad, stopper_espec.item_key, true)?;
            }
        }
        for (projectee_key, projector_especs) in self.blockable_assistance.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_key(uad, *projectee_key, false)?;
            for projector_espec in projector_especs {
                check_item_key(uad, projector_espec.item_key, true)?;
                check_a_effect_id(uad, &projector_espec.a_effect_id)?;
            }
        }
        for (projectee_key, projector_especs) in self.blockable_offense.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_key(uad, *projectee_key, false)?;
            for projector_espec in projector_especs {
                check_item_key(uad, projector_espec.item_key, true)?;
                check_a_effect_id(uad, &projector_espec.a_effect_id)?;
            }
        }
        for (projectee_aspec, projector_especs) in self.resist_immunity.iter() {
            // There is no logic which ensures that projection target is loaded
            check_item_key(uad, projectee_aspec.item_key, false)?;
            for projector_espec in projector_especs {
                check_item_key(uad, projector_espec.item_key, true)?;
                check_a_effect_id(uad, &projector_espec.a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.lr_shield.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.lr_shield_limitable.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.lr_armor.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.lr_armor_limitable.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.lr_hull.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.orr_shield.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.orr_armor.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.orr_hull.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for (&item_key, item_data) in self.orr_cap.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in item_data.keys() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        Ok(())
    }
}
