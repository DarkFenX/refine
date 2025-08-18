use crate::{
    misc::Coordinates,
    sol::{SolarSystem, api::DroneMut},
    ud::{UCoordinates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_coordinates(
        &mut self,
        drone_key: UItemKey,
        u_coordinates: UCoordinates,
    ) {
        let u_drone = self.u_data.items.get_mut(drone_key).get_drone_mut().unwrap();
        if u_drone.get_position().coordinates == u_coordinates {
            return;
        }
        u_drone.get_position_mut().coordinates = u_coordinates;
        SolarSystem::util_update_drone_position(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_key);
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.sol.internal_set_drone_coordinates(self.key, coordinates.into())
    }
}
