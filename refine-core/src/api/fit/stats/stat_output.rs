use crate::{
    api::{FitAppliedStatError, FitMut},
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

impl<'s> FitMut<'s> {
    pub fn get_stat_dmg(&mut self, item_kinds: StatDmgItemKinds, time_options: StatTimeOptions) -> StatDmg {
        self.sol.svc.get_stat_fit_dmg_raw(
            &mut CseqMap::new(),
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
    ) -> Result<StatDmgApplied, FitAppliedStatError> {
        let projectee_uid = self.sol.u_data.get_projectee_uid(projectee_item_id)?;
        Ok(self.sol.svc.get_stat_fit_dmg_applied(
            &mut CseqMap::new(),
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
        self.sol.svc.get_stat_fit_mps(
            &mut CseqMap::new(),
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
        self.sol.svc.get_stat_fit_outgoing_rps(
            &mut CseqMap::new(),
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
    ) -> Result<StatOutReps, FitAppliedStatError> {
        let projectee_uid = self.sol.u_data.get_projectee_uid(projectee_item_id)?;
        Ok(self.sol.svc.get_stat_fit_outgoing_rps(
            &mut CseqMap::new(),
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
            .get_stat_fit_outgoing_cps(&mut CseqMap::new(), &self.sol.u_data, self.uid, time_options, None)
    }
    pub fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, FitAppliedStatError> {
        let projectee_uid = self.sol.u_data.get_projectee_uid(projectee_item_id)?;
        Ok(self.sol.svc.get_stat_fit_outgoing_cps(
            &mut CseqMap::new(),
            &self.sol.u_data,
            self.uid,
            time_options,
            Some(projectee_uid),
        ))
    }
    pub fn get_stat_outgoing_nps(&mut self, item_kinds: StatNeutItemKinds, time_options: StatTimeOptions) -> PValue {
        self.sol.svc.get_stat_fit_outgoing_nps(
            &mut CseqMap::new(),
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
    ) -> Result<PValue, FitAppliedStatError> {
        let projectee_uid = self.sol.u_data.get_projectee_uid(projectee_item_id)?;
        Ok(self.sol.svc.get_stat_fit_outgoing_nps(
            &mut CseqMap::new(),
            &self.sol.u_data,
            self.uid,
            item_kinds,
            time_options,
            Some(projectee_uid),
        ))
    }
}
