use crate::{
    ad::AItemId,
    api::{DroneMut, ItemTypeId},
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_type_aid(&mut self, drone_uid: UItemId, type_aid: AItemId) {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        let old_type_aid = u_drone.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        // All base types a mutator applies to share mutated item type, in which case only attribute
        // values can change. Effective mutation is checked here, so that type changes of unmutated
        // items, the most common ones, do not pay for the call
        if u_drone.get_mutation_data().is_some_and(|v| v.is_effective())
            && let Some(changed_attr_rids) = u_drone.try_set_base_type_aid_inplace(type_aid, &self.u_data.r_data)
        {
            if self.undo.is_recording() {
                self.undo.record(UndoStep::DroneType(drone_uid, old_type_aid));
            }
            for attr_rid in changed_attr_rids.into_iter() {
                self.svc
                    .notify_base_attr_value_changed(&self.u_data, drone_uid, attr_rid);
            }
            return;
        }
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.reuse.eupdates);
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        u_drone.set_base_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::DroneType(drone_uid, old_type_aid));
        }
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_uid);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.reuse.eupdates);
    }
}

impl<'s> DroneMut<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data. If
    /// item is mutated, base item type ID is updated.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_drone_type_aid(self.uid, type_id.into_aid())
    }
}
