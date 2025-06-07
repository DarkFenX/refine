use crate::sol::{api::FitMut, svc::vast::StatRes};

impl<'a> FitMut<'a> {
    pub fn get_stat_cpu(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_cpu(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_powergrid(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_powergrid(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_calibration(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_calibration(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_drone_bay_volume(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_drone_bay_volume(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_drone_bandwidth(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_drone_bandwidth(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_fighter_bay_volume(&mut self) -> StatRes {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol.svc.vast.get_fit_data(&self.key).get_stat_fighter_bay_volume(
            &self.sol.uad,
            &mut self.sol.svc.calc,
            fit,
        )
    }
}
