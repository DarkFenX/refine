use crate::{
    ItemId, PValue,
    api::{active_stat_cseq_cache, active_stat_prepare},
    stats::{
        ItemStats, StatCritOptions, StatDmg, StatDmgApplied, StatItemChargeOptions, StatItemStateOptions, StatMining,
        StatMiningResourceKind, StatOutReps, StatTimeOptions,
        err::{StatItemAppliedError, StatItemError},
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> ItemStats<'s> {
    pub fn get_stat_dmg(
        &mut self,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
        state_options: StatItemStateOptions,
    ) -> Result<StatDmg, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let savepoint = active_stat_prepare(sol, item_uid, charge_options, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_dmg_raw(
                cseq_cache,
                &sol.u_data,
                item_uid,
                time_options,
                crit_options,
                charge_options,
            )
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_dmg_applied(
        &mut self,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
        state_options: StatItemStateOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, StatItemAppliedError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        // Check projectee before switching states, so that its errors leave states untouched
        let projectee_uid = sol.u_data.get_projectee_uid(projectee_item_id)?;
        let savepoint = active_stat_prepare(sol, item_uid, charge_options, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_dmg_applied(
                cseq_cache,
                &sol.u_data,
                item_uid,
                time_options,
                crit_options,
                charge_options,
                projectee_uid,
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_mps(
        &mut self,
        time_options: StatTimeOptions,
        resource_kind: StatMiningResourceKind,
        state_options: StatItemStateOptions,
    ) -> Result<StatMining, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let savepoint = active_stat_prepare(sol, item_uid, StatItemChargeOptions::Exclude, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_mps(cseq_cache, &sol.u_data, item_uid, time_options, resource_kind)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_outgoing_nps(
        &mut self,
        time_options: StatTimeOptions,
        charge_options: StatItemChargeOptions,
        state_options: StatItemStateOptions,
    ) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let savepoint = active_stat_prepare(sol, item_uid, charge_options, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_outgoing_nps(cseq_cache, &sol.u_data, item_uid, time_options, charge_options, None)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_outgoing_nps_applied(
        &mut self,
        time_options: StatTimeOptions,
        charge_options: StatItemChargeOptions,
        state_options: StatItemStateOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, StatItemAppliedError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        // Check projectee before switching states, so that its errors leave states untouched
        let projectee_uid = sol.u_data.get_projectee_uid(projectee_item_id)?;
        let savepoint = active_stat_prepare(sol, item_uid, charge_options, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_outgoing_nps(
                cseq_cache,
                &sol.u_data,
                item_uid,
                time_options,
                charge_options,
                Some(projectee_uid),
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_outgoing_rps(
        &mut self,
        time_options: StatTimeOptions,
        state_options: StatItemStateOptions,
    ) -> Result<StatOutReps, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let savepoint = active_stat_prepare(sol, item_uid, StatItemChargeOptions::Exclude, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_outgoing_rps(cseq_cache, &sol.u_data, item_uid, time_options, None)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_outgoing_rps_applied(
        &mut self,
        time_options: StatTimeOptions,
        state_options: StatItemStateOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatOutReps, StatItemAppliedError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        // Check projectee before switching states, so that its errors leave states untouched
        let projectee_uid = sol.u_data.get_projectee_uid(projectee_item_id)?;
        let savepoint = active_stat_prepare(sol, item_uid, StatItemChargeOptions::Exclude, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_outgoing_rps(cseq_cache, &sol.u_data, item_uid, time_options, Some(projectee_uid))
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_outgoing_cps(
        &mut self,
        time_options: StatTimeOptions,
        state_options: StatItemStateOptions,
    ) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let savepoint = active_stat_prepare(sol, item_uid, StatItemChargeOptions::Exclude, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_outgoing_cps(cseq_cache, &sol.u_data, item_uid, time_options, None)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
    pub fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        state_options: StatItemStateOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, StatItemAppliedError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        // Check projectee before switching states, so that its errors leave states untouched
        let projectee_uid = sol.u_data.get_projectee_uid(projectee_item_id)?;
        let savepoint = active_stat_prepare(sol, item_uid, StatItemChargeOptions::Exclude, state_options);
        let cseq_cache = active_stat_cseq_cache(&mut sol.reuse, &sol.undo, &savepoint);
        let result = sol
            .svc
            .get_stat_item_outgoing_cps(cseq_cache, &sol.u_data, item_uid, time_options, Some(projectee_uid))
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items));
        sol.internal_tx_rollback(savepoint);
        result
    }
}
