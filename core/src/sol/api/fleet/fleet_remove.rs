use itertools::Itertools;

use crate::{
    sol::{SolarSystem, api::FleetMut},
    ud::UFleetKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fleet(&mut self, fleet_key: UFleetKey) {
        let u_fleet = self.u_data.fleets.get(fleet_key);
        let fit_keys = u_fleet.iter_fits().collect_vec();
        for fit_key in fit_keys {
            self.svc.notify_fit_removed_from_fleet(&self.u_data, u_fleet, fit_key);
            let u_fit = self.u_data.fits.get_mut(fit_key);
            u_fit.fleet = None;
        }
        self.u_data.fleets.remove(fleet_key);
    }
}

impl<'a> FleetMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_fleet(self.key);
    }
}
