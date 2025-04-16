use crate::sol::{
    FitKey, ItemKey, ItemTypeId, SolarSystem,
    api::{DroneMut, FitMut},
    uad::item::{ItemAddMutation, MinionState, UadDrone, UadItem},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_drone(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: MinionState,
        mutation: Option<ItemAddMutation>,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let uad_drone = UadDrone::new(&self.uad.src, item_id, type_id, fit_key, state, mutation);
        let uad_item = UadItem::Drone(uad_drone);
        let item_key = self.uad.items.add(uad_item);
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.drones.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_drone(
        &'a mut self,
        type_id: ItemTypeId,
        state: MinionState,
        mutation: Option<ItemAddMutation>,
    ) -> DroneMut<'a> {
        let item_key = self.sol.internal_add_drone(self.key, type_id, state, mutation);
        DroneMut::new(self.sol, item_key)
    }
}
