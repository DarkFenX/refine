use crate::{
    defs::{SsFitId, SsItemId},
    ss::{
        fit::SsFit,
        item::{SsItem, SsModule},
        SsModRack, SsView,
    },
    util::{DebugError, DebugResult},
};

impl SsFit {
    pub(in crate::ss) fn debug_consistency_check(
        &self,
        ss_view: &SsView,
        seen_items: &mut Vec<SsItemId>,
    ) -> DebugResult {
        // If fleet is defined, it should exist and refer fit back
        if let Some(fleet_id) = self.fleet {
            let fleet = match ss_view.fleets.get_fleet(&fleet_id) {
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
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Character(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Skills
        for item_id in self.skills.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Skill(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Implants
        for item_id in self.implants.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Implant(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Boosters
        for item_id in self.boosters.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Booster(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Ship
        if let Some(item_id) = self.ship {
            seen_items.push(item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Ship(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Structure
        if let Some(item_id) = self.structure {
            seen_items.push(item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Structure(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Stance
        if let Some(item_id) = self.stance {
            seen_items.push(item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Stance(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Subsystems
        for item_id in self.subsystems.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Subsystem(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // High slot modules
        for item_id in self.mods_high.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                SsItem::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.rack, SsModRack::High) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
            check_module_charge(ss_view, &self.id, module, seen_items)?;
        }
        // Mid slot modules
        for item_id in self.mods_mid.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                SsItem::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.rack, SsModRack::Mid) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
            check_module_charge(ss_view, &self.id, module, seen_items)?;
        }
        // Low slot modules
        for item_id in self.mods_low.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            let module = match item {
                SsItem::Module(module) => module,
                _ => return Err(DebugError::new()),
            };
            if !matches!(module.rack, SsModRack::Low) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
            check_module_charge(ss_view, &self.id, module, seen_items)?;
        }
        // Rigs
        for item_id in self.rigs.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Rig(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Drones
        for item_id in self.drones.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Drone(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Fighters
        for item_id in self.fighters.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::Fighter(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        // Fit-wide effects
        for item_id in self.fw_effects.iter() {
            seen_items.push(*item_id);
            let item = match ss_view.items.get_item(&item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if item.get_fit_id() != Some(self.id) {
                return Err(DebugError::new());
            }
            if !matches!(item, SsItem::FwEffect(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(ss_view)?;
        }
        Ok(())
    }
}

fn check_module_charge(
    ss_view: &SsView,
    fit_id: &SsFitId,
    module: &SsModule,
    seen_items: &mut Vec<SsItemId>,
) -> DebugResult {
    if let Some(item_id) = module.charge_item_id {
        seen_items.push(item_id);
        let item = match ss_view.items.get_item(&item_id) {
            Ok(item) => item,
            _ => return Err(DebugError::new()),
        };
        if item.get_fit_id() != Some(*fit_id) {
            return Err(DebugError::new());
        }
        let charge = match item {
            SsItem::Charge(charge) => charge,
            _ => return Err(DebugError::new()),
        };
        if charge.cont_id != module.id {
            return Err(DebugError::new());
        }
        item.debug_consistency_check(ss_view)?;
    }
    Ok(())
}
