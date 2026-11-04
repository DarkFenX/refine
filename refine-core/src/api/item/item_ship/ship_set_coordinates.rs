use crate::{
    Coordinates, Ship,
    misc::Xyz,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Ship<'s> {
    /// Set ship position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.core
            .internal_set_ship_coordinates(self.uid, coordinates.into_xyz())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_ship_coordinates(&mut self, ship_uid: UItemId, coordinates: Xyz) {
        let u_ship = self.u_data.items.get_mut(ship_uid).dc_ship_mut().unwrap();
        if u_ship.get_physics().coordinates == coordinates {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record_physics(UndoStep::ShipPhysics(ship_uid), *u_ship.get_physics());
        }
        u_ship.get_physics_mut().coordinates = coordinates;
        CoreInner::util_update_ship_physics(&mut self.u_data, &self.rev_projs, &mut self.svc, ship_uid);
    }
}
