use super::{Vast, VastFitData};
use crate::{
    Count,
    dbg::DebugResult,
    ud::{UData, UFitId},
};

impl Vast {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            fit_datas,
            not_loaded,
            irr_shield,
            irr_shield_limitable,
            irr_armor,
            irr_armor_limitable,
            irr_hull,
            in_cap,
            in_neuts,
            in_ecm,
        } = self;
        for (fit_uid, fit_data) in fit_datas.iter() {
            fit_uid.consistency_check(u_data)?;
            fit_data.consistency_check(u_data, fit_uid)?;
        }
        for item_uid in not_loaded.iter() {
            item_uid.consistency_check(u_data, false)?;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - incoming remote reps
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (projectee_uid, projector_data) in irr_shield.iter() {
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
        for (projectee_uid, projector_data) in irr_shield_limitable.iter() {
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
        for (projectee_uid, projector_data) in irr_armor.iter() {
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
        for (projectee_uid, projector_data) in irr_armor_limitable.iter() {
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
        for (projectee_uid, projector_data) in irr_hull.iter() {
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
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - cap
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (projectee_uid, projector_data) in in_cap.iter() {
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
        for (projectee_uid, projector_data) in in_neuts.iter() {
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
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - ewar
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (projectee_uid, projector_data) in in_ecm.iter() {
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
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData, fit_uid: UFitId) -> DebugResult {
        let Self {
            mods_svcs_online,
            rigs_offline_calibration,
            drones_volume,
            drones_bandwidth,
            drones_online_bandwidth,
            fighters_volume,
            mods_turret,
            mods_launcher,
            fighters_online,
            light_fighters,
            light_fighters_online,
            heavy_fighters,
            heavy_fighters_online,
            support_fighters,
            support_fighters_online,
            st_light_fighters,
            st_light_fighters_online,
            st_heavy_fighters,
            st_heavy_fighters_online,
            st_support_fighters,
            st_support_fighters_online,
            slotted_implants,
            slotted_boosters,
            slotted_subsystems,
            ship_limited_items,
            ship_limit_mismatches,
            rigs_rig_size,
            rig_size_mismatches,
            mods_rigs_svcs_vs_ship_kind,
            mods_capital,
            drone_groups,
            mods_svcs_max_type_fitted,
            mods_svcs_rigs_max_group_fitted_all,
            mods_svcs_rigs_max_group_fitted_limited,
            mods_svcs_max_group_online_all,
            mods_svcs_max_group_online_limited,
            mods_max_group_active_all,
            mods_max_group_active_limited,
            charge_group,
            charge_cont_group,
            charge_size,
            charge_volume,
            projectee_filter,
            blockable_assistance,
            blockable_offense,
            resist_immunity,
            stopped_effects,
            srqs_skill_item_map,
            srqs_missing,
            overload_td_lvl,
            sec_zone_fitted,
            sec_zone_fitted_wspace_banned,
            sec_zone_online_class,
            sec_zone_active,
            sec_zone_unonlineable_class,
            sec_zone_unactivable,
            sec_zone_effect,
            not_loaded,
            item_kind,
            mods_state,
            mods_active,
            mods_active_cloaks,
            mods_active_block_cloak,
            mods_active_block_in_assist,
            mods_cap_consumers,
            fighter_squad_size,
            mods_fitted_cloaks,
            dmg_normal,
            dmg_breacher,
            mining_ore,
            mining_ice,
            mining_gas,
            orr_shield,
            orr_armor,
            orr_hull,
            out_neuts,
            out_cap,
            lr_shield,
            lr_shield_limitable,
            lr_armor,
            lr_armor_limitable,
            lr_hull,
            cap_consumers,
            cap_nosfs,
            cap_injects,
            effects_weapons_timer,
            mod_effects_disallow_cloak,
            mod_effects_disallow_warp,
            mod_effects_disallow_jump_gate,
            mod_effects_disallow_jump_wh,
            mod_effects_disallow_jump_drive,
            mod_effects_disallow_dock,
            mod_effects_disallow_tether,
            conduit_enablers,
            portal_enablers,
            // Holds item group IDs only
            drone_group_limit: _,
        } = self;
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - resources
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_uid in mods_svcs_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for (item_uid, val) in rigs_offline_calibration.iter() {
            item_uid.consistency_check(u_data, true)?;
            val.consistency_check()?;
        }
        for (item_uid, val) in drones_volume.iter() {
            item_uid.consistency_check(u_data, true)?;
            val.consistency_check()?;
        }
        for (item_uid, val) in drones_bandwidth.iter() {
            item_uid.consistency_check(u_data, true)?;
            val.consistency_check()?;
        }
        for (item_uid, val) in drones_online_bandwidth.iter() {
            // Holds not loaded drones as well
            item_uid.consistency_check(u_data, false)?;
            val.consistency_check()?;
        }
        for (item_uid, val) in fighters_volume.iter() {
            item_uid.consistency_check(u_data, true)?;
            val.consistency_check()?;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - slots
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_uid in mods_turret.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in mods_launcher.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in fighters_online.iter() {
            // Holds not loaded fighters as well
            item_uid.consistency_check(u_data, false)?;
        }
        for item_uid in light_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in light_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in support_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in support_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in heavy_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in heavy_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in st_light_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in st_light_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in st_heavy_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in st_heavy_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in st_support_fighters.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in st_support_fighters_online.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - slot index
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_uids in slotted_implants.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uids in slotted_boosters.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uids in slotted_subsystems.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - restrictions between ship type/attribute and its items
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_uid in ship_limited_items.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in ship_limit_mismatches.iter() {
            item_uid.consistency_check(u_data, true)?;
            if !ship_limited_items.contains(item_uid) {
                return Err(Default::default());
            }
        }
        for (item_uid, val) in rigs_rig_size.iter() {
            // This container can store info about non-loaded rigs
            item_uid.consistency_check(u_data, false)?;
            if let Some(val) = val {
                val.consistency_check()?;
            }
        }
        for item_uid in rig_size_mismatches.iter() {
            item_uid.consistency_check(u_data, true)?;
            if !rigs_rig_size.contains_key(item_uid) {
                return Err(Default::default());
            }
        }
        for item_uid in mods_rigs_svcs_vs_ship_kind.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for (item_uid, val) in mods_capital.iter() {
            item_uid.consistency_check(u_data, true)?;
            val.consistency_check()?;
        }
        for item_uid in drone_groups.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - max type/group
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_data in mods_svcs_max_type_fitted.values() {
            for item_uid in item_data.keys() {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uids in mods_svcs_rigs_max_group_fitted_all.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in mods_svcs_rigs_max_group_fitted_limited.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uids in mods_svcs_max_group_online_all.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in mods_svcs_max_group_online_limited.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uids in mods_max_group_active_all.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in mods_max_group_active_limited.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - module-charge restrictions
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (charge_uid, cont_uid) in charge_group.iter() {
            cont_uid.consistency_check(u_data, true)?;
            charge_uid.consistency_check(u_data, true)?;
        }
        for (charge_uid, cont_uid) in charge_cont_group.iter() {
            cont_uid.consistency_check(u_data, true)?;
            charge_uid.consistency_check(u_data, true)?;
        }
        for (charge_uid, cont_uid) in charge_size.iter() {
            cont_uid.consistency_check(u_data, true)?;
            charge_uid.consistency_check(u_data, true)?;
        }
        for (charge_uid, cont_uid) in charge_volume.iter() {
            cont_uid.consistency_check(u_data, true)?;
            charge_uid.consistency_check(u_data, true)?;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - projection
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (projector_espec, projectee_uids) in projectee_filter.iter() {
            projector_espec.consistency_check(u_data, true)?;
            for projectee_uid in projectee_uids.keys() {
                // Target is not guaranteed to be loaded
                projectee_uid.consistency_check(u_data, false)?;
            }
        }
        for (projectee_uid, projector_especs) in blockable_assistance.iter() {
            // There is no logic which ensures that projection target is loaded
            projectee_uid.consistency_check(u_data, false)?;
            for projector_espec in projector_especs {
                projector_espec.consistency_check(u_data, true)?;
            }
        }
        for (projectee_uid, projector_especs) in blockable_offense.iter() {
            // There is no logic which ensures that projection target is loaded
            projectee_uid.consistency_check(u_data, false)?;
            for projector_espec in projector_especs {
                projector_espec.consistency_check(u_data, true)?;
            }
        }
        for (projectee_aspec, projector_especs) in resist_immunity.iter() {
            // There is no logic which ensures that projection target is loaded
            projectee_aspec.consistency_check(u_data, false)?;
            for projector_espec in projector_especs {
                projector_espec.consistency_check(u_data, true)?;
            }
        }
        for (stopped_espec, stopper_especs) in stopped_effects.iter() {
            // There is no logic which ensures that projection target is loaded
            stopped_espec.consistency_check(u_data, false)?;
            for stopper_espec in stopper_especs {
                stopper_espec.consistency_check(u_data, true)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - skills
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_uids in srqs_skill_item_map.values() {
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for item_uid in srqs_missing.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in overload_td_lvl.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - security zone
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_uid in sec_zone_fitted.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in sec_zone_fitted_wspace_banned.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for (item_uid, val) in sec_zone_online_class.iter() {
            item_uid.consistency_check(u_data, true)?;
            val.consistency_check()?;
        }
        for item_uid in sec_zone_active.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for (item_uid, val) in sec_zone_unonlineable_class.iter() {
            item_uid.consistency_check(u_data, true)?;
            val.consistency_check()?;
        }
        for item_uid in sec_zone_unactivable.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for (item_uid, item_data) in sec_zone_effect.iter() {
            item_uid.consistency_check(u_data, true)?;
            for effect_rid in item_data.keys() {
                effect_rid.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Validation-related - misc
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_uid in not_loaded.iter() {
            item_uid.consistency_check(u_data, false)?;
        }
        for item_uid in item_kind.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in mods_state.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in mods_active.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for espec in mods_active_cloaks.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mods_active_block_cloak.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mods_active_block_in_assist.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for item_uid in mods_cap_consumers.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in fighter_squad_size.keys() {
            item_uid.consistency_check(u_data, true)?;
        }
        // Check cloak count
        let mut cloak_count = Count::ZERO;
        for module_uid in u_data.fits.get(fit_uid).iter_module_uids() {
            if let Some(module_rib) = u_data.items.get(module_uid).get_r_item_base()
                && module_rib.is_cloak
            {
                cloak_count += Count::ONE;
            }
        }
        if *mods_fitted_cloaks != cloak_count {
            return Err(Default::default());
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - damage output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in dmg_normal.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in dmg_breacher.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - mining output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in mining_ore.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in mining_ice.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in mining_gas.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - RR output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in orr_shield.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in orr_armor.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in orr_hull.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - misc output
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in out_neuts.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in out_cap.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - local active tank
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in lr_shield.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in lr_shield_limitable.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in lr_armor.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in lr_armor_limitable.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in lr_hull.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - cap
        ////////////////////////////////////////////////////////////////////////////////////////////
        for (item_uid, item_data) in cap_consumers.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in cap_nosfs.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        for (item_uid, item_data) in cap_injects.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (effect_rid, ospec) in item_data.iter() {
                effect_rid.consistency_check(u_data)?;
                ospec.consistency_check(u_data)?;
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - effect-based restrictions
        ////////////////////////////////////////////////////////////////////////////////////////////
        for espec in effects_weapons_timer.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mod_effects_disallow_cloak.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mod_effects_disallow_warp.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mod_effects_disallow_jump_gate.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mod_effects_disallow_jump_wh.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mod_effects_disallow_jump_drive.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mod_effects_disallow_dock.iter() {
            espec.consistency_check(u_data, true)?;
        }
        for espec in mod_effects_disallow_tether.iter() {
            espec.consistency_check(u_data, true)?;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Stats-related - misc
        ////////////////////////////////////////////////////////////////////////////////////////////
        for item_uid in conduit_enablers.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        for item_uid in portal_enablers.iter() {
            item_uid.consistency_check(u_data, true)?;
        }
        Ok(())
    }
}
