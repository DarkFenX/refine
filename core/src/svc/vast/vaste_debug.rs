use super::{Vast, VastFitData};
use crate::{dbg::DebugResult, ud::UData};

impl Vast {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (fit_uid, fit_data) in self.fit_datas.iter() {
            fit_uid.consistency_check(u_data)?;
            fit_data.consistency_check(u_data)?;
        }
        for item_uid in self.not_loaded.iter() {
            item_uid.consistency_check(u_data, false)?;
        }
        for (projectee_uid, projector_data) in self.irr_shield.iter() {
            // Projectees are not guaranteed to be loaded
            projectee_uid.consistency_check(u_data, false)?;
            for (projector_uid, projector_data) in projector_data.iter() {
                projector_uid.consistency_check(u_data, true)?;
                for (effect_rid, ospec) in projector_data.iter() {
                    effect_rid.consistency_check(u_data)?;
                    ospec.consistency_check(u_data)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.irr_shield_limitable.iter() {
            // Projectees are not guaranteed to be loaded
            projectee_uid.consistency_check(u_data, false)?;
            for (projector_uid, projector_data) in projector_data.iter() {
                projector_uid.consistency_check(u_data, true)?;
                for (effect_rid, ospec) in projector_data.iter() {
                    effect_rid.consistency_check(u_data)?;
                    ospec.consistency_check(u_data)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.irr_armor.iter() {
            // Projectees are not guaranteed to be loaded
            projectee_uid.consistency_check(u_data, false)?;
            for (projector_uid, projector_data) in projector_data.iter() {
                projector_uid.consistency_check(u_data, true)?;
                for (effect_rid, ospec) in projector_data.iter() {
                    effect_rid.consistency_check(u_data)?;
                    ospec.consistency_check(u_data)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.irr_armor_limitable.iter() {
            // Projectees are not guaranteed to be loaded
            projectee_uid.consistency_check(u_data, false)?;
            for (projector_uid, projector_data) in projector_data.iter() {
                projector_uid.consistency_check(u_data, true)?;
                for (effect_rid, ospec) in projector_data.iter() {
                    effect_rid.consistency_check(u_data)?;
                    ospec.consistency_check(u_data)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.irr_hull.iter() {
            // Projectees are not guaranteed to be loaded
            projectee_uid.consistency_check(u_data, false)?;
            for (projector_uid, projector_data) in projector_data.iter() {
                projector_uid.consistency_check(u_data, true)?;
                for (effect_rid, ospec) in projector_data.iter() {
                    effect_rid.consistency_check(u_data)?;
                    ospec.consistency_check(u_data)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.in_cap.iter() {
            // Projectees are not guaranteed to be loaded
            projectee_uid.consistency_check(u_data, false)?;
            for (projector_uid, projector_data) in projector_data.iter() {
                projector_uid.consistency_check(u_data, true)?;
                for (effect_rid, ospec) in projector_data.iter() {
                    effect_rid.consistency_check(u_data)?;
                    ospec.consistency_check(u_data)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.in_neuts.iter() {
            // Projectees are not guaranteed to be loaded
            projectee_uid.consistency_check(u_data, false)?;
            for (projector_uid, projector_data) in projector_data.iter() {
                projector_uid.consistency_check(u_data, true)?;
                for (effect_rid, ospec) in projector_data.iter() {
                    effect_rid.consistency_check(u_data)?;
                    ospec.consistency_check(u_data)?;
                }
            }
        }
        for (projectee_uid, projector_data) in self.in_ecm.iter() {
            // Projectees are not guaranteed to be loaded
            projectee_uid.consistency_check(u_data, false)?;
            for (projector_uid, projector_data) in projector_data.iter() {
                projector_uid.consistency_check(u_data, true)?;
                for (effect_rid, ospec) in projector_data.iter() {
                    effect_rid.consistency_check(u_data)?;
                    ospec.consistency_check(u_data)?;
                }
            }
        }
        Ok(())
    }
}

impl VastFitData {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for item_uid in self.mods_svcs_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.rigs_offline_calibration.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.drones_volume.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.drones_bandwidth.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.drones_online_bandwidth.keys() {
            // Holds not loaded drones as well
            item_uid.consistency_check(u_data, false)?;
        }
        for item_uid in self.fighters_volume.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.fighters_online.iter() {
            // Holds not loaded fighters as well
            item_uid.consistency_check(u_data, false)?;
        }
        for item_uid in self.light_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.light_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.support_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.support_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.heavy_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.heavy_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.st_light_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.st_light_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.st_heavy_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.st_heavy_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.st_support_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.st_support_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.mods_turret.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.mods_launcher.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uids in self.slotted_implants.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uids in self.slotted_boosters.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uids in self.slotted_subsystems.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in self.ship_limited_items.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uids in self.mods_svcs_rigs_max_group_fitted_all.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in self.mods_svcs_rigs_max_group_fitted_limited.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uids in self.mods_svcs_max_group_online_all.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in self.mods_svcs_max_group_online_limited.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uids in self.mods_max_group_active_all.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in self.mods_max_group_active_limited.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.rigs_rig_size.keys() {
            // This container can store info about non-loaded rigs
            item_uid.consistency_check(u_data, false)?;
        }
        for item_uids in self.srqs_skill_item_map.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in self.srqs_missing.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for (charge_uid, cont_uid) in self.charge_group.iter() {
            cont_uid.consistency_check(u_data, true)?;
            charge_uid.consistency_check(u_data, true)?;
        }
        for (charge_uid, cont_uid) in self.charge_cont_group.iter() {
            cont_uid.consistency_check(u_data, true)?;
            charge_uid.consistency_check(u_data, true)?;
        }
        for (charge_uid, cont_uid) in self.charge_size.iter() {
            cont_uid.consistency_check(u_data, true)?;
            charge_uid.consistency_check(u_data, true)?;
        }
        for (charge_uid, cont_uid) in self.charge_volume.iter() {
            cont_uid.consistency_check(u_data, true)?;
            charge_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.mods_capital.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.not_loaded.iter() {
            item_uid.consistency_check(u_data, false)?;
        }
        for item_uid in self.mods_state.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.item_kind.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.drone_groups.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.fighter_squad_size.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.overload_td_lvl.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_data in self.mods_svcs_max_type_fitted.values() {
            for item_uid in item_data.keys() {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in self.sec_zone_fitted.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.sec_zone_fitted_wspace_banned.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.sec_zone_online_class.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.sec_zone_active.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.sec_zone_unonlineable_class.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.sec_zone_unactivable.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for (item_uid, item_data) in self.sec_zone_effect.iter() {
            item_uid.consistency_check(u_data, true)?;
            for effect_rid in item_data.keys() {
                effect_rid.consistency_check(u_data)?;
            }
        }
        for item_uid in self.mods_active.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in self.mods_rigs_svcs_vs_ship_kind.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for (stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            // There is no logic which ensures that projection target is loaded
            stopped_espec.consistency_check(u_data, false)?;
            for stopper_espec in stopper_especs {
                stopper_espec.consistency_check(u_data, true)?;
            }
        }
        for (projector_espec, projectee_uids) in self.projectee_filter.iter() {
            projector_espec.consistency_check(u_data, true)?;
            for projectee_uid in projectee_uids.keys() {
                // Target is not guaranteed to be loaded
                projectee_uid.consistency_check(u_data, false)?;
            }
        }
        for (projectee_uid, projector_especs) in self.blockable_assistance.iter() {
            // There is no logic which ensures that projection target is loaded
            projectee_uid.consistency_check(u_data, false)?;
            for projector_espec in projector_especs {
                projector_espec.consistency_check(u_data, true)?;
            }
        }
        for (projectee_uid, projector_especs) in self.blockable_offense.iter() {
            // There is no logic which ensures that projection target is loaded
            projectee_uid.consistency_check(u_data, false)?;
            for projector_espec in projector_especs {
                projector_espec.consistency_check(u_data, true)?;
            }
        }
        for (projectee_aspec, projector_especs) in self.resist_immunity.iter() {
            // There is no logic which ensures that projection target is loaded
            projectee_aspec.consistency_check(u_data, false)?;
            for projector_espec in projector_especs {
                projector_espec.consistency_check(u_data, true)?;
            }
        }
        for (item_uid, attr_rids) in self.cap_consumers_all.iter() {
            item_uid.consistency_check(u_data, true)?;
            for attr_rid in attr_rids {
                attr_rid.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - damage output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in self.dmg_normal.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.dmg_breacher.iter() {
            item_uid.consistency_check(u_data, true)?;
            for effect_rid in item_data.keys() {
                effect_rid.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - mining output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in self.mining_ore.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.mining_ice.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.mining_gas.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - RR output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in self.orr_shield.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.orr_armor.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.orr_hull.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - misc output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in self.out_neuts.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.out_cap.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - local active tank
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in self.lr_shield.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.lr_shield_limitable.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.lr_armor.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.lr_armor_limitable.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.lr_hull.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - cap
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in self.cap_consumers_active.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, attr_rid) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                attr_rid.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in self.cap_injects.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - misc
        ////////////////////////////////////////////////////////////////////////////////////////////
        for espec in self.aggro_effects.iter() {
            espec.consistency_check(u_data, true)?;
        }
        Ok(())
    }
}
