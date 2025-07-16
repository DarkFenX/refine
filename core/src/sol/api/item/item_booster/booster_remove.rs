use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::BoosterMut},
    uad::UadEffectUpdates,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_booster(
        &mut self,
        item_key: ItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        let uad_booster = uad_item.get_booster().unwrap();
        SolarSystem::util_remove_booster(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_fit = self.uad.fits.get_mut(uad_booster.get_fit_key());
        uad_fit.boosters.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> BoosterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_booster(self.key, &mut reuse_eupdates);
    }
}
