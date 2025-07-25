use crate::{
    def::AttrVal,
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    ud::{UData, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_item_speed(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_speed_checked(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_agility(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_agility_checked(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_align_time(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_align_time_checked(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
