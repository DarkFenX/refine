use crate::{
    defs::{SolFitId, SolItemId},
    sol::{
        fit::SolFit,
        item::{SolFighter, SolItem, SolModule},
        SolModRack, SolView,
    },
    util::{DebugError, DebugResult},
};

impl SolFit {
    pub(in crate::sol) fn debug_consistency_check(
        &self,
        sol_view: &SolView,
        seen_items: &mut Vec<SolItemId>,
    ) -> DebugResult {
        // If fleet is defined, it should exist and refer fit back
        if let Some(fleet_id) = self.fleet {
            let fleet = match sol_view.fleets.get_fleet(&fleet_id) {
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
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Character(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Skills
        for item_id in self.skills.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Skill(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Implants
        for item_id in self.implants.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Implant(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Boosters
        for item_id in self.boosters.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Booster(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Ship
        if let Some(item_id) = self.ship {
            seen_items.push(item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Ship(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Stance
        if let Some(item_id) = self.stance {
            seen_items.push(item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Stance(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Subsystems
        for item_id in self.subsystems.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Subsystem(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // High slot modules
        for item_id in self.mods_high.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                SolItem::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.rack, SolModRack::High) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
            check_module_charge(sol_view, &self.id, module, seen_items)?;
        }
        // Mid slot modules
        for item_id in self.mods_mid.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                SolItem::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.rack, SolModRack::Mid) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
            check_module_charge(sol_view, &self.id, module, seen_items)?;
        }
        // Low slot modules
        for item_id in self.mods_low.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                SolItem::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.rack, SolModRack::Low) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
            check_module_charge(sol_view, &self.id, module, seen_items)?;
        }
        // Rigs
        for item_id in self.rigs.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Rig(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Drones
        for item_id in self.drones.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::Drone(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Fighters
        for item_id in self.fighters.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let fighter = match item {
                SolItem::Fighter(fighter) => fighter,
                _ => return Err(DebugError::new()),
            };
            item.debug_consistency_check(sol_view)?;
            check_fighter_autocharges(sol_view, &self.id, fighter, seen_items)?;
        }
        // Fit-wide effects
        for item_id in self.fw_effects.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SolItem::FwEffect(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        Ok(())
    }
}

fn check_module_charge(
    sol_view: &SolView,
    fit_id: &SolFitId,
    module: &SolModule,
    seen_items: &mut Vec<SolItemId>,
) -> DebugResult {
    if let Some(item_id) = module.charge_item_id {
        seen_items.push(item_id);
        let item = match sol_view.items.get_item(&item_id) {
            Ok(item) => item,
            _ => return Err(DebugError::new()),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(DebugError::new());
        }
        let charge = match item {
            SolItem::Charge(charge) => charge,
            _ => return Err(DebugError::new()),
        };
        if charge.cont_id != module.base.id {
            return Err(DebugError::new());
        }
        item.debug_consistency_check(sol_view)?;
    }
    Ok(())
}

fn check_fighter_autocharges(
    sol_view: &SolView,
    fit_id: &SolFitId,
    fighter: &SolFighter,
    seen_items: &mut Vec<SolItemId>,
) -> DebugResult {
    for item_id in fighter.autocharges.values() {
        seen_items.push(*item_id);
        let item = match sol_view.items.get_item(item_id) {
            Ok(item) => item,
            _ => return Err(DebugError::new()),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(DebugError::new());
        }
        let charge = match item {
            SolItem::Charge(charge) => charge,
            _ => return Err(DebugError::new()),
        };
        if charge.cont_id != fighter.base.id {
            return Err(DebugError::new());
        }
        item.debug_consistency_check(sol_view)?;
    }
    Ok(())
}
