use std::collections::hash_map::Entry;

use crate::{
    ec,
    sol::{
        svc::vast::{SolVast, SolVastSkillReq},
        uad::{item::SolItem, SolUad},
    },
    util::StMap,
};

impl SolVast {
    pub(in crate::sol::svc) fn item_loaded(&mut self, uad: &SolUad, item: &SolItem) {
        let item_id = item.get_id();
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_id).unwrap();
        // Skill requirements
        if let Some(skill_reqs) = item.get_effective_skill_reqs() {
            if !skill_reqs.is_empty() {
                let mut missing_skills = StMap::new();
                let fit = uad.fits.get_fit(&fit_id).unwrap();
                for (&skill_type_id, &required_level) in skill_reqs.iter() {
                    fit_data.srqs_skill_item_map.add_entry(skill_type_id, item_id);
                    let current_level = fit.skills.get(&skill_type_id).map(|v| v.level);
                    if current_level.unwrap_or(0) < required_level {
                        missing_skills.insert(skill_type_id, SolVastSkillReq::new(current_level, required_level));
                    }
                }
                fit_data.srqs_missing.insert(item_id, missing_skills);
            }
        }
        match item {
            SolItem::Module(module) => {
                let extras = module.get_a_extras().unwrap();
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_mods_rigs_subs.insert(item_id, ship_limit.clone());
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    fit_data.mods_rigs_max_group_fitted_all.add_entry(grp_id, item_id);
                    if module.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_FITTED) {
                        fit_data.mods_rigs_max_group_fitted_limited.insert(item_id, grp_id);
                    }
                }
                if extras.charge_limit.is_some() {
                    // If there is a charge, calculate later, otherwise mark as no issue
                    match module.get_charge_id() {
                        Some(_) => fit_data.charge_group.insert(item_id, None),
                        None => fit_data.charge_group.insert(item_id, Some(None)),
                    };
                }
            }
            SolItem::Rig(rig) => {
                let extras = rig.get_a_extras().unwrap();
                if let Some(rig_size) = item.get_attrs().unwrap().get(&ec::attrs::RIG_SIZE) {
                    fit_data.rigs_rig_size.insert(item_id, *rig_size);
                }
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_mods_rigs_subs.insert(item_id, ship_limit.clone());
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    fit_data.mods_rigs_max_group_fitted_all.add_entry(grp_id, item_id);
                    if rig.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_FITTED) {
                        fit_data.mods_rigs_max_group_fitted_limited.insert(item_id, grp_id);
                    }
                }
            }
            SolItem::Drone(drone) => {
                if let Some(val) = drone.get_a_extras().unwrap().volume {
                    fit_data.drones_volume.insert(item_id, val);
                }
            }
            SolItem::Implant(implant) => {
                if let Some(slot) = implant.get_slot() {
                    fit_data.slotted_implants.add_entry(slot, item_id);
                }
            }
            SolItem::Booster(booster) => {
                if let Some(slot) = booster.get_slot() {
                    fit_data.slotted_boosters.add_entry(slot, item_id);
                }
            }
            SolItem::Subsystem(subsystem) => {
                if let Some(slot) = subsystem.get_slot() {
                    fit_data.slotted_subsystems.add_entry(slot, item_id);
                }
                if let Some(ship_limit) = &subsystem.get_a_extras().unwrap().ship_limit {
                    fit_data.ship_limited_mods_rigs_subs.insert(item_id, ship_limit.clone());
                }
            }
            SolItem::Charge(charge) => {
                // Reset result to uncalculated when adding a charge
                if let Entry::Occupied(mut entry) = fit_data.charge_group.entry(charge.get_cont_id()) {
                    entry.insert(None);
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn item_unloaded(&mut self, item: &SolItem) {
        let item_id = item.get_id();
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_id).unwrap();
        // Skill requirements
        if let Some(skill_reqs) = item.get_effective_skill_reqs() {
            if !skill_reqs.is_empty() {
                for skill_type_id in skill_reqs.keys() {
                    fit_data.srqs_skill_item_map.remove_entry(skill_type_id, &item_id);
                }
                fit_data.srqs_missing.remove(&item_id);
            }
        }
        match item {
            SolItem::Module(module) => {
                let extras = module.get_a_extras().unwrap();
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_mods_rigs_subs.remove(&item_id);
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    fit_data.mods_rigs_max_group_fitted_all.remove_entry(&grp_id, &item_id);
                    fit_data.mods_rigs_max_group_fitted_limited.remove(&item_id);
                }
                if extras.charge_limit.is_some() {
                    fit_data.charge_group.remove(&item_id);
                }
            }
            SolItem::Rig(rig) => {
                let extras = rig.get_a_extras().unwrap();
                fit_data.rigs_rig_size.remove(&item_id);
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_mods_rigs_subs.remove(&item_id);
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    fit_data.mods_rigs_max_group_fitted_all.remove_entry(&grp_id, &item_id);
                    fit_data.mods_rigs_max_group_fitted_limited.remove(&item_id);
                }
            }
            SolItem::Drone(_) => {
                fit_data.drones_volume.remove(&item_id);
            }
            SolItem::Implant(implant) => {
                if let Some(slot) = implant.get_slot() {
                    fit_data.slotted_implants.remove_entry(&slot, &item_id);
                }
            }
            SolItem::Booster(booster) => {
                if let Some(slot) = booster.get_slot() {
                    fit_data.slotted_boosters.remove_entry(&slot, &item_id);
                }
            }
            SolItem::Subsystem(subsystem) => {
                if let Some(slot) = subsystem.get_slot() {
                    fit_data.slotted_subsystems.remove_entry(&slot, &item_id);
                }
                if subsystem.get_a_extras().unwrap().ship_limit.is_some() {
                    fit_data.ship_limited_mods_rigs_subs.remove(&item_id);
                }
            }
            SolItem::Charge(charge) => {
                // No charge - check should pass
                if let Entry::Occupied(mut entry) = fit_data.charge_group.entry(charge.get_cont_id()) {
                    entry.insert(Some(None));
                }
            }
            _ => (),
        }
    }
}
