use itertools::chain;

use crate::{
    ac, ad,
    def::AttrVal,
    misc::ModRack,
    rd,
    svc::vast::{ValFighterSquadSizeFighterInfo, ValItemKindItemInfo, ValShipKind, ValSrqSkillInfo, Vast, VastFitData},
    ud::{UData, UFitKey, UItem, UItemKey, UModule, UShipKind},
    util::RMap,
};

impl Vast {
    pub(in crate::svc) fn item_loaded(&mut self, u_data: &UData, item_key: UItemKey, item: &UItem) {
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_key);
        // Skill requirements
        if let Some(a_srqs) = item.get_effective_skill_reqs()
            && !a_srqs.is_empty()
        {
            let mut missing_skills = RMap::new();
            let fit = u_data.fits.get(fit_key);
            for (&skill_a_item_id, &required_a_lvl) in a_srqs.iter() {
                fit_data.srqs_skill_item_map.add_entry(skill_a_item_id, item_key);
                let current_lvl = fit.skills.get(&skill_a_item_id).map(|v| v.level);
                if match current_lvl {
                    Some(current_lvl) => current_lvl.get_inner() < required_a_lvl.get_inner(),
                    None => true,
                } {
                    missing_skills.insert(
                        skill_a_item_id,
                        ValSrqSkillInfo {
                            current_lvl,
                            required_lvl: required_a_lvl.into(),
                        },
                    );
                }
            }
            if !missing_skills.is_empty() {
                fit_data.srqs_missing.insert(item_key, missing_skills);
            }
        }
        match item {
            UItem::Booster(booster) => {
                let item_axt = booster.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Booster);
                if let Some(a_slot) = booster.get_slot() {
                    fit_data.slotted_boosters.add_entry(a_slot, item_key);
                }
            }
            UItem::Character(character) => {
                let item_axt = character.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Character);
            }
            UItem::Charge(charge) => {
                let item_axt = charge.get_axt().unwrap();
                let cont_key = charge.get_cont_item_key();
                let cont_item = u_data.items.get(cont_key);
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Charge);
                if let Some(cont_axt) = cont_item.get_axt() {
                    handle_charge_group_add(fit_data, cont_key, cont_axt, item_key, &charge.get_group_id().unwrap());
                    handle_charge_size_add(fit_data, cont_key, cont_axt, item_key, item_axt);
                    handle_charge_volume_add(fit_data, cont_key, cont_axt, item_key, item_axt);
                }
                if let Some(cont_a_grp_id) = cont_item.get_group_id() {
                    handle_charge_cont_group_add(fit_data, cont_key, &cont_a_grp_id, item_key, item_axt);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_key);
                }
            }
            UItem::Drone(drone) => {
                let item_axt = drone.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Drone);
                fit_data.drones_volume.insert(item_key, item_axt.volume);
                if let Some(bandwidth) = item_axt.bandwidth_use {
                    fit_data.drones_bandwidth.insert(item_key, bandwidth);
                };
                if !fit_data.drone_group_limit.is_empty() {
                    let drone_a_group_id = drone.get_group_id().unwrap();
                    if !fit_data.drone_group_limit.contains(&drone_a_group_id) {
                        fit_data.drone_groups.insert(item_key, drone_a_group_id);
                    }
                }
            }
            UItem::Fighter(fighter) => {
                let item_axt = fighter.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Fighter);
                let count = fighter.get_count().unwrap();
                fit_data
                    .fighters_volume
                    .insert(item_key, item_axt.volume * AttrVal::from(count.current));
                if count.current > count.max {
                    fit_data.fighter_squad_size.insert(
                        item_key,
                        ValFighterSquadSizeFighterInfo {
                            size: count.current,
                            max_size: count.max,
                        },
                    );
                }
                if item_axt.is_light_fighter {
                    fit_data.light_fighters.insert(item_key);
                }
                if item_axt.is_heavy_fighter {
                    fit_data.heavy_fighters.insert(item_key);
                }
                if item_axt.is_support_fighter {
                    fit_data.support_fighters.insert(item_key);
                }
                if item_axt.is_st_light_fighter {
                    fit_data.st_light_fighters.insert(item_key);
                }
                if item_axt.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.insert(item_key);
                }
                if item_axt.is_st_support_fighter {
                    fit_data.st_support_fighters.insert(item_key);
                }
            }
            UItem::Implant(implant) => {
                let item_axt = implant.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Implant);
                if let Some(a_slot) = implant.get_slot() {
                    fit_data.slotted_implants.add_entry(a_slot, item_key);
                }
            }
            UItem::Module(module) => {
                let item_axt = module.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, get_module_expected_kind(module));
                if module.takes_turret_hardpoint() {
                    fit_data.mods_turret.insert(item_key);
                }
                if module.takes_launcher_hardpoint() {
                    fit_data.mods_launcher.insert(item_key);
                }
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = module.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(a_item_grp_id, item_key);
                    if module.get_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_FITTED) {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_key, a_item_grp_id);
                    }
                }
                if let Some(charge_key) = module.get_charge_key() {
                    let charge_item = u_data.items.get(charge_key);
                    if let Some(charge_a_grp_id) = charge_item.get_group_id() {
                        handle_charge_group_add(fit_data, item_key, item_axt, charge_key, &charge_a_grp_id);
                    }
                    if let Some(charge_axt) = charge_item.get_axt() {
                        if let Some(a_grp_id) = item.get_group_id() {
                            handle_charge_cont_group_add(fit_data, item_key, &a_grp_id, charge_key, charge_axt);
                        }
                        handle_charge_size_add(fit_data, item_key, item_axt, charge_key, charge_axt);
                        handle_charge_volume_add(fit_data, item_key, item_axt, charge_key, charge_axt);
                    }
                }
                if let Some(max_fitted) = item_axt.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(module.get_type_id(), item_key, max_fitted);
                }
                if let Some(rd::RShipKind::CapitalShip) = item_axt.item_ship_kind {
                    fit_data.mods_capital.insert(item_key, item_axt.volume);
                }
                if let Some(sec_class) = item_axt.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_key, sec_class);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_key);
                }
                item_vs_ship_kind_add(
                    u_data,
                    fit_data,
                    item_key,
                    module.get_category_id().unwrap(),
                    module.get_fit_key(),
                );
            }
            UItem::Rig(rig) => {
                let item_axt = rig.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Rig);
                let rig_size = rig.get_attrs().unwrap().get(&ac::attrs::RIG_SIZE).copied();
                fit_data.rigs_rig_size.insert(item_key, rig_size);
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = rig.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(a_item_grp_id, item_key);
                    if rig.get_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_FITTED) {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_key, a_item_grp_id);
                    }
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_key);
                }
                item_vs_ship_kind_add(
                    u_data,
                    fit_data,
                    item_key,
                    rig.get_category_id().unwrap(),
                    rig.get_fit_key(),
                );
            }
            UItem::Service(service) => {
                let item_axt = service.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Service);
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = service.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(a_item_grp_id, item_key);
                    if service.get_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_FITTED) {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_key, a_item_grp_id);
                    }
                }
                if let Some(max_fitted) = item_axt.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(service.get_type_id(), item_key, max_fitted);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_key);
                }
                if let Some(sec_class) = item_axt.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_key, sec_class);
                }
                item_vs_ship_kind_add(
                    u_data,
                    fit_data,
                    item_key,
                    service.get_category_id().unwrap(),
                    service.get_fit_key(),
                );
            }
            UItem::Ship(ship) => {
                let fit = u_data.fits.get(fit_key);
                let item_axt = ship.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Ship);
                // If new ship limits drones which can be used, fill the mismatch data up
                if let Some(drone_limit) = &item_axt.drone_limit {
                    fit_data.drone_group_limit.extend(drone_limit.group_ids.iter());
                    for &drone_key in fit.drones.iter() {
                        let drone_item = u_data.items.get(drone_key);
                        // Not every drone is guaranteed to be loaded
                        if let Some(drone_a_group_id) = drone_item.get_group_id()
                            && !drone_limit.group_ids.contains(&drone_a_group_id)
                        {
                            fit_data.drone_groups.insert(drone_key, drone_a_group_id);
                        }
                    }
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_key);
                }
                if ship.get_disallowed_in_wspace().unwrap() {
                    fit_data.sec_zone_fitted_wspace_banned.insert(item_key);
                }
                // Ship/structure modules are not enforced when ship is not set. When we get one,
                // fill the data container up
                for item_key in chain!(
                    fit.iter_module_keys(),
                    fit.rigs.iter().copied(),
                    fit.services.iter().copied(),
                ) {
                    let item = u_data.items.get(item_key);
                    // Not every item is guaranteed to be loaded
                    if let Some(item_cat_id) = item.get_category_id() {
                        match item_cat_id {
                            ac::itemcats::MODULE => {
                                if !matches!(fit.kind, UShipKind::Ship) {
                                    fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_key, ValShipKind::Ship);
                                }
                            }
                            ac::itemcats::STRUCTURE_MODULE => {
                                if !matches!(fit.kind, UShipKind::Structure) {
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
            UItem::Skill(skill) => {
                let item_axt = skill.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Skill);
            }
            UItem::Stance(stance) => {
                let item_axt = stance.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Stance);
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
            }
            UItem::Subsystem(subsystem) => {
                let item_axt = subsystem.get_axt().unwrap();
                item_kind_add(fit_data, item_key, item_axt.kind, rd::RItemKind::Subsystem);
                if let Some(a_slot) = subsystem.get_slot() {
                    fit_data.slotted_subsystems.add_entry(a_slot, item_key);
                }
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn item_unloaded(&mut self, item_key: &UItemKey, item: &UItem) {
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_key);
        // Skill requirements
        if let Some(a_srqs) = item.get_effective_skill_reqs()
            && !a_srqs.is_empty()
        {
            for &skill_a_item_id in a_srqs.keys() {
                fit_data.srqs_skill_item_map.remove_entry(skill_a_item_id, item_key);
            }
            fit_data.srqs_missing.remove(item_key);
        }
        match item {
            UItem::Booster(booster) => {
                let item_axt = booster.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Booster);
                if let Some(slot) = booster.get_slot() {
                    fit_data.slotted_boosters.remove_entry(slot, item_key);
                }
            }
            UItem::Character(character) => {
                let item_axt = character.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Character);
            }
            UItem::Charge(charge) => {
                let item_axt = charge.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Charge);
                fit_data.charge_group.remove(item_key);
                if item_axt.cont_limit.is_some() {
                    fit_data.charge_cont_group.remove(item_key);
                }
                fit_data.charge_size.remove(item_key);
                fit_data.charge_volume.remove(item_key);
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_key);
                }
            }
            UItem::Drone(drone) => {
                let item_axt = drone.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Drone);
                fit_data.drones_volume.remove(item_key);
                if item_axt.bandwidth_use.is_some() {
                    fit_data.drones_bandwidth.remove(item_key);
                }
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_groups.remove(item_key);
                }
            }
            UItem::Fighter(fighter) => {
                let item_axt = fighter.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Fighter);
                fit_data.fighters_volume.remove(item_key);
                let count = fighter.get_count().unwrap();
                if count.current > count.max {
                    fit_data.fighter_squad_size.remove(item_key);
                }
                if item_axt.is_light_fighter {
                    fit_data.light_fighters.remove(item_key);
                }
                if item_axt.is_heavy_fighter {
                    fit_data.heavy_fighters.remove(item_key);
                }
                if item_axt.is_support_fighter {
                    fit_data.support_fighters.remove(item_key);
                }
                if item_axt.is_st_light_fighter {
                    fit_data.st_light_fighters.remove(item_key);
                }
                if item_axt.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.remove(item_key);
                }
                if item_axt.is_st_support_fighter {
                    fit_data.st_support_fighters.remove(item_key);
                }
            }
            UItem::Implant(implant) => {
                let item_axt = implant.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Implant);
                if let Some(slot) = implant.get_slot() {
                    fit_data.slotted_implants.remove_entry(slot, item_key);
                }
            }
            UItem::Module(module) => {
                let item_axt = module.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, get_module_expected_kind(module));
                if module.takes_turret_hardpoint() {
                    fit_data.mods_turret.remove(item_key);
                }
                if module.takes_launcher_hardpoint() {
                    fit_data.mods_launcher.remove(item_key);
                }
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = module.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if let Some(charge_key) = module.get_charge_key() {
                    if item_axt.charge_limit.is_some() {
                        fit_data.charge_group.remove(&charge_key);
                    }
                    fit_data.charge_cont_group.remove(&charge_key);
                    if item_axt.charge_size.is_some() {
                        fit_data.charge_size.remove(&charge_key);
                    }
                    fit_data.charge_volume.remove(&charge_key);
                }
                if let Some(rd::RShipKind::CapitalShip) = item_axt.item_ship_kind {
                    fit_data.mods_capital.remove(item_key);
                }
                if item_axt.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(&module.get_type_id(), item_key);
                }
                if item_axt.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_key);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UItem::Rig(rig) => {
                let item_axt = rig.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Rig);
                fit_data.rigs_rig_size.remove(item_key);
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = rig.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UItem::Service(service) => {
                let item_axt = service.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Service);
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = service.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if item_axt.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(&service.get_type_id(), item_key);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                if item_axt.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UItem::Ship(ship) => {
                let item_axt = ship.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Ship);
                // If any drone group limits were defined, clear the mismatch data
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_group_limit.clear();
                    fit_data.drone_groups.clear();
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                if ship.get_disallowed_in_wspace().unwrap() {
                    fit_data.sec_zone_fitted_wspace_banned.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.clear();
            }
            UItem::Skill(skill) => {
                let item_axt = skill.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Skill);
            }
            UItem::Stance(stance) => {
                let item_axt = stance.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Stance);
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
            }
            UItem::Subsystem(subsystem) => {
                let item_axt = subsystem.get_axt().unwrap();
                item_kind_remove(fit_data, item_key, item_axt.kind, rd::RItemKind::Subsystem);
                if let Some(slot) = subsystem.get_slot() {
                    fit_data.slotted_subsystems.remove_entry(slot, item_key);
                }
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
            }
            _ => (),
        }
    }
}

fn get_module_expected_kind(module: &UModule) -> rd::RItemKind {
    match module.get_rack() {
        ModRack::High => rd::RItemKind::ModuleHigh,
        ModRack::Mid => rd::RItemKind::ModuleMid,
        ModRack::Low => rd::RItemKind::ModuleLow,
    }
}
fn item_kind_add(
    fit_data: &mut VastFitData,
    item_key: UItemKey,
    item_kind: Option<rd::RItemKind>,
    expected_kind: rd::RItemKind,
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
    item_key: &UItemKey,
    item_kind: Option<rd::RItemKind>,
    expected_kind: rd::RItemKind,
) {
    if item_kind != Some(expected_kind) {
        fit_data.item_kind.remove(item_key);
    }
}
fn item_vs_ship_kind_add(
    u_data: &UData,
    fit_data: &mut VastFitData,
    item_key: UItemKey,
    item_cat: ad::AItemCatId,
    fit_key: UFitKey,
) {
    let fit = u_data.fits.get(fit_key);
    let ship_key = match fit.ship {
        Some(ship_id) => ship_id,
        None => return,
    };
    match item_cat {
        ac::itemcats::MODULE => match fit.kind {
            UShipKind::Ship => (),
            UShipKind::Structure => {
                fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_key, ValShipKind::Ship);
            }
            UShipKind::Unknown => {
                let ship = u_data.items.get(ship_key);
                if ship.is_loaded() {
                    fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_key, ValShipKind::Ship);
                }
            }
        },
        ac::itemcats::STRUCTURE_MODULE => match fit.kind {
            UShipKind::Ship => {
                fit_data
                    .mods_rigs_svcs_vs_ship_kind
                    .insert(item_key, ValShipKind::Structure);
            }
            UShipKind::Structure => (),
            UShipKind::Unknown => {
                let ship = u_data.items.get(ship_key);
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

fn handle_charge_group_add(
    fit_data: &mut VastFitData,
    cont_key: UItemKey,
    cont_axt: &rd::RItemAXt,
    charge_key: UItemKey,
    charge_a_group_id: &ad::AItemGrpId,
) {
    if let Some(charge_limit) = &cont_axt.charge_limit
        && !charge_limit.group_ids.contains(charge_a_group_id)
    {
        fit_data.charge_group.insert(charge_key, cont_key);
    }
}

fn handle_charge_cont_group_add(
    fit_data: &mut VastFitData,
    cont_key: UItemKey,
    cont_a_group_id: &ad::AItemGrpId,
    charge_key: UItemKey,
    charge_axt: &rd::RItemAXt,
) {
    if let Some(charge_cont_limit) = &charge_axt.cont_limit
        && !charge_cont_limit.group_ids.contains(cont_a_group_id)
    {
        fit_data.charge_cont_group.insert(charge_key, cont_key);
    }
}

fn handle_charge_size_add(
    fit_data: &mut VastFitData,
    cont_key: UItemKey,
    cont_axt: &rd::RItemAXt,
    charge_key: UItemKey,
    charge_axt: &rd::RItemAXt,
) {
    // Charge size mismatch happens when parent module requires some charge size
    if cont_axt.charge_size.is_some() && cont_axt.charge_size != charge_axt.charge_size {
        fit_data.charge_size.insert(charge_key, cont_key);
    }
}

fn handle_charge_volume_add(
    fit_data: &mut VastFitData,
    cont_key: UItemKey,
    cont_axt: &rd::RItemAXt,
    charge_key: UItemKey,
    charge_axt: &rd::RItemAXt,
) {
    if cont_axt.capacity < charge_axt.volume {
        fit_data.charge_volume.insert(charge_key, cont_key);
    }
}
