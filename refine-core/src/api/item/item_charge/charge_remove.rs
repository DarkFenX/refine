use crate::{
    api::ChargeMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_charge(&mut self, charge_uid: UItemId) {
        let u_charge = self.u_data.items.get(charge_uid).dc_charge().unwrap();
        let module_uid = u_charge.get_cont_item_uid();
        // Remove outgoing projections; charge keeps them, in case it gets reattached
        SolarSystem::util_unreg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, charge_uid);
        // Update services
        SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_uid, &mut self.cache.eupdates);
        // Update user data
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.set_charge_uid(None);
        self.internal_discard_item(charge_uid, UndoStep::ChargeRemoved);
    }
}

impl<'s> ChargeMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_charge(self.uid)
    }
}
