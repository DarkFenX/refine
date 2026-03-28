use crate::{
    api::{FitMut, FitStatAppliedError},
    num::PValue,
    svc::{
        cycle::CseqMap,
        vast::{
            StatDmg, StatDmgApplied, StatDmgItemKinds, StatMining, StatMiningItemKinds, StatNeutItemKinds,
            StatOutRepItemKinds, StatOutReps, StatTimeOptions,
        },
    },
    ud::ItemId,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_dmg(&mut self, item_kinds: StatDmgItemKinds, time_options: StatTimeOptions) -> StatDmg {
        let mut reuse_cseq_map = CseqMap::new();
        self.sol.svc.get_stat_fit_dmg_raw(
            &mut reuse_cseq_map,
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
        )
    }
    pub fn get_stat_dmg_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FitStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let mut reuse_cseq_map = CseqMap::new();
        Ok(self.sol.svc.get_stat_fit_dmg_applied(
            &mut reuse_cseq_map,
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            projectee_uid,
        ))
    }
    pub fn get_stat_mps(
        &mut self,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
        mission_ore: bool,
    ) -> StatMining {
        let mut reuse_cseq_map = CseqMap::new();
        self.sol.svc.get_stat_fit_mps(
            &mut reuse_cseq_map,
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            mission_ore,
        )
    }
    pub fn get_stat_outgoing_rps(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
    ) -> StatOutReps {
        let mut reuse_cseq_map = CseqMap::new();
        self.sol.svc.get_stat_fit_outgoing_rps(
            &mut reuse_cseq_map,
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            None,
        )
    }
    pub fn get_stat_outgoing_rps_applied(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatOutReps, FitStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let mut reuse_cseq_map = CseqMap::new();
        Ok(self.sol.svc.get_stat_fit_outgoing_rps(
            &mut reuse_cseq_map,
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            Some(projectee_uid),
        ))
    }
    pub fn get_stat_outgoing_cps(&mut self, time_options: StatTimeOptions) -> PValue {
        let mut reuse_cseq_map = CseqMap::new();
        self.sol
            .svc
            .get_stat_fit_outgoing_cps(&mut reuse_cseq_map, &self.sol.u_data, self.uid, time_options, None)
    }
    pub fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, FitStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let mut reuse_cseq_map = CseqMap::new();
        Ok(self.sol.svc.get_stat_fit_outgoing_cps(
            &mut reuse_cseq_map,
            &self.sol.u_data,
            self.uid,
            time_options,
            Some(projectee_uid),
        ))
    }
    pub fn get_stat_outgoing_nps(&mut self, item_kinds: StatNeutItemKinds, time_options: StatTimeOptions) -> PValue {
        let mut reuse_cseq_map = CseqMap::new();
        self.sol.svc.get_stat_fit_outgoing_nps(
            &mut reuse_cseq_map,
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            None,
        )
    }
    pub fn get_stat_outgoing_nps_applied(
        &mut self,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, FitStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let mut reuse_cseq_map = CseqMap::new();
        Ok(self.sol.svc.get_stat_fit_outgoing_nps(
            &mut reuse_cseq_map,
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            Some(projectee_uid),
        ))
    }
}
