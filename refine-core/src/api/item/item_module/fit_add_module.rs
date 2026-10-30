use super::shared::get_fit_rack_mut;
use crate::{
    ad::AItemId,
    api::{AddMode, FitMut, ItemTypeId, ModuleMut, ModuleState, RemoveMode},
    misc::ModRack,
    num::Index,
    sol::{SolarSystem, UndoStep},
    ud::{UCharge, UFitId, UItem, UItemId, UItemMutationRequest, UModule},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_module(
        &mut self,
        fit_uid: UFitId,
        rack: ModRack,
        pos_mode: AddMode,
        type_aid: AItemId,
        state: ModuleState,
        mutation: Option<UItemMutationRequest>,
        charge_type_aid: Option<AItemId>,
    ) -> UItemId {
        let module_item_id = self.u_data.items.alloc_id();
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
        // Assume some random position for now; it will be overwritten later. Record effects to
        // start into the effect container
        let u_module = UModule::new(
            module_item_id,
            type_aid,
            fit_uid,
            state,
            rack,
            Index::ZERO,
            mutation,
            None,
            &self.u_data.r_data,
        );
        let module_u_item = UItem::Module(u_module);
        let module_uid = self.u_data.items.add(module_u_item);
        // Calculate position for the module and update part of user data (fit rack and modules from
        // it)
        let pos = match pos_mode {
            // Add to the end of module rack
            AddMode::Append => u_fit_rack.append(module_uid),
            // Take first spare slot in the rack
            AddMode::Equip => u_fit_rack.equip(module_uid),
            // Insert at specified position, shifting other modules to the right
            AddMode::Insert(pos) => {
                for shift_module_uid in u_fit_rack.insert(pos, module_uid) {
                    let shift_u_module = self.u_data.items.get_mut(shift_module_uid).dc_module_mut().unwrap();
                    shift_u_module.set_pos(shift_u_module.get_pos() + Index::ONE)
                }
                pos
            }
            // Check if there is a module on position we want to have module, and if yes, remove it
            // before adding new one
            AddMode::Replace(pos) => {
                match u_fit_rack.get(pos) {
                    Some(old_module_uid) => {
                        self.internal_remove_module(old_module_uid, RemoveMode::Free);
                        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
                        u_fit_rack.place(pos, module_uid);
                    }
                    None => u_fit_rack.place(pos, module_uid),
                }
                pos
            }
        };
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
        // Update on-module data regarding position and charge
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.set_pos(pos);
        u_module.set_charge_uid(charge_uid);
        self.attach_module(module_uid, charge_uid);
        if self.undo.is_recording() {
            // Inserted module shifted other modules, so it has to shift them back when removed
            let undo_pos_mode = match pos_mode {
                AddMode::Insert(_) => RemoveMode::Remove,
                AddMode::Append | AddMode::Equip | AddMode::Replace(_) => RemoveMode::Free,
            };
            self.undo.record(UndoStep::ModuleAdded(module_uid, undo_pos_mode));
        }
        module_uid
    }
    pub(in crate::api) fn internal_reattach_module(&mut self, module_uid: UItemId, pos_mode: RemoveMode) {
        self.u_data.items.reattach(module_uid);
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        let fit_uid = u_module.get_fit_uid();
        let rack = u_module.get_rack();
        let pos = u_module.get_pos();
        let charge_uid = u_module.get_charge_uid();
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
                    shift_u_module.set_pos(shift_u_module.get_pos() + Index::ONE)
                }
            }
        }
        self.attach_module(module_uid, charge_uid);
    }
    fn attach_module(&mut self, module_uid: UItemId, charge_uid: Option<UItemId>) {
        // Add module to services. While adding module, effect updates structure records if charge
        // needs to be activated
        SolarSystem::util_add_module(&mut self.u_data, &mut self.svc, module_uid, &mut self.cache.eupdates);
        SolarSystem::util_reg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, module_uid);
        if let Some(charge_uid) = charge_uid {
            if self.cache.eupdates.charge.unwrap_or(false) {
                let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
                u_charge.set_activated(true);
            }
            SolarSystem::util_add_charge(&mut self.u_data, &mut self.svc, charge_uid, &mut self.cache.eupdates);
            SolarSystem::util_reg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, charge_uid);
        }
    }
}

impl<'s> FitMut<'s> {
    pub fn add_module(
        &mut self,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
    ) -> ModuleMut<'_> {
        let module_uid = self
            .sol
            .internal_add_module(self.uid, rack, pos_mode, type_id.into_aid(), state, None, None);
        ModuleMut::new(self.sol, module_uid)
    }
}
