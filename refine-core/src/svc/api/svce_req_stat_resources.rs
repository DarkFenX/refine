use crate::{
    Value,
    stats::{StatFighterBay, StatResource},
    svc::{Svc, SvcCtx, err::IntStatItemError, vast::Vast},
    ud::{UData, UFit, UFitId, UItemId},
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
    pub(crate) fn get_stat_item_cpu_use(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<Value, IntStatItemError<!>> {
        Vast::get_stat_item_cpu_use(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_powergrid_use(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<Value, IntStatItemError<!>> {
        Vast::get_stat_item_powergrid_use(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_calibration_use(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<Value, IntStatItemError<!>> {
        Vast::get_stat_item_calibration_use(&SvcCtx::new(u_data, &self.eff_projs), item_uid)
    }
}
