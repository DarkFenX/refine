use crate::{
    Coordinates, Drone,
    misc::Xyz,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Drone<'s> {
    /// Set drone position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.core
            .internal_set_drone_coordinates(self.uid, coordinates.into_xyz())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_drone_coordinates(&mut self, drone_uid: UItemId, coordinates: Xyz) {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        if u_drone.get_physics().coordinates == coordinates {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record_physics(UndoStep::DronePhysics(drone_uid), *u_drone.get_physics());
        }
        u_drone.get_physics_mut().coordinates = coordinates;
        CoreInner::util_update_drone_physics(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_uid);
    }
}
