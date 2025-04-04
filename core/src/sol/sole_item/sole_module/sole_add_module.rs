use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemId, ItemTypeId, ModRack, SolarSystem,
        info::{ChargeInfo, ModuleInfo},
        uad::item::{Charge, Item, ItemAddMutation, Module, ModuleState},
    },
};

use super::{
    misc::get_fit_rack,
    pos_modes::{AddMode, RmMode},
};

impl SolarSystem {
    pub fn add_module(
        &mut self,
        fit_id: FitId,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
        mutation: Option<ItemAddMutation>,
        charge_type_id: Option<ItemTypeId>,
    ) -> Result<ModuleInfo, AddModuleError> {
        let item_id = self.add_module_internal(fit_id, rack, pos_mode, type_id, state, mutation, charge_type_id)?;
        Ok(self.get_module(&item_id).unwrap())
    }
    pub(in crate::sol) fn add_module_internal(
        &mut self,
        fit_id: FitId,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
        mutation: Option<ItemAddMutation>,
        charge_type_id: Option<ItemTypeId>,
    ) -> Result<ItemId, AddModuleError> {
        let module_item_id = self.uad.items.alloc_item_id();
        let fit_rack = get_fit_rack(&mut self.uad.fits, &fit_id, rack)?;
        // Calculate position for the module and update part of user data (fit rack and modules from
        // it)
        let pos = match pos_mode {
            // Add to the end of module rack
            AddMode::Append => fit_rack.append(module_item_id),
            // Take first spare slot in the rack
            AddMode::Equip => fit_rack.equip(module_item_id),
            // Insert at specified position, shifting other modules to the right
            AddMode::Insert(pos) => {
                // True means inserted module is not the last in the rack
                if fit_rack.insert(pos, module_item_id) {
                    for (i, rack_module_id) in fit_rack.inner()[pos + 1..].iter().enumerate() {
                        if let Some(rack_module_id) = rack_module_id {
                            self.uad
                                .items
                                .get_item_mut(rack_module_id)
                                .unwrap()
                                .get_module_mut()
                                .unwrap()
                                .set_pos(pos + 1 + i);
                        }
                    }
                }
                pos
            }
            // Check if there is a module on position we want to have module, and if yes, remove it
            // before adding new one
            AddMode::Replace(pos) => {
                match fit_rack.get(pos) {
                    Some(old_module_id) => {
                        self.remove_module(&old_module_id, RmMode::Free).unwrap();
                        let fit_rack = get_fit_rack(&mut self.uad.fits, &fit_id, rack).unwrap();
                        fit_rack.place(pos, module_item_id);
                    }
                    None => fit_rack.place(pos, module_item_id),
                }
                pos
            }
        };
        // Create module and add it to items
        let module = Module::new(
            &self.uad.src,
            module_item_id,
            type_id,
            fit_id,
            state,
            rack,
            pos,
            mutation,
            None,
        );
        let module_item = Item::Module(module);
        self.uad.items.add_item(module_item);
        let charge_item_id = match charge_type_id {
            Some(charge_type_id) => {
                let charge_item_id = self.uad.items.alloc_item_id();
                // Update user data with new charge info
                self.uad
                    .items
                    .get_item_mut(&module_item_id)
                    .unwrap()
                    .get_module_mut()
                    .unwrap()
                    .set_charge_item_id(Some(charge_item_id));
                let charge = Charge::new(
                    &self.uad.src,
                    charge_item_id,
                    charge_type_id,
                    fit_id,
                    module_item_id,
                    state.into(),
                    false,
                );
                let item = Item::Charge(charge);
                self.uad.items.add_item(item);
                Some(charge_item_id)
            }
            None => None,
        };
        // Add module and charge to services
        self.add_item_id_to_svc(&module_item_id);
        if let Some(charge_item_id) = charge_item_id {
            self.add_item_id_to_svc(&charge_item_id);
        }
        Ok(module_item_id)
    }
}

#[derive(Debug)]
pub enum AddModuleError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddModuleError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
