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
    pub(crate) fn get_stat_item_remote_nps(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        ignore_state: bool,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_remote_nps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            ignore_state,
        )
    }
}
