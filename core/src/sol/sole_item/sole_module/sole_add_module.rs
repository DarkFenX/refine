use crate::{
    defs::{EItemId, SolFitId},
    err::basic::{FitFoundError, ItemAllocError, OrderedSlotError},
    sol::{
        item::{SolCharge, SolItem, SolItemState, SolModule},
        item_info::SolModuleInfo,
        sole_item::misc::find_equip_pos,
        SolModRack, SolOrdAddMode, SolView, SolarSystem,
    },
    SolChargeInfo,
};

impl SolarSystem {
    pub fn add_module(
        &mut self,
        fit_id: SolFitId,
        rack: SolModRack,
        pos_mode: SolOrdAddMode,
        type_id: EItemId,
        state: SolItemState,
        charge_type_id: Option<EItemId>,
    ) -> Result<SolModuleInfo, AddModuleError> {
        let module_item_id = self.items.alloc_item_id()?;
        // Calculate position for the module
        let fit = self.fits.get_fit(&fit_id)?;
        let infos = self.int_get_fit_module_infos(fit, rack);
        let mut old_module_item_id = None;
        let pos = match pos_mode {
            SolOrdAddMode::Append => infos.iter().map(|v| v.pos).max().map(|v| 1 + v).unwrap_or(0),
            SolOrdAddMode::Equip => {
                let positions = infos.iter().map(|v| v.pos).collect();
                find_equip_pos(positions)
            }
            SolOrdAddMode::Insert(pos) => pos,
            // Check if there is a module on position we want to have module:
            // - if it's there, and we were asked to replace it, record ID to do it later
            // - if it's there, and we were not asked to replace it, return an error
            SolOrdAddMode::Place(pos, replace) => {
                for info in infos.iter() {
                    let module = self.items.get_item(&info.id).unwrap().get_module().unwrap();
                    if module.get_rack() == rack && module.get_pos() == pos {
                        old_module_item_id = Some(info.id);
                        break;
                    }
                }
                if let (Some(old_module_item_id), false) = (old_module_item_id, replace) {
                    return Err(OrderedSlotError::new(rack, pos, old_module_item_id).into());
                }
                pos
            }
        };
        // Create module and add it to items, to ensure its ID is taken
        let module = SolModule::new(&self.src, module_item_id, type_id, fit_id, state, rack, pos, None, None);
        let module_item = SolItem::Module(module);
        self.items.add_item(module_item);
        let mut charge_info = None;
        if let Some(charge_type_id) = charge_type_id {
            let charge_id = match self.items.alloc_item_id() {
                Ok(charge_id) => charge_id,
                // Revert the only change we already did if charge allocation fails
                Err(e) => {
                    self.items.remove_item(&module_item_id).unwrap();
                    return Err(e.into());
                }
            };
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
        // After this point we can't fail, so do the rest of changes - update positions if we were
        // inserting the module
        if let SolOrdAddMode::Insert(_) = pos_mode {
            for info in infos.iter() {
                let module = self.items.get_item_mut(&info.id).unwrap().get_module_mut().unwrap();
                let module_pos = module.get_pos();
                if module_pos >= pos && module.get_rack() == rack {
                    module.set_pos(module_pos + 1);
                }
            }
        }
        // Remove old module if needed
        if let Some(old_module_item_id) = old_module_item_id {
            self.remove_module(&old_module_item_id).unwrap();
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
    ItemIdAllocFailed(ItemAllocError),
    SlotTaken(OrderedSlotError),
}
impl std::error::Error for AddModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
            Self::SlotTaken(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
            Self::SlotTaken(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddModuleError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for AddModuleError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl From<OrderedSlotError> for AddModuleError {
    fn from(error: OrderedSlotError) -> Self {
        Self::SlotTaken(error)
    }
}
