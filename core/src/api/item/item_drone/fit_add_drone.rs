use crate::{
    ad::AItemId,
    api::{Coordinates, DroneMut, FitMut, MinionState, Movement},
    def::ItemTypeId,
    misc::NpcProp,
    sol::SolarSystem,
    ud::{UDrone, UEffectUpdates, UFitKey, UItem, UItemKey, UItemMutationRequest, UNpcProp, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_drone(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        state: MinionState,
        mutation: Option<UItemMutationRequest>,
        physics: UPhysics,
        prop_mode: UNpcProp,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let item_id = self.u_data.items.alloc_id();
        let u_drone = UDrone::new(
            item_id,
            type_id,
            fit_key,
            state,
            mutation,
            physics,
            prop_mode,
            &self.u_data.src,
        );
        let u_item = UItem::Drone(u_drone);
        let drone_key = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.drones.insert(drone_key);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        drone_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_drone(
        &mut self,
        type_id: ItemTypeId,
        state: MinionState,
        coordinates: Option<Coordinates>,
        movement: Option<Movement>,
        prop_mode: NpcProp,
    ) -> DroneMut<'_> {
        let mut u_physics = UPhysics::default();
        if let Some(coordinates) = coordinates {
            u_physics.coordinates = coordinates.into();
        }
        if let Some(movement) = movement {
            u_physics.direction = movement.direction.into();
            u_physics.speed = movement.speed;
        }
        let mut reuse_eupdates = UEffectUpdates::new();
        let drone_key = self.sol.internal_add_drone(
            self.key,
            type_id,
            state,
            None,
            u_physics,
            prop_mode.into(),
            &mut reuse_eupdates,
        );
        DroneMut::new(self.sol, drone_key)
    }
}
