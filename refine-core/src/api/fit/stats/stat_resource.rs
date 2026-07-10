use crate::{api::FitMut, svc::vast::StatResource};

impl<'a> FitMut<'a> {
    pub fn get_stat_cpu(&mut self) -> StatResource {
        let fit = self.sol.u_data.fits.get(self.uid);
        self.sol.svc.get_stat_fit_cpu(&self.sol.u_data, self.uid, fit)
    }
    pub fn get_stat_powergrid(&mut self) -> StatResource {
        let fit = self.sol.u_data.fits.get(self.uid);
        self.sol.svc.get_stat_fit_powergrid(&self.sol.u_data, self.uid, fit)
    }
    pub fn get_stat_calibration(&mut self) -> StatResource {
        let fit = self.sol.u_data.fits.get(self.uid);
        self.sol.svc.get_stat_fit_calibration(&self.sol.u_data, self.uid, fit)
    }
    pub fn get_stat_drone_bay_volume(&mut self) -> StatResource {
        let fit = self.sol.u_data.fits.get(self.uid);
        self.sol
            .svc
            .get_stat_fit_drone_bay_volume(&self.sol.u_data, self.uid, fit)
    }
    pub fn get_stat_drone_bandwidth(&mut self) -> StatResource {
        let fit = self.sol.u_data.fits.get(self.uid);
        self.sol
            .svc
            .get_stat_fit_drone_bandwidth(&self.sol.u_data, self.uid, fit)
    }
    pub fn get_stat_fighter_bay_volume(&mut self) -> StatResource {
        let fit = self.sol.u_data.fits.get(self.uid);
        self.sol
            .svc
            .get_stat_fit_fighter_bay_volume(&self.sol.u_data, self.uid, fit)
    }
}
