use crate::{
    def::AttrVal,
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_item_drone_control_range(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_drone_control_range(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_can_warp(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        Vast::get_stat_item_can_warp(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_can_jump_gate(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        self.vast
            .get_stat_item_can_jump_gate(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_can_jump_drive(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        Vast::get_stat_item_can_jump_drive(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_can_dock_station(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        self.vast
            .get_stat_item_can_dock_station(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_can_dock_citadel(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        self.vast
            .get_stat_item_can_dock_citadel(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_can_tether(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        self.vast
            .get_stat_item_can_tether(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
