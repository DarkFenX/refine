use crate::{
    def::AttrVal,
    misc::{DmgKinds, Spool},
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    ud::{UData, UFitKey},
};

impl Svc {
    pub(crate) fn get_stat_fit_dps(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        reload: bool,
        spool: Option<Spool>,
    ) -> DmgKinds<AttrVal> {
        self.vast.get_fit_data(&fit_key).get_stat_dps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            reload,
            spool,
        )
    }
    pub(crate) fn get_stat_item_dps(
        &mut self,
        u_data: &UData,
        item_key: UFitKey,
        reload: bool,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<DmgKinds<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_dps_checked(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            reload,
            spool,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_fit_volley(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        spool: Option<Spool>,
    ) -> DmgKinds<AttrVal> {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_volley(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, spool)
    }
    pub(crate) fn get_stat_item_volley(
        &mut self,
        u_data: &UData,
        item_key: UFitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<DmgKinds<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_volley_checked(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            spool,
            ignore_state,
        )
    }
}
