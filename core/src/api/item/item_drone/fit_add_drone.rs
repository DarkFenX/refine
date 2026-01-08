use crate::{
    ad::AItemId,
    api::{Coordinates, DroneMut, FitMut, ItemTypeId, MinionState, Movement},
    sol::SolarSystem,
    ud::{UDrone, UEffectUpdates, UFitId, UItem, UItemId, UItemMutationRequest, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_drone(
        &mut self,
        fit_uid: UFitId,
        type_id: AItemId,
        state: MinionState,
        mutation: Option<UItemMutationRequest>,
        physics: UPhysics,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_drone = UDrone::new(item_id, type_id, fit_uid, state, mutation, physics, &self.u_data.src);
        let u_item = UItem::Drone(u_drone);
        let drone_uid = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.drones.insert(drone_uid);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, reuse_eupdates);
        drone_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn add_drone(
        &mut self,
        type_id: ItemTypeId,
        state: MinionState,
        coordinates: Option<Coordinates>,
        movement: Option<Movement>,
    ) -> DroneMut<'_> {
        let mut u_physics = UPhysics::default();
        if let Some(coordinates) = coordinates {
            u_physics.coordinates = coordinates.into_xyz();
        }
        if let Some(movement) = movement {
            u_physics.direction = movement.direction.into_xyz();
            u_physics.speed = movement.speed;
        }
        let mut reuse_eupdates = UEffectUpdates::new();
        let drone_uid = self.sol.internal_add_drone(
            self.uid,
            type_id.into_aid(),
            state,
            None,
            u_physics,
            &mut reuse_eupdates,
        );
        DroneMut::new(self.sol, drone_uid)
    }
}
