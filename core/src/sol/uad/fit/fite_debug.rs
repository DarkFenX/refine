use crate::sol::{
    FitId, ItemKey, ModRack,
    debug::{DebugError, DebugResult},
    uad::{
        Uad,
        fit::Fit,
        item::{Fighter, Item, Module},
    },
};

impl Fit {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad, seen_item_keys: &mut Vec<ItemKey>) -> DebugResult {
        // If fleet is defined, it should exist and refer fit back
        if let Some(fleet_id) = self.fleet {
            let fleet = match uad.fleets.get_fleet(&fleet_id) {
                Ok(fleet) => fleet,
                _ => return Err(DebugError {}),
            };
            if !fleet.contains_fit(&self.id) {
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
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Character(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // Skills
        for fit_skill in self.skills.values() {
            seen_item_keys.push(fit_skill.item_key);
            let item = match uad.items.try_get(fit_skill.item_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            match item {
                Item::Skill(skill) => {
                    if skill.get_a_level() != fit_skill.level {
                        return Err(DebugError {});
                    }
                }
                _ => return Err(DebugError {}),
            }
            item.debug_consistency_check(uad)?;
        }
        // Implants
        for &implant_key in self.implants.iter() {
            seen_item_keys.push(implant_key);
            let item = match uad.items.try_get(implant_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Implant(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // Boosters
        for &booster_key in self.boosters.iter() {
            seen_item_keys.push(booster_key);
            let item = match uad.items.try_get(booster_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Booster(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // Ship
        if let Some(ship_key) = self.ship {
            seen_item_keys.push(ship_key);
            let item = match uad.items.try_get(ship_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Ship(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // Stance
        if let Some(stance_key) = self.stance {
            seen_item_keys.push(stance_key);
            let item = match uad.items.try_get(stance_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Stance(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // Subsystems
        for &subsystem_key in self.subsystems.iter() {
            seen_item_keys.push(subsystem_key);
            let item = match uad.items.try_get(subsystem_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Subsystem(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // High slot modules
        self.mods_high.debug_consistency_check()?;
        for &module_key in self.mods_high.iter_keys() {
            seen_item_keys.push(module_key);
            let item = match uad.items.try_get(module_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            let module = match item {
                Item::Module(module) => module,
                _ => return Err(DebugError {}),
            };
            if !matches!(module.get_rack(), ModRack::High) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
            check_module_charge(uad, &self.id, module_key, module, seen_item_keys)?;
        }
        // Mid slot modules
        self.mods_mid.debug_consistency_check()?;
        for &module_key in self.mods_mid.iter_keys() {
            seen_item_keys.push(module_key);
            let item = match uad.items.try_get(module_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            let module = match item {
                Item::Module(module) => module,
                _ => return Err(DebugError {}),
            };
            if !matches!(module.get_rack(), ModRack::Mid) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
            check_module_charge(uad, &self.id, module_key, module, seen_item_keys)?;
        }
        // Low slot modules
        self.mods_low.debug_consistency_check()?;
        for &module_key in self.mods_low.iter_keys() {
            seen_item_keys.push(module_key);
            let item = match uad.items.try_get(module_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            let module = match item {
                Item::Module(module) => module,
                _ => return Err(DebugError {}),
            };
            if !matches!(module.get_rack(), ModRack::Low) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
            check_module_charge(uad, &self.id, module_key, module, seen_item_keys)?;
        }
        // Rigs
        for &rig_key in self.rigs.iter() {
            seen_item_keys.push(rig_key);
            let item = match uad.items.try_get(rig_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Rig(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // Services
        for &service_key in self.services.iter() {
            seen_item_keys.push(service_key);
            let item = match uad.items.try_get(service_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Service(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // Drones
        for &drone_key in self.drones.iter() {
            seen_item_keys.push(drone_key);
            let item = match uad.items.try_get(drone_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::Drone(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        // Fighters
        for &fighter_key in self.fighters.iter() {
            seen_item_keys.push(fighter_key);
            let item = match uad.items.try_get(fighter_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            let fighter = match item {
                Item::Fighter(fighter) => fighter,
                _ => return Err(DebugError {}),
            };
            item.debug_consistency_check(uad)?;
            check_fighter_autocharges(uad, &self.id, fighter_key, fighter, seen_item_keys)?;
        }
        // Fit-wide effects
        for &fw_effect_key in self.fw_effects.iter() {
            seen_item_keys.push(fw_effect_key);
            let item = match uad.items.try_get(fw_effect_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError {});
            }
            if !matches!(item, Item::FwEffect(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(uad)?;
        }
        Ok(())
    }
}

fn check_module_charge(
    uad: &Uad,
    fit_id: &FitId,
    module_key: ItemKey,
    module: &Module,
    seen_items: &mut Vec<ItemKey>,
) -> DebugResult {
    if let Some(charge_key) = module.get_charge_item_key() {
        seen_items.push(charge_key);
        let item = match uad.items.try_get(charge_key) {
            Some(item) => item,
            None => return Err(DebugError {}),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(DebugError {});
        }
        let charge = match item {
            Item::Charge(charge) => charge,
            _ => return Err(DebugError {}),
        };
        if charge.get_cont_item_key() != module_key {
            return Err(DebugError {});
        }
        item.debug_consistency_check(uad)?;
    }
    Ok(())
}

fn check_fighter_autocharges(
    uad: &Uad,
    fit_id: &FitId,
    fighter_key: ItemKey,
    fighter: &Fighter,
    seen_items: &mut Vec<ItemKey>,
) -> DebugResult {
    for &autocharge_key in fighter.get_autocharges().values() {
        seen_items.push(autocharge_key);
        let item = match uad.items.try_get(autocharge_key) {
            Some(item) => item,
            None => return Err(DebugError {}),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(DebugError {});
        }
        let autocharge = match item {
            Item::Autocharge(autocharge) => autocharge,
            _ => return Err(DebugError {}),
        };
        if autocharge.get_cont_item_key() != fighter_key {
            return Err(DebugError {});
        }
        item.debug_consistency_check(uad)?;
    }
    Ok(())
}
