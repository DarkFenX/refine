use crate::{
    api::{FitMut, FitStatAppliedError},
    misc::Spool,
    num::PValue,
    svc::vast::{
        StatDmg, StatDmgApplied, StatDmgItemKinds, StatMining, StatMiningItemKinds, StatNeutItemKinds,
        StatOutRepItemKinds, StatOutReps, StatTimeOptions,
    },
    ud::ItemId,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_dmg(&mut self, item_kinds: StatDmgItemKinds, time_options: StatTimeOptions) -> StatDmg {
        self.sol
            .svc
            .get_stat_fit_dmg_raw(&self.sol.u_data, self.uid, item_kinds, time_options)
    }
    pub fn get_stat_dmg_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FitStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_dmg_applied(&self.sol.u_data, self.uid, item_kinds, time_options, projectee_uid))
    }
    pub fn get_stat_mps(&mut self, item_kinds: StatMiningItemKinds, time_options: StatTimeOptions) -> StatMining {
        self.sol
            .svc
            .get_stat_fit_mps(&self.sol.u_data, self.uid, item_kinds, time_options)
    }
    pub fn get_stat_outgoing_rps(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
    ) -> StatOutReps {
        self.sol
            .svc
            .get_stat_fit_outgoing_rps(&self.sol.u_data, self.uid, item_kinds, time_options, None)
    }
    pub fn get_stat_outgoing_rps_applied(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatOutReps, FitStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        Ok(self.sol.svc.get_stat_fit_outgoing_rps(
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            Some(projectee_uid),
        ))
    }
    pub fn get_stat_outgoing_cps(&mut self, time_options: StatTimeOptions) -> PValue {
        self.sol
            .svc
            .get_stat_fit_outgoing_cps(&self.sol.u_data, self.uid, time_options, None)
    }
    pub fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, FitStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_outgoing_cps(&self.sol.u_data, self.uid, time_options, Some(projectee_uid)))
    }
    pub fn get_stat_outgoing_nps(&mut self, item_kinds: StatNeutItemKinds, time_options: StatTimeOptions) -> PValue {
        self.sol
            .svc
            .get_stat_fit_outgoing_nps(&self.sol.u_data, self.uid, item_kinds, time_options, None)
    }
    pub fn get_stat_outgoing_nps_applied(
        &mut self,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, FitStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        Ok(self.sol.svc.get_stat_fit_outgoing_nps(
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            Some(projectee_uid),
        ))
    }
}
