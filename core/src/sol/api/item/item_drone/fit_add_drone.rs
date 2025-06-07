use crate::{
    ad,
    sol::{
        FitKey, ItemKey, ItemMutationRequest, ItemTypeId, MinionState, SolarSystem,
        api::{DroneMut, FitMut},
        uad::item::{UadDrone, UadItem},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_drone(
        &mut self,
        fit_key: FitKey,
        a_item_id: ad::AItemId,
        state: MinionState,
        mutation: Option<ItemMutationRequest>,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let uad_drone = UadDrone::new(&self.uad.src, item_id, a_item_id, fit_key, state, mutation);
        let uad_item = UadItem::Drone(uad_drone);
        let item_key = self.uad.items.add(uad_item);
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.drones.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_drone(&mut self, type_id: ItemTypeId, state: MinionState) -> DroneMut<'_> {
        let item_key = self.sol.internal_add_drone(self.key, type_id, state, None);
        DroneMut::new(self.sol, item_key)
    }
}
