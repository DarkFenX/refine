use itertools::Itertools;

use crate::sol::{
    ItemKey, ItemTypeId, SolarSystem,
    api::{ChargeMut, ModuleMut},
    uad::item::{UadCharge, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_charge(
        &mut self,
        item_key: ItemKey,
        charge_type_id: ItemTypeId,
    ) -> ItemKey {
        let uad_module = self.uad.items.get(item_key).get_module().unwrap();
        let fit_id = uad_module.get_fit_key();
        let module_a_state = uad_module.get_a_state();
        let module_projs = uad_module
            .get_projs()
            .iter()
            .map(|(projectee_item_key, range)| (*projectee_item_key, *range))
            .collect_vec();
        // Remove old charge, if it was set
        if let Some(old_charge_key) = uad_module.get_charge_item_key() {
            // Remove outgoing projections
            let charge_uad_item = self.uad.items.get(old_charge_key);
            // Use module projections because they should be identical
            for (projectee_item_key, _) in module_projs.iter() {
                // Update services for charge being removed
                let projectee_uad_item = self.uad.items.get(*projectee_item_key);
                self.svc
                    .remove_item_projection(&self.uad, old_charge_key, *projectee_item_key, projectee_uad_item);
                // Update user data for charge - do not touch projections container on charge
                // itself, because we're removing it anyway
                self.proj_tracker.unreg_projectee(&old_charge_key, projectee_item_key);
            }
            // Update services for charge being removed
            self.svc.remove_item(&self.uad, old_charge_key, charge_uad_item);
            // Update user data for charge - do not update module<->charge references because charge
            // will be removed, and module will be updated later
            self.uad.items.remove(old_charge_key);
        };
        // Set new charge
        // Allocation can fail only if we didn't remove charge first, so if it fails - we don't need
        // to restore anything
        let charge_id = self.uad.items.alloc_id();
        // Update user data
        let uad_charge = UadCharge::new(
            &self.uad.src,
            charge_id,
            charge_type_id,
            fit_id,
            item_key,
            module_a_state,
            false,
        );
        let charge_uad_item = UadItem::Charge(uad_charge);
        let new_charge_key = self.uad.items.add(charge_uad_item);
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        uad_module.set_charge_item_key(Some(new_charge_key));
        // Update services
        self.internal_add_item_key_to_svc(new_charge_key);
        // Reapply module projections to charge
        if !module_projs.is_empty() {
            let charge_projs = self
                .uad
                .items
                .get_mut(new_charge_key)
                .get_charge_mut()
                .unwrap()
                .get_projs_mut();
            // Update user data for charge
            for (projectee_item_id, range) in module_projs.iter() {
                self.proj_tracker.reg_projectee(new_charge_key, *projectee_item_id);
                charge_projs.add(*projectee_item_id, *range);
            }
            // Update services for charge
            for (projectee_item_key, range) in module_projs {
                let projectee_uad_item = self.uad.items.get(projectee_item_key);
                self.svc
                    .add_item_projection(&self.uad, new_charge_key, projectee_item_key, projectee_uad_item, range);
            }
        }
        new_charge_key
    }
}

impl<'a> ModuleMut<'a> {
    pub fn set_charge(&mut self, charge_type_id: ItemTypeId) -> ChargeMut {
        let charge_key = self.sol.internal_set_module_charge(self.key, charge_type_id);
        ChargeMut::new(self.sol, charge_key)
    }
}
