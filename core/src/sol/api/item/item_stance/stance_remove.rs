use crate::{
    sol::{SolarSystem, api::StanceMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_stance(
        &mut self,
        stance_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_stance(&mut self.u_data, &mut self.svc, stance_key, reuse_eupdates);
        let u_stance = self.u_data.items.get(stance_key).dc_stance().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_stance.get_fit_key());
        u_fit.stance = None;
        self.u_data.items.remove(stance_key);
    }
}

impl<'a> StanceMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_stance(self.key, &mut reuse_eupdates);
    }
}
