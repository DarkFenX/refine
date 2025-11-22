use itertools::Itertools;

use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{ChargeMut, ModuleMut},
    },
    ud::{UCharge, UEffectUpdates, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_charge(
        &mut self,
        module_key: UItemKey,
        charge_type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_module = self.u_data.items.get(module_key).get_module().unwrap();
        let fit_key = u_module.get_fit_key();
        let module_projs = u_module.get_projs().iter().collect_vec();
        let mut activated = None;
        // Remove old charge, if it was set
        if let Some(old_charge_key) = u_module.get_charge_key() {
            // Remove outgoing projections
            let old_u_charge = self.u_data.items.get(old_charge_key).get_charge().unwrap();
            activated = Some(old_u_charge.get_activated());
            if !old_u_charge.get_projs().is_empty() {
                for (projectee_key, _) in old_u_charge.get_projs().iter() {
                    // Update services for charge being removed
                    SolarSystem::util_remove_item_projection(
                        &self.u_data,
                        &mut self.svc,
                        old_charge_key,
                        projectee_key,
                    );
                    // Update user data for charge - do not touch projections container on charge
                    // itself, because we're removing it anyway
                    self.rev_projs.unreg_projectee(&old_charge_key, projectee_key);
                }
                let old_u_charge = self.u_data.items.get_mut(old_charge_key).get_charge_mut().unwrap();
                old_u_charge.get_projs_mut().clear();
            }
            // Update services for charge being removed
            SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, old_charge_key, reuse_eupdates);
            // Update user data for charge - do not update module<->charge references because charge
            // will be removed, and module will be updated later
            self.u_data.items.remove(old_charge_key);
        };
        let activated = match activated {
            // Use data from old charge for activation, if it is available
            Some(activated) => activated,
            // Otherwise calculate from scratch
            None => {
                let module_u_item = self.u_data.items.get(module_key);
                let u_module = module_u_item.get_module().unwrap();
                match u_module.get_defeff_key() {
                    Some(Some(defeff_key)) => {
                        self.u_data.src.get_effect(defeff_key).activates_charge()
                            && u_module.get_reffs().is_some_and(|v| v.contains(&defeff_key))
                    }
                    _ => false,
                }
            }
        };
        // Set new charge
        let charge_item_id = self.u_data.items.alloc_id();
        // Update user data
        let u_charge = UCharge::new(
            charge_item_id,
            charge_type_id,
            fit_key,
            module_key,
            activated,
            false,
            &self.u_data.src,
        );
        let charge_u_item = UItem::Charge(u_charge);
        let new_charge_key = self.u_data.items.add(charge_u_item);
        let u_module = self.u_data.items.get_mut(module_key).get_module_mut().unwrap();
        u_module.set_charge_key(Some(new_charge_key));
        // Update services
        SolarSystem::util_add_charge(&mut self.u_data, &mut self.svc, new_charge_key, reuse_eupdates);
        // Reapply module projections to charge
        if !module_projs.is_empty() {
            let u_charge = self.u_data.items.get_mut(new_charge_key).get_charge_mut().unwrap();
            for (projectee_key, range) in module_projs.into_iter() {
                u_charge.get_projs_mut().add(projectee_key, range);
                self.rev_projs.reg_projectee(new_charge_key, projectee_key);
            }
            let new_u_charge = self.u_data.items.get(new_charge_key).get_charge().unwrap();
            for (projectee_key, range) in new_u_charge.get_projs().iter() {
                // Update services for charge
                SolarSystem::util_add_item_projection(
                    &self.u_data,
                    &mut self.svc,
                    new_charge_key,
                    projectee_key,
                    range,
                );
            }
        }
        new_charge_key
    }
}

impl<'a> ModuleMut<'a> {
    pub fn set_charge_type_id(&mut self, charge_type_id: ItemTypeId) -> ChargeMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let charge_key = self
            .sol
            .internal_set_module_charge(self.key, charge_type_id, &mut reuse_eupdates);
        ChargeMut::new(self.sol, charge_key)
    }
}
