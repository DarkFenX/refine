use crate::{
    ad,
    def::ItemTypeId,
    misc::{ItemMutationRequest, MinionState},
    sol::{
        SolarSystem,
        api::{DroneMut, FitMut},
    },
    uad::{UadDrone, UadEffectUpdates, UadFitKey, UadItem, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_drone(
        &mut self,
        fit_key: UadFitKey,
        a_item_id: ad::AItemId,
        state: MinionState,
        mutation: Option<ItemMutationRequest>,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> UadItemKey {
        let item_id = self.uad.items.alloc_id();
        let uad_drone = UadDrone::new(
            item_id,
            a_item_id,
            fit_key,
            state,
            mutation,
            &self.uad.src,
            reuse_eupdates,
        );
        let uad_item = UadItem::Drone(uad_drone);
        let item_key = self.uad.items.add(uad_item);
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.drones.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_drone(&mut self, type_id: ItemTypeId, state: MinionState) -> DroneMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self
            .sol
            .internal_add_drone(self.key, type_id, state, None, &mut reuse_eupdates);
        DroneMut::new(self.sol, item_key)
    }
}
