use crate::{api::ShipMut, def::AttrVal, misc::Movement, sol::SolarSystem, ud::UItemKey, util::Xyz};

impl SolarSystem {
    pub(in crate::api) fn internal_set_ship_movement(&mut self, ship_key: UItemKey, direction: Xyz, speed: AttrVal) {
        let u_ship = self.u_data.items.get_mut(ship_key).dc_ship_mut().unwrap();
        let ship_u_physics = u_ship.get_physics_mut();
        if ship_u_physics.direction == direction && ship_u_physics.speed == speed {
            return;
        }
        ship_u_physics.direction = direction;
        ship_u_physics.speed = speed;
        SolarSystem::util_update_ship_physics(&mut self.u_data, &self.rev_projs, &mut self.svc, ship_key);
    }
}

impl<'a> ShipMut<'a> {
    /// Set ship movement.
    pub fn set_movement(&mut self, movement: Movement) {
        self.sol
            .internal_set_ship_movement(self.key, movement.direction.into(), movement.speed);
    }
}
