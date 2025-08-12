use crate::{
    def::AttrVal,
    misc::Spool,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatRemoteRpsItemKinds, StatTank, Vast},
    },
    ud::{UData, UFitKey},
};

impl Svc {
    pub(crate) fn get_stat_fit_remote_rps(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatRemoteRpsItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        self.vast.get_stat_fit_remote_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
            spool,
        )
    }
    pub(crate) fn get_stat_fit_remote_cps(&mut self, u_data: &UData, fit_key: UFitKey) -> AttrVal {
        self.vast
            .get_stat_fit_remote_cps(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit_key)
    }
    pub(crate) fn get_stat_item_remote_rps(
        &mut self,
        u_data: &UData,
        item_key: UFitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_remote_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            spool,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_remote_cps(
        &mut self,
        u_data: &UData,
        item_key: UFitKey,
        ignore_state: bool,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_remote_cps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            ignore_state,
        )
    }
}
