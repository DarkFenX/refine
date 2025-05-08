use crate::sol::{
    FitKey, ItemKey, ItemTypeId, MinionState, SolarSystem,
    api::{FighterMut, FitMut},
    uad::item::{UadFighter, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fighter(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: MinionState,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_fighter = UadFighter::new(&self.uad.src, item_id, type_id, fit_key, state);
        let uad_item = UadItem::Fighter(uad_fighter);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.fighters.insert(item_key);
        self.internal_add_item_autocharges(item_key);
        // Add fighter and autocharges to services
        let uad_item = self.uad.items.get(item_key);
        self.svc.add_item(&self.uad, item_key, uad_item);
        for &autocharge_key in uad_item.get_fighter().unwrap().get_autocharges().values() {
            let autocharge_uad_item = self.uad.items.get(autocharge_key);
            self.svc.add_item(&self.uad, autocharge_key, autocharge_uad_item);
        }
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fighter(&mut self, type_id: ItemTypeId, state: MinionState) -> FighterMut {
        let item_key = self.sol.internal_add_fighter(self.key, type_id, state);
        FighterMut::new(self.sol, item_key)
    }
}
