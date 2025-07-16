use crate::{
    def::{AttrVal, FitKey},
    misc::Spool,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatTank, Vast},
    },
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_fit_remote_rps(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_remote_rps(SvcCtx::new(uad, &self.eprojs), &mut self.calc, spool)
    }
    pub(crate) fn get_stat_fit_remote_cps(&mut self, uad: &Uad, fit_key: FitKey) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_remote_cps(SvcCtx::new(uad, &self.eprojs), &mut self.calc)
    }
    pub(crate) fn get_stat_item_remote_rps(
        &mut self,
        uad: &Uad,
        item_key: FitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_remote_rps_checked(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            item_key,
            spool,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_remote_cps(
        &mut self,
        uad: &Uad,
        item_key: FitKey,
        ignore_state: bool,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_remote_cps_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key, ignore_state)
    }
}
