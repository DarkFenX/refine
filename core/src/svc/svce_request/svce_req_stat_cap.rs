use crate::{
    def::AttrVal,
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    ud::{UData, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_item_cap(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_cap(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_neut_resist(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_neut_resist(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
