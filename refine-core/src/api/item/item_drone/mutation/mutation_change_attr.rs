use crate::{
    sol::{SolarSystem, UndoStep},
    ud::{UAttrMutationRequest, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api::item) fn internal_change_drone_mutation_attr(
        &mut self,
        drone_uid: UItemId,
        mut attr_mutation: UAttrMutationRequest,
    ) -> Result<(), ItemMutatedError> {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        let new_roll = attr_mutation.roll;
        let changed_attr_rid = u_drone.change_mutation_attr(&self.u_data.r_data, &mut attr_mutation)?;
        // Request now carries replaced roll, which is the same if nothing changed
        if self.undo.is_recording() && attr_mutation.roll != new_roll {
            self.undo
                .record_mutation_attr_single(UndoStep::DroneMutationAttrSingle(drone_uid), attr_mutation);
        }
        if let Some(attr_rid) = changed_attr_rid {
            self.svc
                .notify_base_attr_value_changed(&self.u_data, drone_uid, attr_rid);
        }
        Ok(())
    }
    pub(in crate::api::item) fn internal_change_drone_mutation_attrs(
        &mut self,
        drone_uid: UItemId,
        mut attr_mutations: Vec<UAttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        let changed_attr_rids = u_drone.change_mutation_attrs(&self.u_data.r_data, &mut attr_mutations)?;
        if self.undo.is_recording() {
            // Reversed just to handle multiple requests for the same attribute correctly
            attr_mutations.reverse();
            self.undo
                .record_mutation_attr_batch(UndoStep::DroneMutationAttrBatch(drone_uid), attr_mutations);
        }
        for attr_rid in changed_attr_rids.into_iter() {
            self.svc
                .notify_base_attr_value_changed(&self.u_data, drone_uid, attr_rid);
        }
        Ok(())
    }
}
