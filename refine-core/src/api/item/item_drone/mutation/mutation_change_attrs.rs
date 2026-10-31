use crate::{
    sol::{SolarSystem, UndoStep},
    ud::{UAttrMutationRequest, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_change_drone_mutation_attrs(
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
                .record_mutation_attrs(UndoStep::DroneMutationAttrs(drone_uid), attr_mutations);
        }
        for attr_rid in changed_attr_rids {
            self.svc
                .notify_base_attr_value_changed(&self.u_data, drone_uid, attr_rid);
        }
        Ok(())
    }
}
