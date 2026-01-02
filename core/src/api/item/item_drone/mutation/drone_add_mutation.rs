use crate::{
    api::{AddMutationError, DroneMut, MutationMut},
    def::ItemTypeId,
    err::basic::ItemNotMutatedError,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId, UItemMutationRequest},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_drone_mutation(
        &mut self,
        drone_key: UItemId,
        mutation: UItemMutationRequest,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemNotMutatedError> {
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        let u_drone = self.u_data.items.get_mut(drone_key).dc_drone_mut().unwrap();
        if let Err(error) = u_drone.mutate(mutation, &self.u_data.src) {
            SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_key);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        Ok(())
    }
}

impl<'a> DroneMut<'a> {
    pub fn mutate(&mut self, mutator_id: ItemTypeId) -> Result<MutationMut<'_>, AddMutationError> {
        let mutation = UItemMutationRequest {
            mutator_id,
            attrs: Vec::new(),
        };
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_add_drone_mutation(self.key, mutation, &mut reuse_eupdates)?;
        Ok(self.get_mutation_mut().unwrap())
    }
}
