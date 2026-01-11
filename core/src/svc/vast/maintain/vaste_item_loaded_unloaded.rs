use itertools::chain;

use crate::{
    ad::{AItemCatId, AItemGrpId},
    misc::{ItemKind, ModRack},
    rd::{RItemAXt, RShipKind},
    svc::vast::{ValFighterSquadSizeFighterInfo, ValItemKindItemInfo, ValShipKind, ValSrqSkillInfo, Vast, VastFitData},
    ud::{UData, UFitId, UItem, UItemId, UModule, UShipKind},
    util::RMap,
};

impl Vast {
    pub(in crate::svc) fn item_loaded(&mut self, u_data: &UData, item_uid: UItemId, item: &UItem) {
        let fit_uid = match item.get_fit_uid() {
            Some(fit_uid) => fit_uid,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_uid);
        // Skill requirements
        if let Some(a_srqs) = item.get_effective_skill_reqs()
            && !a_srqs.is_empty()
        {
            let mut missing_skills = RMap::new();
            let fit = u_data.fits.get(fit_uid);
            for (&skill_item_aid, &required_lvl) in a_srqs.iter() {
                fit_data.srqs_skill_item_map.add_entry(skill_item_aid, item_uid);
                let current_lvl = fit.skills.get(&skill_item_aid).map(|v| v.level);
                if match current_lvl {
                    Some(current_lvl) => current_lvl < required_lvl,
                    None => true,
                } {
                    missing_skills.insert(
                        skill_item_aid,
                        ValSrqSkillInfo {
                            current_lvl,
                            required_lvl,
                        },
                    );
                }
            }
            if !missing_skills.is_empty() {
                fit_data.srqs_missing.insert(item_uid, missing_skills);
            }
        }
        match item {
            UItem::Booster(booster) => {
                let item_axt = booster.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Booster);
                if let Some(a_slot) = booster.get_slot() {
                    fit_data.slotted_boosters.add_entry(a_slot, item_uid);
                }
            }
            UItem::Character(character) => {
                let item_axt = character.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Character);
            }
            UItem::Charge(charge) => {
                let item_axt = charge.get_axt().unwrap();
                let cont_uid = charge.get_cont_item_uid();
                let cont_item = u_data.items.get(cont_uid);
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Charge);
                if let Some(cont_axt) = cont_item.get_axt() {
                    handle_charge_group_add(fit_data, cont_uid, cont_axt, item_uid, &charge.get_group_id().unwrap());
                    handle_charge_size_add(fit_data, cont_uid, cont_axt, item_uid, item_axt);
                    handle_charge_volume_add(fit_data, cont_uid, cont_axt, item_uid, item_axt);
                }
                if let Some(cont_grp_aid) = cont_item.get_group_id() {
                    handle_charge_cont_group_add(fit_data, cont_uid, &cont_grp_aid, item_uid, item_axt);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_uid);
                }
            }
            UItem::Drone(drone) => {
                let item_axt = drone.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Drone);
                fit_data.drones_volume.insert(item_uid, item_axt.volume);
                if let Some(bandwidth) = item_axt.bandwidth_use {
                    fit_data.drones_bandwidth.insert(item_uid, bandwidth);
                };
                if !fit_data.drone_group_limit.is_empty() {
                    let drone_group_aid = drone.get_group_id().unwrap();
                    if !fit_data.drone_group_limit.contains(&drone_group_aid) {
                        fit_data.drone_groups.insert(item_uid, drone_group_aid);
                    }
                }
            }
            UItem::Fighter(fighter) => {
                let item_axt = fighter.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Fighter);
                let count = fighter.get_count().unwrap();
                fit_data
                    .fighters_volume
                    .insert(item_uid, item_axt.volume * count.current.into_pvalue());
                if count.current > count.max {
                    fit_data.fighter_squad_size.insert(
                        item_uid,
                        ValFighterSquadSizeFighterInfo {
                            size: count.current,
                            max_size: count.max,
                        },
                    );
                }
                if item_axt.is_light_fighter {
                    fit_data.light_fighters.insert(item_uid);
                }
                if item_axt.is_heavy_fighter {
                    fit_data.heavy_fighters.insert(item_uid);
                }
                if item_axt.is_support_fighter {
                    fit_data.support_fighters.insert(item_uid);
                }
                if item_axt.is_st_light_fighter {
                    fit_data.st_light_fighters.insert(item_uid);
                }
                if item_axt.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.insert(item_uid);
                }
                if item_axt.is_st_support_fighter {
                    fit_data.st_support_fighters.insert(item_uid);
                }
            }
            UItem::Implant(implant) => {
                let item_axt = implant.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Implant);
                if let Some(a_slot) = implant.get_slot() {
                    fit_data.slotted_implants.add_entry(a_slot, item_uid);
                }
            }
            UItem::Module(module) => {
                let item_axt = module.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, get_module_expected_kind(module));
                if module.takes_turret_hardpoint() {
                    fit_data.mods_turret.insert(item_uid);
                }
                if module.takes_launcher_hardpoint() {
                    fit_data.mods_launcher.insert(item_uid);
                }
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = module.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if item_axt.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if let Some(charge_uid) = module.get_charge_uid() {
                    let charge_item = u_data.items.get(charge_uid);
                    if let Some(charge_grp_aid) = charge_item.get_group_id() {
                        handle_charge_group_add(fit_data, item_uid, item_axt, charge_uid, &charge_grp_aid);
                    }
                    if let Some(charge_axt) = charge_item.get_axt() {
                        if let Some(grp_aid) = item.get_group_id() {
                            handle_charge_cont_group_add(fit_data, item_uid, &grp_aid, charge_uid, charge_axt);
                        }
                        handle_charge_size_add(fit_data, item_uid, item_axt, charge_uid, charge_axt);
                        handle_charge_volume_add(fit_data, item_uid, item_axt, charge_uid, charge_axt);
                    }
                }
                if let Some(max_fitted) = item_axt.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(module.get_type_aid(), item_uid, max_fitted);
                }
                if let Some(RShipKind::CapitalShip) = item_axt.item_ship_kind {
                    fit_data.mods_capital.insert(item_uid, item_axt.volume);
                }
                if let Some(sec_class) = item_axt.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_uid, sec_class);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_uid);
                }
                if let Some(cap_attr_rids) = module.get_cap_use_attr_rids()
                    && !cap_attr_rids.is_empty()
                {
                    fit_data.cap_consumers_all.insert(item_uid, cap_attr_rids.clone());
                }
                item_vs_ship_kind_add(
                    u_data,
                    fit_data,
                    item_uid,
                    module.get_category_id().unwrap(),
                    module.get_fit_uid(),
                );
            }
            UItem::Rig(rig) => {
                let item_axt = rig.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Rig);
                fit_data.rigs_rig_size.insert(item_uid, item_axt.rig_size);
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = rig.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if item_axt.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                item_vs_ship_kind_add(
                    u_data,
                    fit_data,
                    item_uid,
                    rig.get_category_id().unwrap(),
                    rig.get_fit_uid(),
                );
            }
            UItem::Service(service) => {
                let item_axt = service.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Service);
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = service.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if item_axt.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if let Some(max_fitted) = item_axt.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(service.get_type_aid(), item_uid, max_fitted);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                if let Some(sec_class) = item_axt.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_uid, sec_class);
                }
                item_vs_ship_kind_add(
                    u_data,
                    fit_data,
                    item_uid,
                    service.get_category_id().unwrap(),
                    service.get_fit_uid(),
                );
            }
            UItem::Ship(ship) => {
                let fit = u_data.fits.get(fit_uid);
                let item_axt = ship.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Ship);
                // If new ship limits drones which can be used, fill the mismatch data up
                if let Some(drone_limit) = &item_axt.drone_limit {
                    fit_data.drone_group_limit.extend(drone_limit.group_ids.iter());
                    for &drone_uid in fit.drones.iter() {
                        let drone_item = u_data.items.get(drone_uid);
                        // Not every drone is guaranteed to be loaded
                        if let Some(drone_group_aid) = drone_item.get_group_id()
                            && !drone_limit.group_ids.contains(&drone_group_aid)
                        {
                            fit_data.drone_groups.insert(drone_uid, drone_group_aid);
                        }
                    }
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                if ship.get_disallowed_in_wspace().unwrap() {
                    fit_data.sec_zone_fitted_wspace_banned.insert(item_uid);
                }
                // Ship/structure modules are not enforced when ship is not set. When we get one,
                // fill the data container up
                for item_uid in chain!(
                    fit.iter_module_uids(),
                    fit.rigs.iter().copied(),
                    fit.services.iter().copied(),
                ) {
                    let item = u_data.items.get(item_uid);
                    // Not every item is guaranteed to be loaded
                    if let Some(item_cat_id) = item.get_category_id() {
                        match item_cat_id {
                            AItemCatId::MODULE => {
                                if !matches!(fit.ship_kind, UShipKind::Ship) {
                                    fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_uid, ValShipKind::Ship);
                                }
                            }
                            AItemCatId::STRUCTURE_MODULE => {
                                if !matches!(fit.ship_kind, UShipKind::Structure) {
                                    fit_data
                                        .mods_rigs_svcs_vs_ship_kind
                                        .insert(item_uid, ValShipKind::Structure);
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
            UItem::Skill(skill) => {
                let item_axt = skill.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Skill);
            }
            UItem::Stance(stance) => {
                let item_axt = stance.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Stance);
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
            }
            UItem::Subsystem(subsystem) => {
                let item_axt = subsystem.get_axt().unwrap();
                item_kind_add(fit_data, item_uid, item_axt.kind, ItemKind::Subsystem);
                if let Some(a_slot) = subsystem.get_slot() {
                    fit_data.slotted_subsystems.add_entry(a_slot, item_uid);
                }
                if let Some(ship_limit) = &item_axt.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn item_unloaded(&mut self, item_uid: &UItemId, item: &UItem) {
        let fit_uid = match item.get_fit_uid() {
            Some(fit_uid) => fit_uid,
            None => return,
        };
        let fit_data = self.get_fit_data_mut(&fit_uid);
        // Skill requirements
        if let Some(a_srqs) = item.get_effective_skill_reqs()
            && !a_srqs.is_empty()
        {
            for &skill_item_aid in a_srqs.keys() {
                fit_data.srqs_skill_item_map.remove_entry(skill_item_aid, item_uid);
            }
            fit_data.srqs_missing.remove(item_uid);
        }
        match item {
            UItem::Booster(booster) => {
                let item_axt = booster.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Booster);
                if let Some(slot) = booster.get_slot() {
                    fit_data.slotted_boosters.remove_entry(slot, item_uid);
                }
            }
            UItem::Character(character) => {
                let item_axt = character.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Character);
            }
            UItem::Charge(charge) => {
                let item_axt = charge.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Charge);
                fit_data.charge_group.remove(item_uid);
                if item_axt.cont_limit.is_some() {
                    fit_data.charge_cont_group.remove(item_uid);
                }
                fit_data.charge_size.remove(item_uid);
                fit_data.charge_volume.remove(item_uid);
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_uid);
                }
            }
            UItem::Drone(drone) => {
                let item_axt = drone.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Drone);
                fit_data.drones_volume.remove(item_uid);
                if item_axt.bandwidth_use.is_some() {
                    fit_data.drones_bandwidth.remove(item_uid);
                }
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_groups.remove(item_uid);
                }
            }
            UItem::Fighter(fighter) => {
                let item_axt = fighter.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Fighter);
                fit_data.fighters_volume.remove(item_uid);
                let count = fighter.get_count().unwrap();
                if count.current > count.max {
                    fit_data.fighter_squad_size.remove(item_uid);
                }
                if item_axt.is_light_fighter {
                    fit_data.light_fighters.remove(item_uid);
                }
                if item_axt.is_heavy_fighter {
                    fit_data.heavy_fighters.remove(item_uid);
                }
                if item_axt.is_support_fighter {
                    fit_data.support_fighters.remove(item_uid);
                }
                if item_axt.is_st_light_fighter {
                    fit_data.st_light_fighters.remove(item_uid);
                }
                if item_axt.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.remove(item_uid);
                }
                if item_axt.is_st_support_fighter {
                    fit_data.st_support_fighters.remove(item_uid);
                }
            }
            UItem::Implant(implant) => {
                let item_axt = implant.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Implant);
                if let Some(slot) = implant.get_slot() {
                    fit_data.slotted_implants.remove_entry(slot, item_uid);
                }
            }
            UItem::Module(module) => {
                let item_axt = module.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, get_module_expected_kind(module));
                if module.takes_turret_hardpoint() {
                    fit_data.mods_turret.remove(item_uid);
                }
                if module.takes_launcher_hardpoint() {
                    fit_data.mods_launcher.remove(item_uid);
                }
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = module.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if item_axt.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if let Some(charge_uid) = module.get_charge_uid() {
                    if item_axt.charge_limit.is_some() {
                        fit_data.charge_group.remove(&charge_uid);
                    }
                    fit_data.charge_cont_group.remove(&charge_uid);
                    if item_axt.charge_size.is_some() {
                        fit_data.charge_size.remove(&charge_uid);
                    }
                    fit_data.charge_volume.remove(&charge_uid);
                }
                if let Some(RShipKind::CapitalShip) = item_axt.item_ship_kind {
                    fit_data.mods_capital.remove(item_uid);
                }
                if item_axt.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(module.get_type_aid(), item_uid);
                }
                if item_axt.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_uid);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
                if let Some(cap_attr_rids) = module.get_cap_use_attr_rids()
                    && !cap_attr_rids.is_empty()
                {
                    fit_data.cap_consumers_all.remove(item_uid);
                }
            }
            UItem::Rig(rig) => {
                let item_axt = rig.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Rig);
                fit_data.rigs_rig_size.remove(item_uid);
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = rig.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if item_axt.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
            }
            UItem::Service(service) => {
                let item_axt = service.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Service);
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = service.get_val_fitted_group_id() {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if item_axt.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if item_axt.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(service.get_type_aid(), item_uid);
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                if item_axt.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
            }
            UItem::Ship(ship) => {
                let item_axt = ship.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Ship);
                // If any drone group limits were defined, clear the mismatch data
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_group_limit.clear();
                    fit_data.drone_groups.clear();
                }
                if item_axt.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                if ship.get_disallowed_in_wspace().unwrap() {
                    fit_data.sec_zone_fitted_wspace_banned.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.clear();
            }
            UItem::Skill(skill) => {
                let item_axt = skill.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Skill);
            }
            UItem::Stance(stance) => {
                let item_axt = stance.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Stance);
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
            }
            UItem::Subsystem(subsystem) => {
                let item_axt = subsystem.get_axt().unwrap();
                item_kind_remove(fit_data, item_uid, item_axt.kind, ItemKind::Subsystem);
                if let Some(slot) = subsystem.get_slot() {
                    fit_data.slotted_subsystems.remove_entry(slot, item_uid);
                }
                if item_axt.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
            }
            _ => (),
        }
    }
}

fn get_module_expected_kind(module: &UModule) -> ItemKind {
    match module.get_rack() {
        ModRack::High => ItemKind::ModuleHigh,
        ModRack::Mid => ItemKind::ModuleMid,
        ModRack::Low => ItemKind::ModuleLow,
    }
}
fn item_kind_add(fit_data: &mut VastFitData, item_uid: UItemId, item_kind: Option<ItemKind>, expected_kind: ItemKind) {
    if item_kind != Some(expected_kind) {
        fit_data.item_kind.insert(
            item_uid,
            ValItemKindItemInfo {
                kind: item_kind,
                expected_kind,
            },
        );
    }
}
fn item_kind_remove(
    fit_data: &mut VastFitData,
    item_uid: &UItemId,
    item_kind: Option<ItemKind>,
    expected_kind: ItemKind,
) {
    if item_kind != Some(expected_kind) {
        fit_data.item_kind.remove(item_uid);
    }
}
fn item_vs_ship_kind_add(
    u_data: &UData,
    fit_data: &mut VastFitData,
    item_uid: UItemId,
    item_cat: AItemCatId,
    fit_uid: UFitId,
) {
    let fit = u_data.fits.get(fit_uid);
    let ship_uid = match fit.ship {
        Some(ship_id) => ship_id,
        None => return,
    };
    match item_cat {
        AItemCatId::MODULE => match fit.ship_kind {
            UShipKind::Ship => (),
            UShipKind::Structure => {
                fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_uid, ValShipKind::Ship);
            }
            UShipKind::Unknown => {
                let ship = u_data.items.get(ship_uid);
                if ship.is_loaded() {
                    fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_uid, ValShipKind::Ship);
                }
            }
        },
        AItemCatId::STRUCTURE_MODULE => match fit.ship_kind {
            UShipKind::Ship => {
                fit_data
                    .mods_rigs_svcs_vs_ship_kind
                    .insert(item_uid, ValShipKind::Structure);
            }
            UShipKind::Structure => (),
            UShipKind::Unknown => {
                let ship = u_data.items.get(ship_uid);
                if ship.is_loaded() {
                    fit_data
                        .mods_rigs_svcs_vs_ship_kind
                        .insert(item_uid, ValShipKind::Structure);
                }
            }
        },
        _ => (),
    }
}

fn handle_charge_group_add(
    fit_data: &mut VastFitData,
    cont_uid: UItemId,
    cont_axt: &RItemAXt,
    charge_uid: UItemId,
    charge_group_aid: &AItemGrpId,
) {
    if let Some(charge_limit) = &cont_axt.charge_limit
        && !charge_limit.group_ids.contains(charge_group_aid)
    {
        fit_data.charge_group.insert(charge_uid, cont_uid);
    }
}

fn handle_charge_cont_group_add(
    fit_data: &mut VastFitData,
    cont_uid: UItemId,
    cont_group_aid: &AItemGrpId,
    charge_uid: UItemId,
    charge_axt: &RItemAXt,
) {
    if let Some(charge_cont_limit) = &charge_axt.cont_limit
        && !charge_cont_limit.group_ids.contains(cont_group_aid)
    {
        fit_data.charge_cont_group.insert(charge_uid, cont_uid);
    }
}

fn handle_charge_size_add(
    fit_data: &mut VastFitData,
    cont_uid: UItemId,
    cont_axt: &RItemAXt,
    charge_uid: UItemId,
    charge_axt: &RItemAXt,
) {
    // Charge size mismatch happens when parent module requires some charge size
    if cont_axt.charge_size.is_some() && cont_axt.charge_size != charge_axt.charge_size {
        fit_data.charge_size.insert(charge_uid, cont_uid);
    }
}

fn handle_charge_volume_add(
    fit_data: &mut VastFitData,
    cont_uid: UItemId,
    cont_axt: &RItemAXt,
    charge_uid: UItemId,
    charge_axt: &RItemAXt,
) {
    if cont_axt.capacity < charge_axt.volume {
        fit_data.charge_volume.insert(charge_uid, cont_uid);
    }
}
