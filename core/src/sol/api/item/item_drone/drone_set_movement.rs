use crate::{
    def::AttrVal,
    misc::Movement,
    sol::{SolarSystem, api::DroneMut},
    ud::{UDirection, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_movement(
        &mut self,
        drone_key: UItemKey,
        u_direction: UDirection,
        speed: AttrVal,
    ) {
        let u_drone = self.u_data.items.get_mut(drone_key).get_drone_mut().unwrap();
        let drone_u_position = u_drone.get_position_mut();
        if drone_u_position.direction == u_direction && drone_u_position.speed == speed {
            return;
        }
        drone_u_position.direction = u_direction;
        drone_u_position.speed = speed;
        SolarSystem::util_update_drone_position(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_key);
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone movement.
    pub fn set_movement(&mut self, movement: Movement) {
        self.sol
            .internal_set_drone_movement(self.key, movement.direction.into(), movement.speed);
    }
}
