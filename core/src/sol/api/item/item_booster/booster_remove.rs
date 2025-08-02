use crate::{
    sol::{SolarSystem, api::BoosterMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_booster(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_booster(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        let u_booster = self.u_data.items.get(item_key).get_booster().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_booster.get_fit_key());
        u_fit.boosters.remove(&item_key);
        self.u_data.items.remove(item_key);
    }
}

impl<'a> BoosterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_booster(self.key, &mut reuse_eupdates);
    }
}
