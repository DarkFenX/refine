use crate::{
    num::PValue,
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_item_speed(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        Vast::get_stat_item_speed(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_agility(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        Vast::get_stat_item_agility(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_align_time(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        Vast::get_stat_item_align_time(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_sig_radius(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        Vast::get_stat_item_sig_radius(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_mass(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        Vast::get_stat_item_mass(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_warp_speed(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        Vast::get_stat_item_warp_speed(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_max_warp_range(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        Vast::get_stat_item_max_warp_range(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
}
