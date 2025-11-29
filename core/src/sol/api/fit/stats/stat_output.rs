use crate::{
    def::{AttrVal, ItemId},
    misc::Spool,
    sol::api::{FitMut, FitStatAppliedError},
    svc::vast::{
        StatDmg, StatDmgApplied, StatDmgItemKinds, StatMining, StatMiningItemKinds, StatNeutItemKinds,
        StatOutRepItemKinds, StatTank,
    },
};

impl<'a> FitMut<'a> {
    pub fn get_stat_dps(&mut self, item_kinds: StatDmgItemKinds, reload: bool, spool: Option<Spool>) -> StatDmg {
        self.sol
            .svc
            .get_stat_fit_dps_raw(&self.sol.u_data, self.key, item_kinds, reload, spool)
    }
    pub fn get_stat_dps_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FitStatAppliedError> {
        let projectee_key = self.get_stat_applied_projectee_key(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_dps_applied(&self.sol.u_data, self.key, item_kinds, reload, spool, projectee_key))
    }
    pub fn get_stat_volley(&mut self, item_kinds: StatDmgItemKinds, spool: Option<Spool>) -> StatDmg {
        self.sol
            .svc
            .get_stat_fit_volley_raw(&self.sol.u_data, self.key, item_kinds, spool)
    }
    pub fn get_stat_volley_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FitStatAppliedError> {
        let projectee_key = self.get_stat_applied_projectee_key(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_volley_applied(&self.sol.u_data, self.key, item_kinds, spool, projectee_key))
    }
    pub fn get_stat_mps(&mut self, item_kinds: StatMiningItemKinds, reload: bool) -> StatMining {
        self.sol
            .svc
            .get_stat_fit_mps(&self.sol.u_data, self.key, item_kinds, reload)
    }
    pub fn get_stat_outgoing_rps(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        self.sol
            .svc
            .get_stat_fit_outgoing_rps(&self.sol.u_data, self.key, item_kinds, spool)
    }
    pub fn get_stat_outgoing_cps(&mut self) -> AttrVal {
        self.sol.svc.get_stat_fit_outgoing_cps(&self.sol.u_data, self.key)
    }
    pub fn get_stat_outgoing_nps(&mut self, item_kinds: StatNeutItemKinds) -> AttrVal {
        self.sol
            .svc
            .get_stat_fit_outgoing_nps(&self.sol.u_data, self.key, item_kinds, None)
    }
    pub fn get_stat_outgoing_nps_applied(
        &mut self,
        item_kinds: StatNeutItemKinds,
        projectee_item_id: &ItemId,
    ) -> Result<AttrVal, FitStatAppliedError> {
        let projectee_key = self.get_stat_applied_projectee_key(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_outgoing_nps(&self.sol.u_data, self.key, item_kinds, Some(projectee_key)))
    }
}
