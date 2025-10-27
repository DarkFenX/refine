use crate::{
    def::AttrVal,
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    ud::{UData, UFitKey, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_fits_remote_nps(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
    ) -> AttrVal {
        self.vast
            .get_stat_fits_remote_nps(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit_keys)
    }
    pub(crate) fn get_stat_fit_remote_nps(&mut self, u_data: &UData, fit_key: UFitKey) -> AttrVal {
        self.vast
            .get_stat_fit_remote_nps(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit_key)
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
