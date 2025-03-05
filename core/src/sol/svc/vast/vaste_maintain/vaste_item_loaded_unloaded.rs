use std::collections::hash_map::Entry;

use crate::{
    ad,
    defs::{OF, SolItemId},
    ec,
    sol::{
        SolModRack,
        svc::vast::{SolDroneGroupMismatch, SolItemKindValFail, SolValCache, SolVast, SolVastFitData, SolVastSkillReq},
        uad::{
            SolUad,
            item::{SolItem, SolModule},
        },
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
            SolItem::Booster(booster) => {
                let extras = booster.get_a_extras().unwrap();
                if let Some(slot) = booster.get_slot() {
                    fit_data.slotted_boosters.add_entry(slot, item_id);
                }
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Booster);
            }
            SolItem::Character(character) => {
                let extras = character.get_a_extras().unwrap();
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Character);
            }
            SolItem::Charge(charge) => {
                let extras = charge.get_a_extras().unwrap();
                // Reset result to uncalculated when adding a charge
                if let Entry::Occupied(mut entry) = fit_data.mods_charge_group.entry(charge.get_cont_id()) {
                    entry.insert(SolValCache::Todo(()));
                }
                // Reset result to uncalculated when adding a charge
                if let Entry::Occupied(mut entry) = fit_data.mods_charge_size.entry(charge.get_cont_id()) {
                    match entry.get() {
                        SolValCache::Pass(allowed_charge_size) => {
                            entry.insert(SolValCache::Todo(*allowed_charge_size));
                        }
                        SolValCache::Fail(fail) => {
                            entry.insert(SolValCache::Todo(fail.allowed_size));
                        }
                        _ => (),
                    }
                }
                // Add entry for charges with volume higher than 0
                if let Some(&charge_volume) = charge.get_attrs().unwrap().get(&ec::attrs::VOLUME) {
                    if charge_volume > OF(0.0) {
                        fit_data
                            .mods_charge_volume
                            .insert(charge.get_cont_id(), SolValCache::Todo(charge_volume));
                    }
                }
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Charge);
            }
            SolItem::Drone(drone) => {
                let extras = drone.get_a_extras().unwrap();
                if let Some(val) = extras.volume {
                    fit_data.drones_volume.insert(item_id, val);
                }
                if !fit_data.drone_group_limit.is_empty() {
                    let drone_group_id = drone.get_group_id().unwrap();
                    if !fit_data.drone_group_limit.contains(&drone_group_id) {
                        fit_data
                            .drone_group_mismatches
                            .insert(item_id, SolDroneGroupMismatch::new(item_id, drone_group_id));
                    }
                }
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Drone);
            }
            SolItem::Fighter(fighter) => {
                let extras = fighter.get_a_extras().unwrap();
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Fighter);
            }
            SolItem::Implant(implant) => {
                let extras = implant.get_a_extras().unwrap();
                if let Some(slot) = implant.get_slot() {
                    fit_data.slotted_implants.add_entry(slot, item_id);
                }
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Implant);
            }
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
                        Some(_) => fit_data.mods_charge_group.insert(item_id, SolValCache::Todo(())),
                        None => fit_data.mods_charge_group.insert(item_id, SolValCache::Pass(())),
                    };
                }
                if let Some(allowed_charge_size) = module.get_attrs().unwrap().get(&ec::attrs::CHARGE_SIZE) {
                    // If there is a charge, calculate later, otherwise mark as no issue
                    match module.get_charge_id() {
                        Some(_) => fit_data
                            .mods_charge_size
                            .insert(item_id, SolValCache::Todo(*allowed_charge_size)),
                        None => fit_data
                            .mods_charge_size
                            .insert(item_id, SolValCache::Pass(*allowed_charge_size)),
                    };
                }
                // Data is added to / removed from this map when charges are added/removed; here,
                // we just reset validation result when a module is being loaded
                handle_charge_volume_for_module(fit_data, item_id);
                if let Some(ad::AShipKind::CapitalShip) = extras.item_ship_kind {
                    fit_data.mods_capital.insert(item_id);
                }
                item_kind_add(fit_data, item_id, extras.kind, get_module_expected_kind(module));
            }
            SolItem::Rig(rig) => {
                let extras = rig.get_a_extras().unwrap();
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_mods_rigs_subs.insert(item_id, ship_limit.clone());
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    fit_data.mods_rigs_max_group_fitted_all.add_entry(grp_id, item_id);
                    if rig.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_FITTED) {
                        fit_data.mods_rigs_max_group_fitted_limited.insert(item_id, grp_id);
                    }
                }
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Rig);
            }
            SolItem::Ship(ship) => {
                let extras = ship.get_a_extras().unwrap();
                // If new ship limits drones which can be used, fill the mismatch data up
                if let Some(drone_limit) = &extras.drone_limit {
                    fit_data.drone_group_limit.extend(drone_limit.group_ids.iter());
                    let fit = uad.fits.get_fit(&fit_id).unwrap();
                    for drone_item_id in fit.drones.iter() {
                        let drone_item = uad.items.get_item(drone_item_id).unwrap();
                        if let Some(drone_group_id) = drone_item.get_group_id() {
                            if !drone_limit.group_ids.contains(&drone_group_id) {
                                fit_data.drone_group_mismatches.insert(
                                    *drone_item_id,
                                    SolDroneGroupMismatch::new(*drone_item_id, drone_group_id),
                                );
                            }
                        }
                    }
                }
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Ship);
            }
            SolItem::Skill(skill) => {
                let extras = skill.get_a_extras().unwrap();
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Skill);
            }
            SolItem::Stance(stance) => {
                let extras = stance.get_a_extras().unwrap();
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Stance);
            }
            SolItem::Subsystem(subsystem) => {
                let extras = subsystem.get_a_extras().unwrap();
                if let Some(slot) = subsystem.get_slot() {
                    fit_data.slotted_subsystems.add_entry(slot, item_id);
                }
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_mods_rigs_subs.insert(item_id, ship_limit.clone());
                }
                item_kind_add(fit_data, item_id, extras.kind, ad::AItemKind::Subsystem);
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
            SolItem::Booster(booster) => {
                let extras = booster.get_a_extras().unwrap();
                if let Some(slot) = booster.get_slot() {
                    fit_data.slotted_boosters.remove_entry(&slot, &item_id);
                }
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Booster);
            }
            SolItem::Character(character) => {
                let extras = character.get_a_extras().unwrap();
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Character);
            }
            SolItem::Charge(charge) => {
                let extras = charge.get_a_extras().unwrap();
                if let Entry::Occupied(mut entry) = fit_data.mods_charge_group.entry(charge.get_cont_id()) {
                    // No charge - check should pass
                    entry.insert(SolValCache::Pass(()));
                }
                if let Entry::Occupied(mut entry) = fit_data.mods_charge_size.entry(charge.get_cont_id()) {
                    // No charge - check should pass
                    match entry.get() {
                        SolValCache::Todo(allowed_charge_size) => {
                            entry.insert(SolValCache::Pass(*allowed_charge_size));
                        }
                        SolValCache::Fail(fail) => {
                            entry.insert(SolValCache::Pass(fail.allowed_size));
                        }
                        _ => (),
                    }
                }
                fit_data.mods_charge_volume.remove(&charge.get_cont_id());
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Charge);
            }
            SolItem::Drone(drone) => {
                let extras = drone.get_a_extras().unwrap();
                fit_data.drones_volume.remove(&item_id);
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_group_mismatches.remove(&item_id);
                }
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Drone);
            }
            SolItem::Fighter(fighter) => {
                let extras = fighter.get_a_extras().unwrap();
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Fighter);
            }
            SolItem::Implant(implant) => {
                let extras = implant.get_a_extras().unwrap();
                if let Some(slot) = implant.get_slot() {
                    fit_data.slotted_implants.remove_entry(&slot, &item_id);
                }
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Implant);
            }
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
                    fit_data.mods_charge_group.remove(&item_id);
                }
                fit_data.mods_charge_size.remove(&item_id);
                // Data is added to / removed from this map when charges are added/removed; here,
                // we just reset validation result when a module is being unloaded
                handle_charge_volume_for_module(fit_data, item_id);
                if let Some(ad::AShipKind::CapitalShip) = extras.item_ship_kind {
                    fit_data.mods_capital.remove(&item_id);
                }
                item_kind_remove(fit_data, &item_id, extras.kind, get_module_expected_kind(module));
            }
            SolItem::Rig(rig) => {
                let extras = rig.get_a_extras().unwrap();
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_mods_rigs_subs.remove(&item_id);
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    fit_data.mods_rigs_max_group_fitted_all.remove_entry(&grp_id, &item_id);
                    fit_data.mods_rigs_max_group_fitted_limited.remove(&item_id);
                }
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Rig);
            }
            SolItem::Ship(ship) => {
                let extras = ship.get_a_extras().unwrap();
                // If any drone group limits were defined, clear the mismatch data
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_group_limit.clear();
                    fit_data.drone_group_mismatches.clear();
                }
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Ship);
            }
            SolItem::Skill(skill) => {
                let extras = skill.get_a_extras().unwrap();
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Skill);
            }
            SolItem::Stance(stance) => {
                let extras = stance.get_a_extras().unwrap();
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Stance);
            }
            SolItem::Subsystem(subsystem) => {
                let extras = subsystem.get_a_extras().unwrap();
                if let Some(slot) = subsystem.get_slot() {
                    fit_data.slotted_subsystems.remove_entry(&slot, &item_id);
                }
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_mods_rigs_subs.remove(&item_id);
                }
                item_kind_remove(fit_data, &item_id, extras.kind, ad::AItemKind::Subsystem);
            }
            _ => (),
        }
    }
}

fn handle_charge_volume_for_module(fit_data: &mut SolVastFitData, module_item_id: SolItemId) {
    if let Entry::Occupied(mut entry) = fit_data.mods_charge_volume.entry(module_item_id) {
        match entry.get() {
            SolValCache::Pass(charge_volume) => {
                entry.insert(SolValCache::Todo(*charge_volume));
            }
            SolValCache::Fail(fail) => {
                entry.insert(SolValCache::Todo(fail.charge_volume));
            }
            _ => (),
        }
    }
}

fn get_module_expected_kind(module: &SolModule) -> ad::AItemKind {
    match module.get_rack() {
        SolModRack::High => ad::AItemKind::ModuleHigh,
        SolModRack::Mid => ad::AItemKind::ModuleMid,
        SolModRack::Low => ad::AItemKind::ModuleLow,
    }
}
fn item_kind_add(
    fit_data: &mut SolVastFitData,
    item_id: SolItemId,
    item_kind: Option<ad::AItemKind>,
    expected_kind: ad::AItemKind,
) {
    if item_kind != Some(expected_kind) {
        fit_data
            .item_kind
            .insert(item_id, SolItemKindValFail::new(item_id, item_kind, expected_kind));
    }
}
fn item_kind_remove(
    fit_data: &mut SolVastFitData,
    item_id: &SolItemId,
    item_kind: Option<ad::AItemKind>,
    expected_kind: ad::AItemKind,
) {
    if item_kind != Some(expected_kind) {
        fit_data.item_kind.remove(item_id);
    }
}
