use crate::{
    sol::{SolarSystem, UndoStep},
    ud::{UAttrMutationRequest, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_drone_mutation(
        &mut self,
        drone_uid: UItemId,
    ) -> Result<(), ItemMutatedError> {
        // Checked before services are touched, so that failed requests do nothing
        let u_drone = self.u_data.items.get(drone_uid).dc_drone().unwrap();
        if u_drone.get_mutator_type_aid().is_none() {
            return Err(ItemMutatedError);
        }
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        let item_mutation_data = u_drone.unmutate(&self.u_data.r_data).unwrap();
        if self.undo.is_recording() {
            let attr_mutations = item_mutation_data
                .get_attr_rolls()
                .iter()
                .map(|(&attr_aid, &roll)| UAttrMutationRequest {
                    attr_aid,
                    roll: Some(roll),
                })
                .collect();
            self.undo.record_mutation_attrs(
                UndoStep::DroneMutationRemoved(drone_uid, item_mutation_data.get_mutator_type_aid()),
                attr_mutations,
            );
        }
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_uid);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
        Ok(())
    }
}
