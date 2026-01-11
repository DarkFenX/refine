use itertools::Itertools;

use crate::{
    ad::AItemId,
    api::{ChargeMut, ItemTypeId, ModuleMut},
    sol::SolarSystem,
    ud::{UCharge, UEffectUpdates, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_charge(
        &mut self,
        module_uid: UItemId,
        charge_type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        let fit_uid = u_module.get_fit_uid();
        let module_projs = u_module.get_projs().iter().collect_vec();
        let mut activated = None;
        // Remove old charge, if it was set
        if let Some(old_charge_uid) = u_module.get_charge_uid() {
            // Remove outgoing projections
            let old_u_charge = self.u_data.items.get(old_charge_uid).dc_charge().unwrap();
            activated = Some(old_u_charge.get_activated());
            if !old_u_charge.get_projs().is_empty() {
                for (projectee_uid, _) in old_u_charge.get_projs().iter() {
                    // Update services for charge being removed
                    SolarSystem::util_remove_item_projection(
                        &self.u_data,
                        &mut self.svc,
                        old_charge_uid,
                        projectee_uid,
                    );
                    // Update user data for charge - do not touch projections container on charge
                    // itself, because we're removing it anyway
                    self.rev_projs.unreg_projectee(&old_charge_uid, projectee_uid);
                }
                let old_u_charge = self.u_data.items.get_mut(old_charge_uid).dc_charge_mut().unwrap();
                old_u_charge.get_projs_mut().clear();
            }
            // Update services for charge being removed
            SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, old_charge_uid, reuse_eupdates);
            // Update user data for charge - do not update module<->charge references because charge
            // will be removed, and module will be updated later
            self.u_data.items.remove(old_charge_uid);
        };
        let activated = match activated {
            // Use data from old charge for activation, if it is available
            Some(activated) => activated,
            // Otherwise calculate from scratch
            None => {
                let module_u_item = self.u_data.items.get(module_uid);
                let u_module = module_u_item.dc_module().unwrap();
                match u_module.get_defeff_rid() {
                    Some(Some(defeff_rid)) => {
                        self.u_data.src.get_effect_by_rid(defeff_rid).activates_charge()
                            && u_module.get_reffs().is_some_and(|v| v.contains(&defeff_rid))
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
            charge_type_aid,
            fit_uid,
            module_uid,
            activated,
            false,
            &self.u_data.src,
        );
        let charge_u_item = UItem::Charge(u_charge);
        let new_charge_uid = self.u_data.items.add(charge_u_item);
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.set_charge_uid(Some(new_charge_uid));
        // Update services
        SolarSystem::util_add_charge(&mut self.u_data, &mut self.svc, new_charge_uid, reuse_eupdates);
        // Reapply module projections to charge
        if !module_projs.is_empty() {
            let u_charge = self.u_data.items.get_mut(new_charge_uid).dc_charge_mut().unwrap();
            for (projectee_uid, range) in module_projs.into_iter() {
                u_charge.get_projs_mut().add(projectee_uid, range);
                self.rev_projs.reg_projectee(new_charge_uid, projectee_uid);
            }
            let new_u_charge = self.u_data.items.get(new_charge_uid).dc_charge().unwrap();
            for (projectee_uid, range) in new_u_charge.get_projs().iter() {
                // Update services for charge
                SolarSystem::util_add_item_projection(
                    &self.u_data,
                    &mut self.svc,
                    new_charge_uid,
                    projectee_uid,
                    range,
                );
            }
        }
        new_charge_uid
    }
}

impl<'a> ModuleMut<'a> {
    pub fn set_charge_type_id(&mut self, charge_type_id: ItemTypeId) -> ChargeMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let charge_uid = self
            .sol
            .internal_set_module_charge(self.uid, charge_type_id.into_aid(), &mut reuse_eupdates);
        ChargeMut::new(self.sol, charge_uid)
    }
}
