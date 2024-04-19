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
            seen_items.push(item_id);
        }
        // Skills
        for item_id in self.skills.iter() {
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
            seen_items.push(*item_id);
        }
        // Implants
        for item_id in self.implants.iter() {
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
            seen_items.push(*item_id);
        }
        // Boosters
        for item_id in self.boosters.iter() {
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
            seen_items.push(*item_id);
        }
        // Ship
        if let Some(item_id) = self.ship {
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
            seen_items.push(item_id);
        }
        // Structure
        if let Some(item_id) = self.structure {
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
            seen_items.push(item_id);
        }
        // Stance
        if let Some(item_id) = self.stance {
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
            seen_items.push(item_id);
        }
        // Subsystems
        for item_id in self.subsystems.iter() {
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
            seen_items.push(*item_id);
        }
        // High slot modules
        for item_id in self.mods_high.iter() {
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
            check_module_charge(ss_view, &self.id, module, seen_items)?;
            seen_items.push(*item_id);
        }
        // Mid slot modules
        for item_id in self.mods_mid.iter() {
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
            check_module_charge(ss_view, &self.id, module, seen_items)?;
            seen_items.push(*item_id);
        }
        // Low slot modules
        for item_id in self.mods_low.iter() {
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
            check_module_charge(ss_view, &self.id, module, seen_items)?;
            seen_items.push(*item_id);
        }
        // Rigs
        for item_id in self.rigs.iter() {
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
            seen_items.push(*item_id);
        }
        // Drones
        for item_id in self.drones.iter() {
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
            seen_items.push(*item_id);
        }
        // Fighters
        for item_id in self.fighters.iter() {
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
            seen_items.push(*item_id);
        }
        // Fit-wide effects
        for item_id in self.fw_effects.iter() {
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
            seen_items.push(*item_id);
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
        seen_items.push(item_id);
    }
    Ok(())
}
