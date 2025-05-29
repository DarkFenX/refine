use crate::{
    ad,
    sol::{
        FitKey, ItemKey, ItemTypeId, SolarSystem,
        api::{BoosterMut, FitMut},
        uad::item::{UadBooster, UadItem},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_booster(&mut self, fit_key: FitKey, a_item_id: ad::AItemId) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_booster = UadBooster::new(&self.uad.src, item_id, a_item_id, fit_key, true);
        let uad_item = UadItem::Booster(uad_booster);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.boosters.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_booster(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_booster(&mut self, type_id: ItemTypeId) -> BoosterMut {
        let item_key = self.sol.internal_add_booster(self.key, type_id);
        BoosterMut::new(self.sol, item_key)
    }
}
