use crate::{
    api::{Coordinates, ShipMut},
    misc::Xyz,
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_ship_coordinates(&mut self, ship_uid: UItemId, coordinates: Xyz) {
        let u_ship = self.u_data.items.get_mut(ship_uid).dc_ship_mut().unwrap();
        if u_ship.get_physics().coordinates == coordinates {
            return;
        }
        u_ship.get_physics_mut().coordinates = coordinates;
        SolarSystem::util_update_ship_physics(&mut self.u_data, &self.rev_projs, &mut self.svc, ship_uid);
    }
}

impl<'a> ShipMut<'a> {
    /// Set ship position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.sol.internal_set_ship_coordinates(self.uid, coordinates.into_xyz())
    }
}
