use crate::sol::{ItemKey, SolarSystem, api::DroneMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone(&mut self, item_key: ItemKey) {
        // Remove outgoing projections
        let uad_item = self.uad.items.get(item_key);
        let uad_drone = uad_item.get_drone().unwrap();
        let fit_key = uad_drone.get_fit_key();
        for &projectee_item_key in uad_drone.get_projs().iter_projectee_item_keys() {
            let projectee_uad_item = self.uad.items.get(projectee_item_key);
            SolarSystem::util_remove_item_projection(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                item_key,
                uad_item,
                projectee_item_key,
                projectee_uad_item,
            );
            self.rprojs.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove incoming projections
        self.internal_remove_incoming_projections(item_key);
        // Update services
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_remove_item_without_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Update user data
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.drones.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> DroneMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_drone(self.key);
    }
}
