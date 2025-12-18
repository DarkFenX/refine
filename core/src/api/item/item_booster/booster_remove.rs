use crate::{
    api::BoosterMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_booster(
        &mut self,
        booster_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_booster(&mut self.u_data, &mut self.svc, booster_key, reuse_eupdates);
        let u_booster = self.u_data.items.get(booster_key).dc_booster().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_booster.get_fit_key());
        u_fit.boosters.remove(&booster_key);
        self.u_data.items.remove(booster_key);
    }
}

impl<'a> BoosterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_booster(self.key, &mut reuse_eupdates);
    }
}
