use crate::{
    def::FitKey,
    svc::{Svc, SvcCtx, vast::StatRes},
    uad::{Uad, UadFit},
};

impl Svc {
    pub(crate) fn get_stat_fit_cpu(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_cpu(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_powergrid(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_powergrid(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_calibration(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_calibration(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_drone_bay_volume(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_drone_bay_volume(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_drone_bandwidth(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_drone_bandwidth(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_fighter_bay_volume(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast.get_fit_data(&fit_key).get_stat_fighter_bay_volume(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
}
