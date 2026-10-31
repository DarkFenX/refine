use crate::{
    ad::AItemId,
    sol::{SolarSystem, UndoStep},
    ud::{UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_mutator_type_aid(
        &mut self,
        drone_uid: UItemId,
        mutator_type_aid: AItemId,
    ) -> Result<(), ItemMutatedError> {
        let u_drone = self.u_data.items.get(drone_uid).dc_drone().unwrap();
        let Some(old_mutator_type_aid) = u_drone.get_mutator_type_aid() else {
            return Err(ItemMutatedError);
        };
        if old_mutator_type_aid == mutator_type_aid {
            return Ok(());
        }
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        u_drone
            .set_mutator_type_aid(mutator_type_aid, &self.u_data.r_data)
            .unwrap();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::DroneMutator(drone_uid, old_mutator_type_aid));
        }
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_uid);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
        Ok(())
    }
}
