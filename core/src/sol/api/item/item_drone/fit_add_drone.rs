use crate::{
    ad::AItemId,
    def::ItemTypeId,
    misc::{ItemMutationRequest, MinionState},
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
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let item_id = self.u_data.items.alloc_id();
        let u_drone = UDrone::new(
            item_id,
            type_id,
            fit_key,
            state,
            mutation,
            UPosition::default(),
            &self.u_data.src,
        );
        let u_item = UItem::Drone(u_drone);
        let item_key = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.drones.insert(item_key);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_drone(&mut self, type_id: ItemTypeId, state: MinionState) -> DroneMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self
            .sol
            .internal_add_drone(self.key, type_id, state, None, &mut reuse_eupdates);
        DroneMut::new(self.sol, item_key)
    }
}
