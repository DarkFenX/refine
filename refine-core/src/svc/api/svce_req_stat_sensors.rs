use crate::{
    Count, PValue,
    stats::{StatInJam, StatSensors, StatTimeOptions, err::StatProbingSizeError},
    svc::{Svc, SvcCtx, Vast, cycle::CseqMap, err::IntStatItemError},
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_item_locks(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<Count, IntStatItemError<!>> {
        Vast::get_stat_item_locks(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_lock_range(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<PValue, IntStatItemError<!>> {
        Vast::get_stat_item_lock_range(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_scan_res(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<PValue, IntStatItemError<!>> {
        Vast::get_stat_item_scan_res(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_sensors(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<StatSensors, IntStatItemError<!>> {
        Vast::get_stat_item_sensors(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_dscan_range(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<PValue, IntStatItemError<!>> {
        Vast::get_stat_item_dscan_range(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_probing_size(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<PValue, IntStatItemError<StatProbingSizeError>> {
        Vast::get_stat_item_probing_size(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_incoming_jam(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
    ) -> Result<StatInJam, IntStatItemError<!>> {
        self.vast.get_stat_item_incoming_jam(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
        )
    }
}
