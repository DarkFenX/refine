use crate::{
    dbg::DebugResult,
    misc::ModRack,
    ud::{UData, UFighter, UFit, UFitId, UItem, UItemId, UModule},
};

impl UFit {
    pub(in crate::ud) fn consistency_check(&self, u_data: &UData, seen_item_uids: &mut Vec<UItemId>) -> DebugResult {
        let Some(fit_uid) = u_data.fits.iid_by_xid(&self.id) else {
            return Err(Default::default());
        };
        // If fleet is defined, it should exist and refer fit back
        if let Some(fleet_uid) = self.fleet {
            let Some(fleet) = u_data.fleets.try_get(fleet_uid) else {
                return Err(Default::default());
            };
            if !fleet.contains_fit(&fit_uid) {
                return Err(Default::default());
            }
        }
        // Character
        if let Some(character_uid) = self.character {
            seen_item_uids.push(character_uid);
            let Some(item) = u_data.items.try_get(character_uid) else {
                return Err(Default::default());
            };
            let UItem::Character(character) = item else {
                return Err(Default::default());
            };
            if character.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Skills
        for fit_skill in self.skills.values() {
            seen_item_uids.push(fit_skill.skill_uid);
            let Some(item) = u_data.items.try_get(fit_skill.skill_uid) else {
                return Err(Default::default());
            };
            let UItem::Skill(skill) = item else {
                return Err(Default::default());
            };
            if skill.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            if skill.get_level() != fit_skill.level {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Implants
        for &implant_uid in self.implants.iter() {
            seen_item_uids.push(implant_uid);
            let Some(item) = u_data.items.try_get(implant_uid) else {
                return Err(Default::default());
            };
            let UItem::Implant(implant) = item else {
                return Err(Default::default());
            };
            if implant.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Boosters
        for &booster_uid in self.boosters.iter() {
            seen_item_uids.push(booster_uid);
            let Some(item) = u_data.items.try_get(booster_uid) else {
                return Err(Default::default());
            };
            let UItem::Booster(booster) = item else {
                return Err(Default::default());
            };
            if booster.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Ship
        if let Some(ship_uid) = self.ship {
            seen_item_uids.push(ship_uid);
            let Some(item) = u_data.items.try_get(ship_uid) else {
                return Err(Default::default());
            };
            let UItem::Ship(ship) = item else {
                return Err(Default::default());
            };
            if ship.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Stance
        if let Some(stance_uid) = self.stance {
            seen_item_uids.push(stance_uid);
            let Some(item) = u_data.items.try_get(stance_uid) else {
                return Err(Default::default());
            };
            let UItem::Stance(stance) = item else {
                return Err(Default::default());
            };
            if stance.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Subsystems
        for &subsystem_uid in self.subsystems.iter() {
            seen_item_uids.push(subsystem_uid);
            let Some(item) = u_data.items.try_get(subsystem_uid) else {
                return Err(Default::default());
            };
            let UItem::Subsystem(subsystem) = item else {
                return Err(Default::default());
            };
            if subsystem.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // High slot modules
        self.mods_high.consistency_check()?;
        for &module_uid in self.mods_high.iter_uids() {
            seen_item_uids.push(module_uid);
            let Some(item) = u_data.items.try_get(module_uid) else {
                return Err(Default::default());
            };
            let UItem::Module(module) = item else {
                return Err(Default::default());
            };
            if module.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            if !matches!(module.get_rack(), ModRack::High) {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
            check_module_charge(u_data, fit_uid, module_uid, module, seen_item_uids)?;
        }
        // Mid slot modules
        self.mods_mid.consistency_check()?;
        for &module_uid in self.mods_mid.iter_uids() {
            seen_item_uids.push(module_uid);
            let Some(item) = u_data.items.try_get(module_uid) else {
                return Err(Default::default());
            };
            let UItem::Module(module) = item else {
                return Err(Default::default());
            };
            if module.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            if !matches!(module.get_rack(), ModRack::Mid) {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
            check_module_charge(u_data, fit_uid, module_uid, module, seen_item_uids)?;
        }
        // Low slot modules
        self.mods_low.consistency_check()?;
        for &module_uid in self.mods_low.iter_uids() {
            seen_item_uids.push(module_uid);
            let Some(item) = u_data.items.try_get(module_uid) else {
                return Err(Default::default());
            };
            let UItem::Module(module) = item else {
                return Err(Default::default());
            };
            if module.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            if !matches!(module.get_rack(), ModRack::Low) {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
            check_module_charge(u_data, fit_uid, module_uid, module, seen_item_uids)?;
        }
        // Rigs
        for &rig_uid in self.rigs.iter() {
            seen_item_uids.push(rig_uid);
            let Some(item) = u_data.items.try_get(rig_uid) else {
                return Err(Default::default());
            };
            let UItem::Rig(rig) = item else {
                return Err(Default::default());
            };
            if rig.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Services
        for &service_uid in self.services.iter() {
            seen_item_uids.push(service_uid);
            let Some(item) = u_data.items.try_get(service_uid) else {
                return Err(Default::default());
            };
            let UItem::Service(service) = item else {
                return Err(Default::default());
            };
            if service.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Drones
        for &drone_uid in self.drones.iter() {
            seen_item_uids.push(drone_uid);
            let Some(item) = u_data.items.try_get(drone_uid) else {
                return Err(Default::default());
            };
            let UItem::Drone(drone) = item else {
                return Err(Default::default());
            };
            if drone.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        // Fighters
        for &fighter_uid in self.fighters.iter() {
            seen_item_uids.push(fighter_uid);
            let Some(item) = u_data.items.try_get(fighter_uid) else {
                return Err(Default::default());
            };
            let UItem::Fighter(fighter) = item else {
                return Err(Default::default());
            };
            if fighter.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
            check_fighter_autocharges(u_data, fit_uid, fighter_uid, fighter, seen_item_uids)?;
        }
        // Fit-wide effects
        for &fw_effect_uid in self.fw_effects.iter() {
            seen_item_uids.push(fw_effect_uid);
            let Some(item) = u_data.items.try_get(fw_effect_uid) else {
                return Err(Default::default());
            };
            let UItem::FwEffect(fw_effect) = item else {
                return Err(Default::default());
            };
            if fw_effect.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        Ok(())
    }
}

fn check_module_charge(
    u_data: &UData,
    fit_uid: UFitId,
    module_uid: UItemId,
    module: &UModule,
    seen_items: &mut Vec<UItemId>,
) -> DebugResult {
    if let Some(charge_uid) = module.get_charge_uid() {
        seen_items.push(charge_uid);
        let Some(item) = u_data.items.try_get(charge_uid) else {
            return Err(Default::default());
        };
        let UItem::Charge(charge) = item else {
            return Err(Default::default());
        };
        if charge.get_fit_uid() != fit_uid {
            return Err(Default::default());
        }
        if charge.get_cont_item_uid() != module_uid {
            return Err(Default::default());
        }
        item.consistency_check(u_data)?;
    }
    Ok(())
}

fn check_fighter_autocharges(
    u_data: &UData,
    fit_uid: UFitId,
    fighter_uid: UItemId,
    fighter: &UFighter,
    seen_items: &mut Vec<UItemId>,
) -> DebugResult {
    for autocharge_uid in fighter.get_autocharges().values() {
        seen_items.push(autocharge_uid);
        let Some(item) = u_data.items.try_get(autocharge_uid) else {
            return Err(Default::default());
        };
        let UItem::Autocharge(autocharge) = item else {
            return Err(Default::default());
        };
        if autocharge.get_fit_uid() != fit_uid {
            return Err(Default::default());
        }
        if autocharge.get_cont_item_uid() != fighter_uid {
            return Err(Default::default());
        }
        item.consistency_check(u_data)?;
    }
    Ok(())
}
