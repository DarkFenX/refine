use crate::{
    def::AttrVal,
    misc::Movement,
    sol::{SolarSystem, api::ShipMut},
    ud::{UDirection, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_movement(
        &mut self,
        ship_key: UItemKey,
        u_direction: UDirection,
        speed: AttrVal,
    ) {
        let u_ship = self.u_data.items.get_mut(ship_key).get_ship_mut().unwrap();
        let ship_u_position = u_ship.get_position_mut();
        if ship_u_position.direction == u_direction && ship_u_position.speed == speed {
            return;
        }
        ship_u_position.direction = u_direction;
        ship_u_position.speed = speed;
        SolarSystem::util_update_ship_position(&mut self.u_data, &self.rev_projs, &mut self.svc, ship_key);
    }
}

impl<'a> ShipMut<'a> {
    /// Set ship movement.
    pub fn set_movement(&mut self, movement: Movement) {
        self.sol
            .internal_set_ship_movement(self.key, movement.direction.into(), movement.speed);
    }
}
