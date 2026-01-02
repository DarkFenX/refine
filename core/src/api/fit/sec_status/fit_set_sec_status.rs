use crate::{api::FitMut, misc::FitSecStatus, sol::SolarSystem, ud::UFitId};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fit_sec_status(&mut self, fit_key: UFitId, sec_status: FitSecStatus) {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let old_sec_status = u_fit.sec_status;
        if old_sec_status == sec_status {
            return;
        }
        u_fit.sec_status = sec_status;
        if let Some(ship_key) = u_fit.ship {
            self.svc.notify_ship_sec_status_changed(&self.u_data, ship_key);
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn set_sec_status(&mut self, sec_status: FitSecStatus) {
        self.sol.internal_set_fit_sec_status(self.key, sec_status);
    }
}
