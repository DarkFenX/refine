use itertools::chain;

use crate::{
    Count, ModRack,
    ad::{AItemCatId, AItemGrpId},
    misc::DetectedItemKind,
    rd::{RItemBase, RItemFlexData, RShipKind},
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
                let booster_rifd = booster.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, booster_rifd.kind, DetectedItemKind::Booster);
                if let Some(slot) = booster_rifd.booster_slot {
                    fit_data.slotted_boosters.add_entry(slot, item_uid);
                }
            }
            UItem::Character(character) => {
                let character_rifd = character.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, character_rifd.kind, DetectedItemKind::Character);
            }
            UItem::Charge(charge) => {
                let charge_rib = charge.get_r_item_base().unwrap();
                let charge_rifd = charge.get_r_item_flex_data().unwrap();
                let cont_uid = charge.get_cont_item_uid();
                let cont_item = u_data.items.get(cont_uid);
                item_kind_add(fit_data, item_uid, charge_rifd.kind, DetectedItemKind::Charge);
                if let (Some(cont_rib), Some(cont_rifd)) =
                    (cont_item.get_r_item_base(), cont_item.get_r_item_flex_data())
                {
                    handle_charge_group_add(fit_data, cont_uid, cont_rifd, item_uid, &charge_rib.grp_id);
                    handle_charge_cont_group_add(fit_data, cont_uid, &cont_rib.grp_id, item_uid, charge_rifd);
                    handle_charge_size_add(fit_data, cont_uid, cont_rifd, item_uid, charge_rifd);
                    handle_charge_volume_add(fit_data, cont_uid, cont_rib, item_uid, charge_rib);
                }
                if charge_rifd.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_uid);
                }
            }
            UItem::Drone(drone) => {
                let drone_rib = drone.get_r_item_base().unwrap();
                let drone_rifd = drone.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, drone_rifd.kind, DetectedItemKind::Drone);
                fit_data.drones_volume.insert(item_uid, drone_rib.volume);
                if let Some(bandwidth) = drone_rifd.bandwidth_use {
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
                let fighter_rib = fighter.get_r_item_base().unwrap();
                let fighter_rifd = fighter.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, fighter_rifd.kind, DetectedItemKind::Fighter);
                let count = fighter.get_count_info().unwrap();
                fit_data
                    .fighters_volume
                    .insert(item_uid, fighter_rib.volume * count.current.into_pvalue());
                if count.current > count.max {
                    fit_data.fighter_squad_size.insert(
                        item_uid,
                        ValFighterSquadSizeFighterStored {
                            size: count.current,
                            max_size: count.max,
                        },
                    );
                }
                if fighter_rib.is_light_fighter {
                    fit_data.light_fighters.insert(item_uid);
                }
                if fighter_rib.is_heavy_fighter {
                    fit_data.heavy_fighters.insert(item_uid);
                }
                if fighter_rib.is_support_fighter {
                    fit_data.support_fighters.insert(item_uid);
                }
                if fighter_rib.is_st_light_fighter {
                    fit_data.st_light_fighters.insert(item_uid);
                }
                if fighter_rib.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.insert(item_uid);
                }
                if fighter_rib.is_st_support_fighter {
                    fit_data.st_support_fighters.insert(item_uid);
                }
            }
            UItem::Implant(implant) => {
                let implant_rifd = implant.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, implant_rifd.kind, DetectedItemKind::Implant);
                if let Some(slot) = implant_rifd.implant_slot {
                    fit_data.slotted_implants.add_entry(slot, item_uid);
                }
            }
            UItem::Module(module) => {
                let module_rib = module.get_r_item_base().unwrap();
                let module_rifd = module.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, module_rifd.kind, get_module_expected_kind(module));
                if module_rib.takes_turret_hardpoint {
                    fit_data.mods_turret.insert(item_uid);
                }
                if module_rib.takes_launcher_hardpoint {
                    fit_data.mods_launcher.insert(item_uid);
                }
                if module_rib.is_cloak {
                    fit_data.mods_fitted_cloaks += Count::ONE;
                }
                if let Some(ship_limit) = &module_rifd.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = module_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if module_rifd.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if let Some(charge_uid) = module.get_charge_uid() {
                    let charge_item = u_data.items.get(charge_uid);
                    if let (Some(charge_rib), Some(charge_rifd)) =
                        (charge_item.get_r_item_base(), charge_item.get_r_item_flex_data())
                    {
                        handle_charge_group_add(fit_data, item_uid, module_rifd, charge_uid, &charge_rib.grp_id);
                        handle_charge_cont_group_add(fit_data, item_uid, &module_rib.grp_id, charge_uid, charge_rifd);
                        handle_charge_size_add(fit_data, item_uid, module_rifd, charge_uid, charge_rifd);
                        handle_charge_volume_add(fit_data, item_uid, module_rib, charge_uid, charge_rib);
                    }
                }
                if let Some(max_fitted) = module_rifd.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(module.get_type_aid(), item_uid, max_fitted);
                }
                if let Some(RShipKind::CapitalShip) = module_rib.item_ship_kind {
                    fit_data.mods_capital.insert(item_uid, module_rib.volume);
                }
                if let Some(sec_class) = module_rifd.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_uid, sec_class);
                }
                if module_rifd.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.insert(item_uid);
                }
                if !module_rib.cap_consumers.is_empty() {
                    fit_data.mods_cap_consumers.insert(item_uid);
                }
                item_vs_ship_kind_add(u_data, fit_data, item_uid, module_rib.cat_id, module.get_fit_uid());
            }
            UItem::Rig(rig) => {
                let rig_rib = rig.get_r_item_base().unwrap();
                let rig_rifd = rig.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, rig_rifd.kind, DetectedItemKind::Rig);
                fit_data.rigs_rig_size.insert(item_uid, rig_rifd.rig_size);
                if let Some(ship_limit) = &rig_rifd.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = rig_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if rig_rifd.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if rig_rifd.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                item_vs_ship_kind_add(u_data, fit_data, item_uid, rig_rib.cat_id, rig.get_fit_uid());
            }
            UItem::Service(service) => {
                let service_rib = service.get_r_item_base().unwrap();
                let service_rifd = service.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, service_rifd.kind, DetectedItemKind::Service);
                if let Some(ship_limit) = &service_rifd.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
                if let Some(item_grp_aid) = service_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .add_entry(item_grp_aid, item_uid);
                    if service_rifd.max_group_fitted_limited {
                        fit_data
                            .mods_svcs_rigs_max_group_fitted_limited
                            .insert(item_uid, item_grp_aid);
                    }
                }
                if let Some(max_fitted) = service_rifd.max_type_fitted {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .add_entry(service.get_type_aid(), item_uid, max_fitted);
                }
                if service_rifd.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                if let Some(sec_class) = service_rifd.online_max_sec_class {
                    fit_data.sec_zone_unonlineable_class.insert(item_uid, sec_class);
                }
                item_vs_ship_kind_add(u_data, fit_data, item_uid, service_rib.cat_id, service.get_fit_uid());
            }
            UItem::Ship(ship) => {
                let fit = u_data.fits.get(fit_uid);
                let ship_rib = ship.get_r_item_base().unwrap();
                let ship_rifd = ship.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, ship_rifd.kind, DetectedItemKind::Ship);
                // If new ship limits drones which can be used, fill the mismatch data up
                if let Some(drone_limit) = &ship_rifd.drone_limit {
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
                if ship_rifd.sec_zone_limitable {
                    fit_data.sec_zone_fitted.insert(item_uid);
                }
                if ship_rib.disallowed_in_wspace {
                    fit_data.sec_zone_fitted_wspace_banned.insert(item_uid);
                }
                if ship_rifd.enables_conduit {
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
                let skill_rifd = skill.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, skill_rifd.kind, DetectedItemKind::Skill);
            }
            UItem::Stance(stance) => {
                let stance_rifd = stance.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, stance_rifd.kind, DetectedItemKind::Stance);
                if let Some(ship_limit) = &stance_rifd.ship_limit {
                    fit_data.ship_limited_items.insert(item_uid, ship_limit.clone());
                }
            }
            UItem::Subsystem(subsystem) => {
                let subsystem_rifd = subsystem.get_r_item_flex_data().unwrap();
                item_kind_add(fit_data, item_uid, subsystem_rifd.kind, DetectedItemKind::Subsystem);
                if let Some(slot) = subsystem_rifd.subsystem_slot {
                    fit_data.slotted_subsystems.add_entry(slot, item_uid);
                }
                if let Some(ship_limit) = &subsystem_rifd.ship_limit {
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
                let booster_rifd = booster.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, booster_rifd.kind, DetectedItemKind::Booster);
                if let Some(slot) = booster_rifd.booster_slot {
                    fit_data.slotted_boosters.remove_entry(slot, item_uid);
                }
            }
            UItem::Character(character) => {
                let character_rifd = character.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, character_rifd.kind, DetectedItemKind::Character);
            }
            UItem::Charge(charge) => {
                let charge_rifd = charge.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, charge_rifd.kind, DetectedItemKind::Charge);
                fit_data.charge_group.remove(item_uid);
                if charge_rifd.cont_limit.is_some() {
                    fit_data.charge_cont_group.remove(item_uid);
                }
                fit_data.charge_size.remove(item_uid);
                fit_data.charge_volume.remove(item_uid);
                if charge_rifd.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_uid);
                }
            }
            UItem::Drone(drone) => {
                let drone_rifd = drone.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, drone_rifd.kind, DetectedItemKind::Drone);
                fit_data.drones_volume.remove(item_uid);
                if drone_rifd.bandwidth_use.is_some() {
                    fit_data.drones_bandwidth.remove(item_uid);
                }
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_groups.remove(item_uid);
                }
            }
            UItem::Fighter(fighter) => {
                let fighter_rib = fighter.get_r_item_base().unwrap();
                let fighter_rifd = fighter.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, fighter_rifd.kind, DetectedItemKind::Fighter);
                fit_data.fighters_volume.remove(item_uid);
                let count = fighter.get_count_info().unwrap();
                if count.current > count.max {
                    fit_data.fighter_squad_size.remove(item_uid);
                }
                if fighter_rib.is_light_fighter {
                    fit_data.light_fighters.remove(item_uid);
                }
                if fighter_rib.is_heavy_fighter {
                    fit_data.heavy_fighters.remove(item_uid);
                }
                if fighter_rib.is_support_fighter {
                    fit_data.support_fighters.remove(item_uid);
                }
                if fighter_rib.is_st_light_fighter {
                    fit_data.st_light_fighters.remove(item_uid);
                }
                if fighter_rib.is_st_heavy_fighter {
                    fit_data.st_heavy_fighters.remove(item_uid);
                }
                if fighter_rib.is_st_support_fighter {
                    fit_data.st_support_fighters.remove(item_uid);
                }
            }
            UItem::Implant(implant) => {
                let implant_rifd = implant.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, implant_rifd.kind, DetectedItemKind::Implant);
                if let Some(slot) = implant_rifd.implant_slot {
                    fit_data.slotted_implants.remove_entry(slot, item_uid);
                }
            }
            UItem::Module(module) => {
                let module_rib = module.get_r_item_base().unwrap();
                let module_rifd = module.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, module_rifd.kind, get_module_expected_kind(module));
                if module_rib.takes_turret_hardpoint {
                    fit_data.mods_turret.remove(item_uid);
                }
                if module_rib.takes_launcher_hardpoint {
                    fit_data.mods_launcher.remove(item_uid);
                }
                if module_rib.is_cloak {
                    fit_data.mods_fitted_cloaks -= Count::ONE;
                }
                if module_rifd.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = module_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if module_rifd.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if let Some(charge_uid) = module.get_charge_uid() {
                    if module_rifd.charge_limit.is_some() {
                        fit_data.charge_group.remove(&charge_uid);
                    }
                    fit_data.charge_cont_group.remove(&charge_uid);
                    if module_rifd.charge_size.is_some() {
                        fit_data.charge_size.remove(&charge_uid);
                    }
                    fit_data.charge_volume.remove(&charge_uid);
                }
                if let Some(RShipKind::CapitalShip) = module_rib.item_ship_kind {
                    fit_data.mods_capital.remove(item_uid);
                }
                if module_rifd.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(module.get_type_aid(), item_uid);
                }
                if module_rifd.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_uid);
                }
                if module_rifd.sec_zone_limitable {
                    fit_data.sec_zone_unactivable.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
                if !module_rib.cap_consumers.is_empty() {
                    fit_data.mods_cap_consumers.remove(item_uid);
                }
            }
            UItem::Rig(rig) => {
                let rig_rib = rig.get_r_item_base().unwrap();
                let rig_rifd = rig.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, rig_rifd.kind, DetectedItemKind::Rig);
                fit_data.rigs_rig_size.remove(item_uid);
                if rig_rifd.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = rig_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if rig_rifd.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if rig_rifd.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
            }
            UItem::Service(service) => {
                let service_rib = service.get_r_item_base().unwrap();
                let service_rifd = service.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, service_rifd.kind, DetectedItemKind::Service);
                if service_rifd.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
                if let Some(item_grp_aid) = service_rib.val_fitted_group_id {
                    fit_data
                        .mods_svcs_rigs_max_group_fitted_all
                        .remove_entry(item_grp_aid, item_uid);
                    if service_rifd.max_group_fitted_limited {
                        fit_data.mods_svcs_rigs_max_group_fitted_limited.remove(item_uid);
                    }
                }
                if service_rifd.max_type_fitted.is_some() {
                    fit_data
                        .mods_svcs_max_type_fitted
                        .remove_l2(service.get_type_aid(), item_uid);
                }
                if service_rifd.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                if service_rifd.online_max_sec_class.is_some() {
                    fit_data.sec_zone_unonlineable_class.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.remove(item_uid);
            }
            UItem::Ship(ship) => {
                let ship_rib = ship.get_r_item_base().unwrap();
                let ship_rifd = ship.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, ship_rifd.kind, DetectedItemKind::Ship);
                // If any drone group limits were defined, clear the mismatch data
                if !fit_data.drone_group_limit.is_empty() {
                    fit_data.drone_group_limit.clear();
                    fit_data.drone_groups.clear();
                }
                if ship_rifd.sec_zone_limitable {
                    fit_data.sec_zone_fitted.remove(item_uid);
                }
                if ship_rib.disallowed_in_wspace {
                    fit_data.sec_zone_fitted_wspace_banned.remove(item_uid);
                }
                if ship_rifd.enables_conduit {
                    fit_data.conduit_enablers.remove(item_uid);
                }
                fit_data.mods_rigs_svcs_vs_ship_kind.clear();
            }
            UItem::Skill(skill) => {
                let skill_rifd = skill.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, skill_rifd.kind, DetectedItemKind::Skill);
            }
            UItem::Stance(stance) => {
                let item_rifd = stance.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, item_rifd.kind, DetectedItemKind::Stance);
                if item_rifd.ship_limit.is_some() {
                    fit_data.ship_limited_items.remove(item_uid);
                }
            }
            UItem::Subsystem(subsystem) => {
                let subsystem_rifd = subsystem.get_r_item_flex_data().unwrap();
                item_kind_remove(fit_data, item_uid, subsystem_rifd.kind, DetectedItemKind::Subsystem);
                if let Some(slot) = subsystem_rifd.subsystem_slot {
                    fit_data.slotted_subsystems.remove_entry(slot, item_uid);
                }
                if subsystem_rifd.ship_limit.is_some() {
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
    cont_rifd: &RItemFlexData,
    charge_uid: UItemId,
    charge_group_aid: &AItemGrpId,
) {
    if let Some(charge_limit) = &cont_rifd.charge_limit
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
    charge_rifd: &RItemFlexData,
) {
    if let Some(charge_cont_limit) = &charge_rifd.cont_limit
        && !charge_cont_limit.group_ids.contains(cont_group_aid)
    {
        fit_data.charge_cont_group.insert(charge_uid, cont_uid);
    }
}

fn handle_charge_size_add(
    fit_data: &mut VastFitData,
    cont_uid: UItemId,
    cont_rifd: &RItemFlexData,
    charge_uid: UItemId,
    charge_rifd: &RItemFlexData,
) {
    // Charge size mismatch happens when parent module requires some charge size
    if cont_rifd.charge_size.is_some() && cont_rifd.charge_size != charge_rifd.charge_size {
        fit_data.charge_size.insert(charge_uid, cont_uid);
    }
}

fn handle_charge_volume_add(
    fit_data: &mut VastFitData,
    cont_uid: UItemId,
    cont_rib: &RItemBase,
    charge_uid: UItemId,
    charge_rib: &RItemBase,
) {
    if cont_rib.capacity < charge_rib.volume {
        fit_data.charge_volume.insert(charge_uid, cont_uid);
    }
}
