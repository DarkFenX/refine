use crate::{
    api::{FitMut, RmMode},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fit(&mut self, fit_uid: UFitId, reuse_eupdates: &mut UEffectUpdates) {
        let u_fit = self.u_data.fits.get(fit_uid);
        for item_uid in u_fit.all_direct_items().into_iter() {
            self.internal_remove_item(item_uid, RmMode::Free, reuse_eupdates)
                .unwrap();
        }
        self.svc.notify_fit_removed(fit_uid);
        let u_fit = self.u_data.fits.remove(fit_uid);
        if let Some(fleet_uid) = u_fit.fleet {
            let u_fleet = self.u_data.fleets.get_mut(fleet_uid);
            u_fleet.remove_fit(&fit_uid);
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_fit(self.uid, &mut reuse_eupdates);
    }
}
