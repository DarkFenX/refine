use crate::{
    sol::{SolarSystem, api::DroneMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Remove outgoing projections
        let u_item = self.u_data.items.get(item_key);
        let u_drone = u_item.get_drone().unwrap();
        let fit_key = u_drone.get_fit_key();
        for projectee_key in u_drone.get_projs().iter_projectees() {
            let projectee_u_item = self.u_data.items.get(projectee_key);
            SolarSystem::util_remove_item_projection(
                &self.u_data,
                &mut self.svc,
                item_key,
                u_item,
                projectee_key,
                projectee_u_item,
            );
            self.rprojs.unreg_projectee(&item_key, &projectee_key);
        }
        // Remove incoming projections
        self.internal_remove_incoming_projections(item_key);
        // Update services
        let u_item = self.u_data.items.get(item_key);
        SolarSystem::util_remove_item_without_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        // Update user data
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.drones.remove(&item_key);
        self.u_data.items.remove(item_key);
    }
}

impl<'a> DroneMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_drone(self.key, &mut reuse_eupdates);
    }
}
