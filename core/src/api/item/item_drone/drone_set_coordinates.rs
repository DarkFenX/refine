use crate::{
    api::{Coordinates, DroneMut},
    misc::Xyz,
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_coordinates(&mut self, drone_uid: UItemId, coordinates: Xyz) {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        if u_drone.get_physics().coordinates == coordinates {
            return;
        }
        u_drone.get_physics_mut().coordinates = coordinates;
        SolarSystem::util_update_drone_physics(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_uid);
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.sol
            .internal_set_drone_coordinates(self.uid, coordinates.into_xyz())
    }
}
