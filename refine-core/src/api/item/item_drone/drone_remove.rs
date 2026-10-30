use crate::{
    api::DroneMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_drone(&mut self, drone_uid: UItemId) {
        // Remove incoming projections
        self.internal_remove_incoming_projections(drone_uid);
        // Remove outgoing projections; drone keeps them, in case it gets reattached
        let fit_uid = self.u_data.items.get(drone_uid).dc_drone().unwrap().get_fit_uid();
        SolarSystem::util_unreg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, drone_uid);
        // Update services
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
        // Update user data
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.drones.remove(&drone_uid);
        self.internal_discard_item(drone_uid, UndoStep::DroneRemoved);
    }
}

impl<'s> DroneMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_drone(self.uid);
    }
}
