use crate::{
    ad::AItemId,
    api::{ChargeMut, ItemTypeId, ModuleMut},
    sol::{SolarSystem, UndoStep},
    ud::{UCharge, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_charge(
        &mut self,
        module_uid: UItemId,
        charge_type_aid: AItemId,
    ) -> UItemId {
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        let fit_uid = u_module.get_fit_uid();
        let activated = match u_module.get_charge_uid() {
            // Use data from old charge for activation, if it is available
            Some(old_charge_uid) => {
                let old_u_charge = self.u_data.items.get(old_charge_uid).dc_charge().unwrap();
                let activated = old_u_charge.get_activated();
                self.internal_remove_charge(old_charge_uid);
                activated
            }
            // Otherwise calculate from scratch
            None => match u_module.get_r_item_base() {
                Some(rib) if let Some(defeff_rid) = rib.defeff_rid => {
                    self.u_data.r_data.get_effect_by_rid(defeff_rid).activates_charge()
                        && u_module.get_reffs().is_some_and(|v| v.contains(&defeff_rid))
                }
                _ => false,
            },
        };
        // Set new charge
        let charge_item_id = self.u_data.items.alloc_id();
        let mut u_charge = UCharge::new(
            charge_item_id,
            charge_type_aid,
            fit_uid,
            module_uid,
            activated,
            false,
            &self.u_data.r_data,
        );
        // Charge projects whatever its module projects
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        for (projectee_uid, range) in u_module.get_projs().iter() {
            u_charge.get_projs_mut().add(projectee_uid, range);
        }
        let charge_u_item = UItem::Charge(u_charge);
        let charge_uid = self.u_data.items.add(charge_u_item);
        self.attach_charge(module_uid, charge_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::ChargeAdded(charge_uid));
        }
        charge_uid
    }
    pub(in crate::api) fn internal_reattach_charge(&mut self, charge_uid: UItemId) {
        self.u_data.items.reattach(charge_uid);
        let u_charge = self.u_data.items.get(charge_uid).dc_charge().unwrap();
        let module_uid = u_charge.get_cont_item_uid();
        self.attach_charge(module_uid, charge_uid);
    }
    fn attach_charge(&mut self, module_uid: UItemId, charge_uid: UItemId) {
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.set_charge_uid(Some(charge_uid));
        // Update services
        SolarSystem::util_add_charge(&mut self.u_data, &mut self.svc, charge_uid, &mut self.cache.eupdates);
        // Apply projections
        let u_charge = self.u_data.items.get(charge_uid).dc_charge().unwrap();
        for (projectee_uid, range) in u_charge.get_projs().iter() {
            self.rev_projs.reg_projectee(charge_uid, projectee_uid);
            SolarSystem::util_add_item_projection(&self.u_data, &mut self.svc, charge_uid, projectee_uid, range);
        }
    }
}

impl<'s> ModuleMut<'s> {
    pub fn set_charge_type_id(&mut self, charge_type_id: ItemTypeId) -> ChargeMut<'_> {
        let charge_uid = self.sol.internal_set_module_charge(self.uid, charge_type_id.into_aid());
        ChargeMut::new(self.sol, charge_uid)
    }
}
