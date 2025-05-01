use std::collections::hash_map::Entry;

use itertools::chain;
use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        AttrVal, FitKey, ItemKey, ModRack,
        svc::vast::{
            ValCache, ValFighterSquadSizeFighterInfo, ValItemKindItemInfo, ValShipKind, ValSrqSkillInfo, Vast,
            VastFitData,
        },
        uad::{
            Uad,
            item::{ShipKind, UadItem, UadModule},
        },
    },
    util::RMap,
};

impl Vast {
    pub(in crate::sol::svc) fn item_loaded(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_key);
        // Skill requirements
        if let Some(a_srqs) = item.get_effective_a_skill_reqs() {
            if !a_srqs.is_empty() {
                let mut missing_skills = RMap::new();
                let fit = uad.fits.get(fit_key);
                for (&skill_a_item_id, &required_a_lvl) in a_srqs.iter() {
                    fit_data.srqs_skill_item_map.add_entry(skill_a_item_id, item_key);
                    let current_lvl = fit.skills.get(&skill_a_item_id).map(|v| v.level);
                    let effective_lvl = match current_lvl {
                        Some(current_lvl) => current_lvl.get_inner(),
                        None => 0,
                    };
                    if effective_lvl < required_a_lvl.get_inner() {
                        missing_skills.insert(
                            skill_a_item_id,
                            ValSrqSkillInfo {
                                current_lvl,
                                required_lvl: required_a_lvl.into(),
                            },
                        );
                    }
                }
                fit_data.srqs_missing.insert(item_key, missing_skills);
            }
        }
        match item {
            UadItem::Booster(booster) => {
                let extras = booster.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Booster);
                if let Some(a_slot) = booster.get_a_slot() {
                    fit_data.slotted_boosters.add_entry(a_slot, item_key);
                }
            }
            UadItem::Character(character) => {
                let extras = character.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Character);
            }
            UadItem::Charge(charge) => {
                let extras = charge.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Charge);
                // Reset result to uncalculated when adding a charge
                if let Entry::Occupied(mut entry) = fit_data.mods_charge_group.entry(charge.get_cont_item_key()) {
                    entry.insert(ValCache::Todo(()));
                }
                // Reset result to uncalculated when adding a charge
                if let Entry::Occupied(mut entry) = fit_data.mods_charge_size.entry(charge.get_cont_item_key()) {
                    match entry.get() {
                        ValCache::Pass(allowed_charge_size) => {
                            entry.insert(ValCache::Todo(*allowed_charge_size));
                        }
                        ValCache::Fail(fail_cache) => {
                            entry.insert(ValCache::Todo(fail_cache.allowed_size));
                        }
                        _ => (),
                    }
                }
                // Add entry for charges with volume higher than 0
                if let Some(volume) = extras.volume {
                    if volume > OF(0.0) {
                        fit_data
                            .mods_charge_volume
                            .insert(charge.get_cont_item_key(), ValCache::Todo(volume));
                    }
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_key);
                }
            }
            UadItem::Drone(drone) => {
                let extras = drone.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Drone);
                if let Some(volume) = extras.volume {
                    fit_data.drones_volume.insert(item_key, volume);
                }
                if let Some(bandwidth) = extras.bandwidth_use {
                    fit_data.drones_bandwidth.insert(item_key, bandwidth);
                };
                if !fit_data.drone_group_limit.is_empty() {
                    let drone_a_group_id = drone.get_a_group_id().unwrap();
                    if !fit_data.drone_group_limit.contains(&drone_a_group_id) {
                        fit_data.drone_groups.insert(item_key, drone_a_group_id);
                    }
                }
            }
            UadItem::Fighter(fighter) => {
                let extras = fighter.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Fighter);
                let count = fighter.get_count().unwrap();
                if let Some(volume) = extras.volume {
                    fit_data
                        .fighters_volume
                        .insert(item_key, volume * AttrVal::from(count.current));
                }
                if count.current > count.max {
                    fit_data.fighter_squad_size.insert(
                        item_key,
                        ValFighterSquadSizeFighterInfo {
                            size: count.current,
                            max_size: count.max,
                        },
                    );
                }
                if extras.is_light_fighter {
                    fit_data.light_fighters.insert(item_key);
                }
                if extras.is_heavy_fighter {
                    fit_data.heavy_fighters.insert(item_key);
                }
                if extras.is_support_fighter {
                    fit_data.support_fighters.insert(item_key);
                }
                if extras.is_standup_light_fighter {
                    fit_data.standup_light_fighters.insert(item_key);
                }
                if extras.is_standup_heavy_fighter {
                    fit_data.standup_heavy_fighters.insert(item_key);
                }
                if extras.is_standup_support_fighter {
                    fit_data.standup_support_fighters.insert(item_key);
                }
            }
            UadItem::Implant(implant) => {
                let extras = implant.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Implant);
                if let Some(a_slot) = implant.get_a_slot() {
                    fit_data.slotted_implants.add_entry(a_slot, item_key);
                }
            }
            UadItem::Module(module) => {
                let extras = module.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, get_module_expected_kind(module));
                if extras.takes_turret_hardpoint {
                    fit_data.mods_turret.insert(item_key);
                }
                if extras.takes_launcher_hardpoint {
                    fit_data.mods_launcher.insert(item_key);
                }
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = extras.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(a_item_grp_id, item_key);
                    if module.get_a_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_FITTED) {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_key, a_item_grp_id);
                    }
                }
                if extras.charge_limit.is_some() {
                    // If there is a charge, calculate later, otherwise mark as no issue
                    match module.get_charge_item_key() {
                        Some(_) => fit_data.mods_charge_group.insert(item_key, ValCache::Todo(())),
                        None => fit_data.mods_charge_group.insert(item_key, ValCache::Pass(())),
                    };
                }
                if let Some(allowed_charge_size) = module.get_a_attrs().unwrap().get(&ac::attrs::CHARGE_SIZE) {
                    // If there is a charge, calculate later, otherwise mark as no issue
                    match module.get_charge_item_key() {
                        Some(_) => fit_data
                            .mods_charge_size
                            .insert(item_key, ValCache::Todo(*allowed_charge_size)),
                        None => fit_data
                            .mods_charge_size
                            .insert(item_key, ValCache::Pass(*allowed_charge_size)),
                    };
                }
                if let Some(max_fitted) = extras.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_value(module.get_a_item_id(), item_key, max_fitted);
                }
                // Data is added to / removed from this map when charges are added/removed; here,
                // we just reset validation result when a module is being loaded
                handle_charge_volume_for_module(fit_data, item_key);
                if let Some(ad::AShipKind::CapitalShip) = extras.item_ship_kind {
                    // Unwrap, since item ship kind is set to capital only when volume is available
                    fit_data.mods_capital.insert(item_key, extras.volume.unwrap());
                }
                if let Some(sec_class) = extras.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_key, sec_class);
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_key);
                }
                item_vs_ship_kind_add(
                    uad,
                    fit_data,
                    item_key,
                    module.get_a_category_id().unwrap(),
                    module.get_fit_key(),
                );
            }
            UadItem::Rig(rig) => {
                let extras = rig.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Rig);
                let rig_size = rig.get_a_attrs().unwrap().get(&ac::attrs::RIG_SIZE).copied();
                fit_data.rigs_rig_size.insert(item_key, rig_size);
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = extras.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(a_item_grp_id, item_key);
                    if rig.get_a_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_FITTED) {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_key, a_item_grp_id);
                    }
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_key);
                }
                item_vs_ship_kind_add(
                    uad,
                    fit_data,
                    item_key,
                    rig.get_a_category_id().unwrap(),
                    rig.get_fit_key(),
                );
            }
            UadItem::Service(service) => {
                let extras = service.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Service);
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = extras.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(a_item_grp_id, item_key);
                    if service
                        .get_a_attrs()
                        .unwrap()
                        .contains_key(&ac::attrs::MAX_GROUP_FITTED)
                    {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_key, a_item_grp_id);
                    }
                }
                if let Some(max_fitted) = extras.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_value(service.get_a_item_id(), item_key, max_fitted);
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_key);
                }
                if let Some(sec_class) = extras.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_key, sec_class);
                }
                item_vs_ship_kind_add(
                    uad,
                    fit_data,
                    item_key,
                    service.get_a_category_id().unwrap(),
                    service.get_fit_key(),
                );
            }
            UadItem::Ship(ship) => {
                let fit = uad.fits.get(fit_key);
                let extras = ship.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Ship);
                // If new ship limits drones which can be used, fill the mismatch data up
                if let Some(drone_limit) = &extras.drone_limit {
                    fit_data.drone_group_limit.extend(drone_limit.group_ids.iter());
                    for &drone_item_key in fit.drones.iter() {
                        let drone_item = uad.items.get(drone_item_key);
                        // Not every drone is guaranteed to be loaded
                        if let Some(drone_a_group_id) = drone_item.get_a_group_id() {
                            if !drone_limit.group_ids.contains(&drone_a_group_id) {
                                fit_data.drone_groups.insert(drone_item_key, drone_a_group_id);
                            }
                        }
                    }
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_key);
                }
                if extras.disallowed_in_wspace {
                    fit_data.sec_zone_fitted_wspace_banned.insert(item_key);
                }
                // Ship/structure modules are not enforced when ship is not set. When we get one,
                // fill the data container up
                for &item_key in chain!(
                    fit.mods_high.iter_keys(),
                    fit.mods_mid.iter_keys(),
                    fit.mods_low.iter_keys(),
                    fit.rigs.iter(),
                    fit.services.iter()
                ) {
                    let item = uad.items.get(item_key);
                    // Not every item is guaranteed to be loaded
                    if let Some(item_cat_id) = item.get_a_category_id() {
                        match item_cat_id {
                            ac::itemcats::MODULE => {
                                if !matches!(fit.kind, ShipKind::Ship) {
                                    fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_key, ValShipKind::Ship);
                                }
                            }
                            ac::itemcats::STRUCTURE_MODULE => {
                                if !matches!(fit.kind, ShipKind::Structure) {
                                    fit_data
                                        .mods_rigs_svcs_vs_ship_kind
                                        .insert(item_key, ValShipKind::Structure);
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
            UadItem::Skill(skill) => {
                let extras = skill.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Skill);
            }
            UadItem::Stance(stance) => {
                let extras = stance.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Stance);
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
            }
            UadItem::Subsystem(subsystem) => {
                let extras = subsystem.get_a_extras().unwrap();
                item_kind_add(fit_data, item_key, extras.kind, ad::AItemKind::Subsystem);
                if let Some(a_slot) = subsystem.get_a_slot() {
                    fit_data.slotted_subsystems.add_entry(a_slot, item_key);
                }
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn item_unloaded(&mut self, item_key: &ItemKey, item: &UadItem) {
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_key);
        // Skill requirements
        if let Some(a_srqs) = item.get_effective_a_skill_reqs() {
            if !a_srqs.is_empty() {
                for skill_a_item_id in a_srqs.keys() {
                    fit_data.srqs_skill_item_map.remove_entry(skill_a_item_id, item_key);
                }
                fit_data.srqs_missing.remove(item_key);
            }
        }
        match item {
            UadItem::Booster(booster) => {
                let extras = booster.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Booster);
                if let Some(slot) = booster.get_a_slot() {
                    fit_data.slotted_boosters.remove_entry(&slot, item_key);
                }
            }
            UadItem::Character(character) => {
                let extras = character.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Character);
            }
            UadItem::Charge(charge) => {
                let extras = charge.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Charge);
                if let Entry::Occupied(mut entry) = fit_data.mods_charge_group.entry(charge.get_cont_item_key()) {
                    // No charge - check should pass
                    entry.insert(ValCache::Pass(()));
                }
                if let Entry::Occupied(mut entry) = fit_data.mods_charge_size.entry(charge.get_cont_item_key()) {
                    // No charge - check should pass
                    match entry.get() {
                        ValCache::Todo(allowed_charge_size) => {
                            entry.insert(ValCache::Pass(*allowed_charge_size));
                        }
                        ValCache::Fail(fail_cache) => {
                            entry.insert(ValCache::Pass(fail_cache.allowed_size));
                        }
                        _ => (),
                    }
                }
                fit_data.mods_charge_volume.remove(&charge.get_cont_item_key());
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_key);
                }
            }
            UadItem::Drone(drone) => {
                let extras = drone.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Drone);
                fit_data.drones_volume.remove(item_key);
                if extras.bandwidth_use.is_some() {
                    fit_data.drones_bandwidth.remove(item_key);
                }
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_groups.remove(item_key);
                }
            }
            UadItem::Fighter(fighter) => {
                let extras = fighter.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Fighter);
                fit_data.fighters_volume.remove(item_key);
                let count = fighter.get_count().unwrap();
                if count.current > count.max {
                    fit_data.fighter_squad_size.remove(item_key);
                }
                if extras.is_light_fighter {
                    fit_data.light_fighters.remove(item_key);
                }
                if extras.is_heavy_fighter {
                    fit_data.heavy_fighters.remove(item_key);
                }
                if extras.is_support_fighter {
                    fit_data.support_fighters.remove(item_key);
                }
                if extras.is_standup_light_fighter {
                    fit_data.standup_light_fighters.remove(item_key);
                }
                if extras.is_standup_heavy_fighter {
                    fit_data.standup_heavy_fighters.remove(item_key);
                }
                if extras.is_standup_support_fighter {
                    fit_data.standup_support_fighters.remove(item_key);
                }
            }
            UadItem::Implant(implant) => {
                let extras = implant.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Implant);
                if let Some(slot) = implant.get_a_slot() {
                    fit_data.slotted_implants.remove_entry(&slot, item_key);
                }
            }
            UadItem::Module(module) => {
                let extras = module.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, get_module_expected_kind(module));
                if extras.takes_turret_hardpoint {
                    fit_data.mods_turret.remove(item_key);
                }
                if extras.takes_launcher_hardpoint {
                    fit_data.mods_launcher.remove(item_key);
                }
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = extras.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(&a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if extras.charge_limit.is_some() {
                    fit_data.mods_charge_group.remove(item_key);
                }
                fit_data.mods_charge_size.remove(item_key);
                // Data is added to / removed from this map when charges are added/removed; here,
                // we just reset validation result when a module is being unloaded
                handle_charge_volume_for_module(fit_data, *item_key);
                if let Some(ad::AShipKind::CapitalShip) = extras.item_ship_kind {
                    fit_data.mods_capital.remove(item_key);
                }
                if extras.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(&module.get_a_item_id(), item_key);
                }
                if extras.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_key);
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UadItem::Rig(rig) => {
                let extras = rig.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Rig);
                fit_data.rigs_rig_size.remove(item_key);
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = extras.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(&a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UadItem::Service(service) => {
                let extras = service.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Service);
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = extras.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(&a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if extras.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(&service.get_a_item_id(), item_key);
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                if extras.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UadItem::Ship(ship) => {
                let extras = ship.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Ship);
                // If any drone group limits were defined, clear the mismatch data
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_group_limit.clear();
                    fit_data.drone_groups.clear();
                }
                if extras.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                if extras.disallowed_in_wspace {
                    fit_data.sec_zone_fitted_wspace_banned.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.clear();
            }
            UadItem::Skill(skill) => {
                let extras = skill.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Skill);
            }
            UadItem::Stance(stance) => {
                let extras = stance.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Stance);
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
            }
            UadItem::Subsystem(subsystem) => {
                let extras = subsystem.get_a_extras().unwrap();
                item_kind_remove(fit_data, item_key, extras.kind, ad::AItemKind::Subsystem);
                if let Some(slot) = subsystem.get_a_slot() {
                    fit_data.slotted_subsystems.remove_entry(&slot, item_key);
                }
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
            }
            _ => (),
        }
    }
}

fn handle_charge_volume_for_module(fit_data: &mut VastFitData, module_item_key: ItemKey) {
    if let Entry::Occupied(mut entry) = fit_data.mods_charge_volume.entry(module_item_key) {
        match entry.get() {
            ValCache::Pass(charge_volume) => {
                entry.insert(ValCache::Todo(*charge_volume));
            }
            ValCache::Fail(fail_cache) => {
                entry.insert(ValCache::Todo(fail_cache.charge_volume));
            }
            _ => (),
        }
    }
}

fn get_module_expected_kind(module: &UadModule) -> ad::AItemKind {
    match module.get_rack() {
        ModRack::High => ad::AItemKind::ModuleHigh,
        ModRack::Mid => ad::AItemKind::ModuleMid,
        ModRack::Low => ad::AItemKind::ModuleLow,
    }
}
fn item_kind_add(
    fit_data: &mut VastFitData,
    item_key: ItemKey,
    item_kind: Option<ad::AItemKind>,
    expected_kind: ad::AItemKind,
) {
    if item_kind != Some(expected_kind) {
        fit_data.item_kind.insert(
            item_key,
            ValItemKindItemInfo {
                kind: item_kind,
                expected_kind,
            },
        );
    }
}
fn item_kind_remove(
    fit_data: &mut VastFitData,
    item_key: &ItemKey,
    item_kind: Option<ad::AItemKind>,
    expected_kind: ad::AItemKind,
) {
    if item_kind != Some(expected_kind) {
        fit_data.item_kind.remove(item_key);
    }
}
fn item_vs_ship_kind_add(
    uad: &Uad,
    fit_data: &mut VastFitData,
    item_key: ItemKey,
    item_cat: ad::AItemCatId,
    fit_key: FitKey,
) {
    let fit = uad.fits.get(fit_key);
    let ship_key = match fit.ship {
        Some(ship_id) => ship_id,
        None => return,
    };
    match item_cat {
        ac::itemcats::MODULE => match fit.kind {
            ShipKind::Ship => (),
            ShipKind::Structure => {
                fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_key, ValShipKind::Ship);
            }
            ShipKind::Unknown => {
                let ship = uad.items.get(ship_key);
                if ship.is_loaded() {
                    fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_key, ValShipKind::Ship);
                }
            }
        },
        ac::itemcats::STRUCTURE_MODULE => match fit.kind {
            ShipKind::Ship => {
                fit_data
                    .mods_rigs_svcs_vs_ship_kind
                    .insert(item_key, ValShipKind::Structure);
            }
            ShipKind::Structure => (),
            ShipKind::Unknown => {
                let ship = uad.items.get(ship_key);
                if ship.is_loaded() {
                    fit_data
                        .mods_rigs_svcs_vs_ship_kind
                        .insert(item_key, ValShipKind::Structure);
                }
            }
        },
        _ => (),
    }
}
