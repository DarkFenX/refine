use itertools::Itertools;

use crate::{api::FleetMut, sol::SolarSystem, ud::UFleetId};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fleet(&mut self, fleet_uid: UFleetId) {
        let u_fleet = self.u_data.fleets.get(fleet_uid);
        let fit_uids = u_fleet.iter_fits().collect_vec();
        for fit_uid in fit_uids {
            self.svc.notify_fit_removed_from_fleet(&self.u_data, u_fleet, fit_uid);
            let u_fit = self.u_data.fits.get_mut(fit_uid);
            u_fit.fleet = None;
        }
        self.u_data.fleets.remove(fleet_uid);
    }
}

impl<'a> FleetMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_fleet(self.uid);
    }
}
