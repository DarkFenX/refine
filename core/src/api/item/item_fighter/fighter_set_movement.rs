use crate::{
    api::{FighterMut, Movement},
    def::AttrVal,
    sol::SolarSystem,
    ud::UItemKey,
    util::Xyz,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fighter_movement(
        &mut self,
        fighter_key: UItemKey,
        direction: Xyz,
        speed: AttrVal,
    ) {
        let u_fighter = self.u_data.items.get_mut(fighter_key).dc_fighter_mut().unwrap();
        let fighter_u_physics = u_fighter.get_physics_mut();
        if fighter_u_physics.direction == direction && fighter_u_physics.speed == speed {
            return;
        }
        fighter_u_physics.direction = direction;
        fighter_u_physics.speed = speed;
        SolarSystem::util_update_fighter_physics(&mut self.u_data, &self.rev_projs, &mut self.svc, fighter_key);
    }
}

impl<'a> FighterMut<'a> {
    /// Set fighter movement.
    pub fn set_movement(&mut self, movement: Movement) {
        self.sol
            .internal_set_fighter_movement(self.key, movement.direction.into(), movement.speed);
    }
}
