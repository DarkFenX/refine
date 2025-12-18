use crate::{
    api::FitMut,
    misc::RmMode,
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fit(&mut self, fit_key: UFitKey, reuse_eupdates: &mut UEffectUpdates) {
        let u_fit = self.u_data.fits.get(fit_key);
        for item_key in u_fit.all_direct_items().into_iter() {
            self.internal_remove_item(item_key, RmMode::Free, reuse_eupdates)
                .unwrap();
        }
        self.svc.notify_fit_removed(fit_key);
        let u_fit = self.u_data.fits.remove(fit_key);
        if let Some(fleet_key) = u_fit.fleet {
            let u_fleet = self.u_data.fleets.get_mut(fleet_key);
            u_fleet.remove_fit(&fit_key);
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_fit(self.key, &mut reuse_eupdates);
    }
}
