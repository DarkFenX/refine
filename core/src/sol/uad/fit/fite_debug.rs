use crate::sol::{
    FitId, ItemId, ModRack,
    debug::{DebugError, DebugResult},
    uad::{
        Uad,
        fit::Fit,
        item::{Fighter, Item, Module},
    },
};

impl Fit {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad, seen_items: &mut Vec<ItemId>) -> DebugResult {
        // If fleet is defined, it should exist and refer fit back
        if let Some(fleet_id) = self.fleet {
            let fleet = match uad.fleets.get_fleet(&fleet_id) {
                Ok(fleet) => fleet,
                _ => return Err(DebugError::new()),
            };
            if !fleet.contains_fit(&self.id) {
                return Err(DebugError::new());
            }
        }
        // Character
        if let Some(item_id) = self.character {
            seen_items.push(item_id);
            let item = match uad.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Character(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Skills
        for fit_skill in self.skills.values() {
            seen_items.push(fit_skill.item_id);
            let item = match uad.items.get_item(&fit_skill.item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            match item {
                Item::Skill(skill) => {
                    if skill.get_a_level() != fit_skill.level {
                        return Err(DebugError::new());
                    }
                }
                _ => return Err(DebugError::new()),
            }
            item.debug_consistency_check(uad)?;
        }
        // Implants
        for item_id in self.implants.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Implant(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Boosters
        for item_id in self.boosters.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Booster(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Ship
        if let Some(item_id) = self.ship {
            seen_items.push(item_id);
            let item = match uad.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Ship(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Stance
        if let Some(item_id) = self.stance {
            seen_items.push(item_id);
            let item = match uad.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Stance(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Subsystems
        for item_id in self.subsystems.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Subsystem(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // High slot modules
        self.mods_high.debug_consistency_check()?;
        for item_id in self.mods_high.iter_ids() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                Item::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.get_rack(), ModRack::High) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
            check_module_charge(uad, &self.id, module, seen_items)?;
        }
        // Mid slot modules
        self.mods_mid.debug_consistency_check()?;
        for item_id in self.mods_mid.iter_ids() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                Item::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.get_rack(), ModRack::Mid) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
            check_module_charge(uad, &self.id, module, seen_items)?;
        }
        // Low slot modules
        self.mods_low.debug_consistency_check()?;
        for item_id in self.mods_low.iter_ids() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                Item::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.get_rack(), ModRack::Low) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
            check_module_charge(uad, &self.id, module, seen_items)?;
        }
        // Rigs
        for item_id in self.rigs.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Rig(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Services
        for item_id in self.services.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Service(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Drones
        for item_id in self.drones.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::Drone(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Fighters
        for item_id in self.fighters.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let fighter = match item {
                Item::Fighter(fighter) => fighter,
                _ => return Err(DebugError::new()),
            };
            item.debug_consistency_check(uad)?;
            check_fighter_autocharges(uad, &self.id, fighter, seen_items)?;
        }
        // Fit-wide effects
        for item_id in self.fw_effects.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, Item::FwEffect(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        Ok(())
    }
}

fn check_module_charge(uad: &Uad, fit_id: &FitId, module: &Module, seen_items: &mut Vec<ItemId>) -> DebugResult {
    if let Some(item_id) = module.get_charge_item_id() {
        seen_items.push(item_id);
        let item = match uad.items.get_item(&item_id) {
            Ok(item) => item,
            _ => return Err(DebugError::new()),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(DebugError::new());
        }
        let charge = match item {
            Item::Charge(charge) => charge,
            _ => return Err(DebugError::new()),
        };
        if charge.get_cont_item_id() != module.get_item_id() {
            return Err(DebugError::new());
        }
        item.debug_consistency_check(uad)?;
    }
    Ok(())
}

fn check_fighter_autocharges(
    uad: &Uad,
    fit_id: &FitId,
    fighter: &Fighter,
    seen_items: &mut Vec<ItemId>,
) -> DebugResult {
    for item_id in fighter.get_autocharges().values() {
        seen_items.push(*item_id);
        let item = match uad.items.get_item(item_id) {
            Ok(item) => item,
            _ => return Err(DebugError::new()),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(DebugError::new());
        }
        let autocharge = match item {
            Item::Autocharge(autocharge) => autocharge,
            _ => return Err(DebugError::new()),
        };
        if autocharge.get_cont_item_id() != fighter.get_item_id() {
            return Err(DebugError::new());
        }
        item.debug_consistency_check(uad)?;
    }
    Ok(())
}
