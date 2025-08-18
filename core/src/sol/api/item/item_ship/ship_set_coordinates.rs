use crate::{
    misc::Coordinates,
    sol::{SolarSystem, api::ShipMut},
    ud::{UCoordinates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_coordinates(
        &mut self,
        ship_key: UItemKey,
        u_coordinates: UCoordinates,
    ) {
        let u_ship = self.u_data.items.get_mut(ship_key).get_ship_mut().unwrap();
        if u_ship.get_position().coordinates == u_coordinates {
            return;
        }
        u_ship.get_position_mut().coordinates = u_coordinates;
        SolarSystem::util_update_ship_position(&mut self.u_data, &self.rev_projs, &mut self.svc, ship_key);
    }
}

impl<'a> ShipMut<'a> {
    /// Set ship position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.sol.internal_set_ship_coordinates(self.key, coordinates.into())
    }
}
