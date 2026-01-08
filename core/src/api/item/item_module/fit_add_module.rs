use super::shared::get_fit_rack_mut;
use crate::{
    ad::AItemId,
    api::{AddMode, FitMut, ItemTypeId, ModuleMut, ModuleState, RmMode},
    misc::{Index, ModRack},
    sol::SolarSystem,
    ud::{UCharge, UEffectUpdates, UFitId, UItem, UItemId, UItemMutationRequest, UModule},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_module(
        &mut self,
        fit_uid: UFitId,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: AItemId,
        state: ModuleState,
        mutation: Option<UItemMutationRequest>,
        charge_type_id: Option<AItemId>,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let module_item_id = self.u_data.items.alloc_id();
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
        // Assume some random position for now; it will be overwritten later. Record effects to
        // start into the effect container
        let u_module = UModule::new(
            module_item_id,
            type_id,
            fit_uid,
            state,
            rack,
            Index::ZERO,
            mutation,
            None,
            &self.u_data.src,
        );
        let module_u_item = UItem::Module(u_module);
        let module_uid = self.u_data.items.add(module_u_item);
        // Calculate position for the module and update part of user data (fit rack and modules from
        // it)
        let pos = match pos_mode {
            // Add to the end of module rack
            AddMode::Append => Index::from_usize(u_fit_rack.append(module_uid)),
            // Take first spare slot in the rack
            AddMode::Equip => Index::from_usize(u_fit_rack.equip(module_uid)),
            // Insert at specified position, shifting other modules to the right
            AddMode::Insert(pos) => {
                // True means inserted module is not the last in the rack
                if u_fit_rack.insert(pos.into_usize(), module_uid) {
                    for (i, rack_module_uid) in u_fit_rack.inner()[pos.into_usize() + 1..].iter().enumerate() {
                        if let Some(rack_module_uid) = rack_module_uid {
                            self.u_data
                                .items
                                .get_mut(*rack_module_uid)
                                .dc_module_mut()
                                .unwrap()
                                .set_pos(Index::from_usize(pos.into_usize() + 1 + i));
                        }
                    }
                }
                pos
            }
            // Check if there is a module on position we want to have module, and if yes, remove it
            // before adding new one
            AddMode::Replace(pos) => {
                match u_fit_rack.get(pos.into_usize()) {
                    Some(old_module_uid) => {
                        self.internal_remove_module(old_module_uid, RmMode::Free, reuse_eupdates);
                        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
                        u_fit_rack.place(pos.into_usize(), module_uid);
                    }
                    None => u_fit_rack.place(pos.into_usize(), module_uid),
                }
                pos
            }
        };
        // Create and add charge
        let charge_uid = match charge_type_id {
            Some(charge_type_id) => {
                let charge_item_id = self.u_data.items.alloc_id();
                // Update user data with new charge info
                let u_charge = UCharge::new(
                    charge_item_id,
                    charge_type_id,
                    fit_uid,
                    module_uid,
                    false,
                    false,
                    &self.u_data.src,
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
        // Add module to services. While adding module, effect updates structure records if charge
        // needs to be activated
        SolarSystem::util_add_module(&mut self.u_data, &mut self.svc, module_uid, reuse_eupdates);
        if let Some(charge_uid) = charge_uid {
            if reuse_eupdates.charge.unwrap_or(false) {
                let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
                u_charge.set_activated(true);
            }
            SolarSystem::util_add_charge(&mut self.u_data, &mut self.svc, charge_uid, reuse_eupdates);
        }
        module_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn add_module(
        &mut self,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
    ) -> ModuleMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let module_uid = self.sol.internal_add_module(
            self.uid,
            rack,
            pos_mode,
            type_id.into_aid(),
            state,
            None,
            None,
            &mut reuse_eupdates,
        );
        ModuleMut::new(self.sol, module_uid)
    }
}
