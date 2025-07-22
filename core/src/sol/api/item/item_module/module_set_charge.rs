use itertools::Itertools;

use crate::{
    ad,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{ChargeMut, ModuleMut},
    },
    uad::{UadCharge, UadEffectUpdates, UadItem, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_charge(
        &mut self,
        item_key: UadItemKey,
        charge_a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> UadItemKey {
        let uad_module = self.uad.items.get(item_key).get_module().unwrap();
        let fit_key = uad_module.get_fit_key();
        let module_a_state = uad_module.get_a_state();
        let module_projs = uad_module.get_projs().iter().collect_vec();
        // Remove old charge, if it was set
        if let Some(old_charge_key) = uad_module.get_charge_key() {
            // Remove outgoing projections
            let old_charge_uad_item = self.uad.items.get(old_charge_key);
            // Use module projections because they should be identical
            for (projectee_key, _) in module_projs.iter() {
                // Update services for charge being removed
                let projectee_uad_item = self.uad.items.get(*projectee_key);
                SolarSystem::util_remove_item_projection(
                    &self.uad,
                    &mut self.svc,
                    old_charge_key,
                    old_charge_uad_item,
                    *projectee_key,
                    projectee_uad_item,
                );
                // Update user data for charge - do not touch projections container on charge
                // itself, because we're removing it anyway
                self.rprojs.unreg_projectee(&old_charge_key, projectee_key);
            }
            // Update services for charge being removed
            SolarSystem::util_remove_item_without_projs(
                &self.uad,
                &mut self.svc,
                old_charge_key,
                old_charge_uad_item,
                reuse_eupdates,
            );
            // Update user data for charge - do not update module<->charge references because charge
            // will be removed, and module will be updated later
            self.uad.items.remove(old_charge_key);
        };
        // Set new charge
        let charge_item_id = self.uad.items.alloc_id();
        // Update user data
        let mut uad_charge = UadCharge::new(
            charge_item_id,
            charge_a_item_id,
            fit_key,
            item_key,
            module_a_state,
            false,
            &self.uad.src,
            reuse_eupdates,
        );
        for (projectee_key, range) in module_projs.into_iter() {
            uad_charge.get_projs_mut().add(projectee_key, range);
        }
        let charge_uad_item = UadItem::Charge(uad_charge);
        let new_charge_key = self.uad.items.add(charge_uad_item);
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        uad_module.set_charge_key(Some(new_charge_key));
        // Update services
        let new_charge_uad_item = self.uad.items.get(new_charge_key);
        SolarSystem::util_add_item_without_projs(
            &self.uad,
            &mut self.svc,
            new_charge_key,
            new_charge_uad_item,
            reuse_eupdates,
        );
        // Reapply module projections to charge
        // Update user data for charge
        for (projectee_key, range) in new_charge_uad_item.get_charge().unwrap().get_projs().iter() {
            self.rprojs.reg_projectee(new_charge_key, projectee_key);
            // Update services for charge
            let projectee_uad_item = self.uad.items.get(projectee_key);
            SolarSystem::util_add_item_projection(
                &self.uad,
                &mut self.svc,
                new_charge_key,
                new_charge_uad_item,
                projectee_key,
                projectee_uad_item,
                range,
            );
        }
        new_charge_key
    }
}

impl<'a> ModuleMut<'a> {
    pub fn set_charge_type_id(&mut self, charge_type_id: ItemTypeId) -> ChargeMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let charge_key = self
            .sol
            .internal_set_module_charge(self.key, charge_type_id, &mut reuse_eupdates);
        ChargeMut::new(self.sol, charge_key)
    }
}
