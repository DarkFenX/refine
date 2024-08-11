use crate::{
    defs::{SolFitId, SolItemId},
    sol::{
        fit::SolFit,
        item::{SolFighter, SolItem, SolModule},
        SolDebugError, SolDebugResult, SolModRack, SolView,
    },
};

impl SolFit {
    pub(in crate::sol) fn debug_consistency_check(
        &self,
        sol_view: &SolView,
        seen_items: &mut Vec<SolItemId>,
    ) -> SolDebugResult {
        // If fleet is defined, it should exist and refer fit back
        if let Some(fleet_id) = self.fleet {
            let fleet = match sol_view.fleets.get_fleet(&fleet_id) {
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
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Character(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Skills
        for item_id in self.skills.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Skill(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Implants
        for item_id in self.implants.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Implant(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Boosters
        for item_id in self.boosters.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Booster(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Ship
        if let Some(item_id) = self.ship {
            seen_items.push(item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Ship(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Stance
        if let Some(item_id) = self.stance {
            seen_items.push(item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Stance(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Subsystems
        for item_id in self.subsystems.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Subsystem(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // High slot modules
        for item_id in self.mods_high.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
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
            if !matches!(module.rack, SolModRack::High) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
            check_module_charge(sol_view, &self.id, module, seen_items)?;
        }
        // Mid slot modules
        for item_id in self.mods_mid.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
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
            if !matches!(module.rack, SolModRack::Mid) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
            check_module_charge(sol_view, &self.id, module, seen_items)?;
        }
        // Low slot modules
        for item_id in self.mods_low.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
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
            if !matches!(module.rack, SolModRack::Low) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
            check_module_charge(sol_view, &self.id, module, seen_items)?;
        }
        // Rigs
        for item_id in self.rigs.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Rig(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Drones
        for item_id in self.drones.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::Drone(_)) {
                return Err(SolDebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Fighters
        for item_id in self.fighters.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
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
            item.debug_consistency_check(sol_view)?;
            check_fighter_autocharges(sol_view, &self.id, fighter, seen_items)?;
        }
        // Fit-wide effects
        for item_id in self.fw_effects.iter() {
            seen_items.push(*item_id);
            let item = match sol_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(SolDebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(SolDebugError::new());
            }
            if !matches!(item, SolItem::FwEffect(_)) {
                return Err(SolDebugError::new());
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
) -> SolDebugResult {
    if let Some(item_id) = module.charge_item_id {
        seen_items.push(item_id);
        let item = match sol_view.items.get_item(&item_id) {
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
        if charge.cont_id != module.get_id() {
            return Err(SolDebugError::new());
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
) -> SolDebugResult {
    for item_id in fighter.autocharges.values() {
        seen_items.push(*item_id);
        let item = match sol_view.items.get_item(item_id) {
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
        if autocharge.cont_id != fighter.get_id() {
            return Err(SolDebugError::new());
        }
        item.debug_consistency_check(sol_view)?;
    }
    Ok(())
}
