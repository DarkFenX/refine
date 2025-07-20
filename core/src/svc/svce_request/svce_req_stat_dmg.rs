use crate::{
    def::{AttrVal, FitKey},
    misc::{DmgKinds, Spool},
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_fit_dps(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        reload: bool,
        spool: Option<Spool>,
    ) -> DmgKinds<AttrVal> {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_dps(SvcCtx::new(uad, &self.eprojs), &mut self.calc, reload, spool)
    }
    pub(crate) fn get_stat_item_dps(
        &mut self,
        uad: &Uad,
        item_key: FitKey,
        reload: bool,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<DmgKinds<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_dps_checked(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            item_key,
            reload,
            spool,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_fit_volley(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        spool: Option<Spool>,
    ) -> DmgKinds<AttrVal> {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_volley(SvcCtx::new(uad, &self.eprojs), &mut self.calc, spool)
    }
    pub(crate) fn get_stat_item_volley(
        &mut self,
        uad: &Uad,
        item_key: FitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<DmgKinds<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_volley_checked(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            item_key,
            spool,
            ignore_state,
        )
    }
}
