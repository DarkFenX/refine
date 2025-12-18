use crate::{
    ad::AItemId,
    api::{BoosterMut, FitMut},
    def::ItemTypeId,
    sol::SolarSystem,
    ud::{UBooster, UEffectUpdates, UFitKey, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_booster(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_booster = UBooster::new(item_id, type_id, fit_key, true, &self.u_data.src);
        let u_item = UItem::Booster(u_booster);
        let booster_key = self.u_data.items.add(u_item);
        u_fit.boosters.insert(booster_key);
        SolarSystem::util_add_booster(&mut self.u_data, &mut self.svc, booster_key, reuse_eupdates);
        booster_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_booster(&mut self, type_id: ItemTypeId) -> BoosterMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let booster_key = self.sol.internal_add_booster(self.key, type_id, &mut reuse_eupdates);
        BoosterMut::new(self.sol, booster_key)
    }
}
