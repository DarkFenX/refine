use crate::{Fit, stats::StatResource};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn get_stat_cpu(&mut self) -> StatResource {
        let fit = self.core.u_data.fits.get(self.uid);
        self.core.svc.get_stat_fit_cpu(&self.core.u_data, self.uid, fit)
    }
    pub fn get_stat_powergrid(&mut self) -> StatResource {
        let fit = self.core.u_data.fits.get(self.uid);
        self.core.svc.get_stat_fit_powergrid(&self.core.u_data, self.uid, fit)
    }
    pub fn get_stat_calibration(&mut self) -> StatResource {
        let fit = self.core.u_data.fits.get(self.uid);
        self.core.svc.get_stat_fit_calibration(&self.core.u_data, self.uid, fit)
    }
    pub fn get_stat_drone_bay_volume(&mut self) -> StatResource {
        let fit = self.core.u_data.fits.get(self.uid);
        self.core
            .svc
            .get_stat_fit_drone_bay_volume(&self.core.u_data, self.uid, fit)
    }
    pub fn get_stat_drone_bandwidth(&mut self) -> StatResource {
        let fit = self.core.u_data.fits.get(self.uid);
        self.core
            .svc
            .get_stat_fit_drone_bandwidth(&self.core.u_data, self.uid, fit)
    }
    pub fn get_stat_fighter_bay_volume(&mut self) -> StatResource {
        let fit = self.core.u_data.fits.get(self.uid);
        self.core
            .svc
            .get_stat_fit_fighter_bay_volume(&self.core.u_data, self.uid, fit)
    }
}
