use crate::{
    api::{Coordinates, FighterMut},
    misc::Xyz,
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fighter_coordinates(&mut self, fighter_uid: UItemId, coordinates: Xyz) {
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        if u_fighter.get_physics().coordinates == coordinates {
            return;
        }
        u_fighter.get_physics_mut().coordinates = coordinates;
        SolarSystem::util_update_fighter_physics(&mut self.u_data, &self.rev_projs, &mut self.svc, fighter_uid);
    }
}

impl<'a> FighterMut<'a> {
    /// Set fighter position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.sol
            .internal_set_fighter_coordinates(self.uid, coordinates.into_xyz())
    }
}
