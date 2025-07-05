use crate::{
    dbg::{DebugError, DebugResult},
    def::{FitKey, ItemKey},
    misc::ModRack,
    uad::{Uad, UadFighter, UadFit, UadItem, UadModule},
};

impl UadFit {
    pub(in crate::uad) fn consistency_check(&self, uad: &Uad, seen_item_keys: &mut Vec<ItemKey>) -> DebugResult {
        let fit_key = match uad.fits.key_by_id(&self.id) {
            Some(fit_key) => fit_key,
            None => return Err(DebugError {}),
        };
        // If fleet is defined, it should exist and refer fit back
        if let Some(fleet_key) = self.fleet {
            let fleet = match uad.fleets.try_get(fleet_key) {
                Some(fleet) => fleet,
                _ => return Err(DebugError {}),
            };
            if !fleet.contains_fit(&fit_key) {
                return Err(DebugError {});
            }
        }
        // Character
        if let Some(character_key) = self.character {
            seen_item_keys.push(character_key);
            let item = match uad.items.try_get(character_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let character = match item {
                UadItem::Character(character) => character,
                _ => return Err(DebugError {}),
            };
            if character.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Skills
        for fit_skill in self.skills.values() {
            seen_item_keys.push(fit_skill.item_key);
            let item = match uad.items.try_get(fit_skill.item_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let skill = match item {
                UadItem::Skill(skill) => skill,
                _ => return Err(DebugError {}),
            };
            if skill.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            if skill.get_a_level() != fit_skill.level {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Implants
        for &implant_key in self.implants.iter() {
            seen_item_keys.push(implant_key);
            let item = match uad.items.try_get(implant_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let implant = match item {
                UadItem::Implant(implant) => implant,
                _ => return Err(DebugError {}),
            };
            if implant.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Boosters
        for &booster_key in self.boosters.iter() {
            seen_item_keys.push(booster_key);
            let item = match uad.items.try_get(booster_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let booster = match item {
                UadItem::Booster(booster) => booster,
                _ => return Err(DebugError {}),
            };
            if booster.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Ship
        if let Some(ship_key) = self.ship {
            seen_item_keys.push(ship_key);
            let item = match uad.items.try_get(ship_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let ship = match item {
                UadItem::Ship(ship) => ship,
                _ => return Err(DebugError {}),
            };
            if ship.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Stance
        if let Some(stance_key) = self.stance {
            seen_item_keys.push(stance_key);
            let item = match uad.items.try_get(stance_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let stance = match item {
                UadItem::Stance(stance) => stance,
                _ => return Err(DebugError {}),
            };
            if stance.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Subsystems
        for &subsystem_key in self.subsystems.iter() {
            seen_item_keys.push(subsystem_key);
            let item = match uad.items.try_get(subsystem_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let subsystem = match item {
                UadItem::Subsystem(subsystem) => subsystem,
                _ => return Err(DebugError {}),
            };
            if subsystem.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // High slot modules
        self.mods_high.consistency_check()?;
        for &module_key in self.mods_high.iter_keys() {
            seen_item_keys.push(module_key);
            let item = match uad.items.try_get(module_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let module = match item {
                UadItem::Module(module) => module,
                _ => return Err(DebugError {}),
            };
            if module.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            if !matches!(module.get_rack(), ModRack::High) {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
            check_module_charge(uad, fit_key, module_key, module, seen_item_keys)?;
        }
        // Mid slot modules
        self.mods_mid.consistency_check()?;
        for &module_key in self.mods_mid.iter_keys() {
            seen_item_keys.push(module_key);
            let item = match uad.items.try_get(module_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let module = match item {
                UadItem::Module(module) => module,
                _ => return Err(DebugError {}),
            };
            if module.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            if !matches!(module.get_rack(), ModRack::Mid) {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
            check_module_charge(uad, fit_key, module_key, module, seen_item_keys)?;
        }
        // Low slot modules
        self.mods_low.consistency_check()?;
        for &module_key in self.mods_low.iter_keys() {
            seen_item_keys.push(module_key);
            let item = match uad.items.try_get(module_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let module = match item {
                UadItem::Module(module) => module,
                _ => return Err(DebugError {}),
            };
            if module.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            if !matches!(module.get_rack(), ModRack::Low) {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
            check_module_charge(uad, fit_key, module_key, module, seen_item_keys)?;
        }
        // Rigs
        for &rig_key in self.rigs.iter() {
            seen_item_keys.push(rig_key);
            let item = match uad.items.try_get(rig_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let rig = match item {
                UadItem::Rig(rig) => rig,
                _ => return Err(DebugError {}),
            };
            if rig.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Services
        for &service_key in self.services.iter() {
            seen_item_keys.push(service_key);
            let item = match uad.items.try_get(service_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let service = match item {
                UadItem::Service(service) => service,
                _ => return Err(DebugError {}),
            };
            if service.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Drones
        for &drone_key in self.drones.iter() {
            seen_item_keys.push(drone_key);
            let item = match uad.items.try_get(drone_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let drone = match item {
                UadItem::Drone(drone) => drone,
                _ => return Err(DebugError {}),
            };
            if drone.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        // Fighters
        for &fighter_key in self.fighters.iter() {
            seen_item_keys.push(fighter_key);
            let item = match uad.items.try_get(fighter_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let fighter = match item {
                UadItem::Fighter(fighter) => fighter,
                _ => return Err(DebugError {}),
            };
            if fighter.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
            check_fighter_autocharges(uad, fit_key, fighter_key, fighter, seen_item_keys)?;
        }
        // Fit-wide effects
        for &fw_effect_key in self.fw_effects.iter() {
            seen_item_keys.push(fw_effect_key);
            let item = match uad.items.try_get(fw_effect_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            let fw_effect = match item {
                UadItem::FwEffect(fw_effect) => fw_effect,
                _ => return Err(DebugError {}),
            };
            if fw_effect.get_fit_key() != fit_key {
                return Err(DebugError {});
            }
            item.consistency_check(uad)?;
        }
        Ok(())
    }
}

fn check_module_charge(
    uad: &Uad,
    fit_key: FitKey,
    module_key: ItemKey,
    module: &UadModule,
    seen_items: &mut Vec<ItemKey>,
) -> DebugResult {
    if let Some(charge_key) = module.get_charge_key() {
        seen_items.push(charge_key);
        let item = match uad.items.try_get(charge_key) {
            Some(item) => item,
            None => return Err(DebugError {}),
        };
        if item.get_fit_key() != Some(fit_key) {
            return Err(DebugError {});
        }
        let charge = match item {
            UadItem::Charge(charge) => charge,
            _ => return Err(DebugError {}),
        };
        if charge.get_cont_key() != module_key {
            return Err(DebugError {});
        }
        item.consistency_check(uad)?;
    }
    Ok(())
}

fn check_fighter_autocharges(
    uad: &Uad,
    fit_key: FitKey,
    fighter_key: ItemKey,
    fighter: &UadFighter,
    seen_items: &mut Vec<ItemKey>,
) -> DebugResult {
    for &autocharge_key in fighter.get_autocharges().values() {
        seen_items.push(autocharge_key);
        let item = match uad.items.try_get(autocharge_key) {
            Some(item) => item,
            None => return Err(DebugError {}),
        };
        if item.get_fit_key() != Some(fit_key) {
            return Err(DebugError {});
        }
        let autocharge = match item {
            UadItem::Autocharge(autocharge) => autocharge,
            _ => return Err(DebugError {}),
        };
        if autocharge.get_cont_key() != fighter_key {
            return Err(DebugError {});
        }
        item.consistency_check(uad)?;
    }
    Ok(())
}
