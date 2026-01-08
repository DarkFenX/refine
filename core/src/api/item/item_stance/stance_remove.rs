use crate::{
    api::StanceMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_stance(&mut self, stance_uid: UItemId, reuse_eupdates: &mut UEffectUpdates) {
        SolarSystem::util_remove_stance(&mut self.u_data, &mut self.svc, stance_uid, reuse_eupdates);
        let u_stance = self.u_data.items.get(stance_uid).dc_stance().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_stance.get_fit_uid());
        u_fit.stance = None;
        self.u_data.items.remove(stance_uid);
    }
}

impl<'a> StanceMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_stance(self.uid, &mut reuse_eupdates);
    }
}
