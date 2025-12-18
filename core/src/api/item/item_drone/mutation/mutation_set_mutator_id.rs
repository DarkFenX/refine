use crate::{
    ad::AItemId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_mutator_id(
        &mut self,
        drone_key: UItemKey,
        mutator_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        let u_drone = self.u_data.items.get_mut(drone_key).dc_drone_mut().unwrap();
        if let Err(error) = u_drone.set_mutator_id(mutator_id, &self.u_data.src) {
            SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_key);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        Ok(())
    }
}
