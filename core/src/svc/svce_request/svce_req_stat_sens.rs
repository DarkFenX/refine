use crate::{
    def::{AttrVal, Count},
    misc::Sensor,
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    ud::{UData, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_item_locks(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<Count, StatItemCheckError> {
        Vast::get_stat_item_locks(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_lock_range(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_lock_range(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_scan_res(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_scan_res(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_sensor(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<Sensor, StatItemCheckError> {
        Vast::get_stat_item_sensor(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
