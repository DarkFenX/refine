use crate::{
    def::AttrVal,
    misc::Movement,
    sol::{SolarSystem, api::FighterMut},
    ud::{UDirection, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_movement(
        &mut self,
        fighter_key: UItemKey,
        u_direction: UDirection,
        speed: AttrVal,
    ) {
        let u_fighter = self.u_data.items.get_mut(fighter_key).get_fighter_mut().unwrap();
        let fighter_u_position = u_fighter.get_position_mut();
        if fighter_u_position.direction == u_direction && fighter_u_position.speed == speed {
            return;
        }
        fighter_u_position.direction = u_direction;
        fighter_u_position.speed = speed;
        SolarSystem::util_update_fighter_position(&mut self.u_data, &self.rev_projs, &mut self.svc, fighter_key);
    }
}

impl<'a> FighterMut<'a> {
    /// Set fighter movement.
    pub fn set_movement(&mut self, movement: Movement) {
        self.sol
            .internal_set_fighter_movement(self.key, movement.direction.into(), movement.speed);
    }
}
