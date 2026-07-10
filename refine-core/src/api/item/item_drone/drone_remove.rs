use crate::{
    api::DroneMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_drone(&mut self, drone_uid: UItemId, reuse_eupdates: &mut UEffectUpdates) {
        // Remove incoming projections
        self.internal_remove_incoming_projections(drone_uid);
        // Remove outgoing projections
        let u_drone = self.u_data.items.get(drone_uid).dc_drone().unwrap();
        let fit_uid = u_drone.get_fit_uid();
        if !u_drone.get_projs().is_empty() {
            for projectee_uid in u_drone.get_projs().iter_projectees() {
                SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, drone_uid, projectee_uid);
                self.rev_projs.unreg_projectee(&drone_uid, projectee_uid);
            }
            let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
            u_drone.get_projs_mut().clear();
        }
        // Update services
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_uid, reuse_eupdates);
        // Update user data
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.drones.remove(&drone_uid);
        self.u_data.items.remove(drone_uid);
    }
}

impl<'a> DroneMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_drone(self.uid, &mut reuse_eupdates);
    }
}
