use crate::{
    ad::AItemId,
    sol::{SolCore, UndoStep},
    ud::{
        UItemId,
        err::{ItemMutatedError, MutatorInplaceError},
    },
};

impl SolCore {
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
        // Item type normally stays the same (mutators share mutated item type, or mutation stays
        // dormant), in which case only attribute values can change
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        match u_drone.try_set_mutator_type_aid_inplace(mutator_type_aid, &self.u_data.r_data) {
            Ok(changed_attr_rids) => {
                if self.undo.is_recording() {
                    self.undo
                        .record(UndoStep::DroneMutator(drone_uid, old_mutator_type_aid));
                }
                for attr_rid in changed_attr_rids.into_iter() {
                    self.svc
                        .notify_base_attr_value_changed(&self.u_data, drone_uid, attr_rid);
                }
                return Ok(());
            }
            Err(MutatorInplaceError::NotMutated(_)) => unreachable!("item is checked to be mutated"),
            Err(MutatorInplaceError::MutatedTypeChange) => (),
        }
        SolCore::util_remove_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.reuse.eupdates);
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        u_drone
            .set_mutator_type_aid_replace(mutator_type_aid, &self.u_data.r_data)
            .unwrap();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::DroneMutator(drone_uid, old_mutator_type_aid));
        }
        SolCore::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_uid);
        SolCore::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.reuse.eupdates);
        Ok(())
    }
}
