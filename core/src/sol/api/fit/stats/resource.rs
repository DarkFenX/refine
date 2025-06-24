use crate::sol::{api::FitMut, svc::vast::StatRes};

impl<'a> FitMut<'a> {
    pub fn get_stat_cpu(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol.svc.get_stat_fit_cpu(&self.sol.uad, self.key, fit)
    }
    pub fn get_stat_powergrid(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol.svc.get_stat_fit_powergrid(&self.sol.uad, self.key, fit)
    }
    pub fn get_stat_calibration(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol.svc.get_stat_fit_calibration(&self.sol.uad, self.key, fit)
    }
    pub fn get_stat_drone_bay_volume(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol.svc.get_stat_fit_drone_bay_volume(&self.sol.uad, self.key, fit)
    }
    pub fn get_stat_drone_bandwidth(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol.svc.get_stat_fit_drone_bandwidth(&self.sol.uad, self.key, fit)
    }
    pub fn get_stat_fighter_bay_volume(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .get_stat_fit_fighter_bay_volume(&self.sol.uad, self.key, fit)
    }
}
