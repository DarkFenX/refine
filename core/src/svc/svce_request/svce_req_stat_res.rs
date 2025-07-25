use crate::{
    svc::{Svc, SvcCtx, vast::StatRes},
    ud::{UData, UFit, UFitKey},
};

impl Svc {
    pub(crate) fn get_stat_fit_cpu(&mut self, u_data: &UData, fit_key: UFitKey, fit: &UFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_cpu(SvcCtx::new(u_data, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_powergrid(&mut self, u_data: &UData, fit_key: UFitKey, fit: &UFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_powergrid(SvcCtx::new(u_data, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_calibration(&mut self, u_data: &UData, fit_key: UFitKey, fit: &UFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_calibration(SvcCtx::new(u_data, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_drone_bay_volume(&mut self, u_data: &UData, fit_key: UFitKey, fit: &UFit) -> StatRes {
        self.vast.get_fit_data(&fit_key).get_stat_drone_bay_volume(
            SvcCtx::new(u_data, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
    pub(crate) fn get_stat_fit_drone_bandwidth(&mut self, u_data: &UData, fit_key: UFitKey, fit: &UFit) -> StatRes {
        self.vast.get_fit_data(&fit_key).get_stat_drone_bandwidth(
            SvcCtx::new(u_data, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
    pub(crate) fn get_stat_fit_fighter_bay_volume(&mut self, u_data: &UData, fit_key: UFitKey, fit: &UFit) -> StatRes {
        self.vast.get_fit_data(&fit_key).get_stat_fighter_bay_volume(
            SvcCtx::new(u_data, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
}
