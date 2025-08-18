use crate::{
    ad::AItemId,
    def::ItemTypeId,
    misc::{Coordinates, ItemMutationRequest, MinionState},
    sol::{
        SolarSystem,
        api::{DroneMut, FitMut},
    },
    ud::{UDrone, UEffectUpdates, UFitKey, UItem, UItemKey, UPosition},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_drone(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        state: MinionState,
        mutation: Option<ItemMutationRequest>,
        position: UPosition,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let item_id = self.u_data.items.alloc_id();
        let u_drone = UDrone::new(item_id, type_id, fit_key, state, mutation, position, &self.u_data.src);
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
    ) -> DroneMut<'_> {
        let mut u_position = UPosition::default();
        if let Some(coordinates) = coordinates {
            u_position.coordinates = coordinates.into();
        }
        let mut reuse_eupdates = UEffectUpdates::new();
        let drone_key = self
            .sol
            .internal_add_drone(self.key, type_id, state, None, u_position, &mut reuse_eupdates);
        DroneMut::new(self.sol, drone_key)
    }
}
