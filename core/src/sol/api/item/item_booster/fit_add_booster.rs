use crate::{
    ad,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{BoosterMut, FitMut},
    },
    ud::{UBooster, UEffectUpdates, UFitKey, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_booster(
        &mut self,
        fit_key: UFitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_booster = UBooster::new(item_id, a_item_id, fit_key, true, &self.u_data.src, reuse_eupdates);
        let u_item = UItem::Booster(u_booster);
        let item_key = self.u_data.items.add(u_item);
        u_fit.boosters.insert(item_key);
        SolarSystem::util_add_booster(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_booster(&mut self, type_id: ItemTypeId) -> BoosterMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self.sol.internal_add_booster(self.key, type_id, &mut reuse_eupdates);
        BoosterMut::new(self.sol, item_key)
    }
}
