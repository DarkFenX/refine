use crate::{
    sol::SolarSystem,
    ud::{UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api::item) fn internal_remove_drone_mutation(
        &mut self,
        drone_uid: UItemId,
    ) -> Result<(), ItemMutatedError> {
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        if let Err(error) = u_drone.unmutate(&self.u_data.r_data) {
            SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
            return Err(error);
        }
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_uid);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
        Ok(())
    }
}
