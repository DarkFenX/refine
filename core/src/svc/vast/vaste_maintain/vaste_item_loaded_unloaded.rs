use itertools::chain;

use crate::{
    ac, ad,
    def::AttrVal,
    misc::ModRack,
    svc::vast::{ValFighterSquadSizeFighterInfo, ValItemKindItemInfo, ValShipKind, ValSrqSkillInfo, Vast, VastFitData},
    uad::{ShipKind, Uad, UadFitKey, UadItem, UadItemKey, UadModule},
    util::RMap,
};

impl Vast {
    pub(in crate::svc) fn item_loaded(&mut self, uad: &Uad, item_key: UadItemKey, item: &UadItem) {
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_key);
        // Skill requirements
        if let Some(a_srqs) = item.get_effective_a_skill_reqs()
            && !a_srqs.is_empty()
        {
            let mut missing_skills = RMap::new();
            let fit = uad.fits.get(fit_key);
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
            UadItem::Booster(booster) => {
                let a_item_xt = booster.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Booster);
                if let Some(a_slot) = booster.get_a_slot() {
                    fit_data.slotted_boosters.add_entry(a_slot, item_key);
                }
            }
            UadItem::Character(character) => {
                let a_item_xt = character.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Character);
            }
            UadItem::Charge(charge) => {
                let a_item_xt = charge.get_a_xt().unwrap();
                let cont_key = charge.get_cont_key();
                let cont_item = uad.items.get(cont_key);
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Charge);
                if let Some(cont_a_item_xt) = cont_item.get_a_xt() {
                    handle_charge_group_add(
                        fit_data,
                        cont_key,
                        cont_a_item_xt,
                        item_key,
                        &charge.get_a_group_id().unwrap(),
                    );
                    handle_charge_size_add(fit_data, cont_key, cont_a_item_xt, item_key, a_item_xt);
                    handle_charge_volume_add(fit_data, cont_key, cont_a_item_xt, item_key, a_item_xt);
                }
                if let Some(cont_a_grp_id) = cont_item.get_a_group_id() {
                    handle_charge_cont_group_add(fit_data, cont_key, &cont_a_grp_id, item_key, a_item_xt);
                }
                if a_item_xt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_key);
                }
            }
            UadItem::Drone(drone) => {
                let a_item_xt = drone.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Drone);
                fit_data.drones_volume.insert(item_key, a_item_xt.volume);
                if let Some(bandwidth) = a_item_xt.bandwidth_use {
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
                let a_item_xt = fighter.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Fighter);
                let count = fighter.get_count().unwrap();
                fit_data
                    .fighters_volume
                    .insert(item_key, a_item_xt.volume * AttrVal::from(count.current));
                if count.current > count.max {
                    fit_data.fighter_squad_size.insert(
                        item_key,
                        ValFighterSquadSizeFighterInfo {
                            size: count.current,
                            max_size: count.max,
                        },
                    );
                }
                if a_item_xt.is_light_fighter {
                    fit_data.light_fighters.insert(item_key);
                }
                if a_item_xt.is_heavy_fighter {
                    fit_data.heavy_fighters.insert(item_key);
                }
                if a_item_xt.is_support_fighter {
                    fit_data.support_fighters.insert(item_key);
                }
                if a_item_xt.is_st_light_fighter {
                    fit_data.st_light_fighters.insert(item_key);
                }
                if a_item_xt.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.insert(item_key);
                }
                if a_item_xt.is_st_support_fighter {
                    fit_data.st_support_fighters.insert(item_key);
                }
            }
            UadItem::Implant(implant) => {
                let a_item_xt = implant.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Implant);
                if let Some(a_slot) = implant.get_a_slot() {
                    fit_data.slotted_implants.add_entry(a_slot, item_key);
                }
            }
            UadItem::Module(module) => {
                let a_item_xt = module.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, get_module_expected_kind(module));
                if a_item_xt.takes_turret_hardpoint {
                    fit_data.mods_turret.insert(item_key);
                }
                if a_item_xt.takes_launcher_hardpoint {
                    fit_data.mods_launcher.insert(item_key);
                }
                if let Some(ship_limit) = &a_item_xt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = module.get_val_fitted_a_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(a_item_grp_id, item_key);
                    if module.get_a_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_FITTED) {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_key, a_item_grp_id);
                    }
                }
                if let Some(charge_key) = module.get_charge_key() {
                    let charge_item = uad.items.get(charge_key);
                    if let Some(charge_a_grp_id) = charge_item.get_a_group_id() {
                        handle_charge_group_add(fit_data, item_key, a_item_xt, charge_key, &charge_a_grp_id);
                    }
                    if let Some(charge_a_item_xt) = charge_item.get_a_xt() {
                        if let Some(a_grp_id) = item.get_a_group_id() {
                            handle_charge_cont_group_add(fit_data, item_key, &a_grp_id, charge_key, charge_a_item_xt);
                        }
                        handle_charge_size_add(fit_data, item_key, a_item_xt, charge_key, charge_a_item_xt);
                        handle_charge_volume_add(fit_data, item_key, a_item_xt, charge_key, charge_a_item_xt);
                    }
                }
                if let Some(max_fitted) = a_item_xt.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(module.get_a_item_id(), item_key, max_fitted);
                }
                if let Some(ad::AShipKind::CapitalShip) = a_item_xt.item_ship_kind {
                    fit_data.mods_capital.insert(item_key, a_item_xt.volume);
                }
                if let Some(sec_class) = a_item_xt.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_key, sec_class);
                }
                if a_item_xt.sec_zone_limitable {
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
                let a_item_xt = rig.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Rig);
                let rig_size = rig.get_a_attrs().unwrap().get(&ac::attrs::RIG_SIZE).copied();
                fit_data.rigs_rig_size.insert(item_key, rig_size);
                if let Some(ship_limit) = &a_item_xt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = rig.get_val_fitted_a_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(a_item_grp_id, item_key);
                    if rig.get_a_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_FITTED) {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_key, a_item_grp_id);
                    }
                }
                if a_item_xt.sec_zone_limitable {
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
                let a_item_xt = service.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Service);
                if let Some(ship_limit) = &a_item_xt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
                if let Some(a_item_grp_id) = service.get_val_fitted_a_group_id() {
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
                if let Some(max_fitted) = a_item_xt.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(service.get_a_item_id(), item_key, max_fitted);
                }
                if a_item_xt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_key);
                }
                if let Some(sec_class) = a_item_xt.online_max_sec_class {
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
                let a_item_xt = ship.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Ship);
                // If new ship limits drones which can be used, fill the mismatch data up
                if let Some(drone_limit) = &a_item_xt.drone_limit {
                    fit_data.drone_group_limit.extend(drone_limit.group_ids.iter());
                    for &drone_key in fit.drones.iter() {
                        let drone_item = uad.items.get(drone_key);
                        // Not every drone is guaranteed to be loaded
                        if let Some(drone_a_group_id) = drone_item.get_a_group_id()
                            && !drone_limit.group_ids.contains(&drone_a_group_id)
                        {
                            fit_data.drone_groups.insert(drone_key, drone_a_group_id);
                        }
                    }
                }
                if a_item_xt.sec_zone_limitable {
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
                let a_item_xt = skill.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Skill);
            }
            UadItem::Stance(stance) => {
                let a_item_xt = stance.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Stance);
                if let Some(ship_limit) = &a_item_xt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
            }
            UadItem::Subsystem(subsystem) => {
                let a_item_xt = subsystem.get_a_xt().unwrap();
                item_kind_add(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Subsystem);
                if let Some(a_slot) = subsystem.get_a_slot() {
                    fit_data.slotted_subsystems.add_entry(a_slot, item_key);
                }
                if let Some(ship_limit) = &a_item_xt.ship_limit {
                    fit_data.ship_limited_items.insert(item_key, ship_limit.clone());
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn item_unloaded(&mut self, item_key: &UadItemKey, item: &UadItem) {
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_key);
        // Skill requirements
        if let Some(a_srqs) = item.get_effective_a_skill_reqs()
            && !a_srqs.is_empty()
        {
            for skill_a_item_id in a_srqs.keys() {
                fit_data.srqs_skill_item_map.remove_entry(skill_a_item_id, item_key);
            }
            fit_data.srqs_missing.remove(item_key);
        }
        match item {
            UadItem::Booster(booster) => {
                let a_item_xt = booster.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Booster);
                if let Some(slot) = booster.get_a_slot() {
                    fit_data.slotted_boosters.remove_entry(&slot, item_key);
                }
            }
            UadItem::Character(character) => {
                let a_item_xt = character.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Character);
            }
            UadItem::Charge(charge) => {
                let a_item_xt = charge.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Charge);
                fit_data.charge_group.remove(item_key);
                if a_item_xt.cont_limit.is_some() {
                    fit_data.charge_cont_group.remove(item_key);
                }
                fit_data.charge_size.remove(item_key);
                fit_data.charge_volume.remove(item_key);
                if a_item_xt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_key);
                }
            }
            UadItem::Drone(drone) => {
                let a_item_xt = drone.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Drone);
                fit_data.drones_volume.remove(item_key);
                if a_item_xt.bandwidth_use.is_some() {
                    fit_data.drones_bandwidth.remove(item_key);
                }
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_groups.remove(item_key);
                }
            }
            UadItem::Fighter(fighter) => {
                let a_item_xt = fighter.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Fighter);
                fit_data.fighters_volume.remove(item_key);
                let count = fighter.get_count().unwrap();
                if count.current > count.max {
                    fit_data.fighter_squad_size.remove(item_key);
                }
                if a_item_xt.is_light_fighter {
                    fit_data.light_fighters.remove(item_key);
                }
                if a_item_xt.is_heavy_fighter {
                    fit_data.heavy_fighters.remove(item_key);
                }
                if a_item_xt.is_support_fighter {
                    fit_data.support_fighters.remove(item_key);
                }
                if a_item_xt.is_st_light_fighter {
                    fit_data.st_light_fighters.remove(item_key);
                }
                if a_item_xt.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.remove(item_key);
                }
                if a_item_xt.is_st_support_fighter {
                    fit_data.st_support_fighters.remove(item_key);
                }
            }
            UadItem::Implant(implant) => {
                let a_item_xt = implant.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Implant);
                if let Some(slot) = implant.get_a_slot() {
                    fit_data.slotted_implants.remove_entry(&slot, item_key);
                }
            }
            UadItem::Module(module) => {
                let a_item_xt = module.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, get_module_expected_kind(module));
                if a_item_xt.takes_turret_hardpoint {
                    fit_data.mods_turret.remove(item_key);
                }
                if a_item_xt.takes_launcher_hardpoint {
                    fit_data.mods_launcher.remove(item_key);
                }
                if a_item_xt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = module.get_val_fitted_a_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(&a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if let Some(charge_key) = module.get_charge_key() {
                    if a_item_xt.charge_limit.is_some() {
                        fit_data.charge_group.remove(&charge_key);
                    }
                    fit_data.charge_cont_group.remove(&charge_key);
                    if a_item_xt.charge_size.is_some() {
                        fit_data.charge_size.remove(&charge_key);
                    }
                    fit_data.charge_volume.remove(&charge_key);
                }
                if let Some(ad::AShipKind::CapitalShip) = a_item_xt.item_ship_kind {
                    fit_data.mods_capital.remove(item_key);
                }
                if a_item_xt.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(&module.get_a_item_id(), item_key);
                }
                if a_item_xt.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_key);
                }
                if a_item_xt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UadItem::Rig(rig) => {
                let a_item_xt = rig.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Rig);
                fit_data.rigs_rig_size.remove(item_key);
                if a_item_xt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = rig.get_val_fitted_a_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(&a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if a_item_xt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UadItem::Service(service) => {
                let a_item_xt = service.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Service);
                if a_item_xt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
                if let Some(a_item_grp_id) = service.get_val_fitted_a_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(&a_item_grp_id, item_key);
                    fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_key);
                }
                if a_item_xt.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(&service.get_a_item_id(), item_key);
                }
                if a_item_xt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                if a_item_xt.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_key);
            }
            UadItem::Ship(ship) => {
                let a_item_xt = ship.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Ship);
                // If any drone group limits were defined, clear the mismatch data
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_group_limit.clear();
                    fit_data.drone_groups.clear();
                }
                if a_item_xt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_key);
                }
                if ship.get_disallowed_in_wspace().unwrap() {
                    fit_data.sec_zone_fitted_wspace_banned.remove(item_key);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.clear();
            }
            UadItem::Skill(skill) => {
                let a_item_xt = skill.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Skill);
            }
            UadItem::Stance(stance) => {
                let a_item_xt = stance.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Stance);
                if a_item_xt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
            }
            UadItem::Subsystem(subsystem) => {
                let a_item_xt = subsystem.get_a_xt().unwrap();
                item_kind_remove(fit_data, item_key, a_item_xt.kind, ad::AItemKind::Subsystem);
                if let Some(slot) = subsystem.get_a_slot() {
                    fit_data.slotted_subsystems.remove_entry(&slot, item_key);
                }
                if a_item_xt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_key);
                }
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
    item_key: UadItemKey,
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
    item_key: &UadItemKey,
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
    item_key: UadItemKey,
    item_cat: ad::AItemCatId,
    fit_key: UadFitKey,
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

fn handle_charge_group_add(
    fit_data: &mut VastFitData,
    cont_key: UadItemKey,
    cont_a_item_xt: &ad::AItemXt,
    charge_key: UadItemKey,
    charge_a_group_id: &ad::AItemGrpId,
) {
    if let Some(charge_limit) = &cont_a_item_xt.charge_limit
        && !charge_limit.group_ids.contains(charge_a_group_id)
    {
        fit_data.charge_group.insert(charge_key, cont_key);
    }
}

fn handle_charge_cont_group_add(
    fit_data: &mut VastFitData,
    cont_key: UadItemKey,
    cont_a_group_id: &ad::AItemGrpId,
    charge_key: UadItemKey,
    charge_a_item_xt: &ad::AItemXt,
) {
    if let Some(charge_cont_limit) = &charge_a_item_xt.cont_limit
        && !charge_cont_limit.group_ids.contains(cont_a_group_id)
    {
        fit_data.charge_cont_group.insert(charge_key, cont_key);
    }
}

fn handle_charge_size_add(
    fit_data: &mut VastFitData,
    cont_key: UadItemKey,
    cont_a_item_xt: &ad::AItemXt,
    charge_key: UadItemKey,
    charge_a_item_xt: &ad::AItemXt,
) {
    // Charge size mismatch happens when parent module requires some charge size
    if cont_a_item_xt.charge_size.is_some() && cont_a_item_xt.charge_size != charge_a_item_xt.charge_size {
        fit_data.charge_size.insert(charge_key, cont_key);
    }
}

fn handle_charge_volume_add(
    fit_data: &mut VastFitData,
    cont_key: UadItemKey,
    cont_a_item_xt: &ad::AItemXt,
    charge_key: UadItemKey,
    charge_a_item_xt: &ad::AItemXt,
) {
    if cont_a_item_xt.capacity < charge_a_item_xt.volume {
        fit_data.charge_volume.insert(charge_key, cont_key);
    }
}
