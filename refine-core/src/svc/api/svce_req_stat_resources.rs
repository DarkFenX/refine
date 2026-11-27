use crate::{
    stats::{StatFighterBay, StatResource},
    svc::{Svc, SvcCtx},
    ud::{UData, UFit, UFitId},
};

impl Svc {
    pub(crate) fn get_stat_fit_cpu(&mut self, u_data: &UData, fit_uid: UFitId, fit: &UFit) -> StatResource {
        self.vast
            .get_fit_data(fit_uid)
            .get_stat_cpu(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_powergrid(&mut self, u_data: &UData, fit_uid: UFitId, fit: &UFit) -> StatResource {
        self.vast
            .get_fit_data(fit_uid)
            .get_stat_powergrid(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_calibration(&mut self, u_data: &UData, fit_uid: UFitId, fit: &UFit) -> StatResource {
        self.vast
            .get_fit_data(fit_uid)
            .get_stat_calibration(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_drone_bay(&mut self, u_data: &UData, fit_uid: UFitId, fit: &UFit) -> StatResource {
        self.vast
            .get_fit_data(fit_uid)
            .get_stat_drone_bay(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_drone_bandwidth(&mut self, u_data: &UData, fit_uid: UFitId, fit: &UFit) -> StatResource {
        self.vast.get_fit_data(fit_uid).get_stat_drone_bandwidth(
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit,
        )
    }
    pub(crate) fn get_stat_fit_fighter_bay(&mut self, u_data: &UData, fit_uid: UFitId, fit: &UFit) -> StatFighterBay {
        self.vast
            .get_fit_data(fit_uid)
            .get_stat_fighter_bay(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit)
    }
}
