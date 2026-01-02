use crate::{
    ad::AItemId,
    api::{Coordinates, FighterMut, FitMut, MinionState, Movement},
    def::ItemTypeId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UFighter, UFitId, UItem, UItemId, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_fighter(
        &mut self,
        fit_key: UFitId,
        type_id: AItemId,
        state: MinionState,
        physics: UPhysics,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_fighter = UFighter::new(item_id, type_id, fit_key, state, physics, &self.u_data.src);
        let u_item = UItem::Fighter(u_fighter);
        let fighter_key = self.u_data.items.add(u_item);
        u_fit.fighters.insert(fighter_key);
        // Add fighter and autocharges to services
        SolarSystem::util_add_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_key,
            reuse_eupdates,
        );
        fighter_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fighter(
        &mut self,
        type_id: ItemTypeId,
        state: MinionState,
        coordinates: Option<Coordinates>,
        movement: Option<Movement>,
    ) -> FighterMut<'_> {
        let mut u_physics = UPhysics::default();
        if let Some(coordinates) = coordinates {
            u_physics.coordinates = coordinates.into();
        }
        if let Some(movement) = movement {
            u_physics.direction = movement.direction.into();
            u_physics.speed = movement.speed;
        }
        let mut reuse_eupdates = UEffectUpdates::new();
        let fighter_key = self
            .sol
            .internal_add_fighter(self.key, type_id, state, u_physics, &mut reuse_eupdates);
        FighterMut::new(self.sol, fighter_key)
    }
}
