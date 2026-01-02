use crate::{
    def::{AttrVal, Count},
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatJamApplied, StatSensors, Vast},
    },
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_item_locks(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<Count, StatItemCheckError> {
        Vast::get_stat_item_locks(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_lock_range(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_lock_range(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_scan_res(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_scan_res(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_sensors(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<StatSensors, StatItemCheckError> {
        Vast::get_stat_item_sensors(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_dscan_range(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_dscan_range(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_probing_size(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_probing_size(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_incoming_jam(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<StatJamApplied, StatItemCheckError> {
        self.vast
            .get_stat_item_incoming_jam(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
