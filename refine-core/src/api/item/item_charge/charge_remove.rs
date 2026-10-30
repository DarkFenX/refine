use crate::{
    api::ChargeMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_charge(&mut self, charge_uid: UItemId) {
        let u_charge = self.u_data.items.get(charge_uid).dc_charge().unwrap();
        let module_uid = u_charge.get_cont_item_uid();
        // Remove outgoing projections; charge itself keeps them, in case it gets reattached
        for projectee_uid in u_charge.get_projs().iter_projectees() {
            // Update services
            SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, charge_uid, projectee_uid);
            // Reverse projections
            self.rev_projs.unreg_projectee(&charge_uid, projectee_uid);
        }
        // Update services
        SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_uid, &mut self.cache.eupdates);
        // Update user data
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.set_charge_uid(None);
        match self.undo.is_recording() {
            true => {
                self.u_data.items.detach(charge_uid);
                self.undo.record(UndoStep::ChargeRemoved(charge_uid));
            }
            false => {
                self.u_data.items.remove(charge_uid);
            }
        }
    }
}

impl<'s> ChargeMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_charge(self.uid)
    }
}
