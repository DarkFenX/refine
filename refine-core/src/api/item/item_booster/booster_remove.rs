use crate::{
    api::BoosterMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_booster(
        &mut self,
        booster_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_booster(&mut self.u_data, &mut self.svc, booster_uid, reuse_eupdates);
        let u_booster = self.u_data.items.get(booster_uid).dc_booster().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_booster.get_fit_uid());
        u_fit.boosters.remove(&booster_uid);
        self.u_data.items.remove(booster_uid);
    }
}

impl<'a> BoosterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_booster(self.uid, &mut reuse_eupdates);
    }
}
