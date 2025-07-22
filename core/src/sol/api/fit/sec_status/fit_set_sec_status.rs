use crate::{
    misc::FitSecStatus,
    sol::{SolarSystem, api::FitMut},
    uad::UadFitKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_sec_status(&mut self, fit_key: UadFitKey, sec_status: FitSecStatus) {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let old_sec_status = uad_fit.sec_status;
        if old_sec_status == sec_status {
            return;
        }
        uad_fit.sec_status = sec_status;
        if let Some(ship_key) = uad_fit.ship {
            self.svc.notify_ship_sec_status_changed(&self.uad, ship_key);
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn set_sec_status(&mut self, sec_status: FitSecStatus) {
        self.sol.internal_set_fit_sec_status(self.key, sec_status);
    }
}
