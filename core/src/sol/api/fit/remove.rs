use crate::sol::{FitKey, RmMode, SolarSystem, api::FitMut};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_fit(&mut self, fit_key: FitKey) {
        let uad_fit = self.uad.fits.get(fit_key);
        for item_key in uad_fit.all_direct_items().into_iter() {
            self.internal_remove_item(item_key, RmMode::Free).unwrap();
        }
        self.svc.remove_fit(fit_key);
        let uad_fit = self.uad.fits.remove(fit_key);
        if let Some(fleet_key) = uad_fit.fleet {
            let uad_fleet = self.uad.fleets.get_mut(fleet_key);
            uad_fleet.remove_fit(&fit_key);
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_fit(self.key);
    }
}
