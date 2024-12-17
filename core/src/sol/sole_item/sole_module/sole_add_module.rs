use crate::{
    defs::{EItemId, SolFitId},
    err::basic::{FitFoundError, OrderedSlotError},
    sol::{
        item::{SolCharge, SolItem, SolItemMutation, SolItemState, SolModule},
        item_info::{SolChargeInfo, SolModuleInfo},
        sole_item::misc::find_equip_pos,
        SolModRack, SolOrdAddMode, SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_module(
        &mut self,
        fit_id: SolFitId,
        rack: SolModRack,
        pos_mode: SolOrdAddMode,
        type_id: EItemId,
        state: SolItemState,
        mutation: Option<SolItemMutation>,
        charge_type_id: Option<EItemId>,
    ) -> Result<SolModuleInfo, AddModuleError> {
        let module_item_id = self.items.alloc_item_id();
        let fit = self.fits.get_fit(&fit_id)?;
        // Calculate position for the module
        let rack_module_ids = match rack {
            SolModRack::High => &fit.mods_high,
            SolModRack::Mid => &fit.mods_mid,
            SolModRack::Low => &fit.mods_low,
        };
        let pos = match pos_mode {
            // Add to the end of module rack
            SolOrdAddMode::Append => {
                match rack_module_ids
                    .iter()
                    .map(|v| self.items.get_item(v).unwrap().get_module().unwrap().get_pos())
                    .max()
                {
                    Some(pos) => pos + 1,
                    None => 0,
                }
            }
            // Take first spare slot in the rack
            SolOrdAddMode::Equip => {
                let positions = rack_module_ids
                    .iter()
                    .map(|v| self.items.get_item(v).unwrap().get_module().unwrap().get_pos())
                    .collect();
                find_equip_pos(positions)
            }
            // Insert at specified position, shifting other modules to the right
            SolOrdAddMode::Insert(pos) => {
                for rack_module_id in rack_module_ids.iter() {
                    let rack_module = self
                        .items
                        .get_item_mut(rack_module_id)
                        .unwrap()
                        .get_module_mut()
                        .unwrap();
                    let rack_module_pos = rack_module.get_pos();
                    if rack_module_pos >= pos {
                        rack_module.set_pos(rack_module_pos + 1);
                    }
                }
                pos
            }
            // Check if there is a module on position we want to have module:
            // - if it's there, and we were asked to replace it, remove old module
            // - if it's there, and we were not asked to replace it, return an error
            SolOrdAddMode::Place(pos, replace) => {
                let mut old_module_id = None;
                for rack_module_id in rack_module_ids.iter() {
                    let module = self.items.get_item(rack_module_id).unwrap().get_module().unwrap();
                    if module.get_pos() == pos {
                        if replace {
                            old_module_id = Some(*rack_module_id);
                            break;
                        } else {
                            return Err(OrderedSlotError::new(rack, pos, *rack_module_id).into());
                        }
                    }
                }
                if let Some(old_module_id) = old_module_id {
                    self.remove_module(&old_module_id).unwrap();
                }
                pos
            }
        };
        // Create module and add it to items
        let module = SolModule::new(
            &self.src,
            module_item_id,
            type_id,
            fit_id,
            state,
            rack,
            pos,
            mutation,
            None,
        );
        let module_item = SolItem::Module(module);
        self.items.add_item(module_item);
        let mut charge_info = None;
        if let Some(charge_type_id) = charge_type_id {
            let charge_id = self.items.alloc_item_id();
            // Update skeleton with new charge info
            self.items
                .get_item_mut(&module_item_id)
                .unwrap()
                .get_module_mut()
                .unwrap()
                .set_charge_id(Some(charge_id));
            let charge = SolCharge::new(
                &self.src,
                charge_id,
                charge_type_id,
                fit_id,
                module_item_id,
                state,
                false,
            );
            charge_info = Some(SolChargeInfo::from(&charge));
            let item = SolItem::Charge(charge);
            self.items.add_item(item);
        }
        // Finalize updating skeleton
        let fit = self.fits.get_fit_mut(&fit_id).unwrap();
        match rack {
            SolModRack::High => fit.mods_high.insert(module_item_id),
            SolModRack::Mid => fit.mods_mid.insert(module_item_id),
            SolModRack::Low => fit.mods_low.insert(module_item_id),
        };
        // Add module and charge to services
        let module_item = self.items.get_item(&module_item_id).unwrap();
        let module = module_item.get_module().unwrap();
        let module_info = SolModuleInfo::from_mod_and_charge_with_source(&self.src, module, charge_info);
        self.svcs.add_item(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            module_item,
        );
        if let Some(charge_info) = &module_info.charge {
            self.add_item_id_to_svcs(&charge_info.id);
        }
        Ok(module_info)
    }
}

#[derive(Debug)]
pub enum AddModuleError {
    FitNotFound(FitFoundError),
    SlotTaken(OrderedSlotError),
}
impl std::error::Error for AddModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::SlotTaken(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::SlotTaken(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddModuleError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<OrderedSlotError> for AddModuleError {
    fn from(error: OrderedSlotError) -> Self {
        Self::SlotTaken(error)
    }
}
