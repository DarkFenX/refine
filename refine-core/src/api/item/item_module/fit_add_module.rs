use super::shared::get_fit_rack_mut;
use crate::{
    AddMode, Fit, ItemTypeId, ModRack, Module, ModuleState, RemoveMode,
    ad::AItemId,
    err::basic::RackPosLimitError,
    sol::{SolCore, UndoStep},
    ud::{UCharge, UFitId, UItem, UItemId, UItemMutationRequest, UModule},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn add_module(
        &mut self,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
    ) -> Result<Module<'_>, ModuleAddError> {
        let module_uid =
            self.sol
                .internal_add_module(self.uid, rack, pos_mode, type_id.into_aid(), state, None, None)?;
        Ok(Module::new(self.sol, module_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModuleAddError {
    #[error(transparent)]
    RackPosLimit(#[from] RackPosLimitError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_add_module(
        &mut self,
        fit_uid: UFitId,
        rack: ModRack,
        pos_mode: AddMode,
        type_aid: AItemId,
        state: ModuleState,
        mutation: Option<UItemMutationRequest>,
        charge_type_aid: Option<AItemId>,
    ) -> Result<UItemId, RackPosLimitError> {
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
        // Calculate position for the module before changing anything, since the module or modules
        // it shifts might need a position past the last one
        let pos = match pos_mode {
            // Add to the end of module rack
            AddMode::Append => u_fit_rack.get_append_pos(),
            // Take first spare slot in the rack
            AddMode::Equip => u_fit_rack.get_equip_pos(),
            AddMode::Insert(pos) => u_fit_rack.can_insert().then_some(pos),
            AddMode::Replace(pos) => Some(pos),
        };
        let Some(pos) = pos else {
            return Err(RackPosLimitError {
                fit_id: self.u_data.fits.ext_id_by_int_id(fit_uid),
                rack,
            });
        };
        let module_item_id = self.u_data.items.alloc_id();
        let u_module = UModule::new(
            module_item_id,
            type_aid,
            fit_uid,
            state,
            rack,
            pos,
            mutation,
            None,
            &self.u_data.r_data,
        );
        let module_u_item = UItem::Module(u_module);
        let module_uid = self.u_data.items.add(module_u_item);
        // Update part of user data (fit rack and modules from it)
        match pos_mode {
            AddMode::Append | AddMode::Equip => u_fit_rack.place(pos, module_uid),
            // Insert at specified position, shifting other modules to the right
            AddMode::Insert(..) => {
                for shift_module_uid in u_fit_rack.insert(pos, module_uid) {
                    let shift_u_module = self.u_data.items.get_mut(shift_module_uid).dc_module_mut().unwrap();
                    shift_u_module.set_pos(shift_u_module.get_pos().inc_unchecked())
                }
            }
            // Check if there is a module on position we want to have module, and if yes, remove it
            // before adding new one
            AddMode::Replace(..) => match u_fit_rack.get(pos) {
                Some(old_module_uid) => {
                    self.internal_remove_module(old_module_uid, RemoveMode::Free);
                    let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
                    u_fit_rack.place(pos, module_uid);
                }
                None => u_fit_rack.place(pos, module_uid),
            },
        }
        // Create and add charge
        let charge_uid = match charge_type_aid {
            Some(charge_type_aid) => {
                let charge_item_id = self.u_data.items.alloc_id();
                // Update user data with new charge info
                let u_charge = UCharge::new(
                    charge_item_id,
                    charge_type_aid,
                    fit_uid,
                    module_uid,
                    false,
                    false,
                    &self.u_data.r_data,
                );
                let charge_u_item = UItem::Charge(u_charge);
                let charge_uid = self.u_data.items.add(charge_u_item);
                Some(charge_uid)
            }
            None => None,
        };
        // Update on-module data regarding charge
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.set_charge_uid(charge_uid);
        self.attach_module(module_uid, charge_uid);
        if self.undo.is_recording() {
            // Inserted module shifted other modules, so it has to shift them back when removed
            let undo_pos_mode = match pos_mode {
                AddMode::Insert(..) => RemoveMode::Remove,
                AddMode::Append | AddMode::Equip | AddMode::Replace(..) => RemoveMode::Free,
            };
            self.undo.record(UndoStep::ModuleAdded(module_uid, undo_pos_mode));
        }
        Ok(module_uid)
    }
    pub(in crate::api) fn internal_reattach_module(&mut self, module_uid: UItemId, pos_mode: RemoveMode) {
        self.u_data.items.reattach(module_uid);
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        let fit_uid = u_module.get_fit_uid();
        let rack = u_module.get_rack();
        let pos = u_module.get_pos();
        let charge_uid = u_module.get_charge_uid();
        let has_projs = !u_module.get_projs().is_empty();
        if let Some(charge_uid) = charge_uid {
            self.u_data.items.reattach(charge_uid);
        }
        // Put module back to the position it was removed from
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
        match pos_mode {
            RemoveMode::Free => u_fit_rack.place(pos, module_uid),
            RemoveMode::Remove => {
                for shift_module_uid in u_fit_rack.insert(pos, module_uid) {
                    let shift_u_module = self.u_data.items.get_mut(shift_module_uid).dc_module_mut().unwrap();
                    shift_u_module.set_pos(shift_u_module.get_pos().inc_unchecked())
                }
            }
        }
        self.attach_module(module_uid, charge_uid);
        // Only reattached modules and charges can have projections. Charge projects whatever its
        // module projects
        if has_projs {
            SolCore::util_reg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, module_uid);
            if let Some(charge_uid) = charge_uid {
                SolCore::util_reg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, charge_uid);
            }
        }
    }
    fn attach_module(&mut self, module_uid: UItemId, charge_uid: Option<UItemId>) {
        // Add module to services. While adding module, effect updates structure records if charge
        // needs to be activated
        SolCore::util_add_module(&mut self.u_data, &mut self.svc, module_uid, &mut self.reuse.eupdates);
        if let Some(charge_uid) = charge_uid {
            if self.reuse.eupdates.charge.unwrap_or(false) {
                let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
                u_charge.set_activated(true);
            }
            SolCore::util_add_charge(&mut self.u_data, &mut self.svc, charge_uid, &mut self.reuse.eupdates);
        }
    }
}
