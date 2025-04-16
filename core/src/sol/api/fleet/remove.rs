use itertools::Itertools;

use crate::sol::{FleetKey, SolarSystem, api::FleetMut};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_fleet(&mut self, fleet_key: FleetKey) {
        let uad_fleet = self.uad.fleets.get(fleet_key);
        let fit_keys = uad_fleet.iter_fits().copied().collect_vec();
        for fit_key in fit_keys {
            self.svc.remove_fit_from_fleet(&self.uad, uad_fleet, &fit_key);
            let uad_fit = self.uad.fits.get_mut(fit_key);
            uad_fit.fleet = None;
        }
        self.uad.fleets.remove(fleet_key);
    }
}

impl<'a> FleetMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_fleet(self.key);
    }
}
