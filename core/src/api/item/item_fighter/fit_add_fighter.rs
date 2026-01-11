use crate::{
    ad::AItemId,
    api::{Coordinates, FighterMut, FitMut, ItemTypeId, MinionState, Movement},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFighter, UFitId, UItem, UItemId, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_fighter(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        state: MinionState,
        physics: UPhysics,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_fighter = UFighter::new(item_id, type_aid, fit_uid, state, physics, &self.u_data.src);
        let u_item = UItem::Fighter(u_fighter);
        let fighter_uid = self.u_data.items.add(u_item);
        u_fit.fighters.insert(fighter_uid);
        // Add fighter and autocharges to services
        SolarSystem::util_add_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            reuse_eupdates,
        );
        fighter_uid
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
            u_physics.coordinates = coordinates.into_xyz();
        }
        if let Some(movement) = movement {
            u_physics.direction = movement.direction.into_xyz();
            u_physics.speed = movement.speed;
        }
        let mut reuse_eupdates = UEffectUpdates::new();
        let fighter_uid =
            self.sol
                .internal_add_fighter(self.uid, type_id.into_aid(), state, u_physics, &mut reuse_eupdates);
        FighterMut::new(self.sol, fighter_uid)
    }
}
