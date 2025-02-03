use crate::{
    defs::{SolFitId, SolItemId},
    sol::{
        debug::{SolDebugError, SolDebugResult},
        uad::{
            fit::SolFit,
            item::{SolFighter, SolItem, SolModule},
            SolUad,
        },
        SolModRack,
    },
};

impl SolFit {
    pub(in crate::sol) fn debug_consistency_check(
        &self,
        uad: &SolUad,
        seen_items: &mut Vec<SolItemId>,
    ) -> SolDebugResult {
        // If fleet is defined, it should exist and refer fit back
        if let Some(fleet_id) = self.fleet {
            let fleet = match uad.fleets.get_fleet(&fleet_id) {
                Ok(fleet) => fleet,
                _ => return Err(SolDebugError::new()),
            };
            if !fleet.contains_fit(&self.id) {
                return Err(SolDebugError::new());
            }
        }
        // Character
        if let Some(item_id) = self.character {
            seen_items.push(item_id);
            let item = match uad.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Character(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Skills
        for fit_skill in self.skills.values() {
            seen_items.push(fit_skill.item_id);
            let item = match uad.items.get_item(&fit_skill.item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            match item {
                SolItem::Skill(skill) => {
                    if skill.get_level() != fit_skill.level {
                        return Err(SolDebugError::new());
                    }
                }
                _ => return Err(SolDebugError::new()),
            }
            item.debug_consistency_check(uad)?;
        }
        // Implants
        for item_id in self.implants.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Implant(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Boosters
        for item_id in self.boosters.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Booster(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Ship
        if let Some(item_id) = self.ship {
            seen_items.push(item_id);
            let item = match uad.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Ship(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Stance
        if let Some(item_id) = self.stance {
            seen_items.push(item_id);
            let item = match uad.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Stance(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Subsystems
        for item_id in self.subsystems.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Subsystem(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // High slot modules
        self.mods_high.debug_consistency_check()?;
        for item_id in self.mods_high.iter_ids() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            let module = match item {
                SolItem::Module(module) => module,
                _ => return Err(SolDebugError::new()),
            };
            if !matches!(module.get_rack(), SolModRack::High) {
                return Err(SolDebugError::new());
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
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            let module = match item {
                SolItem::Module(module) => module,
                _ => return Err(SolDebugError::new()),
            };
            if !matches!(module.get_rack(), SolModRack::Mid) {
                return Err(SolDebugError::new());
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
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            let module = match item {
                SolItem::Module(module) => module,
                _ => return Err(SolDebugError::new()),
            };
            if !matches!(module.get_rack(), SolModRack::Low) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
            check_module_charge(uad, &self.id, module, seen_items)?;
        }
        // Rigs
        for item_id in self.rigs.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Rig(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Drones
        for item_id in self.drones.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Drone(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        // Fighters
        for item_id in self.fighters.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            let fighter = match item {
                SolItem::Fighter(fighter) => fighter,
                _ => return Err(SolDebugError::new()),
            };
            item.debug_consistency_check(uad)?;
            check_fighter_autocharges(uad, &self.id, fighter, seen_items)?;
        }
        // Fit-wide effects
        for item_id in self.fw_effects.iter() {
            seen_items.push(*item_id);
            let item = match uad.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::FwEffect(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(uad)?;
        }
        Ok(())
    }
}

fn check_module_charge(
    uad: &SolUad,
    fit_id: &SolFitId,
    module: &SolModule,
    seen_items: &mut Vec<SolItemId>,
) -> SolDebugResult {
    if let Some(item_id) = module.get_charge_id() {
        seen_items.push(item_id);
        let item = match uad.items.get_item(&item_id) {
            Ok(item) => item,
            _ => return Err(SolDebugError::new()),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(SolDebugError::new());
        }
        let charge = match item {
            SolItem::Charge(charge) => charge,
            _ => return Err(SolDebugError::new()),
        };
        if charge.get_cont_id() != module.get_id() {
            return Err(SolDebugError::new());
        }
        item.debug_consistency_check(uad)?;
    }
    Ok(())
}

fn check_fighter_autocharges(
    uad: &SolUad,
    fit_id: &SolFitId,
    fighter: &SolFighter,
    seen_items: &mut Vec<SolItemId>,
) -> SolDebugResult {
    for item_id in fighter.get_autocharges().values() {
        seen_items.push(*item_id);
        let item = match uad.items.get_item(item_id) {
            Ok(item) => item,
            _ => return Err(SolDebugError::new()),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(SolDebugError::new());
        }
        let autocharge = match item {
            SolItem::Autocharge(autocharge) => autocharge,
            _ => return Err(SolDebugError::new()),
        };
        if autocharge.get_cont_id() != fighter.get_id() {
            return Err(SolDebugError::new());
        }
        item.debug_consistency_check(uad)?;
    }
    Ok(())
}
