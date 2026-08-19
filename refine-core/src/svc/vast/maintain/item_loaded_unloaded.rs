use itertools::chain;

use crate::{
    Count, ModRack,
    ad::{AItemCatId, AItemGrpId},
    misc::DetectedItemKind,
    rd::{RItemAttrData, RShipKind},
    svc::{
        Vast,
        vast::{
            ValShipKind, VastFitData,
            val::{ValFighterSquadSizeFighterStored, ValItemKindItemStored, ValSrqSkillStored},
        },
    },
    ud::{UData, UFitId, UItem, UItemId, UModule, UShipKind},
    util::RMap,
};

impl Vast {
    pub(in crate::svc) fn item_loaded(&mut self, u_data: &UData, item_uid: UItemId, item: &UItem) {
        let Some(fit_uid) = item.get_fit_uid() else {
            return;
        };
        let fit_data = self.get_fit_data_mut(fit_uid);
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
                        ValSrqSkillStored {
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
                let booster_riad = booster.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, booster_riad.kind, DetectedItemKind::Booster);
                if let Some(slot) = booster_riad.booster_slot {
                    fit_data.slotted_boosters.add_entry(slot, item_uid);
                }
            }
            UItem::Character(character) => {
                let character_riad = character.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, character_riad.kind, DetectedItemKind::Character);
            }
            UItem::Charge(charge) => {
                let charge_rib = charge.get_r_item_base().unwrap();
                let charge_riad = charge.get_r_item_attr_data().unwrap();
                let cont_uid = charge.get_cont_item_uid();
                let cont_item = u_data.items.get(cont_uid);
                item_kind_add(fit_data, item_uid, charge_riad.kind, DetectedItemKind::Charge);
                if let (Some(cont_rib), Some(cont_riad)) =
                    (cont_item.get_r_item_base(), cont_item.get_r_item_attr_data())
                {
                    handle_charge_group_add(fit_data, cont_uid, cont_riad, item_uid, &charge_rib.grp_id);
                    handle_charge_cont_group_add(fit_data, cont_uid, &cont_rib.grp_id, item_uid, charge_riad);
                    handle_charge_size_add(fit_data, cont_uid, cont_riad, item_uid, charge_riad);
                    handle_charge_volume_add(fit_data, cont_uid, cont_riad, item_uid, charge_riad);
                }
                if charge_riad.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_uid);
                }
            }
            UItem::Drone(drone) => {
                let drone_riad = drone.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, drone_riad.kind, DetectedItemKind::Drone);
                fit_data.drones_volume.insert(item_uid, drone_riad.volume);
                if let Some(bandwidth) = drone_riad.bandwidth_use {
                    fit_data.drones_bandwidth.insert(item_uid, bandwidth);
                };
                if !fit_data.drone_group_limit.is_empty() {
                    let drone_rib = drone.get_r_item_base().unwrap();
                    if !fit_data.drone_group_limit.contains(&drone_rib.grp_id) {
                        fit_data.drone_groups.insert(item_uid, drone_rib.grp_id);
                    }
                }
            }
            UItem::Fighter(fighter) => {
                let fighter_riad = fighter.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, fighter_riad.kind, DetectedItemKind::Fighter);
                let count = fighter.get_count_info().unwrap();
                fit_data
                    .fighters_volume
                    .insert(item_uid, fighter_riad.volume * count.current.into_pvalue());
                if count.current > count.max {
                    fit_data.fighter_squad_size.insert(
                        item_uid,
                        ValFighterSquadSizeFighterStored {
                            size: count.current,
                            max_size: count.max,
                        },
                    );
                }
                if fighter_riad.is_light_fighter {
                    fit_data.light_fighters.insert(item_uid);
                }
                if fighter_riad.is_heavy_fighter {
                    fit_data.heavy_fighters.insert(item_uid);
                }
                if fighter_riad.is_support_fighter {
                    fit_data.support_fighters.insert(item_uid);
                }
                if fighter_riad.is_st_light_fighter {
                    fit_data.st_light_fighters.insert(item_uid);
                }
                if fighter_riad.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.insert(item_uid);
                }
                if fighter_riad.is_st_support_fighter {
                    fit_data.st_support_fighters.insert(item_uid);
                }
            }
            UItem::Implant(implant) => {
                let implant_riad = implant.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, implant_riad.kind, DetectedItemKind::Implant);
                if let Some(slot) = implant_riad.implant_slot {
                    fit_data.slotted_implants.add_entry(slot, item_uid);
                }
            }
            UItem::Module(module) => {
                let module_rib = module.get_r_item_base().unwrap();
                let module_riad = module.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, module_riad.kind, get_module_expected_kind(module));
                if module_rib.takes_turret_hardpoint {
                    fit_data.mods_turret.insert(item_uid);
                }
                if module_rib.takes_launcher_hardpoint {
                    fit_data.mods_launcher.insert(item_uid);
                }
                if module_rib.is_cloak {
                    fit_data.mods_fitted_cloaks += Count::ONE;
                }
                if let Some(ship_limit) = &module_riad.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = module_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if module_riad.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if let Some(charge_uid) = module.get_charge_uid() {
                    let charge_item = u_data.items.get(charge_uid);
                    if let (Some(charge_rib), Some(charge_riad)) =
                        (charge_item.get_r_item_base(), charge_item.get_r_item_attr_data())
                    {
                        handle_charge_group_add(fit_data, item_uid, module_riad, charge_uid, &charge_rib.grp_id);
                        handle_charge_cont_group_add(fit_data, item_uid, &module_rib.grp_id, charge_uid, charge_riad);
                        handle_charge_size_add(fit_data, item_uid, module_riad, charge_uid, charge_riad);
                        handle_charge_volume_add(fit_data, item_uid, module_riad, charge_uid, charge_riad);
                    }
                }
                if let Some(max_fitted) = module_riad.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(module.get_type_aid(), item_uid, max_fitted);
                }
                if let Some(RShipKind::CapitalShip) = module_riad.item_ship_kind {
                    fit_data.mods_capital.insert(item_uid, module_riad.volume);
                }
                if let Some(sec_class) = module_riad.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_uid, sec_class);
                }
                if module_riad.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_uid);
                }
                if !module_rib.cap_consumers.is_empty() {
                    fit_data.mods_cap_consumers.insert(item_uid);
                }
                item_vs_ship_kind_add(u_data, fit_data, item_uid, module_rib.cat_id, module.get_fit_uid());
            }
            UItem::Rig(rig) => {
                let rig_rib = rig.get_r_item_base().unwrap();
                let rig_riad = rig.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, rig_riad.kind, DetectedItemKind::Rig);
                fit_data.rigs_rig_size.insert(item_uid, rig_riad.rig_size);
                if let Some(ship_limit) = &rig_riad.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = rig_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if rig_riad.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if rig_riad.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                item_vs_ship_kind_add(u_data, fit_data, item_uid, rig_rib.cat_id, rig.get_fit_uid());
            }
            UItem::Service(service) => {
                let service_rib = service.get_r_item_base().unwrap();
                let service_riad = service.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, service_riad.kind, DetectedItemKind::Service);
                if let Some(ship_limit) = &service_riad.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = service_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if service_riad.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if let Some(max_fitted) = service_riad.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(service.get_type_aid(), item_uid, max_fitted);
                }
                if service_riad.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                if let Some(sec_class) = service_riad.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_uid, sec_class);
                }
                item_vs_ship_kind_add(u_data, fit_data, item_uid, service_rib.cat_id, service.get_fit_uid());
            }
            UItem::Ship(ship) => {
                let fit = u_data.fits.get(fit_uid);
                let ship_rib = ship.get_r_item_base().unwrap();
                let ship_riad = ship.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, ship_riad.kind, DetectedItemKind::Ship);
                // If new ship limits drones which can be used, fill the mismatch data up
                if let Some(drone_limit) = &ship_riad.drone_limit {
                    fit_data.drone_group_limit.extend(drone_limit.group_ids.iter());
                    for &drone_uid in fit.drones.iter() {
                        let drone_item = u_data.items.get(drone_uid);
                        // Not every drone is guaranteed to be loaded
                        if let Some(drone_rib) = drone_item.get_r_item_base()
                            && !drone_limit.group_ids.contains(&drone_rib.grp_id)
                        {
                            fit_data.drone_groups.insert(drone_uid, drone_rib.grp_id);
                        }
                    }
                }
                if ship_riad.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                if ship_rib.disallowed_in_wspace {
                    fit_data.sec_zone_fitted_wspace_banned.insert(item_uid);
                }
                if ship_riad.enables_conduit {
                    fit_data.conduit_enablers.insert(item_uid);
                }
                // Ship/structure modules are not enforced when ship is not set. When we get one,
                // fill the data container up
                for item_uid in chain!(
                    fit.iter_module_uids(),
                    fit.rigs.iter().copied(),
                    fit.services.iter().copied(),
                ) {
                    let child_item = u_data.items.get(item_uid);
                    // Not every item is guaranteed to be loaded
                    if let Some(child_item_rib) = child_item.get_r_item_base() {
                        match child_item_rib.cat_id {
                            AItemCatId::MODULE if !matches!(fit.ship_kind, UShipKind::Ship) => {
                                fit_data.mods_rigs_svcs_vs_ship_kind.insert(item_uid, ValShipKind::Ship);
                            }
                            AItemCatId::STRUCTURE_MODULE if !matches!(fit.ship_kind, UShipKind::Structure) => {
                                fit_data
                                    .mods_rigs_svcs_vs_ship_kind
                                    .insert(item_uid, ValShipKind::Structure);
                            }
                            _ => (),
                        }
                    }
                }
            }
            UItem::Skill(skill) => {
                let skill_riad = skill.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, skill_riad.kind, DetectedItemKind::Skill);
            }
            UItem::Stance(stance) => {
                let stance_riad = stance.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, stance_riad.kind, DetectedItemKind::Stance);
                if let Some(ship_limit) = &stance_riad.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
            }
            UItem::Subsystem(subsystem) => {
                let subsystem_riad = subsystem.get_r_item_attr_data().unwrap();
                item_kind_add(fit_data, item_uid, subsystem_riad.kind, DetectedItemKind::Subsystem);
                if let Some(slot) = subsystem_riad.subsystem_slot {
                    fit_data.slotted_subsystems.add_entry(slot, item_uid);
                }
                if let Some(ship_limit) = &subsystem_riad.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn item_unloaded(&mut self, item_uid: &UItemId, item: &UItem) {
        let Some(fit_uid) = item.get_fit_uid() else {
            return;
        };
        let fit_data = self.get_fit_data_mut(fit_uid);
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
                let booster_riad = booster.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, booster_riad.kind, DetectedItemKind::Booster);
                if let Some(slot) = booster_riad.booster_slot {
                    fit_data.slotted_boosters.remove_entry(slot, item_uid);
                }
            }
            UItem::Character(character) => {
                let character_riad = character.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, character_riad.kind, DetectedItemKind::Character);
            }
            UItem::Charge(charge) => {
                let charge_riad = charge.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, charge_riad.kind, DetectedItemKind::Charge);
                fit_data.charge_group.remove(item_uid);
                if charge_riad.cont_limit.is_some() {
                    fit_data.charge_cont_group.remove(item_uid);
                }
                fit_data.charge_size.remove(item_uid);
                fit_data.charge_volume.remove(item_uid);
                if charge_riad.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_uid);
                }
            }
            UItem::Drone(drone) => {
                let drone_riad = drone.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, drone_riad.kind, DetectedItemKind::Drone);
                fit_data.drones_volume.remove(item_uid);
                if drone_riad.bandwidth_use.is_some() {
                    fit_data.drones_bandwidth.remove(item_uid);
                }
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_groups.remove(item_uid);
                }
            }
            UItem::Fighter(fighter) => {
                let fighter_riad = fighter.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, fighter_riad.kind, DetectedItemKind::Fighter);
                fit_data.fighters_volume.remove(item_uid);
                let count = fighter.get_count_info().unwrap();
                if count.current > count.max {
                    fit_data.fighter_squad_size.remove(item_uid);
                }
                if fighter_riad.is_light_fighter {
                    fit_data.light_fighters.remove(item_uid);
                }
                if fighter_riad.is_heavy_fighter {
                    fit_data.heavy_fighters.remove(item_uid);
                }
                if fighter_riad.is_support_fighter {
                    fit_data.support_fighters.remove(item_uid);
                }
                if fighter_riad.is_st_light_fighter {
                    fit_data.st_light_fighters.remove(item_uid);
                }
                if fighter_riad.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.remove(item_uid);
                }
                if fighter_riad.is_st_support_fighter {
                    fit_data.st_support_fighters.remove(item_uid);
                }
            }
            UItem::Implant(implant) => {
                let implant_riad = implant.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, implant_riad.kind, DetectedItemKind::Implant);
                if let Some(slot) = implant_riad.implant_slot {
                    fit_data.slotted_implants.remove_entry(slot, item_uid);
                }
            }
            UItem::Module(module) => {
                let module_rib = module.get_r_item_base().unwrap();
                let module_riad = module.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, module_riad.kind, get_module_expected_kind(module));
                if module_rib.takes_turret_hardpoint {
                    fit_data.mods_turret.remove(item_uid);
                }
                if module_rib.takes_launcher_hardpoint {
                    fit_data.mods_launcher.remove(item_uid);
                }
                if module_rib.is_cloak {
                    fit_data.mods_fitted_cloaks -= Count::ONE;
                }
                if module_riad.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = module_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if module_riad.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if let Some(charge_uid) = module.get_charge_uid() {
                    if module_riad.charge_limit.is_some() {
                        fit_data.charge_group.remove(&charge_uid);
                    }
                    fit_data.charge_cont_group.remove(&charge_uid);
                    if module_riad.charge_size.is_some() {
                        fit_data.charge_size.remove(&charge_uid);
                    }
                    fit_data.charge_volume.remove(&charge_uid);
                }
                if let Some(RShipKind::CapitalShip) = module_riad.item_ship_kind {
                    fit_data.mods_capital.remove(item_uid);
                }
                if module_riad.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(module.get_type_aid(), item_uid);
                }
                if module_riad.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_uid);
                }
                if module_riad.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
                if !module_rib.cap_consumers.is_empty() {
                    fit_data.mods_cap_consumers.remove(item_uid);
                }
            }
            UItem::Rig(rig) => {
                let rig_rib = rig.get_r_item_base().unwrap();
                let rig_riad = rig.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, rig_riad.kind, DetectedItemKind::Rig);
                fit_data.rigs_rig_size.remove(item_uid);
                if rig_riad.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = rig_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if rig_riad.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if rig_riad.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
            }
            UItem::Service(service) => {
                let service_rib = service.get_r_item_base().unwrap();
                let service_riad = service.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, service_riad.kind, DetectedItemKind::Service);
                if service_riad.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = service_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if service_riad.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if service_riad.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(service.get_type_aid(), item_uid);
                }
                if service_riad.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                if service_riad.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
            }
            UItem::Ship(ship) => {
                let ship_rib = ship.get_r_item_base().unwrap();
                let ship_riad = ship.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, ship_riad.kind, DetectedItemKind::Ship);
                // If any drone group limits were defined, clear the mismatch data
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_group_limit.clear();
                    fit_data.drone_groups.clear();
                }
                if ship_riad.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                if ship_rib.disallowed_in_wspace {
                    fit_data.sec_zone_fitted_wspace_banned.remove(item_uid);
                }
                if ship_riad.enables_conduit {
                    fit_data.conduit_enablers.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.clear();
            }
            UItem::Skill(skill) => {
                let skill_riad = skill.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, skill_riad.kind, DetectedItemKind::Skill);
            }
            UItem::Stance(stance) => {
                let item_riad = stance.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, item_riad.kind, DetectedItemKind::Stance);
                if item_riad.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
            }
            UItem::Subsystem(subsystem) => {
                let subsystem_riad = subsystem.get_r_item_attr_data().unwrap();
                item_kind_remove(fit_data, item_uid, subsystem_riad.kind, DetectedItemKind::Subsystem);
                if let Some(slot) = subsystem_riad.subsystem_slot {
                    fit_data.slotted_subsystems.remove_entry(slot, item_uid);
                }
                if subsystem_riad.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
            }
            _ => (),
        }
    }
}

fn get_module_expected_kind(module: &UModule) -> DetectedItemKind {
    match module.get_rack() {
        ModRack::High => DetectedItemKind::ModuleHigh,
        ModRack::Mid => DetectedItemKind::ModuleMid,
        ModRack::Low => DetectedItemKind::ModuleLow,
    }
}
fn item_kind_add(
    fit_data: &mut VastFitData,
    item_uid: UItemId,
    item_kind: Option<DetectedItemKind>,
    expected_kind: DetectedItemKind,
) {
    if item_kind != Some(expected_kind) {
        fit_data.item_kind.insert(
            item_uid,
            ValItemKindItemStored {
                kind: item_kind,
                expected_kind,
            },
        );
    }
}
fn item_kind_remove(
    fit_data: &mut VastFitData,
    item_uid: &UItemId,
    item_kind: Option<DetectedItemKind>,
    expected_kind: DetectedItemKind,
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
    let Some(ship_uid) = fit.ship else {
        return;
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
    cont_riad: &RItemAttrData,
    charge_uid: UItemId,
    charge_group_aid: &AItemGrpId,
) {
    if let Some(charge_limit) = &cont_riad.charge_limit
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
    charge_riad: &RItemAttrData,
) {
    if let Some(charge_cont_limit) = &charge_riad.cont_limit
        && !charge_cont_limit.group_ids.contains(cont_group_aid)
    {
        fit_data.charge_cont_group.insert(charge_uid, cont_uid);
    }
}

fn handle_charge_size_add(
    fit_data: &mut VastFitData,
    cont_uid: UItemId,
    cont_riad: &RItemAttrData,
    charge_uid: UItemId,
    charge_riad: &RItemAttrData,
) {
    // Charge size mismatch happens when parent module requires some charge size
    if cont_riad.charge_size.is_some() && cont_riad.charge_size != charge_riad.charge_size {
        fit_data.charge_size.insert(charge_uid, cont_uid);
    }
}

fn handle_charge_volume_add(
    fit_data: &mut VastFitData,
    cont_uid: UItemId,
    cont_riad: &RItemAttrData,
    charge_uid: UItemId,
    charge_riad: &RItemAttrData,
) {
    if cont_riad.capacity < charge_riad.volume {
        fit_data.charge_volume.insert(charge_uid, cont_uid);
    }
}
