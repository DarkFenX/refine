use crate::{
    sol::{SolarSystem, api::DroneMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone(
        &mut self,
        drone_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Remove incoming projections
        self.internal_remove_incoming_projections(drone_key);
        // Remove outgoing projections
        let u_drone = self.u_data.items.get(drone_key).get_drone().unwrap();
        let fit_key = u_drone.get_fit_key();
        if !u_drone.get_projs().is_empty() {
            for projectee_key in u_drone.get_projs().iter_projectees() {
                SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, drone_key, projectee_key);
                self.rev_projs.unreg_projectee(&drone_key, &projectee_key);
            }
            let u_drone = self.u_data.items.get_mut(drone_key).get_drone_mut().unwrap();
            u_drone.get_projs_mut().clear();
        }
        // Update services
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        // Update user data
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.drones.remove(&drone_key);
        self.u_data.items.remove(drone_key);
    }
}

impl<'a> DroneMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_drone(self.key, &mut reuse_eupdates);
    }
}
