use super::shared::get_fit_rack_mut;
use crate::{
    ad,
    def::{FitKey, ItemKey, ItemTypeId},
    misc::{AddMode, ItemMutationRequest, ModRack, ModuleState, RmMode},
    sol::{
        SolarSystem,
        api::{FitMut, ModuleMut},
    },
    uad::{UadCharge, UadEffectUpdates, UadItem, UadModule},
};

struct ChargeData {
    item_key: ItemKey,
    eupdates: UadEffectUpdates,
}

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_module(
        &mut self,
        fit_key: FitKey,
        rack: ModRack,
        pos_mode: AddMode,
        a_item_id: ad::AItemId,
        state: ModuleState,
        mutation: Option<ItemMutationRequest>,
        charge_a_item_id: Option<ad::AItemId>,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let module_item_id = self.uad.items.alloc_id();
        let uad_fit_rack = get_fit_rack_mut(&mut self.uad.fits, fit_key, rack);
        // Assume some random position for now; it will be overwritten later. Record effects to
        // start into the effect container
        let uad_module = UadModule::new(
            module_item_id,
            a_item_id,
            fit_key,
            state,
            rack,
            0,
            mutation,
            None,
            &self.uad.src,
            reuse_eupdates,
        );
        let module_uad_item = UadItem::Module(uad_module);
        let module_key = self.uad.items.add(module_uad_item);
        // Calculate position for the module and update part of user data (fit rack and modules from
        // it)
        let pos = match pos_mode {
            // Add to the end of module rack
            AddMode::Append => uad_fit_rack.append(module_key),
            // Take first spare slot in the rack
            AddMode::Equip => uad_fit_rack.equip(module_key),
            // Insert at specified position, shifting other modules to the right
            AddMode::Insert(pos) => {
                // True means inserted module is not the last in the rack
                if uad_fit_rack.insert(pos, module_key) {
                    for (i, rack_module_key) in uad_fit_rack.inner()[pos + 1..].iter().enumerate() {
                        if let Some(rack_module_key) = rack_module_key {
                            self.uad
                                .items
                                .get_mut(*rack_module_key)
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
                match uad_fit_rack.get(pos) {
                    Some(old_module_key) => {
                        // Create another effect update container for module being removed, since
                        // primary one is carrying effect info of the primary module being added
                        let mut replacee_eupdates = UadEffectUpdates::new();
                        self.internal_remove_module(old_module_key, RmMode::Free, &mut replacee_eupdates);
                        let uad_fit_rack = get_fit_rack_mut(&mut self.uad.fits, fit_key, rack);
                        uad_fit_rack.place(pos, module_key);
                    }
                    None => uad_fit_rack.place(pos, module_key),
                }
                pos
            }
        };
        // Create and add charge
        let charge_data = match charge_a_item_id {
            Some(charge_type_id) => {
                let charge_item_id = self.uad.items.alloc_id();
                // Create new container to carry info about charge effects, external one is carrying
                // module effects
                let mut charge_eupdates = UadEffectUpdates::new();
                // Update user data with new charge info
                let uad_charge = UadCharge::new(
                    charge_item_id,
                    charge_type_id,
                    fit_key,
                    module_key,
                    state.into(),
                    false,
                    &self.uad.src,
                    &mut charge_eupdates,
                );
                let charge_uad_item = UadItem::Charge(uad_charge);
                let charge_key = self.uad.items.add(charge_uad_item);
                Some(ChargeData {
                    item_key: charge_key,
                    eupdates: charge_eupdates,
                })
            }
            None => None,
        };
        // Update on-module data regarding position and charge
        let uad_module = self.uad.items.get_mut(module_key).get_module_mut().unwrap();
        uad_module.set_pos(pos);
        uad_module.set_charge_key(charge_data.as_ref().map(|v| v.item_key));
        // Add module and charge to services. For module effects, use the container which has been
        // passed to the method from the caller
        let module_uad_item = self.uad.items.get(module_key);
        SolarSystem::util_add_item_without_projs(&self.uad, &mut self.svc, module_key, module_uad_item, reuse_eupdates);
        if let Some(charge_data) = charge_data {
            let charge_uad_item = self.uad.items.get(charge_data.item_key);
            SolarSystem::util_add_item_without_projs(
                &self.uad,
                &mut self.svc,
                charge_data.item_key,
                charge_uad_item,
                &charge_data.eupdates,
            );
        }
        module_key
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
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.sol.internal_add_module(
            self.key,
            rack,
            pos_mode,
            type_id,
            state,
            None,
            None,
            &mut reuse_eupdates,
        );
        ModuleMut::new(self.sol, item_key)
    }
}
