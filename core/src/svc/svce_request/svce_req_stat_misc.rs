use crate::{
    def::AttrVal,
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    ud::{UData, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_item_drone_control_range(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_drone_control_range(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_can_warp(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        Vast::get_stat_item_can_warp(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_can_dock(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        self.vast
            .get_stat_item_can_dock(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
