use crate::{
    api::AffectionDir,
    stats::{FitStats, StatCtlAffectors, StatResource},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FitStats<'s> {
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
    pub fn get_stat_drone_bandwidth(&mut self, decreasers: StatCtlAffectors) -> StatResource {
        let fit_uid = self.uid;
        let sol = &mut *self.sol;
        let savepoint = sol.internal_tx_savepoint();
        if let Some(ship_uid) = sol.u_data.fits.get(fit_uid).ship {
            sol.internal_ctl_affectors_switch(
                ship_uid,
                sol.u_data.r_data.get_attr_consts().drone_bandwidth,
                decreasers,
                AffectionDir::Decrease,
            );
        }
        let fit = sol.u_data.fits.get(fit_uid);
        let result = sol.svc.get_stat_fit_drone_bandwidth(&sol.u_data, fit_uid, fit);
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_fighter_bay_volume(&mut self) -> StatResource {
        let fit = self.sol.u_data.fits.get(self.uid);
        self.sol
            .svc
            .get_stat_fit_fighter_bay_volume(&self.sol.u_data, self.uid, fit)
    }
}
